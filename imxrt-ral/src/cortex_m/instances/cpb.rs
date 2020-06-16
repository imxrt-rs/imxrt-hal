#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cache and branch predictor maintenance operations
//!
//! Used by: armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::cpb::Instance;
pub use crate::cortex_m::peripherals::cpb::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::cpb::{
    BPIALL, DCCIMVAC, DCCISW, DCCMVAC, DCCMVAU, DCCSW, DCIMVAC, DCISW, ICIALLU, ICIMVAU,
};

/// Access functions for the CPB peripheral instance
pub mod CPB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ef50,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CPB
    pub const reset: ResetValues = ResetValues {
        ICIALLU: 0x00000000,
        ICIMVAU: 0x00000000,
        DCIMVAC: 0x00000000,
        DCISW: 0x00000000,
        DCCMVAU: 0x00000000,
        DCCMVAC: 0x00000000,
        DCCSW: 0x00000000,
        DCCIMVAC: 0x00000000,
        DCCISW: 0x00000000,
        BPIALL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CPB_TAKEN: bool = false;

    /// Safe access to CPB
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
            if CPB_TAKEN {
                None
            } else {
                CPB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CPB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CPB_TAKEN && inst.addr == INSTANCE.addr {
                CPB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CPB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CPB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CPB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CPB: *const RegisterBlock = 0xe000ef50 as *const _;
