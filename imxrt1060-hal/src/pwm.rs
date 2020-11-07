//! Pulse Width Modulation (PWM)
//!
//! Provides an implementation of `embedded_hal::Pwm` for iMXRT PWM
//! submodules. The flow follows
//!
//! - Enable clocks to the selected PWM peripheral by calling `clock()` on
//!   an `Unclocked` struct. The return is a `PWM` type that provides a register
//!   handle and PWM submodules
//! - Turn submodules into `Pins` by supplying two processor pins to `output`. The
//!   return is a `Pins` struct that wraps the two pins. The `Pins` struct are inert;
//!   they're simply a handle that can provide PWM control. Call `control()` and pass
//!   in a `Handle`, to acquire a `Controller`.
//! - `Controller` implements `embedded_hal::Pwm`. It lets you set PWM duty cycles.
//!   Once you're done setting duty cycles, drop the `Controller`.
//!
//! # Example
//!
//! ```no_run
//! use imxrt1060_hal;
//! use imxrt1060_hal::pwm::Channel;
//! use embedded_hal::Pwm;
//!
//! let mut p = imxrt1060_hal::Peripherals::take().unwrap();
//!
//! let (_, ipg_hz) =
//!     p.ccm
//!         .pll1
//!         .set_arm_clock(imxrt1060_hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
//!
//! let mut pwm2 = p.pwm2.clock(&mut p.ccm.handle);
//!
//! let mut sm2 = pwm2
//!     .sm2
//!     .outputs(
//!         &mut pwm2.handle,
//!         p.iomuxc.b0.p10,
//!         p.iomuxc.b0.p11,
//!         imxrt1060_hal::pwm::Timing {
//!             clock_select: imxrt1060_hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
//!             prescalar: imxrt1060_hal::ccm::pwm::Prescalar::PRSC_5,
//!             switching_period: core::time::Duration::from_micros(1000),
//!         },
//!     )
//!     .unwrap();
//!
//! let (duty1, duty2) = (core::u16::MAX / 4, core::u16::MAX / 2);
//! let mut ctrl = sm2.control(&mut pwm2.handle);
//!
//! ctrl.enable(Channel::A);
//! ctrl.enable(Channel::B);
//! ctrl.set_duty(Channel::A, duty1);
//! ctrl.set_duty(Channel::B, duty2);
//! ```

use crate::ccm;
use crate::iomuxc::consts::{Unsigned, U0, U1, U2, U3, U4};
use crate::iomuxc::pwm;
use crate::iomuxc::pwm::Pin;
use crate::ral::{self, pwm::Instance};
use core::marker::PhantomData;
use core::ops::DerefMut;

use embedded_hal::Pwm;

/// PWM peripheral handle
///
/// Most operations that could affect multiple submodules require
/// access to this handle.
pub struct Handle<M> {
    reg: Instance,
    _marker: PhantomData<M>,
}

/// A PWM submodule
///
/// A submodule may be consumed to create PWM pin instances
pub struct Submodule<M, S> {
    _marker: PhantomData<(M, S)>,
}

/// A PWM peripheral
///
/// The PWM peripheral is broken into
///
/// - the PWM master handle, `handle`,
/// - four submodules, numbered `0` through `3`.
///
/// The submodules are taken when you want to turn pins into
/// PWM outputs. The handle provides access to registers that
/// are shared across PWM submodules.
pub struct PWM<M> {
    /// The peripheral handle
    ///
    /// Methods that need access to peripheral-level registers, rather than
    /// just submodule-level registers, must have a mutable reference to
    /// the handle.
    pub handle: Handle<M>,
    /// Submodule 0
    pub sm0: Submodule<M, U0>,
    /// Submodule 1
    pub sm1: Submodule<M, U1>,
    /// Submodule 2
    pub sm2: Submodule<M, U2>,
    /// Submodule 3
    pub sm3: Submodule<M, U3>,
}

