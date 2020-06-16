//! Parent module for all IMXRT102 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "imxrt1021", feature = "doc"))]
pub mod imxrt1021;
