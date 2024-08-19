//! The chip API when no chip is selected.

pub mod ccm {
    pub use crate::common::ccm::*;
}
pub mod dma {}
pub mod lpi2c {}
pub mod drivers {}

pub(crate) mod reexports {}
pub(crate) mod iomuxc {}
