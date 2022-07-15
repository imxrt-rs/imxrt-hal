//! imxrt-ral shim module, providing cross-board RTIC support.
//!
//! This shim exists to work around cortex-m-rtic #197 [1]. Before this
//! shim, we maintained unique RTIC app modules per board, just so that
//! we could rename an interrupt. This shim library performs that renaming
//! for us.
//!
//! It re-exports all of imxrt-ral, then overrides the Interrupt items
//! It only supports the needs of the imxrt-hal examples.
//!
//! [1]: https://github.com/rtic-rs/cortex-m-rtic/issues/197

use imxrt_ral::Interrupt as ISR;
pub use imxrt_ral::NVIC_PRIO_BITS;

// Represents the 'enum' namespace.
pub use interrupt_shims as Interrupt;
pub use interrupt_shims as interrupt;

type Vector = unsafe extern "C" fn();

// Generated by build.rs.
include!(concat!(env!("OUT_DIR"), "/interrupt_shims.rs"));

/// Use this symbol to access the 'DMA_A' channel in the
/// collection of all DMA channels.
pub const BOARD_DMA_A_INDEX: usize = 7;

/// Insert shim vectors into the vector table.
///
/// # Safety
///
/// Only safe when called from the imxrt-hal examples support crate.
pub(crate) unsafe fn shim_vectors() {
    use core::cell::UnsafeCell;

    extern "C" {
        // Defined in real imxrt-ral crate. Definitely not marked
        // 'mut', nor does it use UnsafeCell. YOLO.
        // (No one observes the __INTERRUPTS immutable static anyway.)
        static __INTERRUPTS: [UnsafeCell<Vector>; 158];
    }

    for (nr, isr) in INT_TO_VEC.iter() {
        __INTERRUPTS
            // Safety: nr provided by imxrt-ral, which ensures that
            // the index is within bounds of this array. extern "C"
            // array size is the largest possible number (based on 1060
            // table size), but the imxrt-ral chip selection still ensures
            // that we'll never reach beyond the max limit supported
            // by a chip.
            .get_unchecked(*nr as usize)
            .get()
            // Safety: memory location is valid, given the linkage and
            // bounds check.
            .write_volatile(*isr);
    }
}
