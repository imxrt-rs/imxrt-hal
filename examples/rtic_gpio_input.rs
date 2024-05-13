//! Demonstrates how to react to a GPIO input using RTIC.
//!
//! It uses your board's button to react to falling edges.
//! The LED's state changes every time you press the button.
//! A timer provides a little wait for debounce.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use hal::{gpio::Trigger, timer::BlockingPit};
    use imxrt_hal as hal;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        button: board::Button,
        delay: BlockingPit<2, { board::PIT_FREQUENCY }>,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (
            board::Common {
                pit: (_, _, pit, _),
                ..
            },
            board::Specifics {
                led,
                button,
                mut ports,
                ..
            },
        ) = board::new();
        led.set();
        let delay = BlockingPit::from_pit(pit);
        let button_port = ports.button_mut();
        button_port.set_interrupt(&button, Some(Trigger::FallingEdge));
        (Shared {}, Local { led, button, delay })
    }

    #[task(binds = BOARD_BUTTON, local = [led, delay, button])]
    fn button_press(cx: button_press::Context) {
        let delay = cx.local.delay;
        let led = cx.local.led;
        let button = cx.local.button;

        led.toggle();
        button.clear_triggered();
        delay.block_us(1000);
    }
}
