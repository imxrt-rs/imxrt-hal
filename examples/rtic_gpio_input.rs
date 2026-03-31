//! Demonstrates how to react to a GPIO input using RTIC.
//!
//! It uses your board's button to react to falling edges.
//! The LED's state changes every time you press the button.
//! A PIT channel provides a little wait for debounce.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use hal::gpio::Trigger;
    use hal::pit::{Channel, Pit};
    use imxrt_hal as hal;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        button: board::Button,
        pit: Pit,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (
            board::Common { pit, .. },
            board::Specifics {
                led,
                button,
                mut ports,
                ..
            },
        ) = board::new();
        led.set();
        let button_port = ports.button_mut();
        button_port
            .set_interrupt(&button, Some(Trigger::FallingEdge))
            .unwrap();
        (Shared {}, Local { led, button, pit })
    }

    #[task(binds = BOARD_BUTTON, local = [led, pit, button])]
    fn button_press(cx: button_press::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;
        let button = cx.local.button;

        led.toggle();
        button.clear_triggered();

        // Simple blocking delay for debounce using PIT channel 2.
        let channel = Channel::Chan2;
        let ticks = board::PIT_FREQUENCY / 1_000; // 1 ms
        pit.set_load_timer_value(channel, ticks);
        pit.enable(channel);
        while !pit.is_elapsed(channel) {}
        pit.clear_elapsed(channel);
        pit.disable(channel);
    }
}
