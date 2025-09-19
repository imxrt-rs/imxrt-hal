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
    let (board::Common { mut pit, .. }, board::Specifics { led, mut pwm, .. }) = board::new();
    pit.0.set_load_timer_value(PIT_DELAY_MS);
    pit.0.enable();

    pwm.set_debug_enable(board::pwm::SM, true);
    pwm.set_wait_enable(board::pwm::SM, true);
    pwm.set_clock_select(board::pwm::SM, hal::flexpwm::ClockSelect::Ipg);
    pwm.set_prescaler(board::pwm::SM, board::PWM_PRESCALER);
    pwm.set_pair_operation(board::pwm::SM, hal::flexpwm::PairOperation::Independent);
    pwm.set_load_mode(board::pwm::SM, hal::flexpwm::LoadMode::reload_full());
    pwm.set_load_frequency(board::pwm::SM, 1);
    pwm.set_initial_count(board::pwm::SM, SWITCHING_FREQ / -2i16);
    pwm.set_value(
        board::pwm::SM,
        hal::flexpwm::FULL_RELOAD_VALUE_REGISTER,
        SWITCHING_FREQ / 2i16,
    );

    pwm.set_turn_on(board::pwm::SM, board::pwm::A, PWM_A_DUTY as i16 / -2i16);
    pwm.set_turn_off(board::pwm::SM, board::pwm::A, PWM_A_DUTY as i16 / 2i16);

    pwm.set_turn_on(board::pwm::SM, board::pwm::B, PWM_B_DUTY as i16 / -2i16);
    pwm.set_turn_off(board::pwm::SM, board::pwm::B, PWM_B_DUTY as i16 / 2i16);

    pwm.set_output_enable(board::pwm::SM.mask(), board::pwm::A);
    pwm.set_output_enable(board::pwm::SM.mask(), board::pwm::B);

    pwm.set_load_ok(board::pwm::SM.mask());
    pwm.set_run(board::pwm::SM.mask());

    loop {
        while !pit.0.is_elapsed() {}
        pit.0.clear_elapsed();
        led.toggle();
    }
}
