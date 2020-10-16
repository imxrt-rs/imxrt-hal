#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC
//!
//! Used by: imxrt1015, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::gpc_v1::Instance;
pub use crate::imxrt::peripherals::gpc_v1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::gpc_v1::{
    CNTR, IMR1, IMR2, IMR3, IMR4, IMR5, ISR1, ISR2, ISR3, ISR4, ISR5,
};

/// Access functions for the GPC peripheral instance
pub mod GPC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400f4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPC
    pub const reset: ResetValues = ResetValues {
        CNTR: 0x00520000,
        IMR1: 0x00000000,
        IMR2: 0x00000000,
        IMR3: 0x00000000,
        IMR4: 0x00000000,
        ISR1: 0x00000000,
        ISR2: 0x00000000,
        ISR3: 0x00000000,
        ISR4: 0x00000000,
        IMR5: 0x00000000,
        ISR5: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPC_TAKEN: bool = false;

    /// Safe access to GPC
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
            if GPC_TAKEN {
                None
            } else {
                GPC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPC_TAKEN && inst.addr == INSTANCE.addr {
                GPC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPC: *const RegisterBlock = 0x400f4000 as *const _;
