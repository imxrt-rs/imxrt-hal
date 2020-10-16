#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPT
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::gpt::Instance;
pub use crate::imxrt::peripherals::gpt::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::gpt::{CNT, CR, ICR1, ICR2, IR, OCR1, OCR2, OCR3, PR, SR};

/// Access functions for the GPT1 peripheral instance
pub mod GPT1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401ec000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPT1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPT1_TAKEN: bool = false;

    /// Safe access to GPT1
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
            if GPT1_TAKEN {
                None
            } else {
                GPT1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPT1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPT1_TAKEN && inst.addr == INSTANCE.addr {
                GPT1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPT1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPT1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPT1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT1: *const RegisterBlock = 0x401ec000 as *const _;

/// Access functions for the GPT2 peripheral instance
pub mod GPT2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401f0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPT2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        PR: 0x00000000,
        SR: 0x00000000,
        IR: 0x00000000,
        OCR1: 0xFFFFFFFF,
        OCR2: 0xFFFFFFFF,
        OCR3: 0xFFFFFFFF,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        CNT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPT2_TAKEN: bool = false;

    /// Safe access to GPT2
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
            if GPT2_TAKEN {
                None
            } else {
                GPT2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPT2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPT2_TAKEN && inst.addr == INSTANCE.addr {
                GPT2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPT2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPT2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPT2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPT2: *const RegisterBlock = 0x401f0000 as *const _;
