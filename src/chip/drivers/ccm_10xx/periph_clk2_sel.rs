//! Peripheral clock 2 selection.
//!
//! Dividers are chip specific, and may not exist.

use crate::ral::{self, ccm::CCM};

/// Peripheral CLK2 selection.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Selection {
    /// Use PLL3 (possibly bypassed by an upstream mux).
    Pll3Sw = 0,
    /// Crystall oscillator.
    Osc = 1,
    /// The PLL2 bypass.
    Pll2Bypass = 2,
}

/// Set the peripheral clock2 selection.
#[inline(always)]
pub fn set_selection(ccm: &mut CCM, selection: Selection) {
    ral::modify_reg!(ral::ccm, ccm, CBCMR, PERIPH_CLK2_SEL: selection as u32);
    crate::ccm::wait_handshake(ccm);
}

/// Returns the peripheral clock2 selection.
#[inline(always)]
pub fn selection(ccm: &CCM) -> Selection {
    let raw = ral::read_reg!(ral::ccm, ccm, CBCMR, PERIPH_CLK2_SEL);
    match raw {
        0 => Selection::Pll3Sw,
        1 => Selection::Osc,
        2 => Selection::Pll2Bypass,
        _ => unreachable!(),
    }
}
