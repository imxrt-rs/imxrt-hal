//! Demonstrates defmt logging using RTT with RTIC.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    //
    // Configure the demo below.
    //

    /// How frequently (milliseconds) should we make a log message?
    ///
    /// Decrease this constant to log more frequently.
    const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

    use imxrt_hal as hal;
    //
    // End configurations.
    //

    #[local]
    struct Local {
        /// Toggle when we make a log.
        led: board::Led,
        /// This timer tells us how frequently to generate
        /// logs. It's always used.
        make_log: hal::pit::Pit<2>,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut cortex_m = cx.core;
        let (
            board::Common {
                pit: (_, _, mut make_log, _),
                ..
            },
            board::Specifics { led, .. },
        ) = board::new();
        cortex_m.DCB.enable_trace();
        cortex_m::peripheral::DWT::unlock();
        cortex_m.DWT.enable_cycle_counter();

        make_log.set_load_timer_value(MAKE_LOG_INTERVAL_MS);
        make_log.set_interrupt_enable(true);
        make_log.enable();

        (Shared {}, Local { led, make_log })
    }

    #[task(binds = BOARD_PIT, local = [led, make_log, counter: u32 = 0], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            make_log,
            led,
            counter,
            ..
        } = cx.local;

        // Is it time for us to send a new log message?
        if make_log.is_elapsed() {
            led.toggle();
            while make_log.is_elapsed() {
                make_log.clear_elapsed();
            }

            let count = cycles(|| {
                defmt::println!("Hello from defmt over RTT! The counter is {=u32}", counter)
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
