#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CCM Control Register
pub mod CCR {

    /// Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened.
    pub mod OSCNT {
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

    /// On chip oscillator enable bit - this bit value is reflected on the output cosc_en
    pub mod COSC_EN {
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

            /// 0b0: disable on chip oscillator
            pub const COSC_EN_0: u32 = 0b0;

            /// 0b1: enable on chip oscillator
            pub const COSC_EN_1: u32 = 0b1;
        }
    }

    /// Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ
    pub mod REG_BYPASS_COUNT {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (6 bits: 0x3f << 21)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: no delay
            pub const REG_BYPASS_COUNT_0: u32 = 0b000000;

            /// 0b000001: 1 CKIL clock period delay
            pub const REG_BYPASS_COUNT_1: u32 = 0b000001;

            /// 0b111111: 63 CKIL clock periods delay
            pub const REG_BYPASS_COUNT_63: u32 = 0b111111;
        }
    }

    /// Enable for REG_BYPASS_COUNTER
    pub mod RBC_EN {
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

            /// 0b0: REG_BYPASS_COUNTER disabled
            pub const RBC_EN_0: u32 = 0b0;

            /// 0b1: REG_BYPASS_COUNTER enabled.
            pub const RBC_EN_1: u32 = 0b1;
        }
    }
}

/// CCM Status Register
pub mod CSR {

    /// Status of the value of CCM_REF_EN_B output of ccm
    pub mod REF_EN_B {
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

            /// 0b0: value of CCM_REF_EN_B is '0'
            pub const REF_EN_B_0: u32 = 0b0;

            /// 0b1: value of CCM_REF_EN_B is '1'
            pub const REF_EN_B_1: u32 = 0b1;
        }
    }

    /// Status indication of CAMP2.
    pub mod CAMP2_READY {
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

            /// 0b0: CAMP2 is not ready.
            pub const CAMP2_READY_0: u32 = 0b0;

            /// 0b1: CAMP2 is ready.
            pub const CAMP2_READY_1: u32 = 0b1;
        }
    }

    /// Status indication of on board oscillator
    pub mod COSC_READY {
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

            /// 0b0: on board oscillator is not ready.
            pub const COSC_READY_0: u32 = 0b0;

            /// 0b1: on board oscillator is ready.
            pub const COSC_READY_1: u32 = 0b1;
        }
    }
}

/// CCM Clock Switcher Register
pub mod CCSR {

    /// Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes.
    pub mod PLL3_SW_CLK_SEL {
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

            /// 0b0: pll3_main_clk
            pub const PLL3_SW_CLK_SEL_0: u32 = 0b0;

            /// 0b1: pll3 bypass clock
            pub const PLL3_SW_CLK_SEL_1: u32 = 0b1;
        }
    }
}

/// CCM Arm Clock Root Register
pub mod CACRR {

    /// Divider for ARM clock root
    pub mod ARM_PODF {
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

            /// 0b000: divide by 1
            pub const ARM_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const ARM_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const ARM_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const ARM_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const ARM_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const ARM_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const ARM_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const ARM_PODF_7: u32 = 0b111;
        }
    }
}

/// CCM Bus Clock Divider Register
pub mod CBCDR {

    /// SEMC clock source select
    pub mod SEMC_CLK_SEL {
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

            /// 0b0: Periph_clk output will be used as SEMC clock root
            pub const SEMC_CLK_SEL_0: u32 = 0b0;

            /// 0b1: SEMC alternative clock will be used as SEMC clock root
            pub const SEMC_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// SEMC alternative clock select
    pub mod SEMC_ALT_CLK_SEL {
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

            /// 0b0: PLL2 PFD2 will be selected as alternative clock for SEMC root clock
            pub const SEMC_ALT_CLK_SEL_0: u32 = 0b0;

            /// 0b1: PLL3 PFD1 will be selected as alternative clock for SEMC root clock
            pub const SEMC_ALT_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Divider for ipg podf.
    pub mod IPG_PODF {
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

            /// 0b00: divide by 1
            pub const IPG_PODF_0: u32 = 0b00;

            /// 0b01: divide by 2
            pub const IPG_PODF_1: u32 = 0b01;

            /// 0b10: divide by 3
            pub const IPG_PODF_2: u32 = 0b10;

            /// 0b11: divide by 4
            pub const IPG_PODF_3: u32 = 0b11;
        }
    }

    /// Divider for AHB PODF
    pub mod AHB_PODF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const AHB_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const AHB_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const AHB_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const AHB_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const AHB_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const AHB_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const AHB_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const AHB_PODF_7: u32 = 0b111;
        }
    }

    /// Post divider for SEMC clock
    pub mod SEMC_PODF {
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

            /// 0b000: divide by 1
            pub const SEMC_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const SEMC_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const SEMC_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const SEMC_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const SEMC_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const SEMC_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const SEMC_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const SEMC_PODF_7: u32 = 0b111;
        }
    }

    /// Selector for peripheral main clock
    pub mod PERIPH_CLK_SEL {
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

            /// 0b0: derive clock from pre_periph_clk_sel
            pub const PERIPH_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from periph_clk2_clk_divided
            pub const PERIPH_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Divider for periph_clk2_podf.
    pub mod PERIPH_CLK2_PODF {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const PERIPH_CLK2_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const PERIPH_CLK2_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const PERIPH_CLK2_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const PERIPH_CLK2_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const PERIPH_CLK2_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const PERIPH_CLK2_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const PERIPH_CLK2_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const PERIPH_CLK2_PODF_7: u32 = 0b111;
        }
    }
}

/// CCM Bus Clock Multiplexer Register
pub mod CBCMR {

    /// Selector for lpspi clock multiplexer
    pub mod LPSPI_CLK_SEL {
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

            /// 0b00: derive clock from PLL3 PFD1 clk
            pub const LPSPI_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL3 PFD0
            pub const LPSPI_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL2
            pub const LPSPI_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from PLL2 PFD2
            pub const LPSPI_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Selector for flexspi2 clock multiplexer
    pub mod FLEXSPI2_CLK_SEL {
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

            /// 0b00: derive clock from PLL2 PFD2
            pub const FLEXSPI2_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL3 PFD0
            pub const FLEXSPI2_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL3 PFD1
            pub const FLEXSPI2_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from PLL2 (pll2_main_clk)
            pub const FLEXSPI2_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Selector for peripheral clk2 clock multiplexer
    pub mod PERIPH_CLK2_SEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from pll3_sw_clk
            pub const PERIPH_CLK2_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from osc_clk (pll1_ref_clk)
            pub const PERIPH_CLK2_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from pll2_bypass_clk
            pub const PERIPH_CLK2_SEL_2: u32 = 0b10;
        }
    }

    /// Selector for Trace clock multiplexer
    pub mod TRACE_CLK_SEL {
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

