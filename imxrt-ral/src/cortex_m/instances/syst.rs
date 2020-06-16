#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System Timer
//!
//! Used by: armv6m, armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::syst::Instance;
pub use crate::cortex_m::peripherals::syst::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::syst::{CALIB, CSR, CVR, RVR};

/// Access functions for the SYST peripheral instance
pub mod SYST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e010,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYST
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        RVR: 0x00000000,
        CVR: 0x00000000,
        CALIB: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SYST_TAKEN: bool = false;

    /// Safe access to SYST
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
            if SYST_TAKEN {
                None
            } else {
                SYST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYST_TAKEN && inst.addr == INSTANCE.addr {
                SYST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SYST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SYST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SYST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYST: *const RegisterBlock = 0xe000e010 as *const _;
