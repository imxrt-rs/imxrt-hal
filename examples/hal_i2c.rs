//! Demonstrates an I2C master.
//!
//! Requires an MPU9250. The board queries the sensor's WHO_AM_I
//! register using various types of I2C write-read sequences. The
//! clock should be 400KHz.

#![no_main]
#![no_std]

use imxrt_hal as hal;

use eh02::{blocking::i2c, blocking::serial::Write as _};

/// MPU9250 I2C slave address
const SLAVE_ADDRESS: u8 = 0x68;
const WHO_AM_I_REG: u8 = 0x75;
const WHO_AM_I_RESP: u8 = 0x71;

const GPT1_DELAY_MS: u32 = board::GPT1_FREQUENCY / 1_000 * 500;
const GPT1_OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR1;

/// This should show a repeated start.
fn who_am_i_write_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::WriteRead,
{
    let mut out = [0; 1];
    i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I_REG], &mut out)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a stop after the write, then a new start
/// for the read.
fn who_am_i_write_then_read<I, E>(i2c: &mut I) -> Result<bool, E>
where
    I: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    i2c.write(SLAVE_ADDRESS, &[WHO_AM_I_REG])?;
    let mut out = [0; 1];
    i2c.read(SLAVE_ADDRESS, &mut out)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a repeated start when using the transaction API.
fn who_am_i_transaction_write_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::Transactional,
{
    use i2c::Operation;
    let mut out = [0u8; 1];
    let mut ops = [Operation::Write(&[WHO_AM_I_REG]), Operation::Read(&mut out)];
    i2c.exec(SLAVE_ADDRESS, &mut ops)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a stop after the write, then a new start
/// for the read.
fn who_am_i_transaction_write_then_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::Transactional,
{
    use i2c::Operation;
    let mut ops = [Operation::Write(&[WHO_AM_I_REG])];
    i2c.exec(SLAVE_ADDRESS, &mut ops)?;
    let mut out = [0u8; 1];
    ops[0] = Operation::Read(&mut out);
    i2c.exec(SLAVE_ADDRESS, &mut ops)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

fn write_error(console: &mut board::Console, _: hal::lpi2c::MasterStatus) {
    // TODO more helpful error reporting...
    console.bwrite_all(b"I2C error\r\n").ok();
}

fn query_mpu(
    ctx: &[u8],
    console: &mut board::Console,
    func: impl FnOnce() -> Result<bool, hal::lpi2c::MasterStatus>,
) {
    console.bwrite_all(ctx).ok();
    match func() {
        Ok(true) => {
            console.bwrite_all(b"OK\r\n").ok();
        }
        Ok(false) => {
            console.bwrite_all(b"Wrong response\r\n").ok();
        }
        Err(err) => {
            write_error(console, err);
        }
    }
}

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common { mut gpt1, .. },
        board::Specifics {
            led,
            mut console,
            mut i2c,
            ..
        },
    ) = board::new();

    gpt1.set_output_compare_count(GPT1_OCR, GPT1_DELAY_MS);
    gpt1.set_mode(hal::gpt::Mode::Restart);
    gpt1.enable();

    led.set();
    loop {
        while !gpt1.is_elapsed(GPT1_OCR) {}
        gpt1.clear_elapsed(GPT1_OCR);

        query_mpu(
            b"Querying WHO_AM_I with write-read... ",
            &mut console,
            || who_am_i_write_read(&mut i2c),
        );

        query_mpu(
            b"Querying WHO_AM_I with write-then-read... ",
            &mut console,
            || who_am_i_write_then_read(&mut i2c),
        );

        query_mpu(
            b"Querying WHO_AM_I with transactional write-read... ",
            &mut console,
            || who_am_i_transaction_write_read(&mut i2c),
        );

        query_mpu(
            b"Querying WHO_AM_I with transactional write-then-read... ",
            &mut console,
            || who_am_i_transaction_write_then_read(&mut i2c),
        );

        led.toggle();
        console.bwrite_all(b"\r\n\r\n").ok();
    }
}
