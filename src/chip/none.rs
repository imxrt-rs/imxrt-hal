//! The chip API when no chip is selected.

pub mod ccm {
    pub use crate::common::ccm::*;
}
pub mod dma {}
pub(crate) mod iomuxc {}
