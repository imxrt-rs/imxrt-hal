#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::dmamux::Instance;
pub use crate::imxrt::peripherals::dmamux::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::dmamux::{
    CHCFG0, CHCFG1, CHCFG10, CHCFG11, CHCFG12, CHCFG13, CHCFG14, CHCFG15, CHCFG16, CHCFG17,
    CHCFG18, CHCFG19, CHCFG2, CHCFG20, CHCFG21, CHCFG22, CHCFG23, CHCFG24, CHCFG25, CHCFG26,
    CHCFG27, CHCFG28, CHCFG29, CHCFG3, CHCFG30, CHCFG31, CHCFG4, CHCFG5, CHCFG6, CHCFG7, CHCFG8,
    CHCFG9,
};

/// Access functions for the DMAMUX peripheral instance
pub mod DMAMUX {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400ec000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX
    pub const reset: ResetValues = ResetValues {
        CHCFG0: 0x00000000,
        CHCFG1: 0x00000000,
        CHCFG2: 0x00000000,
        CHCFG3: 0x00000000,
        CHCFG4: 0x00000000,
        CHCFG5: 0x00000000,
        CHCFG6: 0x00000000,
        CHCFG7: 0x00000000,
        CHCFG8: 0x00000000,
        CHCFG9: 0x00000000,
        CHCFG10: 0x00000000,
        CHCFG11: 0x00000000,
        CHCFG12: 0x00000000,
        CHCFG13: 0x00000000,
        CHCFG14: 0x00000000,
        CHCFG15: 0x00000000,
        CHCFG16: 0x00000000,
        CHCFG17: 0x00000000,
        CHCFG18: 0x00000000,
        CHCFG19: 0x00000000,
        CHCFG20: 0x00000000,
        CHCFG21: 0x00000000,
        CHCFG22: 0x00000000,
        CHCFG23: 0x00000000,
        CHCFG24: 0x00000000,
        CHCFG25: 0x00000000,
        CHCFG26: 0x00000000,
        CHCFG27: 0x00000000,
        CHCFG28: 0x00000000,
        CHCFG29: 0x00000000,
        CHCFG30: 0x00000000,
        CHCFG31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX_TAKEN: bool = false;

    /// Safe access to DMAMUX
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
            if DMAMUX_TAKEN {
                None
            } else {
                DMAMUX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX: *const RegisterBlock = 0x400ec000 as *const _;
