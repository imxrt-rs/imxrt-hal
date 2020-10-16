#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AIPSTZ Control Registers
//!
//! Used by: imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::aipstz::Instance;
pub use crate::imxrt::peripherals::aipstz::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::aipstz::{MPR, OPACR, OPACR1, OPACR2, OPACR3, OPACR4};

/// Access functions for the AIPSTZ1 peripheral instance
pub mod AIPSTZ1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4007c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AIPSTZ1
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AIPSTZ1_TAKEN: bool = false;

    /// Safe access to AIPSTZ1
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
            if AIPSTZ1_TAKEN {
                None
            } else {
                AIPSTZ1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AIPSTZ1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AIPSTZ1_TAKEN && inst.addr == INSTANCE.addr {
                AIPSTZ1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AIPSTZ1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AIPSTZ1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ1: *const RegisterBlock = 0x4007c000 as *const _;

/// Access functions for the AIPSTZ2 peripheral instance
pub mod AIPSTZ2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4017c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AIPSTZ2
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AIPSTZ2_TAKEN: bool = false;

    /// Safe access to AIPSTZ2
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
            if AIPSTZ2_TAKEN {
                None
            } else {
                AIPSTZ2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AIPSTZ2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AIPSTZ2_TAKEN && inst.addr == INSTANCE.addr {
                AIPSTZ2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AIPSTZ2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AIPSTZ2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ2: *const RegisterBlock = 0x4017c000 as *const _;

/// Access functions for the AIPSTZ3 peripheral instance
pub mod AIPSTZ3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4027c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AIPSTZ3
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AIPSTZ3_TAKEN: bool = false;

    /// Safe access to AIPSTZ3
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
            if AIPSTZ3_TAKEN {
                None
            } else {
                AIPSTZ3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AIPSTZ3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AIPSTZ3_TAKEN && inst.addr == INSTANCE.addr {
                AIPSTZ3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AIPSTZ3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AIPSTZ3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ3: *const RegisterBlock = 0x4027c000 as *const _;

/// Access functions for the AIPSTZ4 peripheral instance
pub mod AIPSTZ4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4037c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AIPSTZ4
    pub const reset: ResetValues = ResetValues {
        MPR: 0x77000000,
        OPACR: 0x44444444,
        OPACR1: 0x44444444,
        OPACR2: 0x44444444,
        OPACR3: 0x44444444,
        OPACR4: 0x44444444,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AIPSTZ4_TAKEN: bool = false;

    /// Safe access to AIPSTZ4
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
            if AIPSTZ4_TAKEN {
                None
            } else {
                AIPSTZ4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AIPSTZ4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AIPSTZ4_TAKEN && inst.addr == INSTANCE.addr {
                AIPSTZ4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AIPSTZ4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AIPSTZ4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AIPSTZ4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AIPSTZ4: *const RegisterBlock = 0x4037c000 as *const _;
