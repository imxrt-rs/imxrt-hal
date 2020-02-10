#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! I2S

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Version ID Register
pub mod VERID {

    /// Feature Specification Number
    pub mod FEATURE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Standard feature set.
            pub const FEATURE_0: u32 = 0b0000000000000000;
        }
    }

    /// Minor Version Number
    pub mod MINOR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Major Version Number
    pub mod MAJOR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Parameter Register
pub mod PARAM {

    /// Number of Datalines
    pub mod DATALINE {
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

    /// FIFO Size
    pub mod FIFO {
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

    /// Frame Size
    pub mod FRAME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI Transmit Control Register
pub mod TCSR {

    /// FIFO Request DMA Enable
    pub mod FRDE {
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

            /// 0b0: Disables the DMA request.
            pub const FRDE_0: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const FRDE_1: u32 = 0b1;
        }
    }

    /// FIFO Warning DMA Enable
    pub mod FWDE {
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

            /// 0b0: Disables the DMA request.
            pub const FWDE_0: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const FWDE_1: u32 = 0b1;
        }
    }

    /// FIFO Request Interrupt Enable
    pub mod FRIE {
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

            /// 0b0: Disables the interrupt.
            pub const FRIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FRIE_1: u32 = 0b1;
        }
    }

    /// FIFO Warning Interrupt Enable
    pub mod FWIE {
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

            /// 0b0: Disables the interrupt.
            pub const FWIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FWIE_1: u32 = 0b1;
        }
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: Disables the interrupt.
            pub const FEIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FEIE_1: u32 = 0b1;
        }
    }

    /// Sync Error Interrupt Enable
    pub mod SEIE {
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

            /// 0b0: Disables interrupt.
            pub const SEIE_0: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const SEIE_1: u32 = 0b1;
        }
    }

    /// Word Start Interrupt Enable
    pub mod WSIE {
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

            /// 0b0: Disables interrupt.
            pub const WSIE_0: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const WSIE_1: u32 = 0b1;
        }
    }

    /// FIFO Request Flag
    pub mod FRF {
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

            /// 0b0: Transmit FIFO watermark has not been reached.
            pub const FRF_0: u32 = 0b0;

            /// 0b1: Transmit FIFO watermark has been reached.
            pub const FRF_1: u32 = 0b1;
        }
    }

    /// FIFO Warning Flag
    pub mod FWF {
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

            /// 0b0: No enabled transmit FIFO is empty.
            pub const FWF_0: u32 = 0b0;

            /// 0b1: Enabled transmit FIFO is empty.
            pub const FWF_1: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: Transmit underrun not detected.
            pub const FEF_0: u32 = 0b0;

            /// 0b1: Transmit underrun detected.
            pub const FEF_1: u32 = 0b1;
        }
    }

    /// Sync Error Flag
    pub mod SEF {
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

            /// 0b0: Sync error not detected.
            pub const SEF_0: u32 = 0b0;

            /// 0b1: Frame sync error detected.
            pub const SEF_1: u32 = 0b1;
        }
    }

    /// Word Start Flag
    pub mod WSF {
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

            /// 0b0: Start of word not detected.
            pub const WSF_0: u32 = 0b0;

            /// 0b1: Start of word detected.
            pub const WSF_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SR {
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

            /// 0b0: No effect.
            pub const SR_0: u32 = 0b0;

            /// 0b1: Software reset.
            pub const SR_1: u32 = 0b1;
        }
    }

    /// FIFO Reset
    pub mod FR {
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

            /// 0b0: No effect.
            pub const FR_0: u32 = 0b0;

            /// 0b1: FIFO reset.
            pub const FR_1: u32 = 0b1;
        }
    }

    /// Bit Clock Enable
    pub mod BCE {
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

            /// 0b0: Transmit bit clock is disabled.
            pub const BCE_0: u32 = 0b0;

            /// 0b1: Transmit bit clock is enabled.
            pub const BCE_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGE {
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

            /// 0b0: Transmitter is disabled in Debug mode, after completing the current frame.
            pub const DBGE_0: u32 = 0b0;

            /// 0b1: Transmitter is enabled in Debug mode.
            pub const DBGE_1: u32 = 0b1;
        }
    }

    /// Stop Enable
    pub mod STOPE {
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

            /// 0b0: Transmitter disabled in Stop mode.
            pub const STOPE_0: u32 = 0b0;

            /// 0b1: Transmitter enabled in Stop mode.
            pub const STOPE_1: u32 = 0b1;
        }
    }

    /// Transmitter Enable
    pub mod TE {
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

            /// 0b0: Transmitter is disabled.
            pub const TE_0: u32 = 0b0;

            /// 0b1: Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame.
            pub const TE_1: u32 = 0b1;
        }
    }
}

