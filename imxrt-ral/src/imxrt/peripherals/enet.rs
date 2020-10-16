#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet MAC-NET Core
//!
//! Used by: imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt Event Register
pub mod EIR {

    /// Timestamp Timer
    pub mod TS_TIMER {
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

    /// Transmit Timestamp Available
    pub mod TS_AVAIL {
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

    /// Node Wakeup Request Indication
    pub mod WAKEUP {
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

    /// Payload Receive Error
    pub mod PLR {
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

    /// Transmit FIFO Underrun
    pub mod UN {
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

    /// Collision Retry Limit
    pub mod RL {
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

    /// Late Collision
    pub mod LC {
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

    /// Ethernet Bus Error
    pub mod EBERR {
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

    /// MII Interrupt.
    pub mod MII {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Buffer Interrupt
    pub mod RXB {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Frame Interrupt
    pub mod RXF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit Buffer Interrupt
    pub mod TXB {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit Frame Interrupt
    pub mod TXF {
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

    /// Graceful Stop Complete
    pub mod GRA {
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

    /// Babbling Transmit Error
    pub mod BABT {
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

    /// Babbling Receive Error
    pub mod BABR {
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
}

/// Interrupt Mask Register
pub mod EIMR {

    /// TS_TIMER Interrupt Mask
    pub mod TS_TIMER {
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

    /// TS_AVAIL Interrupt Mask
    pub mod TS_AVAIL {
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

    /// WAKEUP Interrupt Mask
    pub mod WAKEUP {
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

    /// PLR Interrupt Mask
    pub mod PLR {
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

    /// UN Interrupt Mask
    pub mod UN {
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

    /// RL Interrupt Mask
    pub mod RL {
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

    /// LC Interrupt Mask
    pub mod LC {
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

    /// EBERR Interrupt Mask
    pub mod EBERR {
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

    /// MII Interrupt Mask
    pub mod MII {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXB Interrupt Mask
    pub mod RXB {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXF Interrupt Mask
    pub mod RXF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXB Interrupt Mask
    pub mod TXB {
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

            /// 0b0: The corresponding interrupt source is masked.
            pub const TXB_0: u32 = 0b0;

            /// 0b1: The corresponding interrupt source is not masked.
            pub const TXB_1: u32 = 0b1;
        }
    }

    /// TXF Interrupt Mask
    pub mod TXF {
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

            /// 0b0: The corresponding interrupt source is masked.
            pub const TXF_0: u32 = 0b0;

            /// 0b1: The corresponding interrupt source is not masked.
            pub const TXF_1: u32 = 0b1;
        }
    }

    /// GRA Interrupt Mask
    pub mod GRA {
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

            /// 0b0: The corresponding interrupt source is masked.
            pub const GRA_0: u32 = 0b0;

            /// 0b1: The corresponding interrupt source is not masked.
            pub const GRA_1: u32 = 0b1;
        }
    }

    /// BABT Interrupt Mask
    pub mod BABT {
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

            /// 0b0: The corresponding interrupt source is masked.
            pub const BABT_0: u32 = 0b0;

            /// 0b1: The corresponding interrupt source is not masked.
            pub const BABT_1: u32 = 0b1;
        }
    }

    /// BABR Interrupt Mask
    pub mod BABR {
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

            /// 0b0: The corresponding interrupt source is masked.
            pub const BABR_0: u32 = 0b0;

            /// 0b1: The corresponding interrupt source is not masked.
            pub const BABR_1: u32 = 0b1;
        }
    }
}

/// Receive Descriptor Active Register
pub mod RDAR {

    /// Receive Descriptor Active
    pub mod RDAR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Descriptor Active Register
pub mod TDAR {

    /// Transmit Descriptor Active
    pub mod TDAR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Ethernet Control Register
pub mod ECR {

    /// Ethernet MAC Reset
    pub mod RESET {
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

    /// Ethernet Enable
    pub mod ETHEREN {
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

            /// 0b0: Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame.
            pub const ETHEREN_0: u32 = 0b0;

            /// 0b1: MAC is enabled, and reception and transmission are possible.
            pub const ETHEREN_1: u32 = 0b1;
        }
    }

    /// Magic Packet Detection Enable
    pub mod MAGICEN {
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

            /// 0b0: Magic detection logic disabled.
            pub const MAGICEN_0: u32 = 0b0;

            /// 0b1: The MAC core detects magic packets and asserts EIR\[WAKEUP\] when a frame is detected.
            pub const MAGICEN_1: u32 = 0b1;
        }
    }

    /// Sleep Mode Enable
    pub mod SLEEP {
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

            /// 0b0: Normal operating mode.
            pub const SLEEP_0: u32 = 0b0;

            /// 0b1: Sleep mode.
            pub const SLEEP_1: u32 = 0b1;
        }
    }

    /// EN1588 Enable
    pub mod EN1588 {
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

            /// 0b0: Legacy FEC buffer descriptors and functions enabled.
            pub const EN1588_0: u32 = 0b0;

            /// 0b1: Enhanced frame time-stamping functions enabled.
            pub const EN1588_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGEN {
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

            /// 0b0: MAC continues operation in debug mode.
            pub const DBGEN_0: u32 = 0b0;

            /// 0b1: MAC enters hardware freeze mode when the processor is in debug mode.
            pub const DBGEN_1: u32 = 0b1;
        }
    }

    /// Descriptor Byte Swapping Enable
    pub mod DBSWP {
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

            /// 0b0: The buffer descriptor bytes are not swapped to support big-endian devices.
            pub const DBSWP_0: u32 = 0b0;

            /// 0b1: The buffer descriptor bytes are swapped to support little-endian devices.
            pub const DBSWP_1: u32 = 0b1;
        }
    }
}

/// MII Management Frame Register
pub mod MMFR {

    /// Management Frame Data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turn Around
    pub mod TA {
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

    /// Register Address
    pub mod RA {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (5 bits: 0b11111 << 18)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHY Address
    pub mod PA {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (5 bits: 0b11111 << 23)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Operation Code
    pub mod OP {
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

    /// Start Of Frame Delimiter
    pub mod ST {
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

/// MII Speed Control Register
pub mod MSCR {

