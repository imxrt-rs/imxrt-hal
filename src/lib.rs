//! A hardware abstraction layer (HAL) for i.MX RT processors.
//!
//! `imxrt-hal` provides implementations of the `embedded-hal` traits. It also
//! supports embedded frameworks like RTIC.
//!
//! # Building
//!
//! This package requires that you, or something in your dependency graph,
//! enable a chip-specific feature from the i.MX RT _register access layer
//! (RAL)_, also known as the `imxrt-ral` package.  Without this, the HAL does
//! not build. Since the HAL uses the RAL in its public API, you're expected to
//! depend on both packages.
//!
//! Here's an example of a project that builds the `imxrt-hal` for an i.MX RT
//! 1062 system.
//!
//! ```toml
//! [dependencies.imxrt-hal] # There's no required feature here...
//! version = # ...
//!
//! [dependencies.imxrt-ral]
//! version = # ...
//! features = ["imxrt1062"] # ...but this feature is required.
//! ```
//!
//! Once you've enabled a RAL feature, the HAL builds without any additional
//! features. All APIs exposed in this build are portable across all i.MX RT
//! chips.
//!
//! # Examples
//!
//! See each module's documentation for examples. Note that documentation
//! examples may assume a specific chip and chip family, so you may need to
//! adapt the example for your hardware.
//!
//! The `imxrt-hal` repository maintains examples that run on various i.MX RT
//! development boards. See the project documentation for more information.
//!
//! # Configuration
//!
//! Use these optional package features to control the HAL build.
//!
//! | Feature           | Description                                                      |
//! |-------------------|------------------------------------------------------------------|
//! | `"imxrt1010"`     | Enable features for the 1010 chip family.                        |
//! | `"imxrt1060"`     | Enable features for the 1060 chip family.                        |
//! | `"imxrt1064"`     | Enable features for the 1064 chip family.                        |
//! | `"eh02-unproven"` | Enable implementations for embedded-hal 0.2 `"unproven"` traits. |
//! | `"rand_core"`     | Allows the TRNG to be used with the `rand` package.              |
//!
//! The APIs exposed by the various `"imxrt10xx"` features are specific to a
//! chip family. The HAL does not support building with more than one of these
//! features at a time.
//!
//! When enabling a HAL chip family feature, make sure that it pairs properly
//! with your RAL chip selection. You are responsible for making sure that your
//! RAL chip feature is appropriate for the HAL family feature. For instance,
//! mixing the RAL's `imxrt1062` feature with the HAL's `imxrt1010` feature is
//! not supported.
//!
//! ```toml
//! [dependencies.imxrt-hal]
//! version = # ...
//! #Bad: doesn't support RAL feature.
//! #features = ["imxrt1010"]
//!
//! #Good: supports RAL feature
//! features = ["imxrt1060"]
//!
//! [dependencies.imxrt-ral]
//! version = # ...
//! features = ["imxrt1062"] # Informs the HAL family feature
//! ```
//!
//! The `"eh02-unproven"` feature will not build without the corresponding
//! `"unproven"` feature enabled in embedded-hal 0.2.

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
#[cfg(family = "imxrt10xx")]
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
    pub mod dma;
    pub mod flexpwm;
    pub mod gpio;
    pub mod gpt;
    pub mod lpi2c;
    pub mod lpspi;
    pub mod lpuart;
    pub mod pit;
}

// These common drivers have no associated chip APIs, so
// export them directly.
pub use common::{flexpwm, gpio, gpt, lpspi, lpuart, pit};

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
/// to start a transfer. Since these are futures, you may use these futures in `async` code.
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
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
/// hal::ccm::clock_gate::dma().set(&mut ccm, hal::ccm::clock_gate::ON);
///
/// let mut channels = hal::dma::channels(
///     unsafe { ral::dma::DMA::instance() },
///     unsafe { ral::dmamux::DMAMUX::instance() },
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
/// # async fn a() -> Option<()> {
/// # use imxrt_hal as hal;
/// # use imxrt_ral as ral;
/// # let mut channel = unsafe { hal::dma::channel::Channel::new(13) };
/// let source = [4u32, 5, 6, 7];
/// let mut destination = [0u32; 4];
///
/// let memcpy = hal::dma::memcpy::memcpy(&source, &mut destination, &mut channel);
/// memcpy.await.ok()?;
/// # Some(()) }
/// ```
///
/// For examples of using DMA with a peripheral, see the peripheral's documentation.
pub mod dma {
    pub use crate::chip::dma::*;
    pub use crate::common::dma::*;
}

