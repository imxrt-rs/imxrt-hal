#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PMU
//!
//! Used by: imxrt1051, imxrt1052

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Regulator 1P1 Register
pub mod REG_1P1 {

    /// Control bit to enable the regulator output.
    pub mod ENABLE_LINREG {
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

    /// Control bit to enable the brownout circuitry in the regulator.
    pub mod ENABLE_BO {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to enable the current-limit circuitry in the regulator.
    pub mod ENABLE_ILIMIT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to enable the pull-down circuitry in the regulator
    pub mod ENABLE_PULLDOWN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bits to adjust the regulator brownout offset voltage in 25mV steps
    pub mod BO_OFFSET {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bits to adjust the regulator output voltage
    pub mod OUTPUT_TRG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00100: 0.8V
            pub const OUTPUT_TRG_4: u32 = 0b00100;

            /// 0b10000: 1.1V
            pub const OUTPUT_TRG_16: u32 = 0b10000;
        }
    }

    /// Status bit that signals when a brownout is detected on the regulator output.
    pub mod BO_VDD1P1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target
    pub mod OK_VDD1P1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the weak 1p1 regulator
    pub mod ENABLE_WEAK_LINREG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Selects the source for the reference voltage of the weak 1p1 regulator.
    pub mod SELREF_WEAK_LINREG {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Weak-linreg output tracks low-power-bandgap voltage
            pub const SELREF_WEAK_LINREG_0: u32 = 0b0;

            /// 0b1: Weak-linreg output tracks VDD_SOC_IN voltage
            pub const SELREF_WEAK_LINREG_1: u32 = 0b1;
        }
    }
}

/// Regulator 1P1 Register
pub mod REG_1P1_SET {
    pub use super::REG_1P1::BO_OFFSET;
    pub use super::REG_1P1::BO_VDD1P1;
    pub use super::REG_1P1::ENABLE_BO;
    pub use super::REG_1P1::ENABLE_ILIMIT;
    pub use super::REG_1P1::ENABLE_LINREG;
    pub use super::REG_1P1::ENABLE_PULLDOWN;
    pub use super::REG_1P1::ENABLE_WEAK_LINREG;
    pub use super::REG_1P1::OK_VDD1P1;
    pub use super::REG_1P1::OUTPUT_TRG;
    pub use super::REG_1P1::SELREF_WEAK_LINREG;
}

/// Regulator 1P1 Register
pub mod REG_1P1_CLR {
    pub use super::REG_1P1::BO_OFFSET;
    pub use super::REG_1P1::BO_VDD1P1;
    pub use super::REG_1P1::ENABLE_BO;
    pub use super::REG_1P1::ENABLE_ILIMIT;
    pub use super::REG_1P1::ENABLE_LINREG;
    pub use super::REG_1P1::ENABLE_PULLDOWN;
    pub use super::REG_1P1::ENABLE_WEAK_LINREG;
    pub use super::REG_1P1::OK_VDD1P1;
    pub use super::REG_1P1::OUTPUT_TRG;
    pub use super::REG_1P1::SELREF_WEAK_LINREG;
}

/// Regulator 1P1 Register
pub mod REG_1P1_TOG {
    pub use super::REG_1P1::BO_OFFSET;
    pub use super::REG_1P1::BO_VDD1P1;
    pub use super::REG_1P1::ENABLE_BO;
    pub use super::REG_1P1::ENABLE_ILIMIT;
    pub use super::REG_1P1::ENABLE_LINREG;
    pub use super::REG_1P1::ENABLE_PULLDOWN;
    pub use super::REG_1P1::ENABLE_WEAK_LINREG;
    pub use super::REG_1P1::OK_VDD1P1;
    pub use super::REG_1P1::OUTPUT_TRG;
    pub use super::REG_1P1::SELREF_WEAK_LINREG;
}

/// Regulator 3P0 Register
pub mod REG_3P0 {

    /// Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference
    pub mod ENABLE_LINREG {
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

    /// Control bit to enable the brownout circuitry in the regulator.
    pub mod ENABLE_BO {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to enable the current-limit circuitry in the regulator.
    pub mod ENABLE_ILIMIT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bits to adjust the regulator brownout offset voltage in 25mV steps
    pub mod BO_OFFSET {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS
    pub mod VBUS_SEL {
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

            /// 0b0: Utilize VBUS OTG2 power
            pub const USB_OTG2_VBUS: u32 = 0b0;

            /// 0b1: Utilize VBUS OTG1 power
            pub const USB_OTG1_VBUS: u32 = 0b1;
        }
    }

    /// Control bits to adjust the regulator output voltage
    pub mod OUTPUT_TRG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 2.625V
            pub const OUTPUT_TRG_0: u32 = 0b00000;

            /// 0b01111: 3.000V
            pub const OUTPUT_TRG_15: u32 = 0b01111;