    /// MII Speed
    pub mod MII_SPEED {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (6 bits: 0x3f << 1)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Disable Preamble
    pub mod DIS_PRE {
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

            /// 0b0: Preamble enabled.
            pub const DIS_PRE_0: u32 = 0b0;

            /// 0b1: Preamble (32 ones) is not prepended to the MII management frame.
            pub const DIS_PRE_1: u32 = 0b1;
        }
    }

    /// Hold time On MDIO Output
    pub mod HOLDTIME {
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

            /// 0b000: 1 internal module clock cycle
            pub const HOLDTIME_0: u32 = 0b000;

            /// 0b001: 2 internal module clock cycles
            pub const HOLDTIME_1: u32 = 0b001;

            /// 0b010: 3 internal module clock cycles
            pub const HOLDTIME_2: u32 = 0b010;

            /// 0b111: 8 internal module clock cycles
            pub const HOLDTIME_7: u32 = 0b111;
        }
    }
}

/// MIB Control Register
pub mod MIBC {

    /// MIB Clear
    pub mod MIB_CLEAR {
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

            /// 0b0: See note above.
            pub const MIB_CLEAR_0: u32 = 0b0;

            /// 0b1: All statistics counters are reset to 0.
            pub const MIB_CLEAR_1: u32 = 0b1;
        }
    }

    /// MIB Idle
    pub mod MIB_IDLE {
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

            /// 0b0: The MIB block is updating MIB counters.
            pub const MIB_IDLE_0: u32 = 0b0;

            /// 0b1: The MIB block is not currently updating any MIB counters.
            pub const MIB_IDLE_1: u32 = 0b1;
        }
    }

    /// Disable MIB Logic
    pub mod MIB_DIS {
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

            /// 0b0: MIB logic is enabled.
            pub const MIB_DIS_0: u32 = 0b0;

            /// 0b1: MIB logic is disabled. The MIB logic halts and does not update any MIB counters.
            pub const MIB_DIS_1: u32 = 0b1;
        }
    }
}

/// Receive Control Register
pub mod RCR {

    /// Internal Loopback
    pub mod LOOP {
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

            /// 0b0: Loopback disabled.
            pub const LOOP_0: u32 = 0b0;

            /// 0b1: Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared.
            pub const LOOP_1: u32 = 0b1;
        }
    }

    /// Disable Receive On Transmit
    pub mod DRT {
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

            /// 0b0: Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode.
            pub const DRT_0: u32 = 0b0;

            /// 0b1: Disable reception of frames while transmitting. (Normally used for half-duplex mode.)
            pub const DRT_1: u32 = 0b1;
        }
    }

    /// Media Independent Interface Mode
    pub mod MII_MODE {
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

            /// 0b1: MII or RMII mode, as indicated by the RMII_MODE field.
            pub const MII_MODE_1: u32 = 0b1;
        }
    }

    /// Promiscuous Mode
    pub mod PROM {
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

            /// 0b0: Disabled.
            pub const PROM_0: u32 = 0b0;

            /// 0b1: Enabled.
            pub const PROM_1: u32 = 0b1;
        }
    }

    /// Broadcast Frame Reject
    pub mod BC_REJ {
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

    /// Flow Control Enable
    pub mod FCE {
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

    /// RMII Mode Enable
    pub mod RMII_MODE {
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

            /// 0b0: MAC configured for MII mode.
            pub const RMII_MODE_0: u32 = 0b0;

            /// 0b1: MAC configured for RMII operation.
            pub const RMII_MODE_1: u32 = 0b1;
        }
    }

    /// Enables 10-Mbit/s mode of the RMII .
    pub mod RMII_10T {
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

            /// 0b0: 100-Mbit/s operation.
            pub const RMII_10T_0: u32 = 0b0;

            /// 0b1: 10-Mbit/s operation.
            pub const RMII_10T_1: u32 = 0b1;
        }
    }

    /// Enable Frame Padding Remove On Receive
    pub mod PADEN {
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

            /// 0b0: No padding is removed on receive by the MAC.
            pub const PADEN_0: u32 = 0b0;

            /// 0b1: Padding is removed from received frames.
            pub const PADEN_1: u32 = 0b1;
        }
    }

    /// Terminate/Forward Pause Frames
    pub mod PAUFWD {
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

            /// 0b0: Pause frames are terminated and discarded in the MAC.
            pub const PAUFWD_0: u32 = 0b0;

            /// 0b1: Pause frames are forwarded to the user application.
            pub const PAUFWD_1: u32 = 0b1;
        }
    }

    /// Terminate/Forward Received CRC
    pub mod CRCFWD {
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

            /// 0b0: The CRC field of received frames is transmitted to the user application.
            pub const CRCFWD_0: u32 = 0b0;

            /// 0b1: The CRC field is stripped from the frame.
            pub const CRCFWD_1: u32 = 0b1;
        }
    }

    /// MAC Control Frame Enable
    pub mod CFEN {
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

            /// 0b0: MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface.
            pub const CFEN_0: u32 = 0b0;

            /// 0b1: MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded.
            pub const CFEN_1: u32 = 0b1;
        }
    }

    /// Maximum Frame Length
    pub mod MAX_FL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Payload Length Check Disable
    pub mod NLC {
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

            /// 0b0: The payload length check is disabled.
            pub const NLC_0: u32 = 0b0;

            /// 0b1: The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\[PLR\] field.
            pub const NLC_1: u32 = 0b1;
        }
    }

    /// Graceful Receive Stopped
    pub mod GRS {
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

/// Transmit Control Register
pub mod TCR {

    /// Graceful Transmit Stop
    pub mod GTS {
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

    /// Full-Duplex Enable
    pub mod FDEN {
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

    /// Transmit Frame Control Pause
    pub mod TFC_PAUSE {
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

            /// 0b0: No PAUSE frame transmitted.
            pub const TFC_PAUSE_0: u32 = 0b0;

            /// 0b1: The MAC stops transmission of data frames after the current transmission is complete.
            pub const TFC_PAUSE_1: u32 = 0b1;
        }
    }

    /// Receive Frame Control Pause
    pub mod RFC_PAUSE {
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

    /// Source MAC Address Select On Transmit
    pub mod ADDSEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Node MAC address programmed on PADDR1/2 registers.
            pub const ADDSEL_0: u32 = 0b000;
        }
    }

