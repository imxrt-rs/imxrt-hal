//! Parent module for all IMXRT105 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature="imxrt1051", feature="doc"))]
pub mod imxrt1051;

#[cfg(any(feature="imxrt1052", feature="doc"))]
pub mod imxrt1052;

