//! Board configurations for `imxrt-hal` examples.

#![no_std]

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

mod rt;

#[cfg(feature = "imxrt1010evk")]
#[path = "imxrt1010evk.rs"]
mod board;

#[cfg(feature = "teensy4")]
#[path = "teensy4.rs"]
mod board;

pub use board::*;

/// Resources available for all boards.
pub struct Board {
    /// GPIO output for the board's LED.
    pub led: Led,
}

#[cfg(imxrt1010)]
use iomuxc::imxrt1010::Pads;

#[cfg(imxrt1060)]
use iomuxc::imxrt1060::Pads;

/// Acquire the IOMUXC pads, returning all pads.
///
/// # Panics
///
/// Panics if the IOMUXC peripheral is already taken.
fn take_iomuxc() -> Pads {
    ral::iomuxc::IOMUXC::take().expect("Unable to take IOMUXC peripheral");
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

/// TODO featureful panic handler
#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
