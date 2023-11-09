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

/// The USB PLL.
///
/// When an implementation has multiple USB peripherals, this
/// PLL is associated with USB1.
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

    /// Restart the USB(1) PLL.
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
/// The Audio PLL
pub mod pll4 {

    /// The smallest PLL4_PFD divider.
    pub const MIN_FRAC: u8 = 12;
    /// The largest PLL4_PFD divider.
    pub const MAX_FRAC: u8 = 35;

    use crate::{common::ccm, ral};

    /// Post divider options for Pll4
    #[derive(PartialEq, Eq)]
    pub enum PostDivider {
        /// Post divider set to 1
        U1 = 1,
        /// Post divider set to 2
        U2 = 2,
        /// Post divider set to 4
        U4 = 4,
        /// Post divider set to 8
        U8 = 8,
        /// Post divider set to 16
        U16 = 16,
    }

    /// Helper to calculate the output frequency given PLL settings
    fn output_freq(div_select: u32, pll_num: u32, pll_denom: u32) -> u32 {
        // This could be incorrect in some cases where 32 bit integer overflows occur so the math
        // is done in 64bit signed math which will not overflow
        //
        // Worst case is pll_num = pow(2,30)
        // pow(2,30) * 24000000 = 25769803776000000
        // pow(2,63) = 9223372036854775808
        (((ccm::XTAL_OSCILLATOR_HZ * div_select) as u64)
            + ((ccm::XTAL_OSCILLATOR_HZ as u64) * (pll_num as u64)) / (pll_denom as u64))
            as u32
    }

    /// (Re-)configure the Audio PLL where all numerical options are enforced at build time.
    ///
    /// The output frequency of the PLL will be
    /// (fref*(div_select + pll_num/pll_denom))/post_div_select
    ///
    /// Where fref is the reference clock frequency given by [`ccm::XTAL_OSCILLATOR_HZ`]
    pub fn reconfigure(
        ccm_analog: &mut ral::ccm_analog::CCM_ANALOG,
        div_select: u32,
        pll_num: u32,
        pll_denom: u32,
        post_divider: PostDivider,
    ) {
        assert!(
            div_select >= 27 && div_select <= 54,
            "Audio PLL divider selection must be in range from 27 to 54 inclusive"
        );
        assert!(
            (pll_num as u32) <= pll_denom,
            "PLL requires numerator be less than the denominator"
        );
        let out_freq: u32 = output_freq(div_select, pll_num, pll_denom);
        assert!(
            out_freq > 650_000_000 && out_freq <= 1_300_000_000,
            "Maximum PLL4 output range is from 650MHz to 1.3GHz"
        );
        // disable and power down pll
        loop {
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, BYPASS == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_SET, BYPASS: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, ENABLE == 1) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_CLR, ENABLE: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, POWERDOWN == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_SET, POWERDOWN: 1);
                continue;
            }
            break;
        }

        let (post_div, audio_div_msb, audio_div_lsb) = match post_divider {
            PostDivider::U1 => (2, 0, 0),
            PostDivider::U2 => (1, 0, 0),
            PostDivider::U4 => (2, 1, 1),
            PostDivider::U8 => (1, 1, 1),
            PostDivider::U16 => (0, 1, 1),
        };

        // set div_select, num, denom, post_div_select, and audio_div
        ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_NUM, pll_num);
        ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_DENOM, pll_denom);
        ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, DIV_SELECT: div_select);
        ral::modify_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, POST_DIV_SELECT: post_div);
        ral::modify_reg!(ral::ccm_analog, ccm_analog, MISC2, AUDIO_DIV_MSB: audio_div_msb, AUDIO_DIV_LSB: audio_div_lsb);

        // power on, enable, and lock
        loop {
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, ENABLE == 0) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_SET, ENABLE: 1);
                continue;
            }
            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, POWERDOWN == 1) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_CLR, POWERDOWN: 1);
                continue;
            }

            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, LOCK == 0) {
                continue;
            }

            if ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, BYPASS == 1) {
                ral::write_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_CLR, BYPASS: 1);
                continue;
            }
            break;
        }
    }

    /// Get the current clock rate for the Audio PLL by inspecting the dividers
    pub fn frequency() -> u32 {
        // Safety: Only reading registers
        let ccm_analog = unsafe { ral::ccm_analog::CCM_ANALOG::instance() };

        let div_select: u32 = ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO, DIV_SELECT);
        let pll_num: u32 = ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_NUM);
        let pll_denom: u32 = ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_DENOM);
        let post_div_select: u32 = ral::read_reg!(ral::ccm_analog, ccm_analog, PLL_AUDIO_DENOM);
        let (audio_div_msb, audio_div_lsb) = ral::read_reg!(
            ral::ccm_analog,
            ccm_analog,
            MISC2,
            AUDIO_DIV_MSB,
            AUDIO_DIV_LSB
        );

        let post_div = match (post_div_select, audio_div_msb, audio_div_lsb) {
            (2, 0, 0) => 1,
            (1, 0, 0) => 2,
            (2, 1, 1) => 4,
            (1, 1, 1) => 8,
            (0, 1, 1) => 16,
            _ => panic!("Undefined match for post div to calculate pll4 clock rate"),
        };

        output_freq(div_select, pll_num, pll_denom) / post_div
    }
}
