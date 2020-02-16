// Copyright 2020 Tom Burdick
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! NXP i.mx rt microcontrollers.
//!
//! When built, you must specify a device feature, such as `imxrt1062`.
//! This will cause all modules in that device's module to be re-exported
//! from the top level, so that for example `imxrt::gpio` will resolve to
//! `imxrt::imxrt1062::gpio`.
//!
//! In the generated documentation, all devices are visible inside their family
//! modules, but when built for a specific device, only that devices' constants
//! will be available.
//!
//! See the
//! [README](https://github.com/imxrt-rs/imxrt-rs/blob/master/README.md)
//! for example usage.

#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

mod register;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;

pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};
pub use crate::register::{UnsafeWORegister, WORegister};
#[cfg(any(
    feature = "doc",
    feature = "armv6m",
    feature = "armv7em",
    feature = "armv7m"
))]
pub mod cortex_m;

#[cfg(feature = "armv6m")]
pub use cortex_m::armv6m::*;

#[cfg(feature = "armv7em")]
pub use cortex_m::armv7em::*;

#[cfg(feature = "armv7m")]
pub use cortex_m::armv7m::*;

#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod imxrt101;

#[cfg(feature = "imxrt1011")]
pub use imxrt101::imxrt1011::*;

#[cfg(feature = "imxrt1015")]
pub use imxrt101::imxrt1015::*;

#[cfg(any(feature = "doc", feature = "imxrt1021"))]
pub mod imxrt102;

#[cfg(feature = "imxrt1021")]
pub use imxrt102::imxrt1021::*;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod imxrt105;

#[cfg(feature = "imxrt1051")]
pub use imxrt105::imxrt1051::*;

#[cfg(feature = "imxrt1052")]
pub use imxrt105::imxrt1052::*;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod imxrt106;

#[cfg(feature = "imxrt1061")]
pub use imxrt106::imxrt1061::*;

#[cfg(feature = "imxrt1062")]
pub use imxrt106::imxrt1062::*;

#[cfg(feature = "imxrt1064")]
pub use imxrt106::imxrt1064::*;
