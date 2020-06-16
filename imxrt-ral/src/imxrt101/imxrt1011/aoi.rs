#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AND/OR/INVERT module

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::aoi::Instance;
pub use crate::imxrt101::peripherals::aoi::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::aoi::{
    BFCRT010, BFCRT011, BFCRT012, BFCRT013, BFCRT230, BFCRT231, BFCRT232, BFCRT233,
};

/// Access functions for the AOI peripheral instance
pub mod AOI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AOI
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
    static mut AOI_TAKEN: bool = false;

    /// Safe access to AOI
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
            if AOI_TAKEN {
                None
            } else {
                AOI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AOI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AOI_TAKEN && inst.addr == INSTANCE.addr {
                AOI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AOI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AOI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AOI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI: *const RegisterBlock = 0x40094000 as *const _;
