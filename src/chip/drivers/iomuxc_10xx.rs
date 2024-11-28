//! Common IOMUXC driver features for 10xx MCUs.

pub use crate::chip::config::pads;
use crate::ral;

/// Transform the `imxrt-ral` IOMUXC instance into pad objects.
pub fn into_pads(_: ral::iomuxc::IOMUXC) -> pads::Pads {
    // Safety: acquiring pads has the same safety implications
    // as acquiring the IOMUXC instance. The user has already
    // assumed the unsafety.
    unsafe { pads::Pads::new() }
}
