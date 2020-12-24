//! ADC
//!
//! This ADC driver supports `embedded_hal`'s ADC traits
//!
//! # Example
//!
//! ```no_run
//! use imxrt1060_hal as hal;
//! use hal::{adc, ral};
//! use embedded_hal::adc::OneShot;
//!
//! let mut pads = ral::iomuxc::IOMUXC::take()
//!     .map(hal::iomuxc::new)
//!     .unwrap();
//!
//! let mut adc1 = ral::adc::ADC1::take()
//!     .map(|inst| adc::ADC::new(inst, Default::default(), Default::default()))
//!     .unwrap();
//! let mut a1 = adc::AnalogInput::new(pads.ad_b1.p02);
//!
//! let reading: u16 = adc1.read(&mut a1).unwrap();
//! ```
//!
//! The ADC starts out with a default configuration of 4 hardware samples, a conversion speed of
//! medium, a resolution of 10 bits, and low power mode disabled. It's also pre-calibrated using
//! 32 averages and a slow conversion speed.

use crate::iomuxc::adc::{prepare, Pin};
use crate::iomuxc::{
    adc,
    consts::{Unsigned, U1, U2},
};
use crate::ral;
use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};

/// The clock input for an ADC
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSelect {
    /// IPG clock
    IPG,
    /// IPG clock / 2
    IPG_2,
    /// ADC Asynchronous clock
    ADACK,
}

/// How much to divide the clock input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockDivision {
    Div1, // Input clock / 1
    Div2, // Input clock / 2
    Div4, // Input clock / 4
    Div8, // Input clock / 8
}

impl Default for ClockSelect {
    fn default() -> Self {
        ClockSelect::ADACK
    }
}

impl Default for ClockDivision {
    fn default() -> Self {
        ClockDivision::Div2
    }
}

/// Conversion speeds done by clock cycles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionSpeed {
    Slow,     // 25 ADC clock cycles (24 on imxrt102x)
    Medium,   // 17 ADC clock cycles (16 on imxrt102x)
    Fast,     // 9 ADC clock cycles (8 on imxrt102x)
    VeryFast, // 3 ADC clock cycles (2 on imxrt102x)
}

/// Denotes how much hardware averaging to do
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AveragingCount {
    Avg1,
    Avg4,
    Avg8,
    Avg16,
    Avg32,
}

// Specifies the resolution the ADC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionBits {
    Res8,
    Res10,
    Res12,
}

/// A pin representing an analog input for a particular ADC
pub struct AnalogInput<ADCx, P> {
    _module: PhantomData<ADCx>,
    pin: P,
}

impl<P, ADCx> Channel<ADCx> for AnalogInput<ADCx, P>
where
    P: Pin<ADCx>,
    ADCx: adc::ADC,
{
    type ID = u16;

    fn channel() -> Self::ID {
        <P as Pin<ADCx>>::Input::U16
    }
}

impl<P, ADCx> AnalogInput<ADCx, P>
where
    P: Pin<ADCx>,
    ADCx: adc::ADC,
{
    /// Creates a new analog input pin
    pub fn new(mut pin: P) -> Self {
        prepare(&mut pin);
        Self {
            _module: PhantomData,
            pin,
        }
    }

    /// Release the ADC input, returning the underlying hardware pin. This pin is in an
    /// unspecified state
    pub fn release(self) -> P {
        self.pin
    }
}

pub struct ADC<N> {
    reg: ral::adc::Instance<N>,
}

impl<N> ADC<N> {
    /// Constuct an ADC from a RAL ADC instance
    pub fn new(reg: ral::adc::Instance<N>, clock: ClockSelect, division: ClockDivision) -> Self {
        // Enable asynchronous clock if applicable
        ral::modify_reg!(ral::adc, reg, GC, ADACKEN: match clock {
            ClockSelect::ADACK => ADACKEN_1,
            _ => ADACKEN_0
        });

        // Select the clock selection, division, and enable ADHSC if applicable
        ral::modify_reg!(ral::adc, reg, CFG,
            ADICLK: match clock {
                ClockSelect::IPG => ADICLK_0,
                ClockSelect::IPG_2 => ADICLK_1,
                ClockSelect::ADACK => ADICLK_3
            },
            ADIV: match division {
                ClockDivision::Div1 => ADIV_0,
                ClockDivision::Div2 => ADIV_1,
                ClockDivision::Div4 => ADIV_2,
                ClockDivision::Div8 => ADIV_3
            },
            ADHSC: ADHSC_1
        );

        let mut inst = Self { reg };

        inst.set_resolution(ResolutionBits::Res10);
        inst.set_low_power_mode(false);

        // Calibrate w/ slow settings initially
        inst.set_averaging(AveragingCount::Avg32);
        inst.set_conversion_speed(ConversionSpeed::Slow);
        inst.calibrate();

        // Set to default of 4 hardware averages & medium conversion speed
        inst.set_averaging(AveragingCount::Avg4);
        inst.set_conversion_speed(ConversionSpeed::Medium);

        inst
    }

    /// Sets the resolution that analog reads return, in bits
    pub fn set_resolution(&mut self, bits: ResolutionBits) {
        ral::modify_reg!(ral::adc, self.reg, CFG, MODE: match bits {
            ResolutionBits::Res8 => MODE_0,
            ResolutionBits::Res10 => MODE_1,
            ResolutionBits::Res12 => MODE_2
        });
    }

