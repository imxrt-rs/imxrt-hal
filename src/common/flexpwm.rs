//! Pulse width modulation.
//!
//! Each PWM peripheral, [`Pwm`], interacts with four submodules.
//! Each submodule acts as a timer with multiple compare registers, called
//! [`ValueRegister`]s. A comparison event
//!
//! - is signaled through a [`Status`] flag (see [`Pwm::status`]).
//! - can generate an interrupt (see [`Pwm::interrupts`]).
//! - sets a PWM output high or low, depending on the turn on / off values.
//!
//! The PWM driver does not implement any of the embedded-hal PWM traits. You should
//! use these APIs to create your own PWM implementation that satisfies your driver.
//!
//! # Example
//!
//! The PWM submodule counts over the range of `i16` values. The counter runs at
//! the IPG clock frequency. The PWM outputs produce independent, phase-shifted
//! outputs.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! use hal::flexpwm::{self, Pwm, SM::SM2, Channel::{A, B}};
//!
//! # || -> Option<()> {
//! let pwm2 = unsafe { ral::pwm::PWM2::instance() };
//! let mut pwm = Pwm::new::<2>(pwm2);
//!
//! // Keep running in wait, debug modes.
//! pwm.set_debug_enable(SM2, true);
//! pwm.set_wait_enable(SM2, true);
//! // Run on the IPG clock.
//! pwm.set_clock_select(SM2, flexpwm::ClockSelect::Ipg);
//! // Divide the IPG clock by 1.
//! pwm.set_prescaler(SM2, flexpwm::Prescaler::Prescaler1);
//! // Allow PWM outputs to operate independently.
//! pwm.set_pair_operation(SM2, flexpwm::PairOperation::Independent);
//!
//! // Reload every time the full reload value register compares.
//! pwm.set_load_mode(SM2, flexpwm::LoadMode::reload_full());
//! pwm.set_load_frequency(SM2, 1);
//! // Count over the full range of i16 values.
//! pwm.set_initial_count(SM2, i16::MIN);
//! pwm.set_value(SM2, flexpwm::FULL_RELOAD_VALUE_REGISTER, i16::MAX);
//!
//! let mut gpio_b0_10 = // Handle to the pad, channel A
//!     # unsafe { imxrt_iomuxc::imxrt1060::gpio_b0::GPIO_B0_10::new() };
//! let mut gpio_b0_11 = // Handle to the pad, channel B
//!     # unsafe { imxrt_iomuxc::imxrt1060::gpio_b0::GPIO_B0_11::new() };
//! imxrt_iomuxc::flexpwm::prepare(&mut gpio_b0_10);
//! imxrt_iomuxc::flexpwm::prepare(&mut gpio_b0_11);
//!
//! // Set the turn on / off count values.
//! pwm.set_turn_on(SM2, A, i16::MIN / 2);
//! pwm.set_turn_off(SM2, A, i16::MAX / 2);
//! // Output B generates the same duty cycle as A
//! // with a lagging phase shift of 5000 counts.
//! pwm.set_turn_on(SM2, B, pwm.turn_on(SM2, A) + 5000);
//! pwm.set_turn_off(SM2, B, pwm.turn_off(SM2, A) + 5000);
//!
//! // Enable the PWM outputs.
//! pwm.set_output_enable(SM2.mask(), A);
//! pwm.set_output_enable(SM2.mask(), B);
//! // Load the values into the PWM registers.
//! pwm.set_load_ok(SM2.mask());
//! // Start running.
//! pwm.set_run(SM2.mask());
//! # Some(())}();
//! ```

use crate::ral::pwm;

/// Any of the PWM peripheral instances.
type AnyPwmInstance = crate::AnyInstance<pwm::RegisterBlock>;

/// A PWM peripheral.
///
/// The PWM peripheral provides access to peripheral-wide registers
/// and methods to control submodules and pin outputs.
pub struct Pwm {
    pwm: AnyPwmInstance,
}

