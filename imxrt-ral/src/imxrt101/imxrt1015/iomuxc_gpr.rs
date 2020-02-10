#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC_GPR

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPR0 General Purpose Register
pub mod GPR0 {}

/// GPR1 General Purpose Register
pub mod GPR1 {

    /// SAI1 MCLK1 source select
    pub mod SAI1_MCLK1_SEL {
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

            /// 0b000: ccm.ssi1_clk_root
            pub const SAI1_MCLK1_SEL_0: u32 = 0b000;

            /// 0b001: ccm.ssi2_clk_root
            pub const SAI1_MCLK1_SEL_1: u32 = 0b001;

            /// 0b010: ccm.ssi3_clk_root
            pub const SAI1_MCLK1_SEL_2: u32 = 0b010;

            /// 0b011: iomux.sai1_ipg_clk_sai_mclk
            pub const SAI1_MCLK1_SEL_3: u32 = 0b011;

            /// 0b100: iomux.sai2_ipg_clk_sai_mclk
            pub const SAI1_MCLK1_SEL_4: u32 = 0b100;

            /// 0b101: iomux.sai3_ipg_clk_sai_mclk
            pub const SAI1_MCLK1_SEL_5: u32 = 0b101;
        }
    }

    /// SAI1 MCLK2 source select
    pub mod SAI1_MCLK2_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: ccm.ssi1_clk_root
            pub const SAI1_MCLK2_SEL_0: u32 = 0b000;

            /// 0b001: ccm.ssi2_clk_root
            pub const SAI1_MCLK2_SEL_1: u32 = 0b001;

            /// 0b010: ccm.ssi3_clk_root
            pub const SAI1_MCLK2_SEL_2: u32 = 0b010;

            /// 0b011: iomux.sai1_ipg_clk_sai_mclk
            pub const SAI1_MCLK2_SEL_3: u32 = 0b011;

            /// 0b100: iomux.sai2_ipg_clk_sai_mclk
            pub const SAI1_MCLK2_SEL_4: u32 = 0b100;

            /// 0b101: iomux.sai3_ipg_clk_sai_mclk
            pub const SAI1_MCLK2_SEL_5: u32 = 0b101;
        }
    }

    /// SAI1 MCLK3 source select
    pub mod SAI1_MCLK3_SEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: ccm.spdif0_clk_root
            pub const SAI1_MCLK3_SEL_0: u32 = 0b00;

            /// 0b01: SPDIF_EXT_CLK
            pub const SAI1_MCLK3_SEL_1: u32 = 0b01;

            /// 0b10: spdif.spdif_srclk
            pub const SAI1_MCLK3_SEL_2: u32 = 0b10;

            /// 0b11: spdif.spdif_outclock
            pub const SAI1_MCLK3_SEL_3: u32 = 0b11;
        }
    }

    /// SAI2 MCLK3 source select
    pub mod SAI2_MCLK3_SEL {
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

            /// 0b00: ccm.spdif0_clk_root
            pub const SAI2_MCLK3_SEL_0: u32 = 0b00;

            /// 0b01: SPDIF_EXT_CLK
            pub const SAI2_MCLK3_SEL_1: u32 = 0b01;

            /// 0b10: spdif.spdif_srclk
            pub const SAI2_MCLK3_SEL_2: u32 = 0b10;

            /// 0b11: spdif.spdif_outclock
            pub const SAI2_MCLK3_SEL_3: u32 = 0b11;
        }
    }

    /// SAI3 MCLK3 source select
    pub mod SAI3_MCLK3_SEL {
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

            /// 0b00: ccm.spdif0_clk_root
            pub const SAI3_MCLK3_SEL_0: u32 = 0b00;

            /// 0b01: SPDIF_EXT_CLK
            pub const SAI3_MCLK3_SEL_1: u32 = 0b01;

            /// 0b10: spdif.spdif_srclk
            pub const SAI3_MCLK3_SEL_2: u32 = 0b10;

            /// 0b11: spdif.spdif_outclock
            pub const SAI3_MCLK3_SEL_3: u32 = 0b11;
        }
    }

    /// Global Interrupt
    pub mod GINT {
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

            /// 0b0: Global interrupt request is not asserted.
            pub const GINT_0: u32 = 0b0;

            /// 0b1: Global interrupt request is asserted.
            pub const GINT_1: u32 = 0b1;
        }
    }

    /// sai1.MCLK signal direction control
    pub mod SAI1_MCLK_DIR {
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

            /// 0b0: sai1.MCLK is input signal
            pub const SAI1_MCLK_DIR_0: u32 = 0b0;

            /// 0b1: sai1.MCLK is output signal
            pub const SAI1_MCLK_DIR_1: u32 = 0b1;
        }
    }

    /// sai2.MCLK signal direction control
    pub mod SAI2_MCLK_DIR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: sai2.MCLK is input signal
            pub const SAI2_MCLK_DIR_0: u32 = 0b0;

            /// 0b1: sai2.MCLK is output signal
            pub const SAI2_MCLK_DIR_1: u32 = 0b1;
        }
    }

    /// sai3.MCLK signal direction control
    pub mod SAI3_MCLK_DIR {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: sai3.MCLK is input signal
            pub const SAI3_MCLK_DIR_0: u32 = 0b0;

            /// 0b1: sai3.MCLK is output signal
            pub const SAI3_MCLK_DIR_1: u32 = 0b1;
        }
    }

    /// Exclusive monitor response select of illegal command
    pub mod EXC_MON {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OKAY response
            pub const EXC_MON_0: u32 = 0b0;

            /// 0b1: SLVError response
            pub const EXC_MON_1: u32 = 0b1;
        }
    }

    /// ARM CM7 platform AHB clock enable
    pub mod CM7_FORCE_HCLK_EN {
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

            /// 0b0: AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible.
            pub const CM7_FORCE_HCLK_EN_0: u32 = 0b0;

            /// 0b1: AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible.
            pub const CM7_FORCE_HCLK_EN_1: u32 = 0b1;
        }
    }
}

/// GPR2 General Purpose Register
pub mod GPR2 {

    /// Enable power saving features on L2 memory
    pub mod L2_MEM_EN_POWERSAVING {
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

            /// 0b0: Enters power saving mode only when chip is in SUSPEND mode
            pub const L2_MEM_EN_POWERSAVING_0: u32 = 0b0;