            /// 0b00: derive clock from PLL2
            pub const TRACE_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL2 PFD2
            pub const TRACE_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL2 PFD0
            pub const TRACE_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from PLL2 PFD1
            pub const TRACE_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Selector for pre_periph clock multiplexer
    pub mod PRE_PERIPH_CLK_SEL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from PLL2
            pub const PRE_PERIPH_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL2 PFD2
            pub const PRE_PERIPH_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL2 PFD0
            pub const PRE_PERIPH_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from divided PLL1
            pub const PRE_PERIPH_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Post-divider for LCDIF clock.
    pub mod LCDIF_PODF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const LCDIF_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const LCDIF_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const LCDIF_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const LCDIF_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const LCDIF_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const LCDIF_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const LCDIF_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const LCDIF_PODF_7: u32 = 0b111;
        }
    }

    /// Divider for LPSPI. Divider should be updated when output clock is gated.
    pub mod LPSPI_PODF {
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

            /// 0b000: divide by 1
            pub const LPSPI_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const LPSPI_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const LPSPI_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const LPSPI_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const LPSPI_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const LPSPI_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const LPSPI_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const LPSPI_PODF_7: u32 = 0b111;
        }
    }

    /// Divider for flexspi2 clock root.
    pub mod FLEXSPI2_PODF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const FLEXSPI2_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const FLEXSPI2_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const FLEXSPI2_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const FLEXSPI2_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const FLEXSPI2_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const FLEXSPI2_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const FLEXSPI2_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const FLEXSPI2_PODF_7: u32 = 0b111;
        }
    }
}

/// CCM Serial Clock Multiplexer Register 1
pub mod CSCMR1 {

    /// Divider for perclk podf.
    pub mod PERCLK_PODF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }

    /// Selector for the perclk clock multiplexor
    pub mod PERCLK_CLK_SEL {
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

            /// 0b0: derive clock from ipg clk root
            pub const PERCLK_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from osc_clk
            pub const PERCLK_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Selector for sai1 clock multiplexer
    pub mod SAI1_CLK_SEL {
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

            /// 0b00: derive clock from PLL3 PFD2
            pub const SAI1_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL5
            pub const SAI1_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL4
            pub const SAI1_CLK_SEL_2: u32 = 0b10;
        }
    }

    /// Selector for sai2 clock multiplexer
    pub mod SAI2_CLK_SEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from PLL3 PFD2
            pub const SAI2_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL5
            pub const SAI2_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL4
            pub const SAI2_CLK_SEL_2: u32 = 0b10;
        }
    }

    /// Selector for sai3/adc1/adc2 clock multiplexer
    pub mod SAI3_CLK_SEL {
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

            /// 0b00: derive clock from PLL3 PFD2
            pub const SAI3_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL5
            pub const SAI3_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL4
            pub const SAI3_CLK_SEL_2: u32 = 0b10;
        }
    }

    /// Selector for usdhc1 clock multiplexer
    pub mod USDHC1_CLK_SEL {
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

            /// 0b0: derive clock from PLL2 PFD2
            pub const USDHC1_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from PLL2 PFD0
            pub const USDHC1_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Selector for usdhc2 clock multiplexer
    pub mod USDHC2_CLK_SEL {
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

            /// 0b0: derive clock from PLL2 PFD2
            pub const USDHC2_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from PLL2 PFD0
            pub const USDHC2_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Divider for flexspi clock root.
    pub mod FLEXSPI_PODF {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const FLEXSPI_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const FLEXSPI_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const FLEXSPI_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const FLEXSPI_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const FLEXSPI_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const FLEXSPI_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const FLEXSPI_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const FLEXSPI_PODF_7: u32 = 0b111;
        }
    }

    /// Selector for flexspi clock multiplexer
    pub mod FLEXSPI_CLK_SEL {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from semc_clk_root_pre
            pub const FLEXSPI_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from pll3_sw_clk
            pub const FLEXSPI_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL2 PFD2
            pub const FLEXSPI_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from PLL3 PFD0
            pub const FLEXSPI_CLK_SEL_3: u32 = 0b11;
        }
    }
}

/// CCM Serial Clock Multiplexer Register 2
pub mod CSCMR2 {

    /// Divider for CAN/CANFD clock podf.
    pub mod CAN_CLK_PODF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (6 bits: 0x3f << 2)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }

    /// Selector for CAN/CANFD clock multiplexer
    pub mod CAN_CLK_SEL {
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

            /// 0b00: derive clock from pll3_sw_clk divided clock (60M)
            pub const CAN_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from osc_clk (24M)
            pub const CAN_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from pll3_sw_clk divided clock (80M)
            pub const CAN_CLK_SEL_2: u32 = 0b10;

            /// 0b11: Disable FlexCAN clock
            pub const CAN_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Selector for flexio2/flexio3 clock multiplexer
    pub mod FLEXIO2_CLK_SEL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from PLL4 divided clock
            pub const FLEXIO2_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL3 PFD2 clock
            pub const FLEXIO2_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL5 clock
            pub const FLEXIO2_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from pll3_sw_clk
            pub const FLEXIO2_CLK_SEL_3: u32 = 0b11;
        }
    }
}

/// CCM Serial Clock Divider Register 1
pub mod CSCDR1 {

    /// Divider for uart clock podf.
    pub mod UART_CLK_PODF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }

    /// Selector for the UART clock multiplexor
    pub mod UART_CLK_SEL {
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

            /// 0b0: derive clock from pll3_80m
            pub const UART_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from osc_clk
            pub const UART_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Divider for usdhc1 clock podf. Divider should be updated when output clock is gated.
    pub mod USDHC1_PODF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const USDHC1_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const USDHC1_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const USDHC1_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const USDHC1_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const USDHC1_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const USDHC1_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const USDHC1_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const USDHC1_PODF_7: u32 = 0b111;
        }
    }

    /// Divider for usdhc2 clock. Divider should be updated when output clock is gated.
    pub mod USDHC2_PODF {
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

            /// 0b000: divide by 1
            pub const USDHC2_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const USDHC2_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const USDHC2_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const USDHC2_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const USDHC2_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const USDHC2_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const USDHC2_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const USDHC2_PODF_7: u32 = 0b111;
        }
    }

    /// Divider for trace clock. Divider should be updated when output clock is gated.
    pub mod TRACE_PODF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: divide by 1
            pub const TRACE_PODF_0: u32 = 0b00;

            /// 0b01: divide by 2
            pub const TRACE_PODF_1: u32 = 0b01;

            /// 0b10: divide by 3
            pub const TRACE_PODF_2: u32 = 0b10;

            /// 0b11: divide by 4
            pub const TRACE_PODF_3: u32 = 0b11;
        }
    }
}

/// CCM Clock Divider Register
pub mod CS1CDR {

