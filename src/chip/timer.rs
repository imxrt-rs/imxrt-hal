//! General timer APIs.
//!
//! `timer` provides adapters for implementing (non-)blocking
//! timers. It builds upon hardware drivers like [`gpt`](crate::gpt)
//! and [`pit`](crate::pit), and implements various `embedded-hal`
//! timer traits.
//!
//! Use [`Blocking`] to adapt a timer for simple, blocking delays. Blocking
//! delays use durations, not raw clock ticks; this means that you need to
//! specify the clock's frequency.
//!
//! Use [`CountDown`] to adapt a timer for periodic, non-blocking time
//! tracking. This also uses time duration, not raw clock ticks, for
//! timing, so it again needs the clock frequency. Use [`RawCountDown`]
//! if you want to track time in clock ticks.
//!
//! # Raw count representations
//!
//! The raw count, or ticks, supported by the driver influences the API.
//! The table below describes the raw bit width for each adapter.
//!
//! | Driver      | Bit width |
//! | ----------- | --------- |
//! | PIT channel | 32        |
//! | PIT chain   | 64        |
//! | GPT         | 32        |
//!
//! # Computing the clock frequency
//!
//! [`ccm` APIs](crate::ccm) make it easy to understand, at compile time, the intended
//! clock frequency for a given timer. Consider using these to simply compute
//! the clock frequency. See the [`Blocking`] and [`CountDown`] examples for
//! demonstrations.
//!
//! This package does not provide an implementation of `Cancel` from embedded-hal 0.2.
//! See [`RawCountDown`] documentation for more information.
//!
//! # Limitations of `CountDown`
//!
//! This module cannot provide the embedded-hal 0.2 `CountDown` implementation for all
//! possible drivers. You're encouraged to adapt [`RawCountDown`] to implement a
//! `CountDown` implementation for your specific driver.

use crate::{chip::pit, gpt};

/// An interface for hardware timers.
///
/// This is implemented for various GPT and PIT objects.
/// It is not convered by the semver guarantees of this
/// crate. Do not use.
#[doc(hidden)]
pub trait HardwareTimer {
    /// Representation of ticks.
    type Ticks;
    /// Indicates if the timer has elapsed.
    fn is_elapsed(&self) -> bool;
    /// Clears the elapsed flag.
    fn clear_elapsed(&mut self);
    /// Set the number of ticks that the timer counts.
    fn set_ticks(&mut self, ticks: Self::Ticks);
    /// Enable / disable the timer.
    fn set_enable(&mut self, enable: bool);
}

impl<const N: u8> HardwareTimer for pit::Pit<N> {
    type Ticks = u32;
    fn is_elapsed(&self) -> bool {
        pit::Pit::<N>::is_elapsed(self)
    }
    fn clear_elapsed(&mut self) {
        pit::Pit::<N>::clear_elapsed(self);
    }
    fn set_ticks(&mut self, ticks: Self::Ticks) {
        self.set_load_timer_value(ticks);
    }
    fn set_enable(&mut self, enable: bool) {
        if enable {
            self.enable();
        } else {
            self.disable();
        }
    }
}

impl<const L: u8, const R: u8> HardwareTimer for pit::Chained<L, R> {
    type Ticks = u64;
    fn is_elapsed(&self) -> bool {
        pit::Chained::<L, R>::is_elapsed(self)
    }
    fn clear_elapsed(&mut self) {
        pit::Chained::<L, R>::clear_elapsed(self);
    }
    fn set_ticks(&mut self, ticks: Self::Ticks) {
        self.set_load_timer_value(ticks);
    }
    fn set_enable(&mut self, enable: bool) {
        if enable {
            self.enable();
        } else {
            self.disable();
        }
    }
}

/// The GPT OCR used for timers implementations.
const GPT_OCR: gpt::OutputCompareRegister = gpt::OutputCompareRegister::OCR1;

impl<const N: u8> HardwareTimer for gpt::Gpt<N> {
    type Ticks = u32;
    fn is_elapsed(&self) -> bool {
        self.is_elapsed(GPT_OCR)
    }
    fn clear_elapsed(&mut self) {
        gpt::Gpt::<N>::clear_elapsed(self, GPT_OCR);
    }
    fn set_ticks(&mut self, ticks: Self::Ticks) {
        self.set_output_compare_count(GPT_OCR, ticks);
    }
    fn set_enable(&mut self, enable: bool) {
        if enable {
            self.enable();
        } else {
            self.disable();
        }
    }
}

