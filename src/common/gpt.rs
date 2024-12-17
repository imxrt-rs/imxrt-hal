//! General purpose timer.
//!
//! # Features
//!
//! The GPTs are count-up, wrapping timers that run off of the IPG clock or the
//! crystal oscillator (24MHz). Each GPT has three compare registers,
//! called **output comparison registers (OCR)**. When the counter reaches a
//! value in an OCR, the GPT signals the comparison through status flags.
//! A comparison can generate an interrupt.
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
//! # Example
//!
//! These examples do not demonstrate how to configure the GPT clock gates
//! or clock root, PERCLK. To configure PERCLK, see [the CCM peripheral clock
//! module](crate::ccm::perclk_clk).
//!
//! Create, configure, and wait for ticks to elapse:
//!
//! ```no_run
//! use imxrt_ral::gpt::GPT1;
//! use imxrt_hal::gpt;
//!
//! let mut gpt = gpt::Gpt::new(unsafe { GPT1::instance() });
//! gpt.set_clock_source(gpt::ClockSource::HighFrequencyReferenceClock);
//! gpt.set_divider(16);
//! gpt.set_mode(gpt::Mode::FreeRunning);
//! gpt.enable();
//!
//! // Later...
//!
//! const OCR: gpt::OutputCompareRegister = gpt::OutputCompareRegister::OCR1;
//! gpt.clear_elapsed(OCR);
//! let count = gpt.count();
//! let ticks_to_wait = gpt.count().wrapping_add(40_000);
//! gpt.set_output_compare_count(OCR, ticks_to_wait);
//!
//! while !gpt.is_elapsed(OCR) {}
//! gpt.clear_elapsed(OCR);
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

use crate::ral;

/// A general purpose timer
///
/// The timers support three output compare registers. When a compare register
/// matches the value of the counter, the GPT may trigger an interrupt.
///
/// By default, the timer runs in wait mode.
pub struct Gpt<const N: u8> {
    /// Registers for this GPT instance
    gpt: ral::gpt::Instance<N>,
}

/// GPT1 alias.
pub type Gpt1 = Gpt<1>;
/// GPT2 alias.
pub type Gpt2 = Gpt<2>;

/// GPT clock source.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ClockSource {
    /// No clock selection
    NoClock,
    /// Peripheral clock (`ipg_clk`)
    PeripheralClock,
    /// High frequency reference clock (`ipg_clk_highfreq`)
    HighFrequencyReferenceClock,
    /// External clock
    ExternalClock,
    /// Low frequency reference clock (`ipg_clk_32k`)
    LowFrequencyReferenceClock,
    /// Crystal oscillator as reference clock (`ipg_clk_24M`)
    CrystalOscillator,
}

