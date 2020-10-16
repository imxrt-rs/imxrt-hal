#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch
//!
//! Used by: imxrt1015, imxrt1021

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::xbarb::Instance;
pub use crate::imxrt::peripherals::xbarb::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::xbarb::{SEL0, SEL1, SEL2, SEL3, SEL4, SEL5, SEL6, SEL7};

/// Access functions for the XBARB peripheral instance
pub mod XBARB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403c0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in XBARB
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut XBARB_TAKEN: bool = false;

    /// Safe access to XBARB
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
            if XBARB_TAKEN {
                None
            } else {
                XBARB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to XBARB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if XBARB_TAKEN && inst.addr == INSTANCE.addr {
                XBARB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal XBARB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        XBARB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to XBARB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARB: *const RegisterBlock = 0x403c0000 as *const _;
