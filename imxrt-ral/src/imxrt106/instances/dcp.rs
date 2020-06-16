#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCP register reference index
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::dcp::Instance;
pub use crate::imxrt106::peripherals::dcp::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::dcp::{
    CAPABILITY0, CAPABILITY1, CH0CMDPTR, CH0OPTS, CH0OPTS_CLR, CH0OPTS_SET, CH0OPTS_TOG, CH0SEMA,
    CH0STAT, CH0STAT_CLR, CH0STAT_SET, CH0STAT_TOG, CH1CMDPTR, CH1OPTS, CH1OPTS_CLR, CH1OPTS_SET,
    CH1OPTS_TOG, CH1SEMA, CH1STAT, CH1STAT_CLR, CH1STAT_SET, CH1STAT_TOG, CH2CMDPTR, CH2OPTS,
    CH2OPTS_CLR, CH2OPTS_SET, CH2OPTS_TOG, CH2SEMA, CH2STAT, CH2STAT_CLR, CH2STAT_SET, CH2STAT_TOG,
    CH3CMDPTR, CH3OPTS, CH3OPTS_CLR, CH3OPTS_SET, CH3OPTS_TOG, CH3SEMA, CH3STAT, CH3STAT_CLR,
    CH3STAT_SET, CH3STAT_TOG, CHANNELCTRL, CHANNELCTRL_CLR, CHANNELCTRL_SET, CHANNELCTRL_TOG,
    CONTEXT, CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DBGDATA, DBGSELECT, KEY, KEYDATA, PACKET0,
    PACKET1, PACKET2, PACKET3, PACKET4, PACKET5, PACKET6, PAGETABLE, STAT, STAT_CLR, STAT_SET,
    STAT_TOG, VERSION,
};

/// Access functions for the DCP peripheral instance
pub mod DCP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402fc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DCP
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xF0800000,
        CTRL_SET: 0xF0800000,
        CTRL_CLR: 0xF0800000,
        CTRL_TOG: 0xF0800000,
        STAT: 0x10000000,
        STAT_SET: 0x10000000,
        STAT_CLR: 0x10000000,
        STAT_TOG: 0x10000000,
        CHANNELCTRL: 0x00000000,
        CHANNELCTRL_SET: 0x00000000,
        CHANNELCTRL_CLR: 0x00000000,
        CHANNELCTRL_TOG: 0x00000000,
        CAPABILITY0: 0x00000404,
        CAPABILITY1: 0x00070001,
        CONTEXT: 0x00000000,
        KEY: 0x00000000,
        KEYDATA: 0x00000000,
        PACKET0: 0x00000000,
        PACKET1: 0x00000000,
        PACKET2: 0x00000000,
        PACKET3: 0x00000000,
        PACKET4: 0x00000000,
        PACKET5: 0x00000000,
        PACKET6: 0x00000000,
        CH0CMDPTR: 0x00000000,
        CH0SEMA: 0x00000000,
        CH0STAT: 0x00000000,
        CH0STAT_SET: 0x00000000,
        CH0STAT_CLR: 0x00000000,
        CH0STAT_TOG: 0x00000000,
        CH0OPTS: 0x00000000,
        CH0OPTS_SET: 0x00000000,
        CH0OPTS_CLR: 0x00000000,
        CH0OPTS_TOG: 0x00000000,
        CH1CMDPTR: 0x00000000,
        CH1SEMA: 0x00000000,
        CH1STAT: 0x00000000,
        CH1STAT_SET: 0x00000000,
        CH1STAT_CLR: 0x00000000,
        CH1STAT_TOG: 0x00000000,
        CH1OPTS: 0x00000000,
        CH1OPTS_SET: 0x00000000,
        CH1OPTS_CLR: 0x00000000,
        CH1OPTS_TOG: 0x00000000,
        CH2CMDPTR: 0x00000000,
        CH2SEMA: 0x00000000,
        CH2STAT: 0x00000000,
        CH2STAT_SET: 0x00000000,
        CH2STAT_CLR: 0x00000000,
        CH2STAT_TOG: 0x00000000,
        CH2OPTS: 0x00000000,
        CH2OPTS_SET: 0x00000000,
        CH2OPTS_CLR: 0x00000000,
        CH2OPTS_TOG: 0x00000000,
        CH3CMDPTR: 0x00000000,
        CH3SEMA: 0x00000000,
        CH3STAT: 0x00000000,
        CH3STAT_SET: 0x00000000,
        CH3STAT_CLR: 0x00000000,
        CH3STAT_TOG: 0x00000000,
        CH3OPTS: 0x00000000,
        CH3OPTS_SET: 0x00000000,
        CH3OPTS_CLR: 0x00000000,
        CH3OPTS_TOG: 0x00000000,
        DBGSELECT: 0x00000000,
        DBGDATA: 0x00000000,
        PAGETABLE: 0x00000000,
        VERSION: 0x02010000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DCP_TAKEN: bool = false;

    /// Safe access to DCP
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
            if DCP_TAKEN {
                None
            } else {
                DCP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DCP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DCP_TAKEN && inst.addr == INSTANCE.addr {
                DCP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DCP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DCP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DCP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCP: *const RegisterBlock = 0x402fc000 as *const _;
