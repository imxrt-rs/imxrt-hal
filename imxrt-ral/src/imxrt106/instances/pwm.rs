#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::pwm::Instance;
pub use crate::imxrt106::peripherals::pwm::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::pwm::{
    DTSRCSEL, FCTRL0, FCTRL20, FFILT0, FSTS0, FTST0, MASK, MCTRL, MCTRL2, OUTEN, SM0CAPTCOMPA,
    SM0CAPTCOMPB, SM0CAPTCOMPX, SM0CAPTCTRLA, SM0CAPTCTRLB, SM0CAPTCTRLX, SM0CNT, SM0CTRL,
    SM0CTRL2, SM0CVAL0, SM0CVAL0CYC, SM0CVAL1, SM0CVAL1CYC, SM0CVAL2, SM0CVAL2CYC, SM0CVAL3,
    SM0CVAL3CYC, SM0CVAL4, SM0CVAL4CYC, SM0CVAL5, SM0CVAL5CYC, SM0DISMAP0, SM0DISMAP1, SM0DMAEN,
    SM0DTCNT0, SM0DTCNT1, SM0FRACVAL1, SM0FRACVAL2, SM0FRACVAL3, SM0FRACVAL4, SM0FRACVAL5,
    SM0FRCTRL, SM0INIT, SM0INTEN, SM0OCTRL, SM0STS, SM0TCTRL, SM0VAL0, SM0VAL1, SM0VAL2, SM0VAL3,
    SM0VAL4, SM0VAL5, SM1CAPTCOMPA, SM1CAPTCOMPB, SM1CAPTCOMPX, SM1CAPTCTRLA, SM1CAPTCTRLB,
    SM1CAPTCTRLX, SM1CNT, SM1CTRL, SM1CTRL2, SM1CVAL0, SM1CVAL0CYC, SM1CVAL1, SM1CVAL1CYC,
    SM1CVAL2, SM1CVAL2CYC, SM1CVAL3, SM1CVAL3CYC, SM1CVAL4, SM1CVAL4CYC, SM1CVAL5, SM1CVAL5CYC,
    SM1DISMAP0, SM1DISMAP1, SM1DMAEN, SM1DTCNT0, SM1DTCNT1, SM1FRACVAL1, SM1FRACVAL2, SM1FRACVAL3,
    SM1FRACVAL4, SM1FRACVAL5, SM1FRCTRL, SM1INIT, SM1INTEN, SM1OCTRL, SM1STS, SM1TCTRL, SM1VAL0,
    SM1VAL1, SM1VAL2, SM1VAL3, SM1VAL4, SM1VAL5, SM2CAPTCOMPA, SM2CAPTCOMPB, SM2CAPTCOMPX,
    SM2CAPTCTRLA, SM2CAPTCTRLB, SM2CAPTCTRLX, SM2CNT, SM2CTRL, SM2CTRL2, SM2CVAL0, SM2CVAL0CYC,
    SM2CVAL1, SM2CVAL1CYC, SM2CVAL2, SM2CVAL2CYC, SM2CVAL3, SM2CVAL3CYC, SM2CVAL4, SM2CVAL4CYC,
    SM2CVAL5, SM2CVAL5CYC, SM2DISMAP0, SM2DISMAP1, SM2DMAEN, SM2DTCNT0, SM2DTCNT1, SM2FRACVAL1,
    SM2FRACVAL2, SM2FRACVAL3, SM2FRACVAL4, SM2FRACVAL5, SM2FRCTRL, SM2INIT, SM2INTEN, SM2OCTRL,
    SM2STS, SM2TCTRL, SM2VAL0, SM2VAL1, SM2VAL2, SM2VAL3, SM2VAL4, SM2VAL5, SM3CAPTCOMPA,
    SM3CAPTCOMPB, SM3CAPTCOMPX, SM3CAPTCTRLA, SM3CAPTCTRLB, SM3CAPTCTRLX, SM3CNT, SM3CTRL,
    SM3CTRL2, SM3CVAL0, SM3CVAL0CYC, SM3CVAL1, SM3CVAL1CYC, SM3CVAL2, SM3CVAL2CYC, SM3CVAL3,
    SM3CVAL3CYC, SM3CVAL4, SM3CVAL4CYC, SM3CVAL5, SM3CVAL5CYC, SM3DISMAP0, SM3DISMAP1, SM3DMAEN,
    SM3DTCNT0, SM3DTCNT1, SM3FRACVAL1, SM3FRACVAL2, SM3FRACVAL3, SM3FRACVAL4, SM3FRACVAL5,
    SM3FRCTRL, SM3INIT, SM3INTEN, SM3OCTRL, SM3STS, SM3TCTRL, SM3VAL0, SM3VAL1, SM3VAL2, SM3VAL3,
    SM3VAL4, SM3VAL5, SWCOUT,
};

