//! General Purpose Timer (GPT)
//!
//! # Features
//!
//! The GPTs are count-up, wrapping timers that run off of the IPG clock or the
//! crystal oscillator (24MHz). Each GPT has three compare registers,
//! called **output comparison registers (OCR)**. When the counter reaches a
//! value in an OCR, the GPT signals the comparison through status flags.
//! A comparison can generate an interrupt.
//!
//! Use GPTs to
//!
//! - detect if a timing interval has elapsed
//! - trigger an interrupt when an interval elapses
//! - measure code execution
//!
//! # GPT modes
//!
//! The table below summarizes the effects of an output comparison operation
//! when in restart mode, and when in free-running mode. See the subsequent
//! sections for a discussion of the two modes.
//!
//! | GPT Mode     | OCR1                                      | OCR2                                      | OCR3         |
//! | ------------ | ----------------------------------------- | ----------------------------------------- | ------------ |
//! | Restart      | Resets the counter to zero                | No effect; counter continues incrementing | No effect... |
//! | Free-running | No effect; counter continues incrementing | No effect ...                             | No effect... |
//!
//! In summary, **OCR1 is special in restart mode**, as it will reset the value in the GPT counter.
//!
//! Select a mode with [`set_mode()`](struct.GPT.html#method.set_mode).
//!
//! ## Restart mode
//!
//! The GPTs default to 'restart mode.' In restart mode, a compare on **channel
//! 1** will reset the GPT counter to zero. Compare events on channels 2 and 3
//! will not reset the GPT counter. If you would rather have the GPT counter continue
//! no matter the comparison event, set the GPT to free-running mode.
//!
//! ## Free-running mode
//!
//! The GPTs may be in free-running mode. When a comparion event occurs in free-running
//! mode, the counter continues to increment, eventually wrapping around. Free-running
//! mode treats all channels as equal; that is, channel 1 is no different than channel
//! 2 or 3.
//!
//! # Reset on enable
//!
//! Reset on enable is a complementary feature to the two modes. When reset on enable
//! is active, the GPT counter will reset to zero each time the timer is enabled. By default,
//! the counter will restart at whatever value is currently in the counter. The default
//! behavior lets a user 'pause' the counter by disabling the GPT. On the other hand,
//! reset on enable lets users reset the counter by disabling and re-enabling the GPT.
//!
//! The table below summarizes the 'reset on enable' behaviors. Use
//! [`set_reset_on_enable()`](struct.GPT.html#method.set_reset_on_enable) to configure
//! the reset on enable behavior.
//!
//! | State   | Behavior                                                 |
//! | ------- | -------------------------------------------------------- |
//! | `false` | When the GPT is disabled, it maintains its counter value |
//! | `true`  | When the GPT is disabled, the counter resets to zero     |
//!
//! # GPTs and system WAIT / STOP
//!
//! By default, GPTs do not run when the process is in in wait mode. Use
//! [`set_wait_mode_enable(true)`](struct.GPT.html#method.set_wait_mode_enable)
//! to enable GPTs in wait mode.
//!
//! If the GPT stops counting in WAIT / STOP system states, the counter freezes its
//! counter. When the processor transitions into RUNNING, the counter increments from
//! its previously-frozen value (provided the GPT was enabled).
//!
//! # `embedded_hal` implementations
//!
//! The module provides adapters that pair a GPT with an OCR. The
//! adapters implement the `embedded_hal` timers using the GPT and the OCR
//! Users must ensure that the selected mode is suitable for the behaviors of the adapters.
//!
//! The two adapters include
//!
//! - [`CountDown`](struct.CountDown.html), which implements the `CountDown` trait
//! - [`Periodic`](struct.Periodic.html), which implements the `Periodic` trait
//!
//! Although the adapters work on a single OCR, the timer may continue to monitor the
//! other two OCRs.
//!
//! # Example
//!
//! ```no_run
//! use imxrt1062_hal;
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
//! let mut gpt1 = peripherals.gpt1.clock(&mut cfg);
//!
//! gpt1.set_output_interrupt_on_compare(
//!     imxrt1062_hal::gpt::OutputCompareRegister::Three,
//!     true,
//! );
//! gpt1.set_wait_mode_enable(true);
//! gpt1.set_mode(imxrt1062_hal::gpt::Mode::FreeRunning);
//!
//! gpt1.set_output_compare_duration(
//!     imxrt1062_hal::gpt::OutputCompareRegister::Three,
//!     core::time::Duration::from_micros(765),
//! );
//! ```
//!
//! # TODO
//!
//! - Input capture. Each GPT can capture the value of the counter
//!   when a pin state changes. When the pin state changes, the
//!   GPT can generate an interrupt.
//! - Output generation. When one of the three comparison registers
//!   match the counter, the GPT can generate a signal on an output
//!   pin.