    /// Sets the number of hardware averages taken by the ADC
    pub fn set_averaging(&mut self, avg: AveragingCount) {
        ral::modify_reg!(ral::adc, self.reg, GC, AVGE: match avg {
            AveragingCount::Avg1 => AVGE_0,
            _ => AVGE_1
        });
        ral::modify_reg!(ral::adc, self.reg, CFG, AVGS: match avg {
            AveragingCount::Avg32 => AVGS_3,
            AveragingCount::Avg16 => AVGS_2,
            AveragingCount::Avg8 => AVGS_1,
            _ => AVGS_0,
        });
    }

    /// Sets the conversion speed for this ADC, see ConversionSpeed for clock cycle counts.
    pub fn set_conversion_speed(&mut self, conversion_speed: ConversionSpeed) {
        ral::modify_reg!(ral::adc, self.reg, CFG,
            ADSTS: match conversion_speed {
                ConversionSpeed::Slow => ADSTS_3,
                ConversionSpeed::Medium => ADSTS_1,
                ConversionSpeed::Fast => ADSTS_3,
                ConversionSpeed::VeryFast => ADSTS_0
            },
            ADLSMP: match conversion_speed {
                ConversionSpeed::Slow => ADLSMP_1,
                ConversionSpeed::Medium => ADLSMP_1,
                ConversionSpeed::Fast => ADLSMP_0,
                ConversionSpeed::VeryFast => ADLSMP_0
            }
        );
    }

    /// Enables or disables the low power configuration in the ADC. This does limit the
    /// ADACK clock frequency (<= 20MHz)
    pub fn set_low_power_mode(&mut self, state: bool) {
        ral::modify_reg!(ral::adc, self.reg, CFG, ADLPC: if state { ADLPC_1 } else { ADLPC_0 });
    }

    /// Calibrates the ADC, will wait for finish
    pub fn calibrate(&mut self) {
        ral::modify_reg!(ral::adc, self.reg, GC, CAL: 0b1);
        while (ral::read_reg!(ral::adc, self.reg, CAL, CAL_CODE) != 0) {}
    }
}

impl<WORD, P> OneShot<adc::ADC1, WORD, AnalogInput<adc::ADC1, P>> for ADC<U1>
where
    WORD: From<u16>,
    P: Pin<adc::ADC1>,
{
    type Error = core::convert::Infallible;

    /// Read an ADC value from an AnalogInput. This should always return a good result
    fn read(&mut self, _pin: &mut AnalogInput<adc::ADC1, P>) -> nb::Result<WORD, Self::Error> {
        read(self, _pin).map(|r0| r0.into())
    }
}

impl<WORD, P> OneShot<adc::ADC2, WORD, AnalogInput<adc::ADC2, P>> for ADC<U2>
where
    WORD: From<u16>,
    P: Pin<adc::ADC2>,
{
    type Error = core::convert::Infallible;

    /// Read an ADC value from an AnalogInput. This should always return a good result
    fn read(&mut self, _pin: &mut AnalogInput<adc::ADC2, P>) -> nb::Result<WORD, Self::Error> {
        read(self, _pin).map(|r0| r0.into())
    }
}

#[inline(always)]
fn read<N, ADCx, P>(
    adc: &mut ADC<N>,
    _: &mut AnalogInput<ADCx, P>,
) -> nb::Result<u16, core::convert::Infallible>
where
    ADCx: adc::ADC,
    P: Pin<ADCx>,
{
    let channel = <P as Pin<ADCx>>::Input::U32;
    ral::modify_reg!(ral::adc, adc.reg, HC0, |_| channel);
    while (ral::read_reg!(ral::adc, adc.reg, HS, COCO0) == 0) {}

    Ok(ral::read_reg!(ral::adc, adc.reg, R0) as u16)
}

/// ```compile_fail
/// use imxrt1060_hal as hal;
/// use hal::{adc, ral};
/// use embedded_hal::adc::OneShot;
///
/// let mut pads = ral::iomuxc::IOMUXC::take()
///     .map(hal::iomuxc::new)
///     .unwrap();
///
/// let mut adc1 = ral::adc::ADC1::take()
///     .map(|inst| adc::ADC::new(inst, Default::default(), Default::default()))
///     .unwrap();
/// let mut adc2_pin = adc::AnalogInput::new(pads.ad_b1.p12);
///
/// let reading: u16 = adc1.read(&mut adc2_pin).unwrap();
/// ```
#[cfg(doctest)]
struct Adc1Pin2Mismatch;

/// ```no_run
/// use imxrt1060_hal as hal;
/// use hal::{adc, ral};
/// use embedded_hal::adc::OneShot;
///
/// let mut pads = ral::iomuxc::IOMUXC::take()
///     .map(hal::iomuxc::new)
///     .unwrap();
///
/// let mut adc2 = ral::adc::ADC2::take()
///     .map(|inst| adc::ADC::new(inst, adc::ClockSelect::default(), adc::ClockDivision::default()))
///     .unwrap();
/// let mut adc2_pin = adc::AnalogInput::new(pads.ad_b1.p12);
///
/// let reading: u16 = adc2.read(&mut adc2_pin).unwrap();
/// ```
#[cfg(doctest)]
struct Adc2Pin2;

/// ```compile_fail
/// use imxrt1060_hal as hal;
/// use hal::{adc, ral};
/// use embedded_hal::adc::OneShot;
///
/// let mut pads = ral::iomuxc::IOMUXC::take()
///     .map(hal::iomuxc::new)
///     .unwrap();
///
/// let mut adc2 = ral::adc::ADC2::take()
///     .map(|inst| adc::ADC::new(inst, Default::default(), Default::default()))
///     .unwrap();
/// let mut adc2_pin = adc::AnalogInput::new(pads.ad_b0.p13);
///
/// let reading: u16 = adc2.read(&mut adc2_pin).unwrap();
/// ```
#[cfg(doctest)]
struct Adc2Pin1Mismatch;
