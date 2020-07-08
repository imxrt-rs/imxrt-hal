//! Pad configuration

#![allow(clippy::identity_op)] // Visually consistent constants

const HYSTERESIS_MASK: u32 = 1 << 16;

/// The hysteresis (HYS) bit controls whether a pin acts as a Schmitt trigger,
/// which is a comparator remembering its last input state (hysteresis).
#[repr(u32)]
pub enum Hysteresis {
    Enabled = 1 << 16,
    Disabled = 0 << 16,
}

const PULLUPDOWN_MASK: u32 = 0b11 << 14;

/// Controls signals to select pull-up or pull-down internal resistance strength.
#[repr(u32)]
pub enum PullUpDown {
    /// 100KOhm pull Down
    Pulldown100k = 0b00 << 14,
    /// 47KOhm pull up
    Pullup47k = 0b01 << 14,
    /// 100KOhm pull up
    Pullup100k = 0b10 << 14,
    /// 22KOhm pull up
    Pullup22k = 0b11 << 14,
}

const PULL_KEEP_SELECT_MASK: u32 = 1 << 13;

/// Control signal to enable internal pull-up/down resistors or pad keeper functionality.
#[repr(u32)]
pub enum PullKeepSelect {
    /// Keep the previous output value when the output driver is disabled.
    Keeper = 0 << 13,
    /// Pull-up or pull-down (determined by [`PullUpDown`](struct.PullUpDown.html) flags).
    Pull = 1 << 13,
}

const PULLKEEP_MASK: u32 = 1 << 12;

/// Enable or disable the pull / keeper functionality
///
/// When the pull/keeper is disabled, `PullKeepSelect` and `PullUpDown` have no functionality.
#[repr(u32)]
pub enum PullKeep {
    Enabled = 1 << 12,
    Disabled = 0 << 12,
}

const OPENDRAIN_MASK: u32 = 1 << 11;

/// Open Drain Enable Field
#[repr(u32)]
pub enum OpenDrain {
    Enabled = 1 << 11,
    Disabled = 0 << 11,
}

const SPEED_MASK: u32 = 0b11 << 6;

/// Sets electrical characteristics of a pin in a given frequency range
#[repr(u32)]
pub enum Speed {
    Low = 0b00 << 6,
    Medium = 0b01 << 6,
    Fast = 0b10 << 6,
    Max = 0b11 << 6,
}

const DRIVE_STRENGTH_MASK: u32 = 0b111 << 3;

/// Drive strength
///
/// The drive strength enable (DSE) can be explained as series resistance between an ideal driver’s
/// output and its load. To achieve maximal transferred power, the impedance of the driver has to
/// match the load impedance.
#[repr(u32)]
pub enum DriveStrength {
    Disabled = 0b000 << 3,
    /// 150 Ohm @ 3.3V, 260 Ohm@1.8V
    R0 = 0b001 << 3,
    /// R0 / 2
    R0_2 = 0b010 << 3,
    /// R0 / 3
    R0_3 = 0b011 << 3,
    /// R0 / 4
    R0_4 = 0b100 << 3,
    R0_5 = 0b101 << 3,
    R0_6 = 0b110 << 3,
    R0_7 = 0b111 << 3,
}

const SLEW_RATE_MASK: u32 = 1 << 0;

/// Slew Rate
///
/// This controls how fast the pin toggles between the two logic states.
/// Since rapidly changing states consume more power and generate spikes,
/// it should be enabled only when necessary.
#[repr(u32)]
pub enum SlewRate {
    Fast = 1 << 0,
    Slow = 0 << 0,
}

/// An opaque pad configuration
///
/// See [`ConfigBuilder`](struct.ConfigBuilder.html) to create a pad configuration.
/// Once you have a `Config`, use [`configure()`](fn.configure.html) to set the configuration
/// for a pad.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config(pub(crate) u32);

/// A configuration builder capable of compile-time, `const` configuration generation
///
/// A `ConfigBuilder` lets you generate a `Config`:
///
/// ```
/// use imxrt_iomuxc::{Config, ConfigBuilder, SlewRate, Hysteresis};
///
/// const UART_TX_CONFIG: Config = ConfigBuilder::zero()
///     .set_slew_rate(SlewRate::Fast)
///     .set_hysteresis(Hysteresis::Enabled)
///     .build();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ConfigBuilder(u32);

impl ConfigBuilder {
    /// Create a `ConfigBuilder` that has an internal value of zero
    ///
    /// This does *not* represent any register's reset value.
    pub const fn zero() -> Self {
        ConfigBuilder(0)
    }

    /// Consume a `ConfigBuilder` and create a `Config`
    pub const fn build(self) -> Config {
        Config(self.0)
    }

    /// Set the hysteresis bit
    pub const fn set_hysteresis(mut self, hys: Hysteresis) -> Self {
        self.0 = (self.0 & !HYSTERESIS_MASK) | (hys as u32);
        self
    }

    /// Set the pull-up / pull-down value
    pub const fn set_pullupdown(mut self, pud: PullUpDown) -> Self {
        self.0 = (self.0 & !PULLUPDOWN_MASK) | (pud as u32);
        self
    }

    /// Set the the pull-up / pull-down or keeper selection bit
    pub const fn set_pull_keep_select(mut self, pke: PullKeepSelect) -> Self {
        self.0 = (self.0 & !PULL_KEEP_SELECT_MASK) | (pke as u32);
        self
    }

    /// Set the flag that enables the keeper or pull-up / pull-down configuration
    pub const fn set_pull_keep(mut self, pk: PullKeep) -> Self {
        self.0 = (self.0 & !PULLKEEP_MASK) | (pk as u32);
        self
    }

    /// Set the open drain value
    pub const fn set_open_drain(mut self, od: OpenDrain) -> Self {
        self.0 = (self.0 & !OPENDRAIN_MASK) | (od as u32);
        self
    }

    /// Set the pin speed
    pub const fn set_speed(mut self, speed: Speed) -> Self {
        self.0 = (self.0 & !SPEED_MASK) | (speed as u32);
        self
    }

    /// Set the drive strength
    pub const fn set_drive_strength(mut self, dse: DriveStrength) -> Self {
        self.0 = (self.0 & !DRIVE_STRENGTH_MASK) | (dse as u32);
        self
    }

    /// Set the slew rate
    pub const fn set_slew_rate(mut self, sre: SlewRate) -> Self {
        self.0 = (self.0 & !SLEW_RATE_MASK) | (sre as u32);
        self
    }
}
