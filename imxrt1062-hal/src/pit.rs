//! Periodic Interrupt Timer (PIT)
//!
//! # Example
//!
//! ```no_run
//! use imxrt1062_hal;
//! use embedded_hal::timer::CountDown;
//!
//! let mut peripherals = imxrt1062_hal::Peripherals::take().unwrap();
//!
//! let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
//!     imxrt1062_hal::ccm::PLL1::ARM_HZ,
//!     &mut peripherals.ccm.handle,
//!     &mut peripherals.dcdc,
//! );
//!
//! let mut cfg = peripherals.ccm.perclk.configure(
//!     &mut peripherals.ccm.handle,
//!     imxrt1062_hal::ccm::perclk::PODF::DIVIDE_3,
//!     imxrt1062_hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
//! );
//!
//! let (_, _, _, mut timer) = peripherals.pit.clock(&mut cfg);
//! timer.start(core::time::Duration::from_micros(200));
//! ```

use crate::ccm::{perclk, ticks, Divider, Frequency, TicksError};
use crate::ral;
use core::marker::PhantomData;
use embedded_hal::timer::{CountDown, Periodic};

/// An unclocked periodic interrupt timer module
///
/// In order to activate the PIT, we must pass in the
/// configured object returned from the CCM's perclk module.
pub struct UnclockedPIT(ral::pit::Instance);

impl UnclockedPIT {
    pub(crate) fn new(base: ral::pit::Instance) -> Self {
        UnclockedPIT(base)
    }

    /// Activate the PIT module after enabling the clock for the
    /// module.
    pub fn clock(
        self,
        configured: &mut perclk::Configured,
    ) -> (
        PIT<channel::_0>,
        PIT<channel::_1>,
        PIT<channel::_2>,
        PIT<channel::_3>,
    ) {
        let (clock_hz, divider) = configured.enable_pit_clock_gates();
        ral::write_reg!(ral::pit, self.0, MCR, MDIS: MDIS_0);
        // Intentionally dropping the ral::pit::Instance. We will give consumers
        // the appearance that we own it so that they cannot subsequently take it.
        (
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
            PIT::new(clock_hz, divider),
        )
    }
}

pub mod channel {
    use crate::ral;

    /// Dummy channel for describing channel chaining.
    ///
    /// Timer 0 cannot be chained. This is the only "valid" chainable
    /// channel, but it does not exist.
    #[doc(hidden)]
    pub struct _X;
    /// PIT channel 0
    pub struct _0;
    /// PIT channel 1
    pub struct _1;
    /// PIT channel 2
    pub struct _2;
    /// PIT channel 3
    pub struct _3;

    #[doc(hidden)]
    pub trait Channel {
        type ChainedTo: Channel;

        fn enabled() -> bool;
        fn set_enabled(enable: bool);
        fn set_ldval(val: u32);
        fn ldval() -> u32;
        fn cval() -> u32;
        fn tif() -> bool;
        fn clear_tif();
        fn set_interrupt_enable(interrupt: bool);
        fn interrupt_enable() -> bool;
        fn enable_chain();
    }

    macro_rules! _impl_channel {
        ($chan:ty, $chain:ty, $tctrl:ident, $ldval:ident, $tflg:ident, $cval:ident) => {
            impl Channel for $chan {
                type ChainedTo = $chain;

                #[inline(always)]
                fn enabled() -> bool {
                    unsafe { ral::read_reg!(ral::pit, ral::pit::PIT, $tctrl, TEN == TEN_1) }
                }

                #[inline(always)]
                fn set_enabled(enable: bool) {
                    unsafe {
                        ral::modify_reg!(ral::pit, ral::pit::PIT, $tctrl, TEN: u32::from(enable));
                    }
                }

                #[inline(always)]
                fn set_ldval(val: u32) {
                    unsafe {
                        ral::write_reg!(ral::pit, ral::pit::PIT, $ldval, TSV: val);
                    }
                }

                #[inline(always)]
                fn ldval() -> u32 {
                    unsafe { ral::read_reg!(ral::pit, ral::pit::PIT, $ldval, TSV) }
                }

                #[inline(always)]
                fn cval() -> u32 {
                    unsafe { ral::read_reg!(ral::pit, ral::pit::PIT, $cval, TVL) }
                }

                #[inline(always)]
                fn tif() -> bool {
                    unsafe { ral::read_reg!(ral::pit, ral::pit::PIT, $tflg, TIF == TIF_1) }
                }

                #[inline(always)]
                fn clear_tif() {
                    unsafe {
                        ral::write_reg!(ral::pit, ral::pit::PIT, $tflg, TIF: TIF_1);
                    }
                }

                #[inline(always)]
                fn set_interrupt_enable(interrupt: bool) {
                    unsafe {
                        ral::modify_reg!(
                            ral::pit,
                            ral::pit::PIT,
                            $tctrl,
                            TIE: u32::from(interrupt)
                        );
                    }
                }

                #[inline(always)]
                fn interrupt_enable() -> bool {
                    unsafe { ral::read_reg!(ral::pit, ral::pit::PIT, $tctrl, TIE == TIE_1) }
                }

                #[inline(always)]
                fn enable_chain() {
                    unsafe {
                        ral::modify_reg!(ral::pit, ral::pit::PIT, $tctrl, CHN: CHN_1);
                    }
                }
            }
        };
    }

