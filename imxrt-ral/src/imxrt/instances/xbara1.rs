#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch
//!
//! Used by: imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::xbara::Instance;
pub use crate::imxrt::peripherals::xbara::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::xbara::{
    CTRL0, CTRL1, SEL0, SEL1, SEL10, SEL11, SEL12, SEL13, SEL14, SEL15, SEL16, SEL17, SEL18, SEL19,
    SEL2, SEL20, SEL21, SEL22, SEL23, SEL24, SEL25, SEL26, SEL27, SEL28, SEL29, SEL3, SEL30, SEL31,
    SEL32, SEL33, SEL34, SEL35, SEL36, SEL37, SEL38, SEL39, SEL4, SEL40, SEL41, SEL42, SEL43,
    SEL44, SEL45, SEL46, SEL47, SEL48, SEL49, SEL5, SEL50, SEL51, SEL52, SEL53, SEL54, SEL55,
    SEL56, SEL57, SEL58, SEL59, SEL6, SEL60, SEL61, SEL62, SEL63, SEL64, SEL65, SEL7, SEL8, SEL9,
};

/// Access functions for the XBARA1 peripheral instance
pub mod XBARA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403bc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in XBARA1
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
        SEL8: 0x00000000,
        SEL9: 0x00000000,
        SEL10: 0x00000000,
        SEL11: 0x00000000,
        SEL12: 0x00000000,
        SEL13: 0x00000000,
        SEL14: 0x00000000,
        SEL15: 0x00000000,
        SEL16: 0x00000000,
        SEL17: 0x00000000,
        SEL18: 0x00000000,
        SEL19: 0x00000000,
        SEL20: 0x00000000,
        SEL21: 0x00000000,
        SEL22: 0x00000000,
        SEL23: 0x00000000,
        SEL24: 0x00000000,
        SEL25: 0x00000000,
        SEL26: 0x00000000,
        SEL27: 0x00000000,
        SEL28: 0x00000000,
        SEL29: 0x00000000,
        SEL30: 0x00000000,
        SEL31: 0x00000000,
        SEL32: 0x00000000,
        SEL33: 0x00000000,
        SEL34: 0x00000000,
        SEL35: 0x00000000,
        SEL36: 0x00000000,
        SEL37: 0x00000000,
        SEL38: 0x00000000,
        SEL39: 0x00000000,
        SEL40: 0x00000000,
        SEL41: 0x00000000,
        SEL42: 0x00000000,
        SEL43: 0x00000000,
        SEL44: 0x00000000,
        SEL45: 0x00000000,
        SEL46: 0x00000000,
        SEL47: 0x00000000,
        SEL48: 0x00000000,
        SEL49: 0x00000000,
        SEL50: 0x00000000,
        SEL51: 0x00000000,
        SEL52: 0x00000000,
        SEL53: 0x00000000,
        SEL54: 0x00000000,
        SEL55: 0x00000000,
        SEL56: 0x00000000,
        SEL57: 0x00000000,
        SEL58: 0x00000000,
        SEL59: 0x00000000,
        SEL60: 0x00000000,
        SEL61: 0x00000000,
        SEL62: 0x00000000,
        SEL63: 0x00000000,
        SEL64: 0x00000000,
        SEL65: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut XBARA1_TAKEN: bool = false;

    /// Safe access to XBARA1
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
            if XBARA1_TAKEN {
                None
            } else {
                XBARA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to XBARA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if XBARA1_TAKEN && inst.addr == INSTANCE.addr {
                XBARA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal XBARA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        XBARA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to XBARA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARA1: *const RegisterBlock = 0x403bc000 as *const _;