bitflags::bitflags! {
    /// Bitmask for representing submodules.
    ///
    /// `Mask` is used throughout the PWM API. The interpretation of the
    /// bits depends on the function.
    ///
    /// If you have an [`SM`], use [`SM::mask()`] to easily obtain its bitmask.
    pub struct Mask : u8 {
        /// Submodule 0.
        const SM0 = 1 << 0;
        /// Submodule 1.
        const SM1 = 1 << 1;
        /// Submodule 2.
        const SM2 = 1 << 2;
        /// Submodule 3.
        const SM3 = 1 << 3;
    }
}

impl Pwm {
    /// Create a new PWM driver from a peripheral instance.
    pub fn new<const N: u8>(pwm: pwm::Instance<N>) -> Self {
        self::new(crate::into_any(pwm))
    }

    // TODO: MCTRL should be byte accessible (unlike other PWM modules, which are explicitly
    // documented as "not bye accessible"). If we could load and store directly from the low
    // byte -- where LDOK and CLDOK reside -- we might be able to drop the &mut receiver on
    // the LDOK methods. This requires us to re-define the MCTRL register into two halves.
    // Ideally, this happens in the RAL, but it could also happen in our custom RAL module.
    // Any solution needs to account for the differences between the 1010 and all other chips.

    /// Read the `LDOK` bits.
    ///
    /// Note that the hardware will deassert `LDOK` after the values are loaded.
    pub fn load_ok(&self) -> Mask {
        let ldok = crate::ral::read_reg!(crate::ral::pwm, self.pwm, MCTRL, LDOK);
        Mask::from_bits_truncate(ldok as u8)
    }
    /// Set `LDOK` for zero or more submodules.
    ///
    /// A *high bit* indicates which `LDOK` bit(s) will be *set*.
    pub fn set_load_ok(&mut self, mask: Mask) {
        crate::ral::modify_reg!(crate::ral::pwm, self.pwm, MCTRL, LDOK: mask.bits() as u16);
    }
    /// Clear `LDOK` for zero or more submodules.
    ///
    /// A *high bit* indicates which `LDOK` bit(s) will be *cleared*.
    pub fn clear_load_ok(&mut self, mask: Mask) {
        crate::ral::modify_reg!(crate::ral::pwm, self.pwm, MCTRL, CLDOK: mask.bits() as u16);
    }
    /// Read the `RUN` bit(s).
    pub fn run(&self) -> Mask {
        let run = crate::ral::read_reg!(crate::ral::pwm, self.pwm, MCTRL, RUN);
        Mask::from_bits_truncate(run as u8)
    }
    /// Set or clear the `RUN` bit(s) for one or more submodules.
    ///
    /// This bitmask is written directly to the hardware. To perform a read-modify-write
    /// operation on these bits, make sure to read the initial values with [`Pwm::run`].
    pub fn set_run(&mut self, mask: Mask) {
        crate::ral::modify_reg!(crate::ral::pwm, self.pwm, MCTRL, RUN: mask.bits() as u16);
    }
    /// Read a PWM channel's output enable bits.
    pub fn output_enable(&self, channel: Channel) -> Mask {
        let mask = match channel {
            Channel::A => crate::ral::read_reg!(crate::ral::pwm, self.pwm, OUTEN, PWMA_EN),
            Channel::B => crate::ral::read_reg!(crate::ral::pwm, self.pwm, OUTEN, PWMB_EN),
        };
        Mask::from_bits_truncate(mask as u8)
    }
    /// Set a PWM channel's output enable.
    ///
    /// A high bit indicates the channel is enabled. A low bit disables the channel.
    pub fn set_output_enable(&mut self, mask: Mask, channel: Channel) {
        let mask = mask.bits() as u16;
        match channel {
            Channel::A => crate::ral::modify_reg!(crate::ral::pwm, self.pwm, OUTEN, PWMA_EN: mask),
            Channel::B => crate::ral::modify_reg!(crate::ral::pwm, self.pwm, OUTEN, PWMB_EN: mask),
        }
    }
}