/// Extensions for `fugit::TimerDuration`.
///
/// `fugit` does not provide a generic way to work with u32- and u64-backed durations.
/// This trait provides that generalization.
///
/// Do not use directly.
#[doc(hidden)]
pub trait TimerDurationExt {
    /// The duration representation.
    type Repr;
    /// Create yourself from raw ticks.
    fn from_ticks(ticks: Self::Repr) -> Self;
    /// Returns the ticks expressed by the duration.
    fn ticks(&self) -> Self::Repr;
    /// Convert into a different timer duration.
    fn convert<const OTHER_HZ: u32>(&self) -> fugit::TimerDuration<Self::Repr, OTHER_HZ>;
}

impl<const HZ: u32> TimerDurationExt for fugit::TimerDurationU32<HZ> {
    type Repr = u32;
    fn from_ticks(ticks: Self::Repr) -> Self {
        fugit::TimerDuration::<Self::Repr, HZ>::from_ticks(ticks)
    }
    fn ticks(&self) -> Self::Repr {
        fugit::TimerDuration::<Self::Repr, HZ>::ticks(self)
    }
    fn convert<const OTHER_HZ: u32>(&self) -> fugit::TimerDuration<Self::Repr, OTHER_HZ> {
        fugit::TimerDuration::<Self::Repr, HZ>::convert::<1, OTHER_HZ>(*self)
    }
}

impl<const HZ: u32> TimerDurationExt for fugit::TimerDurationU64<HZ> {
    type Repr = u64;
    fn from_ticks(ticks: Self::Repr) -> Self {
        fugit::TimerDuration::<Self::Repr, HZ>::from_ticks(ticks)
    }
    fn ticks(&self) -> Self::Repr {
        fugit::TimerDuration::<Self::Repr, HZ>::ticks(self)
    }
    fn convert<const OTHER_HZ: u32>(&self) -> fugit::TimerDuration<Self::Repr, OTHER_HZ> {
        fugit::TimerDuration::<Self::Repr, HZ>::convert::<1, OTHER_HZ>(*self)
    }
}

/// A blocking timer that runs at `HZ`.
///
/// Blocking occupies the CPU until the timer elapses.
/// You're responsible for specifying the clock frequency
/// `HZ`.
///
/// # Example
///
/// Create a blocking adapter over a single PIT channel. [`ccm` API](crate::ccm)
/// provide a simple way to specify the PIT clock frequency.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// use hal::{
///     ccm::{self, clock_gate, perclk_clk},
///     timer::BlockingPit,
/// };
///
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
///
/// // Before touching the PERCLK clock roots, turn off all downstream clock gates.
/// clock_gate::PERCLK_CLOCK_GATES.iter().for_each(|loc| loc.set(&mut ccm, clock_gate::OFF));
///
/// // Configure PERCLK to match this frequency:
/// const PERCLK_CLK_FREQUENCY_HZ: u32 = ccm::XTAL_OSCILLATOR_HZ / PERCLK_CLK_DIVIDER;
/// const PERCLK_CLK_DIVIDER: u32 = 24;
/// perclk_clk::set_selection(&mut ccm, perclk_clk::Selection::Oscillator);
/// perclk_clk::set_divider(&mut ccm, PERCLK_CLK_DIVIDER);
///
/// // Turn on the PIT clock gate.
/// clock_gate::pit().set(&mut ccm, clock_gate::ON);
///
/// // There's no other divider, so the PIT frequency is the root
/// // clock frequency.
/// const PIT_FREQUENCY_HZ: u32 = PERCLK_CLK_FREQUENCY_HZ;
///
/// let pit = unsafe { ral::pit::PIT::instance() };
/// let (pit0, _, _, _) = hal::pit::new(pit);
///
/// let mut blocking = BlockingPit::<0, PIT_FREQUENCY_HZ>::from_pit(pit0);
/// // Block for milliseconds:
/// blocking.block_ms(1000);
/// // Block for microseconds:
/// blocking.block_us(5000);
///
/// // Use fugit to express other durations.
/// use fugit::ExtU32;
/// blocking.block(1000.millis());
/// blocking.block(1000.micros());
///
/// // All blocking adapters play well with embedded-hal 0.2 interfaces.
/// use eh02::blocking::delay::{DelayMs, DelayUs};
/// blocking.delay_ms(1000u32);
/// blocking.delay_us(1000u32);
/// ```
pub struct Blocking<T, const HZ: u32> {
    timer: T,
}

