//! Demonstrates imxrt-log text and defmt logging using RTIC.
//!
//! Modify the `FRONTEND` and `BACKEND` configurations to combine log / defmt
//! with a LPUART or USB CDC logger. When LPUART is selected, the log messages
//! are written over the board's console; see your board's `Console` documentation
//! for information on the settings. When a USB device is selected, the log messages
//! are written over a USB serial interface.
//!
//! See the `imxrt-log` documentation for more information on each logging backend.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = true, dispatchers = [BOARD_SWTASK0])]
mod app {

    //
    // Configure the demo below.
    //

    /// Change me to change how log messages are serialized
    /// and transported.
    const FRONTEND: Frontend = Frontend::Log;
    /// Change me to select the peripheral used for logging.
    const BACKEND: Backend = Backend::Usbd;

    #[derive(PartialEq, Eq, Debug, defmt::Format)]
    pub enum Frontend {
        Log,
        Defmt,
    }

    #[derive(PartialEq, Eq, Debug, defmt::Format)]
    pub enum Backend {
        Usbd,
        Lpuart,
    }

    /// When using the LPUART backend, you want to occasionally poll
    /// for new log message. This interval (milliseconds) describes how
    /// long you should wait in between polls.
    ///
    /// This constant does nothing when the USBD backend is used, since
    /// the USBD backend uses its own timer for this purpose.
    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 4;
    /// How frequently (milliseconds) should we make a log message?
    ///
    /// Decrease this constant to log more frequently.
    const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

    //
    // End configurations.
    //

    use imxrt_hal as hal;

    #[local]
    struct Local {
        /// Toggle when we make a log.
        led: board::Led,
        /// This timer tells us how frequently to poll
        /// for logs. It's only used with the LPUART
        /// logging backend.
        poll_log: hal::pit::Pit<1>,
        /// This timer tells us how frequently to generate
        /// logs. It's always used.
        make_log: hal::pit::Pit<2>,
    }

    #[shared]
    struct Shared {
        /// The poller drives the logging backend.
        poller: imxrt_log::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Board {
            led,
            pit: (_, mut poll_log, mut make_log, _),
            usb1,
            usbnc1,
            usbphy1,
            mut usb_analog,
            console,
            mut dma,
            ..
        } = board::new(cx.device);
        // We only need the extra timer when the LPUART backend is used.
        // The USBD backend uses the USB peripheral's internal timer to
        // track time for us.
        if BACKEND == Backend::Lpuart {
            poll_log.set_load_timer_value(LPUART_POLL_INTERVAL_MS);
            poll_log.set_interrupt_enable(true);
            poll_log.enable();
        } else {
            poll_log.disable();
        }

        make_log.set_load_timer_value(MAKE_LOG_INTERVAL_MS);
        make_log.set_interrupt_enable(true);
        make_log.enable();

        let usb_instances = hal::usbd::UsbInstances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
            usb_analog: &mut usb_analog,
        };

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = match (FRONTEND, BACKEND) {
            // Logging frontends...
            (Frontend::Log, Backend::Lpuart) => imxrt_log::log::lpuart(
                console,
                dma_a,
                imxrt_log::Interrupts::Enabled,
                Default::default(),
            )
            .unwrap_or_else(|_| panic!()),
            (Frontend::Log, Backend::Usbd) => imxrt_log::log::usb(
                usb_instances,
                imxrt_log::Interrupts::Enabled,
                Default::default(),
            )
            .unwrap_or_else(|_| panic!()),
            // Defmt frontends...
            (Frontend::Defmt, Backend::Lpuart) => {
                imxrt_log::defmt::lpuart(console, dma_a, imxrt_log::Interrupts::Enabled)
                    .unwrap_or_else(|_| panic!())
            }
            (Frontend::Defmt, Backend::Usbd) => {
                imxrt_log::defmt::usb(usb_instances, imxrt_log::Interrupts::Enabled)
                    .unwrap_or_else(|_| panic!())
            }
        };

        (
            Shared { poller },
            Local {
                led,
                poll_log,
                make_log,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }

    /// This interrupt fires
    ///
    /// - when log messages have been written (to the USB host).
    /// - every few milliseconds; check the imxrt-log docs for the
    ///   specific number.
    #[task(binds = BOARD_USB1, priority = 1)]
    fn usb_interrupt(_: usb_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// This interrupt fires
    ///
    /// - when log messages have been written.
    ///
    /// Notice how there's no "periodic" or "time" component here.
    /// When using the LPUART backend, you should use another time
    /// source, or a polling loop, to make sure poll periodically
    /// happens. Without this, you won't see your log messages.
    #[task(binds = BOARD_DMA_A, priority = 1)]
    fn dma_interrupt(_: dma_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// Actually performs the poll call.
    #[task(shared = [poller], priority = 2)]
    fn poll_logger(mut cx: poll_logger::Context) {
        cx.shared.poller.lock(|poller| poller.poll());
    }

    #[task(binds = BOARD_PIT, local = [led, poll_log, make_log, counter: u32 = 0], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            poll_log,
            make_log,
            led,
            counter,
        } = cx.local;

        // Is it time for us to poll the logger?
        // This only happens for the LPUART backend.
        if poll_log.is_elapsed() {
            while poll_log.is_elapsed() {
                poll_log.clear_elapsed();
            }
            poll_logger::spawn().unwrap();
        }

        // Is it time for us to send a new log message?
        if make_log.is_elapsed() {
            led.toggle();
            while make_log.is_elapsed() {
                make_log.clear_elapsed();
            }
            log::info!("Hello from the log framework over {BACKEND:?}! The count is {counter}");
            defmt::println!(
                "Hello from defmt over {}! The count is {=u32}",
                BACKEND,
                counter
            );
            *counter += 1;
        }
    }
}
