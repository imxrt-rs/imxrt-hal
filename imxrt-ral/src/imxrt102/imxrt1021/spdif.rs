#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SPDIF

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SPDIF Configuration Register
pub mod SCR {

    /// no description available
    pub mod USrc_Sel {
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

            /// 0b00: No embedded U channel
            pub const USrc_Sel_0: u32 = 0b00;

            /// 0b01: U channel from SPDIF receive block (CD mode)
            pub const USrc_Sel_1: u32 = 0b01;

            /// 0b11: U channel from on chip transmitter
            pub const USrc_Sel_3: u32 = 0b11;
        }
    }

    /// no description available
    pub mod TxSel {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Off and output 0
            pub const TxSel_0: u32 = 0b000;

            /// 0b001: Feed-through SPDIFIN
            pub const TxSel_1: u32 = 0b001;

            /// 0b101: Tx Normal operation
            pub const TxSel_5: u32 = 0b101;
        }
    }

    /// no description available
    pub mod ValCtrl {
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

            /// 0b0: Outgoing Validity always set
            pub const ValCtrl_0: u32 = 0b0;

            /// 0b1: Outgoing Validity always clear
            pub const ValCtrl_1: u32 = 0b1;
        }
    }

    /// DMA Transmit Request Enable (Tx FIFO empty)
    pub mod DMA_TX_En {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Receive Request Enable (RX FIFO full)
    pub mod DMA_Rx_En {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// no description available
    pub mod TxFIFO_Ctrl {
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

            /// 0b00: Send out digital zero on SPDIF Tx
            pub const TxFIFO_Ctrl_0: u32 = 0b00;

            /// 0b01: Tx Normal operation
            pub const TxFIFO_Ctrl_1: u32 = 0b01;

            /// 0b10: Reset to 1 sample remaining
            pub const TxFIFO_Ctrl_2: u32 = 0b10;
        }
    }

    /// When write 1 to this bit, it will cause SPDIF software reset
    pub mod soft_reset {
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

    /// When write 1 to this bit, it will cause SPDIF enter low-power mode
    pub mod LOW_POWER {
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

    /// no description available
    pub mod TxFIFOEmpty_Sel {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (2 bits: 0b11 << 15)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Empty interrupt if 0 sample in Tx left and right FIFOs
            pub const TxFIFOEmpty_Sel_0: u32 = 0b00;

            /// 0b01: Empty interrupt if at most 4 sample in Tx left and right FIFOs
            pub const TxFIFOEmpty_Sel_1: u32 = 0b01;

            /// 0b10: Empty interrupt if at most 8 sample in Tx left and right FIFOs
            pub const TxFIFOEmpty_Sel_2: u32 = 0b10;

            /// 0b11: Empty interrupt if at most 12 sample in Tx left and right FIFOs
            pub const TxFIFOEmpty_Sel_3: u32 = 0b11;
        }
    }

    /// no description available
    pub mod TxAutoSync {
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

            /// 0b0: Tx FIFO auto sync off
            pub const TxAutoSync_0: u32 = 0b0;

            /// 0b1: Tx FIFO auto sync on
            pub const TxAutoSync_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod RxAutoSync {
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

            /// 0b0: Rx FIFO auto sync off
            pub const RxAutoSync_0: u32 = 0b0;

            /// 0b1: RxFIFO auto sync on
            pub const RxAutoSync_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod RxFIFOFull_Sel {
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

            /// 0b00: Full interrupt if at least 1 sample in Rx left and right FIFOs
            pub const RxFIFOFull_Sel_0: u32 = 0b00;

            /// 0b01: Full interrupt if at least 4 sample in Rx left and right FIFOs
            pub const RxFIFOFull_Sel_1: u32 = 0b01;

            /// 0b10: Full interrupt if at least 8 sample in Rx left and right FIFOs
            pub const RxFIFOFull_Sel_2: u32 = 0b10;

            /// 0b11: Full interrupt if at least 16 sample in Rx left and right FIFO
            pub const RxFIFOFull_Sel_3: u32 = 0b11;
        }
    }

    /// no description available
    pub mod RxFIFO_Rst {
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

            /// 0b0: Normal operation
            pub const RxFIFO_Rst_0: u32 = 0b0;

            /// 0b1: Reset register to 1 sample remaining
            pub const RxFIFO_Rst_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod RxFIFO_Off_On {
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

            /// 0b0: SPDIF Rx FIFO is on
            pub const RxFIFO_Off_On_0: u32 = 0b0;

            /// 0b1: SPDIF Rx FIFO is off. Does not accept data from interface
            pub const RxFIFO_Off_On_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod RxFIFO_Ctrl {
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

            /// 0b0: Normal operation
            pub const RxFIFO_Ctrl_0: u32 = 0b0;

            /// 0b1: Always read zero from Rx data register
            pub const RxFIFO_Ctrl_1: u32 = 0b1;
        }
    }
}

/// CDText Control Register
pub mod SRCD {

    /// no description available
    pub mod USyncMode {
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

            /// 0b0: Non-CD data
            pub const USyncMode_0: u32 = 0b0;

            /// 0b1: CD user channel subcode
            pub const USyncMode_1: u32 = 0b1;
        }
    }
}

/// PhaseConfig Register
pub mod SRPC {

    /// Gain selection:
    pub mod GainSel {
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

            /// 0b000: 24*(2**10)
            pub const GainSel_0: u32 = 0b000;

            /// 0b001: 16*(2**10)
            pub const GainSel_1: u32 = 0b001;

            /// 0b010: 12*(2**10)
            pub const GainSel_2: u32 = 0b010;

            /// 0b011: 8*(2**10)
            pub const GainSel_3: u32 = 0b011;

            /// 0b100: 6*(2**10)
            pub const GainSel_4: u32 = 0b100;

            /// 0b101: 4*(2**10)
            pub const GainSel_5: u32 = 0b101;

            /// 0b110: 3*(2**10)
            pub const GainSel_6: u32 = 0b110;
        }
    }

    /// LOCK bit to show that the internal DPLL is locked, read only
    pub mod LOCK {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock source selection, all other settings not shown are reserved:
    pub mod ClkSrc_Sel {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)
            pub const ClkSrc_Sel_0: u32 = 0b0000;

            /// 0b0001: if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)
            pub const ClkSrc_Sel_1: u32 = 0b0001;

            /// 0b0011: if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK
            pub const ClkSrc_Sel_3: u32 = 0b0011;

            /// 0b0101: REF_CLK_32K (XTALOSC)
            pub const ClkSrc_Sel_5: u32 = 0b0101;

            /// 0b0110: tx_clk (SPDIF0_CLK_ROOT)
            pub const ClkSrc_Sel_6: u32 = 0b0110;

            /// 0b1000: SPDIF_EXT_CLK
            pub const ClkSrc_Sel_8: u32 = 0b1000;
        }
    }
}