            /// 0b1: Controlled by L2_MEM_DEEPSLEEP bitfield
            pub const L2_MEM_EN_POWERSAVING_1: u32 = 0b1;
        }
    }

    /// Automatically gate off RAM clock when RAM is not accessed.
    pub mod RAM_AUTO_CLK_GATING_EN {
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

            /// 0b0: disable automatically gate off RAM clock
            pub const RAM_AUTO_CLK_GATING_EN_0: u32 = 0b0;

            /// 0b1: enable automatically gate off RAM clock
            pub const RAM_AUTO_CLK_GATING_EN_1: u32 = 0b1;
        }
    }

    /// This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low
    pub mod L2_MEM_DEEPSLEEP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No force sleep control supported, memory deep sleep mode only entered when whole system in stop mode (OCRAM in normal mode)
            pub const L2_MEM_DEEPSLEEP_0: u32 = 0b0;

            /// 0b1: Force memory into deep sleep mode (OCRAM in power saving mode)
            pub const L2_MEM_DEEPSLEEP_1: u32 = 0b1;
        }
    }

    /// Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency.
    pub mod MQS_CLK_DIV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: mclk frequency = 1/1 * hmclk frequency
            pub const DIVIDE_1: u32 = 0b00000000;

            /// 0b00000001: mclk frequency = 1/2 * hmclk frequency
            pub const DIVIDE_2: u32 = 0b00000001;

            /// 0b00000010: mclk frequency = 1/3 * hmclk frequency
            pub const DIVIDE_3: u32 = 0b00000010;

            /// 0b00000011: mclk frequency = 1/4 * hmclk frequency
            pub const DIVIDE_4: u32 = 0b00000011;

            /// 0b00000100: mclk frequency = 1/5 * hmclk frequency
            pub const DIVIDE_5: u32 = 0b00000100;

            /// 0b00000101: mclk frequency = 1/6 * hmclk frequency
            pub const DIVIDE_6: u32 = 0b00000101;

            /// 0b00000110: mclk frequency = 1/7 * hmclk frequency
            pub const DIVIDE_7: u32 = 0b00000110;

            /// 0b00000111: mclk frequency = 1/8 * hmclk frequency
            pub const DIVIDE_8: u32 = 0b00000111;

            /// 0b00001000: mclk frequency = 1/9 * hmclk frequency
            pub const DIVIDE_9: u32 = 0b00001000;

            /// 0b00001001: mclk frequency = 1/10 * hmclk frequency
            pub const DIVIDE_10: u32 = 0b00001001;

            /// 0b00001010: mclk frequency = 1/11 * hmclk frequency
            pub const DIVIDE_11: u32 = 0b00001010;

            /// 0b00001011: mclk frequency = 1/12 * hmclk frequency
            pub const DIVIDE_12: u32 = 0b00001011;

            /// 0b00001100: mclk frequency = 1/13 * hmclk frequency
            pub const DIVIDE_13: u32 = 0b00001100;

            /// 0b00001101: mclk frequency = 1/14 * hmclk frequency
            pub const DIVIDE_14: u32 = 0b00001101;

            /// 0b00001110: mclk frequency = 1/15 * hmclk frequency
            pub const DIVIDE_15: u32 = 0b00001110;

            /// 0b00001111: mclk frequency = 1/16 * hmclk frequency
            pub const DIVIDE_16: u32 = 0b00001111;

            /// 0b00010000: mclk frequency = 1/17 * hmclk frequency
            pub const DIVIDE_17: u32 = 0b00010000;

            /// 0b00010001: mclk frequency = 1/18 * hmclk frequency
            pub const DIVIDE_18: u32 = 0b00010001;

            /// 0b00010010: mclk frequency = 1/19 * hmclk frequency
            pub const DIVIDE_19: u32 = 0b00010010;

            /// 0b00010011: mclk frequency = 1/20 * hmclk frequency
            pub const DIVIDE_20: u32 = 0b00010011;

            /// 0b00010100: mclk frequency = 1/21 * hmclk frequency
            pub const DIVIDE_21: u32 = 0b00010100;

            /// 0b00010101: mclk frequency = 1/22 * hmclk frequency
            pub const DIVIDE_22: u32 = 0b00010101;

            /// 0b00010110: mclk frequency = 1/23 * hmclk frequency
            pub const DIVIDE_23: u32 = 0b00010110;

            /// 0b00010111: mclk frequency = 1/24 * hmclk frequency
            pub const DIVIDE_24: u32 = 0b00010111;

            /// 0b00011000: mclk frequency = 1/25 * hmclk frequency
            pub const DIVIDE_25: u32 = 0b00011000;

            /// 0b00011001: mclk frequency = 1/26 * hmclk frequency
            pub const DIVIDE_26: u32 = 0b00011001;

            /// 0b00011010: mclk frequency = 1/27 * hmclk frequency
            pub const DIVIDE_27: u32 = 0b00011010;

            /// 0b00011011: mclk frequency = 1/28 * hmclk frequency
            pub const DIVIDE_28: u32 = 0b00011011;

            /// 0b00011100: mclk frequency = 1/29 * hmclk frequency
            pub const DIVIDE_29: u32 = 0b00011100;

            /// 0b00011101: mclk frequency = 1/30 * hmclk frequency
            pub const DIVIDE_30: u32 = 0b00011101;

            /// 0b00011110: mclk frequency = 1/31 * hmclk frequency
            pub const DIVIDE_31: u32 = 0b00011110;

            /// 0b00011111: mclk frequency = 1/32 * hmclk frequency
            pub const DIVIDE_32: u32 = 0b00011111;

            /// 0b00100000: mclk frequency = 1/33 * hmclk frequency
            pub const DIVIDE_33: u32 = 0b00100000;

            /// 0b00100001: mclk frequency = 1/34 * hmclk frequency
            pub const DIVIDE_34: u32 = 0b00100001;

            /// 0b00100010: mclk frequency = 1/35 * hmclk frequency
            pub const DIVIDE_35: u32 = 0b00100010;

            /// 0b00100011: mclk frequency = 1/36 * hmclk frequency
            pub const DIVIDE_36: u32 = 0b00100011;

            /// 0b00100100: mclk frequency = 1/37 * hmclk frequency
            pub const DIVIDE_37: u32 = 0b00100100;

            /// 0b00100101: mclk frequency = 1/38 * hmclk frequency
            pub const DIVIDE_38: u32 = 0b00100101;

            /// 0b00100110: mclk frequency = 1/39 * hmclk frequency
            pub const DIVIDE_39: u32 = 0b00100110;

            /// 0b00100111: mclk frequency = 1/40 * hmclk frequency
            pub const DIVIDE_40: u32 = 0b00100111;

            /// 0b00101000: mclk frequency = 1/41 * hmclk frequency
            pub const DIVIDE_41: u32 = 0b00101000;

            /// 0b00101001: mclk frequency = 1/42 * hmclk frequency
            pub const DIVIDE_42: u32 = 0b00101001;

            /// 0b00101010: mclk frequency = 1/43 * hmclk frequency
            pub const DIVIDE_43: u32 = 0b00101010;

            /// 0b00101011: mclk frequency = 1/44 * hmclk frequency
            pub const DIVIDE_44: u32 = 0b00101011;

            /// 0b00101100: mclk frequency = 1/45 * hmclk frequency
            pub const DIVIDE_45: u32 = 0b00101100;

            /// 0b00101101: mclk frequency = 1/46 * hmclk frequency
            pub const DIVIDE_46: u32 = 0b00101101;

            /// 0b00101110: mclk frequency = 1/47 * hmclk frequency
            pub const DIVIDE_47: u32 = 0b00101110;

            /// 0b00101111: mclk frequency = 1/48 * hmclk frequency
            pub const DIVIDE_48: u32 = 0b00101111;

            /// 0b00110000: mclk frequency = 1/49 * hmclk frequency
            pub const DIVIDE_49: u32 = 0b00110000;

            /// 0b00110001: mclk frequency = 1/50 * hmclk frequency
            pub const DIVIDE_50: u32 = 0b00110001;

            /// 0b00110010: mclk frequency = 1/51 * hmclk frequency
            pub const DIVIDE_51: u32 = 0b00110010;

            /// 0b00110011: mclk frequency = 1/52 * hmclk frequency
            pub const DIVIDE_52: u32 = 0b00110011;

            /// 0b00110100: mclk frequency = 1/53 * hmclk frequency
            pub const DIVIDE_53: u32 = 0b00110100;

            /// 0b00110101: mclk frequency = 1/54 * hmclk frequency
            pub const DIVIDE_54: u32 = 0b00110101;

            /// 0b00110110: mclk frequency = 1/55 * hmclk frequency
            pub const DIVIDE_55: u32 = 0b00110110;

            /// 0b00110111: mclk frequency = 1/56 * hmclk frequency
            pub const DIVIDE_56: u32 = 0b00110111;

            /// 0b00111000: mclk frequency = 1/57 * hmclk frequency
            pub const DIVIDE_57: u32 = 0b00111000;

            /// 0b00111001: mclk frequency = 1/58 * hmclk frequency
            pub const DIVIDE_58: u32 = 0b00111001;

            /// 0b00111010: mclk frequency = 1/59 * hmclk frequency
            pub const DIVIDE_59: u32 = 0b00111010;

            /// 0b00111011: mclk frequency = 1/60 * hmclk frequency
            pub const DIVIDE_60: u32 = 0b00111011;

            /// 0b00111100: mclk frequency = 1/61 * hmclk frequency
            pub const DIVIDE_61: u32 = 0b00111100;

            /// 0b00111101: mclk frequency = 1/62 * hmclk frequency
            pub const DIVIDE_62: u32 = 0b00111101;

            /// 0b00111110: mclk frequency = 1/63 * hmclk frequency
            pub const DIVIDE_63: u32 = 0b00111110;

            /// 0b00111111: mclk frequency = 1/64 * hmclk frequency
            pub const DIVIDE_64: u32 = 0b00111111;

            /// 0b01000000: mclk frequency = 1/65 * hmclk frequency
            pub const DIVIDE_65: u32 = 0b01000000;

            /// 0b01000001: mclk frequency = 1/66 * hmclk frequency
            pub const DIVIDE_66: u32 = 0b01000001;

            /// 0b01000010: mclk frequency = 1/67 * hmclk frequency
            pub const DIVIDE_67: u32 = 0b01000010;

            /// 0b01000011: mclk frequency = 1/68 * hmclk frequency
            pub const DIVIDE_68: u32 = 0b01000011;

            /// 0b01000100: mclk frequency = 1/69 * hmclk frequency
            pub const DIVIDE_69: u32 = 0b01000100;

            /// 0b01000101: mclk frequency = 1/70 * hmclk frequency
            pub const DIVIDE_70: u32 = 0b01000101;

            /// 0b01000110: mclk frequency = 1/71 * hmclk frequency
            pub const DIVIDE_71: u32 = 0b01000110;

            /// 0b01000111: mclk frequency = 1/72 * hmclk frequency
            pub const DIVIDE_72: u32 = 0b01000111;

            /// 0b01001000: mclk frequency = 1/73 * hmclk frequency
            pub const DIVIDE_73: u32 = 0b01001000;

            /// 0b01001001: mclk frequency = 1/74 * hmclk frequency
            pub const DIVIDE_74: u32 = 0b01001001;

            /// 0b01001010: mclk frequency = 1/75 * hmclk frequency
            pub const DIVIDE_75: u32 = 0b01001010;

            /// 0b01001011: mclk frequency = 1/76 * hmclk frequency
            pub const DIVIDE_76: u32 = 0b01001011;

            /// 0b01001100: mclk frequency = 1/77 * hmclk frequency
            pub const DIVIDE_77: u32 = 0b01001100;

            /// 0b01001101: mclk frequency = 1/78 * hmclk frequency
            pub const DIVIDE_78: u32 = 0b01001101;

            /// 0b01001110: mclk frequency = 1/79 * hmclk frequency
            pub const DIVIDE_79: u32 = 0b01001110;

            /// 0b01001111: mclk frequency = 1/80 * hmclk frequency
            pub const DIVIDE_80: u32 = 0b01001111;

            /// 0b01010000: mclk frequency = 1/81 * hmclk frequency
            pub const DIVIDE_81: u32 = 0b01010000;

            /// 0b01010001: mclk frequency = 1/82 * hmclk frequency
            pub const DIVIDE_82: u32 = 0b01010001;

            /// 0b01010010: mclk frequency = 1/83 * hmclk frequency
            pub const DIVIDE_83: u32 = 0b01010010;

            /// 0b01010011: mclk frequency = 1/84 * hmclk frequency
            pub const DIVIDE_84: u32 = 0b01010011;

            /// 0b01010100: mclk frequency = 1/85 * hmclk frequency
            pub const DIVIDE_85: u32 = 0b01010100;

            /// 0b01010101: mclk frequency = 1/86 * hmclk frequency
            pub const DIVIDE_86: u32 = 0b01010101;

            /// 0b01010110: mclk frequency = 1/87 * hmclk frequency
            pub const DIVIDE_87: u32 = 0b01010110;

            /// 0b01010111: mclk frequency = 1/88 * hmclk frequency
            pub const DIVIDE_88: u32 = 0b01010111;

            /// 0b01011000: mclk frequency = 1/89 * hmclk frequency
            pub const DIVIDE_89: u32 = 0b01011000;

            /// 0b01011001: mclk frequency = 1/90 * hmclk frequency
            pub const DIVIDE_90: u32 = 0b01011001;

            /// 0b01011010: mclk frequency = 1/91 * hmclk frequency
            pub const DIVIDE_91: u32 = 0b01011010;

            /// 0b01011011: mclk frequency = 1/92 * hmclk frequency
            pub const DIVIDE_92: u32 = 0b01011011;

            /// 0b01011100: mclk frequency = 1/93 * hmclk frequency
            pub const DIVIDE_93: u32 = 0b01011100;

            /// 0b01011101: mclk frequency = 1/94 * hmclk frequency
            pub const DIVIDE_94: u32 = 0b01011101;

            /// 0b01011110: mclk frequency = 1/95 * hmclk frequency
            pub const DIVIDE_95: u32 = 0b01011110;

            /// 0b01011111: mclk frequency = 1/96 * hmclk frequency
            pub const DIVIDE_96: u32 = 0b01011111;

            /// 0b01100000: mclk frequency = 1/97 * hmclk frequency
            pub const DIVIDE_97: u32 = 0b01100000;

            /// 0b01100001: mclk frequency = 1/98 * hmclk frequency
            pub const DIVIDE_98: u32 = 0b01100001;

            /// 0b01100010: mclk frequency = 1/99 * hmclk frequency
            pub const DIVIDE_99: u32 = 0b01100010;

            /// 0b01100011: mclk frequency = 1/100 * hmclk frequency
            pub const DIVIDE_100: u32 = 0b01100011;

            /// 0b01100100: mclk frequency = 1/101 * hmclk frequency
            pub const DIVIDE_101: u32 = 0b01100100;

            /// 0b01100101: mclk frequency = 1/102 * hmclk frequency
            pub const DIVIDE_102: u32 = 0b01100101;

            /// 0b01100110: mclk frequency = 1/103 * hmclk frequency
            pub const DIVIDE_103: u32 = 0b01100110;

            /// 0b01100111: mclk frequency = 1/104 * hmclk frequency
            pub const DIVIDE_104: u32 = 0b01100111;

            /// 0b01101000: mclk frequency = 1/105 * hmclk frequency
            pub const DIVIDE_105: u32 = 0b01101000;

            /// 0b01101001: mclk frequency = 1/106 * hmclk frequency
            pub const DIVIDE_106: u32 = 0b01101001;

            /// 0b01101010: mclk frequency = 1/107 * hmclk frequency
            pub const DIVIDE_107: u32 = 0b01101010;

            /// 0b01101011: mclk frequency = 1/108 * hmclk frequency
            pub const DIVIDE_108: u32 = 0b01101011;

            /// 0b01101100: mclk frequency = 1/109 * hmclk frequency
            pub const DIVIDE_109: u32 = 0b01101100;

            /// 0b01101101: mclk frequency = 1/110 * hmclk frequency
            pub const DIVIDE_110: u32 = 0b01101101;

            /// 0b01101110: mclk frequency = 1/111 * hmclk frequency
            pub const DIVIDE_111: u32 = 0b01101110;

            /// 0b01101111: mclk frequency = 1/112 * hmclk frequency
            pub const DIVIDE_112: u32 = 0b01101111;

            /// 0b01110000: mclk frequency = 1/113 * hmclk frequency
            pub const DIVIDE_113: u32 = 0b01110000;

            /// 0b01110001: mclk frequency = 1/114 * hmclk frequency
            pub const DIVIDE_114: u32 = 0b01110001;

            /// 0b01110010: mclk frequency = 1/115 * hmclk frequency
            pub const DIVIDE_115: u32 = 0b01110010;

            /// 0b01110011: mclk frequency = 1/116 * hmclk frequency
            pub const DIVIDE_116: u32 = 0b01110011;

            /// 0b01110100: mclk frequency = 1/117 * hmclk frequency
            pub const DIVIDE_117: u32 = 0b01110100;

            /// 0b01110101: mclk frequency = 1/118 * hmclk frequency
            pub const DIVIDE_118: u32 = 0b01110101;

            /// 0b01110110: mclk frequency = 1/119 * hmclk frequency
            pub const DIVIDE_119: u32 = 0b01110110;

            /// 0b01110111: mclk frequency = 1/120 * hmclk frequency
            pub const DIVIDE_120: u32 = 0b01110111;

            /// 0b01111000: mclk frequency = 1/121 * hmclk frequency
            pub const DIVIDE_121: u32 = 0b01111000;

            /// 0b01111001: mclk frequency = 1/122 * hmclk frequency
            pub const DIVIDE_122: u32 = 0b01111001;

            /// 0b01111010: mclk frequency = 1/123 * hmclk frequency
            pub const DIVIDE_123: u32 = 0b01111010;

            /// 0b01111011: mclk frequency = 1/124 * hmclk frequency
            pub const DIVIDE_124: u32 = 0b01111011;

            /// 0b01111100: mclk frequency = 1/125 * hmclk frequency
            pub const DIVIDE_125: u32 = 0b01111100;

            /// 0b01111101: mclk frequency = 1/126 * hmclk frequency
            pub const DIVIDE_126: u32 = 0b01111101;

            /// 0b01111110: mclk frequency = 1/127 * hmclk frequency
            pub const DIVIDE_127: u32 = 0b01111110;

            /// 0b01111111: mclk frequency = 1/128 * hmclk frequency
            pub const DIVIDE_128: u32 = 0b01111111;

            /// 0b10000000: mclk frequency = 1/129 * hmclk frequency
            pub const DIVIDE_129: u32 = 0b10000000;

            /// 0b10000001: mclk frequency = 1/130 * hmclk frequency
            pub const DIVIDE_130: u32 = 0b10000001;

            /// 0b10000010: mclk frequency = 1/131 * hmclk frequency
            pub const DIVIDE_131: u32 = 0b10000010;

            /// 0b10000011: mclk frequency = 1/132 * hmclk frequency
            pub const DIVIDE_132: u32 = 0b10000011;

            /// 0b10000100: mclk frequency = 1/133 * hmclk frequency
            pub const DIVIDE_133: u32 = 0b10000100;

            /// 0b10000101: mclk frequency = 1/134 * hmclk frequency
            pub const DIVIDE_134: u32 = 0b10000101;

            /// 0b10000110: mclk frequency = 1/135 * hmclk frequency
            pub const DIVIDE_135: u32 = 0b10000110;

            /// 0b10000111: mclk frequency = 1/136 * hmclk frequency
            pub const DIVIDE_136: u32 = 0b10000111;

            /// 0b10001000: mclk frequency = 1/137 * hmclk frequency
            pub const DIVIDE_137: u32 = 0b10001000;

            /// 0b10001001: mclk frequency = 1/138 * hmclk frequency
            pub const DIVIDE_138: u32 = 0b10001001;

            /// 0b10001010: mclk frequency = 1/139 * hmclk frequency
            pub const DIVIDE_139: u32 = 0b10001010;

            /// 0b10001011: mclk frequency = 1/140 * hmclk frequency
            pub const DIVIDE_140: u32 = 0b10001011;

            /// 0b10001100: mclk frequency = 1/141 * hmclk frequency
            pub const DIVIDE_141: u32 = 0b10001100;

            /// 0b10001101: mclk frequency = 1/142 * hmclk frequency
            pub const DIVIDE_142: u32 = 0b10001101;

            /// 0b10001110: mclk frequency = 1/143 * hmclk frequency
            pub const DIVIDE_143: u32 = 0b10001110;

            /// 0b10001111: mclk frequency = 1/144 * hmclk frequency
            pub const DIVIDE_144: u32 = 0b10001111;

            /// 0b10010000: mclk frequency = 1/145 * hmclk frequency
            pub const DIVIDE_145: u32 = 0b10010000;

            /// 0b10010001: mclk frequency = 1/146 * hmclk frequency
            pub const DIVIDE_146: u32 = 0b10010001;

            /// 0b10010010: mclk frequency = 1/147 * hmclk frequency
            pub const DIVIDE_147: u32 = 0b10010010;

            /// 0b10010011: mclk frequency = 1/148 * hmclk frequency
            pub const DIVIDE_148: u32 = 0b10010011;

            /// 0b10010100: mclk frequency = 1/149 * hmclk frequency
            pub const DIVIDE_149: u32 = 0b10010100;

            /// 0b10010101: mclk frequency = 1/150 * hmclk frequency
            pub const DIVIDE_150: u32 = 0b10010101;

            /// 0b10010110: mclk frequency = 1/151 * hmclk frequency
            pub const DIVIDE_151: u32 = 0b10010110;

            /// 0b10010111: mclk frequency = 1/152 * hmclk frequency
            pub const DIVIDE_152: u32 = 0b10010111;

            /// 0b10011000: mclk frequency = 1/153 * hmclk frequency
            pub const DIVIDE_153: u32 = 0b10011000;

            /// 0b10011001: mclk frequency = 1/154 * hmclk frequency
            pub const DIVIDE_154: u32 = 0b10011001;

            /// 0b10011010: mclk frequency = 1/155 * hmclk frequency
            pub const DIVIDE_155: u32 = 0b10011010;

            /// 0b10011011: mclk frequency = 1/156 * hmclk frequency
            pub const DIVIDE_156: u32 = 0b10011011;

            /// 0b10011100: mclk frequency = 1/157 * hmclk frequency
            pub const DIVIDE_157: u32 = 0b10011100;

            /// 0b10011101: mclk frequency = 1/158 * hmclk frequency
            pub const DIVIDE_158: u32 = 0b10011101;

            /// 0b10011110: mclk frequency = 1/159 * hmclk frequency
            pub const DIVIDE_159: u32 = 0b10011110;

            /// 0b10011111: mclk frequency = 1/160 * hmclk frequency
            pub const DIVIDE_160: u32 = 0b10011111;

            /// 0b10100000: mclk frequency = 1/161 * hmclk frequency
            pub const DIVIDE_161: u32 = 0b10100000;

            /// 0b10100001: mclk frequency = 1/162 * hmclk frequency
            pub const DIVIDE_162: u32 = 0b10100001;

            /// 0b10100010: mclk frequency = 1/163 * hmclk frequency
            pub const DIVIDE_163: u32 = 0b10100010;

            /// 0b10100011: mclk frequency = 1/164 * hmclk frequency
            pub const DIVIDE_164: u32 = 0b10100011;

            /// 0b10100100: mclk frequency = 1/165 * hmclk frequency
            pub const DIVIDE_165: u32 = 0b10100100;

            /// 0b10100101: mclk frequency = 1/166 * hmclk frequency
            pub const DIVIDE_166: u32 = 0b10100101;

            /// 0b10100110: mclk frequency = 1/167 * hmclk frequency
            pub const DIVIDE_167: u32 = 0b10100110;

            /// 0b10100111: mclk frequency = 1/168 * hmclk frequency
            pub const DIVIDE_168: u32 = 0b10100111;

            /// 0b10101000: mclk frequency = 1/169 * hmclk frequency
            pub const DIVIDE_169: u32 = 0b10101000;

            /// 0b10101001: mclk frequency = 1/170 * hmclk frequency
            pub const DIVIDE_170: u32 = 0b10101001;

            /// 0b10101010: mclk frequency = 1/171 * hmclk frequency
            pub const DIVIDE_171: u32 = 0b10101010;

            /// 0b10101011: mclk frequency = 1/172 * hmclk frequency
            pub const DIVIDE_172: u32 = 0b10101011;

            /// 0b10101100: mclk frequency = 1/173 * hmclk frequency
            pub const DIVIDE_173: u32 = 0b10101100;

            /// 0b10101101: mclk frequency = 1/174 * hmclk frequency
            pub const DIVIDE_174: u32 = 0b10101101;

            /// 0b10101110: mclk frequency = 1/175 * hmclk frequency
            pub const DIVIDE_175: u32 = 0b10101110;

            /// 0b10101111: mclk frequency = 1/176 * hmclk frequency
            pub const DIVIDE_176: u32 = 0b10101111;

            /// 0b10110000: mclk frequency = 1/177 * hmclk frequency
            pub const DIVIDE_177: u32 = 0b10110000;

            /// 0b10110001: mclk frequency = 1/178 * hmclk frequency
            pub const DIVIDE_178: u32 = 0b10110001;

            /// 0b10110010: mclk frequency = 1/179 * hmclk frequency
            pub const DIVIDE_179: u32 = 0b10110010;

            /// 0b10110011: mclk frequency = 1/180 * hmclk frequency
            pub const DIVIDE_180: u32 = 0b10110011;

            /// 0b10110100: mclk frequency = 1/181 * hmclk frequency
            pub const DIVIDE_181: u32 = 0b10110100;

            /// 0b10110101: mclk frequency = 1/182 * hmclk frequency
            pub const DIVIDE_182: u32 = 0b10110101;

            /// 0b10110110: mclk frequency = 1/183 * hmclk frequency
            pub const DIVIDE_183: u32 = 0b10110110;

            /// 0b10110111: mclk frequency = 1/184 * hmclk frequency
            pub const DIVIDE_184: u32 = 0b10110111;

            /// 0b10111000: mclk frequency = 1/185 * hmclk frequency
            pub const DIVIDE_185: u32 = 0b10111000;

            /// 0b10111001: mclk frequency = 1/186 * hmclk frequency
            pub const DIVIDE_186: u32 = 0b10111001;

            /// 0b10111010: mclk frequency = 1/187 * hmclk frequency
            pub const DIVIDE_187: u32 = 0b10111010;

            /// 0b10111011: mclk frequency = 1/188 * hmclk frequency
            pub const DIVIDE_188: u32 = 0b10111011;

            /// 0b10111100: mclk frequency = 1/189 * hmclk frequency
            pub const DIVIDE_189: u32 = 0b10111100;

            /// 0b10111101: mclk frequency = 1/190 * hmclk frequency
            pub const DIVIDE_190: u32 = 0b10111101;

            /// 0b10111110: mclk frequency = 1/191 * hmclk frequency
            pub const DIVIDE_191: u32 = 0b10111110;

            /// 0b10111111: mclk frequency = 1/192 * hmclk frequency
            pub const DIVIDE_192: u32 = 0b10111111;

            /// 0b11000000: mclk frequency = 1/193 * hmclk frequency
            pub const DIVIDE_193: u32 = 0b11000000;

            /// 0b11000001: mclk frequency = 1/194 * hmclk frequency
            pub const DIVIDE_194: u32 = 0b11000001;

            /// 0b11000010: mclk frequency = 1/195 * hmclk frequency
            pub const DIVIDE_195: u32 = 0b11000010;

            /// 0b11000011: mclk frequency = 1/196 * hmclk frequency
            pub const DIVIDE_196: u32 = 0b11000011;

            /// 0b11000100: mclk frequency = 1/197 * hmclk frequency
            pub const DIVIDE_197: u32 = 0b11000100;

            /// 0b11000101: mclk frequency = 1/198 * hmclk frequency
            pub const DIVIDE_198: u32 = 0b11000101;

            /// 0b11000110: mclk frequency = 1/199 * hmclk frequency
            pub const DIVIDE_199: u32 = 0b11000110;

            /// 0b11000111: mclk frequency = 1/200 * hmclk frequency
            pub const DIVIDE_200: u32 = 0b11000111;

            /// 0b11001000: mclk frequency = 1/201 * hmclk frequency
            pub const DIVIDE_201: u32 = 0b11001000;

            /// 0b11001001: mclk frequency = 1/202 * hmclk frequency
            pub const DIVIDE_202: u32 = 0b11001001;

            /// 0b11001010: mclk frequency = 1/203 * hmclk frequency
            pub const DIVIDE_203: u32 = 0b11001010;

            /// 0b11001011: mclk frequency = 1/204 * hmclk frequency
            pub const DIVIDE_204: u32 = 0b11001011;

            /// 0b11001100: mclk frequency = 1/205 * hmclk frequency
            pub const DIVIDE_205: u32 = 0b11001100;

            /// 0b11001101: mclk frequency = 1/206 * hmclk frequency
            pub const DIVIDE_206: u32 = 0b11001101;

            /// 0b11001110: mclk frequency = 1/207 * hmclk frequency
            pub const DIVIDE_207: u32 = 0b11001110;

            /// 0b11001111: mclk frequency = 1/208 * hmclk frequency
            pub const DIVIDE_208: u32 = 0b11001111;

            /// 0b11010000: mclk frequency = 1/209 * hmclk frequency
            pub const DIVIDE_209: u32 = 0b11010000;

            /// 0b11010001: mclk frequency = 1/210 * hmclk frequency
            pub const DIVIDE_210: u32 = 0b11010001;

            /// 0b11010010: mclk frequency = 1/211 * hmclk frequency
            pub const DIVIDE_211: u32 = 0b11010010;

            /// 0b11010011: mclk frequency = 1/212 * hmclk frequency
            pub const DIVIDE_212: u32 = 0b11010011;

            /// 0b11010100: mclk frequency = 1/213 * hmclk frequency
            pub const DIVIDE_213: u32 = 0b11010100;

            /// 0b11010101: mclk frequency = 1/214 * hmclk frequency
            pub const DIVIDE_214: u32 = 0b11010101;

            /// 0b11010110: mclk frequency = 1/215 * hmclk frequency
            pub const DIVIDE_215: u32 = 0b11010110;

            /// 0b11010111: mclk frequency = 1/216 * hmclk frequency
            pub const DIVIDE_216: u32 = 0b11010111;

            /// 0b11011000: mclk frequency = 1/217 * hmclk frequency
            pub const DIVIDE_217: u32 = 0b11011000;

            /// 0b11011001: mclk frequency = 1/218 * hmclk frequency
            pub const DIVIDE_218: u32 = 0b11011001;

            /// 0b11011010: mclk frequency = 1/219 * hmclk frequency
            pub const DIVIDE_219: u32 = 0b11011010;

            /// 0b11011011: mclk frequency = 1/220 * hmclk frequency
            pub const DIVIDE_220: u32 = 0b11011011;

            /// 0b11011100: mclk frequency = 1/221 * hmclk frequency
            pub const DIVIDE_221: u32 = 0b11011100;

            /// 0b11011101: mclk frequency = 1/222 * hmclk frequency
            pub const DIVIDE_222: u32 = 0b11011101;

            /// 0b11011110: mclk frequency = 1/223 * hmclk frequency
            pub const DIVIDE_223: u32 = 0b11011110;

            /// 0b11011111: mclk frequency = 1/224 * hmclk frequency
            pub const DIVIDE_224: u32 = 0b11011111;

            /// 0b11100000: mclk frequency = 1/225 * hmclk frequency
            pub const DIVIDE_225: u32 = 0b11100000;

            /// 0b11100001: mclk frequency = 1/226 * hmclk frequency
            pub const DIVIDE_226: u32 = 0b11100001;

            /// 0b11100010: mclk frequency = 1/227 * hmclk frequency
            pub const DIVIDE_227: u32 = 0b11100010;

            /// 0b11100011: mclk frequency = 1/228 * hmclk frequency
            pub const DIVIDE_228: u32 = 0b11100011;

            /// 0b11100100: mclk frequency = 1/229 * hmclk frequency
            pub const DIVIDE_229: u32 = 0b11100100;

            /// 0b11100101: mclk frequency = 1/230 * hmclk frequency
            pub const DIVIDE_230: u32 = 0b11100101;

            /// 0b11100110: mclk frequency = 1/231 * hmclk frequency
            pub const DIVIDE_231: u32 = 0b11100110;

            /// 0b11100111: mclk frequency = 1/232 * hmclk frequency
            pub const DIVIDE_232: u32 = 0b11100111;

            /// 0b11101000: mclk frequency = 1/233 * hmclk frequency
            pub const DIVIDE_233: u32 = 0b11101000;

            /// 0b11101001: mclk frequency = 1/234 * hmclk frequency
            pub const DIVIDE_234: u32 = 0b11101001;

            /// 0b11101010: mclk frequency = 1/235 * hmclk frequency
            pub const DIVIDE_235: u32 = 0b11101010;

            /// 0b11101011: mclk frequency = 1/236 * hmclk frequency
            pub const DIVIDE_236: u32 = 0b11101011;

            /// 0b11101100: mclk frequency = 1/237 * hmclk frequency
            pub const DIVIDE_237: u32 = 0b11101100;

            /// 0b11101101: mclk frequency = 1/238 * hmclk frequency
            pub const DIVIDE_238: u32 = 0b11101101;

            /// 0b11101110: mclk frequency = 1/239 * hmclk frequency
            pub const DIVIDE_239: u32 = 0b11101110;

            /// 0b11101111: mclk frequency = 1/240 * hmclk frequency
            pub const DIVIDE_240: u32 = 0b11101111;

            /// 0b11110000: mclk frequency = 1/241 * hmclk frequency
            pub const DIVIDE_241: u32 = 0b11110000;

            /// 0b11110001: mclk frequency = 1/242 * hmclk frequency
            pub const DIVIDE_242: u32 = 0b11110001;

            /// 0b11110010: mclk frequency = 1/243 * hmclk frequency
            pub const DIVIDE_243: u32 = 0b11110010;

            /// 0b11110011: mclk frequency = 1/244 * hmclk frequency
            pub const DIVIDE_244: u32 = 0b11110011;

            /// 0b11110100: mclk frequency = 1/245 * hmclk frequency
            pub const DIVIDE_245: u32 = 0b11110100;

            /// 0b11110101: mclk frequency = 1/246 * hmclk frequency
            pub const DIVIDE_246: u32 = 0b11110101;

            /// 0b11110110: mclk frequency = 1/247 * hmclk frequency
            pub const DIVIDE_247: u32 = 0b11110110;

            /// 0b11110111: mclk frequency = 1/248 * hmclk frequency
            pub const DIVIDE_248: u32 = 0b11110111;

            /// 0b11111000: mclk frequency = 1/249 * hmclk frequency
            pub const DIVIDE_249: u32 = 0b11111000;

            /// 0b11111001: mclk frequency = 1/250 * hmclk frequency
            pub const DIVIDE_250: u32 = 0b11111001;

            /// 0b11111010: mclk frequency = 1/251 * hmclk frequency
            pub const DIVIDE_251: u32 = 0b11111010;

            /// 0b11111011: mclk frequency = 1/252 * hmclk frequency
            pub const DIVIDE_252: u32 = 0b11111011;

            /// 0b11111100: mclk frequency = 1/253 * hmclk frequency
            pub const DIVIDE_253: u32 = 0b11111100;

            /// 0b11111101: mclk frequency = 1/254 * hmclk frequency
            pub const DIVIDE_254: u32 = 0b11111101;

            /// 0b11111110: mclk frequency = 1/255 * hmclk frequency
            pub const DIVIDE_255: u32 = 0b11111110;

            /// 0b11111111: mclk frequency = 1/256 * hmclk frequency
            pub const DIVIDE_256: u32 = 0b11111111;
        }
    }

    /// MQS software reset
    pub mod MQS_SW_RST {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Exit software reset for MQS
            pub const MQS_SW_RST_0: u32 = 0b0;

            /// 0b1: Enable software reset for MQS
            pub const MQS_SW_RST_1: u32 = 0b1;
        }
    }

    /// MQS enable.
    pub mod MQS_EN {
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

            /// 0b0: Disable MQS
            pub const MQS_EN_0: u32 = 0b0;

            /// 0b1: Enable MQS
            pub const MQS_EN_1: u32 = 0b1;
        }
    }

    /// Medium Quality Sound (MQS) Oversample
    pub mod MQS_OVERSAMPLE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 32
            pub const MQS_OVERSAMPLE_0: u32 = 0b0;

            /// 0b1: 64
            pub const MQS_OVERSAMPLE_1: u32 = 0b1;
        }
    }

    /// QTIMER1 timer counter freeze
    pub mod QTIMER1_TMR_CNTS_FREEZE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Timer counter works normally
            pub const QTIMER1_TMR_CNTS_FREEZE_0: u32 = 0b0;

            /// 0b1: Reset counter and ouput flags
            pub const QTIMER1_TMR_CNTS_FREEZE_1: u32 = 0b1;
        }
    }
}

