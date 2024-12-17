//! Pre-peripheral clock.

use crate::ral::{self, ccm::CCM};

/// Pre-peripheral clock selection.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Selection {
    /// PLL2.
    Pll2 = 0,
    /// PFD3 of PLL3.
    Pll3Pfd3 = 1,
    /// PFD3 of PLL2.
    Pll2Pfd3 = 2,
    /// PLL6.
    Pll6 = 3,
}

/// Set the pre-peripheral clock selection.
#[inline(always)]
pub fn set_selection(ccm: &mut CCM, selection: Selection) {
    ral::modify_reg!(ral::ccm, ccm, CBCMR, PRE_PERIPH_CLK_SEL: selection as u32);
}

/// Returns the pre-peripheral clock selection.
#[inline(always)]
pub fn selection(ccm: &CCM) -> Selection {
    use Selection::*;
    match ral::read_reg!(ral::ccm, ccm, CBCMR, PRE_PERIPH_CLK_SEL) {
        0 => Pll2,
        1 => Pll3Pfd3,
        2 => Pll2Pfd3,
        3 => Pll6,
        _ => unreachable!(),
    }
}