            /// 0b11111: 3.400V
            pub const OUTPUT_TRG_31: u32 = 0b11111;
        }
    }

    /// Status bit that signals when a brownout is detected on the regulator output.
    pub mod BO_VDD3P0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target
    pub mod OK_VDD3P0 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Regulator 3P0 Register
pub mod REG_3P0_SET {
    pub use super::REG_3P0::BO_OFFSET;
    pub use super::REG_3P0::BO_VDD3P0;
    pub use super::REG_3P0::ENABLE_BO;
    pub use super::REG_3P0::ENABLE_ILIMIT;
    pub use super::REG_3P0::ENABLE_LINREG;
    pub use super::REG_3P0::OK_VDD3P0;
    pub use super::REG_3P0::OUTPUT_TRG;
    pub use super::REG_3P0::VBUS_SEL;
}

/// Regulator 3P0 Register
pub mod REG_3P0_CLR {
    pub use super::REG_3P0::BO_OFFSET;
    pub use super::REG_3P0::BO_VDD3P0;
    pub use super::REG_3P0::ENABLE_BO;
    pub use super::REG_3P0::ENABLE_ILIMIT;
    pub use super::REG_3P0::ENABLE_LINREG;
    pub use super::REG_3P0::OK_VDD3P0;
    pub use super::REG_3P0::OUTPUT_TRG;
    pub use super::REG_3P0::VBUS_SEL;
}

/// Regulator 3P0 Register
pub mod REG_3P0_TOG {
    pub use super::REG_3P0::BO_OFFSET;
    pub use super::REG_3P0::BO_VDD3P0;
    pub use super::REG_3P0::ENABLE_BO;
    pub use super::REG_3P0::ENABLE_ILIMIT;
    pub use super::REG_3P0::ENABLE_LINREG;
    pub use super::REG_3P0::OK_VDD3P0;
    pub use super::REG_3P0::OUTPUT_TRG;
    pub use super::REG_3P0::VBUS_SEL;
}

/// Regulator 2P5 Register
pub mod REG_2P5 {

    /// Control bit to enable the regulator output.
    pub mod ENABLE_LINREG {
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

    /// Control bit to enable the brownout circuitry in the regulator.
    pub mod ENABLE_BO {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to enable the current-limit circuitry in the regulator.
    pub mod ENABLE_ILIMIT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to enable the pull-down circuitry in the regulator
    pub mod ENABLE_PULLDOWN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bits to adjust the regulator brownout offset voltage in 25mV steps
    pub mod BO_OFFSET {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bits to adjust the regulator output voltage
    pub mod OUTPUT_TRG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 2.10V
            pub const OUTPUT_TRG_0: u32 = 0b00000;

            /// 0b10000: 2.50V
            pub const OUTPUT_TRG_16: u32 = 0b10000;

            /// 0b11111: 2.875V
            pub const OUTPUT_TRG_31: u32 = 0b11111;
        }
    }

    /// Status bit that signals when a brownout is detected on the regulator output.
    pub mod BO_VDD2P5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target
    pub mod OK_VDD2P5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the weak 2p5 regulator
    pub mod ENABLE_WEAK_LINREG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Regulator 2P5 Register
pub mod REG_2P5_SET {
    pub use super::REG_2P5::BO_OFFSET;
    pub use super::REG_2P5::BO_VDD2P5;
    pub use super::REG_2P5::ENABLE_BO;
    pub use super::REG_2P5::ENABLE_ILIMIT;
    pub use super::REG_2P5::ENABLE_LINREG;
    pub use super::REG_2P5::ENABLE_PULLDOWN;
    pub use super::REG_2P5::ENABLE_WEAK_LINREG;
    pub use super::REG_2P5::OK_VDD2P5;
    pub use super::REG_2P5::OUTPUT_TRG;
}

/// Regulator 2P5 Register
pub mod REG_2P5_CLR {
    pub use super::REG_2P5::BO_OFFSET;
    pub use super::REG_2P5::BO_VDD2P5;
    pub use super::REG_2P5::ENABLE_BO;
    pub use super::REG_2P5::ENABLE_ILIMIT;
    pub use super::REG_2P5::ENABLE_LINREG;
    pub use super::REG_2P5::ENABLE_PULLDOWN;
    pub use super::REG_2P5::ENABLE_WEAK_LINREG;
    pub use super::REG_2P5::OK_VDD2P5;
    pub use super::REG_2P5::OUTPUT_TRG;
}

/// Regulator 2P5 Register
pub mod REG_2P5_TOG {
    pub use super::REG_2P5::BO_OFFSET;
    pub use super::REG_2P5::BO_VDD2P5;
    pub use super::REG_2P5::ENABLE_BO;
    pub use super::REG_2P5::ENABLE_ILIMIT;
    pub use super::REG_2P5::ENABLE_LINREG;
    pub use super::REG_2P5::ENABLE_PULLDOWN;
    pub use super::REG_2P5::ENABLE_WEAK_LINREG;
    pub use super::REG_2P5::OK_VDD2P5;
    pub use super::REG_2P5::OUTPUT_TRG;
}

/// Digital Regulator Core Register
pub mod REG_CORE {