/// GPR3 General Purpose Register
pub mod GPR3 {

    /// Select 128-bit DCP key from 256-bit key from SNVS/OCOTP
    pub mod DCP_KEY_SEL {
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

            /// 0b0: Select \[127:0\] from SNVS/OCOTP key as DCP key
            pub const DCP_KEY_SEL_0: u32 = 0b0;

            /// 0b1: Select \[255:128\] from SNVS/OCOTP key as DCP key
            pub const DCP_KEY_SEL_1: u32 = 0b1;
        }
    }
}

/// GPR4 General Purpose Register
pub mod GPR4 {

    /// EDMA stop request.
    pub mod EDMA_STOP_REQ {
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

            /// 0b0: stop request off
            pub const EDMA_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const EDMA_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// TRNG stop request.
    pub mod TRNG_STOP_REQ {
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

            /// 0b0: stop request off
            pub const TRNG_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const TRNG_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// SAI1 stop request.
    pub mod SAI1_STOP_REQ {
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

            /// 0b0: stop request off
            pub const SAI1_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const SAI1_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// SAI2 stop request.
    pub mod SAI2_STOP_REQ {
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

            /// 0b0: stop request off
            pub const SAI2_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const SAI2_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// SAI3 stop request.
    pub mod SAI3_STOP_REQ {
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

