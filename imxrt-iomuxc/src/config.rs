//! Pad configuration

use crate::IOMUX;
use core::ptr;

/// Applies the configuration `config` for the supplied pad
///
/// `configure` lets you specify the pad's drive strength, speed, pull-up or pull-down
/// resistors, and other configurations. See [`Config`](struct.Config.html)
/// for possible configurations.
///
/// # Safety
///
/// We can't guarantee that the pointer to the pad's configuration register is
/// correct.
///
/// # Example
///
/// ```no_run
/// use imxrt_iomuxc::{configure, Config, OpenDrain, PullKeep};
/// # use imxrt_iomuxc::IOMUX; #[allow(non_camel_case_types)] pub struct AD_B0_03;
/// # impl AD_B0_03 { unsafe fn new() -> Self { Self } fn ptr(&self) -> *mut u32 { core::ptr::null_mut() }}
/// # unsafe impl IOMUX for AD_B0_03 { unsafe fn mux(&mut self) -> *mut u32 { self.ptr() } unsafe fn pad(&mut self) -> *mut u32 { self.ptr() }}
///
/// const CONFIG: Config = Config::zero()
///     .set_open_drain(OpenDrain::Enabled)
///     .set_pull_keep(PullKeep::Enabled);
///
/// let mut pad = unsafe { AD_B0_03::new() };
///
/// unsafe { configure(&mut pad, CONFIG) };
/// ```
#[inline(always)]
pub unsafe fn configure<I: IOMUX>(pad: &mut I, config: Config) {
    let cfg = ptr::read_volatile(pad.pad());
    let cfg = (cfg & !config.mask) | config.value;
    ptr::write_volatile(pad.pad(), cfg);
}

const HYSTERESIS_SHIFT: u32 = 16;
const HYSTERESIS_MASK: u32 = 1 << HYSTERESIS_SHIFT;

/// The hysteresis (HYS) bit controls whether a pin acts as a Schmitt trigger,
/// which is a comparator remembering its last input state (hysteresis).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Hysteresis {
    Enabled = 1 << HYSTERESIS_SHIFT,
    Disabled = 0 << HYSTERESIS_SHIFT,
}

const PULLUPDOWN_SHIFT: u32 = 14;
const PULLUPDOWN_MASK: u32 = 0b11 << PULLUPDOWN_SHIFT;

/// Controls signals to select pull-up or pull-down internal resistance strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PullUpDown {
    /// 100KOhm pull Down
    Pulldown100k = 0b00 << PULLUPDOWN_SHIFT,
    /// 47KOhm pull up
    Pullup47k = 0b01 << PULLUPDOWN_SHIFT,
    /// 100KOhm pull up
    Pullup100k = 0b10 << PULLUPDOWN_SHIFT,
    /// 22KOhm pull up
    Pullup22k = 0b11 << PULLUPDOWN_SHIFT,
}

const PULL_KEEP_SELECT_SHIFT: u32 = 13;
const PULL_KEEP_SELECT_MASK: u32 = 1 << PULL_KEEP_SELECT_SHIFT;

/// Control signal to enable internal pull-up/down resistors or pad keeper functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PullKeepSelect {
    /// Keep the previous output value when the output driver is disabled.
    Keeper = 0 << PULL_KEEP_SELECT_SHIFT,
    /// Pull-up or pull-down (determined by [`PullUpDown`](struct.PullUpDown.html) flags).
    Pull = 1 << PULL_KEEP_SELECT_SHIFT,
}

const PULLKEEP_SHIFT: u32 = 12;
const PULLKEEP_MASK: u32 = 1 << PULLKEEP_SHIFT;

/// Enable or disable the pull / keeper functionality
///
/// When the pull/keeper is disabled, `PullKeepSelect` and `PullUpDown` have no functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PullKeep {
    Enabled = 1 << PULLKEEP_SHIFT,
    Disabled = 0 << PULLKEEP_SHIFT,
}

const OPENDRAIN_SHIFT: u32 = 11;
const OPENDRAIN_MASK: u32 = 1 << OPENDRAIN_SHIFT;

