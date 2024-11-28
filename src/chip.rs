//! Chip APIs.

#[cfg_attr(chip = "imxrt1010", path = "chip/imxrt1010.rs")]
#[cfg_attr(chip = "imxrt1020", path = "chip/imxrt1020.rs")]
#[cfg_attr(chip = "imxrt1060", path = "chip/imxrt1060.rs")]
#[cfg_attr(chip = "imxrt1170", path = "chip/imxrt1170.rs")]
#[cfg_attr(chip = "none", path = "chip/none.rs")]
pub(crate) mod selection;

pub use selection::*;
