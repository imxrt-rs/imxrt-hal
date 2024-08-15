//! Custom RAL API for PWM submodules.
//!
//! The auto-generated RAL API is cumbersome. This is a macro-compatible API that's
//! easier to work with. It focuses on de-duplicating symbols for the submodules.

#![allow(unused, non_snake_case, non_upper_case_globals)] // Compatibility with RAL
use crate::ral::{RORegister, RWRegister};

/// This is SM0 from the 1060 PWM peripheral, with the
/// '0's removed. We then use *signed* registers when the
/// reference manual indicates a signed value.
#[repr(C)]
pub struct RegisterBlock {
    /// Counter Register
    pub SMCNT: RORegister<i16>,

    /// Initial Count Register
    pub SMINIT: RWRegister<i16>,

    /// Control 2 Register
    pub SMCTRL2: RWRegister<u16>,

    /// Control Register
    pub SMCTRL: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// Value Register 0
    pub SMVAL0: RWRegister<i16>,

    /// Fractional Value Register 1
    pub SMFRACVAL1: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL1: RWRegister<i16>,

    /// Fractional Value Register 2
    pub SMFRACVAL2: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL2: RWRegister<i16>,

    /// Fractional Value Register 3
    pub SMFRACVAL3: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL3: RWRegister<i16>,

    /// Fractional Value Register 4
    pub SMFRACVAL4: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL4: RWRegister<i16>,

    /// Fractional Value Register 5
    pub SMFRACVAL5: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL5: RWRegister<i16>,

    /// Fractional Control Register
    pub SMFRCTRL: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL: RWRegister<u16>,

    /// Status Register
    pub SMSTS: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT1: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL4: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL5: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC: RORegister<u16>,

    /// Phase Delay Register
    pub SMPHASEDLY: RWRegister<u16>,

    _reserved2: [u16; 3],
}

const _: () = assert!(core::mem::size_of::<RegisterBlock>() == 0x60);

/// A PWM submodule.
///
/// `Submodule`s implement PWM timers. Comparisons in `Submodule` value
/// registers generate PWM outputs.
///
/// The PWM submodule `FlexPWM2_3` is represented as
///
/// ```
/// # use imxrt_hal::flexpwm::Submodule;
/// type PWM2_3 = Submodule<2, 3>;
/// ```
pub struct Submodule<const N: u8, const M: u8>(*const RegisterBlock);

impl<const N: u8, const M: u8> ::core::ops::Deref for Submodule<N, M> {
    type Target = RegisterBlock;
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: Pointer is valid per `submodules` implementation, below.
        // Layout of RegisterBlock is checked against the reference manual.
        // The size of RegisterBlock is correct, meaning that we never access
        // beyond the memory of the original pointer.
        unsafe { &*self.0 }
    }
}

// Safety: The pointer wrapped by Submodule points into static MMIO registers;
// see `submodules` for specifics. That owned pointer can be sent across execution
// contexts.
unsafe impl<const N: u8, const M: u8> Send for Submodule<N, M> {}

/// Four submodules for the `N`th PWM instance.
pub type Submodules<const N: u8> = (
    Submodule<N, 0>,
    Submodule<N, 1>,
    Submodule<N, 2>,
    Submodule<N, 3>,
);

/// Shorthand to allocate four submodules for PWM `N`.
pub(crate) fn submodules<const N: u8>(pwm: &crate::ral::pwm::Instance<N>) -> Submodules<N> {
    (
        Submodule(core::ptr::addr_of!(pwm.SM[0]) as *const _),
        Submodule(core::ptr::addr_of!(pwm.SM[1]) as *const _),
        Submodule(core::ptr::addr_of!(pwm.SM[2]) as *const _),
        Submodule(core::ptr::addr_of!(pwm.SM[3]) as *const _),
    )
}

// Export all submodule fields.
pub use crate::ral::pwm::sm::*;
