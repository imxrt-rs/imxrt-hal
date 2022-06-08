//! The ARM PLL.
//!
//! When available, `pll1` is always divided by the CCM ARM divider.

use crate::ral::{self, ccm_analog};

/// Restart PLL1 with a new divider selection.
///
/// PLL1 should not be driving any components when
/// this restart happens. You're responsible for
/// switching over clocks.
///
/// The implementation clamps `div_sel` between 54 and 108.
///
/// When this function returns, PLL1 is running and stable.
#[inline(always)]
pub fn restart(ccm_analog: &mut ccm_analog::CCM_ANALOG, div_sel: u32) {
    // Restart PLL1.
    ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_ARM, POWERDOWN: 1);
    // Clears POWERDOWN bit from above.
    ral::write_reg!(
        ral::ccm_analog,
        ccm_analog,
        PLL_ARM,
        DIV_SELECT: div_sel.clamp(54, 108)
    );
    ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_ARM_SET, ENABLE: 1);
    // Wait for lock...
    while ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_ARM, LOCK == 0) {}
}

/// Compute the PLL1 frequency (Hz) for a `DIV_SEL` value.
pub const fn frequency(div_sel: u32) -> u32 {
    crate::ccm::XTAL_OSCILLATOR_HZ * div_sel / 2
}