/// Open Drain Enable Field
///
/// If enabled, the output driver drives only logic 0. The drain of the
/// internal transistor is open. It means that logic 1 has to be driven by
/// an external component. This option is essential if connection between
/// the pad and an external component is bi-directional. If disabled, then
/// the output driver drives logic 1 and logic 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OpenDrain {
    Enabled = 1 << OPENDRAIN_SHIFT,
    Disabled = 0 << OPENDRAIN_SHIFT,
}

const SPEED_SHIFT: u32 = 6;
const SPEED_MASK: u32 = 0b11 << SPEED_SHIFT;

/// Sets electrical characteristics of a pin in a given frequency range
///
/// Speed is a selectable bit field that sets electrical characteristics of
/// a pin in a given frequency range. This field provides additional 2-bit
/// slew rate control. These options can either increase the output driver
/// current in the higher frequency range, or reduce the switching noise in
/// the lower frequency range.
///
/// The operational frequency on GPIO pads is dependent on slew rate (SRE),
/// speed (SPEED), and supply voltage (OVDD).
///
/// See Operating Frequency table in the GPIO block guide in the reference
/// manual for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Speed {
    Low = 0b00 << SPEED_SHIFT,
    Medium = 0b01 << SPEED_SHIFT,
    Fast = 0b10 << SPEED_SHIFT,
    Max = 0b11 << SPEED_SHIFT,
}

const DRIVE_STRENGTH_SHIFT: u32 = 3;
const DRIVE_STRENGTH_MASK: u32 = 0b111 << DRIVE_STRENGTH_SHIFT;

/// Drive strength
///
/// The drive strength enable (DSE) can be explained as series resistance between an ideal driver’s
/// output and its load. To achieve maximal transferred power, the impedance of the driver has to
/// match the load impedance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DriveStrength {
    Disabled = 0b000 << DRIVE_STRENGTH_SHIFT,
    /// 150 Ohm @ 3.3V, 260 Ohm@1.8V
    R0 = 0b001 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 2
    R0_2 = 0b010 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 3
    R0_3 = 0b011 << DRIVE_STRENGTH_SHIFT,
    /// R0 / 4
    R0_4 = 0b100 << DRIVE_STRENGTH_SHIFT,
    R0_5 = 0b101 << DRIVE_STRENGTH_SHIFT,
    R0_6 = 0b110 << DRIVE_STRENGTH_SHIFT,
    R0_7 = 0b111 << DRIVE_STRENGTH_SHIFT,
}

const SLEW_RATE_SHIFT: u32 = 0;
const SLEW_RATE_MASK: u32 = 1 << SLEW_RATE_SHIFT;

/// Slew Rate
///
/// This controls how fast the pin toggles between the two logic states.
/// Since rapidly changing states consume more power and generate spikes,
/// it should be enabled only when necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SlewRate {
    Fast = 1 << SLEW_RATE_SHIFT,
    Slow = 0 << SLEW_RATE_SHIFT,
}

/// A configuration capable of compile-time, `const` configuration:
///
/// ```
/// use imxrt_iomuxc::{Config, SlewRate, Hysteresis};
///
/// const UART_TX_CONFIG: Config = Config::zero()
///     .set_slew_rate(SlewRate::Fast)
///     .set_hysteresis(Hysteresis::Enabled);
/// ```
///
/// Use [`configure()`](fn.configure.html) to set configurations to pads.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    value: u32,
    mask: u32,
}

impl Config {
    /// When we create the zero mask, we set all bits high. But,
    /// the highest usable bit in the pad register is bit 16. We
    /// can use a higher bit to represent if the config is generated
    /// from `zero()`, or if it was generated from `modify()`.
    const ZERO_BIT: u32 = 1 << 31;