    /// This field defines the target voltage for the ARM core power domain
    pub mod REG0_TARG {
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

            /// 0b00000: Power gated off
            pub const REG0_TARG_0: u32 = 0b00000;

            /// 0b00001: Target core voltage = 0.725V
            pub const REG0_TARG_1: u32 = 0b00001;

            /// 0b00010: Target core voltage = 0.750V
            pub const REG0_TARG_2: u32 = 0b00010;

            /// 0b00011: Target core voltage = 0.775V
            pub const REG0_TARG_3: u32 = 0b00011;

            /// 0b10000: Target core voltage = 1.100V
            pub const REG0_TARG_16: u32 = 0b10000;

            /// 0b11110: Target core voltage = 1.450V
            pub const REG0_TARG_30: u32 = 0b11110;

            /// 0b11111: Power FET switched full on. No regulation.
            pub const REG0_TARG_31: u32 = 0b11111;
        }
    }

    /// This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.
    pub mod REG0_ADJ {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No adjustment
            pub const REG0_ADJ_0: u32 = 0b0000;

            /// 0b0001: + 0.25%
            pub const REG0_ADJ_1: u32 = 0b0001;

            /// 0b0010: + 0.50%
            pub const REG0_ADJ_2: u32 = 0b0010;

            /// 0b0011: + 0.75%
            pub const REG0_ADJ_3: u32 = 0b0011;

            /// 0b0100: + 1.00%
            pub const REG0_ADJ_4: u32 = 0b0100;

            /// 0b0101: + 1.25%
            pub const REG0_ADJ_5: u32 = 0b0101;

            /// 0b0110: + 1.50%
            pub const REG0_ADJ_6: u32 = 0b0110;

            /// 0b0111: + 1.75%
            pub const REG0_ADJ_7: u32 = 0b0111;

            /// 0b1000: - 0.25%
            pub const REG0_ADJ_8: u32 = 0b1000;

            /// 0b1001: - 0.50%
            pub const REG0_ADJ_9: u32 = 0b1001;

            /// 0b1010: - 0.75%
            pub const REG0_ADJ_10: u32 = 0b1010;

            /// 0b1011: - 1.00%
            pub const REG0_ADJ_11: u32 = 0b1011;

            /// 0b1100: - 1.25%
            pub const REG0_ADJ_12: u32 = 0b1100;

            /// 0b1101: - 1.50%
            pub const REG0_ADJ_13: u32 = 0b1101;

            /// 0b1110: - 1.75%
            pub const REG0_ADJ_14: u32 = 0b1110;

            /// 0b1111: - 2.00%
            pub const REG0_ADJ_15: u32 = 0b1111;
        }
    }

    /// This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation.
    pub mod REG1_TARG {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Power gated off
            pub const REG1_TARG_0: u32 = 0b00000;

            /// 0b00001: Target core voltage = 0.725V
            pub const REG1_TARG_1: u32 = 0b00001;

            /// 0b00010: Target core voltage = 0.750V
            pub const REG1_TARG_2: u32 = 0b00010;

            /// 0b00011: Target core voltage = 0.775V
            pub const REG1_TARG_3: u32 = 0b00011;

            /// 0b10000: Target core voltage = 1.100V
            pub const REG1_TARG_16: u32 = 0b10000;

            /// 0b11110: Target core voltage = 1.450V
            pub const REG1_TARG_30: u32 = 0b11110;

            /// 0b11111: Power FET switched full on. No regulation.
            pub const REG1_TARG_31: u32 = 0b11111;
        }
    }

    /// This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.
    pub mod REG1_ADJ {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (4 bits: 0b1111 << 14)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No adjustment
            pub const REG1_ADJ_0: u32 = 0b0000;

            /// 0b0001: + 0.25%
            pub const REG1_ADJ_1: u32 = 0b0001;

            /// 0b0010: + 0.50%
            pub const REG1_ADJ_2: u32 = 0b0010;

            /// 0b0011: + 0.75%
            pub const REG1_ADJ_3: u32 = 0b0011;

            /// 0b0100: + 1.00%
            pub const REG1_ADJ_4: u32 = 0b0100;

            /// 0b0101: + 1.25%
            pub const REG1_ADJ_5: u32 = 0b0101;

            /// 0b0110: + 1.50%
            pub const REG1_ADJ_6: u32 = 0b0110;

            /// 0b0111: + 1.75%
            pub const REG1_ADJ_7: u32 = 0b0111;

            /// 0b1000: - 0.25%
            pub const REG1_ADJ_8: u32 = 0b1000;

