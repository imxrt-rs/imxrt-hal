#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Quadrature Decoder
//!
//! Used by: imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::enc1::Instance;
pub use crate::imxrt::peripherals::enc1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::enc1::{
    CTRL, CTRL2, FILT, IMR, LCOMP, LINIT, LMOD, LPOS, LPOSH, POSD, POSDH, REV, REVH, TST, UCOMP,
    UINIT, UMOD, UPOS, UPOSH, WTR,
};

/// Access functions for the ENC1 peripheral instance
pub mod ENC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403c8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ENC1
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ENC1_TAKEN: bool = false;

    /// Safe access to ENC1
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
            if ENC1_TAKEN {
                None
            } else {
                ENC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ENC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ENC1_TAKEN && inst.addr == INSTANCE.addr {
                ENC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ENC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ENC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ENC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC1: *const RegisterBlock = 0x403c8000 as *const _;

/// Access functions for the ENC2 peripheral instance
pub mod ENC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403cc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ENC2
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ENC2_TAKEN: bool = false;

    /// Safe access to ENC2
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
            if ENC2_TAKEN {
                None
            } else {
                ENC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ENC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ENC2_TAKEN && inst.addr == INSTANCE.addr {
                ENC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ENC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ENC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ENC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC2: *const RegisterBlock = 0x403cc000 as *const _;

/// Access functions for the ENC3 peripheral instance
pub mod ENC3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403d0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ENC3
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ENC3_TAKEN: bool = false;

    /// Safe access to ENC3
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
            if ENC3_TAKEN {
                None
            } else {
                ENC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ENC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ENC3_TAKEN && inst.addr == INSTANCE.addr {
                ENC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ENC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ENC3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ENC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC3: *const RegisterBlock = 0x403d0000 as *const _;

/// Access functions for the ENC4 peripheral instance
pub mod ENC4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403d4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ENC4
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        FILT: 0x00000000,
        WTR: 0x00000000,
        POSD: 0x00000000,
        POSDH: 0x00000000,
        REV: 0x00000000,
        REVH: 0x00000000,
        UPOS: 0x00000000,
        LPOS: 0x00000000,
        UPOSH: 0x00000000,
        LPOSH: 0x00000000,
        UINIT: 0x00000000,
        LINIT: 0x00000000,
        IMR: 0x00000000,
        TST: 0x00000000,
        CTRL2: 0x00000000,
        UMOD: 0x00000000,
        LMOD: 0x00000000,
        UCOMP: 0x0000FFFF,
        LCOMP: 0x0000FFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ENC4_TAKEN: bool = false;

    /// Safe access to ENC4
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
            if ENC4_TAKEN {
                None
            } else {
                ENC4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ENC4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ENC4_TAKEN && inst.addr == INSTANCE.addr {
                ENC4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ENC4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ENC4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ENC4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ENC4: *const RegisterBlock = 0x403d4000 as *const _;