            /// 0b0: stop request off
            pub const SAI3_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const SAI3_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// PIT stop request.
    pub mod PIT_STOP_REQ {
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

            /// 0b0: stop request off
            pub const PIT_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const PIT_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// FlexSPI stop request.
    pub mod FLEXSPI_STOP_REQ {
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

            /// 0b0: stop request off
            pub const FLEXSPI_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const FLEXSPI_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// FlexIO1 stop request.
    pub mod FLEXIO1_STOP_REQ {
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

            /// 0b0: stop request off
            pub const FLEXIO1_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const FLEXIO1_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// EDMA stop acknowledge. This is a status (read-only) bit
    pub mod EDMA_STOP_ACK {
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

            /// 0b0: EDMA stop acknowledge is not asserted
            pub const EDMA_STOP_ACK_0: u32 = 0b0;

            /// 0b1: EDMA stop acknowledge is asserted (EDMA is in STOP mode).
            pub const EDMA_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// TRNG stop acknowledge
    pub mod TRNG_STOP_ACK {
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

            /// 0b0: TRNG stop acknowledge is not asserted
            pub const TRNG_STOP_ACK_0: u32 = 0b0;

            /// 0b1: TRNG stop acknowledge is asserted
            pub const TRNG_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// SAI1 stop acknowledge
    pub mod SAI1_STOP_ACK {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SAI1 stop acknowledge is not asserted
            pub const SAI1_STOP_ACK_0: u32 = 0b0;

