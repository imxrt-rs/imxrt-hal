//! Demonstrates a GPIO input.
//!
//! It uses your board's button. See the board's documentation
//! to understand if this is a physical button, or if you'll need
//! to build a little circuit.
//!
//! While the button is pressed, the LED turns off. While the button is
//! released, the LED turns on.
//!
//! The board implementation demonstrates how to configure the GPIO
//! input with a pull up / down.

#![no_std]
#![no_main]

#[imxrt_rt::entry]
fn main() -> ! {
    let (_, board::Specifics { button, led, .. }) = board::new();
    loop {
        if button.is_set() {
            led.set();
        } else {
            led.clear();
        }
    }
}
