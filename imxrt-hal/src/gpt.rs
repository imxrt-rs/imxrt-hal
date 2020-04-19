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
    registers: ral::gpt::Instance,
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

        let clock_hz = (freq / div).0;

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
    pub fn set_interrupt_on_compare(&mut self, output: OutputCompareRegister, intr: bool) {
        let ir: u32 = ral::read_reg!(ral::gpt, self.registers, IR);
        let ir: u32 = if intr {
            ir | (1 << (output as u32))
        } else {
            ir & !(1 << (output as u32))
        };
        ral::write_reg!(ral::gpt, self.registers, IR, ir);
    }

    /// Returns the current count of the GPT
    pub fn count(&self) -> u32 {
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
    /// This requires a read of the counter, then a conversion of the duration of the duration
    /// to a count value, then a write to the correct register. If the GPT is currently
    /// enabled, and the duration is small, this could miss the comparison until the counter
    /// wraps. Consider disabling the GPT before setting a wait duration.
    pub fn set_output_compare_duration(
        &mut self,
        output: OutputCompareRegister,
        duration: Duration,
    ) {
        let tics: u32 =
            ticks(duration, self.clock_hz, DEFAULT_PRESCALER).unwrap_or(u32::max_value());
        let next_count = self.count().wrapping_add(tics);
        self.set_output_compare_count(output, next_count);
    }

    /// Returns a handle that can query and modify the output compare status for the provided output
    pub fn output_compare_status(&mut self, output: OutputCompareRegister) -> OutputCompareStatus {
        OutputCompareStatus { gpt: self, output }
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
