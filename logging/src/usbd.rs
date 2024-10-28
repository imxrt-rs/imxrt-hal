//! USB serial (CDC) backend using imxrt-usbd.

use static_cell::StaticCell;
use usb_device::device::UsbDeviceState;

const VID_PID: usb_device::device::UsbVidPid = usb_device::device::UsbVidPid(0x5824, 0x27dd);
const PRODUCT: &str = "imxrt-log";

/// Provide some extra overhead for the interrupt endpoint.
///
/// If you start noticing panics, check to make sure that this buffer
/// is large enough for all the max packet sizes for all the endpoints.
const ENDPOINT_BYTES: usize = MAX_PACKET_SIZE * 2 + EP0_CONTROL_PACKET_SIZE * 2 + 128;
static ENDPOINT_MEMORY: imxrt_usbd::EndpointMemory<ENDPOINT_BYTES> =
    imxrt_usbd::EndpointMemory::new();
static ENDPOINT_STATE: imxrt_usbd::EndpointState<6> = imxrt_usbd::EndpointState::new();

type Bus = imxrt_usbd::BusAdapter;
type BusAllocator = usb_device::bus::UsbBusAllocator<Bus>;
type Class<'a> = usbd_serial::CdcAcmClass<'a, Bus>;
type Device<'a> = usb_device::device::UsbDevice<'a, Bus>;

/// High-speed bulk endpoint limit.
const MAX_PACKET_SIZE: usize = crate::config::USB_BULK_MPS;
/// Size for control transfers on endpoint 0.
const EP0_CONTROL_PACKET_SIZE: usize = 64;
/// The USB GPT timer we use to (infrequently) check for data.
const GPT_INSTANCE: imxrt_usbd::gpt::Instance = imxrt_usbd::gpt::Instance::Gpt0;

pub(crate) struct Backend {
    class: Class<'static>,
    device: Device<'static>,
    consumer: crate::Consumer,
    configured: bool,
}

impl Backend {
    pub(crate) fn poll(&mut self) {
        // Is there a CDC class event, like a completed transfer? If so, check
        // the consumer immediately, even if a timer hasn't expired.
        //
        // Checking the consumer on class traffic lets the driver burst out data.
        // Suppose the user wants to use the USB GPT timer, and they configure a very
        // long interval. That interval expires, and we see tons of data in the consumer.
        // We should write that out as fast as possible, even if the timer hasn't elapsed.
        // That's the behavior provided by the class_event flag.
        let class_event = self.device.poll(&mut [&mut self.class]);
        let timer_event = self.device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
            let mut elapsed = false;
            while gpt.is_elapsed() {
                gpt.clear_elapsed();
                elapsed = true;
            }
            // Simulate a timer event if the timer is not running.
            //
            // If the timer is not running, its because the user disabled interrupts,
            // and they're using their own timer / polling loop. There might not always
            // be a class traffic (transfer complete) event when the user polls, so
            // signaling true allows the poll to check the consumer for new data and
            // send it.
            //
            // If the timer is running, checking the consumer depends on the elapsed
            // timer.
            elapsed || !gpt.is_running()
        });
        let check_consumer = class_event || timer_event;

        if self.device.state() != UsbDeviceState::Configured {
            if self.configured {
                // Turn off the timer, but only if we were previously configured.
                self.device.bus().gpt_mut(GPT_INSTANCE, |gpt| gpt.stop());
            }
            self.configured = false;
            // We can't use the class if we're not configured,
            // so bail out here.
            return;
        }

        // We're now configured. Are we newly configured?
        if !self.configured {
            // Must call this when we transition into configured.
            self.device.bus().configure();
            self.device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
                // There's no need for a timer if interrupts are disabled.
                // If the user disabled USB interrupts and decided to poll this
                // from another timer, this USB timer could unnecessarily block
                // that timer from checking the consumer queue.
                if gpt.is_interrupt_enabled() {
                    gpt.run()
                }
            });
            self.configured = true;
        }

        // If the host sends us data, pretend to read it.
        // This prevents us from continuously NAKing the host,
        // which the host might not appreciate.
        self.class.read_packet(&mut []).ok();

        // There's no need to wait if we were are newly configured.
        if check_consumer {
            if let Ok(grant) = self.consumer.read() {
                let buf = grant.buf();
                // Don't try to write more than we can fit in a single packet!
                // See the usbd-serial documentation for this caveat. We didn't
                // statically allocate enough space for anything larger.
                if let Ok(written) = self
                    .class
                    .write_packet(&buf[..MAX_PACKET_SIZE.min(buf.len())])
                {
                    grant.release(written);
                    // Log data is in the intermediate buffer, so it's OK to release the grant.
                    //
                    // If the I/O fails here, we'll try again on the next poll. There's no guarantee
                    // we'll see a improvement though...
                }
            } // else, no data, or some error. Let those logs accumulate!
        }
    }
}

