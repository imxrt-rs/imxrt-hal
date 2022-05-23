//! A RAL-like module to support DMA register access
//!
//! The RAL has TONS of symbols for DMA. The script that auto-generates
//! the RAL from a SVD file doesn't represent register clusters as an array
//! of structs. The transfer control descriptions, in particularly, could
//! conveniently be represented by 32 TCD structs. Same with the multiplexer
//! registers. Same with the priority registers...
//!
//! This module lets us hit those ideals. At the same time, we can expose an
//! interface that lets us use the RAL macros, where applicable.

#![allow(non_snake_case)] // Compatibility with RAL

pub mod dma;
pub mod dmamux;
pub mod tcd;

pub use imxrt_ral::{modify_reg, read_reg, write_reg};
use imxrt_ral::{RORegister, RWRegister, WORegister};

/// Address to the DMA multiplexer registers
const DMA_MULTIPLEXER_ADDRESS: *const u32 = 0x400E_C000 as *const _;
/// Address to the DMA peripheral registers
const DMA_ADDRESS: *const u32 = 0x400E_8000 as *const _;

pub(super) const MULTIPLEXER: Static<dmamux::RegisterBlock> =
    Static(DMA_MULTIPLEXER_ADDRESS as *const _);

pub(super) const DMA: Static<dma::RegisterBlock> = Static(DMA_ADDRESS as *const _);

//
// Helper types for static memory
//
// Similar to the RAL's `Instance` type, but more copy.
//

pub(super) struct Static<T>(*const T);
impl<T> core::ops::Deref for Static<T> {
    type Target = T;
    fn deref(&self) -> &'static Self::Target {
        // Safety: pointer points to static memory (peripheral memory)
        unsafe { &*self.0 }
    }
}
impl<T> Clone for Static<T> {
    fn clone(&self) -> Self {
        Static(self.0)
    }
}
impl<T> Copy for Static<T> {}