/// InterruptEn Register
pub mod SIE {

    /// SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO.
    pub mod RxFIFOFul {
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

    /// SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO.
    pub mod TxEm {
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

    /// SPDIF receiver loss of lock
    pub mod LockLoss {
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

    /// Rx FIFO resync
    pub mod RxFIFOResyn {
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

    /// Rx FIFO underrun/overrun
    pub mod RxFIFOUnOv {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// U/Q Channel framing error
    pub mod UQErr {
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

    /// U/Q Channel sync found
    pub mod UQSync {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Q Channel receive register overrun
    pub mod QRxOv {
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

    /// Q Channel receive register full, can't be cleared with reg
    pub mod QRxFul {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// U Channel receive register overrun
    pub mod URxOv {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// U Channel receive register full, can't be cleared with reg
    pub mod URxFul {
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

    /// SPDIF receiver found parity bit error
    pub mod BitErr {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPDIF receiver found illegal symbol
    pub mod SymErr {
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

    /// SPDIF validity flag no good
    pub mod ValNoGood {
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

    /// SPDIF receive change in value of control channel
    pub mod CNew {
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

    /// SPDIF Tx FIFO resync
    pub mod TxResyn {
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

    /// SPDIF Tx FIFO under/overrun
    pub mod TxUnOv {
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

