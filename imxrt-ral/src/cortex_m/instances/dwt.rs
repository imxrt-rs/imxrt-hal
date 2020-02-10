#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Data Watchpoint and Trace Unit
//!
//! Used by: armv6m, armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::dwt::Instance;
pub use crate::cortex_m::peripherals::dwt::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::dwt::{
    COMP0, COMP1, COMP10, COMP11, COMP12, COMP13, COMP14, COMP15, COMP2, COMP3, COMP4, COMP5,
    COMP6, COMP7, COMP8, COMP9, CPICNT, CTRL, CYCCNT, EXCCNT, FOLDCNT, FUNCTION0, FUNCTION1,
    FUNCTION10, FUNCTION11, FUNCTION12, FUNCTION13, FUNCTION14, FUNCTION15, FUNCTION2, FUNCTION3,
    FUNCTION4, FUNCTION5, FUNCTION6, FUNCTION7, FUNCTION8, FUNCTION9, LAR, LSR, LSUCNT, MASK0,
    MASK1, MASK10, MASK11, MASK12, MASK13, MASK14, MASK15, MASK2, MASK3, MASK4, MASK5, MASK6,
    MASK7, MASK8, MASK9, PCSR, SLEEPCNT,
};

/// Access functions for the DWT peripheral instance
pub mod DWT {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DWT
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        CYCCNT: 0x00000000,
        CPICNT: 0x00000000,
        EXCCNT: 0x00000000,
        SLEEPCNT: 0x00000000,
        LSUCNT: 0x00000000,
        FOLDCNT: 0x00000000,
        PCSR: 0x00000000,
        COMP0: 0x00000000,
        MASK0: 0x00000000,
        FUNCTION0: 0x00000000,
        COMP1: 0x00000000,
        MASK1: 0x00000000,
        FUNCTION1: 0x00000000,
        COMP2: 0x00000000,
        MASK2: 0x00000000,
        FUNCTION2: 0x00000000,
        COMP3: 0x00000000,
        MASK3: 0x00000000,
        FUNCTION3: 0x00000000,
        COMP4: 0x00000000,
        MASK4: 0x00000000,
        FUNCTION4: 0x00000000,
        COMP5: 0x00000000,
        MASK5: 0x00000000,
        FUNCTION5: 0x00000000,
        COMP6: 0x00000000,
        MASK6: 0x00000000,
        FUNCTION6: 0x00000000,
        COMP7: 0x00000000,
        MASK7: 0x00000000,
        FUNCTION7: 0x00000000,
        COMP8: 0x00000000,
        MASK8: 0x00000000,
        FUNCTION8: 0x00000000,
        COMP9: 0x00000000,
        MASK9: 0x00000000,
        FUNCTION9: 0x00000000,
        COMP10: 0x00000000,
        MASK10: 0x00000000,
        FUNCTION10: 0x00000000,
        COMP11: 0x00000000,
        MASK11: 0x00000000,
        FUNCTION11: 0x00000000,
        COMP12: 0x00000000,
        MASK12: 0x00000000,
        FUNCTION12: 0x00000000,
        COMP13: 0x00000000,
        MASK13: 0x00000000,
        FUNCTION13: 0x00000000,
        COMP14: 0x00000000,
        MASK14: 0x00000000,
        FUNCTION14: 0x00000000,
        COMP15: 0x00000000,
        MASK15: 0x00000000,
        FUNCTION15: 0x00000000,
        LSR: 0x00000000,
        LAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DWT_TAKEN: bool = false;

    /// Safe access to DWT
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
            if DWT_TAKEN {
                None
            } else {
                DWT_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DWT
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DWT_TAKEN && inst.addr == INSTANCE.addr {
                DWT_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DWT
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DWT_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DWT
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DWT: *const RegisterBlock = 0xe0001000 as *const _;