/// Methods that operate on submodules.
impl Pwm {
    fn submodule(&self, sm: SM) -> &pwm::sm::RegisterBlock {
        &self.pwm.SM[sm as usize]
    }

    /// Read the submodule's counter register.
    pub fn sm_count(&self, sm: SM) -> i16 {
        crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCNT) as _
    }

    /// Read a submodule's initial count register.
    ///
    /// This is the value loaded into the submodule counter
    /// when a reload event happens. Note: this reads the
    /// buffered value set with `set_initial_counter` when
    /// the hardware is waiting to load the value.
    pub fn initial_count(&self, sm: SM) -> i16 {
        crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMINIT) as _
    }

    /// Set the submodule initial counter register.
    ///
    /// Note: this value is buffered. It is not reloaded
    /// until the LDOK signal is set and the reload cycle
    /// happens. You cannot write the value when LDOK is
    /// set.
    pub fn set_initial_count(&self, sm: SM, counter: i16) {
        crate::ral::write_reg!(pwm::sm, self.submodule(sm), SMINIT, counter as _);
    }

    /// Returns submodule the load frequency.
    ///
    /// The load frequency describes how many PWM "opportuntities" it will take
    /// before the hardware loads buffered register values into their registers.
    /// This value is between 1 and 16.
    ///
    /// An "opportunity" is one of
    ///
    /// - a full cycle reload (VAL1 matches), if full reload is set.
    /// - a half cycle reload (VAL0 matches), if half reload is set.
    pub fn load_frequency(&self, sm: SM) -> u16 {
        crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL, LDFQ) + 1
    }

    /// Set the submodule load frequency.
    ///
    /// See [`load_frequency`](crate::flexpwm::Pwm::load_frequency) for a
    /// description of load frequency. The implementation clamps the values
    /// between 1 and 16.
    pub fn set_load_frequency(&mut self, sm: SM, ldfq: u16) {
        let ldfq = ldfq.clamp(1, 16) - 1;
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL, LDFQ: ldfq);
    }

    /// Returns the submodule prescaler value.
    pub fn prescaler(&self, sm: SM) -> Prescaler {
        let prescaler = crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL, PRSC);

        #[allow(clippy::assertions_on_constants)]
        {
            use pwm::sm::SMCTRL;
            const _: () = assert!(SMCTRL::PRSC::mask >> SMCTRL::PRSC::offset == 7u16);
            const _: () = assert!(Prescaler::Prescaler128 as u16 == 7u16);
        }

        // Safety: field is three bits wide. Prescaler represents all values in
        // the enum. See the asserts above for tests.
        unsafe { core::mem::transmute(prescaler) }
    }

    /// Set the PWM submodule clock prescaler.
    pub fn set_prescaler(&mut self, sm: SM, prescaler: Prescaler) {
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL, PRSC: prescaler as u16)
    }

    /// Returns the pair operation setting.
    pub fn pair_operation(&self, sm: SM) -> PairOperation {
        let indep = crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL2, INDEP);

        #[allow(clippy::assertions_on_constants)]
        {
            use pwm::sm::SMCTRL2;
            const _: () = assert!(SMCTRL2::INDEP::mask >> SMCTRL2::INDEP::offset == 1u16);
        }

        // Safety: field is one bit. Enum is two variants, representing all values
        // in this one bit state.
        unsafe { core::mem::transmute(indep) }
    }

    /// Set the pair operation setting.
    pub fn set_pair_operation(&mut self, sm: SM, pair_operation: PairOperation) {
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL2, INDEP: pair_operation as u16);
    }

    /// Returns `true` if debug enable is set.
    ///
    /// When set, the PWM continues to run when in debug mode. When clear, the
    /// PWM stops in debug mode, and restarts when debug mode exits.
    pub fn debug_enable(&self, sm: SM) -> bool {
        crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL2, DBGEN == 1)
    }

    /// Set debug enable.
    ///
    /// See [`debug_enable`](Self::debug_enable) for more information on debug
    /// enable.
    pub fn set_debug_enable(&mut self, sm: SM, enable: bool) {
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL2, DBGEN: enable as u16);
    }

    /// Returns `true` if wait enable is set.
    ///
    /// When set, the PWM continues to run when in wait mode. When clear, the PWM
    /// stops in wait mode, and restarts when wait mode exits.
    pub fn wait_enable(&self, sm: SM) -> bool {
        crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL2, WAITEN == 1)
    }

    /// Set wait enable.
    ///
    /// See [`wait_enable`](Self::wait_enable) for more information on wait
    /// enable.
    pub fn set_wait_enable(&mut self, sm: SM, enable: bool) {
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL2, WAITEN: enable as u16);
    }

    /// Returns the clock selection.
    pub fn clock_select(&self, sm: SM) -> ClockSelect {
        const IPG: u16 = ClockSelect::Ipg as u16;
        const EXT: u16 = ClockSelect::External as u16;
        const SM0: u16 = ClockSelect::Submodule0 as u16;

        match crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL2, CLK_SEL) {
            IPG => ClockSelect::Ipg,
            EXT => ClockSelect::External,
            SM0 => ClockSelect::Submodule0,
            _ => unreachable!("Reserved value"),
        }
    }

    /// Set the clock selection.
    ///
    /// Note that you cannot use submodule 0's clock as the submodule 0
    /// source clock. Despite that caveat, this call does not check this
    /// possible configuration.
    ///
    /// # Panics
    ///
    /// You cannot use submodule 0's clock for submodule 0. If the submodule 0 clock
    /// is selected for submodule 0, this call panics.
    pub fn set_clock_select(&mut self, sm: SM, clock_select: ClockSelect) {
        assert!(SM::SM0 != sm || clock_select != ClockSelect::Submodule0);
        crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL2, CLK_SEL: clock_select as u16);
    }

    /// Returns the load mode.
    pub fn load_mode(&self, sm: SM) -> LoadMode {
        let (immediate, full, half) =
            crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMCTRL, LDMOD, FULL, HALF);
        if immediate != 0 {
            LoadMode::Immediate
        } else {
            LoadMode::ReloadCycle {
                full: full != 0,
                half: half != 0,
            }
        }
    }

    /// Set the load mode.
    ///
    /// # Panics
    ///
    /// Panics if the load mode is reload cycle, yet neither `full` nor `half` is set.
    /// Use the [`LoadMode`] helper methods to ensure one of these flags are set.
    pub fn set_load_mode(&mut self, sm: SM, load_mode: LoadMode) {
        match load_mode {
            LoadMode::Immediate => {
                crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL, LDMOD: 1)
            }
            LoadMode::ReloadCycle { full, half } => {
                assert!(
                    full || half,
                    "LoadMode::ReloadCycle must set at least full or half"
                );
                crate::ral::modify_reg!(pwm::sm, self.submodule(sm), SMCTRL, LDMOD: 0, FULL: full as u16, HALF: half as u16)
            }
        }
    }

    /// Read the status flags.
    pub fn status(&self, sm: SM) -> Status {
        let sts = crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMSTS);
        Status::from_bits_truncate(sts)
    }

    /// Clear status flags.
    ///
    /// The high bits are cleared. The implementation will clear the non-W1C bits,
    /// so it's safe to call this with [`Status::all()`].
    pub fn clear_status(&self, sm: SM, status: Status) {
        let sts = status & Status::W1C;
        crate::ral::write_reg!(pwm::sm, self.submodule(sm), SMSTS, sts.bits())
    }

    /// Read the interrupt flags.
    pub fn interrupts(&self, sm: SM) -> Interrupts {
        let inten = crate::ral::read_reg!(pwm::sm, self.submodule(sm), SMINTEN);
        Interrupts::from_bits_truncate(inten)
    }

    /// Set the interrupt flags.
    pub fn set_interrupts(&self, sm: SM, interrupts: Interrupts) {
        crate::ral::write_reg!(pwm::sm, self.submodule(sm), SMINTEN, interrupts.bits());
    }

    /// Read one of the six value registers.
    ///
    /// The return indicates the count value that will cause a comparison.
    pub fn value(&self, sm: SM, value_register: ValueRegister) -> i16 {
        let sm = self.submodule(sm);
        (match value_register {
            ValueRegister::Val0 => crate::ral::read_reg!(pwm::sm, sm, SMVAL0),
            ValueRegister::Val1 => crate::ral::read_reg!(pwm::sm, sm, SMVAL1),
            ValueRegister::Val2 => crate::ral::read_reg!(pwm::sm, sm, SMVAL2),
            ValueRegister::Val3 => crate::ral::read_reg!(pwm::sm, sm, SMVAL3),
            ValueRegister::Val4 => crate::ral::read_reg!(pwm::sm, sm, SMVAL4),
            ValueRegister::Val5 => crate::ral::read_reg!(pwm::sm, sm, SMVAL5),
        }) as _
    }

    /// Get the turn on value for a channel.
    ///
    /// This is the same as using [`turn_on()`] to produce a value register, then
    /// calling [`value()`](Self::value) with that result.
    pub fn turn_on(&self, sm: SM, channel: Channel) -> i16 {
        self.value(sm, turn_on(channel))
    }

    /// Get the turn off value for a channel.
    ///
    /// This is the same as using [`turn_off()`] to produce a value register, then
    /// calling [`value()`](Self::value) with that result.
    pub fn turn_off(&self, sm: SM, channel: Channel) -> i16 {
        self.value(sm, turn_off(channel))
    }

    /// Set one of the six value registers to compare at `value`.
    pub fn set_value(&self, sm: SM, value_register: ValueRegister, value: i16) {
        let value = value as u16;
        let sm = self.submodule(sm);
        match value_register {
            ValueRegister::Val0 => crate::ral::write_reg!(pwm::sm, sm, SMVAL0, value),
            ValueRegister::Val1 => crate::ral::write_reg!(pwm::sm, sm, SMVAL1, value),
            ValueRegister::Val2 => crate::ral::write_reg!(pwm::sm, sm, SMVAL2, value),
            ValueRegister::Val3 => crate::ral::write_reg!(pwm::sm, sm, SMVAL3, value),
            ValueRegister::Val4 => crate::ral::write_reg!(pwm::sm, sm, SMVAL4, value),
            ValueRegister::Val5 => crate::ral::write_reg!(pwm::sm, sm, SMVAL5, value),
        }
    }

    /// Set the turn on compare for a channel.
    ///
    /// This is the same as using [`turn_on()`] to produce a value register, then
    /// calling [`set_value()`](Self::set_value) with that result.
    pub fn set_turn_on(&self, sm: SM, channel: Channel, compare: i16) {
        self.set_value(sm, turn_on(channel), compare);
    }

    /// Set the turn off compare for a channel.
    ///
    /// This is the same as using [`turn_off()`] to produce a value register, then
    /// calling [`set_value()`](Self::set_value) with that result.
    pub fn set_turn_off(&self, sm: SM, channel: Channel, compare: i16) {
        self.set_value(sm, turn_off(channel), compare);
    }
}

