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

use core::cell::UnsafeCell;
type Vector = unsafe extern "C" fn();

pub use imxrt_ral::NVIC_PRIO_BITS;

/// Use this symbol to access the 'DMA_A' channel in the
/// collection of all DMA channels.
pub const BOARD_DMA_A_INDEX: usize = 7;

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

        fn BOARD_CONSOLE();
        fn BOARD_SPI();
        fn BOARD_PWM();
        fn BOARD_DMA_A();
        fn BOARD_PIT();
        fn BOARD_GPT1();
        fn BOARD_GPT2();
        fn BOARD_USB1();
        fn BOARD_SWTASK0();
    }

    // Are you bringing up a new board? You'll need to add a
    // module called 'interrupt' with some constants. Use another
    // board for reference.
    use crate::board_impl::interrupt;

    cortex_m::interrupt::free(|_| {
        __INTERRUPTS[interrupt::BOARD_CONSOLE as usize]
            .get()
            .write_volatile(BOARD_CONSOLE);
        __INTERRUPTS[interrupt::BOARD_SPI as usize]
            .get()
            .write_volatile(BOARD_SPI);
        __INTERRUPTS[interrupt::BOARD_PWM as usize]
            .get()
            .write_volatile(BOARD_PWM);
        __INTERRUPTS[interrupt::BOARD_DMA_A as usize]
            .get()
            .write_volatile(BOARD_DMA_A);
        __INTERRUPTS[interrupt::BOARD_PIT as usize]
            .get()
            .write_volatile(BOARD_PIT);
        __INTERRUPTS[interrupt::BOARD_GPT1 as usize]
            .get()
            .write_volatile(BOARD_GPT1);
        __INTERRUPTS[interrupt::BOARD_GPT2 as usize]
            .get()
            .write_volatile(BOARD_GPT2);
        __INTERRUPTS[interrupt::BOARD_USB1 as usize]
            .get()
            .write_volatile(BOARD_USB1);
        __INTERRUPTS[interrupt::BOARD_SWTASK0 as usize]
            .get()
            .write_volatile(BOARD_SWTASK0);

        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    });
}