impl<M> PWM<M>
where
    M: Unsigned,
{
    fn new(reg: Instance) -> Self {
        // Clear any fault levels
        ral::write_reg!(ral::pwm, reg, FCTRL0, FLVL: 0xF);
        // Clear fault flags
        ral::write_reg!(ral::pwm, reg, FSTS0, FFLAG: 0xF);
        PWM {
            handle: Handle {
                reg,
                _marker: PhantomData,
            },
            sm0: Submodule {
                _marker: PhantomData::<(M, U0)>,
            },
            sm1: Submodule {
                _marker: PhantomData::<(M, U1)>,
            },
            sm2: Submodule {
                _marker: PhantomData::<(M, U2)>,
            },
            sm3: Submodule {
                _marker: PhantomData::<(M, U3)>,
            },
        }
    }
}

/// Executes `act` while the PWM peripheral is not loaded. Once the action completes, load any changes
/// incured by the action. Useful for setting VAL registers.
fn while_reset<M, S, F, R>(handle: &mut Handle<M>, act: F) -> R
where
    M: Unsigned,
    S: Unsigned,
    F: FnOnce(&mut Handle<M>) -> R,
{
    ral::modify_reg!(ral::pwm, handle.reg, MCTRL, CLDOK: 1 << <S as Unsigned>::USIZE);
    let result = act(handle);
    ral::modify_reg!(ral::pwm, handle.reg, MCTRL, LDOK: 1 << <S as Unsigned>::USIZE);
    result
}

macro_rules! submodule_outputs {
    ($SUBMODULE:path, $SMCTRL2:ident, $SMCTRL:ident, $SMOCTRL:ident, $SMDTCNT0:ident, $SMINIT: ident, $SMVAL0:ident, $SMVAL1:ident, $SMVAL2:ident, $SMVAL3:ident, $SMVAL4:ident, $SMVAL5:ident) => {
        impl<M> Submodule<M, $SUBMODULE>
        where
            M: Unsigned,
        {
            /// Converts two pins into PWM outputs. Returns a `Pins` type that wraps the
            /// underlying pins.
            ///
            /// The input pins must support PWM functionality. They must match the module
            /// that they're associated, and they must have the same submodule.
            ///
            /// Requires a mutable reference to a `Handle` in order to modify registers
            /// that are shared across all PWM submodules.s
            pub fn outputs<A, B>(
                self,
                handle: &mut Handle<M>,
                pin_a: A,
                pin_b: B,
                timing: Timing,
            ) -> Option<Pins<A, B>>
            where
                A: Pin<Module = M, Submodule = $SUBMODULE, Output = pwm::A>,
                B: Pin<Module = M, Submodule = $SUBMODULE, Output = pwm::B>,
            {
                let clk_sel: u16 = match timing.clock_select {
                    ccm::pwm::ClockSelect::IPG(_) => ral::pwm::SMCTRL20::CLK_SEL::RW::CLK_SEL_0 as u16,
                };
                while_reset::<M, $SUBMODULE, _, _>(handle, |handle| {
                    // TODO some of these don't have flags in the SVD. May consider adding them.
                    ral::write_reg!(ral::pwm, handle.reg, $SMCTRL2,
                        WAITEN: 1u16,       // Run while in wait mode
                        DBGEN: 1u16,        // Run while in debug mode
                        INDEP: INDEP_1,     // Independent output, as opposed to complementary output
                        CLK_SEL: clk_sel);
                    ral::write_reg!(ral::pwm, handle.reg, $SMCTRL, FULL: FULL_1, PRSC: (timing.prescalar as u16));
                    ral::write_reg!(ral::pwm, handle.reg, $SMOCTRL, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMDTCNT0, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMINIT, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL0, 0);

                    let ticks: u16 = ccm::ticks(
                        timing.switching_period,
                        ccm::Frequency::from(timing.clock_select).0,
                        ccm::Divider::from(timing.prescalar).0,
                    ).ok()?;

                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL1, ticks);
                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL2, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL3, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL4, 0);
                    ral::write_reg!(ral::pwm, handle.reg, $SMVAL5, 0);

                    Some(())
                })?;
                ral::modify_reg!(ral::pwm, handle.reg, MCTRL, RUN: 1 << <$SUBMODULE as Unsigned>::USIZE);
                Some(Pins::new(pin_a, pin_b, timing))
            }
        }
    };
}