#[inline(never)]
fn new(pwm: AnyPwmInstance) -> Pwm {
    // Clear fault levels.
    crate::ral::write_reg!(crate::ral::pwm, pwm, FCTRL0, FLVL: 0xF);
    // Clear fault flags.
    crate::ral::write_reg!(crate::ral::pwm, pwm, FSTS0, FFLAG: 0xF);

    Pwm { pwm }
}

/// Index for submodule access.
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SM {
    /// Submodule 0.
    SM0 = 0,
    /// Submodule 1.
    SM1 = 1,
    /// Submodule 2.
    SM2 = 2,
    /// Submodule 3.
    SM3 = 3,
}

impl SM {
    /// Return this submodule's bitmask.
    #[inline]
    pub const fn mask(self) -> Mask {
        Mask::from_bits_truncate(1 << (self as u8))
    }
}

impl From<SM> for Mask {
    #[inline]
    fn from(value: SM) -> Self {
        value.mask()
    }
}

impl core::ops::BitOr for SM {
    type Output = Mask;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.mask() | rhs.mask()
    }
}

impl core::ops::BitOr<Mask> for SM {
    type Output = Mask;
    #[inline]
    fn bitor(self, rhs: Mask) -> Self::Output {
        self.mask() | rhs
    }
}

impl core::ops::BitOr<SM> for Mask {
    type Output = Mask;
    #[inline]
    fn bitor(self, rhs: SM) -> Self::Output {
        self | rhs.mask()
    }
}