    /// Set MAC Address On Transmit
    pub mod ADDINS {
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

            /// 0b0: The source MAC address is not modified by the MAC.
            pub const ADDINS_0: u32 = 0b0;

            /// 0b1: The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL.
            pub const ADDINS_1: u32 = 0b1;
        }
    }

    /// Forward Frame From Application With CRC
    pub mod CRCFWD {
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

            /// 0b0: TxBD\[TC\] controls whether the frame has a CRC from the application.
            pub const CRCFWD_0: u32 = 0b0;

            /// 0b1: The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application.
            pub const CRCFWD_1: u32 = 0b1;
        }
    }
}

/// Physical Address Lower Register
pub mod PALR {

    /// Pause Address
    pub mod PADDR1 {
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

/// Physical Address Upper Register
pub mod PAUR {

    /// Type Field In PAUSE Frames
    pub mod TYPE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bytes 4 (bits 31:24) and 5 (bits 23:16) of the 6-byte individual address used for exact match, and the source address field in PAUSE frames
    pub mod PADDR2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Opcode/Pause Duration Register
pub mod OPD {

    /// Pause Duration
    pub mod PAUSE_DUR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Opcode Field In PAUSE Frames
    pub mod OPCODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Interrupt Coalescing Register
pub mod TXIC {

    /// Interrupt coalescing timer threshold
    pub mod ICTT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt coalescing frame count threshold
    pub mod ICFT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt Coalescing Timer Clock Source Select
    pub mod ICCS {
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

            /// 0b0: Use MII/GMII TX clocks.
            pub const ICCS_0: u32 = 0b0;

            /// 0b1: Use ENET system clock.
            pub const ICCS_1: u32 = 0b1;
        }
    }

    /// Interrupt Coalescing Enable
    pub mod ICEN {
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

            /// 0b0: Disable Interrupt coalescing.
            pub const ICEN_0: u32 = 0b0;

            /// 0b1: Enable Interrupt coalescing.
            pub const ICEN_1: u32 = 0b1;
        }
    }
}

/// Receive Interrupt Coalescing Register
pub mod RXIC {
    pub use super::TXIC::ICCS;
    pub use super::TXIC::ICEN;
    pub use super::TXIC::ICFT;
    pub use super::TXIC::ICTT;
}

/// Descriptor Individual Upper Address Register
pub mod IAUR {

    /// Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address
    pub mod IADDR1 {
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

/// Descriptor Individual Lower Address Register
pub mod IALR {

    /// Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address
    pub mod IADDR2 {
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

/// Descriptor Group Upper Address Register
pub mod GAUR {

    /// Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address
    pub mod GADDR1 {
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

/// Descriptor Group Lower Address Register
pub mod GALR {

    /// Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address
    pub mod GADDR2 {
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

/// Transmit FIFO Watermark Register
pub mod TFWR {

    /// Transmit FIFO Write
    pub mod TFWR {
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

            /// 0b000000: 64 bytes written.
            pub const TFWR_0: u32 = 0b000000;

            /// 0b000001: 64 bytes written.
            pub const TFWR_1: u32 = 0b000001;

            /// 0b000010: 128 bytes written.
            pub const TFWR_2: u32 = 0b000010;

            /// 0b000011: 192 bytes written.
            pub const TFWR_3: u32 = 0b000011;

            /// 0b011111: 1984 bytes written.
            pub const TFWR_31: u32 = 0b011111;
        }
    }

    /// Store And Forward Enable
    pub mod STRFWD {
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

            /// 0b0: Reset. The transmission start threshold is programmed in TFWR\[TFWR\].
            pub const STRFWD_0: u32 = 0b0;

            /// 0b1: Enabled.
            pub const STRFWD_1: u32 = 0b1;
        }
    }
}

/// Receive Descriptor Ring Start Register
pub mod RDSR {

    /// Pointer to the beginning of the receive buffer descriptor queue.
    pub mod R_DES_START {
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

/// Transmit Buffer Descriptor Ring Start Register
pub mod TDSR {

    /// Pointer to the beginning of the transmit buffer descriptor queue.
    pub mod X_DES_START {
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

/// Maximum Receive Buffer Size Register
pub mod MRBR {

    /// Receive buffer size in bytes
    pub mod R_BUF_SIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (10 bits: 0x3ff << 4)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Receive FIFO Section Full Threshold
pub mod RSFL {

    /// Value Of Receive FIFO Section Full Threshold
    pub mod RX_SECTION_FULL {
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

/// Receive FIFO Section Empty Threshold
pub mod RSEM {

    /// Value Of The Receive FIFO Section Empty Threshold
    pub mod RX_SECTION_EMPTY {
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

    /// RX Status FIFO Section Empty Threshold
    pub mod STAT_SECTION_EMPTY {
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
}

/// Receive FIFO Almost Empty Threshold
pub mod RAEM {

    /// Value Of The Receive FIFO Almost Empty Threshold
    pub mod RX_ALMOST_EMPTY {
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

/// Receive FIFO Almost Full Threshold
pub mod RAFL {

    /// Value Of The Receive FIFO Almost Full Threshold
    pub mod RX_ALMOST_FULL {
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

/// Transmit FIFO Section Empty Threshold
pub mod TSEM {

    /// Value Of The Transmit FIFO Section Empty Threshold
    pub mod TX_SECTION_EMPTY {
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

/// Transmit FIFO Almost Empty Threshold
pub mod TAEM {

    /// Value of Transmit FIFO Almost Empty Threshold
    pub mod TX_ALMOST_EMPTY {
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

/// Transmit FIFO Almost Full Threshold
pub mod TAFL {

    /// Value Of The Transmit FIFO Almost Full Threshold
    pub mod TX_ALMOST_FULL {
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

/// Transmit Inter-Packet Gap
pub mod TIPG {

    /// Transmit Inter-Packet Gap
    pub mod IPG {
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

/// Frame Truncation Length
pub mod FTRL {

    /// Frame Truncation Length
    pub mod TRUNC_FL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmit Accelerator Function Configuration
pub mod TACC {

    /// TX FIFO Shift-16
    pub mod SHIFT16 {
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

            /// 0b0: Disabled.
            pub const SHIFT16_0: u32 = 0b0;

