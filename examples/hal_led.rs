//! Simply turns on the LED.

#![no_main]
#![no_std]

#[cortex_m_rt::entry]
fn main() -> ! {
    let (_, board::Specifics { led, .. }) = board::new();
    loop {
        led.set();
    }
}