/// SAI Transmit Configuration 1 Register
pub mod TCR1 {

    /// Transmit FIFO Watermark
    pub mod TFW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI Transmit Configuration 2 Register
pub mod TCR2 {

    /// Bit Clock Divide
    pub mod DIV {
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

    /// Bit Clock Direction
    pub mod BCD {
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

            /// 0b0: Bit clock is generated externally in Slave mode.
            pub const BCD_0: u32 = 0b0;

            /// 0b1: Bit clock is generated internally in Master mode.
            pub const BCD_1: u32 = 0b1;
        }
    }

    /// Bit Clock Polarity
    pub mod BCP {
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

            /// 0b0: Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge.
            pub const BCP_0: u32 = 0b0;

            /// 0b1: Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge.
            pub const BCP_1: u32 = 0b1;
        }
    }

    /// MCLK Select
    pub mod MSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bus Clock selected.
            pub const MSEL_0: u32 = 0b00;

            /// 0b01: Master Clock (MCLK) 1 option selected.
            pub const MSEL_1: u32 = 0b01;

            /// 0b10: Master Clock (MCLK) 2 option selected.
            pub const MSEL_2: u32 = 0b10;

            /// 0b11: Master Clock (MCLK) 3 option selected.
            pub const MSEL_3: u32 = 0b11;
        }
    }

    /// Bit Clock Input
    pub mod BCI {
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

            /// 0b0: No effect.
            pub const BCI_0: u32 = 0b0;

            /// 0b1: Internal logic is clocked as if bit clock was externally generated.
            pub const BCI_1: u32 = 0b1;
        }
    }

    /// Bit Clock Swap
    pub mod BCS {
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

            /// 0b0: Use the normal bit clock source.
            pub const BCS_0: u32 = 0b0;

            /// 0b1: Swap the bit clock source.
            pub const BCS_1: u32 = 0b1;
        }
    }

    /// Synchronous Mode
    pub mod SYNC {
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

            /// 0b00: Asynchronous mode.
            pub const SYNC_0: u32 = 0b00;

            /// 0b01: Synchronous with receiver.
            pub const SYNC_1: u32 = 0b01;
        }
    }
}

/// SAI Transmit Configuration 3 Register
pub mod TCR3 {

    /// Word Flag Configuration
    pub mod WDFL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit Channel Enable
    pub mod TCE {
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

    /// Channel FIFO Reset
    pub mod CFR {
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
}

/// SAI Transmit Configuration 4 Register
pub mod TCR4 {

    /// Frame Sync Direction
    pub mod FSD {
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

            /// 0b0: Frame sync is generated externally in Slave mode.
            pub const FSD_0: u32 = 0b0;

            /// 0b1: Frame sync is generated internally in Master mode.
            pub const FSD_1: u32 = 0b1;
        }
    }

    /// Frame Sync Polarity
    pub mod FSP {
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

            /// 0b0: Frame sync is active high.
            pub const FSP_0: u32 = 0b0;

            /// 0b1: Frame sync is active low.
            pub const FSP_1: u32 = 0b1;
        }
    }

    /// On Demand Mode
    pub mod ONDEM {
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

            /// 0b0: Internal frame sync is generated continuously.
            pub const ONDEM_0: u32 = 0b0;

