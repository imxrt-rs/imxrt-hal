#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC_SNVS
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::iomuxc_snvs::Instance;
pub use crate::imxrt106::peripherals::iomuxc_snvs::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::iomuxc_snvs::{
    SW_MUX_CTL_PAD_PMIC_ON_REQ, SW_MUX_CTL_PAD_PMIC_STBY_REQ, SW_MUX_CTL_PAD_WAKEUP,
    SW_PAD_CTL_PAD_ONOFF, SW_PAD_CTL_PAD_PMIC_ON_REQ, SW_PAD_CTL_PAD_PMIC_STBY_REQ,
    SW_PAD_CTL_PAD_POR_B, SW_PAD_CTL_PAD_TEST_MODE, SW_PAD_CTL_PAD_WAKEUP,
};

/// Access functions for the IOMUXC_SNVS peripheral instance
pub mod IOMUXC_SNVS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400a8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC_SNVS
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_WAKEUP: 0x00000005,
        SW_MUX_CTL_PAD_PMIC_ON_REQ: 0x00000000,
        SW_MUX_CTL_PAD_PMIC_STBY_REQ: 0x00000000,
        SW_PAD_CTL_PAD_TEST_MODE: 0x000030A0,
        SW_PAD_CTL_PAD_POR_B: 0x0001B0A0,
        SW_PAD_CTL_PAD_ONOFF: 0x0001B0A0,
        SW_PAD_CTL_PAD_WAKEUP: 0x0001B0A0,
        SW_PAD_CTL_PAD_PMIC_ON_REQ: 0x0000B8A0,
        SW_PAD_CTL_PAD_PMIC_STBY_REQ: 0x0000A0A0,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_SNVS_TAKEN: bool = false;

    /// Safe access to IOMUXC_SNVS
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
            if IOMUXC_SNVS_TAKEN {
                None
            } else {
                IOMUXC_SNVS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC_SNVS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_SNVS_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_SNVS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC_SNVS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_SNVS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC_SNVS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_SNVS: *const RegisterBlock = 0x400a8000 as *const _;
