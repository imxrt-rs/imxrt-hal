#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPSPI
//!
//! Used by: imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::lpspi_v2::Instance;
pub use crate::imxrt::peripherals::lpspi_v2::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::lpspi_v2::{
    CCR, CFGR0, CFGR1, CR, DER, DMR0, DMR1, FCR, FSR, IER, PARAM, RDR, RSR, SR, TCR, TDR, VERID,
};

/// Access functions for the LPSPI1 peripheral instance
pub mod LPSPI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40394000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPSPI1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPSPI1_TAKEN: bool = false;

    /// Safe access to LPSPI1
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
            if LPSPI1_TAKEN {
                None
            } else {
                LPSPI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPSPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPSPI1_TAKEN && inst.addr == INSTANCE.addr {
                LPSPI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPSPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPSPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI1: *const RegisterBlock = 0x40394000 as *const _;

/// Access functions for the LPSPI2 peripheral instance
pub mod LPSPI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40398000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPSPI2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPSPI2_TAKEN: bool = false;

    /// Safe access to LPSPI2
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
            if LPSPI2_TAKEN {
                None
            } else {
                LPSPI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPSPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPSPI2_TAKEN && inst.addr == INSTANCE.addr {
                LPSPI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPSPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPSPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI2: *const RegisterBlock = 0x40398000 as *const _;

/// Access functions for the LPSPI3 peripheral instance
pub mod LPSPI3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4039c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPSPI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPSPI3_TAKEN: bool = false;

    /// Safe access to LPSPI3
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
            if LPSPI3_TAKEN {
                None
            } else {
                LPSPI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPSPI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPSPI3_TAKEN && inst.addr == INSTANCE.addr {
                LPSPI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPSPI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPSPI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI3: *const RegisterBlock = 0x4039c000 as *const _;

/// Access functions for the LPSPI4 peripheral instance
pub mod LPSPI4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403a0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPSPI4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01020004,
        PARAM: 0x00040404,
        CR: 0x00000000,
        SR: 0x00000001,
        IER: 0x00000000,
        DER: 0x00000000,
        CFGR0: 0x00000000,
        CFGR1: 0x00000000,
        DMR0: 0x00000000,
        DMR1: 0x00000000,
        CCR: 0x00000000,
        FCR: 0x00000000,
        FSR: 0x00000000,
        TCR: 0x0000001F,
        TDR: 0x00000000,
        RSR: 0x00000002,
        RDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPSPI4_TAKEN: bool = false;

    /// Safe access to LPSPI4
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
            if LPSPI4_TAKEN {
                None
            } else {
                LPSPI4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPSPI4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPSPI4_TAKEN && inst.addr == INSTANCE.addr {
                LPSPI4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPSPI4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPSPI4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPSPI4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPSPI4: *const RegisterBlock = 0x403a0000 as *const _;
