//! Demonstrates direct usage of the PIT timer.
//!
//! The LED turns on for 250ms, then off for 250ms, repeating.

#![no_std]
#![no_main]

const DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Board {
        led,
        mut pit,
        console,
        mut dma,
        ..
    } = board::prepare();
    pit.0.set_load_timer_value(DELAY_MS);
    pit.0.enable();

    // let mut poller = imxrt_log::log::lpuart(
    //     console,
    //     dma[1].take().unwrap(),
    //     imxrt_log::Interrupts::Disabled,
    //     Default::default(),
    // )
    let mut poller = imxrt_log::defmt::lpuart(
        console,
        dma[1].take().unwrap(),
        imxrt_log::Interrupts::Disabled,
    )
    .unwrap_or_else(|_| panic!("Couldn't prepare LPUART DMA logger"));

    let mut counter = 0;
    loop {
        while !pit.0.is_elapsed() {
            poller.poll()
        }
        pit.0.clear_elapsed();
        led.toggle();
        log::info!("Hello world! This is the count: {counter}");
        defmt::println!("Hello world! This is the count: {=u32}", counter);
        counter += 1;
    }
}
