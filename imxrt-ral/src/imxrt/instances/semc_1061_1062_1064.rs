#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEMC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::semc_v2::Instance;
pub use crate::imxrt::peripherals::semc_v2::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::semc_v2::{
    BMCR0, BMCR1, BR0, BR1, BR2, BR3, BR4, BR5, BR6, BR7, BR8, DBICR0, DBICR1, DLLCR, INTEN, INTR,
    IOCR, IPCMD, IPCR0, IPCR1, IPCR2, IPRXDAT, IPTXDAT, MCR, NANDCR0, NANDCR1, NANDCR2, NANDCR3,
    NORCR0, NORCR1, NORCR2, NORCR3, SDRAMCR0, SDRAMCR1, SDRAMCR2, SDRAMCR3, SRAMCR0, SRAMCR1,
    SRAMCR2, SRAMCR3, STS0, STS1, STS10, STS11, STS12, STS13, STS14, STS15, STS2, STS3, STS4, STS5,
    STS6, STS7, STS8, STS9,
};

/// Access functions for the SEMC peripheral instance
pub mod SEMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402f0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEMC
    pub const reset: ResetValues = ResetValues {
        MCR: 0x10000002,
        IOCR: 0x00000000,
        BMCR0: 0x00000000,
        BMCR1: 0x00000000,
        BR0: 0x8000001D,
        BR1: 0x8400001C,
        BR2: 0x8800001C,
        BR3: 0x8C00001C,
        BR4: 0x9E00001A,
        BR5: 0x90000018,
        BR6: 0x98000018,
        BR7: 0x9C00001A,
        BR8: 0x00000026,
        DLLCR: 0x00000100,
        INTEN: 0x00000000,
        INTR: 0x00000000,
        SDRAMCR0: 0x00000C26,
        SDRAMCR1: 0x00994934,
        SDRAMCR2: 0x80000EEE,
        SDRAMCR3: 0x40808000,
        NANDCR0: 0x00000000,
        NANDCR1: 0x00000000,
        NANDCR2: 0x00010410,
        NANDCR3: 0x00000000,
        NORCR0: 0x00000000,
        NORCR1: 0x00000000,
        NORCR2: 0x00000000,
        NORCR3: 0x00000000,
        SRAMCR0: 0x00000000,
        SRAMCR1: 0x00000000,
        SRAMCR2: 0x00000000,
        SRAMCR3: 0x00000000,
        DBICR0: 0x00000000,
        DBICR1: 0x00000000,
        IPCR0: 0x00000000,
        IPCR1: 0x00000000,
        IPCR2: 0x00000000,
        IPCMD: 0x00000000,
        IPTXDAT: 0x00000000,
        IPRXDAT: 0x00000000,
        STS0: 0x00000001,
        STS1: 0x00000000,
        STS2: 0x00000000,
        STS3: 0x00000000,
        STS4: 0x00000000,
        STS5: 0x00000000,
        STS6: 0x00000000,
        STS7: 0x00000000,
        STS8: 0x00000000,
        STS9: 0x00000000,
        STS10: 0x00000000,
        STS11: 0x00000000,
        STS12: 0x00000000,
        STS13: 0x00000100,
        STS14: 0x00000000,
        STS15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEMC_TAKEN: bool = false;

    /// Safe access to SEMC
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
            if SEMC_TAKEN {
                None
            } else {
                SEMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEMC_TAKEN && inst.addr == INSTANCE.addr {
                SEMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEMC: *const RegisterBlock = 0x402f0000 as *const _;
