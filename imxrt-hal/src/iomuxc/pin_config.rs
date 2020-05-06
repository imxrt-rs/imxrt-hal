//! # Pin configuration
//!
//! All GPIOs take a given `PinConfig` that specifies features like
//!
//! - pull up/down resistor connectivity and resistance
//! - pin speed
//! - pin drive strength
//!
//! If the PinConfig does not write to all fields, which is checked
//! by PinConfig::is_modify() the config is applied as a
//! modification to the current PAD_MUX_PAD register value.
//!
//! Otherwise the PAD_MUX_PAD register is overwritten with the new
//! configuration.
//!
//! PinConfig docs supply more information on how to build a
//! PinConfig and what typical defaults from the reference
//! manual look like.
//!
//! # Example using const builder functions
//!
//! ```no_run
//! use imxrt_hal::iomuxc::pin_config::*;
//! use imxrt_hal::Peripherals;
//!
//! const LED_PIN_CONFIG: PinConfig = PinConfig::with_none()
//!                      .set_pull_up(PullUp::PullUp_100KOhm)
//!                      .set_speed(Speed::Speed2_150MHz)
//!                      .set_drive_strength(DriveStrength::R0_DIV_6);
//! let mut peripherals = Peripherals::take().unwrap();
//! peripherals.iomuxc.gpio_ad_b0_00.configure(&LED_PIN_CONFIG);
//! ```

/// localized alias for field specific mods below
mod pad {
    use crate::ral;

    /// All pads support the same configuration; arbitrary choice
    pub use ral::iomuxc::SW_PAD_CTL_PAD_GPIO_EMC_00::*;
}

/// Slew rate mask, offset, and possible values
pub mod slew_rate {
    use super::pad;

    pub const MASK: u32 = pad::SRE::mask;
    pub const OFFSET: u32 = pad::SRE::offset;

    #[repr(u32)]
    pub enum Values {
        Slow = pad::SRE::RW::SRE_0_Slow_Slew_Rate,
        Fast = pad::SRE::RW::SRE_1_Fast_Slew_Rate,
    }
}

pub use slew_rate::Values as SlewRate;

/// Drive strength mask, offset, and possible values
pub mod drive_strength {
    use super::pad;

    pub const MASK: u32 = pad::DSE::mask;
    pub const OFFSET: u32 = pad::DSE::offset;

    #[repr(u32)]
    pub enum Values {
        /// Output driver disabled
        Disabled = pad::DSE::RW::DSE_0_output_driver_disabled,

        /// R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)
        R0 = pad::DSE::RW::DSE_1_R0_150_Ohm_3_3V_260_Ohm_1_8V,

        /// R0/2
        R0_DIV_2 = pad::DSE::RW::DSE_2_R0_2,

        /// R0/3
        R0_DIV_3 = pad::DSE::RW::DSE_3_R0_3,

        /// R0/4
        R0_DIV_4 = pad::DSE::RW::DSE_4_R0_4,

        /// R0/5
        R0_DIV_5 = pad::DSE::RW::DSE_5_R0_5,

        /// R0/6
        R0_DIV_6 = pad::DSE::RW::DSE_6_R0_6,

        /// R0/7
        R0_DIV_7 = pad::DSE::RW::DSE_7_R0_7,
    }
}

pub use drive_strength::Values as DriveStrength;

/// Speed mask, offset, and possible values
pub mod speed {
    use super::pad;

    pub const MASK: u32 = pad::SPEED::mask;
    pub const OFFSET: u32 = pad::SPEED::offset;

    #[repr(u32)]
    pub enum Values {
        Speed0_50MHz = pad::SPEED::RW::SPEED_0_low_50MHz,
        Speed1_100MHz = pad::SPEED::RW::SPEED_1_medium_100MHz,
        //TODO fix svd files for Speed2, ref manual says 150MHz
        Speed2_150MHz = pad::SPEED::RW::SPEED_2_medium_100MHz,
        Speed3_200MHz = pad::SPEED::RW::SPEED_3_max_200MHz,
    }
}

pub use speed::Values as Speed;

