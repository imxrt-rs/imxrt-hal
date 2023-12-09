//! Modules shared across all i.MX RT 11xx processors.
//!
//! Shared modules may rely on configurations from the `config` module.

pub mod ccm;
#[path = "dma.rs"]
pub mod dma;
pub mod gpc;
pub mod pmu;
pub mod usbphy;

cfg_if::cfg_if! {
    if #[cfg(chip = "imxrt1170")] {
        #[path = "imxrt11xx/imxrt1170.rs"]
        pub(crate) mod config;
    }
}

pub(crate) mod reexports {
    pub use super::{gpc, pmu, usbphy};
}

pub(crate) mod iomuxc {
    pub use super::config::pads;
    use crate::ral;

    /// Transform the `imxrt-ral` IOMUXC instances into pad objects.
    pub fn into_pads(_: ral::iomuxc::IOMUXC, _: ral::iomuxc_lpsr::IOMUXC_LPSR) -> pads::Pads {
        // Safety: acquiring pads has the same safety implications
        // as acquiring the IOMUXC instances. The user has already
        // assumed the unsafety.
        unsafe { pads::Pads::new() }
    }
}
