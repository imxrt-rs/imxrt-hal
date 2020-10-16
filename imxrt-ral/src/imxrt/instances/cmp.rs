#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)
//!
//! Used by: imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::cmp::Instance;
pub use crate::imxrt::peripherals::cmp::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::cmp::{CR0, CR1, DACCR, FPR, MUXCR, SCR};

/// Access functions for the CMP1 peripheral instance
pub mod CMP1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP1
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP1_TAKEN: bool = false;

    /// Safe access to CMP1
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
            if CMP1_TAKEN {
                None
            } else {
                CMP1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP1_TAKEN && inst.addr == INSTANCE.addr {
                CMP1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP1: *const RegisterBlock = 0x40094000 as *const _;

/// Access functions for the CMP2 peripheral instance
pub mod CMP2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094008,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP2
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP2_TAKEN: bool = false;

    /// Safe access to CMP2
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
            if CMP2_TAKEN {
                None
            } else {
                CMP2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP2_TAKEN && inst.addr == INSTANCE.addr {
                CMP2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP2: *const RegisterBlock = 0x40094008 as *const _;

/// Access functions for the CMP3 peripheral instance
pub mod CMP3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094010,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP3
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP3_TAKEN: bool = false;

    /// Safe access to CMP3
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
            if CMP3_TAKEN {
                None
            } else {
                CMP3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP3_TAKEN && inst.addr == INSTANCE.addr {
                CMP3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP3: *const RegisterBlock = 0x40094010 as *const _;

/// Access functions for the CMP4 peripheral instance
pub mod CMP4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094018,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP4
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP4_TAKEN: bool = false;

    /// Safe access to CMP4
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
            if CMP4_TAKEN {
                None
            } else {
                CMP4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP4_TAKEN && inst.addr == INSTANCE.addr {
                CMP4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP4: *const RegisterBlock = 0x40094018 as *const _;
