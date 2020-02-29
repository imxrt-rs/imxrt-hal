//! Clock Configuration Module (CCM)

mod arm_clock;
use arm_clock::set_arm_clock;

use core::time::Duration;
use imxrt_ral as ral;

pub struct Handle {
    pub(crate) base: ral::ccm::Instance,
    pub(crate) analog: ral::ccm_analog::Instance,
}

impl Handle {
    pub fn raw(&mut self) -> (&ral::ccm::Instance, &ral::ccm_analog::Instance) {
        (&self.base, &self.analog)
    }
}

pub struct CCM {
    pub handle: Handle,
    pub perclk: perclk::Multiplexer,
    /// ARM PLL, providing typical functioning frequency
    pub pll1: PLL1,
    /// The 480 MHz PFD
    pub pll2: pll2::PFD,
    /// The 528 MHz PFD
    pub pll3: pll3::PFD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmFrequency(Frequency);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPGFrequency(pub(crate) Frequency);

pub struct PLL1(());
impl PLL1 {
    fn new() -> Self {
        PLL1(())
    }

    #[cfg(any(feature = "imxrt1011", feature = "imxrt1015"))]
    pub const ARM_HZ: u32 = 500_000_000;

    #[cfg(any(feature = "imxrt1064", feature = "imxrt1062", feature = "imxrt1061"))]
    pub const ARM_HZ: u32 = 600_000_000;

    /// Set the clock speed for the ARM core. This represents the base processor frequency.
    /// Consider using the 600MHz recommended frequency `PLL1::ARM_HZ`.
    pub fn set_arm_clock(
        &mut self,
        hz: u32,
        handle: &mut Handle,
        dcdc: &mut crate::dcdc::DCDC,
    ) -> (ArmFrequency, IPGFrequency) {
        let (ccm, ccm_analog) = handle.raw();
        let dcdc = dcdc.raw();
        let (arm_freq, ipg_freq) = set_arm_clock(hz, ccm, ccm_analog, dcdc);
        (
            ArmFrequency(Frequency(arm_freq)),
            IPGFrequency(Frequency(ipg_freq)),
        )
    }
}

impl CCM {
    pub(crate) fn new(base: ral::ccm::Instance, analog: ral::ccm_analog::Instance) -> Self {
        CCM {
            handle: Handle { base, analog },
            perclk: perclk::Multiplexer::new(),
            pll1: PLL1::new(),
            pll2: pll2::PFD::new(),
            pll3: pll3::PFD::new(),
        }
    }
}

pub mod perclk {
    use super::{ral, Divider, Frequency, Handle, OSCILLATOR_FREQUENCY};

    use ral::{ccm::CSCMR1::PERCLK_CLK_SEL, modify_reg};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum CLKSEL {
        /// 24MHz oscillator
        OSC,
        /// IPG
        IPG(super::IPGFrequency),
    }

    impl CLKSEL {
        fn to_perclk_clk_sel(self) -> u32 {
            match self {
                CLKSEL::OSC => PERCLK_CLK_SEL::RW::PERCLK_CLK_SEL_1,
                CLKSEL::IPG(_) => PERCLK_CLK_SEL::RW::PERCLK_CLK_SEL_0,
            }
        }
    }

    pub struct Multiplexer;
    pub struct Configured<'a> {
        handle: &'a mut Handle,
        divider: Divider,
        clock_hz: Frequency,
    }

    impl Multiplexer {
        pub(super) fn new() -> Self {
            Multiplexer
        }

        pub fn configure(self, handle: &mut Handle, podf: u8, clksel: CLKSEL) -> Configured {
            modify_reg!(ral::ccm, handle.base, CSCMR1, PERCLK_CLK_SEL: clksel.to_perclk_clk_sel());
            Configured {
                handle,
                divider: Divider::from(podf),
                clock_hz: Frequency::from(clksel),
            }
        }
    }

    impl<'a> Configured<'a> {
        pub(crate) fn enable(self) -> (Frequency, Divider) {
            modify_reg!(ral::ccm, self.handle.base, CCGR1, CG6: 0x3);
            (self.clock_hz, self.divider)
        }
    }

