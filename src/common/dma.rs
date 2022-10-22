//! Direct Memory Access (DMA) driver for i.MX RT processors.
//!
//! This module provides
//!
//! - an unsafe API for defining and scheduling transfers with DMA `Channel`s
//!   and `Transfer`s
//! - safe DMA futures for memcpy, peripheral-to-memory, and memory-to-peripheral
//!   transfers
//!
//! This DMA driver may be re-exported from a HAL. If it is, you should consider
//! using the safer APIs provided by your HAL.
//!
//! # Portability
//!
//! This DMA driver works across all considered i.MX RT variants (1010 and 1060
//! family). You must make sure that the DMA channel you're creating is valid for
//! your i.MX RT processor. This only matters on i.MX RT 1010 processors, which
//! only support 16 DMA channels. Creating an invalid channel for your 1010 processor
//! will result in a channel that references reserved memory.

pub mod channel;
mod element;
mod error;
mod interrupt;
pub mod memcpy;
pub mod peripheral;
mod ral;

pub use element::Element;
pub use error::Error;
pub use interrupt::{on_error, on_interrupt, Transfer};
pub use ral::tcd::BandwidthControl;

/// A DMA result
pub type Result<T> = core::result::Result<T, Error>;