    /// Divider for sai1 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this.
    pub mod SAI1_CLK_PODF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }

    /// Divider for sai1 clock pred.
    pub mod SAI1_CLK_PRED {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const SAI1_CLK_PRED_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const SAI1_CLK_PRED_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const SAI1_CLK_PRED_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const SAI1_CLK_PRED_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const SAI1_CLK_PRED_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const SAI1_CLK_PRED_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const SAI1_CLK_PRED_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const SAI1_CLK_PRED_7: u32 = 0b111;
        }
    }

    /// Divider for flexio2/flexio3 clock.
    pub mod FLEXIO2_CLK_PRED {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const FLEXIO2_CLK_PRED_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const FLEXIO2_CLK_PRED_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const FLEXIO2_CLK_PRED_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const FLEXIO2_CLK_PRED_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const FLEXIO2_CLK_PRED_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const FLEXIO2_CLK_PRED_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const FLEXIO2_CLK_PRED_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const FLEXIO2_CLK_PRED_7: u32 = 0b111;
        }
    }

    /// Divider for sai3 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this.
    pub mod SAI3_CLK_PODF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SAI1_CLK_PODF::RW;
    }

    /// Divider for sai3/adc1/adc2 clock pred.
    pub mod SAI3_CLK_PRED {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const SAI3_CLK_PRED_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const SAI3_CLK_PRED_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const SAI3_CLK_PRED_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const SAI3_CLK_PRED_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const SAI3_CLK_PRED_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const SAI3_CLK_PRED_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const SAI3_CLK_PRED_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const SAI3_CLK_PRED_7: u32 = 0b111;
        }
    }

    /// Divider for flexio2/flexio3 clock. Divider should be updated when output clock is gated.
    pub mod FLEXIO2_CLK_PODF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const DIVIDE_2: u32 = 0b001;

            /// 0b010: Divide by 3
            pub const DIVIDE_3: u32 = 0b010;

            /// 0b011: Divide by 4
            pub const DIVIDE_4: u32 = 0b011;

            /// 0b100: Divide by 5
            pub const DIVIDE_5: u32 = 0b100;

            /// 0b101: Divide by 6
            pub const DIVIDE_6: u32 = 0b101;

            /// 0b110: Divide by 7
            pub const DIVIDE_7: u32 = 0b110;

            /// 0b111: Divide by 8
            pub const DIVIDE_8: u32 = 0b111;
        }
    }
}

/// CCM Clock Divider Register
pub mod CS2CDR {

    /// Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this.
    pub mod SAI2_CLK_PODF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }

    /// Divider for sai2 clock pred.Divider should be updated when output clock is gated.
    pub mod SAI2_CLK_PRED {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const SAI2_CLK_PRED_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const SAI2_CLK_PRED_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const SAI2_CLK_PRED_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const SAI2_CLK_PRED_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const SAI2_CLK_PRED_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const SAI2_CLK_PRED_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const SAI2_CLK_PRED_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const SAI2_CLK_PRED_7: u32 = 0b111;
        }
    }
}

/// CCM D1 Clock Divider Register
pub mod CDCDR {

    /// Selector for flexio1 clock multiplexer
    pub mod FLEXIO1_CLK_SEL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from PLL4
            pub const FLEXIO1_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL3 PFD2
            pub const FLEXIO1_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL5
            pub const FLEXIO1_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from pll3_sw_clk
            pub const FLEXIO1_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Divider for flexio1 clock podf. Divider should be updated when output clock is gated.
    pub mod FLEXIO1_CLK_PODF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const DIVIDE_2: u32 = 0b001;

            /// 0b010: Divide by 3
            pub const DIVIDE_3: u32 = 0b010;

            /// 0b011: Divide by 4
            pub const DIVIDE_4: u32 = 0b011;

            /// 0b100: Divide by 5
            pub const DIVIDE_5: u32 = 0b100;

            /// 0b101: Divide by 6
            pub const DIVIDE_6: u32 = 0b101;

            /// 0b110: Divide by 7
            pub const DIVIDE_7: u32 = 0b110;

            /// 0b111: Divide by 8
            pub const DIVIDE_8: u32 = 0b111;
        }
    }

    /// Divider for flexio1 clock pred. Divider should be updated when output clock is gated.
    pub mod FLEXIO1_CLK_PRED {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLEXIO1_CLK_PODF::RW;
    }

    /// Selector for spdif0 clock multiplexer
    pub mod SPDIF0_CLK_SEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from PLL4
            pub const SPDIF0_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL3 PFD2
            pub const SPDIF0_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from PLL5
            pub const SPDIF0_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from pll3_sw_clk
            pub const SPDIF0_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Divider for spdif0 clock podf. Divider should be updated when output clock is gated.
    pub mod SPDIF0_CLK_PODF {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (3 bits: 0b111 << 22)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLEXIO1_CLK_PODF::RW;
    }

    /// Divider for spdif0 clock pred. Divider should be updated when output clock is gated.
    pub mod SPDIF0_CLK_PRED {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FLEXIO1_CLK_PODF::RW;
    }
}

/// CCM Serial Clock Divider Register 2
pub mod CSCDR2 {

