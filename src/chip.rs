//! Chip family APIs.
//!
//! These submodules may vary by chip family.

#[cfg(family = "imxrt10xx")]
#[path = "imxrt10xx"]
mod family {
    pub mod adc;
    pub mod ccm;
    pub mod dcdc;
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

#[cfg(not(any(family = "imxrt10xx")))]
mod family {
    pub mod ccm {}
    pub mod dma {}
    pub mod lpi2c {}
}

pub use family::*;
