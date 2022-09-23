//! Demonstrates a blocking SPI master.
//!
//! Connect your SDI and SDO pins together, then run this example.
//! The example prints success / errors to the board's serial console.
//! You should see a 1MHz SPI clock, and that the elements of a write /
//! transfer operation occur within a single low PCS.

#![no_main]
#![no_std]

use imxrt_hal as hal;

use eh1 as embedded_hal;

use embedded_hal::{
    serial::blocking::Write as _,
    spi::blocking::{Transfer, Write},
};
use hal::lpspi::LpspiMasterError;

const GPT1_DELAY_MS: u32 = board::GPT1_FREQUENCY / 1_000 * 500;
const GPT1_OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR1;

/// Change me to experiment with different word sizes.
/// Valid types: u8, u16, u32.
type Elem = u8;

fn write_error(console: &mut board::Console, result: Result<(), LpspiMasterError>) {
    use hal::lpspi::Direction;
    match result {
        Err(LpspiMasterError::Busy) => {
            console.write(b"Error: BUSY\r\n").ok();
        }
        Err(LpspiMasterError::Fifo(Direction::Rx)) => {
            console.write(b"Error: RX FIFO\r\n").ok();
        }
        Err(LpspiMasterError::Fifo(Direction::Tx)) => {
            console.write(b"Error: TX FIFO\r\n").ok();
        }
        Err(LpspiMasterError::NoData) => {
            console.write(b"Error: NO DATA\r\n").ok();
        }
        Err(LpspiMasterError::FrameSize) => {
            console.write(b"Error: FRAME SIZE\r\n").ok();
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

    console.write(b"Starting example...\r\n").ok();
    loop {
        let data: [Elem; 5] = [0xDE, 0xAD, 0xBE, 0xEF, 0xA5];
        let mut buffer: [Elem; 5] = data;

        while !gpt1.is_elapsed(GPT1_OCR) {}
        gpt1.clear_elapsed(GPT1_OCR);

        console.write(b"Transfer... ").ok();
        let result = spi.transfer(&mut buffer);
        if result.is_err() {
            write_error(&mut console, result);
        } else if buffer != data {
            console.write(b"Data mismatch\r\n").ok();
        } else {
            console.write(b"OK\r\n").ok();
        }

        while !gpt1.is_elapsed(GPT1_OCR) {}
        gpt1.clear_elapsed(GPT1_OCR);

        console.write(b"Write... ").ok();
        let result = spi.write(&buffer[..3]);
        if result.is_err() {
            write_error(&mut console, result);
        } else {
            console.write(b"OK\r\n").ok();
        }
    }
}
