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

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    //
    // Configure the demo below.
    //

    /// Change me to change how log messages are serialized
    /// and transported.
    const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
    /// This is a function of your board. Want to change it? Change it right
    /// here to explore different example code paths.
    const BACKEND: board::logging::Backend = board::logging::BACKEND;

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
    use imxrt_hal::pit::Channel;

    const POLL_LOG_CHANNEL: Channel = Channel::Chan1;
    const MAKE_LOG_CHANNEL: Channel = Channel::Chan2;

    #[local]
    struct Local {
        /// Toggle when we make a log.
        led: board::Led,
        /// The PIT peripheral for timing operations.
        pit: hal::pit::Pit,
    }

    #[shared]
    struct Shared {
        /// The poller drives the logging backend.
        poller: board::logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut cortex_m = cx.core;
        let (
            board::Common {
                mut pit,
                usb1,
                usbnc1,
                usbphy1,

                mut dma,
                ..
            },
            board::Specifics { led, console, .. },
        ) = board::new();
        cortex_m.DCB.enable_trace();
        cortex_m::peripheral::DWT::unlock();
        cortex_m.DWT.enable_cycle_counter();

        // We only need the extra timer when the LPUART backend is used.
        // The USBD backend uses the USB peripheral's internal timer to
        // track time for us.
        if BACKEND == board::logging::Backend::Lpuart {
            pit.set_load_timer_value(POLL_LOG_CHANNEL, LPUART_POLL_INTERVAL_MS);
            pit.set_interrupt_enable(POLL_LOG_CHANNEL, true);
            pit.enable(POLL_LOG_CHANNEL);
        } else {
            pit.disable(POLL_LOG_CHANNEL);
        }

        pit.set_load_timer_value(MAKE_LOG_CHANNEL, MAKE_LOG_INTERVAL_MS);
        pit.set_interrupt_enable(MAKE_LOG_CHANNEL, true);
        pit.enable(MAKE_LOG_CHANNEL);

        let usbd = imxrt_usbd::Instances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
        };

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

        (Shared { poller }, Local { led, pit })
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
    async fn poll_logger(mut cx: poll_logger::Context) {
        cx.shared.poller.lock(|poller| poller.poll());
    }

    #[task(binds = BOARD_PIT, local = [led, pit, counter: u32 = 0], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            pit, led, counter, ..
        } = cx.local;

        // Is it time for us to poll the logger?
        // This only happens for the LPUART backend.
        if pit.is_elapsed(POLL_LOG_CHANNEL) {
            while pit.is_elapsed(POLL_LOG_CHANNEL) {
                pit.clear_elapsed(POLL_LOG_CHANNEL);
            }
            poll_logger::spawn().unwrap();
        }

        // Is it time for us to send a new log message?
        if pit.is_elapsed(MAKE_LOG_CHANNEL) {
            led.toggle();
            while pit.is_elapsed(MAKE_LOG_CHANNEL) {
                pit.clear_elapsed(MAKE_LOG_CHANNEL);
            }

            let count = cycles(|| {
                log::info!("Hello from the log framework over {BACKEND:?}! The count is {counter}")
            });
            log::info!(
                "That last message took {count} cycles to be copied into the logging buffer"
            );

            let count = cycles(|| {
                defmt::println!(
                    "Hello from defmt over {}! The count is {=u32}",
                    BACKEND,
                    counter
                )
            });
            defmt::println!(
                "That last message took {=u32} cycles to be copied into the logging buffer",
                count
            );

            *counter += 1;
        }
    }

    /// Count the clock cycles required to execute `f`
    fn cycles<F: FnOnce()>(f: F) -> u32 {
        let start = cortex_m::peripheral::DWT::cycle_count();
        f();
        let end = cortex_m::peripheral::DWT::cycle_count();
        end - start
    }
}
