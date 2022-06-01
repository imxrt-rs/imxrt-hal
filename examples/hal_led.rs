//! Simply turns on the LED.

#![no_main]
#![no_std]

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Board { led, .. } = board::prepare();
    loop {
        led.set();
    }
}
