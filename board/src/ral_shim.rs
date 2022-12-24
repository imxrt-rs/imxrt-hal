//! imxrt-ral shim module, providing cross-board RTIC support.
//!
//! This shim exists to work around cortex-m-rtic #197 [1]. Before this
//! shim, we maintained unique RTIC app modules per board, just so that
//! we could rename an interrupt. This shim library performs that renaming
//! for us.
//!
//! Boards export an `interrupts` module with certain constants that represent
//! interrupt vector numbers. This module associates those numbers with symbols
//! for interrupt handlers. This allows a symbol like `BOARD_CONSOLE()` to refer
//! to different interrupt numbers, based on the board.
//!
//! [1]: https://github.com/rtic-rs/cortex-m-rtic/issues/197

use crate::board_interrupts::Vector;
use core::cell::UnsafeCell;

pub use imxrt_ral::NVIC_PRIO_BITS;

/// Use this symbol to access the 'DMA_A' channel in the
/// collection of all DMA channels.
pub const BOARD_DMA_A_INDEX: usize = 7;
/// Use this symbol to access the 'DMA_B' channel in the
/// collection of all DMA channels.
pub const BOARD_DMA_B_INDEX: usize = 11;

/// Insert shim vectors into the vector table.
///
/// # Safety
///
/// Only safe when called from the imxrt-hal examples support crate.
/// It might not even be safe there!
pub(crate) unsafe fn shim_vectors() {
    extern "C" {
        // Defined in cortex-m-rt crate. Definitely not marked
        // 'mut', nor does it use UnsafeCell. YOLO.
        // (No one observes the __INTERRUPTS immutable static anyway.)
        static mut __INTERRUPTS: [UnsafeCell<Vector>; 240];
    }

    use crate::board_impl::interrupt::INTERRUPTS;

    cortex_m::interrupt::free(|_| {
        for (nr, isr) in INTERRUPTS {
            __INTERRUPTS[*nr as usize].get().write_volatile(*isr);
        }

        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    });
}