            /// 0b1: SAI1 stop acknowledge is asserted
            pub const SAI1_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// SAI2 stop acknowledge
    pub mod SAI2_STOP_ACK {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SAI2 stop acknowledge is not asserted
            pub const SAI2_STOP_ACK_0: u32 = 0b0;

            /// 0b1: SAI2 stop acknowledge is asserted
            pub const SAI2_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// SAI3 stop acknowledge
    pub mod SAI3_STOP_ACK {
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

            /// 0b0: SAI3 stop acknowledge is not asserted
            pub const SAI3_STOP_ACK_0: u32 = 0b0;

            /// 0b1: SAI3 stop acknowledge is asserted
            pub const SAI3_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// PIT stop acknowledge
    pub mod PIT_STOP_ACK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PIT stop acknowledge is not asserted
            pub const PIT_STOP_ACK_0: u32 = 0b0;

            /// 0b1: PIT stop acknowledge is asserted
            pub const PIT_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// FLEXSPI stop acknowledge
    pub mod FLEXSPI_STOP_ACK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FLEXSPI stop acknowledge is not asserted
            pub const FLEXSPI_STOP_ACK_0: u32 = 0b0;

            /// 0b1: FLEXSPI stop acknowledge is asserted
            pub const FLEXSPI_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// FLEXIO1 stop acknowledge
    pub mod FLEXIO1_STOP_ACK {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FLEXIO1 stop acknowledge is not asserted
            pub const FLEXIO1_STOP_ACK_0: u32 = 0b0;