/// PWM clock prescaler.
///
/// Affects all timing, except for the glitch filters.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Prescaler {
    /// Divide the PWM clock by 1.
    Prescaler1,
    /// Divide the PWM clock by 2.
    Prescaler2,
    /// Divide the PWM clock by 4.
    Prescaler4,
    /// Divide the PWM clock by 8.
    Prescaler8,
    /// Divide the PWM clock by 16.
    Prescaler16,
    /// Divide the PWM clock by 32.
    Prescaler32,
    /// Divide the PWM clock by 64.
    Prescaler64,
    /// Divide the PWM clock by 128.
    Prescaler128,
}

impl Prescaler {
    /// Returns the prescalar value as a divisor.
    pub const fn divider(self) -> u32 {
        1 << self as u32
    }
}

/// Describes how PWM channels A and B operate.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PairOperation {
    /// A and B form a complementary pair.
    Complementary,
    /// A and B operate independently.
    Independent,
}

/// PWM input clock selection.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ClockSelect {
    /// Derive from the IPG clock.
    Ipg,
    /// Use EXT_CLK, an external clock.
    External,
    /// Use submodule 0's clock.
    ///
    /// The clock is controlled by SM0's run bit. It's
    /// affected by the SM0 prescaler.
    ///
    /// You cannot use this clock for submodule 0 itself.
    Submodule0,
}

