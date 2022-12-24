//! An I2C example that uses the LCD1602 character display.
//!
//! Connect your display to your board's I2C bus. You should see
//! that the backlight turns white, and that a new capital letter
//! is written every 200ms.

#![no_main]
#![no_std]

use board::lcd1602::*;
use imxrt_hal as hal;

const LCD_ADDRESS: u8 = 0x7c >> 1;
const RGB_ADDRESS: u8 = 0xc4 >> 1;

#[imxrt_rt::entry]
fn main() -> ! {
    let (board::Common { gpt1, .. }, board::Specifics { i2c, .. }) = board::new();

    let mut delay = hal::timer::Blocking::<_, { board::GPT1_FREQUENCY }>::from_gpt(gpt1);
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