    /// SPDIF receiver's DPLL is locked
    pub mod Lock {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SIC and SIS
/// SIC: InterruptClear Register
/// SIS: InterruptStat Register
pub mod SI {

    /// SPDIF receiver loss of lock
    pub mod LockLoss {
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

    /// Rx FIFO resync
    pub mod RxFIFOResyn {
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

    /// Rx FIFO underrun/overrun
    pub mod RxFIFOUnOv {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// U/Q Channel framing error
    pub mod UQErr {
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

    /// U/Q Channel sync found
    pub mod UQSync {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Q Channel receive register overrun
    pub mod QRxOv {
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

    /// U Channel receive register overrun
    pub mod URxOv {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPDIF receiver found parity bit error
    pub mod BitErr {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPDIF receiver found illegal symbol
    pub mod SymErr {
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

    /// SPDIF validity flag no good
    pub mod ValNoGood {
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

    /// SPDIF receive change in value of control channel
    pub mod CNew {
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

    /// SPDIF Tx FIFO resync
    pub mod TxResyn {
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

    /// SPDIF Tx FIFO under/overrun
    pub mod TxUnOv {
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

    /// SPDIF receiver's DPLL is locked
    pub mod Lock {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPDIF Rx FIFO full, can't be cleared with reg. IntClear. To clear it, read from Rx FIFO.
    pub mod RxFIFOFul {
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

    /// SPDIF Tx FIFO empty, can't be cleared with reg. IntClear. To clear it, write toTx FIFO.
    pub mod TxEm {
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

    /// Q Channel receive register full, can't be cleared with reg
    pub mod QRxFul {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// U Channel receive register full, can't be cleared with reg
    pub mod URxFul {
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
}

/// SPDIFRxLeft Register
pub mod SRL {

    /// Processor receive SPDIF data left
    pub mod RxDataLeft {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxRight Register
pub mod SRR {

    /// Processor receive SPDIF data right
    pub mod RxDataRight {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxCChannel_h Register
pub mod SRCSH {

    /// SPDIF receive C channel register, contains first 24 bits of C channel without interpretation
    pub mod RxCChannel_h {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFRxCChannel_l Register
pub mod SRCSL {

    /// SPDIF receive C channel register, contains next 24 bits of C channel without interpretation
    pub mod RxCChannel_l {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UchannelRx Register
pub mod SRU {

    /// SPDIF receive U channel register, contains next 3 U channel bytes
    pub mod RxUChannel {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// QchannelRx Register
pub mod SRQ {

    /// SPDIF receive Q channel register, contains next 3 Q channel bytes
    pub mod RxQChannel {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxLeft Register
pub mod STL {

    /// SPDIF transmit left channel data. It is write-only, and always returns zeros when read
    pub mod TxDataLeft {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxRight Register
pub mod STR {

    /// SPDIF transmit right channel data. It is write-only, and always returns zeros when read
    pub mod TxDataRight {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxCChannelCons_h Register
pub mod STCSCH {

    /// SPDIF transmit Cons
    pub mod TxCChannelCons_h {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxCChannelCons_l Register
pub mod STCSCL {

    /// SPDIF transmit Cons
    pub mod TxCChannelCons_l {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FreqMeas Register
pub mod SRFM {

    /// Frequency measurement data
    pub mod FreqMeas {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SPDIFTxClk Register
pub mod STC {

    /// Divider factor (1-128)
    pub mod TxClk_DF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000: divider factor is 1
            pub const TxClk_DF_0: u32 = 0b0000000;

            /// 0b0000001: divider factor is 2
            pub const TxClk_DF_1: u32 = 0b0000001;

            /// 0b1111111: divider factor is 128
            pub const TxClk_DF_127: u32 = 0b1111111;
        }
    }

    /// Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1.
    pub mod tx_all_clk_en {
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

            /// 0b0: disable transfer clock.
            pub const tx_all_clk_en_0: u32 = 0b0;

            /// 0b1: enable transfer clock.
            pub const tx_all_clk_en_1: u32 = 0b1;
        }
    }

    /// no description available
    pub mod TxClk_Source {
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

