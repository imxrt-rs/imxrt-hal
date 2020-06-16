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

            /// 0b010: ccm.ssi3_clk_root
            pub const SAI1_MCLK1_SEL_2: u32 = 0b010;

            /// 0b011: iomux.sai1_ipg_clk_sai_mclk
            pub const SAI1_MCLK1_SEL_3: u32 = 0b011;

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

            /// 0b010: ccm.ssi3_clk_root
            pub const SAI1_MCLK2_SEL_2: u32 = 0b010;

            /// 0b011: iomux.sai1_ipg_clk_sai_mclk
            pub const SAI1_MCLK2_SEL_3: u32 = 0b011;

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

    /// AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority.
    pub mod AXBS_P_M0_HIGH_PRIORITY {
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

            /// 0b0: AXBS_P M0 master doesn't have high priority
            pub const AXBS_P_M0_HIGH_PRIORITY_0: u32 = 0b0;

            /// 0b1: AXBS_P M0 master has high priority
            pub const AXBS_P_M0_HIGH_PRIORITY_1: u32 = 0b1;
        }
    }

    /// AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority.
    pub mod AXBS_P_M1_HIGH_PRIORITY {
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

            /// 0b0: AXBS_P M1 master does not have high priority
            pub const AXBS_P_M1_HIGH_PRIORITY_0: u32 = 0b0;

            /// 0b1: AXBS_P M1 master has high priority
            pub const AXBS_P_M1_HIGH_PRIORITY_1: u32 = 0b1;
        }
    }

    /// Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration.
    pub mod AXBS_P_FORCE_ROUND_ROBIN {
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

            /// 0b0: AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings.
            pub const AXBS_P_FORCE_ROUND_ROBIN_0: u32 = 0b0;

            /// 0b1: AXBS_P masters are arbitored in round robin
            pub const AXBS_P_FORCE_ROUND_ROBIN_1: u32 = 0b1;
        }
    }

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

    /// Divider ratio control for mclk from hmclk
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

            /// 0b00000000: mclk frequency = hmclk frequency
            pub const MQS_CLK_DIV_0: u32 = 0b00000000;

            /// 0b00000001: mclk frequency = 1/2 * hmclk frequency
            pub const MQS_CLK_DIV_1: u32 = 0b00000001;

            /// 0b00000010: mclk frequency = 1/3 * hmclk frequency
            pub const MQS_CLK_DIV_2: u32 = 0b00000010;

            /// 0b11111111: mclk frequency = 1/256 * hmclk frequency
            pub const MQS_CLK_DIV_255: u32 = 0b11111111;
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
}

/// GPR3 General Purpose Register
pub mod GPR3 {

    /// Select 128-bit DCP key from 256-bit key from SNVS Master Key
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

            /// 0b0: Select \[127:0\] from SNVS Master Key as DCP key
            pub const DCP_KEY_SEL_0: u32 = 0b0;

            /// 0b1: Select \[255:128\] from SNVS Master Key as DCP key
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

    /// IOMUXC XBAR_INOUT2 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_2 {
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
            pub const IOMUXC_XBAR_DIR_SEL_2_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_2_1: u32 = 0b1;
        }
    }

    /// IOMUXC XBAR_INOUT3 function direction select
    pub mod IOMUXC_XBAR_DIR_SEL_3 {
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
            pub const IOMUXC_XBAR_DIR_SEL_3_0: u32 = 0b0;

            /// 0b1: XBAR_INOUT as output
            pub const IOMUXC_XBAR_DIR_SEL_3_1: u32 = 0b1;
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

            /// 0b0: Select key from SNVS Master Key.
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
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 25)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Field is not locked
            pub const LOCK_OCRAM_TZ_ADDR_0: u32 = 0b00000;

            /// 0b00001: Field is locked (read access only)
            pub const LOCK_OCRAM_TZ_ADDR_1: u32 = 0b00001;
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

    /// Lock CM7_INIT_VTOR field for changes
    pub mod LOCK_VTOR {
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

            /// 0b0: CM7_INIT_VTOR field is not locked.
            pub const LOCK_VTOR_0: u32 = 0b0;

            /// 0b1: CM7_INIT_VTOR field is locked (read access only).
            pub const LOCK_VTOR_1: u32 = 0b1;
        }
    }

    /// Vector table offset register out of reset
    pub mod CM7_INIT_VTOR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (25 bits: 0x1ffffff << 7)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
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

/// GPR26 General Purpose Register
pub mod GPR26 {

    /// Select GPIO1 or GPIO2
    pub mod GPIO_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR27 General Purpose Register
pub mod GPR27 {

    /// Start address of flexspi1
    pub mod FLEXSPI_REMAP_ADDR_START {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR28 General Purpose Register
pub mod GPR28 {

    /// End address of flexspi1
    pub mod FLEXSPI_REMAP_ADDR_END {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// GPR29 General Purpose Register
pub mod GPR29 {

    /// Offset address of flexspi1
    pub mod FLEXSPI_REMAP_ADDR_OFFSET {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
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

    /// GPR26 General Purpose Register
    pub GPR26: RWRegister<u32>,

    /// GPR27 General Purpose Register
    pub GPR27: RWRegister<u32>,

    /// GPR28 General Purpose Register
    pub GPR28: RWRegister<u32>,

    /// GPR29 General Purpose Register
    pub GPR29: RWRegister<u32>,
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
    pub GPR26: u32,
    pub GPR27: u32,
    pub GPR28: u32,
    pub GPR29: u32,
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
        GPR3: 0x00000FF0,
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
        GPR14: 0x00880000,
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
        GPR26: 0x00000000,
        GPR27: 0x00000000,
        GPR28: 0x00000000,
        GPR29: 0x00000000,
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