            /// 0b1: Internal frame sync is generated when the FIFO warning flag is clear.
            pub const ONDEM_1: u32 = 0b1;
        }
    }

    /// Frame Sync Early
    pub mod FSE {
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

            /// 0b0: Frame sync asserts with the first bit of the frame.
            pub const FSE_0: u32 = 0b0;

            /// 0b1: Frame sync asserts one bit before the first bit of the frame.
            pub const FSE_1: u32 = 0b1;
        }
    }

    /// MSB First
    pub mod MF {
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

            /// 0b0: LSB is transmitted first.
            pub const MF_0: u32 = 0b0;

            /// 0b1: MSB is transmitted first.
            pub const MF_1: u32 = 0b1;
        }
    }

    /// Channel Mode
    pub mod CHMOD {
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

            /// 0b0: TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled.
            pub const CHMOD_0: u32 = 0b0;

            /// 0b1: Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled.
            pub const CHMOD_1: u32 = 0b1;
        }
    }

    /// Sync Width
    pub mod SYWD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame size
    pub mod FRSZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO Packing Mode
    pub mod FPACK {
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

            /// 0b00: FIFO packing is disabled
            pub const FPACK_0: u32 = 0b00;

            /// 0b10: 8-bit FIFO packing is enabled
            pub const FPACK_2: u32 = 0b10;

            /// 0b11: 16-bit FIFO packing is enabled
            pub const FPACK_3: u32 = 0b11;
        }
    }

    /// FIFO Combine Mode
    pub mod FCOMB {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: FIFO combine mode disabled.
            pub const FCOMB_0: u32 = 0b00;

            /// 0b01: FIFO combine mode enabled on FIFO reads (from transmit shift registers).
            pub const FCOMB_1: u32 = 0b01;

            /// 0b10: FIFO combine mode enabled on FIFO writes (by software).
            pub const FCOMB_2: u32 = 0b10;

            /// 0b11: FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software).
            pub const FCOMB_3: u32 = 0b11;
        }
    }

    /// FIFO Continue on Error
    pub mod FCONT {
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

            /// 0b0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared.
            pub const FCONT_0: u32 = 0b0;

            /// 0b1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared.
            pub const FCONT_1: u32 = 0b1;
        }
    }
}

/// SAI Transmit Configuration 5 Register
pub mod TCR5 {

    /// First Bit Shifted
    pub mod FBT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Word 0 Width
    pub mod W0W {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Word N Width
    pub mod WNW {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI Transmit Data Register
pub mod TDR0 {

    /// Transmit Data Register
    pub mod TDR {
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

/// SAI Transmit Data Register
pub mod TDR1 {
    pub use super::TDR0::TDR;
}

/// SAI Transmit FIFO Register
pub mod TFR0 {

    /// Read FIFO Pointer
    pub mod RFP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write FIFO Pointer
    pub mod WFP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write Channel Pointer
    pub mod WCP {
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

            /// 0b0: No effect.
            pub const WCP_0: u32 = 0b0;

            /// 0b1: FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write.
            pub const WCP_1: u32 = 0b1;
        }
    }
}

/// SAI Transmit FIFO Register
pub mod TFR1 {
    pub use super::TFR0::RFP;
    pub use super::TFR0::WCP;
    pub use super::TFR0::WFP;
}

/// SAI Transmit Mask Register
pub mod TMR {

    /// Transmit Word Mask
    pub mod TWM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Word N is enabled.
            pub const TWM_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Word N is masked. The transmit data pins are tri-stated or drive zero when masked.
            pub const TWM_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// SAI Receive Control Register
pub mod RCSR {

    /// FIFO Request DMA Enable
    pub mod FRDE {
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

            /// 0b0: Disables the DMA request.
            pub const FRDE_0: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const FRDE_1: u32 = 0b1;
        }
    }

    /// FIFO Warning DMA Enable
    pub mod FWDE {
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

            /// 0b0: Disables the DMA request.
            pub const FWDE_0: u32 = 0b0;

            /// 0b1: Enables the DMA request.
            pub const FWDE_1: u32 = 0b1;
        }
    }

    /// FIFO Request Interrupt Enable
    pub mod FRIE {
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

            /// 0b0: Disables the interrupt.
            pub const FRIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FRIE_1: u32 = 0b1;
        }
    }

    /// FIFO Warning Interrupt Enable
    pub mod FWIE {
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

            /// 0b0: Disables the interrupt.
            pub const FWIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FWIE_1: u32 = 0b1;
        }
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: Disables the interrupt.
            pub const FEIE_0: u32 = 0b0;

            /// 0b1: Enables the interrupt.
            pub const FEIE_1: u32 = 0b1;
        }
    }

    /// Sync Error Interrupt Enable
    pub mod SEIE {
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