    /// Dummy channel for describing channel chaining.
    ///
    /// Timer 0 cannot be chained. This is the only "valid" chainable
    /// channel, but it does not exist.
    ///
    /// All methods are unreachable, because we cannot call them.
    impl Channel for _X {
        type ChainedTo = _X;
        fn enabled() -> bool {
            unreachable!()
        }
        fn set_enabled(_: bool) {
            unreachable!()
        }
        fn set_ldval(_: u32) {
            unreachable!()
        }
        fn ldval() -> u32 {
            unreachable!()
        }
        fn cval() -> u32 {
            unreachable!()
        }
        fn tif() -> bool {
            unreachable!()
        }
        fn clear_tif() {
            unreachable!()
        }
        fn set_interrupt_enable(_: bool) {
            unreachable!()
        }
        fn interrupt_enable() -> bool {
            unreachable!()
        }
        fn enable_chain() {
            unreachable!()
        }
    }

    _impl_channel!(_0, _X, TCTRL0, LDVAL0, TFLG0, CVAL0);
    _impl_channel!(_1, _0, TCTRL1, LDVAL1, TFLG1, CVAL1);
    _impl_channel!(_2, _1, TCTRL2, LDVAL2, TFLG2, CVAL2);
    _impl_channel!(_3, _2, TCTRL3, LDVAL3, TFLG3, CVAL3);
}

/// A periodic interrupt timer (PIT)
pub struct PIT<Chan> {
    clock_hz: Frequency,
    divider: Divider,
    _chan: PhantomData<Chan>,
}

impl<Chan: channel::Channel> PIT<Chan> {
    fn new(clock_hz: Frequency, divider: Divider) -> PIT<Chan> {
        PIT {
            clock_hz,
            divider,
            _chan: PhantomData,
        }
    }

    #[inline(always)]
    fn disabled<F: FnMut(&Self) -> R, R>(&self, mut act: F) -> R {
        let enabled = Chan::enabled();
        Chan::set_enabled(false);
        let tsv = Chan::ldval();
        let res = act(self);
        self.ldval(tsv);
        Chan::set_enabled(enabled);
        res
    }

    fn ldval(&self, val: u32) {
        Chan::set_ldval(val);
    }

    fn tif(&self) -> bool {
        Chan::tif()
    }

    fn clear_tif(&self) {
        Chan::clear_tif();
    }

    /// Returns the period of the clock ticks. This is the inverse
    /// of the clock frequency
    pub fn clock_period(&self) -> core::time::Duration {
        (self.clock_hz / self.divider).into()
    }

    /// Measure the execution duration of `act` with this timer. Returns the duration
    /// of the action, or `None` if the timer expired before the action completed.
    ///
    /// `time` will measure the difference of counts in a 32 bit register. The counter
    /// changes every clock period. The clock accuracy is based on our ability to round
    /// integers. Consider choosing the input clock frequency and prescalars to define
    /// a clock that can accurately measure your workloads.
    ///
    /// The method will disable any interrupts that this timer has enabled. It will also
    /// reset the timer to execute this measurement.
    ///
    /// If you need a 64 bit timer, use the `chain` function to combine timer 0 and
    /// timer 1. The two can crate the 'lifetime' timer, which is capable of measuring
    /// larger intervals.
    pub fn time<F: FnMut() -> R, R>(&mut self, mut act: F) -> (R, Option<core::time::Duration>) {
        const STARTING_LDVAL: u32 = u32::max_value();
        self.with_interrupts_disabled(|this| {
            this.disabled(|this| {
                this.clear_tif();
                this.ldval(STARTING_LDVAL);
                Chan::set_enabled(true);
                let res = act();
                let counter = Chan::cval();
                if this.tif() {
                    // Action took too long and the timer expired.
                    // The counter value is meaningless
                    (res, None)
                } else {
                    let ticks = STARTING_LDVAL - counter;
                    let clock_period: core::time::Duration = (this.clock_hz / this.divider).into();
                    (res, Some(ticks * clock_period))
                }
            })
        })
    }

    /// Enable the timer to trigger an interrupt when the timer expires
    pub fn set_interrupt_enable(&mut self, interrupt: bool) {
        Chan::set_interrupt_enable(interrupt);
    }

    /// Returns `true` if the timer will trigger an interrupt when
    /// it expires.
    pub fn interrupt_enable(&self) -> bool {
        Chan::interrupt_enable()
    }

    #[inline(always)]
    fn with_interrupts_disabled<F: FnMut(&Self) -> R, R>(&self, mut act: F) -> R {
        let interrupt_enabled = self.interrupt_enable();
        Chan::set_interrupt_enable(false);
        let res = act(self);
        Chan::set_interrupt_enable(interrupt_enabled);
        res
    }
}

