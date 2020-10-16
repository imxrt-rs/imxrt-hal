#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XTALOSC24M
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::xtalosc24m::Instance;
pub use crate::imxrt::peripherals::xtalosc24m::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::xtalosc24m::{
    LOWPWR_CTRL, LOWPWR_CTRL_CLR, LOWPWR_CTRL_SET, LOWPWR_CTRL_TOG, MISC0, MISC0_CLR, MISC0_SET,
    MISC0_TOG, OSC_CONFIG0, OSC_CONFIG0_CLR, OSC_CONFIG0_SET, OSC_CONFIG0_TOG, OSC_CONFIG1,
    OSC_CONFIG1_CLR, OSC_CONFIG1_SET, OSC_CONFIG1_TOG, OSC_CONFIG2, OSC_CONFIG2_CLR,
    OSC_CONFIG2_SET, OSC_CONFIG2_TOG,
};

/// Access functions for the XTALOSC24M peripheral instance
pub mod XTALOSC24M {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in XTALOSC24M
    pub const reset: ResetValues = ResetValues {
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        LOWPWR_CTRL: 0x00004001,
        LOWPWR_CTRL_SET: 0x00004001,
        LOWPWR_CTRL_CLR: 0x00004001,
        LOWPWR_CTRL_TOG: 0x00004001,
        OSC_CONFIG0: 0x00001020,
        OSC_CONFIG0_SET: 0x00001020,
        OSC_CONFIG0_CLR: 0x00001020,
        OSC_CONFIG0_TOG: 0x00001020,
        OSC_CONFIG1: 0x000002EE,
        OSC_CONFIG1_SET: 0x000002EE,
        OSC_CONFIG1_CLR: 0x000002EE,
        OSC_CONFIG1_TOG: 0x000002EE,
        OSC_CONFIG2: 0x000102E2,
        OSC_CONFIG2_SET: 0x000102E2,
        OSC_CONFIG2_CLR: 0x000102E2,
        OSC_CONFIG2_TOG: 0x000102E2,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut XTALOSC24M_TAKEN: bool = false;

    /// Safe access to XTALOSC24M
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
            if XTALOSC24M_TAKEN {
                None
            } else {
                XTALOSC24M_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to XTALOSC24M
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if XTALOSC24M_TAKEN && inst.addr == INSTANCE.addr {
                XTALOSC24M_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal XTALOSC24M
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        XTALOSC24M_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to XTALOSC24M
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XTALOSC24M: *const RegisterBlock = 0x400d8000 as *const _;