    /// Pre-divider for lcdif clock. Divider should be updated when output clock is gated.
    pub mod LCDIF_PRED {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const LCDIF_PRED_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const LCDIF_PRED_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const LCDIF_PRED_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const LCDIF_PRED_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const LCDIF_PRED_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const LCDIF_PRED_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const LCDIF_PRED_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const LCDIF_PRED_7: u32 = 0b111;
        }
    }

    /// Selector for lcdif root clock pre-multiplexer
    pub mod LCDIF_PRE_CLK_SEL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: derive clock from PLL2
            pub const LCDIF_PRE_CLK_SEL_0: u32 = 0b000;

            /// 0b001: derive clock from PLL3 PFD3
            pub const LCDIF_PRE_CLK_SEL_1: u32 = 0b001;

            /// 0b010: derive clock from PLL5
            pub const LCDIF_PRE_CLK_SEL_2: u32 = 0b010;

            /// 0b011: derive clock from PLL2 PFD0
            pub const LCDIF_PRE_CLK_SEL_3: u32 = 0b011;

            /// 0b100: derive clock from PLL2 PFD1
            pub const LCDIF_PRE_CLK_SEL_4: u32 = 0b100;

            /// 0b101: derive clock from PLL3 PFD1
            pub const LCDIF_PRE_CLK_SEL_5: u32 = 0b101;
        }
    }

    /// Selector for the LPI2C clock multiplexor
    pub mod LPI2C_CLK_SEL {
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

            /// 0b0: derive clock from pll3_60m
            pub const LPI2C_CLK_SEL_0: u32 = 0b0;

            /// 0b1: derive clock from osc_clk
            pub const LPI2C_CLK_SEL_1: u32 = 0b1;
        }
    }

    /// Divider for lpi2c clock podf. Divider should be updated when output clock is gated. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this.
    pub mod LPI2C_CLK_PODF {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (6 bits: 0x3f << 19)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Divide by 1
            pub const DIVIDE_1: u32 = 0b000000;

            /// 0b000001: Divide by 2
            pub const DIVIDE_2: u32 = 0b000001;

            /// 0b000010: Divide by 3
            pub const DIVIDE_3: u32 = 0b000010;

            /// 0b000011: Divide by 4
            pub const DIVIDE_4: u32 = 0b000011;

            /// 0b000100: Divide by 5
            pub const DIVIDE_5: u32 = 0b000100;

            /// 0b000101: Divide by 6
            pub const DIVIDE_6: u32 = 0b000101;

            /// 0b000110: Divide by 7
            pub const DIVIDE_7: u32 = 0b000110;

            /// 0b000111: Divide by 8
            pub const DIVIDE_8: u32 = 0b000111;

            /// 0b001000: Divide by 9
            pub const DIVIDE_9: u32 = 0b001000;

            /// 0b001001: Divide by 10
            pub const DIVIDE_10: u32 = 0b001001;

            /// 0b001010: Divide by 11
            pub const DIVIDE_11: u32 = 0b001010;

            /// 0b001011: Divide by 12
            pub const DIVIDE_12: u32 = 0b001011;

            /// 0b001100: Divide by 13
            pub const DIVIDE_13: u32 = 0b001100;

            /// 0b001101: Divide by 14
            pub const DIVIDE_14: u32 = 0b001101;

            /// 0b001110: Divide by 15
            pub const DIVIDE_15: u32 = 0b001110;

            /// 0b001111: Divide by 16
            pub const DIVIDE_16: u32 = 0b001111;

            /// 0b010000: Divide by 17
            pub const DIVIDE_17: u32 = 0b010000;

            /// 0b010001: Divide by 18
            pub const DIVIDE_18: u32 = 0b010001;

            /// 0b010010: Divide by 19
            pub const DIVIDE_19: u32 = 0b010010;

            /// 0b010011: Divide by 20
            pub const DIVIDE_20: u32 = 0b010011;

            /// 0b010100: Divide by 21
            pub const DIVIDE_21: u32 = 0b010100;

            /// 0b010101: Divide by 22
            pub const DIVIDE_22: u32 = 0b010101;

            /// 0b010110: Divide by 23
            pub const DIVIDE_23: u32 = 0b010110;

            /// 0b010111: Divide by 24
            pub const DIVIDE_24: u32 = 0b010111;

            /// 0b011000: Divide by 25
            pub const DIVIDE_25: u32 = 0b011000;

            /// 0b011001: Divide by 26
            pub const DIVIDE_26: u32 = 0b011001;

            /// 0b011010: Divide by 27
            pub const DIVIDE_27: u32 = 0b011010;

            /// 0b011011: Divide by 28
            pub const DIVIDE_28: u32 = 0b011011;

            /// 0b011100: Divide by 29
            pub const DIVIDE_29: u32 = 0b011100;

            /// 0b011101: Divide by 30
            pub const DIVIDE_30: u32 = 0b011101;

            /// 0b011110: Divide by 31
            pub const DIVIDE_31: u32 = 0b011110;

            /// 0b011111: Divide by 32
            pub const DIVIDE_32: u32 = 0b011111;

            /// 0b100000: Divide by 33
            pub const DIVIDE_33: u32 = 0b100000;

            /// 0b100001: Divide by 34
            pub const DIVIDE_34: u32 = 0b100001;

            /// 0b100010: Divide by 35
            pub const DIVIDE_35: u32 = 0b100010;

            /// 0b100011: Divide by 36
            pub const DIVIDE_36: u32 = 0b100011;

            /// 0b100100: Divide by 37
            pub const DIVIDE_37: u32 = 0b100100;

            /// 0b100101: Divide by 38
            pub const DIVIDE_38: u32 = 0b100101;

            /// 0b100110: Divide by 39
            pub const DIVIDE_39: u32 = 0b100110;

            /// 0b100111: Divide by 40
            pub const DIVIDE_40: u32 = 0b100111;

            /// 0b101000: Divide by 41
            pub const DIVIDE_41: u32 = 0b101000;

            /// 0b101001: Divide by 42
            pub const DIVIDE_42: u32 = 0b101001;

            /// 0b101010: Divide by 43
            pub const DIVIDE_43: u32 = 0b101010;

            /// 0b101011: Divide by 44
            pub const DIVIDE_44: u32 = 0b101011;

            /// 0b101100: Divide by 45
            pub const DIVIDE_45: u32 = 0b101100;

            /// 0b101101: Divide by 46
            pub const DIVIDE_46: u32 = 0b101101;

            /// 0b101110: Divide by 47
            pub const DIVIDE_47: u32 = 0b101110;

            /// 0b101111: Divide by 48
            pub const DIVIDE_48: u32 = 0b101111;

            /// 0b110000: Divide by 49
            pub const DIVIDE_49: u32 = 0b110000;

            /// 0b110001: Divide by 50
            pub const DIVIDE_50: u32 = 0b110001;

            /// 0b110010: Divide by 51
            pub const DIVIDE_51: u32 = 0b110010;

            /// 0b110011: Divide by 52
            pub const DIVIDE_52: u32 = 0b110011;

            /// 0b110100: Divide by 53
            pub const DIVIDE_53: u32 = 0b110100;

            /// 0b110101: Divide by 54
            pub const DIVIDE_54: u32 = 0b110101;

            /// 0b110110: Divide by 55
            pub const DIVIDE_55: u32 = 0b110110;

            /// 0b110111: Divide by 56
            pub const DIVIDE_56: u32 = 0b110111;

            /// 0b111000: Divide by 57
            pub const DIVIDE_57: u32 = 0b111000;

            /// 0b111001: Divide by 58
            pub const DIVIDE_58: u32 = 0b111001;

            /// 0b111010: Divide by 59
            pub const DIVIDE_59: u32 = 0b111010;

            /// 0b111011: Divide by 60
            pub const DIVIDE_60: u32 = 0b111011;

            /// 0b111100: Divide by 61
            pub const DIVIDE_61: u32 = 0b111100;

            /// 0b111101: Divide by 62
            pub const DIVIDE_62: u32 = 0b111101;

            /// 0b111110: Divide by 63
            pub const DIVIDE_63: u32 = 0b111110;

            /// 0b111111: Divide by 64
            pub const DIVIDE_64: u32 = 0b111111;
        }
    }
}

/// CCM Serial Clock Divider Register 3
pub mod CSCDR3 {

