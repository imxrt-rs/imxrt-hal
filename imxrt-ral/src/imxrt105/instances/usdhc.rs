#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! uSDHC
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::usdhc::Instance;
pub use crate::imxrt105::peripherals::usdhc::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::usdhc::{
    ADMA_ERR_STATUS, ADMA_SYS_ADDR, AUTOCMD12_ERR_STATUS, BLK_ATT, CLK_TUNE_CTRL_STATUS, CMD_ARG,
    CMD_RSP0, CMD_RSP1, CMD_RSP2, CMD_RSP3, CMD_XFR_TYP, DATA_BUFF_ACC_PORT, DLL_CTRL, DLL_STATUS,
    DS_ADDR, FORCE_EVENT, HOST_CTRL_CAP, INT_SIGNAL_EN, INT_STATUS, INT_STATUS_EN, MIX_CTRL,
    MMC_BOOT, PRES_STATE, PROT_CTRL, SYS_CTRL, TUNING_CTRL, VEND_SPEC, VEND_SPEC2, WTMK_LVL,
};

/// Access functions for the USDHC1 peripheral instance
pub mod USDHC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402c0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USDHC1
    pub const reset: ResetValues = ResetValues {
        DS_ADDR: 0x00000000,
        BLK_ATT: 0x00000000,
        CMD_ARG: 0x00000000,
        CMD_XFR_TYP: 0x00000000,
        CMD_RSP0: 0x00000000,
        CMD_RSP1: 0x00000000,
        CMD_RSP2: 0x00000000,
        CMD_RSP3: 0x00000000,
        DATA_BUFF_ACC_PORT: 0x00000000,
        PRES_STATE: 0x00008080,
        PROT_CTRL: 0x08800020,
        SYS_CTRL: 0x0080800F,
        INT_STATUS: 0x00000000,
        INT_STATUS_EN: 0x00000000,
        INT_SIGNAL_EN: 0x00000000,
        AUTOCMD12_ERR_STATUS: 0x00000000,
        HOST_CTRL_CAP: 0x07F3B407,
        WTMK_LVL: 0x08100810,
        MIX_CTRL: 0x80000000,
        FORCE_EVENT: 0x00000000,
        ADMA_ERR_STATUS: 0x00000000,
        ADMA_SYS_ADDR: 0x00000000,
        DLL_CTRL: 0x00000000,
        DLL_STATUS: 0x00000200,
        CLK_TUNE_CTRL_STATUS: 0x00000000,
        VEND_SPEC: 0x20007809,
        MMC_BOOT: 0x00000000,
        VEND_SPEC2: 0x00001006,
        TUNING_CTRL: 0x00212800,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USDHC1_TAKEN: bool = false;

    /// Safe access to USDHC1
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
            if USDHC1_TAKEN {
                None
            } else {
                USDHC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USDHC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USDHC1_TAKEN && inst.addr == INSTANCE.addr {
                USDHC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USDHC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USDHC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USDHC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USDHC1: *const RegisterBlock = 0x402c0000 as *const _;

/// Access functions for the USDHC2 peripheral instance
pub mod USDHC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402c4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USDHC2
    pub const reset: ResetValues = ResetValues {
        DS_ADDR: 0x00000000,
        BLK_ATT: 0x00000000,
        CMD_ARG: 0x00000000,
        CMD_XFR_TYP: 0x00000000,
        CMD_RSP0: 0x00000000,
        CMD_RSP1: 0x00000000,
        CMD_RSP2: 0x00000000,
        CMD_RSP3: 0x00000000,
        DATA_BUFF_ACC_PORT: 0x00000000,
        PRES_STATE: 0x00008080,
        PROT_CTRL: 0x08800020,
        SYS_CTRL: 0x0080800F,
        INT_STATUS: 0x00000000,
        INT_STATUS_EN: 0x00000000,
        INT_SIGNAL_EN: 0x00000000,
        AUTOCMD12_ERR_STATUS: 0x00000000,
        HOST_CTRL_CAP: 0x07F3B407,
        WTMK_LVL: 0x08100810,
        MIX_CTRL: 0x80000000,
        FORCE_EVENT: 0x00000000,
        ADMA_ERR_STATUS: 0x00000000,
        ADMA_SYS_ADDR: 0x00000000,
        DLL_CTRL: 0x00000000,
        DLL_STATUS: 0x00000200,
        CLK_TUNE_CTRL_STATUS: 0x00000000,
        VEND_SPEC: 0x20007809,
        MMC_BOOT: 0x00000000,
        VEND_SPEC2: 0x00001006,
        TUNING_CTRL: 0x00212800,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USDHC2_TAKEN: bool = false;

    /// Safe access to USDHC2
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
            if USDHC2_TAKEN {
                None
            } else {
                USDHC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USDHC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USDHC2_TAKEN && inst.addr == INSTANCE.addr {
                USDHC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USDHC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USDHC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USDHC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USDHC2: *const RegisterBlock = 0x402c4000 as *const _;
