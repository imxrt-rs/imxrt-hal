//! Chip family APIs.
//!
//! This module is always compiled, but its contents vary based on
//! the chip family selection. The submodules are "configured" by
//! a [`config`] module. These modules may use conditional compilation, and
//! reference the [`config`] module, and re-export [`config`] module symbols.

#[cfg(any(family = "imxrt1010", family = "imxrt1060", family = "imxrt1064",))]
pub mod ccm;

#[cfg(not(any(family = "imxrt1010", family = "imxrt1060", family = "imxrt1064",)))]
pub mod ccm {}

#[cfg(family = "imxrt1010")]
#[path = "chip/imxrt1010.rs"]
mod config;

#[cfg(any(family = "imxrt1060", family = "imxrt1064"))]
#[path = "chip/imxrt1060.rs"]
mod config;

#[cfg(any(family = "imxrt1010", family = "imxrt1060", family = "imxrt1064",))]
pub mod dma;

#[cfg(not(any(family = "imxrt1010", family = "imxrt1060", family = "imxrt1064",)))]
pub mod dma {}
