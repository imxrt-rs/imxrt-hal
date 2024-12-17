//! CCM clock outputs.
//!
//! Use functions in [`clko1`] and [`clko2`] to control the
//! output source clock selection. Create an [`Output`] to
//! prepare the pin.
//!
//! Each module has a `set_selection` function so that you
//! can choose your output clock. If you need to divide the
//! clock before it reaches the output, use `set_divider`.
//! Note that the CLKO1 **output** can represent the CLK2 selection.

pub use crate::common::ccm::Output;

//
// Skipping selection getters until they're needed.
//
// These functions might need to reside in the chip-specific
// modules so that they can handled reserved / invalid values
// safely. According to the reference manual, the 1010 CLKO2_SEL
// POR value is not a valid value, so we might need to handle
// that...
//

/// CLKO1 output source.
pub mod clko1 {
    pub use crate::chip::config::ccm::clko::Clko1Selection as Selection;
    use crate::ral::{self, ccm::CCM};

    /// Indicates if CLKO1 is (`true`) (not, `false`) enabled.
    #[inline]
    pub fn is_enabled(ccm: &CCM) -> bool {
        ral::read_reg!(ral::ccm, ccm, CCOSR, CLKO1_EN == 1)
    }

    /// Enable (`true`) or disable (`false`) CLKO1.
    #[inline]
    pub fn enable(ccm: &mut CCM, enable: bool) {
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO1_EN: enable as u32);
    }

    /// Set the clock selection.
    #[inline]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO1_SEL: selection as u32);
    }

    /// Set the clock divider.
    ///
    /// The implementation clamps the value between 1 and 8.
    #[inline]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let divider = divider.clamp(1, 8) - 1;
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO1_DIV: divider);
    }

    /// Return the clock divider.
    #[inline]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CCOSR, CLKO1_DIV) + 1
    }

    /// Set CLKO1 outputs selection.
    ///
    /// The CLKO1 output *pin* can represent either CLKO1 or CLKO2.
    /// Use [`set_output`] to set this configuration.
    #[repr(u32)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Output {
        /// Use CLKO1's selection.
        Clko1 = 0,
        /// Use CLKO2's selection.
        Clko2 = 1,
    }

    /// Set the output selection.
    #[inline]
    pub fn set_output(ccm: &mut CCM, output: Output) {
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLK_OUT_SEL: output as u32)
    }
}

/// CLKO2 output source.
pub mod clko2 {
    pub use crate::chip::config::ccm::clko::Clko2Selection as Selection;
    use crate::ral::{self, ccm::CCM};

    /// Indicates if CLKO2 is (`true`) (not, `false`) enabled.
    #[inline]
    pub fn is_enabled(ccm: &CCM) -> bool {
        ral::read_reg!(ral::ccm, ccm, CCOSR, CLKO2_EN == 1)
    }

    /// Enable (`true`) or disable (`false`) CLKO2.
    #[inline]
    pub fn enable(ccm: &mut CCM, enable: bool) {
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO2_EN: enable as u32);
    }

    /// Set the clock selection.
    #[inline]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO2_SEL: selection as u32);
    }

    /// Set the clock divider.
    ///
    /// The implementation clamps the value between 1 and 8.
    #[inline]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let divider = divider.clamp(1, 8) - 1;
        ral::modify_reg!(ral::ccm, ccm, CCOSR, CLKO2_DIV: divider);
    }

    /// Return the clock divider.
    #[inline]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CCOSR, CLKO2_DIV) + 1
    }
}