/// PWM (re)load mode.
///
/// Use the associated methods to simply define `ReloadCycle`
/// values.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadMode {
    /// Reload on the next cycle after `LDOK` is set.
    ///
    /// One of these should be set. You may set both
    /// to increase the reload opportunity frequency.
    ReloadCycle {
        /// Reload on a full cycle (VAL1 compares).
        full: bool,
        /// Reload on a half cycle (VAL0 compares).
        half: bool,
    },
    /// Reload immediately after `LDOK` is set.
    Immediate,
}

impl LoadMode {
    /// Full reload cycle.
    pub const fn reload_full() -> Self {
        Self::ReloadCycle {
            full: true,
            half: false,
        }
    }
    /// Half reload cycle.
    pub const fn reload_half() -> Self {
        Self::ReloadCycle {
            full: false,
            half: true,
        }
    }
    /// Full and half reload cycle.
    pub const fn reload_both() -> Self {
        Self::ReloadCycle {
            full: true,
            half: true,
        }
    }
}

bitflags::bitflags! {
    /// Status register flags.
    pub struct Status : u16 {
        /// Registers updated flag.
        ///
        /// This read-only flag is set to 1 when there's a
        /// buffered value that the hardware will load on
        /// the next LDOK assertion. Use this flag to know
        /// if there is data in a buffered register.
        const REGISTER_UPDATED = 1 << 14;
        /// Reload error flag.
        ///
        /// Set when a reload cycle passed, there's something
        /// in the buffered registers, and LDOK was 0. Cleared
        /// by writing 1.
        const RELOAD_ERROR = 1 << 13;
        /// Reload flag.
        ///
        /// Set at the beginning of every reload cycle, regardless
        /// of LDOK. Cleared by writing 1.
        const RELOAD = 1 << 12;

        /// VAL5 compared to the counter value.
        const COMPARE_VAL5 = 1 << 5;
        /// VAL4 compared to the counter value.
        const COMPARE_VAL4 = 1 << 4;
        /// VAL3 compared to the counter value.
        const COMPARE_VAL3 = 1 << 3;
        /// VAL2 compared to the counter value.
        const COMPARE_VAL2 = 1 << 2;
        /// VAL1 compared to the counter value.
        const COMPARE_VAL1 = 1 << 1;
        /// VAL0 compared to the counter value.
        const COMPARE_VAL0 = 1 << 0;
    }
}