    /// Create a `Config` that will zero any unspecified field
    ///
    /// This may *not* represent any register's reset value. A config that's
    /// created using `zero()` will have the effect of writing *all* fields
    /// to the register. Those that are not set explicitly set are written as zero.
    ///
    /// ```
    /// # use imxrt_iomuxc::IOMUX;
    /// # struct Pad(u32); unsafe impl IOMUX for Pad { unsafe fn mux(&mut self) -> *mut u32 { panic!() } unsafe fn pad(&mut self) -> *mut u32 { &mut self.0 as *mut _} }
    /// # let mut ad_b0_13 = Pad(0xFFFF_FFFFu32);
    /// use imxrt_iomuxc::{
    ///     Config, configure, SlewRate,
    ///     Hysteresis, PullUpDown, DriveStrength
    /// };
    ///
    /// unsafe {
    ///     // Set a configuration
    ///     configure(
    ///         &mut ad_b0_13,
    ///         Config::zero()
    ///             .set_slew_rate(SlewRate::Fast)
    ///             .set_drive_strength(DriveStrength::R0_7)
    ///     );
    ///     assert_eq!(
    ///         *ad_b0_13.pad(),
    ///         DriveStrength::R0_7 as u32 | SlewRate::Fast as u32
    ///     );
    ///
    ///     // Completely change that configuration
    ///     configure(
    ///         &mut ad_b0_13,
    ///         Config::zero()
    ///             .set_hysteresis(Hysteresis::Enabled)
    ///             .set_pullupdown(PullUpDown::Pullup22k)
    ///     );
    ///     assert_eq!(
    ///         *ad_b0_13.pad(),
    ///         Hysteresis::Enabled as u32 | PullUpDown::Pullup22k as u32
    ///     );
    /// }
    /// ```
    pub const fn zero() -> Self {
        Config {
            value: 0u32,
            mask: 0xFFFF_FFFFu32,
        }
    }

    /// Create a `Config` that will only modify the specified fields
    ///
    /// Any field that is is *not* specified in the configuration will not be touched.
    ///
    /// ```
    /// # use imxrt_iomuxc::IOMUX;
    /// # struct Pad(u32); unsafe impl IOMUX for Pad { unsafe fn mux(&mut self) -> *mut u32 { panic!() } unsafe fn pad(&mut self) -> *mut u32 { &mut self.0 as *mut _} }
    /// # let mut ad_b0_13 = Pad(0xFFFF_FFFFu32);
    /// use imxrt_iomuxc::{Config, configure, SlewRate, DriveStrength, Hysteresis};
    ///
    /// unsafe {
    ///     // Assume a starting value in the register...
    ///     configure(
    ///         &mut ad_b0_13,
    ///         Config::zero()
    ///             .set_slew_rate(SlewRate::Fast)
    ///             .set_drive_strength(DriveStrength::R0_7)
    ///     );
    ///     assert_eq!(
    ///         *ad_b0_13.pad(),
    ///         DriveStrength::R0_7 as u32 | SlewRate::Fast as u32
    ///     );
    ///
    ///     // Now change the slew rate, and add hysteresis
    ///     configure(
    ///         &mut ad_b0_13,
    ///         Config::modify()
    ///             .set_slew_rate(SlewRate::Slow)
    ///             .set_hysteresis(Hysteresis::Enabled)
    ///     );
    ///     assert_eq!(
    ///         *ad_b0_13.pad(),
    ///         DriveStrength::R0_7 as u32 | Hysteresis::Enabled as u32
    ///     );
    /// }
    /// ```
    pub const fn modify() -> Self {
        Config {
            value: 0u32,
            mask: 0u32,
        }
    }

    /// Returns `true` if this `Config` was created using [`zero()`](struct.Config.html#method.zero), meaning that it will
    /// zero any unspecified fields. If `false`, this config was created using [`modify()`](struct.Config.html#method.modify),
    /// which will not touch unspecified fields.
    ///
    /// ```
    /// use imxrt_iomuxc::Config;
    ///
    /// assert!(Config::zero().is_zero());
    /// assert!(!Config::modify().is_zero());
    /// ```
    pub const fn is_zero(&self) -> bool {
        self.mask & Self::ZERO_BIT != 0
    }

    /// Set the hysteresis bit
    pub const fn set_hysteresis(mut self, hys: Hysteresis) -> Self {
        self.value = (self.value & !HYSTERESIS_MASK) | (hys as u32);
        self.mask |= HYSTERESIS_MASK;
        self
    }

