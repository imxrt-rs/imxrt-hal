//! Demonstrates a blinking LED based on a PIT channel.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use imxrt_hal as hal;

    const PIT_DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        pit: hal::pit::Pit<2>,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            board::Common {
                pit: (_, _, mut pit, _),
                ..
            },
            board::Specifics { led, .. },
        ) = board::new();
        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(PIT_DELAY_MS);
        pit.enable();
        (Shared {}, Local { led, pit }, init::Monotonics())
    }

    #[task(binds = BOARD_PIT, local = [led, pit])]
    fn toggle_led(cx: toggle_led::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;

        led.toggle();
        while pit.is_elapsed() {
            pit.clear_elapsed();
        }
    }
}