/// Low-power inter-integrated circuit.
///
/// The `Lpi2c` driver implements all embedded-hal I2C traits. Use these traits to perform
/// common I2C I/O. The driver also exposes lower-level APIs for the LPI2C controller.
///
/// # Example
///
/// Demonstrates how to create an LPI2C peripheral, and perform a write-read with
/// a device. This example skips the LPI2C clock configuration. To understand LPI2C
/// clock configuration, see the [`ccm::lpi2c_clk`](crate::ccm::lpi2c_clk) documentation.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
/// use hal::lpi2c::{self, Lpi2c};
/// use ral::{ccm::CCM, lpi2c::LPI2C3};
/// use eh02::blocking::i2c::WriteRead;
///
/// let mut pads = // Handle to all processor pads...
///     # unsafe { imxrt_iomuxc::imxrt1060::Pads::new() };
///
/// # || -> Option<()> {
/// let mut ccm = unsafe { CCM::instance() };
/// let mut i2c3 = unsafe { LPI2C3::instance() };
///
/// # const LPI2C_CLK_HZ: u32 = 0;
/// # const RUN_MODE: hal::RunMode = hal::RunMode::Overdrive;
/// const LPI2C_400KHz: lpi2c::Timing = lpi2c::timing(lpi2c::ClockSpeed::KHz400, RUN_MODE);
///
/// let mut i2c3 = Lpi2c::new(
///     i2c3,
///     lpi2c::Pins {
///         scl: pads.gpio_ad_b1.p07,
///         sda: pads.gpio_ad_b1.p06,
///     },
///     &LPI2C_400KHz,
/// );
///
/// let mut input = [0; 3];
/// let output = [0x74];
/// # const MY_DEVICE_ADDRESS: u8 = 0;
///
/// i2c3.write_read(MY_DEVICE_ADDRESS, &output, &mut input).ok()?;
/// # Some(()) }();
/// ```
///
/// # Limitations
///
/// This driver supports standard, fast, and fast+ modes. High speed mode is not
/// yet supported, and supporting the mode was not considered in the initial driver
/// design.
pub mod lpi2c {
    pub use crate::chip::lpi2c::*;
    pub use crate::common::lpi2c::*;
}

/// USB device.
///
/// This module re-exports types from the `imxrt-usbd` package. The driver is compatible
///  with the [`usb-device`](https://docs.rs/usb-device/latest/usb_device/) ecosystem.
///
/// # Example
///
/// Construct a [`BusAdapter`](crate::usbd::BusAdapter) with USB peripheral instances. See the
/// [`BusAdapter`](crate::usbd::BusAdapter) documentation for more information on how to use the bus.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// use hal::usbd;
///
/// # || -> Option<()> {
/// let mut usb_analog = unsafe { ral::usb_analog::USB_ANALOG::instance() };
/// let usb_instances = usbd::Instances {
///     usb: unsafe { ral::usb::USB1::instance() },
///     usbnc: unsafe { ral::usbnc::USBNC1::instance() },
///     usbphy: unsafe { ral::usbphy::USBPHY1::instance() },
/// };
///
/// // Prepare the USB clocks.
/// let mut ccm_analog = unsafe { ral::ccm_analog::CCM_ANALOG::instance() };
/// hal::ccm::analog::pll3::restart(&mut ccm_analog);
///
/// # static mut ENDPOINT_MEMORY: [u8; 4] = [0; 4];
/// # let endpoint_memory = unsafe { &mut ENDPOINT_MEMORY };
/// let usbd = usbd::BusAdapter::new(usb_instances, endpoint_memory);
/// # Some(()) }().unwrap();
/// ```
#[cfg(feature = "imxrt-usbd")]
pub use imxrt_usbd as usbd;

#[cfg(family = "imxrt10xx")]
pub use chip::adc;
#[cfg(family = "imxrt10xx")]
pub use chip::dcdc;
#[cfg(family = "imxrt10xx")]
pub use chip::tempmon;
#[cfg(family = "imxrt10xx")]
pub use chip::trng;

#[cfg(family = "imxrt11xx")]
pub use chip::usbphy;

pub mod timer;