/// Open drain mask, offset, and possible values
pub mod open_drain {
    use super::pad;

    pub const MASK: u32 = pad::ODE::mask;
    pub const OFFSET: u32 = pad::ODE::offset;

    #[repr(u32)]
    pub enum Values {
        Disabled = pad::ODE::RW::ODE_0_Open_Drain_Disabled,
        Enabled = pad::ODE::RW::ODE_1_Open_Drain_Enabled,
    }
}

pub use open_drain::Values as OpenDrain;

/// Pull up/down, keeper options
pub mod pull_up {
    use super::pad;

    pub const MASK: u32 = pad::PKE::mask | pad::PUE::mask | pad::PUS::mask;
    pub const OFFSET: u32 = pad::PKE::offset;

    #[repr(u32)]
    pub enum Values {
        Disabled = pad::PKE::RW::PKE_0_Pull_Keeper_Disabled,
        Keeper = pad::PKE::RW::PKE_1_Pull_Keeper_Enabled,
        PullDown_100KOhm = pad::PKE::RW::PKE_1_Pull_Keeper_Enabled
            | (pad::PUE::RW::PUE_1_Pull << 1)
            | (pad::PUS::RW::PUS_0_100K_Ohm_Pull_Down << 2),
        PullUp_47KOhm = pad::PKE::RW::PKE_1_Pull_Keeper_Enabled
            | (pad::PUE::RW::PUE_1_Pull << 1)
            | (pad::PUS::RW::PUS_1_47K_Ohm_Pull_Up << 2),
        PullUp_100KOhm = pad::PKE::RW::PKE_1_Pull_Keeper_Enabled
            | (pad::PUE::RW::PUE_1_Pull << 1)
            | (pad::PUS::RW::PUS_2_100K_Ohm_Pull_Up << 2),
        PullUp_22KOhm = pad::PKE::RW::PKE_1_Pull_Keeper_Enabled
            | (pad::PUE::RW::PUE_1_Pull << 1)
            | (pad::PUS::RW::PUS_3_22K_Ohm_Pull_Up << 2),
    }
}

pub use pull_up::Values as PullUp;

/// Hysterisis mask, offset, and possible values
pub mod hysterisis {
    use super::pad;

    pub const MASK: u32 = pad::HYS::mask;
    pub const OFFSET: u32 = pad::HYS::offset;

    #[repr(u32)]
    pub enum Values {
        Disabled = pad::HYS::RW::HYS_0_Hysteresis_Disabled,
        Enabled = pad::HYS::RW::HYS_1_Hysteresis_Enabled,
    }
}

pub use hysterisis::Values as Hysterisis;