impl<T, const HZ: u32> Blocking<T, HZ>
where
    T: HardwareTimer,
{
    /// Before calling this method, make sure that
    ///
    /// - the timer is disabled.
    /// - the elapsed flag is cleared.
    /// - interrupts are disabled.
    ///
    /// Also ensure that your timer can operate given the expected
    /// `block` implementation. For example, change the GPT reset
    /// on enable configuration so that the timer always starts at
    /// its newly-loaded value.
    fn new(timer: T) -> Self {
        Self { timer }
    }

    /// Release the underlying timer.
    ///
    /// The released timer's state is unspecified.
    pub fn release(self) -> T {
        self.timer
    }

    /// Occupy the CPU, blocking execution, for a `duration` represented
    /// by the target clock.
    ///
    /// See the [struct-level documentation](crate::timer::Blocking) for an example.
    /// Prefer this API if you would like to catch overflow issues at compile time,
    /// as demonstrated below.
    ///
    /// ```compile_fail
    /// // See struct-level documentation for configuration...
    /// # let pit0 = unsafe { imxrt_hal::pit::Pit::<0>::new(&imxrt_ral::pit::PIT::instance()) };
    /// # let mut blocking = imxrt_hal::timer::BlockingPit::<0, PIT_FREQUENCY_HZ>::from_pit(pit0);
    /// # const PIT_FREQUENCY_HZ: u32 = 75000000;
    /// // 99 seconds, expressed in microseconds, cannot fit within a u32 counter
    /// // that counts at PIT_FREQUENCY_HZ. This fails to compile:
    /// const DELAY: fugit::TimerDurationU32<PIT_FREQUENCY_HZ>
    ///     = fugit::MicrosDurationU32::from_ticks(99_000_000).convert();
    /// blocking.block(DELAY);
    /// ```
    ///
    /// ```no_run
    /// # let pit0 = unsafe { imxrt_hal::pit::Pit::<0>::new(&imxrt_ral::pit::PIT::instance()) };
    /// # let mut blocking = imxrt_hal::timer::BlockingPit::<0, PIT_FREQUENCY_HZ>::from_pit(pit0);
    /// # const PIT_FREQUENCY_HZ: u32 = 75000000;
    /// // However, 99 milliseconds, expressed in microseconds, can fit within a u32
    /// // counter that counts at PIT_FREQENCY_HZ.
    /// const DELAY: fugit::TimerDurationU32<PIT_FREQUENCY_HZ>
    ///     = fugit::MicrosDurationU32::from_ticks(99_000).convert();
    /// blocking.block(DELAY);
    /// ```
    pub fn block(&mut self, duration: fugit::TimerDuration<T::Ticks, HZ>)
    where
        fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
    {
        self.timer.set_ticks(duration.ticks());
        self.timer.set_enable(true);

        while !self.timer.is_elapsed() {}

        self.timer.clear_elapsed();
        self.timer.set_enable(false);
    }

    /// Occupy the CPU, blocking execution, for `ms` milliseconds.
    ///
    /// Note that the type of `ms` depends on the tick representation
    /// of your underlying timer. See the [module documentation](crate::timer)
    /// for specifics.
    ///
    /// # Panics
    ///
    /// Panics if the tick representation in `ms` on the target clock
    /// would overflow.
    ///
    /// # Example
    ///
    /// See the top-level example for a demonstration. Despite the complex
    /// type signature, note that the duration is simply a u32 or u64.
    pub fn block_ms(&mut self, ms: T::Ticks)
    where
        fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
        fugit::TimerDuration<T::Ticks, 1_000>: TimerDurationExt<Repr = T::Ticks>,
    {
        self.block(fugit::TimerDuration::<T::Ticks, 1_000>::from_ticks(ms).convert())
    }

    /// Occupy the CPU, blocking execution, for `us` milliseconds.
    ///
    /// Note that the type of `us` depends on the tick representation
    /// of your underlying timer. See the [module documentation](crate::timer)
    /// for specifics.
    ///
    /// # Panics
    ///
    /// Panics if the tick representation in `us` on the target clock
    /// would overflow.
    ///
    /// # Example
    ///
    /// See the top-level example for a demonstration. Despite the complex
    /// type signature, note that the duration is simply a u32 or u64.
    pub fn block_us(&mut self, us: T::Ticks)
    where
        fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
        fugit::TimerDuration<T::Ticks, 1_000_000>: TimerDurationExt<Repr = T::Ticks>,
    {
        self.block(fugit::TimerDuration::<T::Ticks, 1_000_000>::from_ticks(us).convert())
    }
}

