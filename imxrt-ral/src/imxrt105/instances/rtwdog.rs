#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::rtwdog::Instance;
pub use crate::imxrt105::peripherals::rtwdog::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::rtwdog::{CNT, CS, TOVAL, WIN};

/// Access functions for the RTWDOG peripheral instance
pub mod RTWDOG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400bc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RTWDOG
    pub const reset: ResetValues = ResetValues {
        CS: 0x00002980,
        CNT: 0x00000000,
        TOVAL: 0x00000400,
        WIN: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RTWDOG_TAKEN: bool = false;

    /// Safe access to RTWDOG
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
            if RTWDOG_TAKEN {
                None
            } else {
                RTWDOG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RTWDOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RTWDOG_TAKEN && inst.addr == INSTANCE.addr {
                RTWDOG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RTWDOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RTWDOG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RTWDOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTWDOG: *const RegisterBlock = 0x400bc000 as *const _;
