//! A loopback device. Send characters, and you should see
//! the exact same characters sent back. The LED toggles for
//! every exchanged character.
//!
//! Baud: 115200bps.

#![no_main]
#![no_std]

use embedded_hal::serial::{
    blocking::Write,
    nb::{Read, Write as NbWrite},
};

/// Change me to affect the kind of serial writes
/// that the example tests. If this is zero, the example
/// uses the non-blocking write. Otherwise, it uses the
/// blocking write of a buffer this size.
const ECHO_RESPONSE_SIZE: usize = 0;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Board {
        led, mut console, ..
    } = board::prepare();
    loop {
        led.toggle();
        let byte = embedded_hal::nb::block!(console.read()).unwrap();
        if ECHO_RESPONSE_SIZE == 0 {
            embedded_hal::nb::block!(NbWrite::write(&mut console, byte)).unwrap();
        } else {
            let response: [u8; ECHO_RESPONSE_SIZE] = [byte; ECHO_RESPONSE_SIZE];
            Write::write(&mut console, &response).unwrap();
        }
    }
}
