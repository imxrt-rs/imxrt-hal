#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EWM
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::ewm::Instance;
pub use crate::imxrt::peripherals::ewm::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::ewm::{CLKCTRL, CLKPRESCALER, CMPH, CMPL, CTRL, SERV};

/// Access functions for the EWM peripheral instance
pub mod EWM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400b4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EWM
    pub const reset: ResetValues = ResetValues {
        CTRL: 0x00000000,
        SERV: 0x00000000,
        CMPL: 0x00000000,
        CMPH: 0x000000FF,
        CLKCTRL: 0x00000000,
        CLKPRESCALER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EWM_TAKEN: bool = false;

    /// Safe access to EWM
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
            if EWM_TAKEN {
                None
            } else {
                EWM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EWM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EWM_TAKEN && inst.addr == INSTANCE.addr {
                EWM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EWM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EWM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EWM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EWM: *const RegisterBlock = 0x400b4000 as *const _;
