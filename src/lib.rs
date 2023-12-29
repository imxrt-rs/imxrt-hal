//! A hardware abstraction layer (HAL) for i.MX RT MCUs.
//!
//! `imxrt-hal` contains a collection of hardware drivers for various i.MX RT
//! MCUs. A default build of `imxrt-hal` provides drivers that are portable
//! across all i.MX RT chips. When your specific chip is known, `imxrt-hal`
//! provides additional, chip-specific APIs. Most drivers implement their
//! corresponding `embedded-hal` traits, or they can be adapted to use
//! `embedded-hal` traits in user code.
//!
//! # Building
//!
//! `imxrt-hal` requires that you, or something in your dependency graph, enable
//! a chip-specific feature from `imxrt-ral`, the i.MX RT _register access layer
//! (RAL)_.  Without this, the HAL does not build. Since the HAL uses the RAL in
//! its public API, you're expected to depend on both packages.
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
//! features. All APIs exposed in this build are portable across all supported
//! i.MX RT chips.
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
//! Use these optional features to control the HAL build.
//!
//! | Feature           | Description                                                      |
//! |-------------------|------------------------------------------------------------------|
//! | `"imxrt1010"`     | Enable features for the 1010 chips.                              |
//! | `"imxrt1020"`     | Enable features for the 1020 chips.                              |
//! | `"imxrt1060"`     | Enable features for the 1060 chips.                              |
//! | `"imxrt1064"`     | Enable features for the 1064 chips.                              |
//! | `"imxrt1170"`     | Enable features for the 1170 chips.                              |
//! | `"eh02-unproven"` | Enable implementations for embedded-hal 0.2 `"unproven"` traits. |
//! | `"rand_core"`     | Allows the TRNG to be used with the `rand` package.              |
//!
//! The APIs exposed by the various `"imxrt[...]"` features are chip specific.
//! The HAL does not support building with more than one of these features at a
//! time.
//!
//! When enabling a HAL chip feature, make sure that it pairs properly with your
//! RAL chip selection. You are responsible for making sure that your RAL chip
//! feature is appropriate for the HAL chip feature. For instance, mixing the
//! RAL's `imxrt1062` feature with the HAL's `imxrt1010` feature is not
//! supported.
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
//! features = ["imxrt1062"] # Informs the HAL chip feature
//! ```
//!
//! The `"eh02-unproven"` feature will not build without the corresponding
//! `"unproven"` feature enabled in embedded-hal 0.2.

#![no_std]
#![warn(missing_docs, unsafe_op_in_unsafe_fn)]

use imxrt_ral as ral;

mod chip;

/// Modules that need no HAL conditional compilation.
///
/// These modules only depend on a RAL feature.
mod common {
    pub use imxrt_dma as dma;

    pub mod ccm;
    pub mod flexpwm;
    pub mod gpio;
    pub mod gpt;
    pub mod lpi2c;
    pub mod lpspi;
    pub mod lpuart;
    pub mod pit;
    pub mod snvs;
    pub mod timer;
}

// These common drivers have no associated chip APIs, so
// export them directly.
pub use common::{flexpwm, gpio, gpt, lpi2c, lpspi, lpuart, pit, snvs, timer};

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
/// # let mut channel = unsafe { hal::dma::DMA.channel(13) };
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
    #[cfg_attr(family = "none", allow(unused_imports))] // Nothing to export in this build.
    pub use crate::chip::dma::*;
    pub use crate::common::dma::*;
}

/// USB device.
///
/// This module re-exports types from the `imxrt-usbd` package. The driver is compatible
///  with the [`usb-device`](https://docs.rs/usb-device/latest/usb_device/) ecosystem.
///
/// It also provides [`Instances`](crate::usbd::Instances), an implementation of `imxrt_usbd::Peripherals` over
/// `imxrt-ral` USB instances.
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
/// static ENDPOINT_MEMORY: usbd::EndpointMemory<2048> = usbd::EndpointMemory::new();
/// static ENDPOINT_STATE: usbd::EndpointState = usbd::EndpointState::max_endpoints();
///
/// let usbd = usbd::BusAdapter::new(usb_instances, &ENDPOINT_MEMORY, &ENDPOINT_STATE);
/// # Some(()) }().unwrap();
/// ```
#[cfg(feature = "imxrt-usbd")]
pub mod usbd {
    pub use imxrt_usbd::*;

    use crate::ral;

    /// Aggregate of `imxrt-ral` USB peripheral instances.
    ///
    /// This takes ownership of USB peripheral instances and implements
    /// [`Peripherals`]. You can use this type to allocate a USB device
    /// driver. See the [module-level documentation](crate::usbd) for an
    /// example.
    pub struct Instances<const N: u8> {
        /// USB core registers.
        pub usb: ral::usb::Instance<N>,
        /// USB non-core registers.
        pub usbnc: ral::usbnc::Instance<N>,
        /// USBPHY registers.
        pub usbphy: ral::usbphy::Instance<N>,
    }

    unsafe impl<const N: u8> Peripherals for Instances<N> {
        fn usb(&self) -> *const () {
            (&*self.usb as *const ral::usb::RegisterBlock).cast()
        }
        fn usbphy(&self) -> *const () {
            (&*self.usbphy as *const ral::usbphy::RegisterBlock).cast()
        }
    }
}

/// Pad muxing and configurations.
///
/// This module re-exports select items from the `imxrt-iomuxc` crate. When a chip feature is enabled, the module also exports
/// chip-specific items, like `into_pads`. Use [`into_pads`](crate::iomuxc::into_pads) to transform the `imxrt-ral` instance(s)
/// into pad objects:
///
/// ```
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
/// let pads = hal::iomuxc::into_pads(iomuxc);
/// ```
///
/// [`Pads`](crate::iomuxc::pads::Pads) exposes all pads as individual, owned objects. Use [`configure`](crate::iomuxc::configure)
/// to specify any pad configurations. Then use the pad object(s) to construct your driver.
pub mod iomuxc {
    #[cfg_attr(family = "none", allow(unused_imports))] // Nothing to export in this build.
    pub use crate::chip::iomuxc::*;
    pub use imxrt_iomuxc::prelude::*;
}

#[cfg_attr(family = "none", allow(unused_imports))] // Nothing to export in this build.
pub use crate::chip::reexports::*;

/// Simply spin on the future.
fn spin_on<F: core::future::Future>(future: F) -> F::Output {
    use core::task::{Context, Poll};

    let waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = core::pin::pin!(future);

    loop {
        if let Poll::Ready(result) = future.as_mut().poll(&mut context) {
            return result;
        }
    }
}