            /// 0b000: REF_CLK_32K input (XTALOSC 32 kHz clock)
            pub const TxClk_Source_0: u32 = 0b000;

            /// 0b001: tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)
            pub const TxClk_Source_1: u32 = 0b001;

            /// 0b011: SPDIF_EXT_CLK, from pads
            pub const TxClk_Source_3: u32 = 0b011;

            /// 0b101: ipg_clk input (frequency divided)
            pub const TxClk_Source_5: u32 = 0b101;
        }
    }

    /// system clock divider factor, 2~512.
    pub mod SYSCLK_DF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (9 bits: 0x1ff << 11)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000: no clock signal
            pub const SYSCLK_DF_0: u32 = 0b000000000;

            /// 0b000000001: divider factor is 2
            pub const SYSCLK_DF_1: u32 = 0b000000001;

            /// 0b111111111: divider factor is 512
            pub const SYSCLK_DF_511: u32 = 0b111111111;
        }
    }
}
pub struct RegisterBlock {
    /// SPDIF Configuration Register
    pub SCR: RWRegister<u32>,

    /// CDText Control Register
    pub SRCD: RWRegister<u32>,

    /// PhaseConfig Register
    pub SRPC: RWRegister<u32>,

    /// InterruptEn Register
    pub SIE: RWRegister<u32>,

    /// SIC and SIS
    /// SIC: InterruptClear Register
    /// SIS: InterruptStat Register
    pub SI: RWRegister<u32>,

    /// SPDIFRxLeft Register
    pub SRL: RORegister<u32>,

    /// SPDIFRxRight Register
    pub SRR: RORegister<u32>,

    /// SPDIFRxCChannel_h Register
    pub SRCSH: RORegister<u32>,

    /// SPDIFRxCChannel_l Register
    pub SRCSL: RORegister<u32>,

    /// UchannelRx Register
    pub SRU: RORegister<u32>,

    /// QchannelRx Register
    pub SRQ: RORegister<u32>,

    /// SPDIFTxLeft Register
    pub STL: RWRegister<u32>,

    /// SPDIFTxRight Register
    pub STR: RWRegister<u32>,

    /// SPDIFTxCChannelCons_h Register
    pub STCSCH: RWRegister<u32>,

    /// SPDIFTxCChannelCons_l Register
    pub STCSCL: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// FreqMeas Register
    pub SRFM: RORegister<u32>,

    _reserved2: [u32; 2],

    /// SPDIFTxClk Register
    pub STC: RWRegister<u32>,
}
pub struct ResetValues {
    pub SCR: u32,
    pub SRCD: u32,
    pub SRPC: u32,
    pub SIE: u32,
    pub SI: u32,
    pub SRL: u32,
    pub SRR: u32,
    pub SRCSH: u32,
    pub SRCSL: u32,
    pub SRU: u32,
    pub SRQ: u32,
    pub STL: u32,
    pub STR: u32,
    pub STCSCH: u32,
    pub STCSCL: u32,
    pub SRFM: u32,
    pub STC: u32,
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

/// Access functions for the SPDIF peripheral instance
pub mod SPDIF {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40380000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPDIF
    pub const reset: ResetValues = ResetValues {
        SCR: 0x00000400,
        SRCD: 0x00000000,
        SRPC: 0x00000000,
        SIE: 0x00000000,
        SI: 0x00000002,
        SRL: 0x00000000,
        SRR: 0x00000000,
        SRCSH: 0x00000000,
        SRCSL: 0x00000000,
        SRU: 0x00000000,
        SRQ: 0x00000000,
        STL: 0x00000000,
        STR: 0x00000000,
        STCSCH: 0x00000000,
        STCSCL: 0x00000000,
        SRFM: 0x00000000,
        STC: 0x00020F00,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPDIF_TAKEN: bool = false;

    /// Safe access to SPDIF
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
            if SPDIF_TAKEN {
                None
            } else {
                SPDIF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPDIF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPDIF_TAKEN && inst.addr == INSTANCE.addr {
                SPDIF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPDIF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPDIF_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPDIF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPDIF: *const RegisterBlock = 0x40380000 as *const _;
