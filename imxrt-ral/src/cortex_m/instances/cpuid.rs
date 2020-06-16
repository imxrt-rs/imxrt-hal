#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CPUID
//!
//! Used by: armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::cpuid::Instance;
pub use crate::cortex_m::peripherals::cpuid::{
    Base, AFR0, CCSIDR, CLIDR, CSSELR, CTR, DFR0, ISAR0, ISAR1, ISAR2, ISAR3, ISAR4, MMFR0, MMFR1,
    MMFR2, MMFR3, PFR0, PFR1,
};
pub use crate::cortex_m::peripherals::cpuid::{RegisterBlock, ResetValues};

/// Access functions for the CPUID peripheral instance
pub mod CPUID {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ed00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CPUID
    pub const reset: ResetValues = ResetValues {
        Base: 0x00000000,
        PFR0: 0x00000000,
        PFR1: 0x00000000,
        DFR0: 0x00000000,
        AFR0: 0x00000000,
        MMFR0: 0x00000000,
        MMFR1: 0x00000000,
        MMFR2: 0x00000000,
        MMFR3: 0x00000000,
        ISAR0: 0x00000000,
        ISAR1: 0x00000000,
        ISAR2: 0x00000000,
        ISAR3: 0x00000000,
        ISAR4: 0x00000000,
        CLIDR: 0x00000000,
        CTR: 0x00000000,
        CCSIDR: 0x00000000,
        CSSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CPUID_TAKEN: bool = false;

    /// Safe access to CPUID
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
            if CPUID_TAKEN {
                None
            } else {
                CPUID_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CPUID
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CPUID_TAKEN && inst.addr == INSTANCE.addr {
                CPUID_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CPUID
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CPUID_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CPUID
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CPUID: *const RegisterBlock = 0xe000ed00 as *const _;
