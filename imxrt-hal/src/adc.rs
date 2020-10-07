//! ADC
//! 
//! This ADC driver supports `embedded_hal`'s ADC traits
//! 
//! # Example
//! ```no_run
//! use imxrt_hal::{self, adc};
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//! let (adc1_builder, _) = peripherals.adc.clock(&mut peripherals.ccm.handle);
//! 
//! let mut adc1 = adc1_builder.build(adc::ClockSelect::default(), adc::ClockDivision::default());
//! let mut a1 = adc::AnalogPin::new(peripherals.iomuxc.ad_b1.p02, adc1);
//! 
//! let reading = adc1.read(&mut a1).unwrap();
//!```

use crate::ccm;
use crate::ral;
use crate::iomuxc::adc::{ADC1, ADC2, Pin, prepare_adc_pin};
use crate::iomuxc::{consts::Unsigned, adc};
use core::marker::PhantomData;
use embedded_hal::adc::{Channel, OneShot};

/// The clock input for an ADC
#[allow(non_camel_case_types)]
pub enum ClockSelect {
    IPG,            // IPG clock
    IPG_2,          // IPG clock / 2
    ADACK           // Asynchronous clock 
}

/// How much to divide the clock input
pub enum ClockDivision {
    Div1,           // Input clock / 1
    Div2,           // Input clock / 2
    Div4,           // Input clock / 4
    Div8            // Input clock / 8
}

impl ClockSelect {
    pub fn default() -> Self {
        ClockSelect::ADACK
    }
}

impl ClockDivision {
    pub fn default() -> Self {
        ClockDivision::Div2
    }
}

/// Conversion speeds done by clock cycles
pub enum ConversionSpeed {
    Slow,       // 25 ADC clock cycles (24 on imxrt102x)
    Medium,     // 17 ADC clock cycles (16 on imxrt102x)
    Fast,       // 9 ADC clock cycles (8 on imxrt102x)
    VeryFast    // 3 ADC clock cycles (2 on imxrt102x)
}

/// Denotes how much hardware averaging to do
pub enum AveragingCount {
    Avg1,
    Avg4,
    Avg8,
    Avg16,
    Avg32
}

// Specifies the resolution the ADC
pub enum ResolutionBits {
    Res8,
    Res10,
    Res12
}

/// A pin representing an analog input for a particular ADC
pub struct AnalogInput<ADCx, P>
{
    _module: PhantomData<ADCx>,
    _pin: P
}

impl<P, ADCx> Channel<ADCx> for AnalogInput<ADCx, P>
where 
    P: Pin<ADCx>,
    ADCx: adc::ADC
{
    type ID = u16;

    fn channel() -> Self::ID {
        <P as Pin<ADCx>>::Input::U16
    }
}

impl<P, ADCx> AnalogInput<ADCx, P>
where
    P: Pin<ADCx>,
    ADCx: adc::ADC
{
    /// Creates a new analog input pin
    pub fn new(mut pin: P, _adc: &ADC<ADCx>) -> Self {
        prepare_adc_pin(&mut pin);
        Self {
            _module: PhantomData,
            _pin: pin
        }
    }
}

pub struct ADC<ADCx> {
    _module: PhantomData<ADCx>,
    reg: ral::adc::Instance
}

impl<ADCx> ADC<ADCx> {
    fn new(reg: ral::adc::Instance) -> Self {
        let inst = Self {
            _module: PhantomData,
            reg
        };

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
    pub fn set_resolution(&self, bits: ResolutionBits) {
        ral::modify_reg!(ral::adc, self.reg, CFG, MODE: match bits {
            ResolutionBits::Res8 => MODE_0,
            ResolutionBits::Res10 => MODE_1,
            ResolutionBits::Res12 => MODE_2
        });
    }
    
    /// Sets the number of hardware averages taken by the ADC
    pub fn set_averaging(&self, avg: AveragingCount) {
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
    pub fn set_conversion_speed(&self, conversion_speed: ConversionSpeed) {
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
    pub fn set_low_power_mode(&self, state: bool) {
        ral::modify_reg!(ral::adc, self.reg, CFG, ADLPC: if state { ADLPC_1 } else { ADLPC_0 });
    }

    /// Calibrates the ADC, will wait for finish
    pub fn calibrate(&self) {
        ral::modify_reg!(ral::adc, self.reg, GC, CAL: 0b1);
        while (ral::read_reg!(ral::adc, self.reg, CAL, CAL_CODE) != 0) {}
    }
}

/// Implement embedded-hal traits
impl<ADCx, WORD, P> OneShot<ADCx, WORD, AnalogInput<ADCx, P>> for ADC<ADCx>
where
    ADCx: adc::ADC,
    WORD: From<u16>,
    P: Pin<ADCx>
{
    type Error = ();

    /// Read an ADC value from an AnalogInput. This should always return a good result
    fn read(&mut self, _pin: &mut AnalogInput<ADCx, P>) -> nb::Result<WORD, Self::Error> {
        let channel = <P as Pin<ADCx>>::Input::U32;
        ral::modify_reg!(ral::adc, self.reg, HC0, |_| channel);
        while (ral::read_reg!(ral::adc, self.reg, HS, COCO0) == 0) {}

        Ok((ral::read_reg!(ral::adc, self.reg, R0) as u16).into())
    }
}

/// Unclocked ADC modules
///
/// The `Unclocked` struct represents both unconfigured ADC peripherals.
/// Once clocked, you'll have the ability to either the ADC1 or ADC2 
/// peripherals.
pub struct Unclocked {
    pub(crate) adc1: ral::adc::Instance,
    pub(crate) adc2: ral::adc::Instance
}

impl Unclocked {
    pub fn clock(self, handle: &mut ccm::Handle) -> (Builder<ADC1>, Builder<ADC2>) {
        let (ccm, _) = handle.raw();
        ral::modify_reg!(ral::ccm, ccm, CCGR1, CG8: 0b11); // adc1_clk_enable
        ral::modify_reg!(ral::ccm, ccm, CCGR1, CG4: 0b11); // adc2_clk_enable

        (
            Builder::new(self.adc1),
            Builder::new(self.adc2)
        )
    }
}

/// An ADC builder than can build an ADC1 or ADC2 module
pub struct Builder<ADCx> {
    _module: PhantomData<ADCx>,
    reg: ral::adc::Instance
}

impl<ADCx> Builder<ADCx>
where 
    ADCx: adc::ADC
{
    fn new(reg: ral::adc::Instance) -> Self {
        Self {
            _module: PhantomData,
            reg
        }
    }

    /// Builds an ADC peripheral with a certain clock selection. The ADC starts at
    /// a 10-bit resolution w/ 4 hardware averaging samples
    pub fn build(self, clock: ClockSelect, division: ClockDivision) -> ADC<ADCx> {
        // Enable asynchronous clock if applicable
        ral::modify_reg!(ral::adc, self.reg, GC, ADACKEN: match clock {
            ClockSelect::ADACK => ADACKEN_1,
            _ => ADACKEN_0
        });

        // Select the clock selection, division, and enable ADHSC if applicable
        ral::modify_reg!(ral::adc, self.reg, CFG, 
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

        ADC::new(self.reg)
    }
}
