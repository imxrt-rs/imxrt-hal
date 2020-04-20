//! General Purpose Timer (GPT)
//!
//! ## Features
//!
//! The GPTs are count-up timers that run off of the IPG
//! clock or the crystal oscillator (24MHz). Each GPT has
//! three compare registers. When the counter reaches a
//! value in a compare register, the GPT signals the comparison.
//! A comparison can generate an interrupt.
//!
//! Use GPTs to
//!
//! - detect if a timing interval has elapsed
//! - trigger an interrupt when an interval elapses
//! - measure code execution
//!
//! ## TODO
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
                    CLKSRC: 0b101, // Crystal Oscillator
                    FRR: 1, // Channel 1 doesn't reset the counter on trigger
                    WAITEN: 1 // Run GPT in wait mode
                );
                // The 24MHz prescaler register can't be non-zero. Not sure why.
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
                    CLKSRC: 0b001, // Peripheral Clock
                    FRR: 1, // Channel 1 doesn't reset the counter on trigger
                    WAITEN: 1 // Run GPT in wait mode
                );
                // See the above comment about needing a prescaler for the other
                // clock. This is for consistency, so that we can implement the
                // "divide by two" behavior.
                ral::write_reg!(ral::gpt, self.registers, PR, PRESCALER: (DEFAULT_PRESCALER - 1));
            }
        }

        // Clear all statuses
        ral::write_reg!(ral::gpt, self.registers, SR, 0b11_1111);

        let clock_hz = (freq / div).0 / DEFAULT_PRESCALER;

        GPT {
            registers: self.registers,
            clock_hz,
        }
    }
}

/// An output compare register indication
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputCompareRegister {
    One,
    Two,
    Three,
}

impl GPT {
    /// Enable or disable the GPT
    ///
    /// When enabled, the counter starts counting. When disabled, the counter will
    /// stop counting.
    pub fn set_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, EN: (enable as u32));
    }

    /// Indicates if the GPT is enabled (`true`) or disabled (`false`).
    pub fn is_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.registers, CR, EN == 1)
    }

    /// Allow the GPT to run in wait mode; or, prevent the GPT from running
    /// in wait mode.
    pub fn set_wait_mode_enable(&mut self, wait: bool) {
        ral::modify_reg!(ral::gpt, self.registers, CR, WAITEN: (wait as u32));
    }

    /// Indicates if the GPT runs while in wait mode
    pub fn is_wait_mode_enabled(&self) -> bool {
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
    pub fn is_output_interrupt_on_compare(&self, output: OutputCompareRegister) -> bool {
        ral::read_reg!(ral::gpt, self.registers, IR) & (1 << (output as u32)) != 0
    }

    /// Returns the current count of the GPT
    fn count(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.registers, CNT)
    }

    /// Set an output compare register to trigger on the next `count` value of the
    /// counter.
    fn set_output_compare_count(&mut self, output: OutputCompareRegister, count: u32) {
        match output {
            OutputCompareRegister::One => ral::write_reg!(ral::gpt, self.registers, OCR1, count),
            OutputCompareRegister::Two => ral::write_reg!(ral::gpt, self.registers, OCR2, count),
            OutputCompareRegister::Three => ral::write_reg!(ral::gpt, self.registers, OCR3, count),
        }
    }

    /// Set an output compare register to trigger when the specified duration elapses
    ///
    /// If the duration is very small, the time could elapse before we commit the comparison
    /// value to the correct register. If this is a concern, consider disabling the GPT
    /// before specifying an output duration.
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

    /// Returns an adapter that implements count down traits
    ///
    /// Assumes that the timer is already enabled. Otherwise, the
    /// count down adapter will block forever. The adapter will never
    /// disable the counter, so the borrowed GPT may still track other
    /// times while it is borrowed.
    pub fn count_down(&mut self, output: OutputCompareRegister) -> CountDown {
        CountDown(Timer::oneshot(self, output))
    }

    /// Returns an adapter that implements periodic traits
    ///
    /// Assumes that the timer is already enabled. Otherwise, the
    /// periodic adapter will block forever. The adapter will never
    /// disable the counter. so the borrowed GPT may still track other
    /// times while it is borrowed.
    pub fn periodic(&mut self, output: OutputCompareRegister) -> Periodic {
        Periodic(Timer::periodic(self, output))
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