impl Status {
    /// The set of write-1-clear status bits.
    pub const W1C: Status = Self::REGISTER_UPDATED.complement();
}

bitflags::bitflags! {
    /// Interrupt flags.
    pub struct Interrupts : u16 {
        /// Reload error interrupt enable.
        const RELOAD_ERROR = 1 << 13;
        /// Reload interrupt enable.
        const RELOAD = 1 << 12;

        /// VAL5 compare interrupt enable.
        const COMPARE_VAL5 = 1 << 5;
        /// VAL4 compare interrupt enable.
        const COMPARE_VAL4 = 1 << 4;
        /// VAL3 compare interrupt enable.
        const COMPARE_VAL3 = 1 << 3;
        /// VAL2 compare interrupt enable.
        const COMPARE_VAL2 = 1 << 2;
        /// VAL1 compare interrupt enable.
        const COMPARE_VAL1 = 1 << 1;
        /// VAL0 compare interrupt enable.
        const COMPARE_VAL0 = 1 << 0;
    }
}

/// PWM value registers.
///
/// These value registers describe when PWM counters reset, and when outputs
/// turn on and off. Consider using more descriptive constants, enums, and
/// const functions to describe these values.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueRegister {
    /// The [`HALF_RELOAD_VALUE_REGISTER`].
    Val0,
    /// The [`FULL_RELOAD_VALUE_REGISTER`].
    Val1,
    /// The [`turn_on()`] register for [`Channel::A`].
    Val2,
    /// The [`turn_off()`] register for [`Channel::A`].
    Val3,
    /// The [`turn_on()`] register for [`Channel::B`].
    Val4,
    /// The [`turn_off()`] register for [`Channel::B`].
    Val5,
}

/// The full reload value register.
///
/// When this register compares to the counter value, the counter
/// resets.
pub const FULL_RELOAD_VALUE_REGISTER: ValueRegister = ValueRegister::Val1;
/// The half reload value register.
///
/// When this register compares to the counter value, it represents
/// a half reload opportunity.
pub const HALF_RELOAD_VALUE_REGISTER: ValueRegister = ValueRegister::Val0;

/// Returns the "turn on" value register for an output channel.
///
/// When the counter compares to this value register, the PWM output
/// turns on.
pub const fn turn_on(channel: Channel) -> ValueRegister {
    match channel {
        Channel::A => ValueRegister::Val2,
        Channel::B => ValueRegister::Val4,
    }
}

/// Returns the "turn off" value register for an output channel.
///
/// When the counter compares to this value register, the PWM output
/// turns off.
pub const fn turn_off(channel: Channel) -> ValueRegister {
    match channel {
        Channel::A => ValueRegister::Val3,
        Channel::B => ValueRegister::Val5,
    }
}

/// PWM channels.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Channel A.
    A,
    /// Channel B.
    B,
}
