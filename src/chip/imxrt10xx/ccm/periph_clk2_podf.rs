//! PERIPH_CLK2 divider.
//!
//! All symbols in this module are re-exported into the periph_clk2 module.

use crate::ral::{self, ccm::CCM};

/// Set the peripheral clock 2 divider.
///
/// The implementation clamps `divider` between 1 and 8. You should first switch
/// away the core clock selection before changing this divider.
#[inline(always)]
pub fn set_divider(ccm: &mut CCM, divider: u32) {
    let podf = divider.clamp(1, 8) - 1;
    ral::modify_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK2_PODF: podf);
}

/// Returns the peripheral clock 2 divider.
#[inline(always)]
pub fn divider(ccm: &CCM) -> u32 {
    ral::read_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK2_PODF) + 1
}
