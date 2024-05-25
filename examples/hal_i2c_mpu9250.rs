//! Demonstrates I2C communication with an MPU9250.
//!
//! Requires an MPU9250. The board queries the sensor's WHO_AM_I
//! register using various types of I2C write-read sequences. The
//! clock should be 400KHz.
//!
//! The example tries to pipeline LPI2C transactions. Success is indicated
//! by not panicking and constant activity on your scope. To understand loop
//! start and end points, probe your LED / another output on your scope;
//! the loop flips the output every iteration.

#![no_main]
#![no_std]

use eh02::blocking::i2c;

/// MPU9250 I2C address
const MPU9250_ADDRESS: u8 = 0x68;
const WHO_AM_I_REG: u8 = 0x75;
const WHO_AM_I_RESP: u8 = 0x71;

/// This should show a repeated start.
fn who_am_i_write_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::WriteRead,
{
    let mut out = [0; 1];
    i2c.write_read(MPU9250_ADDRESS, &[WHO_AM_I_REG], &mut out)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a stop after the write, then a new start
/// for the read.
fn who_am_i_write_then_read<I, E>(i2c: &mut I) -> Result<bool, E>
where
    I: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    i2c.write(MPU9250_ADDRESS, &[WHO_AM_I_REG])?;
    let mut out = [0; 1];
    i2c.read(MPU9250_ADDRESS, &mut out)?;
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
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
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
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
    let mut out = [0u8; 1];
    ops[0] = Operation::Read(&mut out);
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common {
            pit: (mut pit, _, _, _),
            ..
        },
        board::Specifics { led, mut i2c, .. },
    ) = board::new();

    // Delay for scope set up / pin settle time.
    pit.set_load_timer_value(board::PIT_FREQUENCY);
    pit.enable();
    while !pit.is_elapsed() {}
    pit.clear_elapsed();
    pit.disable();

    led.set();
    loop {
        assert!(who_am_i_write_read(&mut i2c).unwrap());
        assert!(who_am_i_write_then_read(&mut i2c).unwrap());
        assert!(who_am_i_transaction_write_read(&mut i2c).unwrap());
        assert!(who_am_i_transaction_write_then_read(&mut i2c).unwrap());

        led.toggle();
    }
}