            /// 0b0: Disables interrupt.
            pub const SEIE_0: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const SEIE_1: u32 = 0b1;
        }
    }

    /// Word Start Interrupt Enable
    pub mod WSIE {
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

            /// 0b0: Disables interrupt.
            pub const WSIE_0: u32 = 0b0;

            /// 0b1: Enables interrupt.
            pub const WSIE_1: u32 = 0b1;
        }
    }

    /// FIFO Request Flag
    pub mod FRF {
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

            /// 0b0: Receive FIFO watermark not reached.
            pub const FRF_0: u32 = 0b0;

            /// 0b1: Receive FIFO watermark has been reached.
            pub const FRF_1: u32 = 0b1;
        }
    }

    /// FIFO Warning Flag
    pub mod FWF {
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

            /// 0b0: No enabled receive FIFO is full.
            pub const FWF_0: u32 = 0b0;

            /// 0b1: Enabled receive FIFO is full.
            pub const FWF_1: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: Receive overflow not detected.
            pub const FEF_0: u32 = 0b0;

            /// 0b1: Receive overflow detected.
            pub const FEF_1: u32 = 0b1;
        }
    }

    /// Sync Error Flag
    pub mod SEF {
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

            /// 0b0: Sync error not detected.
            pub const SEF_0: u32 = 0b0;

            /// 0b1: Frame sync error detected.
            pub const SEF_1: u32 = 0b1;
        }
    }

    /// Word Start Flag
    pub mod WSF {
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

            /// 0b0: Start of word not detected.
            pub const WSF_0: u32 = 0b0;

            /// 0b1: Start of word detected.
            pub const WSF_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SR {
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

            /// 0b0: No effect.
            pub const SR_0: u32 = 0b0;

            /// 0b1: Software reset.
            pub const SR_1: u32 = 0b1;
        }
    }

    /// FIFO Reset
    pub mod FR {
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

            /// 0b0: No effect.
            pub const FR_0: u32 = 0b0;

            /// 0b1: FIFO reset.
            pub const FR_1: u32 = 0b1;
        }
    }

    /// Bit Clock Enable
    pub mod BCE {
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

            /// 0b0: Receive bit clock is disabled.
            pub const BCE_0: u32 = 0b0;

            /// 0b1: Receive bit clock is enabled.
            pub const BCE_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGE {
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

            /// 0b0: Receiver is disabled in Debug mode, after completing the current frame.
            pub const DBGE_0: u32 = 0b0;

            /// 0b1: Receiver is enabled in Debug mode.
            pub const DBGE_1: u32 = 0b1;
        }
    }

    /// Stop Enable
    pub mod STOPE {
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

            /// 0b0: Receiver disabled in Stop mode.
            pub const STOPE_0: u32 = 0b0;

            /// 0b1: Receiver enabled in Stop mode.
            pub const STOPE_1: u32 = 0b1;
        }
    }

    /// Receiver Enable
    pub mod RE {
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

            /// 0b0: Receiver is disabled.
            pub const RE_0: u32 = 0b0;

            /// 0b1: Receiver is enabled, or receiver has been disabled and has not yet reached end of frame.
            pub const RE_1: u32 = 0b1;
        }
    }
}

/// SAI Receive Configuration 1 Register
pub mod RCR1 {

    /// Receive FIFO Watermark
    pub mod RFW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI Receive Configuration 2 Register
pub mod RCR2 {

    /// Bit Clock Divide
    pub mod DIV {
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

    /// Bit Clock Direction
    pub mod BCD {
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

            /// 0b0: Bit clock is generated externally in Slave mode.
            pub const BCD_0: u32 = 0b0;

            /// 0b1: Bit clock is generated internally in Master mode.
            pub const BCD_1: u32 = 0b1;
        }
    }

    /// Bit Clock Polarity
    pub mod BCP {
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

            /// 0b0: Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge.
            pub const BCP_0: u32 = 0b0;

            /// 0b1: Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge.
            pub const BCP_1: u32 = 0b1;
        }
    }

    /// MCLK Select
    pub mod MSEL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bus Clock selected.
            pub const MSEL_0: u32 = 0b00;

            /// 0b01: Master Clock (MCLK) 1 option selected.
            pub const MSEL_1: u32 = 0b01;

            /// 0b10: Master Clock (MCLK) 2 option selected.
            pub const MSEL_2: u32 = 0b10;

            /// 0b11: Master Clock (MCLK) 3 option selected.
            pub const MSEL_3: u32 = 0b11;
        }
    }

    /// Bit Clock Input
    pub mod BCI {
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

            /// 0b0: No effect.
            pub const BCI_0: u32 = 0b0;

            /// 0b1: Internal logic is clocked as if bit clock was externally generated.
            pub const BCI_1: u32 = 0b1;
        }
    }

    /// Bit Clock Swap
    pub mod BCS {
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

            /// 0b0: Use the normal bit clock source.
            pub const BCS_0: u32 = 0b0;

            /// 0b1: Swap the bit clock source.
            pub const BCS_1: u32 = 0b1;
        }
    }

    /// Synchronous Mode
    pub mod SYNC {
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

            /// 0b00: Asynchronous mode.
            pub const SYNC_0: u32 = 0b00;

            /// 0b01: Synchronous with transmitter.
            pub const SYNC_1: u32 = 0b01;
        }
    }
}

