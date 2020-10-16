#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register for hardware triggers
pub mod HC0 {

    /// Input Channel Select
    pub mod ADCH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b10000: External channel selection from ADC_ETC
            pub const ADCH_16: u32 = 0b10000;

            /// 0b11001: VREFSH = internal channel, for ADC self-test, hard connected to VRH internally
            pub const ADCH_25: u32 = 0b11001;

            /// 0b11111: Conversion Disabled. Hardware Triggers will not initiate any conversion.
            pub const ADCH_31: u32 = 0b11111;
        }
    }

    /// Conversion Complete Interrupt Enable/Disable Control
    pub mod AIEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Conversion complete interrupt disabled
            pub const AIEN_0: u32 = 0b0;

            /// 0b1: Conversion complete interrupt enabled
            pub const AIEN_1: u32 = 0b1;
        }
    }
}

/// Control register for hardware triggers
pub mod HC1 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC2 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC3 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC4 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC5 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC6 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Control register for hardware triggers
pub mod HC7 {
    pub use super::HC0::ADCH;
    pub use super::HC0::AIEN;
}

/// Status register for HW triggers
pub mod HS {

    /// Conversion Complete Flag
    pub mod COCO0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data result register for HW triggers
pub mod R0 {

    /// Data (result of an ADC conversion)
    pub mod CDATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data result register for HW triggers
pub mod R1 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R2 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R3 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R4 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R5 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R6 {
    pub use super::R0::CDATA;
}

/// Data result register for HW triggers
pub mod R7 {
    pub use super::R0::CDATA;
}

/// Configuration register
pub mod CFG {

    /// Input Clock Select
    pub mod ADICLK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: IPG clock
            pub const ADICLK_0: u32 = 0b00;

            /// 0b01: IPG clock divided by 2
            pub const ADICLK_1: u32 = 0b01;

            /// 0b10: Alternate clock (ALTCLK)
            pub const ADICLK_2: u32 = 0b10;

            /// 0b11: Asynchronous clock (ADACK)
            pub const ADICLK_3: u32 = 0b11;
        }
    }

    /// Conversion Mode Selection
    pub mod MODE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8-bit conversion
            pub const MODE_0: u32 = 0b00;

            /// 0b01: 10-bit conversion
            pub const MODE_1: u32 = 0b01;

            /// 0b10: 12-bit conversion
            pub const MODE_2: u32 = 0b10;
        }
    }

    /// Long Sample Time Configuration
    pub mod ADLSMP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Short sample mode.
            pub const ADLSMP_0: u32 = 0b0;

            /// 0b1: Long sample mode.
            pub const ADLSMP_1: u32 = 0b1;
        }
    }

    /// Clock Divide Select
    pub mod ADIV {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Input clock
            pub const ADIV_0: u32 = 0b00;

            /// 0b01: Input clock / 2
            pub const ADIV_1: u32 = 0b01;

            /// 0b10: Input clock / 4
            pub const ADIV_2: u32 = 0b10;

            /// 0b11: Input clock / 8
            pub const ADIV_3: u32 = 0b11;
        }
    }

    /// Low-Power Configuration
    pub mod ADLPC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ADC hard block not in low power mode.
            pub const ADLPC_0: u32 = 0b0;

            /// 0b1: ADC hard block in low power mode.
            pub const ADLPC_1: u32 = 0b1;
        }
    }

    /// Defines the sample time duration
    pub mod ADSTS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b
            pub const ADSTS_0: u32 = 0b00;

            /// 0b01: Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b
            pub const ADSTS_1: u32 = 0b01;

            /// 0b10: Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b
            pub const ADSTS_2: u32 = 0b10;

            /// 0b11: Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b
            pub const ADSTS_3: u32 = 0b11;
        }
    }

    /// High Speed Configuration
    pub mod ADHSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal conversion selected.
            pub const ADHSC_0: u32 = 0b0;

            /// 0b1: High speed conversion selected.
            pub const ADHSC_1: u32 = 0b1;
        }
    }

    /// Voltage Reference Selection
    pub mod REFSEL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selects VREFH/VREFL as reference voltage.
            pub const REFSEL_0: u32 = 0b00;
        }
    }

    /// Conversion Trigger Select
    pub mod ADTRG {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Software trigger selected
            pub const ADTRG_0: u32 = 0b0;

            /// 0b1: Hardware trigger selected
            pub const ADTRG_1: u32 = 0b1;
        }
    }

    /// Hardware Average select
    pub mod AVGS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 4 samples averaged
            pub const AVGS_0: u32 = 0b00;

            /// 0b01: 8 samples averaged
            pub const AVGS_1: u32 = 0b01;

            /// 0b10: 16 samples averaged
            pub const AVGS_2: u32 = 0b10;

            /// 0b11: 32 samples averaged
            pub const AVGS_3: u32 = 0b11;
        }
    }

    /// Data Overwrite Enable
    pub mod OVWREN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data.
            pub const OVWREN_0: u32 = 0b0;

            /// 0b1: Enable the overwriting.
            pub const OVWREN_1: u32 = 0b1;
        }
    }
}

/// General control register
pub mod GC {

    /// Asynchronous clock output enable
    pub mod ADACKEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active.
            pub const ADACKEN_0: u32 = 0b0;

