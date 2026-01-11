//! Demonstrates a blinking LED based on a PIT channel.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use imxrt_hal as hal;
    use imxrt_hal::pit::Channel;

    const PIT_DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;
    const PIT_CHANNEL: Channel = Channel::Chan2;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        pit: hal::pit::Pit,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (board::Common { mut pit, .. }, board::Specifics { led, .. }) = board::new();
        pit.set_interrupt_enable(PIT_CHANNEL, true);
        pit.set_load_timer_value(PIT_CHANNEL, PIT_DELAY_MS);
        pit.enable(PIT_CHANNEL);
        (Shared {}, Local { led, pit })
    }

    #[task(binds = BOARD_PIT, local = [led, pit])]
    fn toggle_led(cx: toggle_led::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;

        led.toggle();
        while pit.is_elapsed(PIT_CHANNEL) {
            pit.clear_elapsed(PIT_CHANNEL);
        }
    }
}
