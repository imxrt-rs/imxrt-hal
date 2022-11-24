//! CCM clock outputs.
//!
//! Use functions in [`clko1`] and [`clko2`] to control the
//! output source clock selection. Create an [`Output`] to
//! prepare the pin.
//!
//! Each module has a `set_selection` function so that you
//! can choose your output clock. If you need to divide the
//! clock before it reaches the output, use `set_divider`.
//!
//! The implementation assumes that the output source root clocks
//! are in the unassigned mode.

pub use crate::common::ccm::Output;

//
// Skipping selection getters until they're needed.
//
// See notes in the 10xx module.

use crate::ral::{self, ccm::clockroot};

fn is_enabled(clock_root: &clockroot::RegisterBlock) -> bool {
    ral::read_reg!(clockroot, clock_root, CLOCK_ROOT_STATUS0, OFF == 0)
}

fn enable(clock_root: &clockroot::RegisterBlock, enable: bool) {
    ral::modify_reg!(clockroot, clock_root, CLOCK_ROOT_CONTROL, OFF: !enable as u32);
}

fn set_selection(clock_root: &clockroot::RegisterBlock, selection: u32) {
    ral::modify_reg!(clockroot, clock_root, CLOCK_ROOT_CONTROL, MUX: selection);
}

fn set_divider(clock_root: &clockroot::RegisterBlock, divider: u32) {
    ral::modify_reg!(
        clockroot,
        clock_root,
        CLOCK_ROOT_CONTROL,
        DIV: divider as u32
    );
}

fn divider(clock_root: &clockroot::RegisterBlock) -> u32 {
    ral::read_reg!(clockroot, clock_root, CLOCK_ROOT_CONTROL, DIV) as u32
}

macro_rules! api {
    (clock_root = $clock_root:expr) => {
        use crate::ral::ccm::CCM;

        /// Indicates if output is (`true`) (not, `false`) enabled.
        #[inline]
        pub fn is_enabled(ccm: &CCM) -> bool {
            super::is_enabled(&ccm.CLOCK_ROOT[$clock_root])
        }

        /// Enable (`true`) or disable (`false`) the clock output.
        #[inline]
        pub fn enable(ccm: &mut CCM, enable: bool) {
            super::enable(&ccm.CLOCK_ROOT[$clock_root], enable);
        }

        /// Set the clock selection.
        #[inline]
        pub fn set_selection(ccm: &mut CCM, selection: Selection) {
            super::set_selection(&ccm.CLOCK_ROOT[$clock_root], selection as u32);
        }

        /// Set the clock divider.
        ///
        /// The implementation clamps the value between 1 and 256.
        #[inline]
        pub fn set_divider(ccm: &mut CCM, divider: u32) {
            let divider = divider.clamp(1, 256) - 1;
            super::set_divider(&ccm.CLOCK_ROOT[$clock_root], divider);
        }

        /// Returns the clock divider.
        #[inline]
        pub fn divider(ccm: &CCM) -> u32 {
            super::divider(&ccm.CLOCK_ROOT[$clock_root]) + 1
        }
    };
}

/// CLKO1 output source.
pub mod clko1 {
    pub use crate::chip::config::clko::Clko1Selection as Selection;
    api!(clock_root = 77);
}

/// CLO2 output source.
pub mod clko2 {
    pub use crate::chip::config::clko::Clko2Selection as Selection;
    api!(clock_root = 78);
}
