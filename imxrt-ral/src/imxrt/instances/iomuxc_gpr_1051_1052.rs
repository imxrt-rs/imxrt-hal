#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC_GPR
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::iomuxc_gpr_v1::Instance;
pub use crate::imxrt::peripherals::iomuxc_gpr_v1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::iomuxc_gpr_v1::{
    GPR0, GPR1, GPR10, GPR11, GPR12, GPR13, GPR14, GPR15, GPR16, GPR17, GPR18, GPR19, GPR2, GPR20,
    GPR21, GPR22, GPR23, GPR24, GPR25, GPR3, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9,
};

/// Access functions for the IOMUXC_GPR peripheral instance
pub mod IOMUXC_GPR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400ac000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC_GPR
    pub const reset: ResetValues = ResetValues {
        GPR0: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x0000FFF0,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000007,
        GPR11: 0x00000000,
        GPR12: 0x00000000,
        GPR13: 0x00000000,
        GPR14: 0x00AA0000,
        GPR15: 0xFFFFFFFF,
        GPR16: 0x00200003,
        GPR17: 0x00000000,
        GPR18: 0x00000000,
        GPR19: 0x00000000,
        GPR20: 0x00000000,
        GPR21: 0x00000000,
        GPR22: 0x00000000,
        GPR23: 0x00000000,
        GPR24: 0x00000000,
        GPR25: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_GPR_TAKEN: bool = false;

    /// Safe access to IOMUXC_GPR
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
            if IOMUXC_GPR_TAKEN {
                None
            } else {
                IOMUXC_GPR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC_GPR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_GPR_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_GPR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC_GPR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_GPR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC_GPR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_GPR: *const RegisterBlock = 0x400ac000 as *const _;