    /// Selector for csi_mclk multiplexer
    pub mod CSI_CLK_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: derive clock from osc_clk (24M)
            pub const CSI_CLK_SEL_0: u32 = 0b00;

            /// 0b01: derive clock from PLL2 PFD2
            pub const CSI_CLK_SEL_1: u32 = 0b01;

            /// 0b10: derive clock from pll3_120M
            pub const CSI_CLK_SEL_2: u32 = 0b10;

            /// 0b11: derive clock from PLL3 PFD1
            pub const CSI_CLK_SEL_3: u32 = 0b11;
        }
    }

    /// Post divider for csi_mclk. Divider should be updated when output clock is gated.
    pub mod CSI_PODF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const CSI_PODF_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const CSI_PODF_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const CSI_PODF_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const CSI_PODF_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const CSI_PODF_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const CSI_PODF_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const CSI_PODF_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const CSI_PODF_7: u32 = 0b111;
        }
    }
}

/// CCM Divider Handshake In-Process Register
pub mod CDHIPR {

    /// Busy indicator for semc_podf.
    pub mod SEMC_PODF_BUSY {
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

            /// 0b0: divider is not busy and its value represents the actual division.
            pub const SEMC_PODF_BUSY_0: u32 = 0b0;

            /// 0b1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied.
            pub const SEMC_PODF_BUSY_1: u32 = 0b1;
        }
    }

    /// Busy indicator for ahb_podf.
    pub mod AHB_PODF_BUSY {
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

            /// 0b0: divider is not busy and its value represents the actual division.
            pub const AHB_PODF_BUSY_0: u32 = 0b0;

            /// 0b1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied.
            pub const AHB_PODF_BUSY_1: u32 = 0b1;
        }
    }

    /// Busy indicator for periph2_clk_sel mux control.
    pub mod PERIPH2_CLK_SEL_BUSY {
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

            /// 0b0: mux is not busy and its value represents the actual division.
            pub const PERIPH2_CLK_SEL_BUSY_0: u32 = 0b0;

            /// 0b1: mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied.
            pub const PERIPH2_CLK_SEL_BUSY_1: u32 = 0b1;
        }
    }

    /// Busy indicator for periph_clk_sel mux control.
    pub mod PERIPH_CLK_SEL_BUSY {
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

            /// 0b0: mux is not busy and its value represents the actual division.
            pub const PERIPH_CLK_SEL_BUSY_0: u32 = 0b0;

            /// 0b1: mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied.
            pub const PERIPH_CLK_SEL_BUSY_1: u32 = 0b1;
        }
    }

    /// Busy indicator for arm_podf.
    pub mod ARM_PODF_BUSY {
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

            /// 0b0: divider is not busy and its value represents the actual division.
            pub const ARM_PODF_BUSY_0: u32 = 0b0;

            /// 0b1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied.
            pub const ARM_PODF_BUSY_1: u32 = 0b1;
        }
    }
}

/// CCM Low Power Control Register
pub mod CLPCR {

    /// Setting the low power mode that system will enter on next assertion of dsm_request signal.
    pub mod LPM {
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

            /// 0b00: Remain in run mode
            pub const LPM_0: u32 = 0b00;

            /// 0b01: Transfer to wait mode
            pub const LPM_1: u32 = 0b01;

            /// 0b10: Transfer to stop mode
            pub const LPM_2: u32 = 0b10;
        }
    }

    /// Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode
    pub mod ARM_CLK_DIS_ON_LPM {
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

            /// 0b0: ARM clock enabled on wait mode.
            pub const ARM_CLK_DIS_ON_LPM_0: u32 = 0b0;

            /// 0b1: ARM clock disabled on wait mode. .
            pub const ARM_CLK_DIS_ON_LPM_1: u32 = 0b1;
        }
    }

    /// Standby clock oscillator bit
    pub mod SBYOS {
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

            /// 0b0: On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')
            pub const SBYOS_0: u32 = 0b0;

            /// 0b1: On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process.
            pub const SBYOS_1: u32 = 0b1;
        }
    }

    /// dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i
    pub mod DIS_REF_OSC {
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

            /// 0b0: external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'.
            pub const DIS_REF_OSC_0: u32 = 0b0;

            /// 0b1: external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'
            pub const DIS_REF_OSC_1: u32 = 0b1;
        }
    }

    /// Voltage standby request bit
    pub mod VSTBY {
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

            /// 0b0: Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')
            pub const VSTBY_0: u32 = 0b0;

            /// 0b1: Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1').
            pub const VSTBY_1: u32 = 0b1;
        }
    }

    /// Standby counter definition
    pub mod STBY_COUNT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles
            pub const STBY_COUNT_0: u32 = 0b00;

            /// 0b01: CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles
            pub const STBY_COUNT_1: u32 = 0b01;

            /// 0b10: CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles
            pub const STBY_COUNT_2: u32 = 0b10;

            /// 0b11: CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles
            pub const STBY_COUNT_3: u32 = 0b11;
        }
    }

    /// In run mode, software can manually control powering down of on chip oscillator, i
    pub mod COSC_PWRDOWN {
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

            /// 0b0: On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'.
            pub const COSC_PWRDOWN_0: u32 = 0b0;

            /// 0b1: On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'.
            pub const COSC_PWRDOWN_1: u32 = 0b1;
        }
    }

    /// Bypass low power mode handshake. This bit should always be set to 1'b1 by software.
    pub mod BYPASS_LPM_HS1 {
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

    /// Bypass low power mode handshake. This bit should always be set to 1'b1 by software.
    pub mod BYPASS_LPM_HS0 {
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

    /// Mask WFI of core0 for entering low power mode Assertion of all bits\[27:22\] will generate low power mode request
    pub mod MASK_CORE0_WFI {
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

            /// 0b0: WFI of core0 is not masked
            pub const MASK_CORE0_WFI_0: u32 = 0b0;

            /// 0b1: WFI of core0 is masked
            pub const MASK_CORE0_WFI_1: u32 = 0b1;
        }
    }

    /// Mask SCU IDLE for entering low power mode Assertion of all bits\[27:22\] will generate low power mode request
    pub mod MASK_SCU_IDLE {
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

            /// 0b0: SCU IDLE is not masked
            pub const MASK_SCU_IDLE_0: u32 = 0b0;

            /// 0b1: SCU IDLE is masked
            pub const MASK_SCU_IDLE_1: u32 = 0b1;
        }
    }

    /// Mask L2CC IDLE for entering low power mode
    pub mod MASK_L2CC_IDLE {
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

            /// 0b0: L2CC IDLE is not masked
            pub const MASK_L2CC_IDLE_0: u32 = 0b0;

            /// 0b1: L2CC IDLE is masked
            pub const MASK_L2CC_IDLE_1: u32 = 0b1;
        }
    }
}

/// CCM Interrupt Status Register
pub mod CISR {

