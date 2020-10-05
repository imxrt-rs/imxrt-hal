//! ADC
//! 
//! Theoretically maybe might support reading analog values
//! 

use crate::ccm;
use crate::ral;

/// Conversion speeds done by clock cycles
pub enum ConversionSpeed {
    Slow,       // 25 ADC clock cycles
    Medium,     // 17 ADC clock cycles
    Fast,       // 9 ADC clock cycles
    VeryFast    // 3 ADC clock cycles
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

pub struct ADC {
    reg: ral::adc::Instance,

}

impl ADC {
    fn new(reg: ral::adc::Instance) -> Self {
        let inst = Self {
            reg
        };

        inst.set_resolution(ResolutionBits::Res10);
        inst.set_averaging(AveragingCount::Avg4);
        inst.set_conversion_speed(ConversionSpeed::Medium);
        inst.set_low_power_mode(false);
        inst.calibrate();

        inst
    }

    /// Sets the resolution that analog reads return, in bits
    pub fn set_resolution(&self, bits: ResolutionBits) {
        let mode: u32 = match bits {
            ResolutionBits::Res8 => 0b00,
            ResolutionBits::Res10 => 0b01,
            ResolutionBits::Res12 => 0b11
        };

        ral::modify_reg!(ral::adc, self.reg, CFG, MODE: mode);
    }
    
    /// Sets the number of hardware averages taken by the ADC
    pub fn set_averaging(&self, avg: AveragingCount) {
        let avge: u32 = match avg {
            AveragingCount::Avg1 => 0b0,
            _ => 0b1
        };

        let mode: u32 = match avg {
            AveragingCount::Avg32 => 0b11,
            AveragingCount::Avg16 => 0b10,
            AveragingCount::Avg8 => 0b01,
            _ => 0b00
        };

        ral::modify_reg!(ral::adc, self.reg, GC, AVGE: avge);
        ral::modify_reg!(ral::adc, self.reg, CFG, MODE: mode);
    }

    /// Sets the conversion speed for this ADC, see ConversionSpeed for clock cycle counts.
    /// You may also need to recalibrate afterwards
    pub fn set_conversion_speed(&self, conversion_speed: ConversionSpeed) {
        let (adsts, adlsmp) = match conversion_speed {
            ConversionSpeed::Slow => (0b11, 0b1),
            ConversionSpeed::Medium => (0b01, 0b1),
            ConversionSpeed::Fast => (0b11, 0b0),
            ConversionSpeed::VeryFast => (0b00, 0b0)
        };

        ral::modify_reg!(ral::adc, self.reg, CFG, ADSTS: adsts, ADLSMP: adlsmp);

        // TODO Recalibrate?
    }

    /// Enables or disables the low power configuration in the ADC
    pub fn set_low_power_mode(&self, state: bool) {
        ral::modify_reg!(ral::adc, self.reg, CFG, ADLPC: if state { 0b1 } else {  0b0 });
    }

    /// Calibrates the ADC, will wait for finish
    pub fn calibrate(&self) {
        ral::modify_reg!(ral::adc, self.reg, GC, CAL: 0b1);
        while (ral::read_reg!(ral::adc, self.reg, CAL, CAL_CODE) != 0) {}
    }
}

pub struct Unclocked {
    pub(crate) adc1: ral::adc::Instance,
    pub(crate) adc2: ral::adc::Instance
}

impl Unclocked {
    pub fn clock(self, handle: &mut ccm::Handle) -> (ADC, ADC) {
        let (ccm, _) = handle.raw();
        ral::modify_reg!(ral::ccm, ccm, CCGR1, CG8: 0b11); // adc1_clk_enable
        ral::modify_reg!(ral::ccm, ccm, CCGR1, CG4: 0b11); // adc2_clk_enable

        // Set to asynchronous clock
        ral::modify_reg!(ral::adc, self.adc1, GC, ADACKEN: 0b1);
        ral::modify_reg!(ral::adc, self.adc2, GC, ADACKEN: 0b1);
        ral::modify_reg!(ral::adc, self.adc1, CFG, ADICLK: 0b11, ADIV: 0b01);
        ral::modify_reg!(ral::adc, self.adc2, CFG, ADICLK: 0b11, ADIV: 0b01);

        (
            ADC::new(self.adc1),
            ADC::new(self.adc2)
        )
    }
}
