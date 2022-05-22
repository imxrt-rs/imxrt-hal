//! ARM divider.
//!
//! This divides the output from the AHB PLL (either PLL1 or PLL6, depending
//! on the system).

use crate::ral::{self, ccm::CCM};

/// Set the ARM divider.
///
/// The implementation clamps `divider` between 1 and 8.
#[inline(always)]
pub fn set_divider(ccm: &mut CCM, divider: u32) {
    let podf = divider.clamp(1, 8) - 1;
    ral::modify_reg!(ral::ccm, ccm, CACRR, ARM_PODF: podf);
    crate::ccm::wait_handshake(ccm);
}

/// Returns the ARM divider.
#[inline(always)]
pub fn divider(ccm: &CCM) -> u32 {
    ral::read_reg!(ral::ccm, ccm, CACRR, ARM_PODF) + 1
}