            /// 0b1: FLEXIO1 stop acknowledge is asserted
            pub const FLEXIO1_STOP_ACK_1: u32 = 0b1;
        }
    }
}

/// GPR5 General Purpose Register
pub mod GPR5 {

    /// WDOG1 Timeout Mask
    pub mod WDOG1_MASK {
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

            /// 0b0: WDOG1 Timeout behaves normally
            pub const WDOG1_MASK_0: u32 = 0b0;

            /// 0b1: WDOG1 Timeout is masked
            pub const WDOG1_MASK_1: u32 = 0b1;
        }
    }

    /// WDOG2 Timeout Mask
    pub mod WDOG2_MASK {
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

            /// 0b0: WDOG2 Timeout behaves normally
            pub const WDOG2_MASK_0: u32 = 0b0;

            /// 0b1: WDOG2 Timeout is masked
            pub const WDOG2_MASK_1: u32 = 0b1;
        }
    }

    /// GPT2 input capture channel 1 source select
    pub mod GPT2_CAPIN1_SEL {
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

            /// 0b0: source from GPT2_CAPTURE1
            pub const GPT2_CAPIN1_SEL_0: u32 = 0b0;

            /// 0b1: source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)
            pub const GPT2_CAPIN1_SEL_1: u32 = 0b1;
        }
    }

    /// GPT1 1 MHz clock source select
    pub mod VREF_1M_CLK_GPT1 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: GPT1 ipg_clk_highfreq driven by IPG_PERCLK
            pub const VREF_1M_CLK_GPT1_0: u32 = 0b0;

            /// 0b1: GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock
            pub const VREF_1M_CLK_GPT1_1: u32 = 0b1;
        }
    }

    /// GPT2 1 MHz clock source select
    pub mod VREF_1M_CLK_GPT2 {
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

            /// 0b0: GPT2 ipg_clk_highfreq driven by IPG_PERCLK
            pub const VREF_1M_CLK_GPT2_0: u32 = 0b0;

            /// 0b1: GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock
            pub const VREF_1M_CLK_GPT2_1: u32 = 0b1;
        }
    }
}

/// GPR6 General Purpose Register
pub mod GPR6 {

    /// QTIMER1 TMR0 input select
    pub mod QTIMER1_TRM0_INPUT_SEL {
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

            /// 0b0: input from IOMUX
            pub const QTIMER1_TRM0_INPUT_SEL_0: u32 = 0b0;

            /// 0b1: input from XBAR
            pub const QTIMER1_TRM0_INPUT_SEL_1: u32 = 0b1;
        }
    }

    /// QTIMER1 TMR1 input select
    pub mod QTIMER1_TRM1_INPUT_SEL {
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

            /// 0b0: input from IOMUX
            pub const QTIMER1_TRM1_INPUT_SEL_0: u32 = 0b0;

            /// 0b1: input from XBAR
            pub const QTIMER1_TRM1_INPUT_SEL_1: u32 = 0b1;
        }
    }

    /// QTIMER1 TMR2 input select
    pub mod QTIMER1_TRM2_INPUT_SEL {
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

            /// 0b0: input from IOMUX
            pub const QTIMER1_TRM2_INPUT_SEL_0: u32 = 0b0;

            /// 0b1: input from XBAR
            pub const QTIMER1_TRM2_INPUT_SEL_1: u32 = 0b1;
        }
    }

    /// QTIMER1 TMR3 input select
    pub mod QTIMER1_TRM3_INPUT_SEL {
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

            /// 0b0: input from IOMUX
            pub const QTIMER1_TRM3_INPUT_SEL_0: u32 = 0b0;

            /// 0b1: input from XBAR
            pub const QTIMER1_TRM3_INPUT_SEL_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT4 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_4 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_4_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_4_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT5 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_5_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_5_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT6 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_6 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_6_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_6_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT7 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_7 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_7_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_7_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT8 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_8 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_8_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_8_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT9 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_9 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_9_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_9_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT10 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_10 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_10_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_10_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT11 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_11 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_11_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_11_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT12 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_12_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_12_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT13 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_13 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_13_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_13_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT14 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_14 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_14_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_14_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT15 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_15 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_15_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_15_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT16 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_16 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_16_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_16_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT17 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_17 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_17_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_17_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT18 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_18 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_18_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_18_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT19 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_19 {
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

            /// 0b0: XBAR_INOUT as input
            pub const IOMUXC_XBAR_DIR_SEL_19_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_19_1: u32 = 0b1;
        }
    }
}

/// GPR7 General Purpose Register
pub mod GPR7 {

    /// LPI2C1 stop request
    pub mod LPI2C1_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPI2C1_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPI2C1_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPI2C2 stop request
    pub mod LPI2C2_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPI2C2_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPI2C2_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPSPI1 stop request
    pub mod LPSPI1_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPSPI1_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPSPI1_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPSPI2 stop request
    pub mod LPSPI2_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPSPI2_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPSPI2_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPUART1 stop request
    pub mod LPUART1_STOP_REQ {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop request off
            pub const LPUART1_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPUART1_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPUART1 stop request
    pub mod LPUART2_STOP_REQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop request off
            pub const LPUART2_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPUART2_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPUART3 stop request
    pub mod LPUART3_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPUART3_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPUART3_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPUART4 stop request
    pub mod LPUART4_STOP_REQ {
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

            /// 0b0: stop request off
            pub const LPUART4_STOP_REQ_0: u32 = 0b0;

            /// 0b1: stop request on
            pub const LPUART4_STOP_REQ_1: u32 = 0b1;
        }
    }

    /// LPI2C1 stop acknowledge
    pub mod LPI2C1_STOP_ACK {
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

            /// 0b0: stop acknowledge is not asserted
            pub const LPI2C1_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted (the module is in Stop mode)
            pub const LPI2C1_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPI2C2 stop acknowledge
    pub mod LPI2C2_STOP_ACK {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPI2C2_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPI2C2_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPSPI1 stop acknowledge
    pub mod LPSPI1_STOP_ACK {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPSPI1_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPSPI1_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPSPI2 stop acknowledge
    pub mod LPSPI2_STOP_ACK {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPSPI2_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPSPI2_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPUART1 stop acknowledge
    pub mod LPUART1_STOP_ACK {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPUART1_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPUART1_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPUART1 stop acknowledge
    pub mod LPUART2_STOP_ACK {
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

            /// 0b0: stop acknowledge is not asserted
            pub const LPUART2_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPUART2_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPUART3 stop acknowledge
    pub mod LPUART3_STOP_ACK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPUART3_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPUART3_STOP_ACK_1: u32 = 0b1;
        }
    }

    /// LPUART4 stop acknowledge
    pub mod LPUART4_STOP_ACK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: stop acknowledge is not asserted
            pub const LPUART4_STOP_ACK_0: u32 = 0b0;

            /// 0b1: stop acknowledge is asserted
            pub const LPUART4_STOP_ACK_1: u32 = 0b1;
        }
    }
}

/// GPR8 General Purpose Register
pub mod GPR8 {

    /// LPI2C1 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPI2C1_IPG_STOP_MODE {
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

            /// 0b0: the module is functional in Stop mode
            pub const LPI2C1_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPI2C1_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPI2C1 ipg_doze mode
    pub mod LPI2C1_IPG_DOZE {
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

            /// 0b0: not in doze mode
            pub const LPI2C1_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPI2C1_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPI2C2 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPI2C2_IPG_STOP_MODE {
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

            /// 0b0: the module is functional in Stop mode
            pub const LPI2C2_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPI2C2_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPI2C2 ipg_doze mode
    pub mod LPI2C2_IPG_DOZE {
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

            /// 0b0: not in doze mode
            pub const LPI2C2_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPI2C2_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPSPI1 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPSPI1_IPG_STOP_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the module is functional in Stop mode
            pub const LPSPI1_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPSPI1_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPSPI1 ipg_doze mode
    pub mod LPSPI1_IPG_DOZE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: not in doze mode
            pub const LPSPI1_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPSPI1_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPSPI2 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPSPI2_IPG_STOP_MODE {
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

            /// 0b0: the module is functional in Stop mode
            pub const LPSPI2_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPSPI2_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPSPI2 ipg_doze mode
    pub mod LPSPI2_IPG_DOZE {
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

            /// 0b0: not in doze mode
            pub const LPSPI2_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPSPI2_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPUART1 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPUART1_IPG_STOP_MODE {
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

            /// 0b0: the module is functional in Stop mode
            pub const LPUART1_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPUART1_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPUART1 ipg_doze mode
    pub mod LPUART1_IPG_DOZE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: not in doze mode
            pub const LPUART1_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPUART1_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPUART2 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPUART2_IPG_STOP_MODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the module is functional in Stop mode
            pub const LPUART2_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPUART2_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPUART2 ipg_doze mode
    pub mod LPUART2_IPG_DOZE {
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

            /// 0b0: not in doze mode
            pub const LPUART2_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPUART2_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPUART3 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPUART3_IPG_STOP_MODE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the module is functional in Stop mode
            pub const LPUART3_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPUART3_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPUART3 ipg_doze mode
    pub mod LPUART3_IPG_DOZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: not in doze mode
            pub const LPUART3_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPUART3_IPG_DOZE_1: u32 = 0b1;
        }
    }

    /// LPUART4 stop mode selection, cannot change when ipg_stop is asserted.
    pub mod LPUART4_IPG_STOP_MODE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the module is functional in Stop mode
            pub const LPUART4_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted
            pub const LPUART4_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// LPUART4 ipg_doze mode
    pub mod LPUART4_IPG_DOZE {
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

            /// 0b0: not in doze mode
            pub const LPUART4_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: in doze mode
            pub const LPUART4_IPG_DOZE_1: u32 = 0b1;
        }
    }
}

/// GPR9 General Purpose Register
pub mod GPR9 {}

/// GPR10 General Purpose Register
pub mod GPR10 {

    /// ARM non-secure (non-invasive) debug enable
    pub mod NIDEN {
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

            /// 0b0: Debug turned off.
            pub const NIDEN_0: u32 = 0b0;

            /// 0b1: Debug enabled (default).
            pub const NIDEN_1: u32 = 0b1;
        }
    }

    /// ARM invasive debug enable
    pub mod DBG_EN {
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

            /// 0b0: Debug turned off.
            pub const DBG_EN_0: u32 = 0b0;

            /// 0b1: Debug enabled (default).
            pub const DBG_EN_1: u32 = 0b1;
        }
    }

    /// Security error response enable for all security gaskets (on both AHB and AXI buses)
    pub mod SEC_ERR_RESP {
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

            /// 0b0: OKEY response
            pub const SEC_ERR_RESP_0: u32 = 0b0;

            /// 0b1: SLVError (default)
            pub const SEC_ERR_RESP_1: u32 = 0b1;
        }
    }