            /// 0b1: Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header.
            pub const SHIFT16_1: u32 = 0b1;
        }
    }

    /// Enables insertion of IP header checksum.
    pub mod IPCHK {
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

            /// 0b0: Checksum is not inserted.
            pub const IPCHK_0: u32 = 0b0;

            /// 0b1: If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified.
            pub const IPCHK_1: u32 = 0b1;
        }
    }

    /// Enables insertion of protocol checksum.
    pub mod PROCHK {
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

            /// 0b0: Checksum not inserted.
            pub const PROCHK_0: u32 = 0b0;

            /// 0b1: If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified.
            pub const PROCHK_1: u32 = 0b1;
        }
    }
}

/// Receive Accelerator Function Configuration
pub mod RACC {

    /// Enable Padding Removal For Short IP Frames
    pub mod PADREM {
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

            /// 0b0: Padding not removed.
            pub const PADREM_0: u32 = 0b0;

            /// 0b1: Any bytes following the IP payload section of the frame are removed from the frame.
            pub const PADREM_1: u32 = 0b1;
        }
    }

    /// Enable Discard Of Frames With Wrong IPv4 Header Checksum
    pub mod IPDIS {
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

            /// 0b0: Frames with wrong IPv4 header checksum are not discarded.
            pub const IPDIS_0: u32 = 0b0;

            /// 0b1: If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared).
            pub const IPDIS_1: u32 = 0b1;
        }
    }

    /// Enable Discard Of Frames With Wrong Protocol Checksum
    pub mod PRODIS {
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

            /// 0b0: Frames with wrong checksum are not discarded.
            pub const PRODIS_0: u32 = 0b0;

            /// 0b1: If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared).
            pub const PRODIS_1: u32 = 0b1;
        }
    }

    /// Enable Discard Of Frames With MAC Layer Errors
    pub mod LINEDIS {
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

            /// 0b0: Frames with errors are not discarded.
            pub const LINEDIS_0: u32 = 0b0;

            /// 0b1: Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface.
            pub const LINEDIS_1: u32 = 0b1;
        }
    }

    /// RX FIFO Shift-16
    pub mod SHIFT16 {
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

            /// 0b0: Disabled.
            pub const SHIFT16_0: u32 = 0b0;

            /// 0b1: Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO.
            pub const SHIFT16_1: u32 = 0b1;
        }
    }
}

/// Reserved Statistic Register
pub mod RMON_T_DROP {}

/// Tx Packet Count Statistic Register
pub mod RMON_T_PACKETS {

    /// Packet count
    pub mod TXPKTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx Broadcast Packets Statistic Register
pub mod RMON_T_BC_PKT {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Multicast Packets Statistic Register
pub mod RMON_T_MC_PKT {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets with CRC/Align Error Statistic Register
pub mod RMON_T_CRC_ALIGN {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets Less Than Bytes and Good CRC Statistic Register
pub mod RMON_T_UNDERSIZE {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets GT MAX_FL bytes and Good CRC Statistic Register
pub mod RMON_T_OVERSIZE {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register
pub mod RMON_T_FRAG {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register
pub mod RMON_T_JAB {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Collision Count Statistic Register
pub mod RMON_T_COL {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 64-Byte Packets Statistic Register
pub mod RMON_T_P64 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 65- to 127-byte Packets Statistic Register
pub mod RMON_T_P65TO127 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 128- to 255-byte Packets Statistic Register
pub mod RMON_T_P128TO255 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 256- to 511-byte Packets Statistic Register
pub mod RMON_T_P256TO511 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 512- to 1023-byte Packets Statistic Register
pub mod RMON_T_P512TO1023 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx 1024- to 2047-byte Packets Statistic Register
pub mod RMON_T_P1024TO2047 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Packets Greater Than 2048 Bytes Statistic Register
pub mod RMON_T_P_GTE2048 {
    pub use super::RMON_T_PACKETS::TXPKTS;
}

/// Tx Octets Statistic Register
pub mod RMON_T_OCTETS {

    /// Number of transmit octets
    pub mod TXOCTS {
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

/// Reserved Statistic Register
pub mod IEEE_T_DROP {}

/// Frames Transmitted OK Statistic Register
pub mod IEEE_T_FRAME_OK {

    /// Number of frames transmitted OK
    pub mod COUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Frames Transmitted with Single Collision Statistic Register
pub mod IEEE_T_1COL {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted with Multiple Collisions Statistic Register
pub mod IEEE_T_MCOL {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted after Deferral Delay Statistic Register
pub mod IEEE_T_DEF {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted with Late Collision Statistic Register
pub mod IEEE_T_LCOL {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted with Excessive Collisions Statistic Register
pub mod IEEE_T_EXCOL {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted with Tx FIFO Underrun Statistic Register
pub mod IEEE_T_MACERR {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Transmitted with Carrier Sense Error Statistic Register
pub mod IEEE_T_CSERR {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Reserved Statistic Register
pub mod IEEE_T_SQE {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Flow Control Pause Frames Transmitted Statistic Register
pub mod IEEE_T_FDXFC {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Octet Count for Frames Transmitted w/o Error Statistic Register
pub mod IEEE_T_OCTETS_OK {

