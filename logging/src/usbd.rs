//! USB serial (CDC) backend using imxrt-usbd.

use core::mem::MaybeUninit;
use imxrt_usbd::Instances;
use usb_device::device::UsbDeviceState;

const VID_PID: usb_device::device::UsbVidPid = usb_device::device::UsbVidPid(0x5824, 0x27dd);
const PRODUCT: &str = "imxrt-log";

/// Provide some extra overhead for the interrupt endpoint.
///
/// If you start noticing panics, check to make sure that this buffer
/// is large enough for all the max packet sizes for all the endpoints.
const ENDPOINT_BYTES: usize = MAX_PACKET_SIZE * 2 + EP0_CONTROL_PACKET_SIZE * 2 + 128;
static mut ENDPOINT_MEMORY: [u8; ENDPOINT_BYTES] = [0; ENDPOINT_BYTES];

type Bus = imxrt_usbd::BusAdapter;
type BusAllocator = usb_device::bus::UsbBusAllocator<Bus>;
type Class<'a> = usbd_serial::CdcAcmClass<'a, Bus>;
type Device<'a> = usb_device::device::UsbDevice<'a, Bus>;

static mut BUS: MaybeUninit<BusAllocator> = MaybeUninit::uninit();
static mut CLASS: MaybeUninit<Class<'static>> = MaybeUninit::uninit();
static mut DEVICE: MaybeUninit<Device<'static>> = MaybeUninit::uninit();
static mut CONSUMER: MaybeUninit<crate::Consumer> = MaybeUninit::uninit();

/// High-speed bulk endpoint limit.
const MAX_PACKET_SIZE: usize = crate::config::USB_BULK_MPS;
/// Size for control transfers on endpoint 0.
const EP0_CONTROL_PACKET_SIZE: usize = 64;
/// The USB GPT timer we use to (infrequently) check for data.
const GPT_INSTANCE: imxrt_usbd::gpt::Instance = imxrt_usbd::gpt::Instance::Gpt0;
/// How many microseconds to delay before checking for data.
const CHECK_CONSUMER_INTERVAL_US: u32 = crate::config::USB_TIMER_INTERVAL;

/// Drive the logging behavior.
///
/// # Safety
///
/// This may only be called from one execution context. It can only be called
/// after all `static mut`s are initialized.
///
/// By exposing this function through a [`Poller`](crate::Poller), we make both
/// of these guarantees. The `Poller` indirectly "owns" the static mut memory,
/// and the crate design ensures that there's only one `Poller` object in existence.
pub(crate) unsafe fn poll() {
    static mut CONFIGURED: bool = false;
    let device = DEVICE.assume_init_mut();
    let class = CLASS.assume_init_mut();

    // Is there a CDC class event, like a completed transfer? If so, check
    // the consumer immediately, even if a timer hasn't expired.
    //
    // Checking the consumer on class traffic lets the driver burst out data.
    // Suppose the user wants to use the USB GPT timer, and they configure a very
    // long interval. That interval expires, and we see tons of data in the consumer.
    // We should write that out as fast as possible, even if the timer hasn't elapsed.
    // That's the behavior provided by the class_event flag.
    let class_event = device.poll(&mut [class]);
    let timer_event = device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
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

    if device.state() != UsbDeviceState::Configured {
        if CONFIGURED {
            // Turn off the timer, but only if we were previously configured.
            device.bus().gpt_mut(GPT_INSTANCE, |gpt| gpt.stop());
        }
        CONFIGURED = false;
        // We can't use the class if we're not configured,
        // so bail out here.
        return;
    }

    // We're now configured. Are we newly configured?
    if !CONFIGURED {
        // Must call this when we transition into configured.
        device.bus().configure();
        device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
            // There's no need for a timer if interrupts are disabled.
            // If the user disabled USB interrupts and decided to poll this
            // from another timer, this USB timer could unnecessarily block
            // that timer from checking the consumer queue.
            if gpt.is_interrupt_enabled() {
                gpt.run()
            }
        });
        CONFIGURED = true;
    }

    // There's no need to wait if we were are newly configured.
    if check_consumer {
        let consumer = CONSUMER.assume_init_mut();
        if let Ok(grant) = consumer.read() {
            let buf = grant.buf();
            // Don't try to write more than we can fit in a single packet!
            // See the usbd-serial documentation for this caveat. We didn't
            // statically allocate enough space for anything larger.
            if let Ok(written) = class.write_packet(&buf[..MAX_PACKET_SIZE.min(buf.len())]) {
                grant.release(written);
                // Log data is in the intermediate buffer, so it's OK to release the grant.
                //
                // If the I/O fails here, we'll try again on the next poll. There's no guarantee
                // we'll see a improvement though...
            }
        } // else, no data, or some error. Let those logs accumulate!
    }
}

/// Initialize the USB logger.
///
/// # Safety
///
/// This can only be called once.
pub(crate) unsafe fn init<const N: u8>(
    peripherals: Instances<'_, N>,
    interrupts: crate::Interrupts,
    consumer: super::Consumer,
) {
    CONSUMER.write(consumer);

    let bus = {
        let bus = imxrt_usbd::BusAdapter::without_critical_sections(
            peripherals,
            &mut ENDPOINT_MEMORY,
            crate::config::USB_SPEED,
        );
        // Not sure which endpoints the CDC ACM class will pick,
        // so enable the setting for all non-zero endpoints.
        for idx in 1..8 {
            for dir in &[usb_device::UsbDirection::In, usb_device::UsbDirection::Out] {
                let ep_addr = usb_device::endpoint::EndpointAddress::from_parts(idx, *dir);
                // CDC class requires that we send the ZLP.
                // Let the hardware do that for us.
                bus.enable_zlt(ep_addr);
            }
        }

        bus.set_interrupts(interrupts == crate::Interrupts::Enabled);
        bus.gpt_mut(GPT_INSTANCE, |gpt| {
            gpt.stop();
            gpt.clear_elapsed();
            gpt.set_interrupt_enabled(interrupts == crate::Interrupts::Enabled);
            gpt.set_mode(imxrt_usbd::gpt::Mode::Repeat);
            gpt.set_load(CHECK_CONSUMER_INTERVAL_US);
            gpt.reset();
        });
        let bus = usb_device::bus::UsbBusAllocator::new(bus);
        BUS.write(bus)
    };

    {
        let class = usbd_serial::CdcAcmClass::new(bus, MAX_PACKET_SIZE as u16);
        CLASS.write(class);
    }

    {
        let device = usb_device::device::UsbDeviceBuilder::new(bus, VID_PID)
            .product(PRODUCT)
            .device_class(usbd_serial::USB_CLASS_CDC)
            .max_packet_size_0(EP0_CONTROL_PACKET_SIZE as u8)
            .build();
        DEVICE.write(device);
    }
}
