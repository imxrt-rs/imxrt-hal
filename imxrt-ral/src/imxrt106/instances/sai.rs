#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! I2S
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::sai::Instance;
pub use crate::imxrt106::peripherals::sai::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::sai::{
    PARAM, RCR1, RCR2, RCR3, RCR4, RCR5, RCSR, RDR0, RDR1, RDR2, RDR3, RFR0, RFR1, RFR2, RFR3, RMR,
    TCR1, TCR2, TCR3, TCR4, TCR5, TCSR, TDR0, TDR1, TDR2, TDR3, TFR0, TFR1, TFR2, TFR3, TMR, VERID,
};

/// Access functions for the SAI1 peripheral instance
pub mod SAI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40384000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI1_TAKEN: bool = false;

    /// Safe access to SAI1
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
            if SAI1_TAKEN {
                None
            } else {
                SAI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI1_TAKEN && inst.addr == INSTANCE.addr {
                SAI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI1: *const RegisterBlock = 0x40384000 as *const _;

/// Access functions for the SAI2 peripheral instance
pub mod SAI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40388000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI2_TAKEN: bool = false;

    /// Safe access to SAI2
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
            if SAI2_TAKEN {
                None
            } else {
                SAI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI2_TAKEN && inst.addr == INSTANCE.addr {
                SAI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI2: *const RegisterBlock = 0x40388000 as *const _;

/// Access functions for the SAI3 peripheral instance
pub mod SAI3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4038c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050504,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TDR2: 0x00000000,
        TDR3: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TFR2: 0x00000000,
        TFR3: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RDR2: 0x00000000,
        RDR3: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RFR2: 0x00000000,
        RFR3: 0x00000000,
        RMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI3_TAKEN: bool = false;

    /// Safe access to SAI3
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
            if SAI3_TAKEN {
                None
            } else {
                SAI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI3_TAKEN && inst.addr == INSTANCE.addr {
                SAI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI3: *const RegisterBlock = 0x4038c000 as *const _;
