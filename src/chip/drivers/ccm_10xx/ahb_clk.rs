//! AHB (ARM) clock.
//!
//! The AHB module exposes all selections and dividers that affect
//! the AHB / ARM clock. This includes all clock trees behind the main
//! selector, and the driving PLL.
//!
//! Chip-specific features affect what's exposed from this module.
//!
//! Note: the i.MX RT 1010 processors refer to the AHB clock root as the
//! "core clock root." Nevertheless, the core clock root has an "AHB divider."
//! For consistency, we refer to the 1010's core clock root as the AHB clock
//! root.

use crate::ral::{self, ccm::CCM};

/// Set the AHB divider.
///
/// The implementation clamps `divider` between 1 and 8.
#[inline(always)]
pub fn set_divider(ccm: &mut CCM, divider: u32) {
    let podf = divider.clamp(1, 8) - 1;
    ral::modify_reg!(ral::ccm, ccm, CBCDR, AHB_PODF: podf);
    super::wait_handshake(ccm);
}

/// Returns the AHB divider.
#[inline(always)]
pub fn divider(ccm: &CCM) -> u32 {
    ral::read_reg!(ral::ccm, ccm, CBCDR, AHB_PODF) + 1
}

/// Peripheral clock selection.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Selection {
    /// Derive from PRE_PERIPH_CLK.
    PrePeriphClkSel = 0,
    /// Derive from PERIPH_CLK2.
    PeriphClk2Sel = 1,
}

/// Set the peripheral clock selection.
#[inline(always)]
pub fn set_selection(ccm: &mut CCM, selection: Selection) {
    ral::modify_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK_SEL: selection as u32);
    super::wait_handshake(ccm);
}

/// Returns the peripheral clock selection.
#[inline(always)]
pub fn selection(ccm: &CCM) -> Selection {
    match ral::read_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK_SEL) {
        0 => Selection::PrePeriphClkSel,
        1 => Selection::PeriphClk2Sel,
        _ => unreachable!(),
    }
}
