#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Trace Port Interface Unit
//!
//! Used by: armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::tpiu::Instance;
pub use crate::cortex_m::peripherals::tpiu::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::tpiu::{ACPR, CSPSR, LAR, LSR, SPPR, SSPSR, TYPE};

/// Access functions for the TPIU peripheral instance
pub mod TPIU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0040000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TPIU
    pub const reset: ResetValues = ResetValues {
        SSPSR: 0x00000000,
        CSPSR: 0x00000000,
        ACPR: 0x00000000,
        SPPR: 0x00000000,
        TYPE: 0x00000000,
        LSR: 0x00000000,
        LAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TPIU_TAKEN: bool = false;

    /// Safe access to TPIU
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
            if TPIU_TAKEN {
                None
            } else {
                TPIU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TPIU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TPIU_TAKEN && inst.addr == INSTANCE.addr {
                TPIU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TPIU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TPIU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TPIU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TPIU: *const RegisterBlock = 0xe0040000 as *const _;