/// An output compare register (OCR).
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum OutputCompareRegister {
    /// The first output compare register.
    OCR1 = 0,
    /// The second output compare register.
    OCR2 = 1,
    /// The third output compare register.
    OCR3 = 2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Possible modes of the GPT.
pub enum Mode {
    /// Restart mode
    ///
    /// A comparions event on channel 1 will reset the GPT counter.
    /// Comparison events on channels 2 and 3 do not reset the counter.
    Restart,
    /// Free running mode
    ///
    /// Comparisons on channel 1 are treated like comparions on channels
    /// 2 and 3. The counter continues to increment on comparison.
    FreeRunning,
}

impl<const N: u8> Gpt<N> {
    /// Create a GPT timer from the RAL's GPT instance.
    ///
    /// When `new` returns, the GPT is reset and disabled.
    pub fn new(gpt: ral::gpt::Instance<N>) -> Self {
        // Disable the timer.
        ral::modify_reg!(ral::gpt, gpt, CR, EN: 0);
        // Software reset
        ral::modify_reg!(ral::gpt, gpt, CR, SWR: 1);
        while ral::read_reg!(ral::gpt, gpt, CR, SWR == 1) {}
        // Clear all status registers.
        ral::write_reg!(ral::gpt, gpt, SR, 0b11_1111);
        Self { gpt }
    }

    /// Returns the clock divider value.
    pub fn divider(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.gpt, PR, PRESCALER) + 1
    }

    /// Set the divider value.
    ///
    /// The divider value is clamped between 1 and 4096.
    /// A change in the divider cause the prescaler counter
    /// to reset and a new count period to start immediately.
    pub fn set_divider(&mut self, divider: u32) {
        let prescaler = divider.clamp(1, 4096) - 1;
        ral::modify_reg!(ral::gpt, self.gpt, PR, PRESCALER: prescaler);
    }

    /// Return the 24MHz clock divider.
    pub fn divider_24mhz(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.gpt, PR, PRESCALER24M) + 1
    }

    /// Set the 24MHz clock divider.
    ///
    /// The divider value is clamped between 1 and 16.
    /// 24MHz crystal clock is divided by the divider
    /// before selected by the clock selection. If 24M
    /// crystal clock is not selected, this feild takes no effect.
    pub fn set_divider_24mhz(&mut self, divider: u32) {
        let prescaler = divider.clamp(1, 16) - 1;
        ral::modify_reg!(ral::gpt, self.gpt, PR, PRESCALER24M: prescaler);
    }

    /// Returns the current mode of the GPT.
    pub fn mode(&self) -> Mode {
        if ral::read_reg!(ral::gpt, self.gpt, CR, FRR == 0) {
            Mode::Restart
        } else {
            Mode::FreeRunning
        }
    }

    /// Set the GPT mode.
    ///
    /// Refer to the module level documentation for more information on the GPT modes.
    pub fn set_mode(&mut self, mode: Mode) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, FRR: (mode as u32))
    }

    /// Returns the GPT clock source
    pub fn clock_source(&self) -> ClockSource {
        let clock_source = ral::read_reg!(ral::gpt, self.gpt, CR, CLKSRC);
        match clock_source {
            0 => ClockSource::NoClock,
            1 => ClockSource::PeripheralClock,
            2 => ClockSource::HighFrequencyReferenceClock,
            3 => ClockSource::ExternalClock,
            4 => ClockSource::LowFrequencyReferenceClock,
            5 => ClockSource::CrystalOscillator,
            _ => unreachable!("Reserved GPT clock source"),
        }
    }

    /// Set the GPT clock source.
    pub fn set_clock_source(&mut self, clock_source: ClockSource) {
        ral::modify_reg!(
            ral::gpt,
            self.gpt,
            CR,
            CLKSRC: clock_source as u32,
            EN_24M: (ClockSource::CrystalOscillator == clock_source) as u32);
    }

    /// Set the reset on enable behavior
    ///
    /// See [the module-level docs](crate::gpt#reset-on-enable) for more information.
    pub fn set_reset_on_enable(&mut self, reset_on_enable: bool) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, ENMOD: (reset_on_enable as u32));
    }

    /// Returns `true` if the GPT counter will reset the next time it is enabled.
    pub fn is_reset_on_enable(&self) -> bool {
        ral::read_reg!(ral::gpt, self.gpt, CR, ENMOD == 1)
    }

    /// Enable the GPT.
    ///
    /// When enabled, the counter starts counting. The value of the counter
    /// is determined by the reset on enable setting. See
    /// [`set_reset_on_enable`](Gpt::set_reset_on_enable).
    pub fn enable(&mut self) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, EN: 1);
    }

    /// Disable the GPT.
    ///
    /// When disabled, the count will stop counting.
    pub fn disable(&mut self) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, EN: 0);
    }

    /// Indicates if the GPT is enabled (`true`) or disabled (`false`).
    pub fn is_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.gpt, CR, EN == 1)
    }

    /// Allow the GPT to run in wait mode; or, prevent the GPT from running
    /// in wait mode.
    pub fn set_wait_mode_enable(&mut self, wait: bool) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, WAITEN: (wait as u32));
    }

    /// Indicates if the GPT runs while in wait mode.
    pub fn is_wait_mode_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.gpt, CR, WAITEN == 1)
    }

    /// Enable the GPT interrupt when the output compares.
    pub fn set_output_interrupt_on_compare(&mut self, ocr: OutputCompareRegister, intr: bool) {
        match ocr {
            OutputCompareRegister::OCR1 => {
                ral::modify_reg!(ral::gpt, self.gpt, IR, OF1IE: intr as u32)
            }
            OutputCompareRegister::OCR2 => {
                ral::modify_reg!(ral::gpt, self.gpt, IR, OF2IE: intr as u32)
            }
            OutputCompareRegister::OCR3 => {
                ral::modify_reg!(ral::gpt, self.gpt, IR, OF3IE: intr as u32)
            }
        }
    }

    /// Returns `true` if a comparison triggers an interrupt.
    pub fn is_output_interrupt_on_compare(&self, ocr: OutputCompareRegister) -> bool {
        match ocr {
            OutputCompareRegister::OCR1 => ral::read_reg!(ral::gpt, self.gpt, IR, OF1IE == 1),
            OutputCompareRegister::OCR2 => ral::read_reg!(ral::gpt, self.gpt, IR, OF2IE == 1),
            OutputCompareRegister::OCR3 => ral::read_reg!(ral::gpt, self.gpt, IR, OF3IE == 1),
        }
    }

    /// Returns the current count of the GPT.
    pub fn count(&self) -> u32 {
        ral::read_reg!(ral::gpt, self.gpt, CNT)
    }

    /// Set an output compare register to trigger on the next `count` value of the
    /// counter.
    pub fn set_output_compare_count(&self, ocr: OutputCompareRegister, count: u32) {
        ral::write_reg!(ral::gpt, self.gpt, OCR[ocr as usize], count);
    }

    /// Returns the current output compare count for the specified register.
    pub fn output_compare_count(&self, ocr: OutputCompareRegister) -> u32 {
        ral::read_reg!(ral::gpt, self.gpt, OCR[ocr as usize])
    }

    /// Returns `true` if the time tracked by the OCR has elapsed.
    pub fn is_elapsed(&self, ocr: OutputCompareRegister) -> bool {
        match ocr {
            OutputCompareRegister::OCR1 => ral::read_reg!(ral::gpt, self.gpt, SR, OF1 == 1),
            OutputCompareRegister::OCR2 => ral::read_reg!(ral::gpt, self.gpt, SR, OF2 == 1),
            OutputCompareRegister::OCR3 => ral::read_reg!(ral::gpt, self.gpt, SR, OF3 == 1),
        }
    }

    /// Clear the elapsed flag.
    pub fn clear_elapsed(&self, ocr: OutputCompareRegister) {
        match ocr {
            OutputCompareRegister::OCR1 => ral::modify_reg!(ral::gpt, self.gpt, SR, OF1: 1),
            OutputCompareRegister::OCR2 => ral::modify_reg!(ral::gpt, self.gpt, SR, OF2: 1),
            OutputCompareRegister::OCR3 => ral::modify_reg!(ral::gpt, self.gpt, SR, OF3: 1),
        }
    }

    /// Enable / disable an interrupt when the GPT counter rolls over from `u32::max_value()` to
    /// `0`.
    ///
    /// The GPT triggers a rollover regardless of the GPT mode.
    pub fn set_rollover_interrupt_enable(&mut self, rov: bool) {
        ral::modify_reg!(ral::gpt, self.gpt, IR, ROVIE: (rov as u32));
    }

    /// Returns `true` if a rollover generates an interrupt.
    pub fn is_rollover_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::gpt, self.gpt, IR, ROVIE == 1)
    }

    /// Returns `true` if the rollover flag is set.
    ///
    /// A rollover occurs when the counter rolls over from `u32::max_value()` to `0`. Rollover
    /// may occur regardless of the GPT mode.
    pub fn is_rollover(&self) -> bool {
        ral::read_reg!(ral::gpt, self.gpt, SR, ROV == 1)
    }

    /// Clear the rollover status flag.
    ///
    /// Users must clear the rollover flag if a rollover triggered an interrupt.
    pub fn clear_rollover(&self) {
        ral::modify_reg!(ral::gpt, self.gpt, SR, ROV: 1);
    }

    /// Issue a software reset.
    ///
    /// This reset does not change
    ///
    /// - the enable state.
    /// - the clock selection.
    /// - the wait, debug, and doze configurations.
    pub fn reset(&mut self) {
        ral::modify_reg!(ral::gpt, self.gpt, CR, SWR: 1);
        while ral::read_reg!(ral::gpt, self.gpt, CR, SWR == SWR_1) {}
    }

    /// Release the peripheral instance.
    ///
    /// This does not change any peripheral state; it simply releases
    /// the instance as-is. If you need to return the registers in a known,
    /// good state, consider calling [`reset()`](Self::reset) and
    /// [`disable()`](Self::disable) before this call.
    pub fn release(self) -> ral::gpt::Instance<N> {
        self.gpt
    }
}
