//! Pre-peripheral clock.

use crate::ral::{self, ccm::CCM};

/// Pre-peripheral clock selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Selection {
    Pll2 = 0,
    Pll2Pfd2 = 1,
    Pll2Pfd0 = 2,
    Pll1 = 3,
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
        1 => Pll2Pfd2,
        2 => Pll2Pfd0,
        3 => Pll1,
        _ => unreachable!(),
    }
}