    /// CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs
    pub mod LRF_PLL {
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

            /// 0b0: interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs
            pub const LRF_PLL_0: u32 = 0b0;

            /// 0b1: interrupt generated due to lock ready of all enabled and not bypaseed PLLs
            pub const LRF_PLL_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 2 generated due to on board oscillator ready, i
    pub mod COSC_READY {
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

            /// 0b0: interrupt is not generated due to on board oscillator ready
            pub const COSC_READY_0: u32 = 0b0;

            /// 0b1: interrupt generated due to on board oscillator ready
            pub const COSC_READY_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 1 generated due to frequency change of semc_podf
    pub mod SEMC_PODF_LOADED {
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

            /// 0b0: interrupt is not generated due to frequency change of semc_podf
            pub const SEMC_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: interrupt generated due to frequency change of semc_podf
            pub const SEMC_PODF_LOADED_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 1 generated due to frequency change of periph2_clk_sel
    pub mod PERIPH2_CLK_SEL_LOADED {
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

            /// 0b0: interrupt is not generated due to frequency change of periph2_clk_sel
            pub const PERIPH2_CLK_SEL_LOADED_0: u32 = 0b0;

            /// 0b1: interrupt generated due to frequency change of periph2_clk_sel
            pub const PERIPH2_CLK_SEL_LOADED_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 1 generated due to frequency change of ahb_podf
    pub mod AHB_PODF_LOADED {
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

            /// 0b0: interrupt is not generated due to frequency change of ahb_podf
            pub const AHB_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: interrupt generated due to frequency change of ahb_podf
            pub const AHB_PODF_LOADED_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 1 generated due to update of periph_clk_sel.
    pub mod PERIPH_CLK_SEL_LOADED {
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

            /// 0b0: interrupt is not generated due to update of periph_clk_sel.
            pub const PERIPH_CLK_SEL_LOADED_0: u32 = 0b0;

            /// 0b1: interrupt generated due to update of periph_clk_sel.
            pub const PERIPH_CLK_SEL_LOADED_1: u32 = 0b1;
        }
    }

    /// CCM interrupt request 1 generated due to frequency change of arm_podf
    pub mod ARM_PODF_LOADED {
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

            /// 0b0: interrupt is not generated due to frequency change of arm_podf
            pub const ARM_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: interrupt generated due to frequency change of arm_podf
            pub const ARM_PODF_LOADED_1: u32 = 0b1;
        }
    }
}

/// CCM Interrupt Mask Register
pub mod CIMR {

    /// mask interrupt generation due to lrf of PLLs
    pub mod MASK_LRF_PLL {
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

            /// 0b0: don't mask interrupt due to lrf of PLLs - interrupt will be created
            pub const MASK_LRF_PLL_0: u32 = 0b0;

            /// 0b1: mask interrupt due to lrf of PLLs
            pub const MASK_LRF_PLL_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to on board oscillator ready
    pub mod MASK_COSC_READY {
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

            /// 0b0: don't mask interrupt due to on board oscillator ready - interrupt will be created
            pub const MASK_COSC_READY_0: u32 = 0b0;

            /// 0b1: mask interrupt due to on board oscillator ready
            pub const MASK_COSC_READY_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to frequency change of semc_podf
    pub mod MASK_SEMC_PODF_LOADED {
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

            /// 0b0: don't mask interrupt due to frequency change of semc_podf - interrupt will be created
            pub const MASK_SEMC_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: mask interrupt due to frequency change of semc_podf
            pub const MASK_SEMC_PODF_LOADED_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to update of periph2_clk_sel.
    pub mod MASK_PERIPH2_CLK_SEL_LOADED {
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

            /// 0b0: don't mask interrupt due to update of periph2_clk_sel - interrupt will be created
            pub const MASK_PERIPH2_CLK_SEL_LOADED_0: u32 = 0b0;

            /// 0b1: mask interrupt due to update of periph2_clk_sel
            pub const MASK_PERIPH2_CLK_SEL_LOADED_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to frequency change of ahb_podf
    pub mod MASK_AHB_PODF_LOADED {
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

            /// 0b0: don't mask interrupt due to frequency change of ahb_podf - interrupt will be created
            pub const MASK_AHB_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: mask interrupt due to frequency change of ahb_podf
            pub const MASK_AHB_PODF_LOADED_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to update of periph_clk_sel.
    pub mod MASK_PERIPH_CLK_SEL_LOADED {
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

            /// 0b0: don't mask interrupt due to update of periph_clk_sel - interrupt will be created
            pub const MASK_PERIPH_CLK_SEL_LOADED_0: u32 = 0b0;

            /// 0b1: mask interrupt due to update of periph_clk_sel
            pub const MASK_PERIPH_CLK_SEL_LOADED_1: u32 = 0b1;
        }
    }

    /// mask interrupt generation due to frequency change of arm_podf
    pub mod ARM_PODF_LOADED {
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

            /// 0b0: don't mask interrupt due to frequency change of arm_podf - interrupt will be created
            pub const ARM_PODF_LOADED_0: u32 = 0b0;

            /// 0b1: mask interrupt due to frequency change of arm_podf
            pub const ARM_PODF_LOADED_1: u32 = 0b1;
        }
    }
}

/// CCM Clock Output Source Register
pub mod CCOSR {

    /// Selection of the clock to be generated on CCM_CLKO1
    pub mod CLKO1_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: USB1 PLL clock (divided by 2)
            pub const CLKO1_SEL_0: u32 = 0b0000;

            /// 0b0001: SYS PLL clock (divided by 2)
            pub const CLKO1_SEL_1: u32 = 0b0001;

            /// 0b0011: VIDEO PLL clock (divided by 2)
            pub const CLKO1_SEL_3: u32 = 0b0011;

            /// 0b0101: semc_clk_root
            pub const CLKO1_SEL_5: u32 = 0b0101;

            /// 0b1010: lcdif_pix_clk_root
            pub const CLKO1_SEL_10: u32 = 0b1010;

            /// 0b1011: ahb_clk_root
            pub const CLKO1_SEL_11: u32 = 0b1011;

            /// 0b1100: ipg_clk_root
            pub const CLKO1_SEL_12: u32 = 0b1100;

            /// 0b1101: perclk_root
            pub const CLKO1_SEL_13: u32 = 0b1101;

            /// 0b1110: ckil_sync_clk_root
            pub const CLKO1_SEL_14: u32 = 0b1110;

            /// 0b1111: pll4_main_clk
            pub const CLKO1_SEL_15: u32 = 0b1111;
        }
    }

    /// Setting the divider of CCM_CLKO1
    pub mod CLKO1_DIV {
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

