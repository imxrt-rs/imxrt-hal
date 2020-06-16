#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System Control Block
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::systemcontrol::Instance;
pub use crate::imxrt105::peripherals::systemcontrol::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::systemcontrol::{
    ACTLR, AIRCR, BFAR, CCR, CCSIDR, CFSR, CLIDR, CM7_ABFSR, CM7_AHBPCR, CM7_AHBSCR, CM7_CACR,
    CM7_DTCMCR, CM7_ITCMCR, CPACR, CPUID, CSSELR, CTR, DCCIMVAC, DCCISW, DCCMVAC, DCCMVAU, DCCSW,
    DCIMVAC, DCISW, DFSR, HFSR, ICIALLU, ICIMVAU, ICSR, ID_AFR0, ID_DFR0, ID_ISAR0, ID_ISAR1,
    ID_ISAR2, ID_ISAR3, ID_ISAR4, ID_MMFR0, ID_MMFR1, ID_MMFR2, ID_MMFR3, ID_PFR0, ID_PFR1, MMFAR,
    SCR, SHCSR, SHPR1, SHPR2, SHPR3, STIR, VTOR,
};

/// Access functions for the SystemControl peripheral instance
pub mod SystemControl {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SystemControl
    pub const reset: ResetValues = ResetValues {
        ACTLR: 0x00000000,
        CPUID: 0x410FC240,
        ICSR: 0x00000000,
        VTOR: 0x00000000,
        AIRCR: 0xFA050000,
        SCR: 0x00000000,
        CCR: 0x00040000,
        SHPR1: 0x00000000,
        SHPR2: 0x00000000,
        SHPR3: 0x00000000,
        SHCSR: 0x00000000,
        CFSR: 0x00000000,
        HFSR: 0x00000000,
        DFSR: 0x00000000,
        MMFAR: 0x00000000,
        BFAR: 0x00000000,
        ID_PFR0: 0x00000000,
        ID_PFR1: 0x00000000,
        ID_DFR0: 0x00000000,
        ID_AFR0: 0x00000000,
        ID_MMFR0: 0x00000000,
        ID_MMFR1: 0x00000000,
        ID_MMFR2: 0x00000000,
        ID_MMFR3: 0x00000000,
        ID_ISAR0: 0x00000000,
        ID_ISAR1: 0x00000000,
        ID_ISAR2: 0x00000000,
        ID_ISAR3: 0x00000000,
        ID_ISAR4: 0x00000000,
        CLIDR: 0x00000000,
        CTR: 0x8000C000,
        CCSIDR: 0x00000000,
        CSSELR: 0x00000000,
        CPACR: 0x00000000,
        STIR: 0x00000000,
        ICIALLU: 0x00000000,
        ICIMVAU: 0x00000000,
        DCIMVAC: 0x00000000,
        DCISW: 0x00000000,
        DCCMVAU: 0x00000000,
        DCCMVAC: 0x00000000,
        DCCSW: 0x00000000,
        DCCIMVAC: 0x00000000,
        DCCISW: 0x00000000,
        CM7_ITCMCR: 0x00000000,
        CM7_DTCMCR: 0x00000000,
        CM7_AHBPCR: 0x00000000,
        CM7_CACR: 0x00000000,
        CM7_AHBSCR: 0x00000000,
        CM7_ABFSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SystemControl_TAKEN: bool = false;

    /// Safe access to SystemControl
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
            if SystemControl_TAKEN {
                None
            } else {
                SystemControl_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SystemControl
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SystemControl_TAKEN && inst.addr == INSTANCE.addr {
                SystemControl_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SystemControl
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SystemControl_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SystemControl
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SystemControl: *const RegisterBlock = 0xe000e000 as *const _;
