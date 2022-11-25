//! Modules shared across all i.MX RT 10xx processors.
//!
//! Shared modules may rely on configurations from the `config` module.

use crate::{ral, RunMode};

pub mod adc;
pub mod ccm;
pub mod dcdc;
#[path = "dma.rs"]
pub mod dma;
#[path = "lpi2c.rs"]
pub mod lpi2c;
pub mod tempmon;
pub mod trng;

cfg_if::cfg_if! {
    if #[cfg(chip = "imxrt1010")] {
        #[path = "imxrt10xx/imxrt1010.rs"]
        pub(crate) mod config;
    } else if #[cfg(any(chip = "imxrt1060", chip = "imxrt1064"))] {
        #[path = "imxrt10xx/imxrt1060.rs"]
        pub(crate) mod config;
    }
}

/// Set the target power for the provided `run_mode`.
pub fn set_target_power(dcdc: &mut ral::dcdc::DCDC, run_mode: RunMode) {
    let millivolts = match run_mode {
        RunMode::Overdrive => 1250,
    };
    dcdc::set_target_vdd_soc(dcdc, millivolts);
}

pub(crate) mod reexports {
    pub use super::{adc, dcdc, set_target_power, tempmon, trng};
}

pub(crate) mod iomuxc {
    pub use super::config::pads;
    use crate::ral;

    /// Transform the `imxrt-ral` IOMUXC instance into pad objects.
    pub fn into_pads(_: ral::iomuxc::IOMUXC) -> pads::Pads {
        // Safety: acquiring pads has the same safety implications
        // as acquiring the IOMUXC instance. The user has already
        // assumed the unsafety.
        unsafe { pads::Pads::new() }
    }
}