            /// 0b1001: - 0.50%
            pub const REG1_ADJ_9: u32 = 0b1001;

            /// 0b1010: - 0.75%
            pub const REG1_ADJ_10: u32 = 0b1010;

            /// 0b1011: - 1.00%
            pub const REG1_ADJ_11: u32 = 0b1011;

            /// 0b1100: - 1.25%
            pub const REG1_ADJ_12: u32 = 0b1100;

            /// 0b1101: - 1.50%
            pub const REG1_ADJ_13: u32 = 0b1101;

            /// 0b1110: - 1.75%
            pub const REG1_ADJ_14: u32 = 0b1110;

            /// 0b1111: - 2.00%
            pub const REG1_ADJ_15: u32 = 0b1111;
        }
    }

    /// This field defines the target voltage for the SOC power domain
    pub mod REG2_TARG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Power gated off
            pub const REG2_TARG_0: u32 = 0b00000;

            /// 0b00001: Target core voltage = 0.725V
            pub const REG2_TARG_1: u32 = 0b00001;

            /// 0b00010: Target core voltage = 0.750V
            pub const REG2_TARG_2: u32 = 0b00010;

            /// 0b00011: Target core voltage = 0.775V
            pub const REG2_TARG_3: u32 = 0b00011;

            /// 0b10000: Target core voltage = 1.100V
            pub const REG2_TARG_16: u32 = 0b10000;

            /// 0b11110: Target core voltage = 1.450V
            pub const REG2_TARG_30: u32 = 0b11110;

            /// 0b11111: Power FET switched full on. No regulation.
            pub const REG2_TARG_31: u32 = 0b11111;
        }
    }

    /// This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.
    pub mod REG2_ADJ {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (4 bits: 0b1111 << 23)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No adjustment
            pub const REG2_ADJ_0: u32 = 0b0000;

            /// 0b0001: + 0.25%
            pub const REG2_ADJ_1: u32 = 0b0001;

            /// 0b0010: + 0.50%
            pub const REG2_ADJ_2: u32 = 0b0010;

            /// 0b0011: + 0.75%
            pub const REG2_ADJ_3: u32 = 0b0011;

            /// 0b0100: + 1.00%
            pub const REG2_ADJ_4: u32 = 0b0100;

            /// 0b0101: + 1.25%
            pub const REG2_ADJ_5: u32 = 0b0101;

            /// 0b0110: + 1.50%
            pub const REG2_ADJ_6: u32 = 0b0110;

            /// 0b0111: + 1.75%
            pub const REG2_ADJ_7: u32 = 0b0111;

            /// 0b1000: - 0.25%
            pub const REG2_ADJ_8: u32 = 0b1000;

            /// 0b1001: - 0.50%
            pub const REG2_ADJ_9: u32 = 0b1001;

            /// 0b1010: - 0.75%
            pub const REG2_ADJ_10: u32 = 0b1010;

            /// 0b1011: - 1.00%
            pub const REG2_ADJ_11: u32 = 0b1011;

            /// 0b1100: - 1.25%
            pub const REG2_ADJ_12: u32 = 0b1100;

            /// 0b1101: - 1.50%
            pub const REG2_ADJ_13: u32 = 0b1101;

            /// 0b1110: - 1.75%
            pub const REG2_ADJ_14: u32 = 0b1110;

            /// 0b1111: - 2.00%
            pub const REG2_ADJ_15: u32 = 0b1111;
        }
    }

    /// Regulator voltage ramp rate.
    pub mod RAMP_RATE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (2 bits: 0b11 << 27)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Fast
            pub const RAMP_RATE_0: u32 = 0b00;

            /// 0b01: Medium Fast
            pub const RAMP_RATE_1: u32 = 0b01;

            /// 0b10: Medium Slow
            pub const RAMP_RATE_2: u32 = 0b10;

            /// 0b11: Slow
            pub const RAMP_RATE_3: u32 = 0b11;
        }
    }

    /// If set, increases the gate drive on power gating FETs to reduce leakage in the off state
    pub mod FET_ODRIVE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Digital Regulator Core Register
pub mod REG_CORE_SET {
    pub use super::REG_CORE::FET_ODRIVE;
    pub use super::REG_CORE::RAMP_RATE;
    pub use super::REG_CORE::REG0_ADJ;
    pub use super::REG_CORE::REG0_TARG;
    pub use super::REG_CORE::REG1_ADJ;
    pub use super::REG_CORE::REG1_TARG;
    pub use super::REG_CORE::REG2_ADJ;
    pub use super::REG_CORE::REG2_TARG;
}

/// Digital Regulator Core Register
pub mod REG_CORE_CLR {
    pub use super::REG_CORE::FET_ODRIVE;
    pub use super::REG_CORE::RAMP_RATE;
    pub use super::REG_CORE::REG0_ADJ;
    pub use super::REG_CORE::REG0_TARG;
    pub use super::REG_CORE::REG1_ADJ;
    pub use super::REG_CORE::REG1_TARG;
    pub use super::REG_CORE::REG2_ADJ;
    pub use super::REG_CORE::REG2_TARG;
}

