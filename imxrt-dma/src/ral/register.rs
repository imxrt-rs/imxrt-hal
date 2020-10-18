//! `register` contains a fork of the `imxrt-ral` crate's `register` module
//!
//! We do not want to take on any dependencies for `imxrt-dma`. However, when this DMA
//! implementation was first prototyped, it relied on `imxrt-ral` components. We simply
//! take those necessary components and add them here.

use core::cell::UnsafeCell;

/// A read-write register of type T.
///
/// Contains one value of type T and provides volatile read/write functions to it.
///
/// # Safety
/// This register should be used where reads and writes to this peripheral register do not
/// lead to memory unsafety. For example, it is a poor choice for a DMA target, but less
/// worrisome for a GPIO output data register.
///
/// Access to this register must be synchronised; if multiple threads (or the main thread and an
/// interrupt service routine) are accessing it simultaneously you may encounter data races.
pub struct RWRegister<T> {
    register: UnsafeCell<T>,
}

impl<T: Copy> RWRegister<T> {
    /// Reads the value of the register.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.register.get()) }
    }

    /// Writes a new value to the register.
    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe { ::core::ptr::write_volatile(self.register.get(), val) }
    }
}

/// A read-only register of type T.
///
/// Contains one value of type T and provides a volatile read function to it.
///
/// # Safety
/// This register should be used where reads and writes to this peripheral register do not
/// lead to memory unsafety.
///
/// Access to this register must be synchronised; if multiple threads (or the main thread and an
/// interrupt service routine) are accessing it simultaneously you may encounter data races.
pub struct RORegister<T> {
    register: UnsafeCell<T>,
}

impl<T: Copy> RORegister<T> {
    /// Reads the value of the register.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.register.get()) }
    }
}

/// A write-only register of type T.
///
/// Contains one value of type T and provides a volatile write function to it.
///
/// # Safety
/// This register should be used where writes to this peripheral register do not lead to memory
/// unsafety.
///
/// Access to this register must be synchronised; if multiple threads (or the main thread and an
/// interrupt service routine) are accessing it simultaneously you may encounter data races.
pub struct WORegister<T> {
    register: UnsafeCell<T>,
}

impl<T: Copy> WORegister<T> {
    /// Writes a new value to the register.
    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe { ::core::ptr::write_volatile(self.register.get(), val) }
    }
}
