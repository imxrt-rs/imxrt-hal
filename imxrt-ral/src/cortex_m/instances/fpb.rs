#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash Patch and Breakpoint Unit
//!
//! Used by: armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::fpb::Instance;
pub use crate::cortex_m::peripherals::fpb::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::fpb::{
    COMP0, COMP1, COMP10, COMP11, COMP12, COMP13, COMP14, COMP15, COMP2, COMP3, COMP4, COMP5,
    COMP6, COMP7, COMP8, COMP9, CTRL, LAR, LSR, REMAP,
};

/// Access functions for the FPB peripheral instance
pub mod FPB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FPB
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        REMAP: 0x00000000,
        COMP0: 0x00000000,
        COMP1: 0x00000000,
        COMP2: 0x00000000,
        COMP3: 0x00000000,
        COMP4: 0x00000000,
        COMP5: 0x00000000,
        COMP6: 0x00000000,
        COMP7: 0x00000000,
        COMP8: 0x00000000,
        COMP9: 0x00000000,
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP14: 0x00000000,
        COMP15: 0x00000000,
        LSR: 0x00000000,
        LAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FPB_TAKEN: bool = false;

    /// Safe access to FPB
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
            if FPB_TAKEN {
                None
            } else {
                FPB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FPB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FPB_TAKEN && inst.addr == INSTANCE.addr {
                FPB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FPB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FPB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FPB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FPB: *const RegisterBlock = 0xe0002000 as *const _;