/// Prepares a PIT channel to be adapted by blocking / count down
/// adapters.
fn prepare_pit<const N: u8>(pit: &mut pit::Pit<N>) {
    pit.disable();
    pit.clear_elapsed();
    pit.set_chained(false);
    pit.set_interrupt_enable(false);
}

/// Prepares a PIT chain to be adapted by blocking / count down
/// adapters.
fn prepare_pit_chained<const L: u8, const R: u8>(chain: &mut pit::Chained<L, R>) {
    chain.disable();
    chain.clear_elapsed();
    chain.set_interrupt_enable(false);
}

/// Prepares a GPT to be adapted by blocking / count down adapters.
fn prepare_gpt<const N: u8>(gpt: &mut gpt::Gpt<N>) {
    gpt.disable();
    gpt.clear_rollover();
    gpt.set_rollover_interrupt_enable(false);

    // We're using OCR1 so we can achieve the periodic
    // implementation by the hardware behavior.
    gpt.set_mode(gpt::Mode::Restart);
    // Start counting from zero when the timer is enabled.
    gpt.set_reset_on_enable(true);

    use gpt::OutputCompareRegister::*;
    for ocr in [OCR1, OCR2, OCR3] {
        gpt::Gpt::<N>::clear_elapsed(gpt, ocr);
        gpt.set_output_interrupt_on_compare(ocr, false);
    }
}

/// A single PIT channel that acts as a blocking timer.
pub type BlockingPit<const N: u8, const HZ: u32> = Blocking<pit::Pit<N>, HZ>;

impl<const N: u8, const HZ: u32> BlockingPit<N, HZ> {
    /// Create a blocking adapter from a PIT channel.
    pub fn from_pit(mut pit: pit::Pit<N>) -> Self {
        prepare_pit(&mut pit);
        Self::new(pit)
    }
}

/// A chain of PIT channels that act as a blocking timer.
pub type BlockingPitChain<const L: u8, const R: u8, const HZ: u32> =
    Blocking<pit::Chained<L, R>, HZ>;

impl<const L: u8, const R: u8, const HZ: u32> BlockingPitChain<L, R, HZ> {
    /// Create a blocking adapter from chained PIT channels.
    pub fn from_pit_chained(mut chain: pit::Chained<L, R>) -> Self {
        prepare_pit_chained(&mut chain);
        Self::new(chain)
    }
}

/// A GPT that acts as a blocking timer.
pub type BlockingGpt<const N: u8, const HZ: u32> = Blocking<gpt::Gpt<N>, HZ>;

impl<const N: u8, const HZ: u32> BlockingGpt<N, HZ> {
    /// Create a blocking adapter from a GPT.
    pub fn from_gpt(mut gpt: gpt::Gpt<N>) -> Self {
        prepare_gpt(&mut gpt);
        Self::new(gpt)
    }
}

impl<R, T, const HZ: u32> eh02::blocking::delay::DelayMs<R> for Blocking<T, HZ>
where
    R: Into<T::Ticks>,
    T: HardwareTimer,
    fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
    fugit::TimerDuration<T::Ticks, 1_000>: TimerDurationExt<Repr = T::Ticks>,
{
    fn delay_ms(&mut self, ms: R) {
        self.block_ms(ms.into());
    }
}

impl<R, T, const HZ: u32> eh02::blocking::delay::DelayUs<R> for Blocking<T, HZ>
where
    R: Into<T::Ticks>,
    T: HardwareTimer,
    fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
    fugit::TimerDuration<T::Ticks, 1_000_000>: TimerDurationExt<Repr = T::Ticks>,
{
    fn delay_us(&mut self, us: R) {
        self.block_us(us.into());
    }
}

