#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::usb1_v1::Instance;
pub use crate::imxrt::peripherals::usb1_v1::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::usb1_v1::{
    ASYNCLISTADDR, BURSTSIZE, CAPLENGTH, CONFIGFLAG, DCCPARAMS, DCIVERSION, DEVICEADDR,
    ENDPTCOMPLETE, ENDPTCTRL0, ENDPTCTRL1, ENDPTCTRL2, ENDPTCTRL3, ENDPTCTRL4, ENDPTCTRL5,
    ENDPTCTRL6, ENDPTCTRL7, ENDPTFLUSH, ENDPTNAK, ENDPTNAKEN, ENDPTPRIME, ENDPTSETUPSTAT,
    ENDPTSTAT, FRINDEX, GPTIMER0CTRL, GPTIMER0LD, GPTIMER1CTRL, GPTIMER1LD, HCCPARAMS, HCIVERSION,
    HCSPARAMS, HWDEVICE, HWGENERAL, HWHOST, HWRXBUF, HWTXBUF, ID, OTGSC, PORTSC1, SBUSCFG,
    TXFILLTUNING, USBCMD, USBINTR, USBMODE, USBSTS,
};

/// Access functions for the USB1 peripheral instance
pub mod USB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402e0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB1
    pub const reset: ResetValues = ResetValues {
        ID: 0xE4A1FA05,
        HWGENERAL: 0x00000035,
        HWHOST: 0x10020001,
        HWDEVICE: 0x00000011,
        HWTXBUF: 0x80080B08,
        HWRXBUF: 0x00000808,
        GPTIMER0LD: 0x00000000,
        GPTIMER0CTRL: 0x00000000,
        GPTIMER1LD: 0x00000000,
        GPTIMER1CTRL: 0x00000000,
        SBUSCFG: 0x00000002,
        CAPLENGTH: 0x00000040,
        HCIVERSION: 0x00000100,
        HCSPARAMS: 0x00010011,
        HCCPARAMS: 0x00000006,
        DCIVERSION: 0x00000001,
        DCCPARAMS: 0x00000188,
        USBCMD: 0x00080000,
        USBSTS: 0x00000000,
        USBINTR: 0x00000000,
        FRINDEX: 0x00000000,
        DEVICEADDR: 0x00000000,
        ASYNCLISTADDR: 0x00000000,
        BURSTSIZE: 0x00000808,
        TXFILLTUNING: 0x00000000,
        ENDPTNAK: 0x00000000,
        ENDPTNAKEN: 0x00000000,
        CONFIGFLAG: 0x00000001,
        PORTSC1: 0x10000000,
        OTGSC: 0x00001120,
        USBMODE: 0x00005000,
        ENDPTSETUPSTAT: 0x00000000,
        ENDPTPRIME: 0x00000000,
        ENDPTFLUSH: 0x00000000,
        ENDPTSTAT: 0x00000000,
        ENDPTCOMPLETE: 0x00000000,
        ENDPTCTRL0: 0x00800080,
        ENDPTCTRL1: 0x00000000,
        ENDPTCTRL2: 0x00000000,
        ENDPTCTRL3: 0x00000000,
        ENDPTCTRL4: 0x00000000,
        ENDPTCTRL5: 0x00000000,
        ENDPTCTRL6: 0x00000000,
        ENDPTCTRL7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB1_TAKEN: bool = false;

    /// Safe access to USB1
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
            if USB1_TAKEN {
                None
            } else {
                USB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB1_TAKEN && inst.addr == INSTANCE.addr {
                USB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB1: *const RegisterBlock = 0x402e0000 as *const _;