    impl From<CLKSEL> for Frequency {
        fn from(clksel: CLKSEL) -> Frequency {
            match clksel {
                // 24MHz oscillator
                CLKSEL::OSC => OSCILLATOR_FREQUENCY,
                CLKSEL::IPG(ipg_freq) => ipg_freq.0,
            }
        }
    }

    impl From<u8> for Divider {
        fn from(podf: u8) -> Divider {
            Divider((podf + 1) as u32)
        }
    }
}

macro_rules! pfd {
    ($setter:ident, $value:ident) => {
        use super::Handle;
        use imxrt_ral::{self as ral, write_reg};

        fn pfd_gate_frac(pfd: &Option<Frequency>) -> (u32, u32) {
            match pfd {
                Some(Frequency(freq)) => (1, (*freq).into()),
                None => (0, 0),
            }
        }

        pub struct PFD;
        impl PFD {
            pub(super) fn new() -> Self {
                PFD
            }

            pub fn set(&mut self, handle: &mut Handle, pfds: [Option<Frequency>; 4]) {
                let (pfd0_clkgate, pfd0_frac) = pfd_gate_frac(&pfds[0]);
                let (pfd1_clkgate, pfd1_frac) = pfd_gate_frac(&pfds[1]);
                let (pfd2_clkgate, pfd2_frac) = pfd_gate_frac(&pfds[2]);
                let (pfd3_clkgate, pfd3_frac) = pfd_gate_frac(&pfds[3]);

                write_reg!(
                    ral::ccm_analog,
                    handle.analog,
                    $setter,
                    PFD0_CLKGATE: pfd0_clkgate,
                    PFD1_CLKGATE: pfd1_clkgate,
                    PFD2_CLKGATE: pfd2_clkgate,
                    PFD3_CLKGATE: pfd3_clkgate
                );

                // Safety: PDFx_FRAC is 6 bits wide. By the implementations
                // of the `Frequency(..)` newtypes, the wrapped values will
                // never exceed a 6 bit value.
                write_reg!(
                    ral::ccm_analog,
                    handle.analog,
                    $value,
                    PFD0_FRAC: pfd0_frac,
                    PFD1_FRAC: pfd1_frac,
                    PFD2_FRAC: pfd2_frac,
                    PFD3_FRAC: pfd3_frac
                );
            }
        }
    };
}

/// 480 MHz phase fractional divider
pub mod pll3 {

    pfd!(PFD_480_SET, PFD_480);

    pub struct Frequency(u8);

    pub const MHZ_720: Frequency = Frequency(12);
    pub const MHZ_664: Frequency = Frequency(13);
    pub const MHZ_617: Frequency = Frequency(14);
    pub const MHZ_576: Frequency = Frequency(15);
    pub const MHZ_540: Frequency = Frequency(16);
    pub const MHZ_508: Frequency = Frequency(17);
    pub const MHZ_480: Frequency = Frequency(18);
    pub const MHZ_454: Frequency = Frequency(19);
    pub const MHZ_432: Frequency = Frequency(20);
    pub const MHZ_411: Frequency = Frequency(21);
    pub const MHZ_392: Frequency = Frequency(22);
    pub const MHZ_375: Frequency = Frequency(23);
    pub const MHZ_360: Frequency = Frequency(24);
    pub const MHZ_345: Frequency = Frequency(25);
    pub const MHZ_332: Frequency = Frequency(26);
    pub const MHZ_320: Frequency = Frequency(27);
    pub const MHZ_308: Frequency = Frequency(28);
    pub const MHZ_297: Frequency = Frequency(29);
    pub const MHZ_288: Frequency = Frequency(30);
    pub const MHZ_278: Frequency = Frequency(31);
    pub const MHZ_270: Frequency = Frequency(32);
    pub const MHZ_261: Frequency = Frequency(33);
    pub const MHZ_254: Frequency = Frequency(34);
    pub const MHZ_246: Frequency = Frequency(35);
}

/// 528 MHz phase fractional divider
pub mod pll2 {
    pfd!(PFD_528_SET, PFD_528);

    pub struct Frequency(u8);