/// A count down timer that uses ticks for the timeout.
///
/// This adapter does not require you to know about the
/// clock frequency. However, not knowing this means that
/// you're responsible for choosing meaningful count ticks.
///
/// The adapter implements `Periodic`. However, it does not
/// implement `Cancel`, since it cannot decide the error type
/// for all possible users. You're encouraged to build your
/// own adapter atop this type if you need to provide a `Cancel`
/// implementation.
///
/// See [`CountDown`] for an example of using this type with
/// a GPT timer.
pub struct RawCountDown<T> {
    timer: T,
}

impl<T> RawCountDown<T>
where
    T: HardwareTimer,
{
    /// Before calling this method, make sure that the timer
    ///
    /// - is disabled.
    /// - has interrupts disabled.
    /// - is configured for periodic execution.
    fn new(timer: T) -> Self {
        Self { timer }
    }

    /// Release the adapter to acquire the raw count down timer.
    ///
    /// The released timer's state is unspecified.
    pub fn release(self) -> T {
        self.timer
    }

    /// Start the count down timer to periodically elapse every
    /// number of `ticks` clock counts.
    ///
    /// If this is invoked when a timer is already counting,
    /// this resets the timer to run at `ticks`.
    ///
    /// The type of `ticks` depends on the underlying timer.
    /// See the [module documentation](crate::timer) for specifics.
    pub fn start(&mut self, ticks: T::Ticks) {
        self.timer.set_enable(false);
        self.timer.clear_elapsed();
        self.timer.set_ticks(ticks);
        self.timer.set_enable(true);
    }

    /// Cancel a running timer.
    ///
    /// Does nothing if the timer is already canceled / disabled.
    pub fn cancel(&mut self) {
        self.timer.set_enable(false);
    }

    /// Indicates if the timer has elapsed.
    pub fn is_elapsed(&self) -> bool {
        self.timer.is_elapsed()
    }

    /// Clears the elapsed condition.
    pub fn clear_elapsed(&mut self) {
        self.timer.clear_elapsed()
    }
}

/// A count down timer over a PIT channel.
pub type RawCountDownPit<const N: u8> = RawCountDown<pit::Pit<N>>;

impl<const N: u8> RawCountDownPit<N> {
    /// Create a count down timer from a PIT channel.
    pub fn from_pit(mut pit: pit::Pit<N>) -> Self {
        prepare_pit(&mut pit);
        Self::new(pit)
    }
}

/// A count down timer over two chained PIT channels.
pub type RawCountDownPitChain<const L: u8, const R: u8> = RawCountDown<pit::Chained<L, R>>;

impl<const L: u8, const R: u8> RawCountDownPitChain<L, R> {
    /// Create a count down timer from a PIT chain.
    pub fn from_pit_chained(mut chain: pit::Chained<L, R>) -> Self {
        prepare_pit_chained(&mut chain);
        Self::new(chain)
    }
}

/// A count down timer over a GPT.
pub type RawCountDownGpt<const N: u8> = RawCountDown<gpt::Gpt<N>>;

impl<const N: u8> RawCountDownGpt<N> {
    /// Create a count down timer from a GPT.
    pub fn from_gpt(mut gpt: gpt::Gpt<N>) -> Self {
        prepare_gpt(&mut gpt);
        Self::new(gpt)
    }
}

