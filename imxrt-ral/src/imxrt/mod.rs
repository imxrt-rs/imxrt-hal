//! Parent module for all IMXRT devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "imxrt1011", feature = "doc"))]
pub mod imxrt1011;

#[cfg(any(feature = "imxrt1015", feature = "doc"))]
pub mod imxrt1015;

#[cfg(any(feature = "imxrt1021", feature = "doc"))]
pub mod imxrt1021;

#[cfg(any(feature = "imxrt1051", feature = "doc"))]
pub mod imxrt1051;

#[cfg(any(feature = "imxrt1052", feature = "doc"))]
pub mod imxrt1052;

#[cfg(any(feature = "imxrt1061", feature = "doc"))]
pub mod imxrt1061;

#[cfg(any(feature = "imxrt1062", feature = "doc"))]
pub mod imxrt1062;

#[cfg(any(feature = "imxrt1064", feature = "doc"))]
pub mod imxrt1064;