submodule_outputs!(
    U0, SMCTRL20, SMCTRL0, SMOCTRL0, SMDTCNT00, SMINIT0, SMVAL00, SMVAL10, SMVAL20, SMVAL30,
    SMVAL40, SMVAL50
);
submodule_outputs!(
    U1, SMCTRL21, SMCTRL1, SMOCTRL1, SMDTCNT01, SMINIT1, SMVAL01, SMVAL11, SMVAL21, SMVAL31,
    SMVAL41, SMVAL51
);
submodule_outputs!(
    U2, SMCTRL22, SMCTRL2, SMOCTRL2, SMDTCNT02, SMINIT2, SMVAL02, SMVAL12, SMVAL22, SMVAL32,
    SMVAL42, SMVAL52
);
submodule_outputs!(
    U3, SMCTRL23, SMCTRL3, SMOCTRL3, SMDTCNT03, SMINIT3, SMVAL03, SMVAL13, SMVAL23, SMVAL33,
    SMVAL43, SMVAL53
);

/// A pair of submodule PWM pins
///
/// When taken in a `Controller`, you may configure the PWM outputs
pub struct Pins<A, B> {
    _pin_a: A,
    _pin_b: B,
    timing: Timing,
}

impl<A, B> Pins<A, B>
where
    A: Pin<Output = pwm::A>,
    B: Pin<Output = pwm::B, Module = <A as Pin>::Module, Submodule = <A as Pin>::Submodule>,
{
    fn new(pin_a: A, pin_b: B, timing: Timing) -> Self {
        Pins {
            _pin_a: pin_a,
            _pin_b: pin_b,
            timing,
        }
    }
    /// Provides control of PWM pins
    ///
    /// Supply a type that provides mutable access to the PWM handle. The handle is required
    /// to modify peripheral-wide registers for safe manipulation.
    pub fn control<'a, D>(&'a mut self, handle: D) -> Controller<A, B, D, <A as Pin>::Submodule>
    where
        D: 'a + DerefMut<Target = Handle<<A as Pin>::Module>>,
    {
        Controller::new(self, handle)
    }
}

/// A PWM pin channel
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channel {
    /// Channel A
    A,
    /// Channel B
    B,
}

/// A PWM controller, which implements
///
/// The `Controller` enables you to set PWM duty cycles and switching periods.
/// It requires mutable access to both the PWM handle and the PWM pins that you
/// want to control. Once you've set your values, drop the controller. Pair the
/// handle with another `Pair` to set those values.
pub struct Controller<'a, A, B, D, S> {
    pins: &'a mut Pins<A, B>,
    handle: D,
    _submodule: PhantomData<S>,
}

impl<'a, A, B, D, S> Controller<'a, A, B, D, S>
where
    A: Pin<Output = pwm::A>,
    B: Pin<Output = pwm::B, Module = <A as Pin>::Module, Submodule = <A as Pin>::Submodule>,
    D: 'a + DerefMut<Target = Handle<<A as Pin>::Module>>,
    S: Unsigned,
{
    const IDX: u16 = <<A as Pin>::Submodule as Unsigned>::USIZE as u16;
    fn new(pins: &'a mut Pins<A, B>, handle: D) -> Self {
        Self {
            pins,
            handle,
            _submodule: PhantomData,
        }
    }
}

