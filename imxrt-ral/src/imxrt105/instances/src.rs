#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRC
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::src::Instance;
pub use crate::imxrt105::peripherals::src::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::src::{
    GPR1, GPR10, GPR2, GPR3, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9, SBMR1, SBMR2, SCR, SRSR,
};

/// Access functions for the SRC peripheral instance
pub mod SRC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400f8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SRC
    pub const reset: ResetValues = ResetValues {
        SCR: 0xA0480520,
        SBMR1: 0x00000000,
        SRSR: 0x00000001,
        SBMR2: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x00000000,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SRC_TAKEN: bool = false;

    /// Safe access to SRC
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
            if SRC_TAKEN {
                None
            } else {
                SRC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SRC_TAKEN && inst.addr == INSTANCE.addr {
                SRC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SRC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SRC: *const RegisterBlock = 0x400f8000 as *const _;
