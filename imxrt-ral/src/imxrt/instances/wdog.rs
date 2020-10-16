#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG
//!
//! Used by: imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::wdog::Instance;
pub use crate::imxrt::peripherals::wdog::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::wdog::{WCR, WICR, WMCR, WRSR, WSR};

/// Access functions for the WDOG1 peripheral instance
pub mod WDOG1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400b8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WDOG1
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000000,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut WDOG1_TAKEN: bool = false;

    /// Safe access to WDOG1
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
            if WDOG1_TAKEN {
                None
            } else {
                WDOG1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WDOG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WDOG1_TAKEN && inst.addr == INSTANCE.addr {
                WDOG1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WDOG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WDOG1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WDOG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG1: *const RegisterBlock = 0x400b8000 as *const _;

/// Access functions for the WDOG2 peripheral instance
pub mod WDOG2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WDOG2
    pub const reset: ResetValues = ResetValues {
        WCR: 0x00000030,
        WSR: 0x00000000,
        WRSR: 0x00000000,
        WICR: 0x00000004,
        WMCR: 0x00000001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut WDOG2_TAKEN: bool = false;

    /// Safe access to WDOG2
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
            if WDOG2_TAKEN {
                None
            } else {
                WDOG2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WDOG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WDOG2_TAKEN && inst.addr == INSTANCE.addr {
                WDOG2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WDOG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WDOG2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WDOG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WDOG2: *const RegisterBlock = 0x400d0000 as *const _;
