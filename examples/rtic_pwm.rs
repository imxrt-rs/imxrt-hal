//! Demonstrates timing interrupts using the PWM peripheral.
//!
//! The PWM peripheral is a fast timer, and you can use it
//! like any other timer peripheral. In this example, a PWM
//! interrupt triggers every 1ms, incrementing a software counter.
//! When the counter reaches 250, the LED toggles, and the counter
//! resets.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use imxrt_hal as hal;

    const _: () = assert!(board::PWM_FREQUENCY / 1000 < i16::MAX as u32);
    const SWITCHING_FREQ: i16 = (board::PWM_FREQUENCY / 1000) as i16;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        pwm: board::pwm::Pwm,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (_, board::Specifics { led, mut pwm, .. }) = board::new();

        pwm.set_debug_enable(board::pwm::SM, true);
        pwm.set_wait_enable(board::pwm::SM, true);
        pwm.set_clock_select(board::pwm::SM, hal::flexpwm::ClockSelect::Ipg);
        pwm.set_prescaler(board::pwm::SM, board::PWM_PRESCALER);
        pwm.set_pair_operation(board::pwm::SM, hal::flexpwm::PairOperation::Independent);
        pwm.set_load_mode(board::pwm::SM, hal::flexpwm::LoadMode::reload_full());
        pwm.set_load_frequency(board::pwm::SM, 1);
        pwm.set_initial_count(board::pwm::SM, i16::MIN);
        pwm.set_value(
            board::pwm::SM,
            hal::flexpwm::ValueRegister::Val1,
            i16::MIN + SWITCHING_FREQ,
        );
        pwm.set_interrupts(board::pwm::SM, hal::flexpwm::Interrupts::COMPARE_VAL1);
        pwm.set_load_ok(board::pwm::SM.mask());
        pwm.set_run(board::pwm::SM.mask());

        (Shared {}, Local { led, pwm })
    }

    #[task(binds = BOARD_PWM, local = [led, pwm, counter: u32 = 0])]
    fn toggle_led(cx: toggle_led::Context) {
        use hal::flexpwm::Status;
        let pwm = cx.local.pwm;
        while pwm.status(board::pwm::SM).intersects(Status::COMPARE_VAL1) {
            pwm.clear_status(board::pwm::SM, Status::COMPARE_VAL1);
        }

        *cx.local.counter += 1;
        if *cx.local.counter == 250 {
            cx.local.led.toggle();
            *cx.local.counter = 0;
        }
    }
}