/// Digital Regulator Core Register
pub mod REG_CORE_TOG {
    pub use super::REG_CORE::FET_ODRIVE;
    pub use super::REG_CORE::RAMP_RATE;
    pub use super::REG_CORE::REG0_ADJ;
    pub use super::REG_CORE::REG0_TARG;
    pub use super::REG_CORE::REG1_ADJ;
    pub use super::REG_CORE::REG1_TARG;
    pub use super::REG_CORE::REG2_ADJ;
    pub use super::REG_CORE::REG2_TARG;
}

/// Miscellaneous Register 0
pub mod MISC0 {

    /// Control bit to power-down the analog bandgap reference circuitry
    pub mod REFTOP_PWD {
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

    /// Control bit to disable the self-bias circuit in the analog bandgap
    pub mod REFTOP_SELFBIASOFF {
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

            /// 0b0: Uses coarse bias currents for startup
            pub const REFTOP_SELFBIASOFF_0: u32 = 0b0;

            /// 0b1: Uses bandgap-based bias currents for best performance.
            pub const REFTOP_SELFBIASOFF_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod REFTOP_VBGADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Nominal VBG
            pub const REFTOP_VBGADJ_0: u32 = 0b000;

            /// 0b001: VBG+0.78%
            pub const REFTOP_VBGADJ_1: u32 = 0b001;

            /// 0b010: VBG+1.56%
            pub const REFTOP_VBGADJ_2: u32 = 0b010;

            /// 0b011: VBG+2.34%
            pub const REFTOP_VBGADJ_3: u32 = 0b011;

            /// 0b100: VBG-0.78%
            pub const REFTOP_VBGADJ_4: u32 = 0b100;

            /// 0b101: VBG-1.56%
            pub const REFTOP_VBGADJ_5: u32 = 0b101;

            /// 0b110: VBG-2.34%
            pub const REFTOP_VBGADJ_6: u32 = 0b110;

            /// 0b111: VBG-3.12%
            pub const REFTOP_VBGADJ_7: u32 = 0b111;
        }
    }

    /// Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable.
    pub mod REFTOP_VBGUP {
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

    /// Configure the analog behavior in stop mode.
    pub mod STOP_MODE_CONFIG {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SUSPEND (DSM)
            pub const STOP_MODE_CONFIG_0: u32 = 0b00;

            /// 0b01: Analog regulators are ON.
            pub const STANDBY: u32 = 0b01;

            /// 0b10: STOP (lower power)
            pub const STOP_MODE_CONFIG_2: u32 = 0b10;

            /// 0b11: STOP (very lower power)
            pub const STOP_MODE_CONFIG_3: u32 = 0b11;
        }
    }

    /// This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN.
    pub mod DISCON_HIGH_SNVS {
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

            /// 0b0: Turn on the switch
            pub const DISCON_HIGH_SNVS_0: u32 = 0b0;

            /// 0b1: Turn off the switch
            pub const DISCON_HIGH_SNVS_1: u32 = 0b1;
        }
    }

    /// This field determines the bias current in the 24MHz oscillator
    pub mod OSC_I {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Nominal
            pub const NOMINAL: u32 = 0b00;

            /// 0b01: Decrease current by 12.5%
            pub const MINUS_12_5_PERCENT: u32 = 0b01;

            /// 0b10: Decrease current by 25.0%
            pub const MINUS_25_PERCENT: u32 = 0b10;

            /// 0b11: Decrease current by 37.5%
            pub const MINUS_37_5_PERCENT: u32 = 0b11;
        }
    }

    /// Status bit that signals that the output of the 24-MHz crystal oscillator is stable
    pub mod OSC_XTALOK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit enables the detector that signals when the 24MHz crystal oscillator is stable
    pub mod OSC_XTALOK_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block
    pub mod CLKGATE_CTRL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allow the logic to automatically gate the clock when the XTAL is powered down.
            pub const ALLOW_AUTO_GATE: u32 = 0b0;

            /// 0b1: Prevent the logic from ever gating off the clock.
            pub const NO_AUTO_GATE: u32 = 0b1;
        }
    }

    /// This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block
    pub mod CLKGATE_DELAY {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0.5ms
            pub const CLKGATE_DELAY_0: u32 = 0b000;

            /// 0b001: 1.0ms
            pub const CLKGATE_DELAY_1: u32 = 0b001;

            /// 0b010: 2.0ms
            pub const CLKGATE_DELAY_2: u32 = 0b010;

            /// 0b011: 3.0ms
            pub const CLKGATE_DELAY_3: u32 = 0b011;

            /// 0b100: 4.0ms
            pub const CLKGATE_DELAY_4: u32 = 0b100;

            /// 0b101: 5.0ms
            pub const CLKGATE_DELAY_5: u32 = 0b101;

            /// 0b110: 6.0ms
            pub const CLKGATE_DELAY_6: u32 = 0b110;

            /// 0b111: 7.0ms
            pub const CLKGATE_DELAY_7: u32 = 0b111;
        }
    }

    /// This field indicates which chip source is being used for the rtc clock.
    pub mod RTC_XTAL_SOURCE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal ring oscillator
            pub const RTC_XTAL_SOURCE_0: u32 = 0b0;

            /// 0b1: RTC_XTAL
            pub const RTC_XTAL_SOURCE_1: u32 = 0b1;
        }
    }

    /// This field powers down the 24M crystal oscillator if set true.
    pub mod XTAL_24M_PWD {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Predivider for the source clock of the PLL's.
    pub mod VID_PLL_PREDIV {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Divide by 1
            pub const VID_PLL_PREDIV_0: u32 = 0b0;

            /// 0b1: Divide by 2
            pub const VID_PLL_PREDIV_1: u32 = 0b1;
        }
    }
}

