//! Modules shared across all i.MX RT 11xx processors.
//!
//! Shared modules may rely on configurations from the `config` module.

pub mod ccm;
#[path = "dma.rs"]
pub mod dma;
#[path = "lpi2c.rs"]
pub mod lpi2c;

pub mod usbphy;

cfg_if::cfg_if! {
    if #[cfg(chip = "imxrt1170")] {
        #[path = "imxrt11xx/imxrt1170.rs"]
        pub(crate) mod config;
    }
}

pub(crate) mod reexports {
    pub use super::usbphy;
}