/// Pin Config
///
/// A Builder pattern struct to generate a PinConfig that can then be used by
/// the Pad to adjust the PAD_CTL_PAD register in as few operations as possible
///
/// Pin config defaults vary and are likely one of the following, described
/// here for ease of translating to PinConfig constructors
///
/// The binary values are listed in parens and are split with
/// spaces for each field.
///
/// ie: (hys pus pue pke xxx speed dse xx sre)
///
/// # 0000_10B0h (1 0 000 10 110 00 0)
/// ```
/// use imxrt_hal::iomuxc::pin_config::*;
///
/// const pin_cfg: PinConfig = PinConfig::with_all(
///     SlewRate::Slow,
///     DriveStrength::R0_DIV_6,
///     Speed::Speed2_150MHz,
///     OpenDrain::Disabled,
///     PullUp::Keeper,
///     Hysterisis::Disabled,
/// );
/// assert_eq!(pin_cfg.value, 0x000010B0)
/// ```
///
///
/// # 0000_30B0h (1 1 0 000 10 110 00 0)
/// ```
/// use imxrt_hal::iomuxc::pin_config::*;
///
/// const pin_cfg: PinConfig = PinConfig::with_all(
///     SlewRate::Slow,
///     DriveStrength::R0_DIV_6,
///     Speed::Speed2_150MHz,
///     OpenDrain::Disabled,
///     PullUp::PullDown_100KOhm,
///     Hysterisis::Disabled,
/// );
/// assert_eq!(pin_cfg.value, 0x000030B0)
/// ```
///
/// # 0000_70A0h (1 1 1 0 000 10 100 00 0)
/// ```
/// use imxrt_hal::iomuxc::pin_config::*;
/// const pin_cfg: PinConfig = PinConfig::with_all(
///     SlewRate::Slow,
///     DriveStrength::R0_DIV_4,
///     Speed::Speed2_150MHz,
///     OpenDrain::Disabled,
///     PullUp::PullUp_47KOhm,
///     Hysterisis::Disabled,
/// );
/// assert_eq!(pin_cfg.value, 0x000070A0)
/// ```
///
/// # 0000_90B1h (10 0 1 0 000 10 110 00 1)
///
/// This is a bit of an odd ball, as the default pull up field (PUS)
/// is set to the 100KOhm setting, while the pull up enable field (PUE)
/// is disabled. Which according to the reference manual means the pull up
/// selected is going to do nothing at all.
///
/// While functionally the same, the exact value isn't possible with the
/// convienence of the PullUp enum provided here.
///
/// Below is the equivalent functionally, while verifying slightly differently
/// to be equal to 0x000010B1
///
/// ```
/// use imxrt_hal::iomuxc::pin_config::*;
/// const pin_cfg: PinConfig = PinConfig::with_all(
///     SlewRate::Fast,
///     DriveStrength::R0_DIV_6,
///     Speed::Speed2_150MHz,
///     OpenDrain::Disabled,
///     PullUp::Keeper,
///     Hysterisis::Disabled,
/// );
/// assert_eq!(pin_cfg.value, 0x000010B1)
/// ```
///
/// A const pin config value may also be built in a way to not overwrite
/// all values of the current pin config. This can be useful when looking
/// at existing C code and translating to Rust as often not all fields are set.
///
/// This is a less preferred way, and one that isn't accepted in the HAL
/// itself, but still useful potentially in application code.
///
/// As an example setting the pin to drive the LED on the IMXRT1060EVK
/// based on code in Zephyr's board setup
///
/// ```
/// use imxrt_hal::iomuxc::pin_config::*;
/// const pin_cfg: PinConfig = PinConfig::with_none()
///     .set_speed(Speed::Speed2_150MHz)
///     .set_pull_up(PullUp::Keeper)
///     .set_drive_strength(DriveStrength::R0_DIV_6);
/// assert_eq!(pin_cfg.mask, speed::MASK | pull_up::MASK |  drive_strength::MASK);
/// ```
pub struct PinConfig {
    pub mask: u32,
    pub value: u32,
}

// Generate a builder like impl fn for PinConfig for a field
macro_rules! pin_config_setter_impl {
    ( $self:ident, $field:ident, $value:ident ) => {
        PinConfig {
            mask: $self.mask | $field::MASK,
            value: ($self.value & !$field::MASK)
                | ((($value as u32) << $field::OFFSET) & $field::MASK),
        }
    };
}

impl PinConfig {
    /// Create a new, empty PinConfig with no options set
    ///
    /// This allows using the pins default settings only overriding
    /// fields set by the builder like set const functions supplied here
    ///
    /// Its possible that with_none() followed by a series of const fn calls to
    /// set specified fields will be elided at compile time.
    pub const fn with_none() -> PinConfig {
        PinConfig { mask: 0, value: 0 }
    }

    /// Create a new PinConfig with all options explicitly being set.
    ///
    /// This being a const constructor the resulting value is const
    /// and may elide to being created at compile time which is a
    /// very efficient way of creating commonly used PinConfig values.
    pub const fn with_all(
        slew_rate: SlewRate,
        drive_strength: DriveStrength,
        speed: Speed,
        open_drain: OpenDrain,
        pull_up: PullUp,
        hysterisis: Hysterisis,
    ) -> PinConfig {
        PinConfig {
            mask: slew_rate::MASK
                | drive_strength::MASK
                | speed::MASK
                | open_drain::MASK
                | pull_up::MASK
                | hysterisis::MASK,
            value: ((slew_rate as u32) << slew_rate::OFFSET)
                | ((drive_strength as u32) << drive_strength::OFFSET)
                | ((speed as u32) << speed::OFFSET)
                | ((open_drain as u32) << open_drain::OFFSET)
                | ((pull_up as u32) << pull_up::OFFSET)
                | ((hysterisis as u32) << hysterisis::OFFSET),
        }
    }

