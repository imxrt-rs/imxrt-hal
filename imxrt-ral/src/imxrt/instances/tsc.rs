#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Touch Screen Controller
//!
//! Used by: imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::tsc::Instance;
pub use crate::imxrt::peripherals::tsc::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::tsc::{
    BASIC_SETTING, DEBUG_MODE, DEBUG_MODE2, FLOW_CONTROL, INT_EN, INT_SIG_EN, INT_STATUS,
    MEASEURE_VALUE, PRE_CHARGE_TIME,
};

/// Access functions for the TSC peripheral instance
pub mod TSC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400e0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TSC
    pub const reset: ResetValues = ResetValues {
        BASIC_SETTING: 0x00000000,
        PRE_CHARGE_TIME: 0x00000000,
        FLOW_CONTROL: 0x00000000,
        MEASEURE_VALUE: 0x00000000,
        INT_EN: 0x00000000,
        INT_SIG_EN: 0x00000000,
        INT_STATUS: 0x00000000,
        DEBUG_MODE: 0x00000000,
        DEBUG_MODE2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TSC_TAKEN: bool = false;

    /// Safe access to TSC
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
            if TSC_TAKEN {
                None
            } else {
                TSC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TSC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TSC_TAKEN && inst.addr == INSTANCE.addr {
                TSC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TSC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TSC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TSC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TSC: *const RegisterBlock = 0x400e0000 as *const _;