/// Access functions for the PWM1 peripheral instance
pub mod PWM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403dc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWM1
    pub const reset: ResetValues = ResetValues {
        SM0CNT: 0x00000000,
        SM0INIT: 0x00000000,
        SM0CTRL2: 0x00000000,
        SM0CTRL: 0x00000400,
        SM0VAL0: 0x00000000,
        SM0FRACVAL1: 0x00000000,
        SM0VAL1: 0x00000000,
        SM0FRACVAL2: 0x00000000,
        SM0VAL2: 0x00000000,
        SM0FRACVAL3: 0x00000000,
        SM0VAL3: 0x00000000,
        SM0FRACVAL4: 0x00000000,
        SM0VAL4: 0x00000000,
        SM0FRACVAL5: 0x00000000,
        SM0VAL5: 0x00000000,
        SM0FRCTRL: 0x00000000,
        SM0OCTRL: 0x00000000,
        SM0STS: 0x00000000,
        SM0INTEN: 0x00000000,
        SM0DMAEN: 0x00000000,
        SM0TCTRL: 0x00000000,
        SM0DISMAP0: 0x0000FFFF,
        SM0DISMAP1: 0x0000FFFF,
        SM0DTCNT0: 0x000007FF,
        SM0DTCNT1: 0x000007FF,
        SM0CAPTCTRLA: 0x00000000,
        SM0CAPTCOMPA: 0x00000000,
        SM0CAPTCTRLB: 0x00000000,
        SM0CAPTCOMPB: 0x00000000,
        SM0CAPTCTRLX: 0x00000000,
        SM0CAPTCOMPX: 0x00000000,
        SM0CVAL0: 0x00000000,
        SM0CVAL0CYC: 0x00000000,
        SM0CVAL1: 0x00000000,
        SM0CVAL1CYC: 0x00000000,
        SM0CVAL2: 0x00000000,
        SM0CVAL2CYC: 0x00000000,
        SM0CVAL3: 0x00000000,
        SM0CVAL3CYC: 0x00000000,
        SM0CVAL4: 0x00000000,
        SM0CVAL4CYC: 0x00000000,
        SM0CVAL5: 0x00000000,
        SM0CVAL5CYC: 0x00000000,
        SM1CNT: 0x00000000,
        SM1INIT: 0x00000000,
        SM1CTRL2: 0x00000000,
        SM1CTRL: 0x00000400,
        SM1VAL0: 0x00000000,
        SM1FRACVAL1: 0x00000000,
        SM1VAL1: 0x00000000,
        SM1FRACVAL2: 0x00000000,
        SM1VAL2: 0x00000000,
        SM1FRACVAL3: 0x00000000,
        SM1VAL3: 0x00000000,
        SM1FRACVAL4: 0x00000000,
        SM1VAL4: 0x00000000,
        SM1FRACVAL5: 0x00000000,
        SM1VAL5: 0x00000000,
        SM1FRCTRL: 0x00000000,
        SM1OCTRL: 0x00000000,
        SM1STS: 0x00000000,
        SM1INTEN: 0x00000000,
        SM1DMAEN: 0x00000000,
        SM1TCTRL: 0x00000000,
        SM1DISMAP0: 0x0000FFFF,
        SM1DISMAP1: 0x0000FFFF,
        SM1DTCNT0: 0x000007FF,
        SM1DTCNT1: 0x000007FF,
        SM1CAPTCTRLA: 0x00000000,
        SM1CAPTCOMPA: 0x00000000,
        SM1CAPTCTRLB: 0x00000000,
        SM1CAPTCOMPB: 0x00000000,
        SM1CAPTCTRLX: 0x00000000,
        SM1CAPTCOMPX: 0x00000000,
        SM1CVAL0: 0x00000000,
        SM1CVAL0CYC: 0x00000000,
        SM1CVAL1: 0x00000000,
        SM1CVAL1CYC: 0x00000000,
        SM1CVAL2: 0x00000000,
        SM1CVAL2CYC: 0x00000000,
        SM1CVAL3: 0x00000000,
        SM1CVAL3CYC: 0x00000000,
        SM1CVAL4: 0x00000000,
        SM1CVAL4CYC: 0x00000000,
        SM1CVAL5: 0x00000000,
        SM1CVAL5CYC: 0x00000000,
        SM2CNT: 0x00000000,
        SM2INIT: 0x00000000,
        SM2CTRL2: 0x00000000,
        SM2CTRL: 0x00000400,
        SM2VAL0: 0x00000000,
        SM2FRACVAL1: 0x00000000,
        SM2VAL1: 0x00000000,
        SM2FRACVAL2: 0x00000000,
        SM2VAL2: 0x00000000,
        SM2FRACVAL3: 0x00000000,
        SM2VAL3: 0x00000000,
        SM2FRACVAL4: 0x00000000,
        SM2VAL4: 0x00000000,
        SM2FRACVAL5: 0x00000000,
        SM2VAL5: 0x00000000,
        SM2FRCTRL: 0x00000000,
        SM2OCTRL: 0x00000000,
        SM2STS: 0x00000000,
        SM2INTEN: 0x00000000,
        SM2DMAEN: 0x00000000,
        SM2TCTRL: 0x00000000,
        SM2DISMAP0: 0x0000FFFF,
        SM2DISMAP1: 0x0000FFFF,
        SM2DTCNT0: 0x000007FF,
        SM2DTCNT1: 0x000007FF,
        SM2CAPTCTRLA: 0x00000000,
        SM2CAPTCOMPA: 0x00000000,
        SM2CAPTCTRLB: 0x00000000,
        SM2CAPTCOMPB: 0x00000000,
        SM2CAPTCTRLX: 0x00000000,
        SM2CAPTCOMPX: 0x00000000,
        SM2CVAL0: 0x00000000,
        SM2CVAL0CYC: 0x00000000,
        SM2CVAL1: 0x00000000,
        SM2CVAL1CYC: 0x00000000,
        SM2CVAL2: 0x00000000,
        SM2CVAL2CYC: 0x00000000,
        SM2CVAL3: 0x00000000,
        SM2CVAL3CYC: 0x00000000,
        SM2CVAL4: 0x00000000,
        SM2CVAL4CYC: 0x00000000,
        SM2CVAL5: 0x00000000,
        SM2CVAL5CYC: 0x00000000,
        SM3CNT: 0x00000000,
        SM3INIT: 0x00000000,
        SM3CTRL2: 0x00000000,
        SM3CTRL: 0x00000400,
        SM3VAL0: 0x00000000,
        SM3FRACVAL1: 0x00000000,
        SM3VAL1: 0x00000000,
        SM3FRACVAL2: 0x00000000,
        SM3VAL2: 0x00000000,
        SM3FRACVAL3: 0x00000000,
        SM3VAL3: 0x00000000,
        SM3FRACVAL4: 0x00000000,
        SM3VAL4: 0x00000000,
        SM3FRACVAL5: 0x00000000,
        SM3VAL5: 0x00000000,
        SM3FRCTRL: 0x00000000,
        SM3OCTRL: 0x00000000,
        SM3STS: 0x00000000,
        SM3INTEN: 0x00000000,
        SM3DMAEN: 0x00000000,
        SM3TCTRL: 0x00000000,
        SM3DISMAP0: 0x0000FFFF,
        SM3DISMAP1: 0x0000FFFF,
        SM3DTCNT0: 0x000007FF,
        SM3DTCNT1: 0x000007FF,
        SM3CAPTCTRLA: 0x00000000,
        SM3CAPTCOMPA: 0x00000000,
        SM3CAPTCTRLB: 0x00000000,
        SM3CAPTCOMPB: 0x00000000,
        SM3CAPTCTRLX: 0x00000000,
        SM3CAPTCOMPX: 0x00000000,
        SM3CVAL0: 0x00000000,
        SM3CVAL0CYC: 0x00000000,
        SM3CVAL1: 0x00000000,
        SM3CVAL1CYC: 0x00000000,
        SM3CVAL2: 0x00000000,
        SM3CVAL2CYC: 0x00000000,
        SM3CVAL3: 0x00000000,
        SM3CVAL3CYC: 0x00000000,
        SM3CVAL4: 0x00000000,
        SM3CVAL4CYC: 0x00000000,
        SM3CVAL5: 0x00000000,
        SM3CVAL5CYC: 0x00000000,
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWM1_TAKEN: bool = false;

    /// Safe access to PWM1
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
            if PWM1_TAKEN {
                None
            } else {
                PWM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWM1_TAKEN && inst.addr == INSTANCE.addr {
                PWM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM1: *const RegisterBlock = 0x403dc000 as *const _;

/// Access functions for the PWM2 peripheral instance
pub mod PWM2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403e0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWM2
    pub const reset: ResetValues = ResetValues {
        SM0CNT: 0x00000000,
        SM0INIT: 0x00000000,
        SM0CTRL2: 0x00000000,
        SM0CTRL: 0x00000400,
        SM0VAL0: 0x00000000,
        SM0FRACVAL1: 0x00000000,
        SM0VAL1: 0x00000000,
        SM0FRACVAL2: 0x00000000,
        SM0VAL2: 0x00000000,
        SM0FRACVAL3: 0x00000000,
        SM0VAL3: 0x00000000,
        SM0FRACVAL4: 0x00000000,
        SM0VAL4: 0x00000000,
        SM0FRACVAL5: 0x00000000,
        SM0VAL5: 0x00000000,
        SM0FRCTRL: 0x00000000,
        SM0OCTRL: 0x00000000,
        SM0STS: 0x00000000,
        SM0INTEN: 0x00000000,
        SM0DMAEN: 0x00000000,
        SM0TCTRL: 0x00000000,
        SM0DISMAP0: 0x0000FFFF,
        SM0DISMAP1: 0x0000FFFF,
        SM0DTCNT0: 0x000007FF,
        SM0DTCNT1: 0x000007FF,
        SM0CAPTCTRLA: 0x00000000,
        SM0CAPTCOMPA: 0x00000000,
        SM0CAPTCTRLB: 0x00000000,
        SM0CAPTCOMPB: 0x00000000,
        SM0CAPTCTRLX: 0x00000000,
        SM0CAPTCOMPX: 0x00000000,
        SM0CVAL0: 0x00000000,
        SM0CVAL0CYC: 0x00000000,
        SM0CVAL1: 0x00000000,
        SM0CVAL1CYC: 0x00000000,
        SM0CVAL2: 0x00000000,
        SM0CVAL2CYC: 0x00000000,
        SM0CVAL3: 0x00000000,
        SM0CVAL3CYC: 0x00000000,
        SM0CVAL4: 0x00000000,
        SM0CVAL4CYC: 0x00000000,
        SM0CVAL5: 0x00000000,
        SM0CVAL5CYC: 0x00000000,
        SM1CNT: 0x00000000,
        SM1INIT: 0x00000000,
        SM1CTRL2: 0x00000000,
        SM1CTRL: 0x00000400,
        SM1VAL0: 0x00000000,
        SM1FRACVAL1: 0x00000000,
        SM1VAL1: 0x00000000,
        SM1FRACVAL2: 0x00000000,
        SM1VAL2: 0x00000000,
        SM1FRACVAL3: 0x00000000,
        SM1VAL3: 0x00000000,
        SM1FRACVAL4: 0x00000000,
        SM1VAL4: 0x00000000,
        SM1FRACVAL5: 0x00000000,
        SM1VAL5: 0x00000000,
        SM1FRCTRL: 0x00000000,
        SM1OCTRL: 0x00000000,
        SM1STS: 0x00000000,
        SM1INTEN: 0x00000000,
        SM1DMAEN: 0x00000000,
        SM1TCTRL: 0x00000000,
        SM1DISMAP0: 0x0000FFFF,
        SM1DISMAP1: 0x0000FFFF,
        SM1DTCNT0: 0x000007FF,
        SM1DTCNT1: 0x000007FF,
        SM1CAPTCTRLA: 0x00000000,
        SM1CAPTCOMPA: 0x00000000,
        SM1CAPTCTRLB: 0x00000000,
        SM1CAPTCOMPB: 0x00000000,
        SM1CAPTCTRLX: 0x00000000,
        SM1CAPTCOMPX: 0x00000000,
        SM1CVAL0: 0x00000000,
        SM1CVAL0CYC: 0x00000000,
        SM1CVAL1: 0x00000000,
        SM1CVAL1CYC: 0x00000000,
        SM1CVAL2: 0x00000000,
        SM1CVAL2CYC: 0x00000000,
        SM1CVAL3: 0x00000000,
        SM1CVAL3CYC: 0x00000000,
        SM1CVAL4: 0x00000000,
        SM1CVAL4CYC: 0x00000000,
        SM1CVAL5: 0x00000000,
        SM1CVAL5CYC: 0x00000000,
        SM2CNT: 0x00000000,
        SM2INIT: 0x00000000,
        SM2CTRL2: 0x00000000,
        SM2CTRL: 0x00000400,
        SM2VAL0: 0x00000000,
        SM2FRACVAL1: 0x00000000,
        SM2VAL1: 0x00000000,
        SM2FRACVAL2: 0x00000000,
        SM2VAL2: 0x00000000,
        SM2FRACVAL3: 0x00000000,
        SM2VAL3: 0x00000000,
        SM2FRACVAL4: 0x00000000,
        SM2VAL4: 0x00000000,
        SM2FRACVAL5: 0x00000000,
        SM2VAL5: 0x00000000,
        SM2FRCTRL: 0x00000000,
        SM2OCTRL: 0x00000000,
        SM2STS: 0x00000000,
        SM2INTEN: 0x00000000,
        SM2DMAEN: 0x00000000,
        SM2TCTRL: 0x00000000,
        SM2DISMAP0: 0x0000FFFF,
        SM2DISMAP1: 0x0000FFFF,
        SM2DTCNT0: 0x000007FF,
        SM2DTCNT1: 0x000007FF,
        SM2CAPTCTRLA: 0x00000000,
        SM2CAPTCOMPA: 0x00000000,
        SM2CAPTCTRLB: 0x00000000,
        SM2CAPTCOMPB: 0x00000000,
        SM2CAPTCTRLX: 0x00000000,
        SM2CAPTCOMPX: 0x00000000,
        SM2CVAL0: 0x00000000,
        SM2CVAL0CYC: 0x00000000,
        SM2CVAL1: 0x00000000,
        SM2CVAL1CYC: 0x00000000,
        SM2CVAL2: 0x00000000,
        SM2CVAL2CYC: 0x00000000,
        SM2CVAL3: 0x00000000,
        SM2CVAL3CYC: 0x00000000,
        SM2CVAL4: 0x00000000,
        SM2CVAL4CYC: 0x00000000,
        SM2CVAL5: 0x00000000,
        SM2CVAL5CYC: 0x00000000,
        SM3CNT: 0x00000000,
        SM3INIT: 0x00000000,
        SM3CTRL2: 0x00000000,
        SM3CTRL: 0x00000400,
        SM3VAL0: 0x00000000,
        SM3FRACVAL1: 0x00000000,
        SM3VAL1: 0x00000000,
        SM3FRACVAL2: 0x00000000,
        SM3VAL2: 0x00000000,
        SM3FRACVAL3: 0x00000000,
        SM3VAL3: 0x00000000,
        SM3FRACVAL4: 0x00000000,
        SM3VAL4: 0x00000000,
        SM3FRACVAL5: 0x00000000,
        SM3VAL5: 0x00000000,
        SM3FRCTRL: 0x00000000,
        SM3OCTRL: 0x00000000,
        SM3STS: 0x00000000,
        SM3INTEN: 0x00000000,
        SM3DMAEN: 0x00000000,
        SM3TCTRL: 0x00000000,
        SM3DISMAP0: 0x0000FFFF,
        SM3DISMAP1: 0x0000FFFF,
        SM3DTCNT0: 0x000007FF,
        SM3DTCNT1: 0x000007FF,
        SM3CAPTCTRLA: 0x00000000,
        SM3CAPTCOMPA: 0x00000000,
        SM3CAPTCTRLB: 0x00000000,
        SM3CAPTCOMPB: 0x00000000,
        SM3CAPTCTRLX: 0x00000000,
        SM3CAPTCOMPX: 0x00000000,
        SM3CVAL0: 0x00000000,
        SM3CVAL0CYC: 0x00000000,
        SM3CVAL1: 0x00000000,
        SM3CVAL1CYC: 0x00000000,
        SM3CVAL2: 0x00000000,
        SM3CVAL2CYC: 0x00000000,
        SM3CVAL3: 0x00000000,
        SM3CVAL3CYC: 0x00000000,
        SM3CVAL4: 0x00000000,
        SM3CVAL4CYC: 0x00000000,
        SM3CVAL5: 0x00000000,
        SM3CVAL5CYC: 0x00000000,
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWM2_TAKEN: bool = false;

    /// Safe access to PWM2
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
            if PWM2_TAKEN {
                None
            } else {
                PWM2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWM2_TAKEN && inst.addr == INSTANCE.addr {
                PWM2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWM2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM2: *const RegisterBlock = 0x403e0000 as *const _;

/// Access functions for the PWM3 peripheral instance
pub mod PWM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403e4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWM3
    pub const reset: ResetValues = ResetValues {
        SM0CNT: 0x00000000,
        SM0INIT: 0x00000000,
        SM0CTRL2: 0x00000000,
        SM0CTRL: 0x00000400,
        SM0VAL0: 0x00000000,
        SM0FRACVAL1: 0x00000000,
        SM0VAL1: 0x00000000,
        SM0FRACVAL2: 0x00000000,
        SM0VAL2: 0x00000000,
        SM0FRACVAL3: 0x00000000,
        SM0VAL3: 0x00000000,
        SM0FRACVAL4: 0x00000000,
        SM0VAL4: 0x00000000,
        SM0FRACVAL5: 0x00000000,
        SM0VAL5: 0x00000000,
        SM0FRCTRL: 0x00000000,
        SM0OCTRL: 0x00000000,
        SM0STS: 0x00000000,
        SM0INTEN: 0x00000000,
        SM0DMAEN: 0x00000000,
        SM0TCTRL: 0x00000000,
        SM0DISMAP0: 0x0000FFFF,
        SM0DISMAP1: 0x0000FFFF,
        SM0DTCNT0: 0x000007FF,
        SM0DTCNT1: 0x000007FF,
        SM0CAPTCTRLA: 0x00000000,
        SM0CAPTCOMPA: 0x00000000,
        SM0CAPTCTRLB: 0x00000000,
        SM0CAPTCOMPB: 0x00000000,
        SM0CAPTCTRLX: 0x00000000,
        SM0CAPTCOMPX: 0x00000000,
        SM0CVAL0: 0x00000000,
        SM0CVAL0CYC: 0x00000000,
        SM0CVAL1: 0x00000000,
        SM0CVAL1CYC: 0x00000000,
        SM0CVAL2: 0x00000000,
        SM0CVAL2CYC: 0x00000000,
        SM0CVAL3: 0x00000000,
        SM0CVAL3CYC: 0x00000000,
        SM0CVAL4: 0x00000000,
        SM0CVAL4CYC: 0x00000000,
        SM0CVAL5: 0x00000000,
        SM0CVAL5CYC: 0x00000000,
        SM1CNT: 0x00000000,
        SM1INIT: 0x00000000,
        SM1CTRL2: 0x00000000,
        SM1CTRL: 0x00000400,
        SM1VAL0: 0x00000000,
        SM1FRACVAL1: 0x00000000,
        SM1VAL1: 0x00000000,
        SM1FRACVAL2: 0x00000000,
        SM1VAL2: 0x00000000,
        SM1FRACVAL3: 0x00000000,
        SM1VAL3: 0x00000000,
        SM1FRACVAL4: 0x00000000,
        SM1VAL4: 0x00000000,
        SM1FRACVAL5: 0x00000000,
        SM1VAL5: 0x00000000,
        SM1FRCTRL: 0x00000000,
        SM1OCTRL: 0x00000000,
        SM1STS: 0x00000000,
        SM1INTEN: 0x00000000,
        SM1DMAEN: 0x00000000,
        SM1TCTRL: 0x00000000,
        SM1DISMAP0: 0x0000FFFF,
        SM1DISMAP1: 0x0000FFFF,
        SM1DTCNT0: 0x000007FF,
        SM1DTCNT1: 0x000007FF,
        SM1CAPTCTRLA: 0x00000000,
        SM1CAPTCOMPA: 0x00000000,
        SM1CAPTCTRLB: 0x00000000,
        SM1CAPTCOMPB: 0x00000000,
        SM1CAPTCTRLX: 0x00000000,
        SM1CAPTCOMPX: 0x00000000,
        SM1CVAL0: 0x00000000,
        SM1CVAL0CYC: 0x00000000,
        SM1CVAL1: 0x00000000,
        SM1CVAL1CYC: 0x00000000,
        SM1CVAL2: 0x00000000,
        SM1CVAL2CYC: 0x00000000,
        SM1CVAL3: 0x00000000,
        SM1CVAL3CYC: 0x00000000,
        SM1CVAL4: 0x00000000,
        SM1CVAL4CYC: 0x00000000,
        SM1CVAL5: 0x00000000,
        SM1CVAL5CYC: 0x00000000,
        SM2CNT: 0x00000000,
        SM2INIT: 0x00000000,
        SM2CTRL2: 0x00000000,
        SM2CTRL: 0x00000400,
        SM2VAL0: 0x00000000,
        SM2FRACVAL1: 0x00000000,
        SM2VAL1: 0x00000000,
        SM2FRACVAL2: 0x00000000,
        SM2VAL2: 0x00000000,
        SM2FRACVAL3: 0x00000000,
        SM2VAL3: 0x00000000,
        SM2FRACVAL4: 0x00000000,
        SM2VAL4: 0x00000000,
        SM2FRACVAL5: 0x00000000,
        SM2VAL5: 0x00000000,
        SM2FRCTRL: 0x00000000,
        SM2OCTRL: 0x00000000,
        SM2STS: 0x00000000,
        SM2INTEN: 0x00000000,
        SM2DMAEN: 0x00000000,
        SM2TCTRL: 0x00000000,
        SM2DISMAP0: 0x0000FFFF,
        SM2DISMAP1: 0x0000FFFF,
        SM2DTCNT0: 0x000007FF,
        SM2DTCNT1: 0x000007FF,
        SM2CAPTCTRLA: 0x00000000,
        SM2CAPTCOMPA: 0x00000000,
        SM2CAPTCTRLB: 0x00000000,
        SM2CAPTCOMPB: 0x00000000,
        SM2CAPTCTRLX: 0x00000000,
        SM2CAPTCOMPX: 0x00000000,
        SM2CVAL0: 0x00000000,
        SM2CVAL0CYC: 0x00000000,
        SM2CVAL1: 0x00000000,
        SM2CVAL1CYC: 0x00000000,
        SM2CVAL2: 0x00000000,
        SM2CVAL2CYC: 0x00000000,
        SM2CVAL3: 0x00000000,
        SM2CVAL3CYC: 0x00000000,
        SM2CVAL4: 0x00000000,
        SM2CVAL4CYC: 0x00000000,
        SM2CVAL5: 0x00000000,
        SM2CVAL5CYC: 0x00000000,
        SM3CNT: 0x00000000,
        SM3INIT: 0x00000000,
        SM3CTRL2: 0x00000000,
        SM3CTRL: 0x00000400,
        SM3VAL0: 0x00000000,
        SM3FRACVAL1: 0x00000000,
        SM3VAL1: 0x00000000,
        SM3FRACVAL2: 0x00000000,
        SM3VAL2: 0x00000000,
        SM3FRACVAL3: 0x00000000,
        SM3VAL3: 0x00000000,
        SM3FRACVAL4: 0x00000000,
        SM3VAL4: 0x00000000,
        SM3FRACVAL5: 0x00000000,
        SM3VAL5: 0x00000000,
        SM3FRCTRL: 0x00000000,
        SM3OCTRL: 0x00000000,
        SM3STS: 0x00000000,
        SM3INTEN: 0x00000000,
        SM3DMAEN: 0x00000000,
        SM3TCTRL: 0x00000000,
        SM3DISMAP0: 0x0000FFFF,
        SM3DISMAP1: 0x0000FFFF,
        SM3DTCNT0: 0x000007FF,
        SM3DTCNT1: 0x000007FF,
        SM3CAPTCTRLA: 0x00000000,
        SM3CAPTCOMPA: 0x00000000,
        SM3CAPTCTRLB: 0x00000000,
        SM3CAPTCOMPB: 0x00000000,
        SM3CAPTCTRLX: 0x00000000,
        SM3CAPTCOMPX: 0x00000000,
        SM3CVAL0: 0x00000000,
        SM3CVAL0CYC: 0x00000000,
        SM3CVAL1: 0x00000000,
        SM3CVAL1CYC: 0x00000000,
        SM3CVAL2: 0x00000000,
        SM3CVAL2CYC: 0x00000000,
        SM3CVAL3: 0x00000000,
        SM3CVAL3CYC: 0x00000000,
        SM3CVAL4: 0x00000000,
        SM3CVAL4CYC: 0x00000000,
        SM3CVAL5: 0x00000000,
        SM3CVAL5CYC: 0x00000000,
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWM3_TAKEN: bool = false;

    /// Safe access to PWM3
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
            if PWM3_TAKEN {
                None
            } else {
                PWM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWM3_TAKEN && inst.addr == INSTANCE.addr {
                PWM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM3: *const RegisterBlock = 0x403e4000 as *const _;

/// Access functions for the PWM4 peripheral instance
pub mod PWM4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403e8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWM4
    pub const reset: ResetValues = ResetValues {
        SM0CNT: 0x00000000,
        SM0INIT: 0x00000000,
        SM0CTRL2: 0x00000000,
        SM0CTRL: 0x00000400,
        SM0VAL0: 0x00000000,
        SM0FRACVAL1: 0x00000000,
        SM0VAL1: 0x00000000,
        SM0FRACVAL2: 0x00000000,
        SM0VAL2: 0x00000000,
        SM0FRACVAL3: 0x00000000,
        SM0VAL3: 0x00000000,
        SM0FRACVAL4: 0x00000000,
        SM0VAL4: 0x00000000,
        SM0FRACVAL5: 0x00000000,
        SM0VAL5: 0x00000000,
        SM0FRCTRL: 0x00000000,
        SM0OCTRL: 0x00000000,
        SM0STS: 0x00000000,
        SM0INTEN: 0x00000000,
        SM0DMAEN: 0x00000000,
        SM0TCTRL: 0x00000000,
        SM0DISMAP0: 0x0000FFFF,
        SM0DISMAP1: 0x0000FFFF,
        SM0DTCNT0: 0x000007FF,
        SM0DTCNT1: 0x000007FF,
        SM0CAPTCTRLA: 0x00000000,
        SM0CAPTCOMPA: 0x00000000,
        SM0CAPTCTRLB: 0x00000000,
        SM0CAPTCOMPB: 0x00000000,
        SM0CAPTCTRLX: 0x00000000,
        SM0CAPTCOMPX: 0x00000000,
        SM0CVAL0: 0x00000000,
        SM0CVAL0CYC: 0x00000000,
        SM0CVAL1: 0x00000000,
        SM0CVAL1CYC: 0x00000000,
        SM0CVAL2: 0x00000000,
        SM0CVAL2CYC: 0x00000000,
        SM0CVAL3: 0x00000000,
        SM0CVAL3CYC: 0x00000000,
        SM0CVAL4: 0x00000000,
        SM0CVAL4CYC: 0x00000000,
        SM0CVAL5: 0x00000000,
        SM0CVAL5CYC: 0x00000000,
        SM1CNT: 0x00000000,
        SM1INIT: 0x00000000,
        SM1CTRL2: 0x00000000,
        SM1CTRL: 0x00000400,
        SM1VAL0: 0x00000000,
        SM1FRACVAL1: 0x00000000,
        SM1VAL1: 0x00000000,
        SM1FRACVAL2: 0x00000000,
        SM1VAL2: 0x00000000,
        SM1FRACVAL3: 0x00000000,
        SM1VAL3: 0x00000000,
        SM1FRACVAL4: 0x00000000,
        SM1VAL4: 0x00000000,
        SM1FRACVAL5: 0x00000000,
        SM1VAL5: 0x00000000,
        SM1FRCTRL: 0x00000000,
        SM1OCTRL: 0x00000000,
        SM1STS: 0x00000000,
        SM1INTEN: 0x00000000,
        SM1DMAEN: 0x00000000,
        SM1TCTRL: 0x00000000,
        SM1DISMAP0: 0x0000FFFF,
        SM1DISMAP1: 0x0000FFFF,
        SM1DTCNT0: 0x000007FF,
        SM1DTCNT1: 0x000007FF,
        SM1CAPTCTRLA: 0x00000000,
        SM1CAPTCOMPA: 0x00000000,
        SM1CAPTCTRLB: 0x00000000,
        SM1CAPTCOMPB: 0x00000000,
        SM1CAPTCTRLX: 0x00000000,
        SM1CAPTCOMPX: 0x00000000,
        SM1CVAL0: 0x00000000,
        SM1CVAL0CYC: 0x00000000,
        SM1CVAL1: 0x00000000,
        SM1CVAL1CYC: 0x00000000,
        SM1CVAL2: 0x00000000,
        SM1CVAL2CYC: 0x00000000,
        SM1CVAL3: 0x00000000,
        SM1CVAL3CYC: 0x00000000,
        SM1CVAL4: 0x00000000,
        SM1CVAL4CYC: 0x00000000,
        SM1CVAL5: 0x00000000,
        SM1CVAL5CYC: 0x00000000,
        SM2CNT: 0x00000000,
        SM2INIT: 0x00000000,
        SM2CTRL2: 0x00000000,
        SM2CTRL: 0x00000400,
        SM2VAL0: 0x00000000,
        SM2FRACVAL1: 0x00000000,
        SM2VAL1: 0x00000000,
        SM2FRACVAL2: 0x00000000,
        SM2VAL2: 0x00000000,
        SM2FRACVAL3: 0x00000000,
        SM2VAL3: 0x00000000,
        SM2FRACVAL4: 0x00000000,
        SM2VAL4: 0x00000000,
        SM2FRACVAL5: 0x00000000,
        SM2VAL5: 0x00000000,
        SM2FRCTRL: 0x00000000,
        SM2OCTRL: 0x00000000,
        SM2STS: 0x00000000,
        SM2INTEN: 0x00000000,
        SM2DMAEN: 0x00000000,
        SM2TCTRL: 0x00000000,
        SM2DISMAP0: 0x0000FFFF,
        SM2DISMAP1: 0x0000FFFF,
        SM2DTCNT0: 0x000007FF,
        SM2DTCNT1: 0x000007FF,
        SM2CAPTCTRLA: 0x00000000,
        SM2CAPTCOMPA: 0x00000000,
        SM2CAPTCTRLB: 0x00000000,
        SM2CAPTCOMPB: 0x00000000,
        SM2CAPTCTRLX: 0x00000000,
        SM2CAPTCOMPX: 0x00000000,
        SM2CVAL0: 0x00000000,
        SM2CVAL0CYC: 0x00000000,
        SM2CVAL1: 0x00000000,
        SM2CVAL1CYC: 0x00000000,
        SM2CVAL2: 0x00000000,
        SM2CVAL2CYC: 0x00000000,
        SM2CVAL3: 0x00000000,
        SM2CVAL3CYC: 0x00000000,
        SM2CVAL4: 0x00000000,
        SM2CVAL4CYC: 0x00000000,
        SM2CVAL5: 0x00000000,
        SM2CVAL5CYC: 0x00000000,
        SM3CNT: 0x00000000,
        SM3INIT: 0x00000000,
        SM3CTRL2: 0x00000000,
        SM3CTRL: 0x00000400,
        SM3VAL0: 0x00000000,
        SM3FRACVAL1: 0x00000000,
        SM3VAL1: 0x00000000,
        SM3FRACVAL2: 0x00000000,
        SM3VAL2: 0x00000000,
        SM3FRACVAL3: 0x00000000,
        SM3VAL3: 0x00000000,
        SM3FRACVAL4: 0x00000000,
        SM3VAL4: 0x00000000,
        SM3FRACVAL5: 0x00000000,
        SM3VAL5: 0x00000000,
        SM3FRCTRL: 0x00000000,
        SM3OCTRL: 0x00000000,
        SM3STS: 0x00000000,
        SM3INTEN: 0x00000000,
        SM3DMAEN: 0x00000000,
        SM3TCTRL: 0x00000000,
        SM3DISMAP0: 0x0000FFFF,
        SM3DISMAP1: 0x0000FFFF,
        SM3DTCNT0: 0x000007FF,
        SM3DTCNT1: 0x000007FF,
        SM3CAPTCTRLA: 0x00000000,
        SM3CAPTCOMPA: 0x00000000,
        SM3CAPTCTRLB: 0x00000000,
        SM3CAPTCOMPB: 0x00000000,
        SM3CAPTCTRLX: 0x00000000,
        SM3CAPTCOMPX: 0x00000000,
        SM3CVAL0: 0x00000000,
        SM3CVAL0CYC: 0x00000000,
        SM3CVAL1: 0x00000000,
        SM3CVAL1CYC: 0x00000000,
        SM3CVAL2: 0x00000000,
        SM3CVAL2CYC: 0x00000000,
        SM3CVAL3: 0x00000000,
        SM3CVAL3CYC: 0x00000000,
        SM3CVAL4: 0x00000000,
        SM3CVAL4CYC: 0x00000000,
        SM3CVAL5: 0x00000000,
        SM3CVAL5CYC: 0x00000000,
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWM4_TAKEN: bool = false;

    /// Safe access to PWM4
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
            if PWM4_TAKEN {
                None
            } else {
                PWM4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWM4_TAKEN && inst.addr == INSTANCE.addr {
                PWM4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWM4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM4: *const RegisterBlock = 0x403e8000 as *const _;
