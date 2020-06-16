#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Crossbar B Select Register 0
pub mod SEL0 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT0 (refer to Functional Description section for input/output assignment)
    pub mod SEL0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT1 (refer to Functional Description section for input/output assignment)
    pub mod SEL1 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 1
pub mod SEL1 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)
    pub mod SEL2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)
    pub mod SEL3 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 2
pub mod SEL2 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)
    pub mod SEL4 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)
    pub mod SEL5 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 3
pub mod SEL3 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)
    pub mod SEL6 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)
    pub mod SEL7 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 4
pub mod SEL4 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT8 (refer to Functional Description section for input/output assignment)
    pub mod SEL8 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT9 (refer to Functional Description section for input/output assignment)
    pub mod SEL9 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 5
pub mod SEL5 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)
    pub mod SEL10 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)
    pub mod SEL11 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 6
pub mod SEL6 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT12 (refer to Functional Description section for input/output assignment)
    pub mod SEL12 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT13 (refer to Functional Description section for input/output assignment)
    pub mod SEL13 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar B Select Register 7
pub mod SEL7 {

    /// Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)
    pub mod SEL14 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)
    pub mod SEL15 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u16 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Crossbar B Select Register 0
    pub SEL0: RWRegister<u16>,

    /// Crossbar B Select Register 1
    pub SEL1: RWRegister<u16>,

    /// Crossbar B Select Register 2
    pub SEL2: RWRegister<u16>,

    /// Crossbar B Select Register 3
    pub SEL3: RWRegister<u16>,

    /// Crossbar B Select Register 4
    pub SEL4: RWRegister<u16>,

    /// Crossbar B Select Register 5
    pub SEL5: RWRegister<u16>,

    /// Crossbar B Select Register 6
    pub SEL6: RWRegister<u16>,

    /// Crossbar B Select Register 7
    pub SEL7: RWRegister<u16>,
}
pub struct ResetValues {
    pub SEL0: u16,
    pub SEL1: u16,
    pub SEL2: u16,
    pub SEL3: u16,
    pub SEL4: u16,
    pub SEL5: u16,
    pub SEL6: u16,
    pub SEL7: u16,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}

/// Access functions for the XBARB peripheral instance
pub mod XBARB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403c0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in XBARB
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut XBARB_TAKEN: bool = false;

    /// Safe access to XBARB
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if XBARB_TAKEN {
                None
            } else {
                XBARB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to XBARB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if XBARB_TAKEN && inst.addr == INSTANCE.addr {
                XBARB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal XBARB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        XBARB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to XBARB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARB: *const RegisterBlock = 0x403c0000 as *const _;
