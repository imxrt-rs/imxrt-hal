//! Analog clock control module.
//!
//! Each module describes a system PLL. The symbols in this module depend on the
//! chip selection.

pub use crate::chip::config::ccm::analog::*;

/// The system PLL.
pub mod pll2 {
    /// PLL2 frequency (Hz).
    ///
    /// The reference manual notes that PLL2 should always run at 528MHz,
    /// so this constant assumes that PLL2's DIV_SELECT field isn't
    /// changed at runtime.
    pub const FREQUENCY: u32 = 528_000_000;

    /// The smallest PLL2_PFD divider.
    pub const MIN_FRAC: u8 = super::pll3::MIN_FRAC;
    /// The largest PLL2_PFD divider.
    pub const MAX_FRAC: u8 = super::pll3::MAX_FRAC;
}

/// The USB1 PLL.
///
/// This PLL is associated with USB1, the primary USB interface.
pub mod pll3 {
    /// PLL3 frequency (Hz).
    ///
    /// The reference manual notes that PLL3 should always run at 480MHz,
    /// so this constant assumes that PLL3's DIV_SELECT field isn't
    /// changed at runtime.
    pub const FREQUENCY: u32 = 480_000_000;

    /// The smallest PLL3_PFD divider.
    pub const MIN_FRAC: u8 = 12;
    /// The largest PLL3_PFD divider.
    pub const MAX_FRAC: u8 = 35;

    use crate::ral;

    /// Restart the USB1 PLL.
    pub fn restart(ccm_analog: &mut ral::ccm_analog::CCM_ANALOG) {
        loop {
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB1, ENABLE == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB1_SET, ENABLE: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB1, POWER == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB1_SET, POWER: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB1, LOCK == 0) {
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB1, BYPASS == 1) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB1_CLR, BYPASS: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_USB1, EN_USB_CLKS == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_USB1_SET, EN_USB_CLKS: 1);
                continue;
            }
            break;
        }
    }
}

/// The USB2 PLL.
///
/// This PLL is associated with USB2, the secondary USB interface.
pub mod pll7 {
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
}
