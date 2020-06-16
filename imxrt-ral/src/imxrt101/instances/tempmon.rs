#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Temperature Monitor
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::tempmon::Instance;
pub use crate::imxrt101::peripherals::tempmon::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::tempmon::{
    TEMPSENSE0, TEMPSENSE0_CLR, TEMPSENSE0_SET, TEMPSENSE0_TOG, TEMPSENSE1, TEMPSENSE1_CLR,
    TEMPSENSE1_SET, TEMPSENSE1_TOG, TEMPSENSE2, TEMPSENSE2_CLR, TEMPSENSE2_SET, TEMPSENSE2_TOG,
};

/// Access functions for the TEMPMON peripheral instance
pub mod TEMPMON {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TEMPMON
    pub const reset: ResetValues = ResetValues {
        TEMPSENSE0: 0x00000001,
        TEMPSENSE0_SET: 0x00000001,
        TEMPSENSE0_CLR: 0x00000001,
        TEMPSENSE0_TOG: 0x00000001,
        TEMPSENSE1: 0x00000001,
        TEMPSENSE1_SET: 0x00000001,
        TEMPSENSE1_CLR: 0x00000001,
        TEMPSENSE1_TOG: 0x00000001,
        TEMPSENSE2: 0x00000000,
        TEMPSENSE2_SET: 0x00000000,
        TEMPSENSE2_CLR: 0x00000000,
        TEMPSENSE2_TOG: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TEMPMON_TAKEN: bool = false;

    /// Safe access to TEMPMON
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
            if TEMPMON_TAKEN {
                None
            } else {
                TEMPMON_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TEMPMON
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TEMPMON_TAKEN && inst.addr == INSTANCE.addr {
                TEMPMON_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TEMPMON
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TEMPMON_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TEMPMON
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TEMPMON: *const RegisterBlock = 0x400d8000 as *const _;
