//! Demonstrates independent PWM outputs.
//!
//! Each PWM output produces center-aligned pulses at 1KHz.
//! Output A has a 50% duty cycle; output B has half of output
//! A's duty cycle. Every 250ms, the PWM outputs toggle on and
//! off.

#![no_std]
#![no_main]

use imxrt_hal as hal;

const PIT_DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

const _: () = assert!(board::PWM_FREQUENCY / 1000 < i16::MAX as u32);
const SWITCHING_FREQ: i16 = (board::PWM_FREQUENCY / 1000) as i16;

const PWM_A_DUTY: u32 = SWITCHING_FREQ as u32 / 2;
const PWM_B_DUTY: u32 = PWM_A_DUTY / 2;

#[imxrt_rt::entry]
fn main() -> ! {
    let (board::Common { mut pit, .. }, board::Specifics { led, pwm, .. }) = board::new();
    pit.0.set_load_timer_value(PIT_DELAY_MS);
    pit.0.enable();

    let board::Pwm {
        mut module,
        mut submodule,
        outputs: (out_a, out_b),
    } = pwm;

    submodule.set_debug_enable(true);
    submodule.set_wait_enable(true);
    submodule.set_clock_select(hal::flexpwm::ClockSelect::Ipg);
    submodule.set_prescaler(board::PWM_PRESCALER);
    submodule.set_pair_operation(hal::flexpwm::PairOperation::Independent);
    submodule.set_load_mode(hal::flexpwm::LoadMode::reload_full());
    submodule.set_load_frequency(1);
    submodule.set_initial_count(&module, SWITCHING_FREQ / -2i16);
    submodule.set_value(
        hal::flexpwm::FULL_RELOAD_VALUE_REGISTER,
        SWITCHING_FREQ / 2i16,
    );

    out_a.set_turn_on(&submodule, PWM_A_DUTY as i16 / -2i16);
    out_a.set_turn_off(&submodule, PWM_A_DUTY as i16 / 2i16);

    out_b.set_turn_on(&submodule, PWM_B_DUTY as i16 / -2i16);
    out_b.set_turn_off(&submodule, PWM_B_DUTY as i16 / 2i16);

    out_a.set_output_enable(&mut module, true);
    out_b.set_output_enable(&mut module, true);
    submodule.set_load_ok(&mut module);
    submodule.set_running(&mut module, true);

    loop {
        while !pit.0.is_elapsed() {}
        pit.0.clear_elapsed();
        led.toggle();

        let enabled = out_a.output_enable(&module);
        out_a.set_output_enable(&mut module, !enabled);
        out_b.set_output_enable(&mut module, !enabled);
    }
}
