/// CAN clock root
pub mod can_clk {
    use crate::ral::{self, ccm::CCM};

    /// Returns the can clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CSCMR2, CAN_CLK_PODF) + 1
    }

    /// The smallest flexcan clock divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest flexcan clock divider.
    pub const MAX_DIVIDER: u32 = 64;

    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CSCMR2, CAN_CLK_PODF: podf);
    }

    /// Flexcan clock selections.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Selection {
        /// Derive from pll3_sw_clk divided clock (60M)
        PLL3Div8 = 0,
        /// Derive from the crystal oscillator.
        Oscillator = 1,
        /// Derive from pll3_sw_clk divided clock (80M)
        PLL3Div6 = 2,
        /// Disable flexcan clock
        Disable = 3,
    }

    /// Returns the Flexcan clock selection.
    #[inline(always)]
    pub fn selection(ccm: &CCM) -> Selection {
        match ral::read_reg!(ral::ccm, ccm, CSCMR2, CAN_CLK_SEL) {
            0 => Selection::PLL3Div8,
            1 => Selection::Oscillator,
            2 => Selection::PLL3Div6,
            3 => Selection::Disable,
            _ => unreachable!(),
        }
    }

    /// Set the Flexcan clock selection.
    #[inline(always)]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CSCMR2, CAN_CLK_SEL: selection as u32);
    }
}