impl<Chan: channel::Channel> CountDown for PIT<Chan> {
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, ms: T) {
        let ticks: u32 = match ticks(ms.into(), self.clock_hz.0, self.divider.0) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => u32::max_value(),
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => 0,
        };
        Chan::set_enabled(false);
        self.clear_tif();
        self.ldval(ticks);
        Chan::set_enabled(true);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.tif() {
            self.clear_tif();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<Chan: channel::Channel> Periodic for PIT<Chan> {}

/// Two PIT timers chained together
pub struct ChainedPIT<C0, C1> {
    lower: PIT<C0>,
    upper: PIT<C1>,
}

impl<C0, C1> ChainedPIT<C0, C1>
where
    C1: channel::Channel,
{
    /// Control interrupt generation for this chained PIT timer
    pub fn set_interrupt_enable(&mut self, interrupt: bool) {
        self.upper.set_interrupt_enable(interrupt);
    }

    /// Returns `true` if interrupts are enabled, else `false`
    /// if interrupts are disabled.
    pub fn interrupt_enable(&self) -> bool {
        self.upper.interrupt_enable()
    }
}

/// Chain two timers together, returning a `ChainedPIT` timer that can
/// count twice as many ticks.
///
/// The API enforces that channel 1 is chained to channel 0, or channel 2 is
/// chained to channel 1, or channel 3 is chained to channel 2. Any other
/// combination of chaining is prevented by the compiler.
///
/// We do not support chaining more than two timers.
pub fn chain<C1: channel::Channel>(
    mut lower: PIT<<C1 as channel::Channel>::ChainedTo>,
    upper: PIT<C1>,
) -> ChainedPIT<<C1 as channel::Channel>::ChainedTo, C1> {
    // We can only enable the interrupt for the upper timer.
    // Otherwise, we'll interrupt early.
    lower.set_interrupt_enable(false);
    ChainedPIT { lower, upper }
}

impl<C0, C1> CountDown for ChainedPIT<C0, C1>
where
    C0: channel::Channel,
    C1: channel::Channel,
{
    type Time = core::time::Duration;
    fn start<T: Into<Self::Time>>(&mut self, time: T) {
        // clock_hz and divider are equal across all PITs
        let ticks: u64 = match ticks(time.into(), self.lower.clock_hz.0, self.lower.divider.0) {
            Ok(ticks) => ticks,
            // Saturate the load value
            Err(TicksError::TicksOverflow) | Err(TicksError::DurationOverflow) => u64::max_value(),
            // Ratio of freq / div was zero, or divider was zero
            Err(TicksError::DivideByZero) => 0,
        };
        C0::set_enabled(false);
        C1::set_enabled(false);

        self.upper.clear_tif();
        C1::enable_chain();
        self.upper.ldval((ticks >> 32) as u32);
        self.lower.ldval((ticks & 0xFFFF_FFFF) as u32);

        C0::set_enabled(true);
        C1::set_enabled(true);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.upper.tif() {
            self.upper.clear_tif();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// The lifetime timer is PIT0 chained to PIT1.
/// It allows us to time over 64 bits with no
/// carry.
impl ChainedPIT<channel::_0, channel::_1> {
    /// Time the execution duration of `act`. Returns the time it took to run `act`,
    /// or `None` if the timer expired.
    ///
    /// See the notes on `PIT::time`. Unlike `PIT::time`, this `time` method uses
    /// a 64 bit register, which can help measure larger intervals. As with `PIT::time`,
    /// this function will temporarily disable interrupts and reset any currently-running
    /// timer.
    ///
    /// This method is only available when chaining timer 0 to timer 1.
    pub fn time<F: FnMut() -> R, R>(&mut self, mut act: F) -> (R, Option<core::time::Duration>) {
        const STARTING_LDVAL: u32 = u32::max_value();
        self.upper.with_interrupts_disabled(|upper| {
            self.lower.disabled(|lower| {
                upper.disabled(|upper| {
                    upper.clear_tif();
                    upper.ldval(STARTING_LDVAL);
                    lower.ldval(STARTING_LDVAL);

                    use channel::Channel;

                    channel::_1::enable_chain();
                    channel::_1::set_enabled(true);
                    channel::_0::set_enabled(true);

                    let res = act();
                    let lifetime = unsafe {
                        let lifetime =
                            u64::from(ral::read_reg!(ral::pit, ral::pit::PIT, LTMR64H)) << 32;
                        lifetime | u64::from(ral::read_reg!(ral::pit, ral::pit::PIT, LTMR64L))
                    };
                    if upper.tif() {
                        (res, None)
                    } else {
                        let ticks = u64::max_value() - lifetime;
                        // Betting that this isn't lossy...
                        let clock_period: u64 =
                            core::time::Duration::from(upper.clock_hz / upper.divider).as_nanos()
                                as u64;
                        (
                            res,
                            Some(core::time::Duration::from_nanos(ticks * clock_period)),
                        )
                    }
                })
            })
        })
    }
}
