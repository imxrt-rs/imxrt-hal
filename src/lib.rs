//! Hardware abstraction layer for i.MX RT processors.
//!
//! This package provides i.MX RT drivers that implement `embedded-hal` traits.
//! It supports two `embeddd-hal` versions:
//!
//! - [`embedded-hal` 0.2 (EH02)](https://docs.rs/embedded-hal/0.2/embedded_hal/)
//! - [`embedded-hal` 1 (EH1)](https://docs.rs/embedded-hal/1.0/embedded_hal/)
//!
//! # Building
//!
//! This package requires that you, or another package, enable a chip-specific
//! feature from the i.MX RT _register access layer (RAL)_ package, `imxrt-ral`.
//! Without this, this package does not build.
//!
//! For example, to build the HAL from the command line for a 1062 chip, enable
//! the _RAL's_ `"imxrt1062"` feature:
//!
//! ```text
//! cargo build --features=imxrt-ral/imxrt1062
//! ```
//!
//! # Features
//!
//! Use these optional features to control the HAL build:
//!
//! | Feature           | Description                                                      |
//! |-------------------|------------------------------------------------------------------|
//! | `"eh02-unproven"` | Enable implementations for embedded-hal 0.2 `"unproven"` traits. |
//!
//! The `"eh02-unproven"` feature will not build without the corresponding `"unproven"` feature enabled
//! in embedded-hal 0.2.

#![no_std]

use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

pub mod dcdc;
pub mod gpio;
pub mod gpt;
pub mod lpuart;
pub mod pit;

#[cfg(feature = "imxrt1010")]
#[path = "chip/imxrt1010.rs"]
mod chip;

#[cfg(any(feature = "imxrt1060", feature = "imxrt1064"))]
#[path = "chip/imxrt1060.rs"]
mod chip;

#[cfg(any(feature = "imxrt1010", feature = "imxrt1060", feature = "imxrt1064"))]
pub use chip::*;

/// SOC run mode.
///
/// Each MCU specifies its own core clock speed
/// and power settings for these variants. They're
/// typically follow the recommendations in the
/// data sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RunMode {
    /// The fastest, highest-power mode.
    Overdrive,
}

/// Set the target power for the provided `run_mode`.
pub fn set_target_power(dcdc: &mut ral::dcdc::DCDC, run_mode: RunMode) {
    let millivolts = match run_mode {
        RunMode::Overdrive => 1250,
    };
    dcdc::set_target_vdd_soc(dcdc, millivolts);
}

/// Exports common DMA functions.
///
/// Re-exported through the `dma` module when activated through a chip family feature.
mod common {
    pub mod dma;
}
