//! Implements the usb-device test class.
//!
//! This is an important example for USB device testing. It turns your board
//! into a USB device that can be tested from the usb-device host-side testing
//! framework. See the usb-device documentation for more information on the test.
//!
//! The gist of the testing process:
//!
//! 1. Flash this example onto your board. Keep your board connected to your host.
//! 2. Clone the usb-device repository.
//! 3. Run `cargo test` in the usb-device directory.
//!
//! The test harness should detect your board and execute tests against it.
//! All of those tests should pass.
//!
//! If you're playing the benchmark game and optimizing for bulk read / write
//! throughput, configure your runtime to place as much as possible into TCM.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use hal::usbd::{BusAdapter, EndpointMemory, EndpointState, Speed};
    use imxrt_hal as hal;

    use usb_device::{
        bus::UsbBusAllocator,
        device::{UsbDevice, UsbDeviceState},
        test_class::TestClass,
    };

    /// Change me if you want to test a full-speed USB device.
    ///
    /// *NOTE*: if you change this, you need to *disable* the usb-device
    /// "test-class-high-speed" feature, which is enabled by default. See
    /// the top-level Cargo.toml.
    const SPEED: Speed = Speed::High;
    /// How frequently should we poll the logger?
    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 100;
    /// Change me to change how log messages are serialized.
    ///
    /// If changing to `Defmt`, you'll need to update the logging macros in
    /// this example. You'll also need to make sure the USB device you're debugging
    /// uses `defmt`.
    const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;

    /// This allocation is shared across all USB endpoints. It needs to be large
    /// enough to hold the maximum packet size for *all* endpoints. If you start
    /// noticing panics, check to make sure that this is large enough for all endpoints.
    static EP_MEMORY: EndpointMemory<4096> = EndpointMemory::new();
    /// This manages the endpoints. It's large enough to hold the maximum number
    /// of endpoints; we're not using all the endpoints in this example.
    static EP_STATE: EndpointState = EndpointState::max_endpoints();

    type Bus = BusAdapter;

    #[local]
    struct Local {
        class: TestClass<'static, Bus>,
        device: UsbDevice<'static, Bus>,
        poller: board::logging::Poller,
        timer: hal::pit::Pit<0>,
    }

    #[shared]
    struct Shared {}

    #[init(local = [bus: Option<UsbBusAllocator<Bus>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            board::Common {
                pit: (mut timer, _, _, _),
                usb1,
                usbnc1,
                usbphy1,
                mut dma,
                ..
            },
            board::Specifics { console, .. },
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

        let bus = ctx.local.bus.insert(UsbBusAllocator::new(bus));
        let class = TestClass::new(bus);
        let device = class.make_device_builder(bus).max_packet_size_0(64).build();

        (
            Shared {},
            Local {
                class,
                device,
                poller,
                timer,
            },
            init::Monotonics(),
        )
    }

    /// Occasionally try to poll the logger.
    #[task(binds = BOARD_PIT, local = [poller, timer], priority = 1)]
    fn pit_interrupt(ctx: pit_interrupt::Context) {
        while ctx.local.timer.is_elapsed() {
            ctx.local.timer.clear_elapsed();
        }
        ctx.local.poller.poll();
    }

    #[task(binds = BOARD_USB1, local = [class, device, configured: bool = false], priority = 2)]
    fn usb1(ctx: usb1::Context) {
        let usb1::LocalResources {
            class,
            device,
            configured,
        } = ctx.local;

        if device.poll(&mut [class]) {
            if device.state() == UsbDeviceState::Configured {
                if !*configured {
                    device.bus().configure();
                }
                *configured = true;

                class.poll();
            } else {
                *configured = false;
            }
        }
    }
}