    /// DCP Key selection bit.
    pub mod DCPKEY_OCOTP_OR_KEYMUX {
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

            /// 0b0: Select key from Key MUX (SNVS/OTPMK).
            pub const DCPKEY_OCOTP_OR_KEYMUX_0: u32 = 0b0;

            /// 0b1: Select key from OCOTP (SW_GP2).
            pub const DCPKEY_OCOTP_OR_KEYMUX_1: u32 = 0b1;
        }
    }

    /// OCRAM TrustZone (TZ) enable.
    pub mod OCRAM_TZ_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor).
            pub const OCRAM_TZ_EN_0: u32 = 0b0;

            /// 0b1: The TrustZone feature is enabled. Access to address in the range specified by \[ENDADDR:STARTADDR\] follows the execution mode access policy described in CSU chapter.
            pub const OCRAM_TZ_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM TrustZone (TZ) start address
    pub mod OCRAM_TZ_ADDR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (6 bits: 0x3f << 9)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock NIDEN field for changes
    pub mod LOCK_NIDEN {
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

            /// 0b0: Field is not locked
            pub const LOCK_NIDEN_0: u32 = 0b0;

            /// 0b1: Field is locked (read access only)
            pub const LOCK_NIDEN_1: u32 = 0b1;
        }
    }

    /// Lock DBG_EN field for changes
    pub mod LOCK_DBG_EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Field is not locked
            pub const LOCK_DBG_EN_0: u32 = 0b0;

            /// 0b1: Field is locked (read access only)
            pub const LOCK_DBG_EN_1: u32 = 0b1;
        }
    }

    /// Lock SEC_ERR_RESP field for changes
    pub mod LOCK_SEC_ERR_RESP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Field is not locked
            pub const LOCK_SEC_ERR_RESP_0: u32 = 0b0;

            /// 0b1: Field is locked (read access only)
            pub const LOCK_SEC_ERR_RESP_1: u32 = 0b1;
        }
    }

    /// Lock DCP Key OCOTP/Key MUX selection bit
    pub mod LOCK_DCPKEY_OCOTP_OR_KEYMUX {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Field is not locked
            pub const LOCK_DCPKEY_OCOTP_OR_KEYMUX_0: u32 = 0b0;

            /// 0b1: Field is locked (read access only)
            pub const LOCK_DCPKEY_OCOTP_OR_KEYMUX_1: u32 = 0b1;
        }
    }

    /// Lock OCRAM_TZ_EN field for changes
    pub mod LOCK_OCRAM_TZ_EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Field is not locked
            pub const LOCK_OCRAM_TZ_EN_0: u32 = 0b0;

            /// 0b1: Field is locked (read access only)
            pub const LOCK_OCRAM_TZ_EN_1: u32 = 0b1;
        }
    }

    /// Lock OCRAM_TZ_ADDR field for changes
    pub mod LOCK_OCRAM_TZ_ADDR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (7 bits: 0x7f << 25)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000: Field is not locked
            pub const LOCK_OCRAM_TZ_ADDR_0: u32 = 0b0000000;

            /// 0b0000001: Field is locked (read access only)
            pub const LOCK_OCRAM_TZ_ADDR_1: u32 = 0b0000001;
        }
    }
}

/// GPR11 General Purpose Register
pub mod GPR11 {

    /// Access control of memory region-0
    pub mod M7_APC_AC_R0_CTRL {
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

            /// 0b00: No access protection
            pub const M7_APC_AC_R0_CTRL_0: u32 = 0b00;

            /// 0b01: M7 debug protection enabled
            pub const M7_APC_AC_R0_CTRL_1: u32 = 0b01;
        }
    }

    /// Access control of memory region-1
    pub mod M7_APC_AC_R1_CTRL {
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

            /// 0b00: No access protection
            pub const M7_APC_AC_R1_CTRL_0: u32 = 0b00;

            /// 0b01: M7 debug protection enabled
            pub const M7_APC_AC_R1_CTRL_1: u32 = 0b01;
        }
    }

    /// Access control of memory region-2
    pub mod M7_APC_AC_R2_CTRL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No access protection
            pub const M7_APC_AC_R2_CTRL_0: u32 = 0b00;

            /// 0b01: M7 debug protection enabled
            pub const M7_APC_AC_R2_CTRL_1: u32 = 0b01;
        }
    }

    /// Access control of memory region-3
    pub mod M7_APC_AC_R3_CTRL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No access protection
            pub const M7_APC_AC_R3_CTRL_0: u32 = 0b00;

            /// 0b01: M7 debug protection enabled
            pub const M7_APC_AC_R3_CTRL_1: u32 = 0b01;
        }
    }

    /// BEE data decryption of memory region-n (n = 3 to 0)
    pub mod BEE_DE_RX_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock M7_APC_AC_R0_CTRL field for changes
    pub mod LOCK_M7_APC_AC_R0_CTRL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock M7_APC_AC_R1_CTRL field for changes
    pub mod LOCK_M7_APC_AC_R1_CTRL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock M7_APC_AC_R2_CTRL field for changes
    pub mod LOCK_M7_APC_AC_R2_CTRL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock M7_APC_AC_R3_CTRL field for changes
    pub mod LOCK_M7_APC_AC_R3_CTRL {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lock BEE_DE_RX_EN\[n\] (n = 3 to 0) field for changes
    pub mod LOCK_BEE_DE_RX_EN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR12 General Purpose Register
pub mod GPR12 {

    /// FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted.
    pub mod FLEXIO1_IPG_STOP_MODE {
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

            /// 0b0: FlexIO1 is functional in Stop mode.
            pub const FLEXIO1_IPG_STOP_MODE_0: u32 = 0b0;

            /// 0b1: When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode.
            pub const FLEXIO1_IPG_STOP_MODE_1: u32 = 0b1;
        }
    }

    /// FLEXIO1 ipg_doze mode
    pub mod FLEXIO1_IPG_DOZE {
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

            /// 0b0: FLEXIO1 is not in doze mode
            pub const FLEXIO1_IPG_DOZE_0: u32 = 0b0;

            /// 0b1: FLEXIO1 is in doze mode
            pub const FLEXIO1_IPG_DOZE_1: u32 = 0b1;
        }
    }
}

/// GPR13 General Purpose Register
pub mod GPR13 {

    /// uSDHC block cacheable attribute value of AXI read transactions
    pub mod ARCACHE_USDHC {
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

            /// 0b0: Cacheable attribute is off for read transactions.
            pub const ARCACHE_USDHC_0: u32 = 0b0;

            /// 0b1: Cacheable attribute is on for read transactions.
            pub const ARCACHE_USDHC_1: u32 = 0b1;
        }
    }

    /// uSDHC block cacheable attribute value of AXI write transactions
    pub mod AWCACHE_USDHC {
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

            /// 0b0: Cacheable attribute is off for write transactions.
            pub const AWCACHE_USDHC_0: u32 = 0b0;

            /// 0b1: Cacheable attribute is on for write transactions.
            pub const AWCACHE_USDHC_1: u32 = 0b1;
        }
    }

    /// USB block cacheable attribute value of AXI transactions
    pub mod CACHE_USB {
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

            /// 0b0: Cacheable attribute is off for read/write transactions.
            pub const CACHE_USB_0: u32 = 0b0;

            /// 0b1: Cacheable attribute is on for read/write transactions.
            pub const CACHE_USB_1: u32 = 0b1;
        }
    }
}

/// GPR14 General Purpose Register
pub mod GPR14 {

    /// ITCM total size configuration
    pub mod CM7_CFGITCMSZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 0 KB (No ITCM)
            pub const CM7_CFGITCMSZ_0: u32 = 0b0000;

            /// 0b0011: 4 KB
            pub const CM7_CFGITCMSZ_3: u32 = 0b0011;

            /// 0b0100: 8 KB
            pub const CM7_CFGITCMSZ_4: u32 = 0b0100;

            /// 0b0101: 16 KB
            pub const CM7_CFGITCMSZ_5: u32 = 0b0101;

            /// 0b0110: 32 KB
            pub const CM7_CFGITCMSZ_6: u32 = 0b0110;

