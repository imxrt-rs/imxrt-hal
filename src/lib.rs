//! Hardware abstraction layer for i.MX RT processors.
//!
//! This package provides i.MX RT drivers that implement `embedded-hal` traits.
//! It supports two `embeddd-hal` versions:
//!
//! - [`embedded-hal` 0.2 (EH02)](https://docs.rs/embedded-hal/0.2/embedded_hal/)
//! - [`embedded-hal` 1 (EH10)](https://docs.rs/embedded-hal/1.0/embedded_hal/)
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
//! |-------------------+------------------------------------------------------------------|
//! | `"eh02-unproven"` | Enable implementations for embedded-hal 0.2 `"unproven"` traits. |
//!
//! The `"eh02-unproven"` feature will not build without the corresponding `"unproven"` feature enabled
//! in embedded-hal 0.2.

use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

pub mod gpio;
