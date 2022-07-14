//! Demonstrates a blinking LED based on two hardware timers.
//!
//! A GPT1 interrupt turns the LED on, then enables the GPT2 timer.
//! The GPT2 interrupt turns the LED off, then enables the GPT1 timer.
//! This repeats forever.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = true)]
mod app {
    use imxrt_hal as hal;

    const GPT1_DELAY_MS: u32 = board::GPT1_FREQUENCY / 1_000 * 250;
    const GPT2_DELAY_MS: u32 = board::GPT2_FREQUENCY / 1_000 * 250;
    const OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR3;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        led: board::Led,
        gpt1: hal::gpt::Gpt1,
        gpt2: hal::gpt::Gpt2,
    }

    fn init_gpt<const N: u8>(gpt: &mut hal::gpt::Gpt<N>, delay_ticks: u32) {
        gpt.set_output_compare_count(OCR, delay_ticks);
        gpt.set_mode(hal::gpt::Mode::Restart);
        gpt.set_reset_on_enable(true);
        gpt.set_output_interrupt_on_compare(OCR, true);
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Board {
            led,
            mut gpt1,
            mut gpt2,
            ..
        } = board::new(cx.device);
        init_gpt(&mut gpt1, GPT1_DELAY_MS);
        init_gpt(&mut gpt2, GPT2_DELAY_MS);

        gpt1.enable();
        (Shared { led, gpt1, gpt2 }, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }

    #[task(binds = BOARD_GPT1, shared = [led, gpt1, gpt2])]
    fn turn_on(cx: turn_on::Context) {
        let gpt1 = cx.shared.gpt1;
        let gpt2 = cx.shared.gpt2;
        let led = cx.shared.led;

        (gpt1, gpt2, led).lock(|gpt1, gpt2, led| {
            gpt1.clear_elapsed(OCR);
            gpt1.disable();
            led.set();

            while gpt1.is_elapsed(OCR) {}
            gpt2.enable();
        })
    }

    #[task(binds = BOARD_GPT2, shared = [led, gpt1, gpt2])]
    fn turn_off(cx: turn_off::Context) {
        let gpt1 = cx.shared.gpt1;
        let gpt2 = cx.shared.gpt2;
        let led = cx.shared.led;

        (gpt1, gpt2, led).lock(|gpt1, gpt2, led| {
            gpt2.clear_elapsed(OCR);
            gpt2.disable();
            led.clear();

            while gpt2.is_elapsed(OCR) {}
            gpt1.enable();
        })
    }
}