    pub const MHZ_792: Frequency = Frequency(12);
    pub const MHZ_731: Frequency = Frequency(13);
    pub const MHZ_678: Frequency = Frequency(14);
    pub const MHZ_633: Frequency = Frequency(15);
    pub const MHZ_594: Frequency = Frequency(16);
    pub const MHZ_559: Frequency = Frequency(17);
    pub const MHZ_528: Frequency = Frequency(18);
    pub const MHZ_500: Frequency = Frequency(19);
    pub const MHZ_475: Frequency = Frequency(20);
    pub const MHZ_452: Frequency = Frequency(21);
    pub const MHZ_432: Frequency = Frequency(22);
    pub const MHZ_413: Frequency = Frequency(23);
    pub const MHZ_396: Frequency = Frequency(24);
    pub const MHZ_380: Frequency = Frequency(25);
    pub const MHZ_365: Frequency = Frequency(26);
    pub const MHZ_352: Frequency = Frequency(27);
    pub const MHZ_339: Frequency = Frequency(28);
    pub const MHZ_327: Frequency = Frequency(29);
    pub const MHZ_316: Frequency = Frequency(30);
    pub const MHZ_306: Frequency = Frequency(31);
    pub const MHZ_297: Frequency = Frequency(32);
    pub const MHZ_288: Frequency = Frequency(33);
    pub const MHZ_279: Frequency = Frequency(34);
    pub const MHZ_271: Frequency = Frequency(35);
}

use core::convert::TryFrom;
pub trait TicksRepr: TryFrom<u64> {}
impl TicksRepr for u8 {}
impl TicksRepr for u16 {}
impl TicksRepr for u32 {}
impl TicksRepr for u64 {}

/// An opaque duration representing the number of clock ticks
///
/// See the `ticks` function to derive a `Ticks` value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks<R: TicksRepr>(pub(crate) R);

/// Possible errors that could result during a computation of `ticks`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TicksError {
    /// The duration cannot be expressed in a `u64`.
    DurationOverflow,
    /// The number of ticks cannot be expressed in a `u32`
    TicksOverflow,
    /// Computation would divide by zero
    DivideByZero,
}

/// Computes the number of clock ticks that span the provide duration, given
/// the clock frequency and clock divider. If there is no divider, use `Divider::default()`
/// to specify an unused divider. Returns `Ok(ticks)` when the computation of
/// clock ticks succeeds, or an error.
pub fn ticks<R: TicksRepr>(
    dur: Duration,
    freq: Frequency,
    div: Divider,
) -> Result<Ticks<R>, TicksError> {
    // Ticks computed as
    //
    //  ticks = (duration / clock_period) - 1
    //
    // where `clock_period` is the effective clock period: `freq / div`
    let delay_ns = u64::try_from(dur.as_nanos()).map_err(|_| TicksError::DurationOverflow)?;
    let effective_freq = freq
        .0
        .checked_div(div.0)
        .ok_or(TicksError::DurationOverflow)?;
    let clock_period_ns = 1_000_000_000u32
        .checked_div(effective_freq)
        .map(u64::from)
        .ok_or(TicksError::DivideByZero)?;
    delay_ns
        .checked_div(clock_period_ns)
        .and_then(|ticks| ticks.checked_sub(1))
        .and_then(|ticks| R::try_from(ticks).ok())
        .map(Ticks)
        .ok_or(TicksError::TicksOverflow)
}

/// An opaque value representing a clock frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequency(u32);

impl From<Frequency> for core::time::Duration {
    fn from(hz: Frequency) -> core::time::Duration {
        core::time::Duration::from_nanos((1_000_000_000u32 / hz.0).into())
    }
}

impl From<Frequency> for Ticks<u32> {
    fn from(hz: Frequency) -> Ticks<u32> {
        Ticks(hz.0)
    }
}

impl core::ops::Add for Ticks<u32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Ticks(self.0 + rhs.0)
    }
}

impl core::ops::Div<Divider> for Ticks<u32> {
    type Output = Self;
    fn div(self, rhs: Divider) -> Self {
        Ticks(self.0 / rhs.0)
    }
}

impl core::ops::Div for Ticks<u32> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Ticks(self.0 / rhs.0)
    }
}

impl core::ops::Sub for Ticks<u32> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Ticks(self.0 - rhs.0)
    }
}

/// An opaque value representing a clock phase divider
#[derive(Debug, Clone, Copy)]
pub struct Divider(u32);

impl Default for Divider {
    fn default() -> Divider {
        Divider(1)
    }
}

