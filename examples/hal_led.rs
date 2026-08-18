//! Simply turns on the LED.

#![no_main]
#![no_std]

#[board::entry]
fn main() -> ! {
    let (_, board::Specifics { led, .. }) = board::new();
    loop {
        led.set();
    }
}