macro_rules! controller {
    ($SUBMODULE: path, $SMVAL1: ident, $SMVAL3: ident, $SMVAL5: ident) => {
        impl<'a, A, B, D> Pwm for Controller<'a, A, B, D, $SUBMODULE>
        where
            A: Pin<Output = pwm::A, Submodule = $SUBMODULE>,
            B: Pin<Output = pwm::B, Module = <A as Pin>::Module, Submodule = <A as Pin>::Submodule>,
            D: 'a + DerefMut<Target = Handle<<A as Pin>::Module>>,
        {
            type Channel = Channel;
            type Time = core::time::Duration;
            type Duty = u16;

            fn disable(&mut self, channel: Self::Channel) {
                let channel_offset = match channel {
                    Channel::A => 8,
                    Channel::B => 4,
                };
                let offset = channel_offset + Self::IDX;
                let outen: u16 = ral::read_reg!(ral::pwm, self.handle.reg, OUTEN);
                ral::write_reg!(ral::pwm, self.handle.reg, OUTEN, outen & !(1u16 << offset));
            }

            fn enable(&mut self, channel: Self::Channel) {
                let channel_offset = match channel {
                    Channel::A => 8,
                    Channel::B => 4,
                };
                let offset = channel_offset + Self::IDX;
                let outen: u16 = ral::read_reg!(ral::pwm, self.handle.reg, OUTEN);
                ral::write_reg!(ral::pwm, self.handle.reg, OUTEN, outen | (1u16 << offset));
            }

            fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
                let modulo: u32 = ral::read_reg!(ral::pwm, self.handle.reg, $SMVAL1) as u32;
                let cval: u32 = match channel {
                    Channel::A => ral::read_reg!(ral::pwm, self.handle.reg, $SMVAL3) as u32,
                    Channel::B => ral::read_reg!(ral::pwm, self.handle.reg, $SMVAL5) as u32,
                };
                ((cval << 16) / (modulo + 1)) as u16
            }

            fn get_period(&self) -> Self::Time {
                self.pins.timing.switching_period
            }

            fn get_max_duty(&self) -> Self::Duty {
                u16::max_value()
            }

            fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
                while_reset::<<A as Pin>::Module, <A as Pin>::Submodule, _, _>(
                    &mut self.handle,
                    |handle| {
                        let modulo: u32 = ral::read_reg!(ral::pwm, handle.reg, $SMVAL1) as u32;
                        let cval: u32 = ((duty as u32) * (modulo + 1)) >> 16;
                        let cval = if cval > modulo {
                            modulo as u16
                        } else {
                            cval as u16
                        };
                        match channel {
                            Channel::A => ral::write_reg!(ral::pwm, handle.reg, $SMVAL3, cval),
                            Channel::B => ral::write_reg!(ral::pwm, handle.reg, $SMVAL5, cval),
                        }
                    },
                );
            }

            fn set_period<P: Into<Self::Time>>(&mut self, period: P) {
                let period = period.into();
                if let Ok(ticks) = ccm::ticks(
                    period,
                    ccm::Frequency::from(self.pins.timing.clock_select).0,
                    ccm::Divider::from(self.pins.timing.prescalar).0,
                ) {
                    self.pins.timing.switching_period = period;
                    while_reset::<<A as Pin>::Module, <A as Pin>::Submodule, _, _>(
                        &mut self.handle,
                        |handle| {
                            ral::write_reg!(ral::pwm, handle.reg, $SMVAL1, ticks);
                        },
                    );
                }
            }
        }
    };
}

controller!(U0, SMVAL10, SMVAL30, SMVAL50);
controller!(U1, SMVAL11, SMVAL31, SMVAL51);
controller!(U2, SMVAL12, SMVAL32, SMVAL52);
controller!(U3, SMVAL13, SMVAL33, SMVAL53);

/// A PWM peripheral that is not receiving a clock input
///
/// You may access the PWM components by using the `clock()` method.
pub struct Unclocked<M> {
    reg: Instance,
    _module: PhantomData<M>,
}

impl<M> Unclocked<M>
where
    M: Unsigned,
{
    pub(crate) fn new(reg: Instance) -> Self {
        Unclocked {
            reg,
            _module: PhantomData,
        }
    }
}

macro_rules! clock_impl {
    ($module:path, $cg:ident) => {
        impl Unclocked<$module> {
            /// Enable the input clock for this PWM module. Returns a `PWM` instance
            /// that can allocated PWM outputs.
            pub fn clock(self, handle: &mut ccm::Handle) -> PWM<$module> {
                let (ccm, _) = handle.raw();
                ral::modify_reg!(ral::ccm, ccm, CCGR4, $cg: 0x3);
                PWM::new(self.reg)
            }
        }
    };
}

clock_impl!(U1, CG8);
clock_impl!(U2, CG9);
clock_impl!(U3, CG10);
clock_impl!(U4, CG11);

/// Specifies the timing-related parameters for a PWM submodule
#[derive(Clone, Copy)]
pub struct Timing {
    /// The clock selection for the PWM submodule
    pub clock_select: ccm::pwm::ClockSelect,
    /// The clock divider for the PWM submodule
    pub prescalar: ccm::pwm::Prescalar,
    /// The driving (switching) frequency, expressed as a period
    pub switching_period: core::time::Duration,
}