/// High speed oscillator frequency
const OSCILLATOR_FREQUENCY: Frequency = Frequency(24_000_000 /* 24MHz */);

impl core::ops::Div<Divider> for Frequency {
    type Output = Frequency;

    fn div(self, rhs: Divider) -> Frequency {
        Frequency(self.0 / rhs.0)
    }
}

impl core::ops::DivAssign<Divider> for Frequency {
    fn div_assign(&mut self, rhs: Divider) {
        self.0 /= rhs.0;
    }
}

/*
/// Timing configurations for PWM
pub mod pwm {
    use super::{pac::pwm1, Divider, Frequency, IPGFrequency};

    /// PWM submodule clock selection
    #[derive(Clone, Copy)]
    #[non_exhaustive] // not all variants are added
    pub enum ClockSelect {
        /// IPG clock frequency, available via `set_arm_clock`
        IPG(IPGFrequency),
    }

    /// PWM prescalar
    pub type Prescalar = pwm1::sm::smctrl::PRSC_A;

    impl From<Prescalar> for Divider {
        fn from(pre: Prescalar) -> Divider {
            Divider(1u32 << u8::from(pre))
        }
    }

    impl From<ClockSelect> for Frequency {
        fn from(clksel: ClockSelect) -> Frequency {
            match clksel {
                ClockSelect::IPG(IPGFrequency(hz)) => hz,
            }
        }
    }

    impl From<ClockSelect> for pwm1::sm::smctrl2::CLK_SEL_A {
        fn from(clksel: ClockSelect) -> Self {
            match clksel {
                ClockSelect::IPG(_) => pwm1::sm::smctrl2::CLK_SEL_A::CLK_SEL_0,
            }
        }
    }
}
*/

/// Timing configurations for I2C peripherals
pub mod i2c {
    use super::{Divider, Frequency, OSCILLATOR_FREQUENCY};
    use crate::ral;

    /// Clock selection for all I2C peripherals
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    #[non_exhaustive] // Not all variants added
    pub enum ClockSelect {
        /// Derive clock from oscillator
        OSC = ral::ccm::CSCDR2::LPI2C_CLK_SEL::RW::LPI2C_CLK_SEL_1,
    }

    /// Prescalar selection for all I2C input clocks
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    #[allow(non_camel_case_types)] // Easier mapping if the names are consistent
    pub enum PrescalarSelect {
        DIVIDE_1 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_1,
        DIVIDE_2 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_2,
        DIVIDE_3 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_3,
        DIVIDE_4 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_4,
        DIVIDE_5 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_5,
        DIVIDE_6 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_6,
        DIVIDE_7 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_7,
        DIVIDE_8 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_8,
        DIVIDE_9 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_9,
        DIVIDE_10 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_10,
        DIVIDE_11 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_11,
        DIVIDE_12 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_12,
        DIVIDE_13 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_13,
        DIVIDE_14 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_14,
        DIVIDE_15 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_15,
        DIVIDE_16 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_16,
        DIVIDE_17 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_17,
        DIVIDE_18 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_18,
        DIVIDE_19 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_19,
        DIVIDE_20 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_20,
        DIVIDE_21 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_21,
        DIVIDE_22 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_22,
        DIVIDE_23 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_23,
        DIVIDE_24 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_24,
        DIVIDE_25 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_25,
        DIVIDE_26 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_26,
        DIVIDE_27 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_27,
        DIVIDE_28 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_28,
        DIVIDE_29 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_29,
        DIVIDE_30 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_30,
        DIVIDE_31 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_31,
        DIVIDE_32 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_32,
        DIVIDE_33 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_33,
        DIVIDE_34 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_34,
        DIVIDE_35 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_35,
        DIVIDE_36 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_36,
        DIVIDE_37 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_37,
        DIVIDE_38 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_38,
        DIVIDE_39 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_39,
        DIVIDE_40 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_40,
        DIVIDE_41 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_41,
        DIVIDE_42 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_42,
        DIVIDE_43 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_43,
        DIVIDE_44 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_44,
        DIVIDE_45 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_45,
        DIVIDE_46 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_46,
        DIVIDE_47 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_47,
        DIVIDE_48 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_48,
        DIVIDE_49 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_49,
        DIVIDE_50 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_50,
        DIVIDE_51 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_51,
        DIVIDE_52 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_52,
        DIVIDE_53 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_53,
        DIVIDE_54 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_54,
        DIVIDE_55 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_55,
        DIVIDE_56 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_56,
        DIVIDE_57 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_57,
        DIVIDE_58 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_58,
        DIVIDE_59 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_59,
        DIVIDE_60 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_60,
        DIVIDE_61 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_61,
        DIVIDE_62 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_62,
        DIVIDE_63 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_63,
        DIVIDE_64 = ral::ccm::CSCDR2::LPI2C_CLK_PODF::RW::DIVIDE_64,
    }