use crate::{
    ccm::{perclk, ticks},
    ral,
};

use core::time::Duration;

/// An unclocked GPT
///
/// Each GPT starts in an unclocked state. By supplying
/// a proper clock configuration, the GPT will clock
/// itself and prepare for operation.
pub struct Unclocked {
    registers: ral::gpt::Instance,
    instance: Instance,
}

/// GPT instance
///
/// Used for runtime selection of GPT-specific configurations
enum Instance {
    One,
    Two,
}

/// Prescaler applied to each GPT's input clock.
///
/// See comments at usage for justification.
const DEFAULT_PRESCALER: u32 = 2;

/// A general purpose timer
///
/// The timers support three output compare registers. When a compare register
/// matches the value of the counter, the GPT may trigger an interrupt.
///
/// By default, the timer runs in wait mode.
pub struct GPT {
    /// Registers for this GPT instance
    registers: ral::gpt::Instance,
    /// The effective clock frequency that controls
    /// the counter.
    ///
    /// The value accounts for all prescalers and dividers.
    clock_hz: u32,
}

impl Unclocked {
    /// Create an unclocked GPT1
    pub(crate) fn one(registers: ral::gpt::Instance) -> Self {
        Unclocked {
            registers,
            instance: Instance::One,
        }
    }

    /// Create an unclocked GPT2
    pub(crate) fn two(registers: ral::gpt::Instance) -> Self {
        Unclocked {
            registers,
            instance: Instance::Two,
        }
    }

    /// Enable the clocks to the GPT, returning a GPT timer
    ///
    /// `configured` is a handle describing the clocks and clock
    /// configuration for the GPT. See the `ccm` module for more
    /// information.
    pub fn clock(self, configured: &mut perclk::Configured) -> GPT {
        let (freq, div) = match self.instance {
            Instance::One => configured.enable_gpt1_clock_gates(),
            Instance::Two => configured.enable_gpt2_clock_gates(),
        };

        match configured.clock_selection() {
            perclk::CLKSEL::OSC => {
                ral::write_reg!(
                    ral::gpt,
                    self.registers,
                    CR,
                    EN_24M: 1, // Enable crystal oscillator
                    CLKSRC: 0b101 // Crystal Oscillator
                );
                // The 24MHz prescaler register can't be non-zero. Not sure why.
                // The reference manual says its OK, but it doesn't work. The
                // se4L project noted the same issue in their kernel.
                // So, this means that there's a divider of 2 when using the
                // crystal oscillator.
                ral::write_reg!(ral::gpt, self.registers, PR, PRESCALER24M: (DEFAULT_PRESCALER - 1));
            }
            perclk::CLKSEL::IPG(_) => {
                ral::write_reg!(
                    ral::gpt,
                    self.registers,
                    CR,
                    EN_24M: 0, // No crystal oscillator
                    CLKSRC: 0b001 // Peripheral Clock
                );
                // See the above comment about needing a prescaler for the other
                // clock. This is for consistency, so that we can implement the
                // "divide by two" behavior.
                ral::write_reg!(ral::gpt, self.registers, PR, PRESCALER: (DEFAULT_PRESCALER - 1));
            }
        }

        // Clear all statuses
        ral::write_reg!(ral::gpt, self.registers, SR, 0b11_1111);

        GPT {
            registers: self.registers,
            clock_hz: (freq / div).0 / DEFAULT_PRESCALER,
        }
    }
}

/// An output compare register (OCR)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputCompareRegister {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Possible modes of the GPT
pub enum Mode {
    /// Reset mode
    ///
    /// A comparions event on channel 1 will reset the GPT counter.
    /// Comparison events on channels 2 and 3 do not reset the counter.
    Reset,
    /// Free running mode
    ///
    /// Comparisons on channel 1 are treated like comparions on channels
    /// 2 and 3. The counter continues to increment on comparison.
    FreeRunning,
}

