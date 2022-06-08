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

mod chip;

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

/// Modules that need no HAL conditional compilation.
///
/// These modules only depend on a RAL feature.
mod common {
    pub mod dcdc;
    pub mod dma;
    pub mod gpio;
    pub mod gpt;
    pub mod lpi2c;
    pub mod lpspi;
    pub mod lpuart;
    pub mod pit;
}

// These common drivers have no associated chip APIs, so
// export them directly.
pub use common::{dcdc, gpio, gpt, lpi2c, lpspi, lpuart, pit};

/// Clock control module.
///
/// Unlike other drivers in this package, this module only provides a
/// thin layer over the `imxrt-ral` APIs. It's fairly low level, but
/// more discoverable than the registers and reference manual.
///
/// # Overview
///
/// Use [`clock_gate`](crate::ccm::clock_gate) APIs to enable or disable the clock gates for
/// various peripherals. You'll need to enable clock gates before you
/// start using peripherals.
///
/// Use [`clock_tree`](crate::ccm::clock_tree) APIs to control root clocks based on a run mode.
/// These higher-level APIs are consistent across all chip families.
///
/// The remaining modules provide lower-level APIs for the CCM clock
/// tree. These APIs may not be portable across chip families.
///
/// # Visibility
///
/// If you see items in this module, it's because a chip family feature is
/// enabled in the HAL. These symbols may vary depending on the selected
/// feature.
pub mod ccm {
    pub use crate::chip::ccm::*;
}

/// Direct memory access.
///
/// Use the `dma` APIs to perform memory operations without processor intervention.
/// The API supports the following transfers:
///
/// - peripheral to memory
/// - memory to peripheral
/// - memory to memory
///
/// Peripheral support depends on the peripheral. See your peripheral's API for details.
/// Methods that use DMA are typically prefixed with `dma`.
///
/// DMA transfers are modeled as futures. The examples below demonstrate a simple way
/// to start a transfer, then block until completion. But, since these are futures, you
/// may use these futures in `async` code.
///
/// # DMA channels
///
/// The API provides access to at least 16 DMA channels. If you've enabled an optional chip
/// family feature, this number may change. See [`CHANNEL_COUNT`](crate::dma::CHANNEL_COUNT)
/// for more information.
///
/// # Visibility
///
/// Select items become visible when a chip family feature is enabled.
///
/// # Example
///
/// Use [`channels()`](crate::dma::channels) to access all DMA channels for your processor.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// # fn doc() -> Option<()> {
/// let mut ccm = ral::ccm::CCM::take()?;
/// hal::ccm::clock_gate::dma().set(&mut ccm, hal::ccm::clock_gate::ON);
///
/// let mut channels = hal::dma::channels(
///     ral::dma0::DMA0::take()?,
///     ral::dmamux::DMAMUX::take()?,
/// );
///
/// // Selecting the 13th DMA channel for our examples...
/// let mut channel = channels[13].take()?;
/// # Some(()) }
/// ```
///
/// Construct and poll a [`Memcpy`](crate::dma::memcpy::Memcpy) to
/// perform a memory-to-memory transfer.
///
/// ```no_run
/// # use imxrt_hal as hal;
/// # use imxrt_ral as ral;
/// # let mut channel = unsafe { hal::dma::channel::Channel::new(13) };
/// let source = [4u32, 5, 6, 7];
/// let mut destination = [0u32; 4];
///
/// let memcpy = hal::dma::memcpy::memcpy(&source, &mut destination, &mut channel);
/// pin_utils::pin_mut!(memcpy);
/// // This poll call begins the transfer.
/// let poll = hal::dma::poll_no_wake(memcpy.as_mut());
/// assert!(poll.is_pending());
///
/// // Do other work here while the transfer happens...
///
/// // Wait for the copy to complete.
/// let result = hal::dma::block(memcpy);
/// assert!(result.is_ok());
/// ```
///
/// For examples of using DMA with a peripheral, see the peripheral's documentation.
pub mod dma {
    pub use crate::chip::dma::*;
    pub use crate::common::dma::*;
}