/// Initialize the USB logger.
///
/// # Panics
///
/// Panics if called more than once.
pub(crate) fn init<P: imxrt_usbd::Peripherals>(
    peripherals: P,
    interrupts: crate::Interrupts,
    consumer: super::Consumer,
    config: &UsbdConfig,
) -> &'static mut Backend {
    static BACKEND: StaticCell<Backend> = StaticCell::new();
    BACKEND.init_with(|| {
        static BUS: StaticCell<BusAllocator> = StaticCell::new();
        let bus = BUS.init_with(|| {
            // Safety: we ensure that the bus, class, and all other related USB objects
            // are accessed in poll(). poll() is not reentrant, so there's no racing
            // occuring across executing contexts.
            let bus = unsafe {
                imxrt_usbd::BusAdapter::without_critical_sections(
                    peripherals,
                    &ENDPOINT_MEMORY,
                    &ENDPOINT_STATE,
                    crate::config::USB_SPEED,
                )
            };
            bus.set_interrupts(interrupts == crate::Interrupts::Enabled);
            bus.gpt_mut(GPT_INSTANCE, |gpt| {
                gpt.stop();
                gpt.clear_elapsed();
                gpt.set_interrupt_enabled(interrupts == crate::Interrupts::Enabled);
                gpt.set_mode(imxrt_usbd::gpt::Mode::Repeat);
                gpt.set_load(config.poll_interval_us);
                gpt.reset();
            });
            usb_device::bus::UsbBusAllocator::new(bus)
        });
        let class = usbd_serial::CdcAcmClass::new(bus, MAX_PACKET_SIZE as u16);

        let device = usb_device::device::UsbDeviceBuilder::new(bus, VID_PID)
            .strings(&[usb_device::device::StringDescriptors::default().product(PRODUCT)])
            .unwrap()
            .device_class(usbd_serial::USB_CLASS_CDC)
            .max_packet_size_0(EP0_CONTROL_PACKET_SIZE as u8)
            .unwrap()
            .build();

        // Not sure which endpoints the CDC ACM class will pick,
        // so enable the setting for all non-zero endpoints.
        for idx in 1..8 {
            for dir in &[usb_device::UsbDirection::In, usb_device::UsbDirection::Out] {
                let ep_addr = usb_device::endpoint::EndpointAddress::from_parts(idx, *dir);
                // CDC class requires that we send the ZLP.
                // Let the hardware do that for us.
                device.bus().enable_zlt(ep_addr);
            }
        }

        Backend {
            class,
            device,
            consumer,
            configured: false,
        }
    })
}

/// USB device configuration builder.
///
/// Use this to construct a [`UsbdConfig`], which provides settings
/// to the USB device. For additional configurations that can only
/// be safely expressed statically, see the package configuration
/// documentation.
///
/// # Default values
///
/// The snippet below demonstrates the default values.
///
/// ```
/// use imxrt_log::{UsbdConfigBuilder, UsbdConfig};
///
/// const DEFAULT_VALUES: UsbdConfig =
///     UsbdConfigBuilder::new()
///         .poll_interval_us(4_000)
///         .build();
///
/// assert_eq!(DEFAULT_VALUES, UsbdConfigBuilder::new().build());
/// ```
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UsbdConfigBuilder {
    cfg: UsbdConfig,
}

impl Default for UsbdConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbdConfigBuilder {
    /// Create a new builder with the default values.
    pub const fn new() -> Self {
        Self {
            cfg: UsbdConfig::default(),
        }
    }

    /// Build the USB device configuration.
    pub const fn build(self) -> UsbdConfig {
        self.cfg
    }

    /// Set the USB timer polling interval, in microseconds.
    ///
    /// This value has no effect if interrupts are disabled. See the USB device
    /// backend documentation for more information.
    ///
    /// Note that the USB device driver internally clamps this value to 2^24.
    pub const fn poll_interval_us(mut self, poll_interval_us: u32) -> Self {
        self.cfg.poll_interval_us = poll_interval_us;
        self
    }
}

/// A USB device configuration.
///
/// Use [`UsbdConfigBuilder`] to build a configuration.
#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UsbdConfig {
    poll_interval_us: u32,
}

impl UsbdConfig {
    /// Returns a configuration with the default values.
    const fn default() -> Self {
        Self {
            poll_interval_us: 4_000,
        }
    }
}