impl GPT {
    /// Returns the current mode of the GPT
    pub fn mode(&self) -> Mode {
        if ral::read_reg!(ral::gpt, self.registers, CR, FRR == 0) {
            Mode::Reset
        } else {
            Mode::FreeRunning
        }
    }

    /// Set the GPT mode
    ///
    /// Refer to the module level documentation for more information on the GPT modes.
    pub fn set_mode(&mut self, mode: Mode) {
        ral::modify_reg!(ral::gpt, self.registers, CR, FRR: (mode as u32))
    }

    /// Set the reset on enable behavior
    ///
    /// See the module level docs for more information.
    pub fn set_reset_on_enable(&mut self, reset_on_enable: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, ENMOD: (reset_on_enable as u32));
    }

    /// Returns `true` if the GPT counter will reset the next time it is enabled
    pub fn reset_on_enable(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, ENMOD == 1)
    }

    /// Enable or disable the GPT
    ///
    /// When enabled, the counter starts counting. When disabled, the counter will
    /// stop counting.
    pub fn set_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, EN: (enable as u32));
    }

    /// Indicates if the GPT is enabled (`true`) or disabled (`false`).
    pub fn enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, EN == 1)
    }

    /// Allow the GPT to run in wait mode; or, prevent the GPT from running
    /// in wait mode.
    pub fn set_wait_mode_enable(&mut self, wait: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, WAITEN: (wait as u32));
    }

    /// Indicates if the GPT runs while in wait mode
    pub fn wait_mode_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, WAITEN == 1)
    }

    /// Enable the GPT interrupt when the output compares
    pub fn set_output_interrupt_on_compare(&mut self, output: OutputCompareRegister, intr: bool) {
        let ir: u32 = ral::read_reg!(ral::gpt, self.registers, IR);
        let ir: u32 = if intr {
            ir | (1 << (output as u32))
        } else {
            ir & !(1 << (output as u32))
        };
        ral::write_reg!(ral::gpt, self.registers, IR, ir);
    }

    /// Returns `true` if a comparison triggers an interrupt
    pub fn output_interrupt_on_compare(&self, output: OutputCompareRegister) -> bool {
        ral::read_reg!(ral::gpt, self.registers, IR) & (1 << (output as u32)) != 0
    }

    /// Returns the current count of the GPT
    pub fn count(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.registers, CNT)
    }

    /// Set an output compare register to trigger on the next `count` value of the
    /// counter.
    pub fn set_output_compare_count(&mut self, output: OutputCompareRegister, count: u32) {
        match output {
            OutputCompareRegister::One => ral::write_reg!(ral::gpt, self.registers, OCR1, count),
            OutputCompareRegister::Two => ral::write_reg!(ral::gpt, self.registers, OCR2, count),
            OutputCompareRegister::Three => ral::write_reg!(ral::gpt, self.registers, OCR3, count),
        }
    }

    /// Returns the current output compare count for the specified register
    pub fn output_compare_count(&self, output: OutputCompareRegister) -> u32 {
        match output {
            OutputCompareRegister::One => ral::read_reg!(ral::gpt, self.registers, OCR1),
            OutputCompareRegister::Two => ral::read_reg!(ral::gpt, self.registers, OCR2),
            OutputCompareRegister::Three => ral::read_reg!(ral::gpt, self.registers, OCR3),
        }
    }

    /// Set an output compare register to trigger when the specified duration elapses
    ///
    /// This is a convenience for an operation that resembles
    ///
    /// 1. compute the number of counts represented in the duration, based on the clock frequency
    /// 2. acquire the current GPT count
    /// 3. add the number of counts from 1 to the count of 2, accounting for wrap-around
    /// 4. set the value for the OCR
    pub fn set_output_compare_duration(
        &mut self,
        output: OutputCompareRegister,
        duration: Duration,
    ) {
        let counts: u32 = ticks(duration, self.clock_hz, 1).unwrap_or(u32::max_value());
        let next_count = self.count().wrapping_add(counts);
        self.set_output_compare_count(output, next_count);
    }

    /// Returns a handle that can query and modify the output compare status for the provided output
    pub fn output_compare_status(&mut self, output: OutputCompareRegister) -> OutputCompareStatus {
        OutputCompareStatus { gpt: self, output }
    }

    /// Returns the clock period as a duration
    ///
    /// This represents the resolution of the clock. The maximum measurement
    /// interval is `clock_period() * u32::max_value()`.
    pub fn clock_period(&self) -> Duration {
        Duration::from_nanos((1_000_000_000u32 / self.clock_hz).into())
    }

    /// Measure the execution time of an action `act` using the GPT. Returns
    /// the result of the action, along with how long it took to execute the
    /// action.
    ///
    /// User must ensure that the timer is enabled. Otherwise, the resulting
    /// duration is zero.
    ///
    /// User must ensure that the action takes less time than it takes for the
    /// timer to wrap around. We cannot distinguish a wrapped-around counter
    /// from an incrementing counter with this implementation. Consider using
    /// `time_no_overflow` if you need a better guarantee.
    pub fn time<F: FnOnce() -> R, R>(&self, act: F) -> (R, Duration) {
        let start = self.count();
        let result = act();
        let end = self.count();
        let counts = end.wrapping_sub(start);
        (result, counts * self.clock_period())
    }

    /// Time an operation, returning the result, and the amount of time the
    /// operation took.
    ///
    /// Unlike `time()`, `time_no_overflow()` uses an output compare register to check if the
    /// counter overflowed. It requires a mutable GPT, since we need to
    /// disable the timer and modify compare registers to properly measure the interval.
    /// If the timer wrapped around, the return is `None`.
    ///
    /// When `time_no_overflow()` returns,
    ///
    /// - the GPT is disabled
    /// - the interrupt on compare for the output is disabled
    /// - the compare value in the specified output compare register is undefined
    ///
    /// Users are responsible for resetting the state of the GPT as needed.
    ///
    /// Users are responsible for enabling the GPT before using `time_no_overflow()`.
    /// Otherwise, the returned duration is non-`None`, but zero.
    ///
    /// If you know that you're measuring short intervals that will not overflow the
    /// counter, consider using `time()`.
    pub fn time_no_overflow<F: FnOnce() -> R, R>(
        &mut self,
        output: OutputCompareRegister,
        act: F,
    ) -> (R, Option<Duration>) {
        self.set_enable(false);
        self.set_output_interrupt_on_compare(output, false);

        let start = self.count();
        // Set the compare to trigger if the counter elapses the wrapped start time.
        // This is how we know that the timer overflowed.
        self.set_output_compare_count(output, start.wrapping_sub(1));
        self.set_enable(true);
        let result = act();
        // We can read the count faster than we can disable the peripheral.
        let end = self.count();
        self.set_enable(false);

        let mut status = self.output_compare_status(output);
        if status.is_set() {
            status.clear();
            // Compare triggered, so we don't actually know how long this
            // took.
            (result, None)
        } else {
            let counts = end.wrapping_sub(start);
            (result, Some(counts * self.clock_period()))
        }
    }

    /// Returns an adapter that implements the count down trait
    ///
    /// Assumes that the timer is already enabled. Otherwise, the
    /// count down adapter will block forever. The adapter will never
    /// disable the counter, so the borrowed GPT may still track other
    /// times while it is borrowed.
    ///
    /// The adapter assumes the current GPT's mode. User is responsible
    /// for making sure that this mode is sensible for the qualities of
    /// the timer.
    pub fn count_down(&mut self, output: OutputCompareRegister) -> CountDown {
        CountDown(Timer::oneshot(self, output))
    }

    /// Returns an adapter that implements the periodic trait
    ///
    /// Assumes that the timer is already enabled. Otherwise, the
    /// periodic adapter will block forever. The adapter will never
    /// disable the counter. so the borrowed GPT may still track other
    /// times while it is borrowed.
    ///
    /// The adapter assumes the current GPT's mode. User is responsible for
    /// making sure this mode is sensible for the qualities of the timer.
    pub fn periodic(&mut self, output: OutputCompareRegister) -> Periodic {
        Periodic(Timer::periodic(self, output))
    }

    /// Enable / disable an interrupt when the GPT counter rolls over from `u32::max_value()` to
    /// `0`
    ///
    /// The GPT triggers a rollover regardless of the GPT mode.
    pub fn set_rollover_interrupt(&mut self, rov: bool) {
        ral::modify_reg!(ral::gpt, self.registers, IR, ROVIE: (rov as u32));
    }

    /// Returns `true` if a rollover generates an interrupt
    pub fn rollover_interrupt(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, IR, ROVIE == 1)
    }

    /// Returns `true` if the rollover flag is set
    ///
    /// A rollover occurs when the counter rolls over from `u32::max_value()` to `0`. Rollover
    /// may occur regardless of the GPT mode.
    pub fn rollover(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, SR, ROV == 1)
    }

    /// Clear the rollover status flag
    ///
    /// Users must clear the rollover flag if a rollover triggered an interrupt.
    pub fn clear_rollover(&self) {
        ral::write_reg!(ral::gpt, self.registers, SR, ROV: 1);
    }
}

