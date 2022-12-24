//! Demonstrates a blocking SPI peripheral.
//!
//! Connect your SDI and SDO pins together, then run this example.
//! The example prints success / errors to the board's serial console.
//! You should see a 1MHz SPI clock, and that the elements of a write /
//! transfer operation occur within a single low PCS.

#![no_main]
#![no_std]

use imxrt_hal as hal;

use eh02::{
    blocking::serial::Write as _,
    blocking::spi::{Transfer, Write},
};
use hal::lpspi::LpspiError;

const GPT1_DELAY_MS: u32 = board::GPT1_FREQUENCY / 1_000 * 500;
const GPT1_OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR1;

/// Change me to experiment with different word sizes.
/// Valid types: u8, u16, u32.
type Elem = u8;

fn write_error<T>(console: &mut board::Console, result: Result<T, LpspiError>) {
    use hal::lpspi::Direction;
    match result {
        Err(LpspiError::Busy) => {
            console.bwrite_all(b"Error: BUSY\r\n").ok();
        }
        Err(LpspiError::Fifo(Direction::Rx)) => {
            console.bwrite_all(b"Error: RX FIFO\r\n").ok();
        }
        Err(LpspiError::Fifo(Direction::Tx)) => {
            console.bwrite_all(b"Error: TX FIFO\r\n").ok();
        }
        Err(LpspiError::NoData) => {
            console.bwrite_all(b"Error: NO DATA\r\n").ok();
        }
        Err(LpspiError::FrameSize) => {
            console.bwrite_all(b"Error: FRAME SIZE\r\n").ok();
        }
        Ok(_) => {}
    }
}

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common { mut gpt1, .. },
        board::Specifics {
            mut spi,
            mut console,
            ..
        },
    ) = board::new();

    gpt1.set_output_compare_count(GPT1_OCR, GPT1_DELAY_MS);
    gpt1.set_mode(hal::gpt::Mode::Restart);
    gpt1.enable();

    console.bwrite_all(b"Starting example...\r\n").ok();
    loop {
        let data: [Elem; 5] = [0xDE, 0xAD, 0xBE, 0xEF, 0xA5];
        let mut buffer: [Elem; 5] = data;

        while !gpt1.is_elapsed(GPT1_OCR) {}
        gpt1.clear_elapsed(GPT1_OCR);

        console.bwrite_all(b"Transfer... ").ok();
        let result = spi.transfer(&mut buffer);
        if result.is_err() {
            write_error(&mut console, result);
        } else if buffer != data {
            console.bwrite_all(b"Data mismatch\r\n").ok();
        } else {
            console.bwrite_all(b"OK\r\n").ok();
        }

        while !gpt1.is_elapsed(GPT1_OCR) {}
        gpt1.clear_elapsed(GPT1_OCR);

        console.bwrite_all(b"Write... ").ok();
        let result = spi.write(&buffer[..3]);
        if result.is_err() {
            write_error(&mut console, result);
        } else {
            console.bwrite_all(b"OK\r\n").ok();
        }
    }
}
