//! A loopback device. Send characters, and you should see
//! the exact same characters sent back. The LED toggles for
//! every exchanged character.
//!
//! Baud: 115200bps.

#![no_main]
#![no_std]

use eh02::{
    blocking::serial::Write,
    serial::{Read, Write as NbWrite},
};

/// Change me to affect the kind of serial writes
/// that the example tests. If this is zero, the example
/// uses the non-blocking write. Otherwise, it uses the
/// blocking write of a buffer this size.
const ECHO_RESPONSE_SIZE: usize = 0;

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        _,
        board::Specifics {
            led, mut console, ..
        },
    ) = board::new();
    loop {
        led.toggle();
        let byte = nb::block!(console.read()).unwrap();
        if ECHO_RESPONSE_SIZE == 0 {
            nb::block!(NbWrite::write(&mut console, byte)).unwrap();
        } else {
            let response: [u8; ECHO_RESPONSE_SIZE] = [byte; ECHO_RESPONSE_SIZE];
            console.bwrite_all(&response).unwrap();
        }
    }
}
