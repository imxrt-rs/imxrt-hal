#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OCOTP
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::ocotp_v2::Instance;
pub use crate::imxrt::peripherals::ocotp_v2::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::ocotp_v2::{
    ANA0, ANA1, ANA2, CFG0, CFG1, CFG2, CFG3, CFG4, CFG5, CFG6, CRC_ADDR, CRC_VALUE, CTRL,
    CTRL_CLR, CTRL_SET, CTRL_TOG, DATA, GP1, GP2, GP30, GP31, GP32, GP33, GP40, GP41, GP42, GP43,
    LOCK, MAC0, MAC1, MAC2, MEM0, MEM1, MEM2, MEM3, MEM4, MISC_CONF0, MISC_CONF1, OTPMK0, OTPMK1,
    OTPMK2, OTPMK3, OTPMK4, OTPMK5, OTPMK6, OTPMK7, OTPMK_CRC32, READ_CTRL, READ_FUSE_DATA,
    ROM_PATCH0, ROM_PATCH1, ROM_PATCH2, ROM_PATCH3, ROM_PATCH4, ROM_PATCH5, ROM_PATCH6, ROM_PATCH7,
    SCS, SCS_CLR, SCS_SET, SCS_TOG, SJC_RESP0, SJC_RESP1, SRK0, SRK1, SRK2, SRK3, SRK4, SRK5, SRK6,
    SRK7, SRK_REVOKE, SW_GP1, SW_GP20, SW_GP21, SW_GP22, SW_GP23, SW_STICKY, TIMING, TIMING2,
    VERSION,
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
        CTRL: 0x00000000,
        CTRL_SET: 0x00000000,
        CTRL_CLR: 0x00000000,
        CTRL_TOG: 0x00000000,
        TIMING: 0x0C0D9755,
        DATA: 0x00000000,
        READ_CTRL: 0x00000000,
        READ_FUSE_DATA: 0x00000000,
        SW_STICKY: 0x00000000,
        SCS: 0x00000000,
        SCS_SET: 0x00000000,
        SCS_CLR: 0x00000000,
        SCS_TOG: 0x00000000,
        CRC_ADDR: 0x00000000,
        CRC_VALUE: 0x00000000,
        VERSION: 0x03000000,
        TIMING2: 0x01C30092,
        LOCK: 0x00000000,
        CFG0: 0x00000000,
        CFG1: 0x00000000,
        CFG2: 0x00000000,
        CFG3: 0x00000000,
        CFG4: 0x00000000,
        CFG5: 0x00000000,
        CFG6: 0x00000000,
        MEM0: 0x00000000,
        MEM1: 0x00000000,
        MEM2: 0x00000000,
        MEM3: 0x00000000,
        MEM4: 0x00000000,
        ANA0: 0x00000000,
        ANA1: 0x00000000,
        ANA2: 0x00000000,
        OTPMK0: 0x00000000,
        OTPMK1: 0x00000000,
        OTPMK2: 0x00000000,
        OTPMK3: 0x00000000,
        OTPMK4: 0x00000000,
        OTPMK5: 0x00000000,
        OTPMK6: 0x00000000,
        OTPMK7: 0x00000000,
        SRK0: 0x00000000,
        SRK1: 0x00000000,
        SRK2: 0x00000000,
        SRK3: 0x00000000,
        SRK4: 0x00000000,
        SRK5: 0x00000000,
        SRK6: 0x00000000,
        SRK7: 0x00000000,
        SJC_RESP0: 0x00000000,
        SJC_RESP1: 0x00000000,
        MAC0: 0x00000000,
        MAC1: 0x00000000,
        MAC2: 0x00000000,
        OTPMK_CRC32: 0x00000000,
        GP1: 0x00000000,
        GP2: 0x00000000,
        SW_GP1: 0x00000000,
        SW_GP20: 0x00000000,
        SW_GP21: 0x00000000,
        SW_GP22: 0x00000000,
        SW_GP23: 0x00000000,
        MISC_CONF0: 0x00000000,
        MISC_CONF1: 0x00000000,
        SRK_REVOKE: 0x00000000,
        ROM_PATCH0: 0x00000000,
        ROM_PATCH1: 0x00000000,
        ROM_PATCH2: 0x00000000,
        ROM_PATCH3: 0x00000000,
        ROM_PATCH4: 0x00000000,
        ROM_PATCH5: 0x00000000,
        ROM_PATCH6: 0x00000000,
        ROM_PATCH7: 0x00000000,
        GP30: 0x00000000,
        GP31: 0x00000000,
        GP32: 0x00000000,
        GP33: 0x00000000,
        GP40: 0x00000000,
        GP41: 0x00000000,
        GP42: 0x00000000,
        GP43: 0x00000000,
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
