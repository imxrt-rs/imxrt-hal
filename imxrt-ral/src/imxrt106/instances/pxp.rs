#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PXP v2.0 Register Reference Index
//!
//! Used by: imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::pxp::Instance;
pub use crate::imxrt106::peripherals::pxp::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::pxp::{
    AS_BUF, AS_CLRKEYHIGH, AS_CLRKEYLOW, AS_CTRL, AS_PITCH, CSC1_COEF0, CSC1_COEF1, CSC1_COEF2,
    CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, NEXT, OUT_AS_LRC, OUT_AS_ULC, OUT_BUF, OUT_BUF2, OUT_CTRL,
    OUT_CTRL_CLR, OUT_CTRL_SET, OUT_CTRL_TOG, OUT_LRC, OUT_PITCH, OUT_PS_LRC, OUT_PS_ULC,
    PORTER_DUFF_CTRL, POWER, PS_BACKGROUND, PS_BUF, PS_CLRKEYHIGH, PS_CLRKEYLOW, PS_CTRL,
    PS_CTRL_CLR, PS_CTRL_SET, PS_CTRL_TOG, PS_OFFSET, PS_PITCH, PS_SCALE, PS_UBUF, PS_VBUF, STAT,
    STAT_CLR, STAT_SET, STAT_TOG,
};

/// Access functions for the PXP peripheral instance
pub mod PXP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402b4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PXP
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        CTRL_SET: 0xC0000000,
        CTRL_CLR: 0xC0000000,
        CTRL_TOG: 0xC0000000,
        STAT: 0x00000000,
        STAT_SET: 0x00000000,
        STAT_CLR: 0x00000000,
        STAT_TOG: 0x00000000,
        OUT_CTRL: 0x00000000,
        OUT_CTRL_SET: 0x00000000,
        OUT_CTRL_CLR: 0x00000000,
        OUT_CTRL_TOG: 0x00000000,
        OUT_BUF: 0x00000000,
        OUT_BUF2: 0x00000000,
        OUT_PITCH: 0x00000000,
        OUT_LRC: 0x00000000,
        OUT_PS_ULC: 0x00000000,
        OUT_PS_LRC: 0x00000000,
        OUT_AS_ULC: 0x00000000,
        OUT_AS_LRC: 0x00000000,
        PS_CTRL: 0x00000000,
        PS_CTRL_SET: 0x00000000,
        PS_CTRL_CLR: 0x00000000,
        PS_CTRL_TOG: 0x00000000,
        PS_BUF: 0x00000000,
        PS_UBUF: 0x00000000,
        PS_VBUF: 0x00000000,
        PS_PITCH: 0x00000000,
        PS_BACKGROUND: 0x00000000,
        PS_SCALE: 0x10001000,
        PS_OFFSET: 0x00000000,
        PS_CLRKEYLOW: 0x00FFFFFF,
        PS_CLRKEYHIGH: 0x00000000,
        AS_CTRL: 0x00000000,
        AS_BUF: 0x00000000,
        AS_PITCH: 0x00000000,
        AS_CLRKEYLOW: 0x00FFFFFF,
        AS_CLRKEYHIGH: 0x00000000,
        CSC1_COEF0: 0x04000000,
        CSC1_COEF1: 0x01230208,
        CSC1_COEF2: 0x079B076C,
        POWER: 0x00000000,
        NEXT: 0x00000000,
        PORTER_DUFF_CTRL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PXP_TAKEN: bool = false;

    /// Safe access to PXP
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
            if PXP_TAKEN {
                None
            } else {
                PXP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PXP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PXP_TAKEN && inst.addr == INSTANCE.addr {
                PXP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PXP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PXP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PXP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PXP: *const RegisterBlock = 0x402b4000 as *const _;
