#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ROMC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::romc::Instance;
pub use crate::imxrt106::peripherals::romc::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::romc::{
    ROMPATCH0A, ROMPATCH0D, ROMPATCH10A, ROMPATCH11A, ROMPATCH12A, ROMPATCH13A, ROMPATCH14A,
    ROMPATCH15A, ROMPATCH1A, ROMPATCH1D, ROMPATCH2A, ROMPATCH2D, ROMPATCH3A, ROMPATCH3D,
    ROMPATCH4A, ROMPATCH4D, ROMPATCH5A, ROMPATCH5D, ROMPATCH6A, ROMPATCH6D, ROMPATCH7A, ROMPATCH7D,
    ROMPATCH8A, ROMPATCH9A, ROMPATCHCNTL, ROMPATCHENH, ROMPATCHENL, ROMPATCHSR,
};

/// Access functions for the ROMC peripheral instance
pub mod ROMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40180000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ROMC
    pub const reset: ResetValues = ResetValues {
        ROMPATCH7D: 0x00000000,
        ROMPATCH6D: 0x00000000,
        ROMPATCH5D: 0x00000000,
        ROMPATCH4D: 0x00000000,
        ROMPATCH3D: 0x00000000,
        ROMPATCH2D: 0x00000000,
        ROMPATCH1D: 0x00000000,
        ROMPATCH0D: 0x00000000,
        ROMPATCHCNTL: 0x08400000,
        ROMPATCHENH: 0x00000000,
        ROMPATCHENL: 0x00000000,
        ROMPATCH0A: 0x00000000,
        ROMPATCH1A: 0x00000000,
        ROMPATCH2A: 0x00000000,
        ROMPATCH3A: 0x00000000,
        ROMPATCH4A: 0x00000000,
        ROMPATCH5A: 0x00000000,
        ROMPATCH6A: 0x00000000,
        ROMPATCH7A: 0x00000000,
        ROMPATCH8A: 0x00000000,
        ROMPATCH9A: 0x00000000,
        ROMPATCH10A: 0x00000000,
        ROMPATCH11A: 0x00000000,
        ROMPATCH12A: 0x00000000,
        ROMPATCH13A: 0x00000000,
        ROMPATCH14A: 0x00000000,
        ROMPATCH15A: 0x00000000,
        ROMPATCHSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ROMC_TAKEN: bool = false;

    /// Safe access to ROMC
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
            if ROMC_TAKEN {
                None
            } else {
                ROMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ROMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ROMC_TAKEN && inst.addr == INSTANCE.addr {
                ROMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ROMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ROMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ROMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ROMC: *const RegisterBlock = 0x40180000 as *const _;
