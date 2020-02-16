#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::pwm::Instance;
pub use crate::imxrt105::peripherals::pwm::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::pwm::{
    DTSRCSEL, FCTRL0, FCTRL20, FFILT0, FSTS0, FTST0, MASK, MCTRL, MCTRL2, OUTEN, SMCAPTCOMPA0,
    SMCAPTCOMPA1, SMCAPTCOMPA2, SMCAPTCOMPA3, SMCAPTCOMPB0, SMCAPTCOMPB1, SMCAPTCOMPB2,
    SMCAPTCOMPB3, SMCAPTCOMPX0, SMCAPTCOMPX1, SMCAPTCOMPX2, SMCAPTCOMPX3, SMCAPTCTRLA0,
    SMCAPTCTRLA1, SMCAPTCTRLA2, SMCAPTCTRLA3, SMCAPTCTRLB0, SMCAPTCTRLB1, SMCAPTCTRLB2,
    SMCAPTCTRLB3, SMCAPTCTRLX0, SMCAPTCTRLX1, SMCAPTCTRLX2, SMCAPTCTRLX3, SMCNT0, SMCNT1, SMCNT2,
    SMCNT3, SMCTRL0, SMCTRL1, SMCTRL2, SMCTRL20, SMCTRL21, SMCTRL22, SMCTRL23, SMCTRL3, SMCVAL00,
    SMCVAL01, SMCVAL02, SMCVAL03, SMCVAL0CYC0, SMCVAL0CYC1, SMCVAL0CYC2, SMCVAL0CYC3, SMCVAL10,
    SMCVAL11, SMCVAL12, SMCVAL13, SMCVAL1CYC0, SMCVAL1CYC1, SMCVAL1CYC2, SMCVAL1CYC3, SMCVAL20,
    SMCVAL21, SMCVAL22, SMCVAL23, SMCVAL2CYC0, SMCVAL2CYC1, SMCVAL2CYC2, SMCVAL2CYC3, SMCVAL30,
    SMCVAL31, SMCVAL32, SMCVAL33, SMCVAL3CYC0, SMCVAL3CYC1, SMCVAL3CYC2, SMCVAL3CYC3, SMCVAL40,
    SMCVAL41, SMCVAL42, SMCVAL43, SMCVAL4CYC0, SMCVAL4CYC1, SMCVAL4CYC2, SMCVAL4CYC3, SMCVAL50,
    SMCVAL51, SMCVAL52, SMCVAL53, SMCVAL5CYC0, SMCVAL5CYC1, SMCVAL5CYC2, SMCVAL5CYC3, SMDISMAP00,
    SMDISMAP01, SMDISMAP02, SMDISMAP03, SMDISMAP10, SMDISMAP11, SMDISMAP12, SMDISMAP13, SMDMAEN0,
    SMDMAEN1, SMDMAEN2, SMDMAEN3, SMDTCNT00, SMDTCNT01, SMDTCNT02, SMDTCNT03, SMDTCNT10, SMDTCNT11,
    SMDTCNT12, SMDTCNT13, SMFRACVAL10, SMFRACVAL11, SMFRACVAL12, SMFRACVAL13, SMFRACVAL20,
    SMFRACVAL21, SMFRACVAL22, SMFRACVAL23, SMFRACVAL30, SMFRACVAL31, SMFRACVAL32, SMFRACVAL33,
    SMFRACVAL40, SMFRACVAL41, SMFRACVAL42, SMFRACVAL43, SMFRACVAL50, SMFRACVAL51, SMFRACVAL52,
    SMFRACVAL53, SMFRCTRL0, SMFRCTRL1, SMFRCTRL2, SMFRCTRL3, SMINIT0, SMINIT1, SMINIT2, SMINIT3,
    SMINTEN0, SMINTEN1, SMINTEN2, SMINTEN3, SMOCTRL0, SMOCTRL1, SMOCTRL2, SMOCTRL3, SMSTS0, SMSTS1,
    SMSTS2, SMSTS3, SMTCTRL0, SMTCTRL1, SMTCTRL2, SMTCTRL3, SMVAL00, SMVAL01, SMVAL02, SMVAL03,
    SMVAL10, SMVAL11, SMVAL12, SMVAL13, SMVAL20, SMVAL21, SMVAL22, SMVAL23, SMVAL30, SMVAL31,
    SMVAL32, SMVAL33, SMVAL40, SMVAL41, SMVAL42, SMVAL43, SMVAL50, SMVAL51, SMVAL52, SMVAL53,
    SWCOUT,
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
        SMCNT0: 0x00000000,
        SMINIT0: 0x00000000,
        SMCTRL20: 0x00000000,
        SMCTRL0: 0x00000400,
        SMVAL00: 0x00000000,
        SMFRACVAL10: 0x00000000,
        SMVAL10: 0x00000000,
        SMFRACVAL20: 0x00000000,
        SMVAL20: 0x00000000,
        SMFRACVAL30: 0x00000000,
        SMVAL30: 0x00000000,
        SMFRACVAL40: 0x00000000,
        SMVAL40: 0x00000000,
        SMFRACVAL50: 0x00000000,
        SMVAL50: 0x00000000,
        SMFRCTRL0: 0x00000000,
        SMOCTRL0: 0x00000000,
        SMSTS0: 0x00000000,
        SMINTEN0: 0x00000000,
        SMDMAEN0: 0x00000000,
        SMTCTRL0: 0x00000000,
        SMDISMAP00: 0x0000FFFF,
        SMDISMAP10: 0x0000FFFF,
        SMDTCNT00: 0x000007FF,
        SMDTCNT10: 0x000007FF,
        SMCAPTCTRLA0: 0x00000000,
        SMCAPTCOMPA0: 0x00000000,
        SMCAPTCTRLB0: 0x00000000,
        SMCAPTCOMPB0: 0x00000000,
        SMCAPTCTRLX0: 0x00000000,
        SMCAPTCOMPX0: 0x00000000,
        SMCVAL00: 0x00000000,
        SMCVAL0CYC0: 0x00000000,
        SMCVAL10: 0x00000000,
        SMCVAL1CYC0: 0x00000000,
        SMCVAL20: 0x00000000,
        SMCVAL2CYC0: 0x00000000,
        SMCVAL30: 0x00000000,
        SMCVAL3CYC0: 0x00000000,
        SMCVAL40: 0x00000000,
        SMCVAL4CYC0: 0x00000000,
        SMCVAL50: 0x00000000,
        SMCVAL5CYC0: 0x00000000,
        SMCNT1: 0x00000000,
        SMINIT1: 0x00000000,
        SMCTRL21: 0x00000000,
        SMCTRL1: 0x00000400,
        SMVAL01: 0x00000000,
        SMFRACVAL11: 0x00000000,
        SMVAL11: 0x00000000,
        SMFRACVAL21: 0x00000000,
        SMVAL21: 0x00000000,
        SMFRACVAL31: 0x00000000,
        SMVAL31: 0x00000000,
        SMFRACVAL41: 0x00000000,
        SMVAL41: 0x00000000,
        SMFRACVAL51: 0x00000000,
        SMVAL51: 0x00000000,
        SMFRCTRL1: 0x00000000,
        SMOCTRL1: 0x00000000,
        SMSTS1: 0x00000000,
        SMINTEN1: 0x00000000,
        SMDMAEN1: 0x00000000,
        SMTCTRL1: 0x00000000,
        SMDISMAP01: 0x0000FFFF,
        SMDISMAP11: 0x0000FFFF,
        SMDTCNT01: 0x000007FF,
        SMDTCNT11: 0x000007FF,
        SMCAPTCTRLA1: 0x00000000,
        SMCAPTCOMPA1: 0x00000000,
        SMCAPTCTRLB1: 0x00000000,
        SMCAPTCOMPB1: 0x00000000,
        SMCAPTCTRLX1: 0x00000000,
        SMCAPTCOMPX1: 0x00000000,
        SMCVAL01: 0x00000000,
        SMCVAL0CYC1: 0x00000000,
        SMCVAL11: 0x00000000,
        SMCVAL1CYC1: 0x00000000,
        SMCVAL21: 0x00000000,
        SMCVAL2CYC1: 0x00000000,
        SMCVAL31: 0x00000000,
        SMCVAL3CYC1: 0x00000000,
        SMCVAL41: 0x00000000,
        SMCVAL4CYC1: 0x00000000,
        SMCVAL51: 0x00000000,
        SMCVAL5CYC1: 0x00000000,
        SMCNT2: 0x00000000,
        SMINIT2: 0x00000000,
        SMCTRL22: 0x00000000,
        SMCTRL2: 0x00000400,
        SMVAL02: 0x00000000,
        SMFRACVAL12: 0x00000000,
        SMVAL12: 0x00000000,
        SMFRACVAL22: 0x00000000,
        SMVAL22: 0x00000000,
        SMFRACVAL32: 0x00000000,
        SMVAL32: 0x00000000,
        SMFRACVAL42: 0x00000000,
        SMVAL42: 0x00000000,
        SMFRACVAL52: 0x00000000,
        SMVAL52: 0x00000000,
        SMFRCTRL2: 0x00000000,
        SMOCTRL2: 0x00000000,
        SMSTS2: 0x00000000,
        SMINTEN2: 0x00000000,
        SMDMAEN2: 0x00000000,
        SMTCTRL2: 0x00000000,
        SMDISMAP02: 0x0000FFFF,
        SMDISMAP12: 0x0000FFFF,
        SMDTCNT02: 0x000007FF,
        SMDTCNT12: 0x000007FF,
        SMCAPTCTRLA2: 0x00000000,
        SMCAPTCOMPA2: 0x00000000,
        SMCAPTCTRLB2: 0x00000000,
        SMCAPTCOMPB2: 0x00000000,
        SMCAPTCTRLX2: 0x00000000,
        SMCAPTCOMPX2: 0x00000000,
        SMCVAL02: 0x00000000,
        SMCVAL0CYC2: 0x00000000,
        SMCVAL12: 0x00000000,
        SMCVAL1CYC2: 0x00000000,
        SMCVAL22: 0x00000000,
        SMCVAL2CYC2: 0x00000000,
        SMCVAL32: 0x00000000,
        SMCVAL3CYC2: 0x00000000,
        SMCVAL42: 0x00000000,
        SMCVAL4CYC2: 0x00000000,
        SMCVAL52: 0x00000000,
        SMCVAL5CYC2: 0x00000000,
        SMCNT3: 0x00000000,
        SMINIT3: 0x00000000,
        SMCTRL23: 0x00000000,
        SMCTRL3: 0x00000400,
        SMVAL03: 0x00000000,
        SMFRACVAL13: 0x00000000,
        SMVAL13: 0x00000000,
        SMFRACVAL23: 0x00000000,
        SMVAL23: 0x00000000,
        SMFRACVAL33: 0x00000000,
        SMVAL33: 0x00000000,
        SMFRACVAL43: 0x00000000,
        SMVAL43: 0x00000000,
        SMFRACVAL53: 0x00000000,
        SMVAL53: 0x00000000,
        SMFRCTRL3: 0x00000000,
        SMOCTRL3: 0x00000000,
        SMSTS3: 0x00000000,
        SMINTEN3: 0x00000000,
        SMDMAEN3: 0x00000000,
        SMTCTRL3: 0x00000000,
        SMDISMAP03: 0x0000FFFF,
        SMDISMAP13: 0x0000FFFF,
        SMDTCNT03: 0x000007FF,
        SMDTCNT13: 0x000007FF,
        SMCAPTCTRLA3: 0x00000000,
        SMCAPTCOMPA3: 0x00000000,
        SMCAPTCTRLB3: 0x00000000,
        SMCAPTCOMPB3: 0x00000000,
        SMCAPTCTRLX3: 0x00000000,
        SMCAPTCOMPX3: 0x00000000,
        SMCVAL03: 0x00000000,
        SMCVAL0CYC3: 0x00000000,
        SMCVAL13: 0x00000000,
        SMCVAL1CYC3: 0x00000000,
        SMCVAL23: 0x00000000,
        SMCVAL2CYC3: 0x00000000,
        SMCVAL33: 0x00000000,
        SMCVAL3CYC3: 0x00000000,
        SMCVAL43: 0x00000000,
        SMCVAL4CYC3: 0x00000000,
        SMCVAL53: 0x00000000,
        SMCVAL5CYC3: 0x00000000,
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
        SMCNT0: 0x00000000,
        SMINIT0: 0x00000000,
        SMCTRL20: 0x00000000,
        SMCTRL0: 0x00000400,
        SMVAL00: 0x00000000,
        SMFRACVAL10: 0x00000000,
        SMVAL10: 0x00000000,
        SMFRACVAL20: 0x00000000,
        SMVAL20: 0x00000000,
        SMFRACVAL30: 0x00000000,
        SMVAL30: 0x00000000,
        SMFRACVAL40: 0x00000000,
        SMVAL40: 0x00000000,
        SMFRACVAL50: 0x00000000,
        SMVAL50: 0x00000000,
        SMFRCTRL0: 0x00000000,
        SMOCTRL0: 0x00000000,
        SMSTS0: 0x00000000,
        SMINTEN0: 0x00000000,
        SMDMAEN0: 0x00000000,
        SMTCTRL0: 0x00000000,
        SMDISMAP00: 0x0000FFFF,
        SMDISMAP10: 0x0000FFFF,
        SMDTCNT00: 0x000007FF,
        SMDTCNT10: 0x000007FF,
        SMCAPTCTRLA0: 0x00000000,
        SMCAPTCOMPA0: 0x00000000,
        SMCAPTCTRLB0: 0x00000000,
        SMCAPTCOMPB0: 0x00000000,
        SMCAPTCTRLX0: 0x00000000,
        SMCAPTCOMPX0: 0x00000000,
        SMCVAL00: 0x00000000,
        SMCVAL0CYC0: 0x00000000,
        SMCVAL10: 0x00000000,
        SMCVAL1CYC0: 0x00000000,
        SMCVAL20: 0x00000000,
        SMCVAL2CYC0: 0x00000000,
        SMCVAL30: 0x00000000,
        SMCVAL3CYC0: 0x00000000,
        SMCVAL40: 0x00000000,
        SMCVAL4CYC0: 0x00000000,
        SMCVAL50: 0x00000000,
        SMCVAL5CYC0: 0x00000000,
        SMCNT1: 0x00000000,
        SMINIT1: 0x00000000,
        SMCTRL21: 0x00000000,
        SMCTRL1: 0x00000400,
        SMVAL01: 0x00000000,
        SMFRACVAL11: 0x00000000,
        SMVAL11: 0x00000000,
        SMFRACVAL21: 0x00000000,
        SMVAL21: 0x00000000,
        SMFRACVAL31: 0x00000000,
        SMVAL31: 0x00000000,
        SMFRACVAL41: 0x00000000,
        SMVAL41: 0x00000000,
        SMFRACVAL51: 0x00000000,
        SMVAL51: 0x00000000,
        SMFRCTRL1: 0x00000000,
        SMOCTRL1: 0x00000000,
        SMSTS1: 0x00000000,
        SMINTEN1: 0x00000000,
        SMDMAEN1: 0x00000000,
        SMTCTRL1: 0x00000000,
        SMDISMAP01: 0x0000FFFF,
        SMDISMAP11: 0x0000FFFF,
        SMDTCNT01: 0x000007FF,
        SMDTCNT11: 0x000007FF,
        SMCAPTCTRLA1: 0x00000000,
        SMCAPTCOMPA1: 0x00000000,
        SMCAPTCTRLB1: 0x00000000,
        SMCAPTCOMPB1: 0x00000000,
        SMCAPTCTRLX1: 0x00000000,
        SMCAPTCOMPX1: 0x00000000,
        SMCVAL01: 0x00000000,
        SMCVAL0CYC1: 0x00000000,
        SMCVAL11: 0x00000000,
        SMCVAL1CYC1: 0x00000000,
        SMCVAL21: 0x00000000,
        SMCVAL2CYC1: 0x00000000,
        SMCVAL31: 0x00000000,
        SMCVAL3CYC1: 0x00000000,
        SMCVAL41: 0x00000000,
        SMCVAL4CYC1: 0x00000000,
        SMCVAL51: 0x00000000,
        SMCVAL5CYC1: 0x00000000,
        SMCNT2: 0x00000000,
        SMINIT2: 0x00000000,
        SMCTRL22: 0x00000000,
        SMCTRL2: 0x00000400,
        SMVAL02: 0x00000000,
        SMFRACVAL12: 0x00000000,
        SMVAL12: 0x00000000,
        SMFRACVAL22: 0x00000000,
        SMVAL22: 0x00000000,
        SMFRACVAL32: 0x00000000,
        SMVAL32: 0x00000000,
        SMFRACVAL42: 0x00000000,
        SMVAL42: 0x00000000,
        SMFRACVAL52: 0x00000000,
        SMVAL52: 0x00000000,
        SMFRCTRL2: 0x00000000,
        SMOCTRL2: 0x00000000,
        SMSTS2: 0x00000000,
        SMINTEN2: 0x00000000,
        SMDMAEN2: 0x00000000,
        SMTCTRL2: 0x00000000,
        SMDISMAP02: 0x0000FFFF,
        SMDISMAP12: 0x0000FFFF,
        SMDTCNT02: 0x000007FF,
        SMDTCNT12: 0x000007FF,
        SMCAPTCTRLA2: 0x00000000,
        SMCAPTCOMPA2: 0x00000000,
        SMCAPTCTRLB2: 0x00000000,
        SMCAPTCOMPB2: 0x00000000,
        SMCAPTCTRLX2: 0x00000000,
        SMCAPTCOMPX2: 0x00000000,
        SMCVAL02: 0x00000000,
        SMCVAL0CYC2: 0x00000000,
        SMCVAL12: 0x00000000,
        SMCVAL1CYC2: 0x00000000,
        SMCVAL22: 0x00000000,
        SMCVAL2CYC2: 0x00000000,
        SMCVAL32: 0x00000000,
        SMCVAL3CYC2: 0x00000000,
        SMCVAL42: 0x00000000,
        SMCVAL4CYC2: 0x00000000,
        SMCVAL52: 0x00000000,
        SMCVAL5CYC2: 0x00000000,
        SMCNT3: 0x00000000,
        SMINIT3: 0x00000000,
        SMCTRL23: 0x00000000,
        SMCTRL3: 0x00000400,
        SMVAL03: 0x00000000,
        SMFRACVAL13: 0x00000000,
        SMVAL13: 0x00000000,
        SMFRACVAL23: 0x00000000,
        SMVAL23: 0x00000000,
        SMFRACVAL33: 0x00000000,
        SMVAL33: 0x00000000,
        SMFRACVAL43: 0x00000000,
        SMVAL43: 0x00000000,
        SMFRACVAL53: 0x00000000,
        SMVAL53: 0x00000000,
        SMFRCTRL3: 0x00000000,
        SMOCTRL3: 0x00000000,
        SMSTS3: 0x00000000,
        SMINTEN3: 0x00000000,
        SMDMAEN3: 0x00000000,
        SMTCTRL3: 0x00000000,
        SMDISMAP03: 0x0000FFFF,
        SMDISMAP13: 0x0000FFFF,
        SMDTCNT03: 0x000007FF,
        SMDTCNT13: 0x000007FF,
        SMCAPTCTRLA3: 0x00000000,
        SMCAPTCOMPA3: 0x00000000,
        SMCAPTCTRLB3: 0x00000000,
        SMCAPTCOMPB3: 0x00000000,
        SMCAPTCTRLX3: 0x00000000,
        SMCAPTCOMPX3: 0x00000000,
        SMCVAL03: 0x00000000,
        SMCVAL0CYC3: 0x00000000,
        SMCVAL13: 0x00000000,
        SMCVAL1CYC3: 0x00000000,
        SMCVAL23: 0x00000000,
        SMCVAL2CYC3: 0x00000000,
        SMCVAL33: 0x00000000,
        SMCVAL3CYC3: 0x00000000,
        SMCVAL43: 0x00000000,
        SMCVAL4CYC3: 0x00000000,
        SMCVAL53: 0x00000000,
        SMCVAL5CYC3: 0x00000000,
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
        SMCNT0: 0x00000000,
        SMINIT0: 0x00000000,
        SMCTRL20: 0x00000000,
        SMCTRL0: 0x00000400,
        SMVAL00: 0x00000000,
        SMFRACVAL10: 0x00000000,
        SMVAL10: 0x00000000,
        SMFRACVAL20: 0x00000000,
        SMVAL20: 0x00000000,
        SMFRACVAL30: 0x00000000,
        SMVAL30: 0x00000000,
        SMFRACVAL40: 0x00000000,
        SMVAL40: 0x00000000,
        SMFRACVAL50: 0x00000000,
        SMVAL50: 0x00000000,
        SMFRCTRL0: 0x00000000,
        SMOCTRL0: 0x00000000,
        SMSTS0: 0x00000000,
        SMINTEN0: 0x00000000,
        SMDMAEN0: 0x00000000,
        SMTCTRL0: 0x00000000,
        SMDISMAP00: 0x0000FFFF,
        SMDISMAP10: 0x0000FFFF,
        SMDTCNT00: 0x000007FF,
        SMDTCNT10: 0x000007FF,
        SMCAPTCTRLA0: 0x00000000,
        SMCAPTCOMPA0: 0x00000000,
        SMCAPTCTRLB0: 0x00000000,
        SMCAPTCOMPB0: 0x00000000,
        SMCAPTCTRLX0: 0x00000000,
        SMCAPTCOMPX0: 0x00000000,
        SMCVAL00: 0x00000000,
        SMCVAL0CYC0: 0x00000000,
        SMCVAL10: 0x00000000,
        SMCVAL1CYC0: 0x00000000,
        SMCVAL20: 0x00000000,
        SMCVAL2CYC0: 0x00000000,
        SMCVAL30: 0x00000000,
        SMCVAL3CYC0: 0x00000000,
        SMCVAL40: 0x00000000,
        SMCVAL4CYC0: 0x00000000,
        SMCVAL50: 0x00000000,
        SMCVAL5CYC0: 0x00000000,
        SMCNT1: 0x00000000,
        SMINIT1: 0x00000000,
        SMCTRL21: 0x00000000,
        SMCTRL1: 0x00000400,
        SMVAL01: 0x00000000,
        SMFRACVAL11: 0x00000000,
        SMVAL11: 0x00000000,
        SMFRACVAL21: 0x00000000,
        SMVAL21: 0x00000000,
        SMFRACVAL31: 0x00000000,
        SMVAL31: 0x00000000,
        SMFRACVAL41: 0x00000000,
        SMVAL41: 0x00000000,
        SMFRACVAL51: 0x00000000,
        SMVAL51: 0x00000000,
        SMFRCTRL1: 0x00000000,
        SMOCTRL1: 0x00000000,
        SMSTS1: 0x00000000,
        SMINTEN1: 0x00000000,
        SMDMAEN1: 0x00000000,
        SMTCTRL1: 0x00000000,
        SMDISMAP01: 0x0000FFFF,
        SMDISMAP11: 0x0000FFFF,
        SMDTCNT01: 0x000007FF,
        SMDTCNT11: 0x000007FF,
        SMCAPTCTRLA1: 0x00000000,
        SMCAPTCOMPA1: 0x00000000,
        SMCAPTCTRLB1: 0x00000000,
        SMCAPTCOMPB1: 0x00000000,
        SMCAPTCTRLX1: 0x00000000,
        SMCAPTCOMPX1: 0x00000000,
        SMCVAL01: 0x00000000,
        SMCVAL0CYC1: 0x00000000,
        SMCVAL11: 0x00000000,
        SMCVAL1CYC1: 0x00000000,
        SMCVAL21: 0x00000000,
        SMCVAL2CYC1: 0x00000000,
        SMCVAL31: 0x00000000,
        SMCVAL3CYC1: 0x00000000,
        SMCVAL41: 0x00000000,
        SMCVAL4CYC1: 0x00000000,
        SMCVAL51: 0x00000000,
        SMCVAL5CYC1: 0x00000000,
        SMCNT2: 0x00000000,
        SMINIT2: 0x00000000,
        SMCTRL22: 0x00000000,
        SMCTRL2: 0x00000400,
        SMVAL02: 0x00000000,
        SMFRACVAL12: 0x00000000,
        SMVAL12: 0x00000000,
        SMFRACVAL22: 0x00000000,
        SMVAL22: 0x00000000,
        SMFRACVAL32: 0x00000000,
        SMVAL32: 0x00000000,
        SMFRACVAL42: 0x00000000,
        SMVAL42: 0x00000000,
        SMFRACVAL52: 0x00000000,
        SMVAL52: 0x00000000,
        SMFRCTRL2: 0x00000000,
        SMOCTRL2: 0x00000000,
        SMSTS2: 0x00000000,
        SMINTEN2: 0x00000000,
        SMDMAEN2: 0x00000000,
        SMTCTRL2: 0x00000000,
        SMDISMAP02: 0x0000FFFF,
        SMDISMAP12: 0x0000FFFF,
        SMDTCNT02: 0x000007FF,
        SMDTCNT12: 0x000007FF,
        SMCAPTCTRLA2: 0x00000000,
        SMCAPTCOMPA2: 0x00000000,
        SMCAPTCTRLB2: 0x00000000,
        SMCAPTCOMPB2: 0x00000000,
        SMCAPTCTRLX2: 0x00000000,
        SMCAPTCOMPX2: 0x00000000,
        SMCVAL02: 0x00000000,
        SMCVAL0CYC2: 0x00000000,
        SMCVAL12: 0x00000000,
        SMCVAL1CYC2: 0x00000000,
        SMCVAL22: 0x00000000,
        SMCVAL2CYC2: 0x00000000,
        SMCVAL32: 0x00000000,
        SMCVAL3CYC2: 0x00000000,
        SMCVAL42: 0x00000000,
        SMCVAL4CYC2: 0x00000000,
        SMCVAL52: 0x00000000,
        SMCVAL5CYC2: 0x00000000,
        SMCNT3: 0x00000000,
        SMINIT3: 0x00000000,
        SMCTRL23: 0x00000000,
        SMCTRL3: 0x00000400,
        SMVAL03: 0x00000000,
        SMFRACVAL13: 0x00000000,
        SMVAL13: 0x00000000,
        SMFRACVAL23: 0x00000000,
        SMVAL23: 0x00000000,
        SMFRACVAL33: 0x00000000,
        SMVAL33: 0x00000000,
        SMFRACVAL43: 0x00000000,
        SMVAL43: 0x00000000,
        SMFRACVAL53: 0x00000000,
        SMVAL53: 0x00000000,
        SMFRCTRL3: 0x00000000,
        SMOCTRL3: 0x00000000,
        SMSTS3: 0x00000000,
        SMINTEN3: 0x00000000,
        SMDMAEN3: 0x00000000,
        SMTCTRL3: 0x00000000,
        SMDISMAP03: 0x0000FFFF,
        SMDISMAP13: 0x0000FFFF,
        SMDTCNT03: 0x000007FF,
        SMDTCNT13: 0x000007FF,
        SMCAPTCTRLA3: 0x00000000,
        SMCAPTCOMPA3: 0x00000000,
        SMCAPTCTRLB3: 0x00000000,
        SMCAPTCOMPB3: 0x00000000,
        SMCAPTCTRLX3: 0x00000000,
        SMCAPTCOMPX3: 0x00000000,
        SMCVAL03: 0x00000000,
        SMCVAL0CYC3: 0x00000000,
        SMCVAL13: 0x00000000,
        SMCVAL1CYC3: 0x00000000,
        SMCVAL23: 0x00000000,
        SMCVAL2CYC3: 0x00000000,
        SMCVAL33: 0x00000000,
        SMCVAL3CYC3: 0x00000000,
        SMCVAL43: 0x00000000,
        SMCVAL4CYC3: 0x00000000,
        SMCVAL53: 0x00000000,
        SMCVAL5CYC3: 0x00000000,
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
        SMCNT0: 0x00000000,
        SMINIT0: 0x00000000,
        SMCTRL20: 0x00000000,
        SMCTRL0: 0x00000400,
        SMVAL00: 0x00000000,
        SMFRACVAL10: 0x00000000,
        SMVAL10: 0x00000000,
        SMFRACVAL20: 0x00000000,
        SMVAL20: 0x00000000,
        SMFRACVAL30: 0x00000000,
        SMVAL30: 0x00000000,
        SMFRACVAL40: 0x00000000,
        SMVAL40: 0x00000000,
        SMFRACVAL50: 0x00000000,
        SMVAL50: 0x00000000,
        SMFRCTRL0: 0x00000000,
        SMOCTRL0: 0x00000000,
        SMSTS0: 0x00000000,
        SMINTEN0: 0x00000000,
        SMDMAEN0: 0x00000000,
        SMTCTRL0: 0x00000000,
        SMDISMAP00: 0x0000FFFF,
        SMDISMAP10: 0x0000FFFF,
        SMDTCNT00: 0x000007FF,
        SMDTCNT10: 0x000007FF,
        SMCAPTCTRLA0: 0x00000000,
        SMCAPTCOMPA0: 0x00000000,
        SMCAPTCTRLB0: 0x00000000,
        SMCAPTCOMPB0: 0x00000000,
        SMCAPTCTRLX0: 0x00000000,
        SMCAPTCOMPX0: 0x00000000,
        SMCVAL00: 0x00000000,
        SMCVAL0CYC0: 0x00000000,
        SMCVAL10: 0x00000000,
        SMCVAL1CYC0: 0x00000000,
        SMCVAL20: 0x00000000,
        SMCVAL2CYC0: 0x00000000,
        SMCVAL30: 0x00000000,
        SMCVAL3CYC0: 0x00000000,
        SMCVAL40: 0x00000000,
        SMCVAL4CYC0: 0x00000000,
        SMCVAL50: 0x00000000,
        SMCVAL5CYC0: 0x00000000,
        SMCNT1: 0x00000000,
        SMINIT1: 0x00000000,
        SMCTRL21: 0x00000000,
        SMCTRL1: 0x00000400,
        SMVAL01: 0x00000000,
        SMFRACVAL11: 0x00000000,
        SMVAL11: 0x00000000,
        SMFRACVAL21: 0x00000000,
        SMVAL21: 0x00000000,
        SMFRACVAL31: 0x00000000,
        SMVAL31: 0x00000000,
        SMFRACVAL41: 0x00000000,
        SMVAL41: 0x00000000,
        SMFRACVAL51: 0x00000000,
        SMVAL51: 0x00000000,
        SMFRCTRL1: 0x00000000,
        SMOCTRL1: 0x00000000,
        SMSTS1: 0x00000000,
        SMINTEN1: 0x00000000,
        SMDMAEN1: 0x00000000,
        SMTCTRL1: 0x00000000,
        SMDISMAP01: 0x0000FFFF,
        SMDISMAP11: 0x0000FFFF,
        SMDTCNT01: 0x000007FF,
        SMDTCNT11: 0x000007FF,
        SMCAPTCTRLA1: 0x00000000,
        SMCAPTCOMPA1: 0x00000000,
        SMCAPTCTRLB1: 0x00000000,
        SMCAPTCOMPB1: 0x00000000,
        SMCAPTCTRLX1: 0x00000000,
        SMCAPTCOMPX1: 0x00000000,
        SMCVAL01: 0x00000000,
        SMCVAL0CYC1: 0x00000000,
        SMCVAL11: 0x00000000,
        SMCVAL1CYC1: 0x00000000,
        SMCVAL21: 0x00000000,
        SMCVAL2CYC1: 0x00000000,
        SMCVAL31: 0x00000000,
        SMCVAL3CYC1: 0x00000000,
        SMCVAL41: 0x00000000,
        SMCVAL4CYC1: 0x00000000,
        SMCVAL51: 0x00000000,
        SMCVAL5CYC1: 0x00000000,
        SMCNT2: 0x00000000,
        SMINIT2: 0x00000000,
        SMCTRL22: 0x00000000,
        SMCTRL2: 0x00000400,
        SMVAL02: 0x00000000,
        SMFRACVAL12: 0x00000000,
        SMVAL12: 0x00000000,
        SMFRACVAL22: 0x00000000,
        SMVAL22: 0x00000000,
        SMFRACVAL32: 0x00000000,
        SMVAL32: 0x00000000,
        SMFRACVAL42: 0x00000000,
        SMVAL42: 0x00000000,
        SMFRACVAL52: 0x00000000,
        SMVAL52: 0x00000000,
        SMFRCTRL2: 0x00000000,
        SMOCTRL2: 0x00000000,
        SMSTS2: 0x00000000,
        SMINTEN2: 0x00000000,
        SMDMAEN2: 0x00000000,
        SMTCTRL2: 0x00000000,
        SMDISMAP02: 0x0000FFFF,
        SMDISMAP12: 0x0000FFFF,
        SMDTCNT02: 0x000007FF,
        SMDTCNT12: 0x000007FF,
        SMCAPTCTRLA2: 0x00000000,
        SMCAPTCOMPA2: 0x00000000,
        SMCAPTCTRLB2: 0x00000000,
        SMCAPTCOMPB2: 0x00000000,
        SMCAPTCTRLX2: 0x00000000,
        SMCAPTCOMPX2: 0x00000000,
        SMCVAL02: 0x00000000,
        SMCVAL0CYC2: 0x00000000,
        SMCVAL12: 0x00000000,
        SMCVAL1CYC2: 0x00000000,
        SMCVAL22: 0x00000000,
        SMCVAL2CYC2: 0x00000000,
        SMCVAL32: 0x00000000,
        SMCVAL3CYC2: 0x00000000,
        SMCVAL42: 0x00000000,
        SMCVAL4CYC2: 0x00000000,
        SMCVAL52: 0x00000000,
        SMCVAL5CYC2: 0x00000000,
        SMCNT3: 0x00000000,
        SMINIT3: 0x00000000,
        SMCTRL23: 0x00000000,
        SMCTRL3: 0x00000400,
        SMVAL03: 0x00000000,
        SMFRACVAL13: 0x00000000,
        SMVAL13: 0x00000000,
        SMFRACVAL23: 0x00000000,
        SMVAL23: 0x00000000,
        SMFRACVAL33: 0x00000000,
        SMVAL33: 0x00000000,
        SMFRACVAL43: 0x00000000,
        SMVAL43: 0x00000000,
        SMFRACVAL53: 0x00000000,
        SMVAL53: 0x00000000,
        SMFRCTRL3: 0x00000000,
        SMOCTRL3: 0x00000000,
        SMSTS3: 0x00000000,
        SMINTEN3: 0x00000000,
        SMDMAEN3: 0x00000000,
        SMTCTRL3: 0x00000000,
        SMDISMAP03: 0x0000FFFF,
        SMDISMAP13: 0x0000FFFF,
        SMDTCNT03: 0x000007FF,
        SMDTCNT13: 0x000007FF,
        SMCAPTCTRLA3: 0x00000000,
        SMCAPTCOMPA3: 0x00000000,
        SMCAPTCTRLB3: 0x00000000,
        SMCAPTCOMPB3: 0x00000000,
        SMCAPTCTRLX3: 0x00000000,
        SMCAPTCOMPX3: 0x00000000,
        SMCVAL03: 0x00000000,
        SMCVAL0CYC3: 0x00000000,
        SMCVAL13: 0x00000000,
        SMCVAL1CYC3: 0x00000000,
        SMCVAL23: 0x00000000,
        SMCVAL2CYC3: 0x00000000,
        SMCVAL33: 0x00000000,
        SMCVAL3CYC3: 0x00000000,
        SMCVAL43: 0x00000000,
        SMCVAL4CYC3: 0x00000000,
        SMCVAL53: 0x00000000,
        SMCVAL5CYC3: 0x00000000,
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