/// SAI Receive Configuration 3 Register
pub mod RCR3 {

    /// Word Flag Configuration
    pub mod WDFL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Channel Enable
    pub mod RCE {
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

    /// Channel FIFO Reset
    pub mod CFR {
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
}

/// SAI Receive Configuration 4 Register
pub mod RCR4 {

    /// Frame Sync Direction
    pub mod FSD {
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

            /// 0b0: Frame Sync is generated externally in Slave mode.
            pub const FSD_0: u32 = 0b0;

            /// 0b1: Frame Sync is generated internally in Master mode.
            pub const FSD_1: u32 = 0b1;
        }
    }

    /// Frame Sync Polarity
    pub mod FSP {
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

            /// 0b0: Frame sync is active high.
            pub const FSP_0: u32 = 0b0;

            /// 0b1: Frame sync is active low.
            pub const FSP_1: u32 = 0b1;
        }
    }

    /// On Demand Mode
    pub mod ONDEM {
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

            /// 0b0: Internal frame sync is generated continuously.
            pub const ONDEM_0: u32 = 0b0;

            /// 0b1: Internal frame sync is generated when the FIFO warning flag is clear.
            pub const ONDEM_1: u32 = 0b1;
        }
    }

    /// Frame Sync Early
    pub mod FSE {
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

            /// 0b0: Frame sync asserts with the first bit of the frame.
            pub const FSE_0: u32 = 0b0;

            /// 0b1: Frame sync asserts one bit before the first bit of the frame.
            pub const FSE_1: u32 = 0b1;
        }
    }

    /// MSB First
    pub mod MF {
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

            /// 0b0: LSB is received first.
            pub const MF_0: u32 = 0b0;

            /// 0b1: MSB is received first.
            pub const MF_1: u32 = 0b1;
        }
    }

    /// Sync Width
    pub mod SYWD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame Size
    pub mod FRSZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO Packing Mode
    pub mod FPACK {
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

            /// 0b00: FIFO packing is disabled
            pub const FPACK_0: u32 = 0b00;

            /// 0b10: 8-bit FIFO packing is enabled
            pub const FPACK_2: u32 = 0b10;

            /// 0b11: 16-bit FIFO packing is enabled
            pub const FPACK_3: u32 = 0b11;
        }
    }

    /// FIFO Combine Mode
    pub mod FCOMB {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: FIFO combine mode disabled.
            pub const FCOMB_0: u32 = 0b00;

            /// 0b01: FIFO combine mode enabled on FIFO writes (from receive shift registers).
            pub const FCOMB_1: u32 = 0b01;

            /// 0b10: FIFO combine mode enabled on FIFO reads (by software).
            pub const FCOMB_2: u32 = 0b10;

            /// 0b11: FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software).
            pub const FCOMB_3: u32 = 0b11;
        }
    }

    /// FIFO Continue on Error
    pub mod FCONT {
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

            /// 0b0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared.
            pub const FCONT_0: u32 = 0b0;

            /// 0b1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared.
            pub const FCONT_1: u32 = 0b1;
        }
    }
}

/// SAI Receive Configuration 5 Register
pub mod RCR5 {
    pub use super::TCR5::FBT;
    pub use super::TCR5::W0W;
    pub use super::TCR5::WNW;
}

/// SAI Receive Data Register
pub mod RDR0 {

    /// Receive Data Register
    pub mod RDR {
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

/// SAI Receive Data Register
pub mod RDR1 {
    pub use super::RDR0::RDR;
}

/// SAI Receive FIFO Register
pub mod RFR0 {

    /// Read FIFO Pointer
    pub mod RFP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Channel Pointer
    pub mod RCP {
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

            /// 0b0: No effect.
            pub const RCP_0: u32 = 0b0;

            /// 0b1: FIFO combine is enabled for FIFO reads and this FIFO will be read on the next FIFO read.
            pub const RCP_1: u32 = 0b1;
        }
    }

    /// Write FIFO Pointer
    pub mod WFP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SAI Receive FIFO Register
pub mod RFR1 {
    pub use super::RFR0::RCP;
    pub use super::RFR0::RFP;
    pub use super::RFR0::WFP;
}

/// SAI Receive Mask Register
pub mod RMR {