    impl From<ClockSelect> for Frequency {
        fn from(clock_select: ClockSelect) -> Self {
            match clock_select {
                ClockSelect::OSC => OSCILLATOR_FREQUENCY,
            }
        }
    }

    impl From<PrescalarSelect> for Divider {
        fn from(prescalar_select: PrescalarSelect) -> Self {
            Divider((prescalar_select as u32) + 1)
        }
    }
}

/*
pub mod uart {
    use super::{pac::ccm, Divider, Frequency, OSCILLATOR_FREQUENCY};

    #[derive(Clone, Copy)]
    #[non_exhaustive] // Not all variants added
    pub enum ClockSelect {
        /// Oscillator clock
        OSC,
    }

    impl From<ClockSelect> for ccm::cscdr1::UART_CLK_SEL_A {
        fn from(clock_select: ClockSelect) -> Self {
            match clock_select {
                ClockSelect::OSC => ccm::cscdr1::UART_CLK_SEL_A::UART_CLK_SEL_1,
            }
        }
    }

    pub type PrescalarSelect = ccm::cscdr1::UART_CLK_PODF_A;

    impl From<PrescalarSelect> for Divider {
        fn from(prescale: PrescalarSelect) -> Divider {
            Divider((u8::from(prescale) as u32) + 1)
        }
    }

    impl From<ClockSelect> for Frequency {
        fn from(clock_select: ClockSelect) -> Self {
            match clock_select {
                ClockSelect::OSC => OSCILLATOR_FREQUENCY,
            }
        }
    }

    /// An opaque type that describes timing configurations
    pub struct Timings {
        /// OSR register value. Accounts for the -1. May be written
        /// directly to the register
        pub(crate) osr: u8,
        /// True if we need to set BOTHEDGE given the OSR value
        pub(crate) both_edge: bool,
        /// SBR value;
        pub(crate) sbr: u16,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum TimingsError {
        DivideByZero,
        OutOfRange,
    }

    /// Compute timings for a UART peripheral. Returns the timings,
    /// or a string describing an error.
    pub(crate) fn timings(effective_clock: Frequency, baud: u32) -> Result<Timings, TimingsError> {
        let effective_clock = effective_clock.0;

        //        effective_clock
        // baud = ---------------
        //         (OSR+1)(SBR)
        //
        // Solve for SBR:
        //
        //       effective_clock
        // SBR = ---------------
        //        (OSR+1)(baud)
        //
        // After selecting SBR, calculate effective baud.
        // Minimize the error over all OSRs.

        let base_clock: u32 = effective_clock
            .checked_div(baud)
            .ok_or(TimingsError::DivideByZero)?;
        let mut error = u32::max_value();
        let mut best_osr = 16;
        let mut best_sbr = 1;

        for osr in 4..=32 {
            let sbr = base_clock
                .checked_div(osr)
                .ok_or(TimingsError::DivideByZero)?;
            let sbr = sbr.max(1).min(8191);
            let effective_baud = effective_clock
                .checked_div(osr * sbr)
                .ok_or(TimingsError::DivideByZero)?;
            let err = effective_baud.max(baud) - effective_baud.min(baud);
            if err < error {
                best_osr = osr;
                best_sbr = sbr;
                error = err
            }
        }

        use core::convert::TryFrom;
        Ok(Timings {
            osr: u8::try_from(best_osr - 1).map_err(|_| TimingsError::OutOfRange)?,
            sbr: u16::try_from(best_sbr).map_err(|_| TimingsError::OutOfRange)?,
            both_edge: best_osr < 8,
        })
    }
}
*/
