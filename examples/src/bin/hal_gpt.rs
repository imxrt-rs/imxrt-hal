//! General purpose timer (GPT) example.
//!
//! The LED is on for 250ms, then off for 250ms, then
//! on for 250ms, then off for 250ms...

#![no_std]
#![no_main]

use imxrt_hal as hal;

/// CHANGE ME to select either GPT1 or GPT2.
const USE_GPT1: bool = true;

const GPT1_DELAY_MS: u32 = board::GPT1_FREQUENCY / 1_000 * 250;
const GPT2_DELAY_MS: u32 = board::GPT2_FREQUENCY / 1_000 * 250;

/// Note: this must remain as OCR1 for this example.
/// The example uses restart mode, which expects the counter
/// to reset after a compare event.
const OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR1;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Board {
        led, gpt1, gpt2, ..
    } = board::prepare();
    if USE_GPT1 {
        use_gpt(led, gpt1, GPT1_DELAY_MS);
    } else {
        use_gpt(led, gpt2, GPT2_DELAY_MS);
    }
}

fn use_gpt<const N: u8>(led: board::Led, mut gpt: hal::gpt::Gpt<N>, delay_ticks: u32) -> ! {
    gpt.set_output_compare_count(OCR, delay_ticks);
    // When OCR1 compares, restart the counter.
    gpt.set_mode(hal::gpt::Mode::Restart);
    gpt.enable();

    loop {
        while !gpt.is_elapsed(OCR) {}
        gpt.clear_elapsed(OCR);
        led.toggle();
    }
}