    /// Octet count for frames transmitted without error Counts total octets (includes header and FCS fields).
    pub mod COUNT {
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

/// Rx Packet Count Statistic Register
pub mod RMON_R_PACKETS {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Broadcast Packets Statistic Register
pub mod RMON_R_BC_PKT {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Multicast Packets Statistic Register
pub mod RMON_R_MC_PKT {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets with CRC/Align Error Statistic Register
pub mod RMON_R_CRC_ALIGN {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register
pub mod RMON_R_UNDERSIZE {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets Greater Than MAX_FL and Good CRC Statistic Register
pub mod RMON_R_OVERSIZE {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register
pub mod RMON_R_FRAG {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register
pub mod RMON_R_JAB {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Reserved Statistic Register
pub mod RMON_R_RESVD_0 {}

/// Rx 64-Byte Packets Statistic Register
pub mod RMON_R_P64 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx 65- to 127-Byte Packets Statistic Register
pub mod RMON_R_P65TO127 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx 128- to 255-Byte Packets Statistic Register
pub mod RMON_R_P128TO255 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx 256- to 511-Byte Packets Statistic Register
pub mod RMON_R_P256TO511 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx 512- to 1023-Byte Packets Statistic Register
pub mod RMON_R_P512TO1023 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx 1024- to 2047-Byte Packets Statistic Register
pub mod RMON_R_P1024TO2047 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Packets Greater than 2048 Bytes Statistic Register
pub mod RMON_R_P_GTE2048 {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Rx Octets Statistic Register
pub mod RMON_R_OCTETS {
    pub use super::IEEE_T_OCTETS_OK::COUNT;
}

/// Frames not Counted Correctly Statistic Register
pub mod IEEE_R_DROP {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Received OK Statistic Register
pub mod IEEE_R_FRAME_OK {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Received with CRC Error Statistic Register
pub mod IEEE_R_CRC {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Frames Received with Alignment Error Statistic Register
pub mod IEEE_R_ALIGN {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Receive FIFO Overflow Count Statistic Register
pub mod IEEE_R_MACERR {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Flow Control Pause Frames Received Statistic Register
pub mod IEEE_R_FDXFC {
    pub use super::IEEE_T_FRAME_OK::COUNT;
}

/// Octet Count for Frames Received without Error Statistic Register
pub mod IEEE_R_OCTETS_OK {
    pub use super::IEEE_T_OCTETS_OK::COUNT;
}

/// Adjustable Timer Control Register
pub mod ATCR {

    /// Enable Timer
    pub mod EN {
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

            /// 0b0: The timer stops at the current value.
            pub const EN_0: u32 = 0b0;

            /// 0b1: The timer starts incrementing.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// Enable One-Shot Offset Event
    pub mod OFFEN {
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

            /// 0b0: Disable.
            pub const OFFEN_0: u32 = 0b0;

            /// 0b1: The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field.
            pub const OFFEN_1: u32 = 0b1;
        }
    }

    /// Reset Timer On Offset Event
    pub mod OFFRST {
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

            /// 0b0: The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached.
            pub const OFFRST_0: u32 = 0b0;

            /// 0b1: If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt.
            pub const OFFRST_1: u32 = 0b1;
        }
    }

    /// Enable Periodical Event
    pub mod PEREN {
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

            /// 0b0: Disable.
            pub const PEREN_0: u32 = 0b0;

            /// 0b1: A period event interrupt can be generated (EIR\[TS_TIMER\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details.
            pub const PEREN_1: u32 = 0b1;
        }
    }

    /// Enables event signal output assertion on period event
    pub mod PINPER {
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

            /// 0b0: Disable.
            pub const PINPER_0: u32 = 0b0;

            /// 0b1: Enable.
            pub const PINPER_1: u32 = 0b1;
        }
    }

    /// Reset Timer
    pub mod RESTART {
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

    /// Capture Timer Value
    pub mod CAPTURE {
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

            /// 0b0: No effect.
            pub const CAPTURE_0: u32 = 0b0;

            /// 0b1: The current time is captured and can be read from the ATVR register.
            pub const CAPTURE_1: u32 = 0b1;
        }
    }

    /// Enable Timer Slave Mode
    pub mod SLAVE {
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

            /// 0b0: The timer is active and all configuration fields in this register are relevant.
            pub const SLAVE_0: u32 = 0b0;

            /// 0b1: The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value.
            pub const SLAVE_1: u32 = 0b1;
        }
    }
}

/// Timer Value Register
pub mod ATVR {

    /// A write sets the timer
    pub mod ATIME {
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

/// Timer Offset Register
pub mod ATOFF {

    /// Offset value for one-shot event generation
    pub mod OFFSET {
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

/// Timer Period Register
pub mod ATPER {

    /// Value for generating periodic events
    pub mod PERIOD {
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

/// Timer Correction Register
pub mod ATCOR {

    /// Correction Counter Wrap-Around Value
    pub mod COR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (31 bits: 0x7fffffff << 0)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Time-Stamping Clock Period Register
pub mod ATINC {

    /// Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds
    pub mod INC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correction Increment Value
    pub mod INC_CORR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timestamp of Last Transmitted Frame
pub mod ATSTMP {

    /// Timestamp of the last frame transmitted by the core that had TxBD\[TS\] set
    pub mod TIMESTAMP {
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

/// Timer Global Status Register
pub mod TGSR {

    /// Copy Of Timer Flag For Channel 0
    pub mod TF0 {
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

            /// 0b0: Timer Flag for Channel 0 is clear
            pub const TF0_0: u32 = 0b0;

            /// 0b1: Timer Flag for Channel 0 is set
            pub const TF0_1: u32 = 0b1;
        }
    }

    /// Copy Of Timer Flag For Channel 1
    pub mod TF1 {
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

            /// 0b0: Timer Flag for Channel 1 is clear
            pub const TF1_0: u32 = 0b0;

            /// 0b1: Timer Flag for Channel 1 is set
            pub const TF1_1: u32 = 0b1;
        }
    }

    /// Copy Of Timer Flag For Channel 2
    pub mod TF2 {
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

            /// 0b0: Timer Flag for Channel 2 is clear
            pub const TF2_0: u32 = 0b0;

            /// 0b1: Timer Flag for Channel 2 is set
            pub const TF2_1: u32 = 0b1;
        }
    }

    /// Copy Of Timer Flag For Channel 3
    pub mod TF3 {
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

            /// 0b0: Timer Flag for Channel 3 is clear
            pub const TF3_0: u32 = 0b0;

            /// 0b1: Timer Flag for Channel 3 is set
            pub const TF3_1: u32 = 0b1;
        }
    }
}

/// Timer Control Status Register
pub mod TCSR0 {

    /// Timer DMA Request Enable
    pub mod TDRE {
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

            /// 0b0: DMA request is disabled
            pub const TDRE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const TDRE_1: u32 = 0b1;
        }
    }

    /// Timer Mode
    pub mod TMODE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Timer Channel is disabled.
            pub const TMODE_0: u32 = 0b0000;

            /// 0b0001: Timer Channel is configured for Input Capture on rising edge.
            pub const TMODE_1: u32 = 0b0001;

            /// 0b0010: Timer Channel is configured for Input Capture on falling edge.
            pub const TMODE_2: u32 = 0b0010;

