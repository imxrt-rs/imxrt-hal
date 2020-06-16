#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PMU
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::pmu::Instance;
pub use crate::imxrt105::peripherals::pmu::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::pmu::{
    MISC0, MISC0_CLR, MISC0_SET, MISC0_TOG, MISC1, MISC1_CLR, MISC1_SET, MISC1_TOG, MISC2,
    MISC2_CLR, MISC2_SET, MISC2_TOG, REG_1P1, REG_1P1_CLR, REG_1P1_SET, REG_1P1_TOG, REG_2P5,
    REG_2P5_CLR, REG_2P5_SET, REG_2P5_TOG, REG_3P0, REG_3P0_CLR, REG_3P0_SET, REG_3P0_TOG,
    REG_CORE, REG_CORE_CLR, REG_CORE_SET, REG_CORE_TOG,
};

/// Access functions for the PMU peripheral instance
pub mod PMU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PMU
    pub const reset: ResetValues = ResetValues {
        REG_1P1: 0x00001073,
        REG_1P1_SET: 0x00001073,
        REG_1P1_CLR: 0x00001073,
        REG_1P1_TOG: 0x00001073,
        REG_3P0: 0x00000F74,
        REG_3P0_SET: 0x00000F74,
        REG_3P0_CLR: 0x00000F74,
        REG_3P0_TOG: 0x00000F74,
        REG_2P5: 0x00001073,
        REG_2P5_SET: 0x00001073,
        REG_2P5_CLR: 0x00001073,
        REG_2P5_TOG: 0x00001073,
        REG_CORE: 0x00482012,
        REG_CORE_SET: 0x00482012,
        REG_CORE_CLR: 0x00482012,
        REG_CORE_TOG: 0x00482012,
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        MISC1: 0x00000000,
        MISC1_SET: 0x00000000,
        MISC1_CLR: 0x00000000,
        MISC1_TOG: 0x00000000,
        MISC2: 0x00272727,
        MISC2_SET: 0x00272727,
        MISC2_CLR: 0x00272727,
        MISC2_TOG: 0x00272727,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PMU_TAKEN: bool = false;

    /// Safe access to PMU
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
            if PMU_TAKEN {
                None
            } else {
                PMU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PMU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PMU_TAKEN && inst.addr == INSTANCE.addr {
                PMU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PMU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PMU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PMU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PMU: *const RegisterBlock = 0x400d8000 as *const _;
