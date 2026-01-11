//! Demonstrates direct usage of the PIT timer.
//!
//! The LED turns on for 250ms, then off for 250ms, repeating.

#![no_std]
#![no_main]

use imxrt_hal::pit::Channel;

const DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

#[imxrt_rt::entry]
fn main() -> ! {
    let (board::Common { mut pit, .. }, board::Specifics { led, .. }) = board::new();

    pit.set_load_timer_value(Channel::Chan0, DELAY_MS);
    pit.enable(Channel::Chan0);

    loop {
        while !pit.is_elapsed(Channel::Chan0) {}
        pit.clear_elapsed(Channel::Chan0);
        led.toggle();
    }
}