            /// 0b0011: Timer Channel is configured for Input Capture on both edges.
            pub const TMODE_3: u32 = 0b0011;

            /// 0b0100: Timer Channel is configured for Output Compare - software only.
            pub const TMODE_4: u32 = 0b0100;

            /// 0b0101: Timer Channel is configured for Output Compare - toggle output on compare.
            pub const TMODE_5: u32 = 0b0101;

            /// 0b0110: Timer Channel is configured for Output Compare - clear output on compare.
            pub const TMODE_6: u32 = 0b0110;

            /// 0b0111: Timer Channel is configured for Output Compare - set output on compare.
            pub const TMODE_7: u32 = 0b0111;

            /// 0b0000: Timer Channel is configured for Output Compare - set output on compare, clear output on overflow.
            pub const TMODE_9: u32 = 0b0000;

            /// 0b1010: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow.
            pub const TMODE_10: u32 = 0b1010;

            /// 0b1110: Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC.
            pub const TMODE_14: u32 = 0b1110;

            /// 0b1111: Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC.
            pub const TMODE_15: u32 = 0b1111;
        }
    }

    /// Timer Interrupt Enable
    pub mod TIE {
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

            /// 0b0: Interrupt is disabled
            pub const TIE_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled
            pub const TIE_1: u32 = 0b1;
        }
    }

    /// Timer Flag
    pub mod TF {
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

            /// 0b0: Input Capture or Output Compare has not occurred.
            pub const TF_0: u32 = 0b0;

            /// 0b1: Input Capture or Output Compare has occurred.
            pub const TF_1: u32 = 0b1;
        }
    }

    /// Timer PulseWidth Control
    pub mod TPWC {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Pulse width is one 1588-clock cycle.
            pub const TPWC_0: u32 = 0b00000;

            /// 0b00001: Pulse width is two 1588-clock cycles.
            pub const TPWC_1: u32 = 0b00001;

            /// 0b00010: Pulse width is three 1588-clock cycles.
            pub const TPWC_2: u32 = 0b00010;

            /// 0b00011: Pulse width is four 1588-clock cycles.
            pub const TPWC_3: u32 = 0b00011;

            /// 0b11111: Pulse width is 32 1588-clock cycles.
            pub const TPWC_31: u32 = 0b11111;
        }
    }
}

/// Timer Control Status Register
pub mod TCSR1 {
    pub use super::TCSR0::TDRE;
    pub use super::TCSR0::TF;
    pub use super::TCSR0::TIE;
    pub use super::TCSR0::TMODE;
    pub use super::TCSR0::TPWC;
}

/// Timer Control Status Register
pub mod TCSR2 {
    pub use super::TCSR0::TDRE;
    pub use super::TCSR0::TF;
    pub use super::TCSR0::TIE;
    pub use super::TCSR0::TMODE;
    pub use super::TCSR0::TPWC;
}

/// Timer Control Status Register
pub mod TCSR3 {
    pub use super::TCSR0::TDRE;
    pub use super::TCSR0::TF;
    pub use super::TCSR0::TIE;
    pub use super::TCSR0::TMODE;
    pub use super::TCSR0::TPWC;
}

/// Timer Compare Capture Register
pub mod TCCR0 {

    /// Timer Capture Compare
    pub mod TCC {
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

/// Timer Compare Capture Register
pub mod TCCR1 {
    pub use super::TCCR0::TCC;
}

/// Timer Compare Capture Register
pub mod TCCR2 {
    pub use super::TCCR0::TCC;
}

/// Timer Compare Capture Register
pub mod TCCR3 {
    pub use super::TCCR0::TCC;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// Interrupt Event Register
    pub EIR: RWRegister<u32>,

    /// Interrupt Mask Register
    pub EIMR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Receive Descriptor Active Register
    pub RDAR: RWRegister<u32>,

    /// Transmit Descriptor Active Register
    pub TDAR: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Ethernet Control Register
    pub ECR: RWRegister<u32>,

    _reserved4: [u32; 6],

    /// MII Management Frame Register
    pub MMFR: RWRegister<u32>,

    /// MII Speed Control Register
    pub MSCR: RWRegister<u32>,

    _reserved5: [u32; 7],

    /// MIB Control Register
    pub MIBC: RWRegister<u32>,

    _reserved6: [u32; 7],

    /// Receive Control Register
    pub RCR: RWRegister<u32>,

    _reserved7: [u32; 15],

    /// Transmit Control Register
    pub TCR: RWRegister<u32>,

    _reserved8: [u32; 7],

    /// Physical Address Lower Register
    pub PALR: RWRegister<u32>,

    /// Physical Address Upper Register
    pub PAUR: RWRegister<u32>,

    /// Opcode/Pause Duration Register
    pub OPD: RWRegister<u32>,

    /// Transmit Interrupt Coalescing Register
    pub TXIC: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// Receive Interrupt Coalescing Register
    pub RXIC: RWRegister<u32>,

    _reserved10: [u32; 5],

    /// Descriptor Individual Upper Address Register
    pub IAUR: RWRegister<u32>,

    /// Descriptor Individual Lower Address Register
    pub IALR: RWRegister<u32>,

    /// Descriptor Group Upper Address Register
    pub GAUR: RWRegister<u32>,

    /// Descriptor Group Lower Address Register
    pub GALR: RWRegister<u32>,

    _reserved11: [u32; 7],

    /// Transmit FIFO Watermark Register
    pub TFWR: RWRegister<u32>,

    _reserved12: [u32; 14],

    /// Receive Descriptor Ring Start Register
    pub RDSR: RWRegister<u32>,

    /// Transmit Buffer Descriptor Ring Start Register
    pub TDSR: RWRegister<u32>,

    /// Maximum Receive Buffer Size Register
    pub MRBR: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// Receive FIFO Section Full Threshold
    pub RSFL: RWRegister<u32>,

    /// Receive FIFO Section Empty Threshold
    pub RSEM: RWRegister<u32>,

    /// Receive FIFO Almost Empty Threshold
    pub RAEM: RWRegister<u32>,

    /// Receive FIFO Almost Full Threshold
    pub RAFL: RWRegister<u32>,

    /// Transmit FIFO Section Empty Threshold
    pub TSEM: RWRegister<u32>,

