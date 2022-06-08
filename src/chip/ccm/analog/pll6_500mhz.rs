//! A fixed 500MHz ENET PLL.

use crate::ral::{self, ccm_analog};

/// PLL6 frequency (Hz).
pub const FREQUENCY: u32 = 500_000_000;

/// Restart PLL6.
///
/// PLL6 should not be driving any components
/// when this restart happens. You're responsible
/// for switching over clocks.
///
/// When this function returns, PLL6 is running and
/// stable.
#[inline(always)]
pub fn restart(ccm_analog: &mut ccm_analog::CCM_ANALOG) {
    ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_ENET, POWERDOWN: 1);
    // Clears POWERDOWN from above.
    ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_ENET, ENET_500M_REF_EN: 1);
    while ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_ENET, LOCK == 0) {}
}
