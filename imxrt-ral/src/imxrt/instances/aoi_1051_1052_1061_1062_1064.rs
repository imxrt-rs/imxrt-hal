#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AND/OR/INVERT module
//!
//! Used by: imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::aoi::Instance;
pub use crate::imxrt::peripherals::aoi::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::aoi::{
    BFCRT010, BFCRT011, BFCRT012, BFCRT013, BFCRT230, BFCRT231, BFCRT232, BFCRT233,
};

/// Access functions for the AOI1 peripheral instance
pub mod AOI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403b4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AOI1
    pub const reset: ResetValues = ResetValues {
        BFCRT010: 0x00000000,
        BFCRT011: 0x00000000,
        BFCRT012: 0x00000000,
        BFCRT013: 0x00000000,
        BFCRT230: 0x00000000,
        BFCRT231: 0x00000000,
        BFCRT232: 0x00000000,
        BFCRT233: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AOI1_TAKEN: bool = false;

    /// Safe access to AOI1
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
            if AOI1_TAKEN {
                None
            } else {
                AOI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AOI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AOI1_TAKEN && inst.addr == INSTANCE.addr {
                AOI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AOI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AOI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AOI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI1: *const RegisterBlock = 0x403b4000 as *const _;

/// Access functions for the AOI2 peripheral instance
pub mod AOI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403b8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AOI2
    pub const reset: ResetValues = ResetValues {
        BFCRT010: 0x00000000,
        BFCRT011: 0x00000000,
        BFCRT012: 0x00000000,
        BFCRT013: 0x00000000,
        BFCRT230: 0x00000000,
        BFCRT231: 0x00000000,
        BFCRT232: 0x00000000,
        BFCRT233: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AOI2_TAKEN: bool = false;

    /// Safe access to AOI2
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
            if AOI2_TAKEN {
                None
            } else {
                AOI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AOI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AOI2_TAKEN && inst.addr == INSTANCE.addr {
                AOI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AOI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AOI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AOI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI2: *const RegisterBlock = 0x403b8000 as *const _;