/// Miscellaneous Register 0
pub mod MISC0_SET {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_CLR {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_TOG {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 1
pub mod MISC1 {

    /// This field selects the clk to be routed to anaclk1/1b.Not related to PMU.
    pub mod LVDS1_CLK_SEL {
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

            /// 0b00000: Arm PLL
            pub const ARM_PLL: u32 = 0b00000;

            /// 0b00001: System PLL
            pub const SYS_PLL: u32 = 0b00001;

            /// 0b00010: ref_pfd4_clk == pll2_pfd0_clk
            pub const PFD4: u32 = 0b00010;

            /// 0b00011: ref_pfd5_clk == pll2_pfd1_clk
            pub const PFD5: u32 = 0b00011;

            /// 0b00100: ref_pfd6_clk == pll2_pfd2_clk
            pub const PFD6: u32 = 0b00100;

            /// 0b00101: ref_pfd7_clk == pll2_pfd3_clk
            pub const PFD7: u32 = 0b00101;

            /// 0b00110: Audio PLL
            pub const AUDIO_PLL: u32 = 0b00110;

            /// 0b00111: Video PLL
            pub const VIDEO_PLL: u32 = 0b00111;

            /// 0b01001: ethernet ref clock (ENET_PLL)
            pub const ETHERNET_REF: u32 = 0b01001;

            /// 0b01100: USB1 PLL clock
            pub const USB1_PLL: u32 = 0b01100;

            /// 0b01101: USB2 PLL clock
            pub const USB2_PLL: u32 = 0b01101;

            /// 0b01110: ref_pfd0_clk == pll3_pfd0_clk
            pub const PFD0: u32 = 0b01110;

            /// 0b01111: ref_pfd1_clk == pll3_pfd1_clk
            pub const PFD1: u32 = 0b01111;

            /// 0b10000: ref_pfd2_clk == pll3_pfd2_clk
            pub const PFD2: u32 = 0b10000;

            /// 0b10001: ref_pfd3_clk == pll3_pfd3_clk
            pub const PFD3: u32 = 0b10001;

            /// 0b10010: xtal (24M)
            pub const XTAL: u32 = 0b10010;
        }
    }

    /// This field selects the clk to be routed to anaclk2/2b.Not related to PMU.
    pub mod LVDS2_CLK_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (5 bits: 0b11111 << 5)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Arm PLL
            pub const ARM_PLL: u32 = 0b00000;

            /// 0b00001: System PLL
            pub const SYS_PLL: u32 = 0b00001;

            /// 0b00010: ref_pfd4_clk == pll2_pfd0_clk
            pub const PFD4: u32 = 0b00010;

            /// 0b00011: ref_pfd5_clk == pll2_pfd1_clk
            pub const PFD5: u32 = 0b00011;

            /// 0b00100: ref_pfd6_clk == pll2_pfd2_clk
            pub const PFD6: u32 = 0b00100;

            /// 0b00101: ref_pfd7_clk == pll2_pfd3_clk
            pub const PFD7: u32 = 0b00101;

            /// 0b00110: Audio PLL
            pub const AUDIO_PLL: u32 = 0b00110;

            /// 0b00111: Video PLL
            pub const VIDEO_PLL: u32 = 0b00111;

            /// 0b01000: MLB PLL
            pub const MLB_PLL: u32 = 0b01000;

            /// 0b01001: ethernet ref clock (ENET_PLL)
            pub const ETHERNET_REF: u32 = 0b01001;

            /// 0b01010: PCIe ref clock (125M)
            pub const PCIE_REF: u32 = 0b01010;

            /// 0b01011: SATA ref clock (100M)
            pub const SATA_REF: u32 = 0b01011;

            /// 0b01100: USB1 PLL clock
            pub const USB1_PLL: u32 = 0b01100;

