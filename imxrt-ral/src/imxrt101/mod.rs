//! Parent module for all IMXRT101 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "imxrt1011", feature = "doc"))]
pub mod imxrt1011;

#[cfg(any(feature = "imxrt1015", feature = "doc"))]
pub mod imxrt1015;