    /// Reset the pin config to make it reusable
    pub fn reset(&mut self) {
        self.mask = 0;
        self.value = 0;
    }

    /// Determines if the pin config should use a modify
    /// rather than a write operation on the register
    pub fn is_modify(&self) -> bool {
        const WRITE_MASK: u32 = slew_rate::MASK
            | drive_strength::MASK
            | speed::MASK
            | open_drain::MASK
            | pull_up::MASK
            | hysterisis::MASK;
        self.mask != WRITE_MASK
    }

    /// Modify an existing given register value with pin config settings
    pub(crate) fn modify(&self, reg: u32) -> u32 {
        (reg & !self.mask) | (self.value & self.mask)
    }

    /// Set the slew rate of the Pin
    ///
    /// This bitfield controls how fast the pin toggles between the two logic
    /// states. Since rapidly changing states consume more power and generate
    /// spikes, it should be enabled only when necessary. The operational
    /// frequency on GPIO pads is dependent on slew rate (SRE), speed (SPEED),
    /// and supply voltage (OVDD). See Operating Frequency table in the GPIO
    /// block guide for more details.
    pub const fn set_slew_rate(self, value: SlewRate) -> Self {
        pin_config_setter_impl!(self, slew_rate, value)
    }

    /// Set the drive strength
    ///
    /// The drive strength enable (DSE) can be explained as series resistance
    /// between an ideal driver’s output and its load. To achieve maximal
    /// transferred power, the impedance of the driver has to match the load
    /// impedance.
    ///
    /// The reference manual has varying information depending on the Pin.
    ///
    /// Most GPIO pins specify R0 as 150 Ohm @ 3.3V and 260 Ohm @ 1.8V
    ///
    /// There is then the option to divide R0 by a given divider such as
    /// R0_DIV_2 to attempt to best match the load impedance.
    pub const fn set_drive_strength(self, value: DriveStrength) -> Self {
        pin_config_setter_impl!(self, drive_strength, value)
    }

    /// Set the speed of the pin
    ///
    /// Speed is a selectable bit field that sets electrical characteristics of
    /// a pin in a given frequency range. This field provides additional 2-bit
    /// slew rate control. These options can either increase the output driver
    /// current in the higher frequency range, or reduce the switching noise in
    /// the lower frequency range.
    /// The operational frequency on GPIO pads is dependent on slew rate (SRE),
    /// speed (SPEED), and supply voltage (OVDD).
    ///
    /// See Operating Frequency table in the GPIO block guide in the reference
    /// manual for more details.
    pub const fn set_speed(self, value: Speed) -> Self {
        pin_config_setter_impl!(self, speed, value)
    }

    /// Set the pin as open drain or not
    ///
    /// If set to true, the output driver drives only logic 0. The drain of the
    /// internal transistor is open. It means that logic 1 has to be driven by
    /// an external component. This option is essential if connection between
    /// the pad and an external component is bi-directional. If ODE = 0, then
    /// the output driver drives logic 1 and logic 0.
    pub const fn set_open_drain(self, value: OpenDrain) -> Self {
        pin_config_setter_impl!(self, open_drain, value)
    }

    /// Set the pin pull up options
    ///
    /// Disabled no pull up or output keeper is enabled
    /// Keeper an output value is kept when the output driver is disabled
    /// The remaining options setup a pull up or down resistor with a specified
    /// value.
    pub const fn set_pull_up(self, value: PullUp) -> Self {
        pin_config_setter_impl!(self, pull_up, value)
    }

    /// Set hysterisis
    ///
    /// The hysteresis (HYS) bit controls whether a pin acts as a Schmitt
    /// trigger, which is a comparator remembering its last input state
    /// (hysteresis).
    pub const fn set_hysterisis(self, value: Hysterisis) -> Self {
        pin_config_setter_impl!(self, hysterisis, value)
    }
}
