//! Demonstrates a blinking LED based on a PIT channel.

#![no_std]
#![no_main]

#[rtic::app(device = imxrt_ral, peripherals = true)]
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
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Board {
            led,
            pit: (_, _, mut pit, _),
            ..
        } = board::new(cx.device);

        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(PIT_DELAY_MS);
        pit.enable();
        (Shared {}, Local { led, pit }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }

    #[task(binds = PIT, local = [led, pit])]
    fn toggle_led(cx: toggle_led::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;

        led.toggle();
        while pit.is_elapsed() {
            pit.clear_elapsed();
        }
    }
}