            /// 0b0111: 64 KB
            pub const CM7_CFGITCMSZ_7: u32 = 0b0111;

            /// 0b1000: 128 KB
            pub const CM7_CFGITCMSZ_8: u32 = 0b1000;

            /// 0b1001: 256 KB
            pub const CM7_CFGITCMSZ_9: u32 = 0b1001;
        }
    }

    /// DTCM total size configuration
    pub mod CM7_CFGDTCMSZ {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: 0 KB (No DTCM)
            pub const CM7_CFGDTCMSZ_0: u32 = 0b0000;

            /// 0b0011: 4 KB
            pub const CM7_CFGDTCMSZ_3: u32 = 0b0011;

            /// 0b0100: 8 KB
            pub const CM7_CFGDTCMSZ_4: u32 = 0b0100;

            /// 0b0101: 16 KB
            pub const CM7_CFGDTCMSZ_5: u32 = 0b0101;

            /// 0b0110: 32 KB
            pub const CM7_CFGDTCMSZ_6: u32 = 0b0110;

            /// 0b0111: 64 KB
            pub const CM7_CFGDTCMSZ_7: u32 = 0b0111;

            /// 0b1000: 128 KB
            pub const CM7_CFGDTCMSZ_8: u32 = 0b1000;

            /// 0b1001: 256 KB
            pub const CM7_CFGDTCMSZ_9: u32 = 0b1001;
        }
    }
}

/// GPR15 General Purpose Register
pub mod GPR15 {}

/// GPR16 General Purpose Register
pub mod GPR16 {

    /// ITCM enable initialization out of reset
    pub mod INIT_ITCM_EN {
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

            /// 0b0: ITCM is disabled
            pub const INIT_ITCM_EN_0: u32 = 0b0;

            /// 0b1: ITCM is enabled
            pub const INIT_ITCM_EN_1: u32 = 0b1;
        }
    }

    /// DTCM enable initialization out of reset
    pub mod INIT_DTCM_EN {
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

            /// 0b0: DTCM is disabled
            pub const INIT_DTCM_EN_0: u32 = 0b0;

            /// 0b1: DTCM is enabled
            pub const INIT_DTCM_EN_1: u32 = 0b1;
        }
    }

    /// FlexRAM bank config source select
    pub mod FLEXRAM_BANK_CFG_SEL {
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

            /// 0b0: use fuse value to config
            pub const FLEXRAM_BANK_CFG_SEL_0: u32 = 0b0;

            /// 0b1: use FLEXRAM_BANK_CFG to config
            pub const FLEXRAM_BANK_CFG_SEL_1: u32 = 0b1;
        }
    }
}

/// GPR17 General Purpose Register
pub mod GPR17 {

    /// FlexRAM bank config value
    pub mod FLEXRAM_BANK_CFG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR18 General Purpose Register
pub mod GPR18 {

    /// lock M7_APC_AC_R0_BOT field for changes
    pub mod LOCK_M7_APC_AC_R0_BOT {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R0_BOT_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R0_BOT_1: u32 = 0b1;
        }
    }

    /// APC end address of memory region-0
    pub mod M7_APC_AC_R0_BOT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR19 General Purpose Register
pub mod GPR19 {

    /// lock M7_APC_AC_R0_TOP field for changes
    pub mod LOCK_M7_APC_AC_R0_TOP {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R0_TOP_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R0_TOP_1: u32 = 0b1;
        }
    }

    /// APC start address of memory region-0
    pub mod M7_APC_AC_R0_TOP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR20 General Purpose Register
pub mod GPR20 {

    /// lock M7_APC_AC_R1_BOT field for changes
    pub mod LOCK_M7_APC_AC_R1_BOT {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R1_BOT_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R1_BOT_1: u32 = 0b1;
        }
    }

    /// APC end address of memory region-1
    pub mod M7_APC_AC_R1_BOT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR21 General Purpose Register
pub mod GPR21 {

    /// lock M7_APC_AC_R1_TOP field for changes
    pub mod LOCK_M7_APC_AC_R1_TOP {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R1_TOP_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R1_TOP_1: u32 = 0b1;
        }
    }

    /// APC start address of memory region-1
    pub mod M7_APC_AC_R1_TOP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR22 General Purpose Register
pub mod GPR22 {

    /// lock M7_APC_AC_R2_BOT field for changes
    pub mod LOCK_M7_APC_AC_R2_BOT {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R2_BOT_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R2_BOT_1: u32 = 0b1;
        }
    }

    /// APC end address of memory region-2
    pub mod M7_APC_AC_R2_BOT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR23 General Purpose Register
pub mod GPR23 {

    /// lock M7_APC_AC_R2_TOP field for changes
    pub mod LOCK_M7_APC_AC_R2_TOP {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R2_TOP_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R2_TOP_1: u32 = 0b1;
        }
    }

    /// APC start address of memory region-2
    pub mod M7_APC_AC_R2_TOP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR24 General Purpose Register
pub mod GPR24 {

    /// lock M7_APC_AC_R3_BOT field for changes
    pub mod LOCK_M7_APC_AC_R3_BOT {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R3_BOT_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R3_BOT_1: u32 = 0b1;
        }
    }

    /// APC end address of memory region-3
    pub mod M7_APC_AC_R3_BOT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR25 General Purpose Register
pub mod GPR25 {

    /// lock M7_APC_AC_R3_TOP field for changes
    pub mod LOCK_M7_APC_AC_R3_TOP {
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

            /// 0b0: Register field \[31:1\] is not locked
            pub const LOCK_M7_APC_AC_R3_TOP_0: u32 = 0b0;

            /// 0b1: Register field \[31:1\] is locked (read access only)
            pub const LOCK_M7_APC_AC_R3_TOP_1: u32 = 0b1;
        }
    }

    /// APC start address of memory region-3
    pub mod M7_APC_AC_R3_TOP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// GPR0 General Purpose Register
    pub GPR0: RORegister<u32>,

    /// GPR1 General Purpose Register
    pub GPR1: RWRegister<u32>,

    /// GPR2 General Purpose Register
    pub GPR2: RWRegister<u32>,

    /// GPR3 General Purpose Register
    pub GPR3: RWRegister<u32>,

    /// GPR4 General Purpose Register
    pub GPR4: RWRegister<u32>,

    /// GPR5 General Purpose Register
    pub GPR5: RWRegister<u32>,

    /// GPR6 General Purpose Register
    pub GPR6: RWRegister<u32>,

    /// GPR7 General Purpose Register
    pub GPR7: RWRegister<u32>,

    /// GPR8 General Purpose Register
    pub GPR8: RWRegister<u32>,

    /// GPR9 General Purpose Register
    pub GPR9: RORegister<u32>,

    /// GPR10 General Purpose Register
    pub GPR10: RWRegister<u32>,

    /// GPR11 General Purpose Register
    pub GPR11: RWRegister<u32>,

    /// GPR12 General Purpose Register
    pub GPR12: RWRegister<u32>,

    /// GPR13 General Purpose Register
    pub GPR13: RWRegister<u32>,

    /// GPR14 General Purpose Register
    pub GPR14: RWRegister<u32>,

    /// GPR15 General Purpose Register
    pub GPR15: RORegister<u32>,

    /// GPR16 General Purpose Register
    pub GPR16: RWRegister<u32>,

    /// GPR17 General Purpose Register
    pub GPR17: RWRegister<u32>,

    /// GPR18 General Purpose Register
    pub GPR18: RWRegister<u32>,

    /// GPR19 General Purpose Register
    pub GPR19: RWRegister<u32>,

    /// GPR20 General Purpose Register
    pub GPR20: RWRegister<u32>,

    /// GPR21 General Purpose Register
    pub GPR21: RWRegister<u32>,

    /// GPR22 General Purpose Register
    pub GPR22: RWRegister<u32>,

    /// GPR23 General Purpose Register
    pub GPR23: RWRegister<u32>,

    /// GPR24 General Purpose Register
    pub GPR24: RWRegister<u32>,

    /// GPR25 General Purpose Register
    pub GPR25: RWRegister<u32>,
}
pub struct ResetValues {
    pub GPR0: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
    pub GPR11: u32,
    pub GPR12: u32,
    pub GPR13: u32,
    pub GPR14: u32,
    pub GPR15: u32,
    pub GPR16: u32,
    pub GPR17: u32,
    pub GPR18: u32,
    pub GPR19: u32,
    pub GPR20: u32,
    pub GPR21: u32,
    pub GPR22: u32,
    pub GPR23: u32,
    pub GPR24: u32,
    pub GPR25: u32,
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

/// Access functions for the IOMUXC_GPR peripheral instance
pub mod IOMUXC_GPR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400ac000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC_GPR
    pub const reset: ResetValues = ResetValues {
        GPR0: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x0000FFF0,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000007,
        GPR11: 0x00000000,
        GPR12: 0x00000000,
        GPR13: 0x00000000,
        GPR14: 0x00000000,
        GPR15: 0xFFFFFFFF,
        GPR16: 0x00200003,
        GPR17: 0x00000000,
        GPR18: 0x00000000,
        GPR19: 0x00000000,
        GPR20: 0x00000000,
        GPR21: 0x00000000,
        GPR22: 0x00000000,
        GPR23: 0x00000000,
        GPR24: 0x00000000,
        GPR25: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_GPR_TAKEN: bool = false;

    /// Safe access to IOMUXC_GPR
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
            if IOMUXC_GPR_TAKEN {
                None
            } else {
                IOMUXC_GPR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC_GPR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_GPR_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_GPR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC_GPR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_GPR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC_GPR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_GPR: *const RegisterBlock = 0x400ac000 as *const _;
