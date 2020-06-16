#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EWM

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control Register
pub mod CTRL {

    /// EWM enable.
    pub mod EWMEN {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EWM_in's Assertion State Select.
    pub mod ASSIN {
        /// Offset (1 bits)
        pub const offset: u8 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Enable.
    pub mod INEN {
        /// Offset (2 bits)
        pub const offset: u8 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt Enable.
    pub mod INTEN {
        /// Offset (3 bits)
        pub const offset: u8 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u8 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Service Register
pub mod SERV {

    /// SERVICE
    pub mod SERVICE {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compare Low Register
pub mod CMPL {

    /// COMPAREL
    pub mod COMPAREL {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compare High Register
pub mod CMPH {

    /// COMPAREH
    pub mod COMPAREH {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock Control Register
pub mod CLKCTRL {

    /// CLKSEL
    pub mod CLKSEL {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u8 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Clock Prescaler Register
pub mod CLKPRESCALER {

    /// CLK_DIV
    pub mod CLK_DIV {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
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
    /// Control Register
    pub CTRL: RWRegister<u8>,

    /// Service Register
    pub SERV: RWRegister<u8>,

    /// Compare Low Register
    pub CMPL: RWRegister<u8>,

    /// Compare High Register
    pub CMPH: RWRegister<u8>,

    /// Clock Control Register
    pub CLKCTRL: RWRegister<u8>,

    /// Clock Prescaler Register
    pub CLKPRESCALER: RWRegister<u8>,
}
pub struct ResetValues {
    pub CTRL: u8,
    pub SERV: u8,
    pub CMPL: u8,
    pub CMPH: u8,
    pub CLKCTRL: u8,
    pub CLKPRESCALER: u8,
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

/// Access functions for the EWM peripheral instance
pub mod EWM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400b4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EWM
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        SERV: 0x00000000,
        CMPL: 0x00000000,
        CMPH: 0x000000FF,
        CLKCTRL: 0x00000000,
        CLKPRESCALER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EWM_TAKEN: bool = false;

    /// Safe access to EWM
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
            if EWM_TAKEN {
                None
            } else {
                EWM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EWM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EWM_TAKEN && inst.addr == INSTANCE.addr {
                EWM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EWM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EWM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EWM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EWM: *const RegisterBlock = 0x400b4000 as *const _;