/// A handle to evaluate and modify the output compare status
pub struct OutputCompareStatus<'a> {
    gpt: &'a mut GPT,
    output: OutputCompareRegister,
}

impl<'a> OutputCompareStatus<'a> {
    /// Returns true if this output compare has triggered
    pub fn is_set(&self) -> bool {
        let sr = ral::read_reg!(ral::gpt, self.gpt.registers, SR);
        sr & (1 << (self.output as u32)) != 0
    }

    /// Clear the output compare status flag
    ///
    /// It's necessary to clear the flag when the comparison has triggered
    /// an interrupt.
    pub fn clear(&mut self) {
        ral::write_reg!(ral::gpt, self.gpt.registers, SR, 1 << (self.output as u32));
    }
}

/// How the timer should behave
#[derive(PartialEq, Eq)]
enum Policy {
    /// Don't repeat
    Oneshot,
    /// Repeat with the specified duration
    Periodic(Option<Duration>),
}

/// Shared implementation for `embedded_hal` traits
struct Timer<'a> {
    gpt: &'a mut GPT,
    output: OutputCompareRegister,
    policy: Policy,
}

impl<'a> Timer<'a> {
    /// Constructs a timer that implements oneshot behavior (not periodic)
    fn oneshot(gpt: &'a mut GPT, output: OutputCompareRegister) -> Self {
        Timer {
            gpt,
            output,
            policy: Policy::Oneshot,
        }
    }