impl<T> eh02::timer::CountDown for RawCountDown<T>
where
    T: HardwareTimer,
{
    type Time = T::Ticks;
    fn start<C>(&mut self, count: C)
    where
        C: Into<Self::Time>,
    {
        RawCountDown::<T>::start(self, count.into());
    }
    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.is_elapsed() {
            self.clear_elapsed();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<T> eh02::timer::Periodic for RawCountDown<T> {}

/// A count down timer adapter that uses `fugit` durations.
///
/// You're responsible for specifying the `HZ` that represents
/// your underlying clock's frequency. However, it conveniently
/// lets you express timeouts in units of time, not clock ticks.
///
/// To use this type, create a [`RawCountDown`], then simply wrap
/// that object with this adapter. You may also adapt this object
/// to satisfy your driver's need.
///
/// The `CountDown` embedded-hal implementation provided by this
/// crate may not work for your specific use case. If that's the
/// case, you may adapt the [`RawCountDown`] object to satisfy
/// your needs.
///
/// # Example
///
/// Use the GPT as a countdown timer. [`ccm` APIs](crate::ccm) make
/// it easy to configure the root clock. Additional constants ensure
/// that the run-time and compile-time frequencies match.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// use hal::ccm::{self, clock_gate, perclk_clk};
///
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
///
/// // Before touching the PERCLK clock roots, turn off all downstream clock gates.
/// clock_gate::PERCLK_CLOCK_GATES.iter().for_each(|loc| loc.set(&mut ccm, clock_gate::OFF));
///
/// // Configure PERCLK to match this frequency:
/// const PERCLK_CLK_FREQUENCY_HZ: u32 = ccm::XTAL_OSCILLATOR_HZ / PERCLK_CLK_DIVIDER;
/// const PERCLK_CLK_DIVIDER: u32 = 24;
/// perclk_clk::set_selection(&mut ccm, perclk_clk::Selection::Oscillator);
/// perclk_clk::set_divider(&mut ccm, PERCLK_CLK_DIVIDER);
///
/// // Enable the clock gate for our GPT.
/// clock_gate::gpt_bus::<1>().set(&mut ccm, clock_gate::ON);
/// clock_gate::gpt_serial::<1>().set(&mut ccm, clock_gate::ON);
///
/// // GPT1 counts with this frequency:
/// const GPT1_FREQUENCY_HZ: u32 = PERCLK_CLK_FREQUENCY_HZ / GPT1_DIVIDER;
/// const GPT1_DIVIDER: u32 = 100;
/// const GPT1_CLOCK_SOURCE: hal::gpt::ClockSource = hal::gpt::ClockSource::HighFrequencyReferenceClock;
///
/// let gpt1 = unsafe { ral::gpt::GPT1::instance() };
/// let mut gpt1 = hal::gpt::Gpt::new(gpt1);
/// gpt1.set_divider(GPT1_DIVIDER);
/// gpt1.set_clock_source(GPT1_CLOCK_SOURCE);
///
/// let mut count_down = hal::timer::CountDown::<_, GPT1_FREQUENCY_HZ>::new(
///     hal::timer::RawCountDown::from_gpt(gpt1)
/// );
///
/// use fugit::ExtU32;
/// use eh02::timer::CountDown;
/// CountDown::start(&mut count_down, 100.millis());
/// ```
pub struct CountDown<T, const HZ: u32> {
    timer: RawCountDown<T>,
}

impl<T, const HZ: u32> CountDown<T, HZ>
where
    T: HardwareTimer,
{
    /// Create a new count down timer that works with timer units.
    pub fn new(raw: RawCountDown<T>) -> Self {
        Self { timer: raw }
    }

    /// Release the adapter to acquire the raw count down timer.
    pub fn release(self) -> RawCountDown<T> {
        self.timer
    }

    /// Start the timer to periodically elapse every `duration`.
    ///
    /// If this is invoked when a timer is already counting,
    /// this resets the timer to run at `ticks`.
    pub fn start(&mut self, duration: fugit::TimerDuration<T::Ticks, HZ>)
    where
        fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
    {
        self.timer.start(duration.ticks());
    }

    /// Cancel a running timer.
    ///
    /// Does nothing if the timer is already canceled / disabled.
    pub fn cancel(&mut self) {
        self.timer.cancel();
    }

    /// Indicates if the timer has elapsed.
    pub fn is_elapsed(&self) -> bool {
        self.timer.is_elapsed()
    }

    /// Clears the elapsed condition.
    pub fn clear_elapsed(&mut self) {
        self.timer.clear_elapsed()
    }
}

impl<T, const HZ: u32> eh02::timer::CountDown for CountDown<T, HZ>
where
    T: HardwareTimer,
    fugit::TimerDuration<T::Ticks, HZ>: TimerDurationExt<Repr = T::Ticks>,
{
    type Time = fugit::TimerDuration<T::Ticks, HZ>;
    fn start<C>(&mut self, count: C)
    where
        C: Into<Self::Time>,
    {
        let duration = count.into();
        self.start(duration);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        self.timer.wait()
    }
}

impl<T, const HZ: u32> eh02::timer::Periodic for CountDown<T, HZ> {}