    /// Transmit FIFO Almost Empty Threshold
    pub TAEM: RWRegister<u32>,

    /// Transmit FIFO Almost Full Threshold
    pub TAFL: RWRegister<u32>,

    /// Transmit Inter-Packet Gap
    pub TIPG: RWRegister<u32>,

    /// Frame Truncation Length
    pub FTRL: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// Transmit Accelerator Function Configuration
    pub TACC: RWRegister<u32>,

    /// Receive Accelerator Function Configuration
    pub RACC: RWRegister<u32>,

    _reserved15: [u32; 14],

    /// Reserved Statistic Register
    pub RMON_T_DROP: RORegister<u32>,

    /// Tx Packet Count Statistic Register
    pub RMON_T_PACKETS: RORegister<u32>,

    /// Tx Broadcast Packets Statistic Register
    pub RMON_T_BC_PKT: RORegister<u32>,

    /// Tx Multicast Packets Statistic Register
    pub RMON_T_MC_PKT: RORegister<u32>,

    /// Tx Packets with CRC/Align Error Statistic Register
    pub RMON_T_CRC_ALIGN: RORegister<u32>,

    /// Tx Packets Less Than Bytes and Good CRC Statistic Register
    pub RMON_T_UNDERSIZE: RORegister<u32>,

    /// Tx Packets GT MAX_FL bytes and Good CRC Statistic Register
    pub RMON_T_OVERSIZE: RORegister<u32>,

    /// Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register
    pub RMON_T_FRAG: RORegister<u32>,

    /// Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register
    pub RMON_T_JAB: RORegister<u32>,

    /// Tx Collision Count Statistic Register
    pub RMON_T_COL: RORegister<u32>,

    /// Tx 64-Byte Packets Statistic Register
    pub RMON_T_P64: RORegister<u32>,

    /// Tx 65- to 127-byte Packets Statistic Register
    pub RMON_T_P65TO127: RORegister<u32>,

    /// Tx 128- to 255-byte Packets Statistic Register
    pub RMON_T_P128TO255: RORegister<u32>,

    /// Tx 256- to 511-byte Packets Statistic Register
    pub RMON_T_P256TO511: RORegister<u32>,

    /// Tx 512- to 1023-byte Packets Statistic Register
    pub RMON_T_P512TO1023: RORegister<u32>,

    /// Tx 1024- to 2047-byte Packets Statistic Register
    pub RMON_T_P1024TO2047: RORegister<u32>,

    /// Tx Packets Greater Than 2048 Bytes Statistic Register
    pub RMON_T_P_GTE2048: RORegister<u32>,

    /// Tx Octets Statistic Register
    pub RMON_T_OCTETS: RORegister<u32>,

    /// Reserved Statistic Register
    pub IEEE_T_DROP: RORegister<u32>,

    /// Frames Transmitted OK Statistic Register
    pub IEEE_T_FRAME_OK: RORegister<u32>,

    /// Frames Transmitted with Single Collision Statistic Register
    pub IEEE_T_1COL: RORegister<u32>,

    /// Frames Transmitted with Multiple Collisions Statistic Register
    pub IEEE_T_MCOL: RORegister<u32>,

    /// Frames Transmitted after Deferral Delay Statistic Register
    pub IEEE_T_DEF: RORegister<u32>,

    /// Frames Transmitted with Late Collision Statistic Register
    pub IEEE_T_LCOL: RORegister<u32>,

    /// Frames Transmitted with Excessive Collisions Statistic Register
    pub IEEE_T_EXCOL: RORegister<u32>,

    /// Frames Transmitted with Tx FIFO Underrun Statistic Register
    pub IEEE_T_MACERR: RORegister<u32>,

    /// Frames Transmitted with Carrier Sense Error Statistic Register
    pub IEEE_T_CSERR: RORegister<u32>,

    /// Reserved Statistic Register
    pub IEEE_T_SQE: RORegister<u32>,

    /// Flow Control Pause Frames Transmitted Statistic Register
    pub IEEE_T_FDXFC: RORegister<u32>,

    /// Octet Count for Frames Transmitted w/o Error Statistic Register
    pub IEEE_T_OCTETS_OK: RORegister<u32>,

    _reserved16: [u32; 3],

    /// Rx Packet Count Statistic Register
    pub RMON_R_PACKETS: RORegister<u32>,

    /// Rx Broadcast Packets Statistic Register
    pub RMON_R_BC_PKT: RORegister<u32>,

    /// Rx Multicast Packets Statistic Register
    pub RMON_R_MC_PKT: RORegister<u32>,

    /// Rx Packets with CRC/Align Error Statistic Register
    pub RMON_R_CRC_ALIGN: RORegister<u32>,

    /// Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register
    pub RMON_R_UNDERSIZE: RORegister<u32>,

    /// Rx Packets Greater Than MAX_FL and Good CRC Statistic Register
    pub RMON_R_OVERSIZE: RORegister<u32>,

    /// Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register
    pub RMON_R_FRAG: RORegister<u32>,

    /// Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register
    pub RMON_R_JAB: RORegister<u32>,

    /// Reserved Statistic Register
    pub RMON_R_RESVD_0: RORegister<u32>,

    /// Rx 64-Byte Packets Statistic Register
    pub RMON_R_P64: RORegister<u32>,

    /// Rx 65- to 127-Byte Packets Statistic Register
    pub RMON_R_P65TO127: RORegister<u32>,

    /// Rx 128- to 255-Byte Packets Statistic Register
    pub RMON_R_P128TO255: RORegister<u32>,

    /// Rx 256- to 511-Byte Packets Statistic Register
    pub RMON_R_P256TO511: RORegister<u32>,

    /// Rx 512- to 1023-Byte Packets Statistic Register
    pub RMON_R_P512TO1023: RORegister<u32>,

    /// Rx 1024- to 2047-Byte Packets Statistic Register
    pub RMON_R_P1024TO2047: RORegister<u32>,

    /// Rx Packets Greater than 2048 Bytes Statistic Register
    pub RMON_R_P_GTE2048: RORegister<u32>,

    /// Rx Octets Statistic Register
    pub RMON_R_OCTETS: RORegister<u32>,

    /// Frames not Counted Correctly Statistic Register
    pub IEEE_R_DROP: RORegister<u32>,

