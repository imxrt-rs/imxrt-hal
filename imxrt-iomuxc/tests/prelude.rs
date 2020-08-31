//! Ensure that the `prelude` re-exports remain backwards compatible.
//!
//! If these tests do not compile, consider the API broken.

#![allow(unused)]

mod iomuxc {
    #[cfg(feature = "imxrt106x")]
    pub use imxrt_iomuxc::imxrt106x::*;
    pub use imxrt_iomuxc::prelude::*;
}

/// Ensure that prelude modules are re-exported as expected
#[test]
fn use_prelude() {
    use iomuxc::{consts, gpio, i2c, pwm, spi, uart};
}

/// Ensure that the imxrt106x modules are re-exported
#[cfg(feature = "imxrt106x")]
#[test]
fn use_imxrt106x() {
    use iomuxc::{ad_b0, ad_b1, b0, b1, emc, sd_b0, sd_b1, ErasedPads, Pads};
}
