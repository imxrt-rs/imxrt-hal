#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available
//!
//! Used by: imxrt1021, imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::ocotp_v1::Instance;
pub use crate::imxrt::peripherals::ocotp_v1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::ocotp_v1::{
    HW_OCOTP_ANA0, HW_OCOTP_ANA1, HW_OCOTP_ANA2, HW_OCOTP_CFG0, HW_OCOTP_CFG1, HW_OCOTP_CFG2,
    HW_OCOTP_CFG3, HW_OCOTP_CFG4, HW_OCOTP_CFG5, HW_OCOTP_CFG6, HW_OCOTP_CTRL, HW_OCOTP_CTRL_CLR,
    HW_OCOTP_CTRL_SET, HW_OCOTP_CTRL_TOG, HW_OCOTP_DATA, HW_OCOTP_GP1, HW_OCOTP_GP2, HW_OCOTP_GP3,
    HW_OCOTP_LOCK, HW_OCOTP_MAC0, HW_OCOTP_MAC1, HW_OCOTP_MEM0, HW_OCOTP_MEM1, HW_OCOTP_MEM2,
    HW_OCOTP_MEM3, HW_OCOTP_MEM4, HW_OCOTP_MISC_CONF0, HW_OCOTP_MISC_CONF1, HW_OCOTP_READ_CTRL,
    HW_OCOTP_READ_FUSE_DATA, HW_OCOTP_SCS, HW_OCOTP_SCS_CLR, HW_OCOTP_SCS_SET, HW_OCOTP_SCS_TOG,
    HW_OCOTP_SJC_RESP0, HW_OCOTP_SJC_RESP1, HW_OCOTP_SRK0, HW_OCOTP_SRK1, HW_OCOTP_SRK2,
    HW_OCOTP_SRK3, HW_OCOTP_SRK4, HW_OCOTP_SRK5, HW_OCOTP_SRK6, HW_OCOTP_SRK7, HW_OCOTP_SRK_REVOKE,
    HW_OCOTP_SW_GP1, HW_OCOTP_SW_GP20, HW_OCOTP_SW_GP21, HW_OCOTP_SW_GP22, HW_OCOTP_SW_GP23,
    HW_OCOTP_SW_STICKY, HW_OCOTP_TIMING, HW_OCOTP_TIMING2, HW_OCOTP_VERSION,
};

/// Access functions for the OCOTP peripheral instance
pub mod OCOTP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401f4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCOTP
    pub const reset: ResetValues = ResetValues {
        HW_OCOTP_CTRL: 0x00000000,
        HW_OCOTP_CTRL_SET: 0x00000000,
        HW_OCOTP_CTRL_CLR: 0x00000000,
        HW_OCOTP_CTRL_TOG: 0x00000000,
        HW_OCOTP_TIMING: 0x060D9755,
        HW_OCOTP_DATA: 0x00000000,
        HW_OCOTP_READ_CTRL: 0x00000000,
        HW_OCOTP_READ_FUSE_DATA: 0x00000000,
        HW_OCOTP_SW_STICKY: 0x00000000,
        HW_OCOTP_SCS: 0x00000000,
        HW_OCOTP_SCS_SET: 0x00000000,
        HW_OCOTP_SCS_CLR: 0x00000000,
        HW_OCOTP_SCS_TOG: 0x00000000,
        HW_OCOTP_VERSION: 0x06000000,
        HW_OCOTP_TIMING2: 0x01C30092,
        HW_OCOTP_LOCK: 0x00000000,
        HW_OCOTP_CFG0: 0x00000000,
        HW_OCOTP_CFG1: 0x00000000,
        HW_OCOTP_CFG2: 0x00000000,
        HW_OCOTP_CFG3: 0x00000000,
        HW_OCOTP_CFG4: 0x00000000,
        HW_OCOTP_CFG5: 0x00000000,
        HW_OCOTP_CFG6: 0x00000000,
        HW_OCOTP_MEM0: 0x00000000,
        HW_OCOTP_MEM1: 0x00000000,
        HW_OCOTP_MEM2: 0x00000000,
        HW_OCOTP_MEM3: 0x00000000,
        HW_OCOTP_MEM4: 0x00000000,
        HW_OCOTP_ANA0: 0x00000000,
        HW_OCOTP_ANA1: 0x00000000,
        HW_OCOTP_ANA2: 0x00000000,
        HW_OCOTP_SRK0: 0x00000000,
        HW_OCOTP_SRK1: 0x00000000,
        HW_OCOTP_SRK2: 0x00000000,
        HW_OCOTP_SRK3: 0x00000000,
        HW_OCOTP_SRK4: 0x00000000,
        HW_OCOTP_SRK5: 0x00000000,
        HW_OCOTP_SRK6: 0x00000000,
        HW_OCOTP_SRK7: 0x00000000,
        HW_OCOTP_SJC_RESP0: 0x00000000,
        HW_OCOTP_SJC_RESP1: 0x00000000,
        HW_OCOTP_MAC0: 0x00000000,
        HW_OCOTP_MAC1: 0x00000000,
        HW_OCOTP_GP3: 0x00000000,
        HW_OCOTP_GP1: 0x00000000,
        HW_OCOTP_GP2: 0x00000000,
        HW_OCOTP_SW_GP1: 0x00000000,
        HW_OCOTP_SW_GP20: 0x00000000,
        HW_OCOTP_SW_GP21: 0x00000000,
        HW_OCOTP_SW_GP22: 0x00000000,
        HW_OCOTP_SW_GP23: 0x00000000,
        HW_OCOTP_MISC_CONF0: 0x00000000,
        HW_OCOTP_MISC_CONF1: 0x00000000,
        HW_OCOTP_SRK_REVOKE: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCOTP_TAKEN: bool = false;

    /// Safe access to OCOTP
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
            if OCOTP_TAKEN {
                None
            } else {
                OCOTP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCOTP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCOTP_TAKEN && inst.addr == INSTANCE.addr {
                OCOTP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCOTP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCOTP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCOTP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCOTP: *const RegisterBlock = 0x401f4000 as *const _;
