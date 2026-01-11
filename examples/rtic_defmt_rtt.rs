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
    use imxrt_hal::pit::Channel;

    const PIT_CHANNEL: Channel = Channel::Chan2;
    //
    // End configurations.
    //

    #[local]
    struct Local {
        /// Toggle when we make a log.
        led: board::Led,
        /// This timer tells us how frequently to generate
        /// logs. It's always used.
        pit: hal::pit::Pit,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut cortex_m = cx.core;
        let (board::Common { mut pit, .. }, board::Specifics { led, .. }) = board::new();
        cortex_m.DCB.enable_trace();
        cortex_m::peripheral::DWT::unlock();
        cortex_m.DWT.enable_cycle_counter();

        pit.set_load_timer_value(PIT_CHANNEL, MAKE_LOG_INTERVAL_MS);
        pit.set_interrupt_enable(PIT_CHANNEL, true);
        pit.enable(PIT_CHANNEL);

        (Shared {}, Local { led, pit })
    }

    #[task(binds = BOARD_PIT, local = [led, pit, counter: u32 = 0], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            pit, led, counter, ..
        } = cx.local;

        // Is it time for us to send a new log message?
        if pit.is_elapsed(PIT_CHANNEL) {
            led.toggle();
            while pit.is_elapsed(PIT_CHANNEL) {
                pit.clear_elapsed(PIT_CHANNEL);
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
