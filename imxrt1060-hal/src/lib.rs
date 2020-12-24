//! NXP iMXRT hardware abstraction layer
//!
//! An [`embedded-hal`](https://crates.io/crates/embedded-hal) implementation
//! targeting processors in NXP's IMXRT1060 family.
//!
//! See the module-level documentation for more information and examples.

#![no_std]

pub use imxrt_ral as ral;

/// Pin muxing and configuration
///
/// See the `imxrt_iomuxc` crate documentation for more information on this module.
/// This module re-exports that crate, along with a chip-specific IOMUXC crate.
pub mod iomuxc {
    pub use imxrt_iomuxc::imxrt1060::*;
    pub use imxrt_iomuxc::*;

    /// Use this function to acquire the IOMUXC pads. It requires that you have an
    /// instance to the RAL's IOMUXC instance.
    pub fn new(_: crate::ral::iomuxc::Instance) -> Pads {
        unsafe { Pads::new() }
    }
}

pub mod adc;
pub mod ccm;
pub mod dma;
pub mod gpio;
pub mod gpt;
pub mod i2c;
// pub mod pit;
// pub mod pwm;
pub mod spi;
// pub mod srtc;
// pub mod trng;
pub mod uart;

// pub mod dcdc {
//     use imxrt_ral as ral;
//     pub struct DCDC(pub(crate) ral::dcdc::Instance);
//     impl DCDC {
//         pub fn raw(&mut self) -> &ral::dcdc::Instance {
//             &self.0
//         }
//     }
// }