            /// 0b000: divide by 1
            pub const CLKO1_DIV_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const CLKO1_DIV_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const CLKO1_DIV_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const CLKO1_DIV_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const CLKO1_DIV_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const CLKO1_DIV_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const CLKO1_DIV_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const CLKO1_DIV_7: u32 = 0b111;
        }
    }

    /// Enable of CCM_CLKO1 clock
    pub mod CLKO1_EN {
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

            /// 0b0: CCM_CLKO1 disabled.
            pub const CLKO1_EN_0: u32 = 0b0;

            /// 0b1: CCM_CLKO1 enabled.
            pub const CLKO1_EN_1: u32 = 0b1;
        }
    }

    /// CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks
    pub mod CLK_OUT_SEL {
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

            /// 0b0: CCM_CLKO1 output drives CCM_CLKO1 clock
            pub const CLK_OUT_SEL_0: u32 = 0b0;

            /// 0b1: CCM_CLKO1 output drives CCM_CLKO2 clock
            pub const CLK_OUT_SEL_1: u32 = 0b1;
        }
    }

    /// Selection of the clock to be generated on CCM_CLKO2
    pub mod CLKO2_SEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00011: usdhc1_clk_root
            pub const CLKO2_SEL_3: u32 = 0b00011;

            /// 0b00110: lpi2c_clk_root
            pub const CLKO2_SEL_6: u32 = 0b00110;

            /// 0b01011: csi_clk_root
            pub const CLKO2_SEL_11: u32 = 0b01011;

            /// 0b01110: osc_clk
            pub const CLKO2_SEL_14: u32 = 0b01110;

            /// 0b10001: usdhc2_clk_root
            pub const CLKO2_SEL_17: u32 = 0b10001;

            /// 0b10010: sai1_clk_root
            pub const CLKO2_SEL_18: u32 = 0b10010;

            /// 0b10011: sai2_clk_root
            pub const CLKO2_SEL_19: u32 = 0b10011;

            /// 0b10100: sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)
            pub const CLKO2_SEL_20: u32 = 0b10100;

            /// 0b10111: can_clk_root (FlexCAN, shared with CANFD)
            pub const CLKO2_SEL_23: u32 = 0b10111;

            /// 0b11011: flexspi_clk_root
            pub const CLKO2_SEL_27: u32 = 0b11011;

            /// 0b11100: uart_clk_root
            pub const CLKO2_SEL_28: u32 = 0b11100;

            /// 0b11101: spdif0_clk_root
            pub const CLKO2_SEL_29: u32 = 0b11101;
        }
    }

    /// Setting the divider of CCM_CLKO2
    pub mod CLKO2_DIV {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: divide by 1
            pub const CLKO2_DIV_0: u32 = 0b000;

            /// 0b001: divide by 2
            pub const CLKO2_DIV_1: u32 = 0b001;

            /// 0b010: divide by 3
            pub const CLKO2_DIV_2: u32 = 0b010;

            /// 0b011: divide by 4
            pub const CLKO2_DIV_3: u32 = 0b011;

            /// 0b100: divide by 5
            pub const CLKO2_DIV_4: u32 = 0b100;

            /// 0b101: divide by 6
            pub const CLKO2_DIV_5: u32 = 0b101;

            /// 0b110: divide by 7
            pub const CLKO2_DIV_6: u32 = 0b110;

            /// 0b111: divide by 8
            pub const CLKO2_DIV_7: u32 = 0b111;
        }
    }

    /// Enable of CCM_CLKO2 clock
    pub mod CLKO2_EN {
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

            /// 0b0: CCM_CLKO2 disabled.
            pub const CLKO2_EN_0: u32 = 0b0;

            /// 0b1: CCM_CLKO2 enabled.
            pub const CLKO2_EN_1: u32 = 0b1;
        }
    }
}

/// CCM General Purpose Register
pub mod CGPR {

    /// Defines clock dividion of clock for stby_count (pmic delay counter)
    pub mod PMIC_DELAY_SCALER {
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

            /// 0b0: clock is not divided
            pub const PMIC_DELAY_SCALER_0: u32 = 0b0;

            /// 0b1: clock is divided /8
            pub const PMIC_DELAY_SCALER_1: u32 = 0b1;
        }
    }

    /// Defines the value of the output signal cgpr_dout\[4\]. Gate of program supply for efuse programing
    pub mod EFUSE_PROG_SUPPLY_GATE {
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

            /// 0b0: fuse programing supply voltage is gated off to the efuse module
            pub const EFUSE_PROG_SUPPLY_GATE_0: u32 = 0b0;

            /// 0b1: allow fuse programing.
            pub const EFUSE_PROG_SUPPLY_GATE_1: u32 = 0b1;
        }
    }

    /// System memory DS control
    pub mod SYS_MEM_DS_CTRL {
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

            /// 0b00: Disable memory DS mode always
            pub const SYS_MEM_DS_CTRL_0: u32 = 0b00;

            /// 0b01: Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled
            pub const SYS_MEM_DS_CTRL_1: u32 = 0b01;

            /// 0b00: enable memory (outside ARM platform) DS mode when system is in STOP mode
            pub const SYS_MEM_DS_CTRL_2: u32 = 0b00;
        }
    }

    /// Fast PLL enable.
    pub mod FPL {
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

            /// 0b0: Engage PLL enable default way.
            pub const FPL_0: u32 = 0b0;

            /// 0b1: Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode.
            pub const FPL_1: u32 = 0b1;
        }
    }

    /// Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal
    pub mod INT_MEM_CLK_LPM {
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

            /// 0b0: Disable the clock to the ARM platform memories when entering Low Power Mode
            pub const INT_MEM_CLK_LPM_0: u32 = 0b0;

            /// 0b1: Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)
            pub const INT_MEM_CLK_LPM_1: u32 = 0b1;
        }
    }
}

/// CCM Clock Gating Register 0
pub mod CCGR0 {

    /// aips_tz1 clocks (aips_tz1_clk_enable)
    pub mod CG0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// aips_tz2 clocks (aips_tz2_clk_enable)
    pub mod CG1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// mqs clock ( mqs_hmclk_clock_enable)
    pub mod CG2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// flexspi_exsc clock (flexspi_exsc_clk_enable)
    pub mod CG3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// sim_m or sim_main register access clock (sim_m_mainclk_r_enable)
    pub mod CG4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// dcp clock (dcp_clk_enable)
    pub mod CG5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// lpuart3 clock (lpuart3_clk_enable)
    pub mod CG6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// can1 clock (can1_clk_enable)
    pub mod CG7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// can1_serial clock (can1_serial_clk_enable)
    pub mod CG8 {
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

    /// can2 clock (can2_clk_enable)
    pub mod CG9 {
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

    /// can2_serial clock (can2_serial_clk_enable)
    pub mod CG10 {
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

    /// trace clock (trace_clk_enable)
    pub mod CG11 {
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

