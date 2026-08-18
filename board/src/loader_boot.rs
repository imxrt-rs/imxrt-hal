//! Entry for a board that is started by an external loader.
//!
//! Such a board is entered by a call, not by a reset, so the stack pointer,
//! `VTOR`, SysTick and the NVIC all still belong to the loader.
//! `cortex_m_rt::Reset` assumes none of that. [`_loader_entry`], which the
//! generated linker script names as the ELF entry point, takes those over and
//! [`__pre_init`] finishes with the NVIC while interrupts are masked.

/// Vector Table Offset Register (ARMv7-M B3.2.5).
const SCB_VTOR: u32 = 0xE000_ED08;
/// SysTick Control and Status Register (ARMv7-M B3.3.3).
const SYST_CSR: u32 = 0xE000_E010;
/// NVIC Interrupt Clear-Enable Registers, eight words (ARMv7-M B3.4.4).
const NVIC_ICER0: u32 = 0xE000_E180;
/// NVIC Interrupt Clear-Pending Registers, eight words (ARMv7-M B3.4.6).
const NVIC_ICPR0: u32 = 0xE000_E280;

/// The ELF entry point.
///
/// The order matters. Masking interrupts comes first, since everything after it
/// rewrites state the loader is still using. `VTOR` moves before the stack, so a
/// fault while switching stacks reports through our handlers. The stack pointer
/// goes last, because from there on the loader's stack is not ours to use.
///
/// # Safety
///
/// Only ever reached by being named as the ELF entry point, and only valid when
/// the image was loaded to the addresses the linker script describes.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _loader_entry() -> ! {
    core::arch::naked_asm!(
        "cpsid i",
        "ldr r0, ={vtor}",
        "ldr r1, =__vector_table",
        "str r1, [r0]",
        "dsb",
        "isb",
        "ldr r0, =_stack_start",
        "mov sp, r0",
        "b {reset}",
        vtor = const SCB_VTOR,
        reset = sym Reset,
    )
}

unsafe extern "C" {
    /// `cortex-m-rt`'s reset handler. Declared rather than imported, because
    /// the crate only exposes it as a linker symbol.
    fn Reset() -> !;
}

/// Disable everything the loader left running in the NVIC, and stop SysTick.
///
/// [`_loader_entry`] masked interrupts at the core, but once an application
/// unmasks them those sources would fire into our vector table, where they mean
/// something else entirely.
///
/// A board cannot have this and `imxrt-rt`, which defines its own `__pre_init`
/// for FlexRAM and chip-family init that the loader has already done.
///
/// # Safety
///
/// Called by `cortex_m_rt::Reset` alone, once, before `.data` and `.bss` are
/// initialised, so it must not touch statics. Not for application use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pre_init() {
    unsafe {
        core::ptr::write_volatile(SYST_CSR as *mut u32, 0);

        // Eight words covers the 240 external interrupts of any ARMv7-M. Writing
        // a 0 is ignored, so a blanket 0xFFFF_FFFF is safe where fewer exist.
        for word in 0..8 {
            core::ptr::write_volatile((NVIC_ICER0 as *mut u32).add(word), 0xFFFF_FFFF);
            core::ptr::write_volatile((NVIC_ICPR0 as *mut u32).add(word), 0xFFFF_FFFF);
        }
    }
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}