            /// 0b1: Asynchronous clock and clock output enabled regardless of the state of the ADC
            pub const ADACKEN_1: u32 = 0b1;
        }
    }

    /// DMA Enable
    pub mod DMAEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled (default)
            pub const DMAEN_0: u32 = 0b0;

            /// 0b1: DMA enabled
            pub const DMAEN_1: u32 = 0b1;
        }
    }

    /// Compare Function Range Enable
    pub mod ACREN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared.
            pub const ACREN_0: u32 = 0b0;

            /// 0b1: Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared.
            pub const ACREN_1: u32 = 0b1;
        }
    }

    /// Compare Function Greater Than Enable
    pub mod ACFGT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Configures "Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive" functionality based on the values placed in the ADC_CV register.
            pub const ACFGT_0: u32 = 0b0;

            /// 0b1: Configures "Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive" functionality based on the values placed in the ADC_CV registers.
            pub const ACFGT_1: u32 = 0b1;
        }
    }

    /// Compare Function Enable
    pub mod ACFE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Compare function disabled
            pub const ACFE_0: u32 = 0b0;

            /// 0b1: Compare function enabled
            pub const ACFE_1: u32 = 0b1;
        }
    }

    /// Hardware average enable
    pub mod AVGE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware average function disabled
            pub const AVGE_0: u32 = 0b0;

            /// 0b1: Hardware average function enabled
            pub const AVGE_1: u32 = 0b1;
        }
    }

    /// Continuous Conversion Enable
    pub mod ADCO {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion.
            pub const ADCO_0: u32 = 0b0;

            /// 0b1: Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion.
            pub const ADCO_1: u32 = 0b1;
        }
    }

    /// Calibration
    pub mod CAL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// General status register
pub mod GS {

    /// Conversion Active
    pub mod ADACT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Conversion not in progress.
            pub const ADACT_0: u32 = 0b0;

            /// 0b1: Conversion in progress.
            pub const ADACT_1: u32 = 0b1;
        }
    }

    /// Calibration Failed Flag
    pub mod CALF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Calibration completed normally.
            pub const CALF_0: u32 = 0b0;

            /// 0b1: Calibration failed. ADC accuracy specifications are not guaranteed.
            pub const CALF_1: u32 = 0b1;
        }
    }

    /// Asynchronous wakeup interrupt status
    pub mod AWKST {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No asynchronous interrupt.
            pub const AWKST_0: u32 = 0b0;

            /// 0b1: Asynchronous wake up interrupt occurred in stop mode.
            pub const AWKST_1: u32 = 0b1;
        }
    }
}

/// Compare value register
pub mod CV {

    /// Compare Value 1
    pub mod CV1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare Value 2
    pub mod CV2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Offset correction value register
pub mod OFS {

    /// Offset value
    pub mod OFS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sign bit
    pub mod SIGN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The offset value is added with the raw result
            pub const SIGN_0: u32 = 0b0;

            /// 0b1: The offset value is subtracted from the raw converted value
            pub const SIGN_1: u32 = 0b1;
        }
    }
}

/// Calibration value register
pub mod CAL {

    /// Calibration Result Value
    pub mod CAL_CODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Control register for hardware triggers
    pub HC0: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC1: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC2: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC3: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC4: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC5: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC6: RWRegister<u32>,

    /// Control register for hardware triggers
    pub HC7: RWRegister<u32>,

    /// Status register for HW triggers
    pub HS: RORegister<u32>,

    /// Data result register for HW triggers
    pub R0: RORegister<u32>,

    /// Data result register for HW triggers
    pub R1: RORegister<u32>,

    /// Data result register for HW triggers
    pub R2: RORegister<u32>,

    /// Data result register for HW triggers
    pub R3: RORegister<u32>,

    /// Data result register for HW triggers
    pub R4: RORegister<u32>,

    /// Data result register for HW triggers
    pub R5: RORegister<u32>,

    /// Data result register for HW triggers
    pub R6: RORegister<u32>,

    /// Data result register for HW triggers
    pub R7: RORegister<u32>,

    /// Configuration register
    pub CFG: RWRegister<u32>,

    /// General control register
    pub GC: RWRegister<u32>,

    /// General status register
    pub GS: RWRegister<u32>,

    /// Compare value register
    pub CV: RWRegister<u32>,

    /// Offset correction value register
    pub OFS: RWRegister<u32>,

    /// Calibration value register
    pub CAL: RWRegister<u32>,
}
pub struct ResetValues {
    pub HC0: u32,
    pub HC1: u32,
    pub HC2: u32,
    pub HC3: u32,
    pub HC4: u32,
    pub HC5: u32,
    pub HC6: u32,
    pub HC7: u32,
    pub HS: u32,
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub CFG: u32,
    pub GC: u32,
    pub GS: u32,
    pub CV: u32,
    pub OFS: u32,
    pub CAL: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}

/// Access functions for the ADC1 peripheral instance
pub mod ADC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400c4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1
    pub const reset: ResetValues = ResetValues {
        HC0: 0x0000001F,
        HC1: 0x0000001F,
        HC2: 0x0000001F,
        HC3: 0x0000001F,
        HC4: 0x0000001F,
        HC5: 0x0000001F,
        HC6: 0x0000001F,
        HC7: 0x0000001F,
        HS: 0x00000000,
        R0: 0x00000000,
        R1: 0x00000000,
        R2: 0x00000000,
        R3: 0x00000000,
        R4: 0x00000000,
        R5: 0x00000000,
        R6: 0x00000000,
        R7: 0x00000000,
        CFG: 0x00000200,
        GC: 0x00000000,
        GS: 0x00000000,
        CV: 0x00000000,
        OFS: 0x00000000,
        CAL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC1_TAKEN: bool = false;

    /// Safe access to ADC1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_TAKEN {
                None
            } else {
                ADC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_TAKEN && inst.addr == INSTANCE.addr {
                ADC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1: *const RegisterBlock = 0x400c4000 as *const _;
