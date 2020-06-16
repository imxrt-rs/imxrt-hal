#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SNVS
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::snvs::Instance;
pub use crate::imxrt106::peripherals::snvs::{
    LPGPR0_legacy_alias, LPGPR_alias0, LPGPR_alias1, LPGPR_alias2, LPGPR_alias3, HPCOMR, HPCR,
    HPHACIVR, HPHACR, HPLR, HPRTCLR, HPRTCMR, HPSICR, HPSR, HPSVCR, HPSVSR, HPTALR, HPTAMR,
    HPVIDR1, HPVIDR2, LPCR, LPGPR0, LPGPR1, LPGPR2, LPGPR3, LPGPR4, LPGPR5, LPGPR6, LPGPR7, LPLR,
    LPMKCR, LPPGDR, LPSMCLR, LPSMCMR, LPSR, LPSRTCLR, LPSRTCMR, LPSVCR, LPTAR, LPTDCR, LPZMKR0,
    LPZMKR1, LPZMKR2, LPZMKR3, LPZMKR4, LPZMKR5, LPZMKR6, LPZMKR7,
};
pub use crate::imxrt106::peripherals::snvs::{RegisterBlock, ResetValues};

/// Access functions for the SNVS peripheral instance
pub mod SNVS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SNVS
    pub const reset: ResetValues = ResetValues {
        HPLR: 0x00000000,
        HPCOMR: 0x00000000,
        HPCR: 0x00000000,
        HPSICR: 0x00000000,
        HPSVCR: 0x00000000,
        HPSR: 0x80003000,
        HPSVSR: 0x80000000,
        HPHACIVR: 0x00000000,
        HPHACR: 0x00000000,
        HPRTCMR: 0x00000000,
        HPRTCLR: 0x00000000,
        HPTAMR: 0x00000000,
        HPTALR: 0x00000000,
        LPLR: 0x00000000,
        LPCR: 0x00000020,
        LPMKCR: 0x00000000,
        LPSVCR: 0x00000000,
        LPTDCR: 0x00000000,
        LPSR: 0x00000008,
        LPSRTCMR: 0x00000000,
        LPSRTCLR: 0x00000000,
        LPTAR: 0x00000000,
        LPSMCMR: 0x00000000,
        LPSMCLR: 0x00000000,
        LPPGDR: 0x00000000,
        LPGPR0_legacy_alias: 0x00000000,
        LPZMKR0: 0x00000000,
        LPZMKR1: 0x00000000,
        LPZMKR2: 0x00000000,
        LPZMKR3: 0x00000000,
        LPZMKR4: 0x00000000,
        LPZMKR5: 0x00000000,
        LPZMKR6: 0x00000000,
        LPZMKR7: 0x00000000,
        LPGPR_alias0: 0x00000000,
        LPGPR_alias1: 0x00000000,
        LPGPR_alias2: 0x00000000,
        LPGPR_alias3: 0x00000000,
        LPGPR0: 0x00000000,
        LPGPR1: 0x00000000,
        LPGPR2: 0x00000000,
        LPGPR3: 0x00000000,
        LPGPR4: 0x00000000,
        LPGPR5: 0x00000000,
        LPGPR6: 0x00000000,
        LPGPR7: 0x00000000,
        HPVIDR1: 0x003E0104,
        HPVIDR2: 0x06000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SNVS_TAKEN: bool = false;

    /// Safe access to SNVS
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
            if SNVS_TAKEN {
                None
            } else {
                SNVS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SNVS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SNVS_TAKEN && inst.addr == INSTANCE.addr {
                SNVS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SNVS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SNVS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SNVS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SNVS: *const RegisterBlock = 0x400d4000 as *const _;