    /// Constructs a timer that implements periodic behavior
    fn periodic(gpt: &'a mut GPT, output: OutputCompareRegister) -> Self {
        Timer {
            gpt,
            output,
            policy: Policy::Periodic(None),
        }
    }

    /// Start the timer
    ///
    /// Assumes that the GPT is enabled.
    fn start(&mut self, duration: Duration) {
        self.gpt.set_output_compare_duration(self.output, duration);
        if let Policy::Periodic(ref mut period) = &mut self.policy {
            *period = Some(duration);
        }
    }

    /// Returns `Ok(())` when the timer has elapsed; otherwise, returns `Err(nb::WouldBlock)`.
    ///
    /// If the policy is periodic, we will restart the timer. If the user already observed an elapsed
    /// timer, but calls `wait()` again, the behavior is unspecified (as stated by the trait contract).
    fn wait(&mut self) -> nb::Result<(), void::Void> {
        let mut status = self.gpt.output_compare_status(self.output);
        if status.is_set() {
            status.clear();
            match self.policy {
                Policy::Oneshot | Policy::Periodic(None) => (),
                Policy::Periodic(Some(duration)) => {
                    self.gpt.set_output_compare_duration(self.output, duration)
                }
            }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// Adapter that implements [the `CountDown` trait][docs].
///
/// It mutably borrows the GPT, and it uses the supplied
/// output compare register to track time. If the GPT has
/// other compare channels that are active, those channels
/// remain active while the GPT is borrowed.
///
/// [docs]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/timer/trait.CountDown.html
pub struct CountDown<'a>(Timer<'a>);

impl<'a> embedded_hal::timer::CountDown for CountDown<'a> {
    type Time = Duration;

    fn start<T: Into<Duration>>(&mut self, duration: T) {
        self.0.start(duration.into())
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        self.0.wait()
    }
}

/// Adapter that implements [ther `Periodic` trait][docs].
///
/// It mutably borrows the GPT, and it uses the supplied output
/// compare register to track time. If the GPT has other output
/// compare channels that are active, those channels remain active
/// while the GPT is borrowed.
///
/// [docs]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/timer/trait.Periodic.html
pub struct Periodic<'a>(Timer<'a>);

impl<'a> embedded_hal::timer::CountDown for Periodic<'a> {
    type Time = Duration;

    fn start<T: Into<Duration>>(&mut self, duration: T) {
        self.0.start(duration.into())
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        self.0.wait()
    }
}

impl<'a> embedded_hal::timer::Periodic for Periodic<'a> {}
