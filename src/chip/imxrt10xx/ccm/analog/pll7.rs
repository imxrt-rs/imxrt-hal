//! The USB2 PLL.
//!
//! This PLL is associated with USB2, the secondary USB interface.

/// PLL7 frequency (Hz).
///
/// The reference manual notes that PLL7 should always run at 480MHz,
/// so this constant assumes that PLL7's DIV_SELECT field isn't
/// changed at runtime.
pub const FREQUENCY: u32 = 480_000_000;

/// The smallest PLL7_PFD divider.
pub const MIN_FRAC: u8 = 12;
/// The largest PLL7_PFD divider.
pub const MAX_FRAC: u8 = 35;

use crate::ral;

/// Restart the USB2 PLL.
pub fn restart(ccm_analog: &mut ral::ccm_analog::CCM_ANALOG) {
    loop {
        if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB2, ENABLE == 0) {
            ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB2_SET, ENABLE: 1);
            continue;
        }
        if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB2, POWER == 0) {
            ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB2_SET, POWER: 1);
            continue;
        }
        if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB2, LOCK == 0) {
            continue;
        }
        if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB2, BYPASS == 1) {
            ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB2_CLR, BYPASS: 1);
            continue;
        }
        if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB2, EN_USB_CLKS == 0) {
            ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB2_SET, EN_USB_CLKS: 1);
            continue;
        }
        break;
    }
}
