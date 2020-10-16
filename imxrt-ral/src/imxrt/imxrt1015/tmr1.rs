#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Quad Timer

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::tmr1::Instance;
pub use crate::imxrt::peripherals::tmr1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::tmr1::{
    CAPT0, CAPT1, CAPT2, CAPT3, CMPLD10, CMPLD11, CMPLD12, CMPLD13, CMPLD20, CMPLD21, CMPLD22,
    CMPLD23, CNTR0, CNTR1, CNTR2, CNTR3, COMP10, COMP11, COMP12, COMP13, COMP20, COMP21, COMP22,
    COMP23, CSCTRL0, CSCTRL1, CSCTRL2, CSCTRL3, CTRL0, CTRL1, CTRL2, CTRL3, DMA0, DMA1, DMA2, DMA3,
    ENBL, FILT0, FILT1, FILT2, FILT3, HOLD0, HOLD1, HOLD2, HOLD3, LOAD0, LOAD1, LOAD2, LOAD3,
    SCTRL0, SCTRL1, SCTRL2, SCTRL3,
};

/// Access functions for the TMR1 peripheral instance
pub mod TMR1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401dc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TMR1
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TMR1_TAKEN: bool = false;

    /// Safe access to TMR1
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
            if TMR1_TAKEN {
                None
            } else {
                TMR1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TMR1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TMR1_TAKEN && inst.addr == INSTANCE.addr {
                TMR1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TMR1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TMR1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TMR1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR1: *const RegisterBlock = 0x401dc000 as *const _;
