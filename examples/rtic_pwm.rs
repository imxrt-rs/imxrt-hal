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
        submodule: board::pwm::Submodule,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            _,
            board::Specifics {
                led,
                pwm:
                    board::Pwm {
                        mut module,
                        mut submodule,
                        ..
                    },
                ..
            },
        ) = board::new();

        submodule.set_debug_enable(true);
        submodule.set_wait_enable(true);
        submodule.set_clock_select(hal::flexpwm::ClockSelect::Ipg);
        submodule.set_prescaler(board::PWM_PRESCALER);
        submodule.set_pair_operation(hal::flexpwm::PairOperation::Independent);
        submodule.set_load_mode(hal::flexpwm::LoadMode::reload_full());
        submodule.set_load_frequency(1);
        submodule.set_initial_count(&module, i16::MIN);
        submodule.set_value(hal::flexpwm::ValueRegister::Val1, i16::MIN + SWITCHING_FREQ);
        submodule.set_interrupts(hal::flexpwm::Interrupts::COMPARE_VAL1);
        submodule.set_load_ok(&mut module);
        submodule.set_running(&mut module, true);

        (Shared {}, Local { led, submodule }, init::Monotonics())
    }

    #[task(binds = BOARD_PWM, local = [led, submodule, counter: u32 = 0])]
    fn toggle_led(cx: toggle_led::Context) {
        use hal::flexpwm::Status;
        while cx.local.submodule.status().intersects(Status::COMPARE_VAL1) {
            cx.local.submodule.clear_status(Status::COMPARE_VAL1);
        }

        *cx.local.counter += 1;
        if *cx.local.counter == 250 {
            cx.local.led.toggle();
            *cx.local.counter = 0;
        }
    }
}
