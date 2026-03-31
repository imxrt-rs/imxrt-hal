//! An I2C example that uses the LCD1602 character display.
//!
//! Connect your display to your board's I2C bus. You should see
//! that the backlight turns white, and that a new capital letter
//! is written every 200ms.

#![no_main]
#![no_std]

use board::lcd1602::*;
use imxrt_hal as hal;

use hal::gpt::{Gpt, Mode, OutputCompareRegister};

const LCD_ADDRESS: u8 = 0x7c >> 1;
const RGB_ADDRESS: u8 = 0xc4 >> 1;

/// A blocking delay adapter over a GPT timer.
struct GptDelay {
    gpt: Gpt,
    ticks_per_ms: u32,
}

impl GptDelay {
    fn new(mut gpt: Gpt, frequency_hz: u32) -> Self {
        gpt.disable();
        gpt.clear_rollover();
        gpt.set_rollover_interrupt_enable(false);
        gpt.set_mode(Mode::Restart);
        gpt.set_reset_on_enable(true);
        for ocr in [
            OutputCompareRegister::OCR1,
            OutputCompareRegister::OCR2,
            OutputCompareRegister::OCR3,
        ] {
            gpt.clear_elapsed(ocr);
            gpt.set_output_interrupt_on_compare(ocr, false);
        }
        Self {
            gpt,
            ticks_per_ms: frequency_hz / 1_000,
        }
    }

    fn block_ms(&mut self, ms: u32) {
        let ticks = self.ticks_per_ms.saturating_mul(ms);
        let ocr = OutputCompareRegister::OCR1;
        self.gpt.set_output_compare_count(ocr, ticks);
        self.gpt.enable();
        while !self.gpt.is_elapsed(ocr) {}
        self.gpt.clear_elapsed(ocr);
        self.gpt.disable();
    }
}

impl eh02::blocking::delay::DelayMs<u16> for GptDelay {
    fn delay_ms(&mut self, ms: u16) {
        self.block_ms(ms as u32);
    }
}

#[imxrt_rt::entry]
fn main() -> ! {
    let (board::Common { gpt1, .. }, board::Specifics { i2c, .. }) = board::new();

    let mut delay = GptDelay::new(gpt1, board::GPT1_FREQUENCY);
    let mut lcd = Lcd::new(i2c, LCD_ADDRESS, RGB_ADDRESS, &mut delay).unwrap();

    lcd.set_cursor(Cursor::On).unwrap();
    lcd.set_rgb(255, 255, 255).unwrap();
    lcd.write_str("Hello world!").unwrap();

    let chars = 'A'..='Z';
    let chars = chars.cycle();

    let cursors = (0..2u8)
        .cycle()
        .skip(1)
        .flat_map(|y| (0..16u8).map(move |x| (x, y)));

    for ((x, y), char) in cursors.zip(chars) {
        delay.block_ms(200);
        lcd.set_cursor_position(x, y).unwrap();
        lcd.write_char(char).unwrap();
    }
    unreachable!("The iterator never finishes.");
}