            /// 0b01101: USB2 PLL clock
            pub const USB2_PLL: u32 = 0b01101;

            /// 0b01110: ref_pfd0_clk == pll3_pfd0_clk
            pub const PFD0: u32 = 0b01110;

            /// 0b01111: ref_pfd1_clk == pll3_pfd1_clk
            pub const PFD1: u32 = 0b01111;

            /// 0b10000: ref_pfd2_clk == pll3_pfd2_clk
            pub const PFD2: u32 = 0b10000;

            /// 0b10001: ref_pfd3_clk == pll3_pfd3_clk
            pub const PFD3: u32 = 0b10001;

            /// 0b10010: xtal (24M)
            pub const XTAL: u32 = 0b10010;

            /// 0b10011: LVDS1 (loopback)
            pub const LVDS1: u32 = 0b10011;

            /// 0b10100: LVDS2 (not useful)
            pub const LVDS2: u32 = 0b10100;
        }
    }

    /// This enables the LVDS output buffer for anaclk1/1b
    pub mod LVDSCLK1_OBEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables the LVDS output buffer for anaclk2/2b
    pub mod LVDSCLK2_OBEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables the LVDS input buffer for anaclk1/1b
    pub mod LVDSCLK1_IBEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables the LVDS input buffer for anaclk2/2b
    pub mod LVDSCLK2_IBEN {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off
    pub mod PFD_480_AUTOGATE_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off
    pub mod PFD_528_AUTOGATE_EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature
    pub mod IRQ_TEMPPANIC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor low interrupt asserts for low temperature
    pub mod IRQ_TEMPLOW {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor high interrupt asserts for high temperature
    pub mod IRQ_TEMPHIGH {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when when any of the analog regulator brownout interrupts assert
    pub mod IRQ_ANA_BO {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when when any of the digital regulator brownout interrupts assert
    pub mod IRQ_DIG_BO {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Miscellaneous Register 1
pub mod MISC1_SET {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::LVDS1_CLK_SEL;
    pub use super::MISC1::LVDS2_CLK_SEL;
    pub use super::MISC1::LVDSCLK1_IBEN;
    pub use super::MISC1::LVDSCLK1_OBEN;
    pub use super::MISC1::LVDSCLK2_IBEN;
    pub use super::MISC1::LVDSCLK2_OBEN;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Register 1
pub mod MISC1_CLR {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::LVDS1_CLK_SEL;
    pub use super::MISC1::LVDS2_CLK_SEL;
    pub use super::MISC1::LVDSCLK1_IBEN;
    pub use super::MISC1::LVDSCLK1_OBEN;
    pub use super::MISC1::LVDSCLK2_IBEN;
    pub use super::MISC1::LVDSCLK2_OBEN;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Register 1
pub mod MISC1_TOG {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::LVDS1_CLK_SEL;
    pub use super::MISC1::LVDS2_CLK_SEL;
    pub use super::MISC1::LVDSCLK1_IBEN;
    pub use super::MISC1::LVDSCLK1_OBEN;
    pub use super::MISC1::LVDSCLK2_IBEN;
    pub use super::MISC1::LVDSCLK2_OBEN;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Control Register
pub mod MISC2 {

    /// This field defines the brown out voltage offset for the CORE power domain
    pub mod REG0_BO_OFFSET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG0_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG0_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg0 brownout status bit.
    pub mod REG0_BO_STATUS {
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

            /// 0b1: Brownout, supply is below target minus brownout offset.
            pub const REG0_BO_STATUS_1: u32 = 0b1;
        }
    }

    /// Enables the brownout detection.
    pub mod REG0_ENABLE_BO {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Default value of "0"
    pub mod PLL3_disable {
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

    /// This field defines the brown out voltage offset for the xPU power domain
    pub mod REG1_BO_OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG1_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG1_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg1 brownout status bit.
    pub mod REG1_BO_STATUS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Brownout, supply is below target minus brownout offset.
            pub const REG1_BO_STATUS_1: u32 = 0b1;
        }
    }

    /// Enables the brownout detection.
    pub mod REG1_ENABLE_BO {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSB of Post-divider for Audio PLL
    pub mod AUDIO_DIV_LSB {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: divide by 1 (Default)
            pub const AUDIO_DIV_LSB_0: u32 = 0b0;

            /// 0b1: divide by 2
            pub const AUDIO_DIV_LSB_1: u32 = 0b1;
        }
    }

    /// This field defines the brown out voltage offset for the xPU power domain
    pub mod REG2_BO_OFFSET {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG2_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG2_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg2 brownout status bit.
    pub mod REG2_BO_STATUS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the brownout detection.
    pub mod REG2_ENABLE_BO {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Signals that the voltage is above the brownout level for the SOC supply
    pub mod REG2_OK {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSB of Post-divider for Audio PLL
    pub mod AUDIO_DIV_MSB {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: divide by 1 (Default)
            pub const AUDIO_DIV_MSB_0: u32 = 0b0;