    /// Set the pull-up / pull-down value
    pub const fn set_pullupdown(mut self, pud: PullUpDown) -> Self {
        self.value = (self.value & !PULLUPDOWN_MASK) | (pud as u32);
        self.mask |= PULLUPDOWN_MASK;
        self
    }

    /// Set the the pull-up / pull-down or keeper selection bit
    pub const fn set_pull_keep_select(mut self, pke: PullKeepSelect) -> Self {
        self.value = (self.value & !PULL_KEEP_SELECT_MASK) | (pke as u32);
        self.mask |= PULL_KEEP_SELECT_MASK;
        self
    }

    /// Set the flag that enables the keeper or pull-up / pull-down configuration
    pub const fn set_pull_keep(mut self, pk: PullKeep) -> Self {
        self.value = (self.value & !PULLKEEP_MASK) | (pk as u32);
        self.mask |= PULLKEEP_MASK;
        self
    }

    /// Set the open drain value
    pub const fn set_open_drain(mut self, od: OpenDrain) -> Self {
        self.value = (self.value & !OPENDRAIN_MASK) | (od as u32);
        self.mask |= OPENDRAIN_MASK;
        self
    }

    /// Set the pin speed
    pub const fn set_speed(mut self, speed: Speed) -> Self {
        self.value = (self.value & !SPEED_MASK) | (speed as u32);
        self.mask |= SPEED_MASK;
        self
    }

    /// Set the drive strength
    pub const fn set_drive_strength(mut self, dse: DriveStrength) -> Self {
        self.value = (self.value & !DRIVE_STRENGTH_MASK) | (dse as u32);
        self.mask |= DRIVE_STRENGTH_MASK;
        self
    }

    /// Set the slew rate
    pub const fn set_slew_rate(mut self, sre: SlewRate) -> Self {
        self.value = (self.value & !SLEW_RATE_MASK) | (sre as u32);
        self.mask |= SLEW_RATE_MASK;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Pad(u32);

    const PAD_ALL_HIGH: Pad = Pad(0x0001_FFFFu32);

    /// The high bits represent the valid fields in pad registers.
    const PAD_BITMASK: u32 = 0b1_1111_1000_1111_1001u32;

    unsafe impl IOMUX for Pad {
        unsafe fn mux(&mut self) -> *mut u32 {
            panic!("Nothing calls mux() in these tests");
        }
        unsafe fn pad(&mut self) -> *mut u32 {
            &mut self.0 as *mut _
        }
    }

    #[test]
    fn zero() {
        let mut pad = PAD_ALL_HIGH;
        unsafe { configure(&mut pad, Config::zero()) };
        assert_eq!(pad.0, 0);
    }

    #[test]
    fn zero_set_all() {
        let mut pad = PAD_ALL_HIGH;
        const CONFIG: Config = Config::zero()
            .set_hysteresis(Hysteresis::Enabled)
            .set_pullupdown(PullUpDown::Pullup22k)
            .set_pull_keep_select(PullKeepSelect::Pull)
            .set_pull_keep(PullKeep::Enabled)
            .set_open_drain(OpenDrain::Enabled)
            .set_speed(Speed::Max)
            .set_drive_strength(DriveStrength::R0_7)
            .set_slew_rate(SlewRate::Fast);

        unsafe {
            configure(&mut pad, CONFIG);
        }

        assert_eq!(pad.0, PAD_BITMASK);
    }

    #[test]
    fn modify_clear_all() {
        let mut pad = Pad(PAD_BITMASK);
        const CONFIG: Config = Config::modify()
            .set_hysteresis(Hysteresis::Disabled)
            .set_pullupdown(PullUpDown::Pulldown100k)
            .set_pull_keep_select(PullKeepSelect::Keeper)
            .set_pull_keep(PullKeep::Disabled)
            .set_open_drain(OpenDrain::Disabled)
            .set_speed(Speed::Low)
            .set_drive_strength(DriveStrength::Disabled)
            .set_slew_rate(SlewRate::Slow);

        unsafe {
            configure(&mut pad, CONFIG);
        }

        assert_eq!(pad.0, 0);
    }
}
