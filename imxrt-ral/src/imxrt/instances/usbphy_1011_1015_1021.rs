#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHY Register Reference Index
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::usbphy::Instance;
pub use crate::imxrt::peripherals::usbphy::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::usbphy::{
    CTRL, CTRL_CLR, CTRL_SET, CTRL_TOG, DEBUG, DEBUG0_STATUS, DEBUG1, DEBUG1_CLR, DEBUG1_SET,
    DEBUG1_TOG, DEBUG_CLR, DEBUG_SET, DEBUG_TOG, PWD, PWD_CLR, PWD_SET, PWD_TOG, RX, RX_CLR,
    RX_SET, RX_TOG, STATUS, TX, TX_CLR, TX_SET, TX_TOG, VERSION,
};

/// Access functions for the USBPHY peripheral instance
pub mod USBPHY {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d9000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USBPHY
    pub const reset: ResetValues = ResetValues {
        PWD: 0x001E1C00,
        PWD_SET: 0x001E1C00,
        PWD_CLR: 0x001E1C00,
        PWD_TOG: 0x001E1C00,
        TX: 0x10060607,
        TX_SET: 0x10060607,
        TX_CLR: 0x10060607,
        TX_TOG: 0x10060607,
        RX: 0x00000000,
        RX_SET: 0x00000000,
        RX_CLR: 0x00000000,
        RX_TOG: 0x00000000,
        CTRL: 0xC0200000,
        CTRL_SET: 0xC0200000,
        CTRL_CLR: 0xC0200000,
        CTRL_TOG: 0xC0200000,
        STATUS: 0x00000000,
        DEBUG: 0x7F180000,
        DEBUG_SET: 0x7F180000,
        DEBUG_CLR: 0x7F180000,
        DEBUG_TOG: 0x7F180000,
        DEBUG0_STATUS: 0x00000000,
        DEBUG1: 0x00001000,
        DEBUG1_SET: 0x00001000,
        DEBUG1_CLR: 0x00001000,
        DEBUG1_TOG: 0x00001000,
        VERSION: 0x04030000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USBPHY_TAKEN: bool = false;

    /// Safe access to USBPHY
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
            if USBPHY_TAKEN {
                None
            } else {
                USBPHY_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USBPHY
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USBPHY_TAKEN && inst.addr == INSTANCE.addr {
                USBPHY_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USBPHY
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USBPHY_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USBPHY
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHY: *const RegisterBlock = 0x400d9000 as *const _;