            /// 0b1: divide by 2
            pub const AUDIO_DIV_MSB_1: u32 = 0b1;
        }
    }

    /// Number of clock periods (24MHz clock).
    pub mod REG0_STEP_TIME {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64
            pub const _64_CLOCKS: u32 = 0b00;

            /// 0b01: 128
            pub const _128_CLOCKS: u32 = 0b01;

            /// 0b10: 256
            pub const _256_CLOCKS: u32 = 0b10;

            /// 0b11: 512
            pub const _512_CLOCKS: u32 = 0b11;
        }
    }

    /// Number of clock periods (24MHz clock).
    pub mod REG1_STEP_TIME {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::REG0_STEP_TIME::RW;
    }

    /// Number of clock periods (24MHz clock).
    pub mod REG2_STEP_TIME {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::REG0_STEP_TIME::RW;
    }

    /// Post-divider for video
    pub mod VIDEO_DIV {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: divide by 1 (Default)
            pub const VIDEO_DIV_0: u32 = 0b00;

            /// 0b01: divide by 2
            pub const VIDEO_DIV_1: u32 = 0b01;

            /// 0b10: divide by 1
            pub const VIDEO_DIV_2: u32 = 0b10;

            /// 0b11: divide by 4
            pub const VIDEO_DIV_3: u32 = 0b11;
        }
    }
}

/// Miscellaneous Control Register
pub mod MISC2_SET {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}

/// Miscellaneous Control Register
pub mod MISC2_CLR {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}

/// Miscellaneous Control Register
pub mod MISC2_TOG {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}
pub struct RegisterBlock {
    _reserved1: [u32; 68],

    /// Regulator 1P1 Register
    pub REG_1P1: RWRegister<u32>,

    /// Regulator 1P1 Register
    pub REG_1P1_SET: RWRegister<u32>,

    /// Regulator 1P1 Register
    pub REG_1P1_CLR: RWRegister<u32>,

    /// Regulator 1P1 Register
    pub REG_1P1_TOG: RWRegister<u32>,

    /// Regulator 3P0 Register
    pub REG_3P0: RWRegister<u32>,

    /// Regulator 3P0 Register
    pub REG_3P0_SET: RWRegister<u32>,

    /// Regulator 3P0 Register
    pub REG_3P0_CLR: RWRegister<u32>,

    /// Regulator 3P0 Register
    pub REG_3P0_TOG: RWRegister<u32>,

    /// Regulator 2P5 Register
    pub REG_2P5: RWRegister<u32>,

    /// Regulator 2P5 Register
    pub REG_2P5_SET: RWRegister<u32>,

    /// Regulator 2P5 Register
    pub REG_2P5_CLR: RWRegister<u32>,

    /// Regulator 2P5 Register
    pub REG_2P5_TOG: RWRegister<u32>,

    /// Digital Regulator Core Register
    pub REG_CORE: RWRegister<u32>,

    /// Digital Regulator Core Register
    pub REG_CORE_SET: RWRegister<u32>,

    /// Digital Regulator Core Register
    pub REG_CORE_CLR: RWRegister<u32>,

    /// Digital Regulator Core Register
    pub REG_CORE_TOG: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_SET: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_CLR: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_TOG: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_SET: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_CLR: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_TOG: RWRegister<u32>,

    /// Miscellaneous Control Register
    pub MISC2: RWRegister<u32>,

    /// Miscellaneous Control Register
    pub MISC2_SET: RWRegister<u32>,

    /// Miscellaneous Control Register
    pub MISC2_CLR: RWRegister<u32>,

    /// Miscellaneous Control Register
    pub MISC2_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub REG_1P1: u32,
    pub REG_1P1_SET: u32,
    pub REG_1P1_CLR: u32,
    pub REG_1P1_TOG: u32,
    pub REG_3P0: u32,
    pub REG_3P0_SET: u32,
    pub REG_3P0_CLR: u32,
    pub REG_3P0_TOG: u32,
    pub REG_2P5: u32,
    pub REG_2P5_SET: u32,
    pub REG_2P5_CLR: u32,
    pub REG_2P5_TOG: u32,
    pub REG_CORE: u32,
    pub REG_CORE_SET: u32,
    pub REG_CORE_CLR: u32,
    pub REG_CORE_TOG: u32,
    pub MISC0: u32,
    pub MISC0_SET: u32,
    pub MISC0_CLR: u32,
    pub MISC0_TOG: u32,
    pub MISC1: u32,
    pub MISC1_SET: u32,
    pub MISC1_CLR: u32,
    pub MISC1_TOG: u32,
    pub MISC2: u32,
    pub MISC2_SET: u32,
    pub MISC2_CLR: u32,
    pub MISC2_TOG: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
