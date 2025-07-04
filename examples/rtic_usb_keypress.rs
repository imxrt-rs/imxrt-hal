//! Demonstrates a USB keypress using RTIC.
//!
//! Flash your board with this example. Your device will occasionally
//! send some kind of keypress to your host.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState, Speed};
    use imxrt_hal as hal;

    use usb_device::{
        bus::UsbBusAllocator,
        device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    };
    use usbd_hid::{
        descriptor::{KeyboardReport, SerializedDescriptor as _},
        hid_class::HIDClass,
    };

    /// Change me if you want to play with a full-speed USB device.
    const SPEED: Speed = Speed::High;
    /// Matches whatever is in imxrt-log.
    const VID_PID: UsbVidPid = UsbVidPid(0x5824, 0x27dd);
    const PRODUCT: &str = "imxrt-hal-example";
    /// How frequently should we poll the logger?
    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 100;
    /// Change me to change how log messages are serialized.
    ///
    /// If changing to `Defmt`, you'll need to update the logging macros in
    /// this example. You'll also need to make sure the USB device you're debugging
    /// uses `defmt`.
    const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
    /// The USB GPT timer we use to (infrequently) send mouse updates.
    const GPT_INSTANCE: imxrt_usbd::gpt::Instance = imxrt_usbd::gpt::Instance::Gpt0;
    /// How frequently should we push mouse updates to the host?
    const MOUSE_UPDATE_INTERVAL_MS: u32 = 200;

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<1024> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    use core::{iter::Cycle, slice::Iter};
    type MessageIter = Cycle<Iter<'static, u8>>;
    const MESSAGE: &[u8] = b"Ia! Ia! Cthulhu fhtagn!  ";

    type Bus = BusAdapter;

    #[local]
    struct Local {
        class: HIDClass<'static, Bus>,
        device: UsbDevice<'static, Bus>,
        led: board::Led,
        poller: board::logging::Poller,
        timer: hal::pit::Pit<0>,
        message: MessageIter,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<Bus>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let (
            board::Common {
                pit: (mut timer, _, _, _),
                usb1,
                usbnc1,
                usbphy1,
                mut dma,
                ..
            },
            board::Specifics { led, console, .. },
        ) = board::new();
        timer.set_load_timer_value(LPUART_POLL_INTERVAL_MS);
        timer.set_interrupt_enable(true);
        timer.enable();

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = board::logging::lpuart(FRONTEND, console, dma_a);

        let usbd = hal::usbd::Instances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
        };

        let bus = BusAdapter::with_speed(usbd, &EP_MEMORY, &EP_STATE, SPEED);
        bus.set_interrupts(true);
        bus.gpt_mut(GPT_INSTANCE, |gpt| {
            gpt.stop();
            gpt.clear_elapsed();
            gpt.set_interrupt_enabled(true);
            gpt.set_mode(imxrt_usbd::gpt::Mode::Repeat);
            gpt.set_load(MOUSE_UPDATE_INTERVAL_MS * 1000);
            gpt.reset();
            gpt.run();
        });

        let bus = ctx.local.bus.insert(UsbBusAllocator::new(bus));
        // Note that "4" correlates to a 1ms polling interval. Since this is a high speed
        // device, bInterval is computed differently.
        let class = HIDClass::new(bus, KeyboardReport::desc(), 4);
        let device = UsbDeviceBuilder::new(bus, VID_PID)
            .strings(&[usb_device::device::StringDescriptors::default().product(PRODUCT)])
            .unwrap()
            .device_class(usbd_serial::USB_CLASS_CDC)
            .max_packet_size_0(64)
            .unwrap()
            .build();

        (
            Shared {},
            Local {
                class,
                device,
                led,
                poller,
                timer,
                message: MESSAGE.iter().cycle(),
            },
        )
    }

    #[task(binds = BOARD_PIT, local = [poller, timer], priority = 1)]
    fn pit_interrupt(ctx: pit_interrupt::Context) {
        while ctx.local.timer.is_elapsed() {
            ctx.local.timer.clear_elapsed();
        }

        ctx.local.poller.poll();
    }

    #[task(binds = BOARD_USB1, local = [device, class, led, message, configured: bool = false], priority = 2)]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources {
            class,
            device,
            led,
            configured,
            message,
            ..
        } = ctx.local;

        device.poll(&mut [class]);

        if device.state() == UsbDeviceState::Configured {
            if !*configured {
                device.bus().configure();
            }
            *configured = true;
        } else {
            *configured = false;
        }

        if *configured {
            let elapsed = device.bus().gpt_mut(GPT_INSTANCE, |gpt| {
                let elapsed = gpt.is_elapsed();
                while gpt.is_elapsed() {
                    gpt.clear_elapsed();
                }
                elapsed
            });

            if elapsed {
                led.toggle();
                let code = *message.next().unwrap();
                if let Some(report) = translate_char(code) {
                    class.push_input(&report).ok();
                }
            }
        }
    }

    fn translate_char(ch: u8) -> Option<KeyboardReport> {
        fn simple_kr(modifier: u8, keycodes: [u8; 6]) -> Option<KeyboardReport> {
            Some(KeyboardReport {
                modifier,
                reserved: 0,
                leds: 0,
                keycodes,
            })
        }

        match ch {
            b'a'..=b'z' => {
                let code = ch - b'a' + 4;
                simple_kr(0, [code, 0, 0, 0, 0, 0])
            }
            b'A'..=b'Z' => {
                let code = ch - b'A' + 4;
                simple_kr(2, [code, 0, 0, 0, 0, 0])
            }
            b'!'..=b')' => {
                let code = ch - b'!' + 0x1e;
                simple_kr(2, [code, 0, 0, 0, 0, 0])
            }
            b' ' => simple_kr(0, [0x2c, 0, 0, 0, 0, 0]),
            _ => {
                log::error!("Unsupported character '{ch}'");
                None
            }
        }
    }
}
