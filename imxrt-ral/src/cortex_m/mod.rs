//! Parent module for all CORTEX_M devices.

/// Peripherals shared by multiple devices
pub mod peripherals;

/// Peripheral instances shared by multiple devices
pub(crate) mod instances;

#[cfg(any(feature = "armv6m", feature = "doc"))]
pub mod armv6m;

#[cfg(any(feature = "armv7em", feature = "doc"))]
pub mod armv7em;

#[cfg(any(feature = "armv7m", feature = "doc"))]
pub mod armv7m;