    /// Frames Received OK Statistic Register
    pub IEEE_R_FRAME_OK: RORegister<u32>,

    /// Frames Received with CRC Error Statistic Register
    pub IEEE_R_CRC: RORegister<u32>,

    /// Frames Received with Alignment Error Statistic Register
    pub IEEE_R_ALIGN: RORegister<u32>,

    /// Receive FIFO Overflow Count Statistic Register
    pub IEEE_R_MACERR: RORegister<u32>,

    /// Flow Control Pause Frames Received Statistic Register
    pub IEEE_R_FDXFC: RORegister<u32>,

    /// Octet Count for Frames Received without Error Statistic Register
    pub IEEE_R_OCTETS_OK: RORegister<u32>,

    _reserved17: [u32; 71],

    /// Adjustable Timer Control Register
    pub ATCR: RWRegister<u32>,

    /// Timer Value Register
    pub ATVR: RWRegister<u32>,

    /// Timer Offset Register
    pub ATOFF: RWRegister<u32>,

    /// Timer Period Register
    pub ATPER: RWRegister<u32>,

    /// Timer Correction Register
    pub ATCOR: RWRegister<u32>,

    /// Time-Stamping Clock Period Register
    pub ATINC: RWRegister<u32>,

    /// Timestamp of Last Transmitted Frame
    pub ATSTMP: RORegister<u32>,

    _reserved18: [u32; 122],

    /// Timer Global Status Register
    pub TGSR: RWRegister<u32>,

    /// Timer Control Status Register
    pub TCSR0: RWRegister<u32>,

    /// Timer Compare Capture Register
    pub TCCR0: RWRegister<u32>,

    /// Timer Control Status Register
    pub TCSR1: RWRegister<u32>,

    /// Timer Compare Capture Register
    pub TCCR1: RWRegister<u32>,

    /// Timer Control Status Register
    pub TCSR2: RWRegister<u32>,

    /// Timer Compare Capture Register
    pub TCCR2: RWRegister<u32>,

    /// Timer Control Status Register
    pub TCSR3: RWRegister<u32>,

    /// Timer Compare Capture Register
    pub TCCR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub EIR: u32,
    pub EIMR: u32,
    pub RDAR: u32,
    pub TDAR: u32,
    pub ECR: u32,
    pub MMFR: u32,
    pub MSCR: u32,
    pub MIBC: u32,
    pub RCR: u32,
    pub TCR: u32,
    pub PALR: u32,
    pub PAUR: u32,
    pub OPD: u32,
    pub TXIC: u32,
    pub RXIC: u32,
    pub IAUR: u32,
    pub IALR: u32,
    pub GAUR: u32,
    pub GALR: u32,
    pub TFWR: u32,
    pub RDSR: u32,
    pub TDSR: u32,
    pub MRBR: u32,
    pub RSFL: u32,
    pub RSEM: u32,
    pub RAEM: u32,
    pub RAFL: u32,
    pub TSEM: u32,
    pub TAEM: u32,
    pub TAFL: u32,
    pub TIPG: u32,
    pub FTRL: u32,
    pub TACC: u32,
    pub RACC: u32,
    pub RMON_T_DROP: u32,
    pub RMON_T_PACKETS: u32,
    pub RMON_T_BC_PKT: u32,
    pub RMON_T_MC_PKT: u32,
    pub RMON_T_CRC_ALIGN: u32,
    pub RMON_T_UNDERSIZE: u32,
    pub RMON_T_OVERSIZE: u32,
    pub RMON_T_FRAG: u32,
    pub RMON_T_JAB: u32,
    pub RMON_T_COL: u32,
    pub RMON_T_P64: u32,
    pub RMON_T_P65TO127: u32,
    pub RMON_T_P128TO255: u32,
    pub RMON_T_P256TO511: u32,
    pub RMON_T_P512TO1023: u32,
    pub RMON_T_P1024TO2047: u32,
    pub RMON_T_P_GTE2048: u32,
    pub RMON_T_OCTETS: u32,
    pub IEEE_T_DROP: u32,
    pub IEEE_T_FRAME_OK: u32,
    pub IEEE_T_1COL: u32,
    pub IEEE_T_MCOL: u32,
    pub IEEE_T_DEF: u32,
    pub IEEE_T_LCOL: u32,
    pub IEEE_T_EXCOL: u32,
    pub IEEE_T_MACERR: u32,
    pub IEEE_T_CSERR: u32,
    pub IEEE_T_SQE: u32,
    pub IEEE_T_FDXFC: u32,
    pub IEEE_T_OCTETS_OK: u32,
    pub RMON_R_PACKETS: u32,
    pub RMON_R_BC_PKT: u32,
    pub RMON_R_MC_PKT: u32,
    pub RMON_R_CRC_ALIGN: u32,
    pub RMON_R_UNDERSIZE: u32,
    pub RMON_R_OVERSIZE: u32,
    pub RMON_R_FRAG: u32,
    pub RMON_R_JAB: u32,
    pub RMON_R_RESVD_0: u32,
    pub RMON_R_P64: u32,
    pub RMON_R_P65TO127: u32,
    pub RMON_R_P128TO255: u32,
    pub RMON_R_P256TO511: u32,
    pub RMON_R_P512TO1023: u32,
    pub RMON_R_P1024TO2047: u32,
    pub RMON_R_P_GTE2048: u32,
    pub RMON_R_OCTETS: u32,
    pub IEEE_R_DROP: u32,
    pub IEEE_R_FRAME_OK: u32,
    pub IEEE_R_CRC: u32,
    pub IEEE_R_ALIGN: u32,
    pub IEEE_R_MACERR: u32,
    pub IEEE_R_FDXFC: u32,
    pub IEEE_R_OCTETS_OK: u32,
    pub ATCR: u32,
    pub ATVR: u32,
    pub ATOFF: u32,
    pub ATPER: u32,
    pub ATCOR: u32,
    pub ATINC: u32,
    pub ATSTMP: u32,
    pub TGSR: u32,
    pub TCSR0: u32,
    pub TCCR0: u32,
    pub TCSR1: u32,
    pub TCCR1: u32,
    pub TCSR2: u32,
    pub TCCR2: u32,
    pub TCSR3: u32,
    pub TCCR3: u32,
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