    /// Receive Word Mask
    pub mod RWM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: Word N is enabled.
            pub const RWM_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: Word N is masked.
            pub const RWM_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// SAI Transmit Control Register
    pub TCSR: RWRegister<u32>,

    /// SAI Transmit Configuration 1 Register
    pub TCR1: RWRegister<u32>,

    /// SAI Transmit Configuration 2 Register
    pub TCR2: RWRegister<u32>,

    /// SAI Transmit Configuration 3 Register
    pub TCR3: RWRegister<u32>,

    /// SAI Transmit Configuration 4 Register
    pub TCR4: RWRegister<u32>,

    /// SAI Transmit Configuration 5 Register
    pub TCR5: RWRegister<u32>,

    /// SAI Transmit Data Register
    pub TDR0: RWRegister<u32>,

    /// SAI Transmit Data Register
    pub TDR1: RWRegister<u32>,

    _reserved1: [u32; 6],

    /// SAI Transmit FIFO Register
    pub TFR0: RORegister<u32>,

    /// SAI Transmit FIFO Register
    pub TFR1: RORegister<u32>,

    _reserved2: [u32; 6],

    /// SAI Transmit Mask Register
    pub TMR: RWRegister<u32>,

    _reserved3: [u32; 9],

    /// SAI Receive Control Register
    pub RCSR: RWRegister<u32>,

    /// SAI Receive Configuration 1 Register
    pub RCR1: RWRegister<u32>,

    /// SAI Receive Configuration 2 Register
    pub RCR2: RWRegister<u32>,

    /// SAI Receive Configuration 3 Register
    pub RCR3: RWRegister<u32>,

    /// SAI Receive Configuration 4 Register
    pub RCR4: RWRegister<u32>,

    /// SAI Receive Configuration 5 Register
    pub RCR5: RWRegister<u32>,

    /// SAI Receive Data Register
    pub RDR0: RORegister<u32>,

    /// SAI Receive Data Register
    pub RDR1: RORegister<u32>,

    _reserved4: [u32; 6],

    /// SAI Receive FIFO Register
    pub RFR0: RORegister<u32>,

    /// SAI Receive FIFO Register
    pub RFR1: RORegister<u32>,

    _reserved5: [u32; 6],

    /// SAI Receive Mask Register
    pub RMR: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub TCSR: u32,
    pub TCR1: u32,
    pub TCR2: u32,
    pub TCR3: u32,
    pub TCR4: u32,
    pub TCR5: u32,
    pub TDR0: u32,
    pub TDR1: u32,
    pub TFR0: u32,
    pub TFR1: u32,
    pub TMR: u32,
    pub RCSR: u32,
    pub RCR1: u32,
    pub RCR2: u32,
    pub RCR3: u32,
    pub RCR4: u32,
    pub RCR5: u32,
    pub RDR0: u32,
    pub RDR1: u32,
    pub RFR0: u32,
    pub RFR1: u32,
    pub RMR: u32,
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

/// Access functions for the SAI1 peripheral instance
pub mod SAI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401e0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050502,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI1_TAKEN: bool = false;

    /// Safe access to SAI1
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
            if SAI1_TAKEN {
                None
            } else {
                SAI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI1_TAKEN && inst.addr == INSTANCE.addr {
                SAI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI1: *const RegisterBlock = 0x401e0000 as *const _;

/// Access functions for the SAI3 peripheral instance
pub mod SAI3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401e8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x03000000,
        PARAM: 0x00050502,
        TCSR: 0x00000000,
        TCR1: 0x00000000,
        TCR2: 0x00000000,
        TCR3: 0x00000000,
        TCR4: 0x00000000,
        TCR5: 0x00000000,
        TDR0: 0x00000000,
        TDR1: 0x00000000,
        TFR0: 0x00000000,
        TFR1: 0x00000000,
        TMR: 0x00000000,
        RCSR: 0x00000000,
        RCR1: 0x00000000,
        RCR2: 0x00000000,
        RCR3: 0x00000000,
        RCR4: 0x00000000,
        RCR5: 0x00000000,
        RDR0: 0x00000000,
        RDR1: 0x00000000,
        RFR0: 0x00000000,
        RFR1: 0x00000000,
        RMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI3_TAKEN: bool = false;

    /// Safe access to SAI3
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
            if SAI3_TAKEN {
                None
            } else {
                SAI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI3_TAKEN && inst.addr == INSTANCE.addr {
                SAI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI3: *const RegisterBlock = 0x401e8000 as *const _;
