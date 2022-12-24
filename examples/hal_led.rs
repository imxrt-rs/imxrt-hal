//! Simply turns on the LED.

#![no_main]
#![no_std]

#[imxrt_rt::entry]
fn main() -> ! {
    let (_, board::Specifics { led, .. }) = board::new();
    loop {
        led.set();
    }
}
