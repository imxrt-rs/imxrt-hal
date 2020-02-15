//! Parent module for all IMXRT106 devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "imxrt1061", feature = "doc"))]
pub mod imxrt1061;

#[cfg(any(feature = "imxrt1062", feature = "doc"))]
pub mod imxrt1062;

#[cfg(any(feature = "imxrt1064", feature = "doc"))]
pub mod imxrt1064;
