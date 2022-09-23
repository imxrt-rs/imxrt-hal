//! Chip family APIs.
//!
//! These submodules may vary by chip family.

#[cfg(family = "imxrt10xx")]
#[path = "imxrt10xx"]
mod family {
    pub mod adc;
    pub mod ccm;
    pub mod dcdc;
    #[path = "../dma.rs"]
    pub mod dma;
    pub mod lpi2c;
    pub mod trng;

    #[cfg(chip = "imxrt1010")]
    #[path = "imxrt1010.rs"]
    pub(crate) mod config;

    #[cfg(any(chip = "imxrt1060", chip = "imxrt1064"))]
    #[path = "imxrt1060.rs"]
    pub(crate) mod config;
}

#[cfg(family = "imxrt11xx")]
#[path = "imxrt11xx"]
mod family {
    pub mod ccm {}
    #[path = "../dma.rs"]
    pub mod dma;
    pub mod lpi2c {}

    #[cfg(chip = "imxrt1170")]
    #[path = "imxrt1170.rs"]
    pub(crate) mod config;
}

#[cfg(not(any(family = "imxrt10xx", family = "imxrt11xx")))]
mod family {
    pub mod ccm {}
    pub mod dma {}
    pub mod lpi2c {}
}

pub use family::*;
