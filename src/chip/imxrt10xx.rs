//! Modules shared across all i.MX RT 10xx processors.
//!
//! Shared modules may rely on configurations from the `config` module.

pub mod adc;
#[macro_use]
pub mod ccm;
pub mod dcdc;
#[path = "dma.rs"]
pub mod dma;
#[path = "ocotp.rs"]
pub mod ocotp;
pub mod sai;
pub mod tempmon;
pub mod trng;

cfg_if::cfg_if! {
    if #[cfg(chip = "imxrt1010")] {
        #[path = "imxrt10xx/imxrt1010.rs"]
        pub(crate) mod config;
    } else if #[cfg(chip = "imxrt1020")] {
        #[path = "imxrt10xx/imxrt1020.rs"]
        pub(crate) mod config;
    } else if #[cfg(any(chip = "imxrt1060", chip = "imxrt1064"))] {
        #[path = "imxrt10xx/imxrt1060.rs"]
        pub(crate) mod config;
    }
}

pub(crate) mod reexports {
    pub use super::{adc, dcdc, ocotp, sai, tempmon, trng};
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

impl ocotp::Ocotp {
    #[inline(always)]
    fn read_fuse_data(&self) -> u32 {
        crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, READ_FUSE_DATA)
    }

    /// There's no `OUT_STATUS` register on these parts, so
    /// there's DED event to check.
    #[inline(always)]
    fn check_end_fuse_read(&mut self) -> Result<(), ocotp::Error> {
        Ok(())
    }

    /// There's no `OUT_STATUS` register on these parts.
    #[inline(always)]
    fn check_end_fuse_write(&mut self) -> Result<(), ocotp::Error> {
        Ok(())
    }

    const FUSE_ADDRESS_OFFSET: u16 = 0x400;
}