    /// gpt2 bus clocks (gpt2_bus_clk_enable)
    pub mod CG12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// gpt2 serial clocks (gpt2_serial_clk_enable)
    pub mod CG13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// lpuart2 clock (lpuart2_clk_enable)
    pub mod CG14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// gpio2_clocks (gpio2_clk_enable)
    pub mod CG15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCM Clock Gating Register 1
pub mod CCGR1 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 2
pub mod CCGR2 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 3
pub mod CCGR3 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 4
pub mod CCGR4 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 5
pub mod CCGR5 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 6
pub mod CCGR6 {
    pub use super::CCGR0::CG0;
    pub use super::CCGR0::CG1;
    pub use super::CCGR0::CG10;
    pub use super::CCGR0::CG11;
    pub use super::CCGR0::CG12;
    pub use super::CCGR0::CG13;
    pub use super::CCGR0::CG14;
    pub use super::CCGR0::CG15;
    pub use super::CCGR0::CG2;
    pub use super::CCGR0::CG3;
    pub use super::CCGR0::CG4;
    pub use super::CCGR0::CG5;
    pub use super::CCGR0::CG6;
    pub use super::CCGR0::CG7;
    pub use super::CCGR0::CG8;
    pub use super::CCGR0::CG9;
}

/// CCM Clock Gating Register 7
pub mod CCGR7 {

    /// enet2_clk_enable
    pub mod CG0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// flexspi2_clk_enable
    pub mod CG1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// axbs_l_clk_enable
    pub mod CG2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// can3_clk_enable
    pub mod CG3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// can3_serial_clk_enable
    pub mod CG4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// aips_lite_clk_enable
    pub mod CG5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// flexio3_clk_enable
    pub mod CG6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCM Module Enable Overide Register
pub mod CMEOR {

    /// Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'
    pub mod MOD_EN_OV_GPT {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_OV_GPT_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_OV_GPT_1: u32 = 0b1;
        }
    }

    /// Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'
    pub mod MOD_EN_OV_PIT {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_OV_PIT_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_OV_PIT_1: u32 = 0b1;
        }
    }

    /// overide clock enable signal from USDHC.
    pub mod MOD_EN_USDHC {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_USDHC_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_USDHC_1: u32 = 0b1;
        }
    }

    /// Overide clock enable signal from TRNG
    pub mod MOD_EN_OV_TRNG {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_OV_TRNG_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_OV_TRNG_1: u32 = 0b1;
        }
    }

    /// Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'
    pub mod MOD_EN_OV_CANFD_CPI {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_OV_CANFD_CPI_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_OV_CANFD_CPI_1: u32 = 0b1;
        }
    }

    /// Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'
    pub mod MOD_EN_OV_CAN2_CPI {
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

            /// 0b0: don't override module enable signal
            pub const MOD_EN_OV_CAN2_CPI_0: u32 = 0b0;

            /// 0b1: override module enable signal
            pub const MOD_EN_OV_CAN2_CPI_1: u32 = 0b1;
        }
    }

    /// Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'
    pub mod MOD_EN_OV_CAN1_CPI {
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

            /// 0b0: don't overide module enable signal
            pub const MOD_EN_OV_CAN1_CPI_0: u32 = 0b0;

            /// 0b1: overide module enable signal
            pub const MOD_EN_OV_CAN1_CPI_1: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    /// CCM Control Register
    pub CCR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// CCM Status Register
    pub CSR: RORegister<u32>,

    /// CCM Clock Switcher Register
    pub CCSR: RWRegister<u32>,

    /// CCM Arm Clock Root Register
    pub CACRR: RWRegister<u32>,

    /// CCM Bus Clock Divider Register
    pub CBCDR: RWRegister<u32>,

    /// CCM Bus Clock Multiplexer Register
    pub CBCMR: RWRegister<u32>,

    /// CCM Serial Clock Multiplexer Register 1
    pub CSCMR1: RWRegister<u32>,

    /// CCM Serial Clock Multiplexer Register 2
    pub CSCMR2: RWRegister<u32>,

    /// CCM Serial Clock Divider Register 1
    pub CSCDR1: RWRegister<u32>,

    /// CCM Clock Divider Register
    pub CS1CDR: RWRegister<u32>,

    /// CCM Clock Divider Register
    pub CS2CDR: RWRegister<u32>,

    /// CCM D1 Clock Divider Register
    pub CDCDR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// CCM Serial Clock Divider Register 2
    pub CSCDR2: RWRegister<u32>,

    /// CCM Serial Clock Divider Register 3
    pub CSCDR3: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// CCM Divider Handshake In-Process Register
    pub CDHIPR: RORegister<u32>,

    _reserved4: [u32; 2],

    /// CCM Low Power Control Register
    pub CLPCR: RWRegister<u32>,

    /// CCM Interrupt Status Register
    pub CISR: RWRegister<u32>,

    /// CCM Interrupt Mask Register
    pub CIMR: RWRegister<u32>,

    /// CCM Clock Output Source Register
    pub CCOSR: RWRegister<u32>,

    /// CCM General Purpose Register
    pub CGPR: RWRegister<u32>,

    /// CCM Clock Gating Register 0
    pub CCGR0: RWRegister<u32>,

    /// CCM Clock Gating Register 1
    pub CCGR1: RWRegister<u32>,

    /// CCM Clock Gating Register 2
    pub CCGR2: RWRegister<u32>,

    /// CCM Clock Gating Register 3
    pub CCGR3: RWRegister<u32>,

    /// CCM Clock Gating Register 4
    pub CCGR4: RWRegister<u32>,

    /// CCM Clock Gating Register 5
    pub CCGR5: RWRegister<u32>,

    /// CCM Clock Gating Register 6
    pub CCGR6: RWRegister<u32>,

    /// CCM Clock Gating Register 7
    pub CCGR7: RWRegister<u32>,

    /// CCM Module Enable Overide Register
    pub CMEOR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CCR: u32,
    pub CSR: u32,
    pub CCSR: u32,
    pub CACRR: u32,
    pub CBCDR: u32,
    pub CBCMR: u32,
    pub CSCMR1: u32,
    pub CSCMR2: u32,
    pub CSCDR1: u32,
    pub CS1CDR: u32,
    pub CS2CDR: u32,
    pub CDCDR: u32,
    pub CSCDR2: u32,
    pub CSCDR3: u32,
    pub CDHIPR: u32,
    pub CLPCR: u32,
    pub CISR: u32,
    pub CIMR: u32,
    pub CCOSR: u32,
    pub CGPR: u32,
    pub CCGR0: u32,
    pub CCGR1: u32,
    pub CCGR2: u32,
    pub CCGR3: u32,
    pub CCGR4: u32,
    pub CCGR5: u32,
    pub CCGR6: u32,
    pub CCGR7: u32,
    pub CMEOR: u32,
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
