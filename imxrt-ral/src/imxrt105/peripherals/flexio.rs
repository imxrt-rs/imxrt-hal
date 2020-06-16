#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXIO
//!
//! Used by: imxrt1051, imxrt1052

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

            /// 0b0000000000000000: Standard features implemented.
            pub const FEATURE_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Supports state, logic and parallel modes.
            pub const FEATURE_1: u32 = 0b0000000000000001;
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

    /// Shifter Number
    pub mod SHIFTER {
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

    /// Timer Number
    pub mod TIMER {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Pin Number
    pub mod PIN {
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

    /// Trigger Number
    pub mod TRIGGER {
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

/// FlexIO Control Register
pub mod CTRL {

    /// FlexIO Enable
    pub mod FLEXEN {
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

            /// 0b0: FlexIO module is disabled.
            pub const FLEXEN_0: u32 = 0b0;

            /// 0b1: FlexIO module is enabled.
            pub const FLEXEN_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod SWRST {
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

            /// 0b0: Software reset is disabled
            pub const SWRST_0: u32 = 0b0;

            /// 0b1: Software reset is enabled, all FlexIO registers except the Control Register are reset.
            pub const SWRST_1: u32 = 0b1;
        }
    }

    /// Fast Access
    pub mod FASTACC {
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

            /// 0b0: Configures for normal register accesses to FlexIO
            pub const FASTACC_0: u32 = 0b0;

            /// 0b1: Configures for fast register accesses to FlexIO
            pub const FASTACC_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGE {
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

            /// 0b0: FlexIO is disabled in debug modes.
            pub const DBGE_0: u32 = 0b0;

            /// 0b1: FlexIO is enabled in debug modes
            pub const DBGE_1: u32 = 0b1;
        }
    }

    /// Doze Enable
    pub mod DOZEN {
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

            /// 0b0: FlexIO enabled in Doze modes.
            pub const DOZEN_0: u32 = 0b0;

            /// 0b1: FlexIO disabled in Doze modes.
            pub const DOZEN_1: u32 = 0b1;
        }
    }
}

/// Pin State Register
pub mod PIN {

    /// Pin Data Input
    pub mod PDI {
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

/// Shifter Status Register
pub mod SHIFTSTAT {

    /// Shifter Status Flag
    pub mod SSF {
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

/// Shifter Error Register
pub mod SHIFTERR {

    /// Shifter Error Flags
    pub mod SEF {
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

/// Timer Status Register
pub mod TIMSTAT {

    /// Timer Status Flags
    pub mod TSF {
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

/// Shifter Status Interrupt Enable
pub mod SHIFTSIEN {

    /// Shifter Status Interrupt Enable
    pub mod SSIE {
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

/// Shifter Error Interrupt Enable
pub mod SHIFTEIEN {

    /// Shifter Error Interrupt Enable
    pub mod SEIE {
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

/// Timer Interrupt Enable Register
pub mod TIMIEN {

    /// Timer Status Interrupt Enable
    pub mod TEIE {
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

/// Shifter Status DMA Enable
pub mod SHIFTSDEN {

    /// Shifter Status DMA Enable
    pub mod SSDE {
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

/// Shifter State Register
pub mod SHIFTSTATE {

    /// Current State Pointer
    pub mod STATE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Shifter Control N Register
pub mod SHIFTCTL0 {

    /// Shifter Mode
    pub mod SMOD {
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

            /// 0b000: Disabled.
            pub const SMOD_0: u32 = 0b000;

            /// 0b001: Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer.
            pub const SMOD_1: u32 = 0b001;

            /// 0b010: Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer.
            pub const SMOD_2: u32 = 0b010;

            /// 0b100: Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer.
            pub const SMOD_4: u32 = 0b100;

            /// 0b101: Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents.
            pub const SMOD_5: u32 = 0b101;

            /// 0b110: State mode. SHIFTBUF contents are used for storing programmable state attributes.
            pub const SMOD_6: u32 = 0b110;

            /// 0b111: Logic mode. SHIFTBUF contents are used for implementing programmable logic look up table.
            pub const SMOD_7: u32 = 0b111;
        }
    }

    /// Shifter Pin Polarity
    pub mod PINPOL {
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

            /// 0b0: Pin is active high
            pub const PINPOL_0: u32 = 0b0;

            /// 0b1: Pin is active low
            pub const PINPOL_1: u32 = 0b1;
        }
    }

    /// Shifter Pin Select
    pub mod PINSEL {
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

    /// Shifter Pin Configuration
    pub mod PINCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Shifter pin output disabled
            pub const PINCFG_0: u32 = 0b00;

            /// 0b01: Shifter pin open drain or bidirectional output enable
            pub const PINCFG_1: u32 = 0b01;

            /// 0b10: Shifter pin bidirectional output data
            pub const PINCFG_2: u32 = 0b10;

            /// 0b11: Shifter pin output
            pub const PINCFG_3: u32 = 0b11;
        }
    }

    /// Timer Polarity
    pub mod TIMPOL {
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

            /// 0b0: Shift on posedge of Shift clock
            pub const TIMPOL_0: u32 = 0b0;

            /// 0b1: Shift on negedge of Shift clock
            pub const TIMPOL_1: u32 = 0b1;
        }
    }

    /// Timer Select
    pub mod TIMSEL {
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

/// Shifter Control N Register
pub mod SHIFTCTL1 {
    pub use super::SHIFTCTL0::PINCFG;
    pub use super::SHIFTCTL0::PINPOL;
    pub use super::SHIFTCTL0::PINSEL;
    pub use super::SHIFTCTL0::SMOD;
    pub use super::SHIFTCTL0::TIMPOL;
    pub use super::SHIFTCTL0::TIMSEL;
}

/// Shifter Control N Register
pub mod SHIFTCTL2 {
    pub use super::SHIFTCTL0::PINCFG;
    pub use super::SHIFTCTL0::PINPOL;
    pub use super::SHIFTCTL0::PINSEL;
    pub use super::SHIFTCTL0::SMOD;
    pub use super::SHIFTCTL0::TIMPOL;
    pub use super::SHIFTCTL0::TIMSEL;
}

/// Shifter Control N Register
pub mod SHIFTCTL3 {
    pub use super::SHIFTCTL0::PINCFG;
    pub use super::SHIFTCTL0::PINPOL;
    pub use super::SHIFTCTL0::PINSEL;
    pub use super::SHIFTCTL0::SMOD;
    pub use super::SHIFTCTL0::TIMPOL;
    pub use super::SHIFTCTL0::TIMSEL;
}

/// Shifter Configuration N Register
pub mod SHIFTCFG0 {

    /// Shifter Start bit
    pub mod SSTART {
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

            /// 0b00: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable
            pub const SSTART_0: u32 = 0b00;

            /// 0b01: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift
            pub const SSTART_1: u32 = 0b01;

            /// 0b10: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0
            pub const SSTART_2: u32 = 0b10;

            /// 0b11: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1
            pub const SSTART_3: u32 = 0b11;
        }
    }

    /// Shifter Stop bit
    pub mod SSTOP {
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

            /// 0b00: Stop bit disabled for transmitter/receiver/match store
            pub const SSTOP_0: u32 = 0b00;

            /// 0b10: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0
            pub const SSTOP_2: u32 = 0b10;

            /// 0b11: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1
            pub const SSTOP_3: u32 = 0b11;
        }
    }

    /// Input Source
    pub mod INSRC {
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

            /// 0b0: Pin
            pub const INSRC_0: u32 = 0b0;

            /// 0b1: Shifter N+1 Output
            pub const INSRC_1: u32 = 0b1;
        }
    }

    /// Parallel Width
    pub mod PWIDTH {
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

/// Shifter Configuration N Register
pub mod SHIFTCFG1 {
    pub use super::SHIFTCFG0::INSRC;
    pub use super::SHIFTCFG0::PWIDTH;
    pub use super::SHIFTCFG0::SSTART;
    pub use super::SHIFTCFG0::SSTOP;
}

/// Shifter Configuration N Register
pub mod SHIFTCFG2 {
    pub use super::SHIFTCFG0::INSRC;
    pub use super::SHIFTCFG0::PWIDTH;
    pub use super::SHIFTCFG0::SSTART;
    pub use super::SHIFTCFG0::SSTOP;
}

/// Shifter Configuration N Register
pub mod SHIFTCFG3 {
    pub use super::SHIFTCFG0::INSRC;
    pub use super::SHIFTCFG0::PWIDTH;
    pub use super::SHIFTCFG0::SSTART;
    pub use super::SHIFTCFG0::SSTOP;
}

/// Shifter Buffer N Register
pub mod SHIFTBUF0 {

    /// Shift Buffer
    pub mod SHIFTBUF {
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

/// Shifter Buffer N Register
pub mod SHIFTBUF1 {
    pub use super::SHIFTBUF0::SHIFTBUF;
}

/// Shifter Buffer N Register
pub mod SHIFTBUF2 {
    pub use super::SHIFTBUF0::SHIFTBUF;
}

/// Shifter Buffer N Register
pub mod SHIFTBUF3 {
    pub use super::SHIFTBUF0::SHIFTBUF;
}

/// Shifter Buffer N Bit Swapped Register
pub mod SHIFTBUFBIS0 {

    /// Shift Buffer
    pub mod SHIFTBUFBIS {
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

/// Shifter Buffer N Bit Swapped Register
pub mod SHIFTBUFBIS1 {
    pub use super::SHIFTBUFBIS0::SHIFTBUFBIS;
}

/// Shifter Buffer N Bit Swapped Register
pub mod SHIFTBUFBIS2 {
    pub use super::SHIFTBUFBIS0::SHIFTBUFBIS;
}

/// Shifter Buffer N Bit Swapped Register
pub mod SHIFTBUFBIS3 {
    pub use super::SHIFTBUFBIS0::SHIFTBUFBIS;
}

/// Shifter Buffer N Byte Swapped Register
pub mod SHIFTBUFBYS0 {

    /// Shift Buffer
    pub mod SHIFTBUFBYS {
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

/// Shifter Buffer N Byte Swapped Register
pub mod SHIFTBUFBYS1 {
    pub use super::SHIFTBUFBYS0::SHIFTBUFBYS;
}

/// Shifter Buffer N Byte Swapped Register
pub mod SHIFTBUFBYS2 {
    pub use super::SHIFTBUFBYS0::SHIFTBUFBYS;
}

/// Shifter Buffer N Byte Swapped Register
pub mod SHIFTBUFBYS3 {
    pub use super::SHIFTBUFBYS0::SHIFTBUFBYS;
}

/// Shifter Buffer N Bit Byte Swapped Register
pub mod SHIFTBUFBBS0 {

    /// Shift Buffer
    pub mod SHIFTBUFBBS {
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

/// Shifter Buffer N Bit Byte Swapped Register
pub mod SHIFTBUFBBS1 {
    pub use super::SHIFTBUFBBS0::SHIFTBUFBBS;
}

/// Shifter Buffer N Bit Byte Swapped Register
pub mod SHIFTBUFBBS2 {
    pub use super::SHIFTBUFBBS0::SHIFTBUFBBS;
}

/// Shifter Buffer N Bit Byte Swapped Register
pub mod SHIFTBUFBBS3 {
    pub use super::SHIFTBUFBBS0::SHIFTBUFBBS;
}

/// Timer Control N Register
pub mod TIMCTL0 {

    /// Timer Mode
    pub mod TIMOD {
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

            /// 0b00: Timer Disabled.
            pub const TIMOD_0: u32 = 0b00;

            /// 0b01: Dual 8-bit counters baud mode.
            pub const TIMOD_1: u32 = 0b01;

            /// 0b10: Dual 8-bit counters PWM high mode.
            pub const TIMOD_2: u32 = 0b10;

            /// 0b11: Single 16-bit counter mode.
            pub const TIMOD_3: u32 = 0b11;
        }
    }

    /// Timer Pin Polarity
    pub mod PINPOL {
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

            /// 0b0: Pin is active high
            pub const PINPOL_0: u32 = 0b0;

            /// 0b1: Pin is active low
            pub const PINPOL_1: u32 = 0b1;
        }
    }

    /// Timer Pin Select
    pub mod PINSEL {
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

    /// Timer Pin Configuration
    pub mod PINCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Timer pin output disabled
            pub const PINCFG_0: u32 = 0b00;

            /// 0b01: Timer pin open drain or bidirectional output enable
            pub const PINCFG_1: u32 = 0b01;

            /// 0b10: Timer pin bidirectional output data
            pub const PINCFG_2: u32 = 0b10;

            /// 0b11: Timer pin output
            pub const PINCFG_3: u32 = 0b11;
        }
    }

    /// Trigger Source
    pub mod TRGSRC {
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

            /// 0b0: External trigger selected
            pub const TRGSRC_0: u32 = 0b0;

            /// 0b1: Internal trigger selected
            pub const TRGSRC_1: u32 = 0b1;
        }
    }

    /// Trigger Polarity
    pub mod TRGPOL {
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

            /// 0b0: Trigger active high
            pub const TRGPOL_0: u32 = 0b0;

            /// 0b1: Trigger active low
            pub const TRGPOL_1: u32 = 0b1;
        }
    }

    /// Trigger Select
    pub mod TRGSEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Control N Register
pub mod TIMCTL1 {
    pub use super::TIMCTL0::PINCFG;
    pub use super::TIMCTL0::PINPOL;
    pub use super::TIMCTL0::PINSEL;
    pub use super::TIMCTL0::TIMOD;
    pub use super::TIMCTL0::TRGPOL;
    pub use super::TIMCTL0::TRGSEL;
    pub use super::TIMCTL0::TRGSRC;
}

/// Timer Control N Register
pub mod TIMCTL2 {
    pub use super::TIMCTL0::PINCFG;
    pub use super::TIMCTL0::PINPOL;
    pub use super::TIMCTL0::PINSEL;
    pub use super::TIMCTL0::TIMOD;
    pub use super::TIMCTL0::TRGPOL;
    pub use super::TIMCTL0::TRGSEL;
    pub use super::TIMCTL0::TRGSRC;
}

/// Timer Control N Register
pub mod TIMCTL3 {
    pub use super::TIMCTL0::PINCFG;
    pub use super::TIMCTL0::PINPOL;
    pub use super::TIMCTL0::PINSEL;
    pub use super::TIMCTL0::TIMOD;
    pub use super::TIMCTL0::TRGPOL;
    pub use super::TIMCTL0::TRGSEL;
    pub use super::TIMCTL0::TRGSRC;
}

/// Timer Configuration N Register
pub mod TIMCFG0 {

    /// Timer Start Bit
    pub mod TSTART {
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

            /// 0b0: Start bit disabled
            pub const TSTART_0: u32 = 0b0;

            /// 0b1: Start bit enabled
            pub const TSTART_1: u32 = 0b1;
        }
    }

    /// Timer Stop Bit
    pub mod TSTOP {
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

            /// 0b00: Stop bit disabled
            pub const TSTOP_0: u32 = 0b00;

            /// 0b01: Stop bit is enabled on timer compare
            pub const TSTOP_1: u32 = 0b01;

            /// 0b10: Stop bit is enabled on timer disable
            pub const TSTOP_2: u32 = 0b10;

            /// 0b11: Stop bit is enabled on timer compare and timer disable
            pub const TSTOP_3: u32 = 0b11;
        }
    }

    /// Timer Enable
    pub mod TIMENA {
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

            /// 0b000: Timer always enabled
            pub const TIMENA_0: u32 = 0b000;

            /// 0b001: Timer enabled on Timer N-1 enable
            pub const TIMENA_1: u32 = 0b001;

            /// 0b010: Timer enabled on Trigger high
            pub const TIMENA_2: u32 = 0b010;

            /// 0b011: Timer enabled on Trigger high and Pin high
            pub const TIMENA_3: u32 = 0b011;

            /// 0b100: Timer enabled on Pin rising edge
            pub const TIMENA_4: u32 = 0b100;

            /// 0b101: Timer enabled on Pin rising edge and Trigger high
            pub const TIMENA_5: u32 = 0b101;

            /// 0b110: Timer enabled on Trigger rising edge
            pub const TIMENA_6: u32 = 0b110;

            /// 0b111: Timer enabled on Trigger rising or falling edge
            pub const TIMENA_7: u32 = 0b111;
        }
    }

    /// Timer Disable
    pub mod TIMDIS {
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

            /// 0b000: Timer never disabled
            pub const TIMDIS_0: u32 = 0b000;

            /// 0b001: Timer disabled on Timer N-1 disable
            pub const TIMDIS_1: u32 = 0b001;

            /// 0b010: Timer disabled on Timer compare (upper 8-bits match and decrement)
            pub const TIMDIS_2: u32 = 0b010;

            /// 0b011: Timer disabled on Timer compare (upper 8-bits match and decrement) and Trigger Low
            pub const TIMDIS_3: u32 = 0b011;

            /// 0b100: Timer disabled on Pin rising or falling edge
            pub const TIMDIS_4: u32 = 0b100;

            /// 0b101: Timer disabled on Pin rising or falling edge provided Trigger is high
            pub const TIMDIS_5: u32 = 0b101;

            /// 0b110: Timer disabled on Trigger falling edge
            pub const TIMDIS_6: u32 = 0b110;
        }
    }

    /// Timer Reset
    pub mod TIMRST {
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

            /// 0b000: Timer never reset
            pub const TIMRST_0: u32 = 0b000;

            /// 0b010: Timer reset on Timer Pin equal to Timer Output
            pub const TIMRST_2: u32 = 0b010;

            /// 0b011: Timer reset on Timer Trigger equal to Timer Output
            pub const TIMRST_3: u32 = 0b011;

            /// 0b100: Timer reset on Timer Pin rising edge
            pub const TIMRST_4: u32 = 0b100;

            /// 0b110: Timer reset on Trigger rising edge
            pub const TIMRST_6: u32 = 0b110;

            /// 0b111: Timer reset on Trigger rising or falling edge
            pub const TIMRST_7: u32 = 0b111;
        }
    }

    /// Timer Decrement
    pub mod TIMDEC {
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

            /// 0b00: Decrement counter on FlexIO clock, Shift clock equals Timer output.
            pub const TIMDEC_0: u32 = 0b00;

            /// 0b01: Decrement counter on Trigger input (both edges), Shift clock equals Timer output.
            pub const TIMDEC_1: u32 = 0b01;

            /// 0b10: Decrement counter on Pin input (both edges), Shift clock equals Pin input.
            pub const TIMDEC_2: u32 = 0b10;

            /// 0b11: Decrement counter on Trigger input (both edges), Shift clock equals Trigger input.
            pub const TIMDEC_3: u32 = 0b11;
        }
    }

    /// Timer Output
    pub mod TIMOUT {
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

            /// 0b00: Timer output is logic one when enabled and is not affected by timer reset
            pub const TIMOUT_0: u32 = 0b00;

            /// 0b01: Timer output is logic zero when enabled and is not affected by timer reset
            pub const TIMOUT_1: u32 = 0b01;

            /// 0b10: Timer output is logic one when enabled and on timer reset
            pub const TIMOUT_2: u32 = 0b10;

            /// 0b11: Timer output is logic zero when enabled and on timer reset
            pub const TIMOUT_3: u32 = 0b11;
        }
    }
}

/// Timer Configuration N Register
pub mod TIMCFG1 {
    pub use super::TIMCFG0::TIMDEC;
    pub use super::TIMCFG0::TIMDIS;
    pub use super::TIMCFG0::TIMENA;
    pub use super::TIMCFG0::TIMOUT;
    pub use super::TIMCFG0::TIMRST;
    pub use super::TIMCFG0::TSTART;
    pub use super::TIMCFG0::TSTOP;
}

/// Timer Configuration N Register
pub mod TIMCFG2 {
    pub use super::TIMCFG0::TIMDEC;
    pub use super::TIMCFG0::TIMDIS;
    pub use super::TIMCFG0::TIMENA;
    pub use super::TIMCFG0::TIMOUT;
    pub use super::TIMCFG0::TIMRST;
    pub use super::TIMCFG0::TSTART;
    pub use super::TIMCFG0::TSTOP;
}

/// Timer Configuration N Register
pub mod TIMCFG3 {
    pub use super::TIMCFG0::TIMDEC;
    pub use super::TIMCFG0::TIMDIS;
    pub use super::TIMCFG0::TIMENA;
    pub use super::TIMCFG0::TIMOUT;
    pub use super::TIMCFG0::TIMRST;
    pub use super::TIMCFG0::TSTART;
    pub use super::TIMCFG0::TSTOP;
}

/// Timer Compare N Register
pub mod TIMCMP0 {

    /// Timer Compare Value
    pub mod CMP {
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

/// Timer Compare N Register
pub mod TIMCMP1 {
    pub use super::TIMCMP0::CMP;
}

/// Timer Compare N Register
pub mod TIMCMP2 {
    pub use super::TIMCMP0::CMP;
}

/// Timer Compare N Register
pub mod TIMCMP3 {
    pub use super::TIMCMP0::CMP;
}

/// Shifter Buffer N Nibble Byte Swapped Register
pub mod SHIFTBUFNBS0 {

    /// Shift Buffer
    pub mod SHIFTBUFNBS {
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

/// Shifter Buffer N Nibble Byte Swapped Register
pub mod SHIFTBUFNBS1 {
    pub use super::SHIFTBUFNBS0::SHIFTBUFNBS;
}

/// Shifter Buffer N Nibble Byte Swapped Register
pub mod SHIFTBUFNBS2 {
    pub use super::SHIFTBUFNBS0::SHIFTBUFNBS;
}

/// Shifter Buffer N Nibble Byte Swapped Register
pub mod SHIFTBUFNBS3 {
    pub use super::SHIFTBUFNBS0::SHIFTBUFNBS;
}

/// Shifter Buffer N Half Word Swapped Register
pub mod SHIFTBUFHWS0 {

    /// Shift Buffer
    pub mod SHIFTBUFHWS {
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

/// Shifter Buffer N Half Word Swapped Register
pub mod SHIFTBUFHWS1 {
    pub use super::SHIFTBUFHWS0::SHIFTBUFHWS;
}

/// Shifter Buffer N Half Word Swapped Register
pub mod SHIFTBUFHWS2 {
    pub use super::SHIFTBUFHWS0::SHIFTBUFHWS;
}

/// Shifter Buffer N Half Word Swapped Register
pub mod SHIFTBUFHWS3 {
    pub use super::SHIFTBUFHWS0::SHIFTBUFHWS;
}

/// Shifter Buffer N Nibble Swapped Register
pub mod SHIFTBUFNIS0 {

    /// Shift Buffer
    pub mod SHIFTBUFNIS {
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

/// Shifter Buffer N Nibble Swapped Register
pub mod SHIFTBUFNIS1 {
    pub use super::SHIFTBUFNIS0::SHIFTBUFNIS;
}

/// Shifter Buffer N Nibble Swapped Register
pub mod SHIFTBUFNIS2 {
    pub use super::SHIFTBUFNIS0::SHIFTBUFNIS;
}

/// Shifter Buffer N Nibble Swapped Register
pub mod SHIFTBUFNIS3 {
    pub use super::SHIFTBUFNIS0::SHIFTBUFNIS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// FlexIO Control Register
    pub CTRL: RWRegister<u32>,

    /// Pin State Register
    pub PIN: RORegister<u32>,

    /// Shifter Status Register
    pub SHIFTSTAT: RWRegister<u32>,

    /// Shifter Error Register
    pub SHIFTERR: RWRegister<u32>,

    /// Timer Status Register
    pub TIMSTAT: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Shifter Status Interrupt Enable
    pub SHIFTSIEN: RWRegister<u32>,

    /// Shifter Error Interrupt Enable
    pub SHIFTEIEN: RWRegister<u32>,

    /// Timer Interrupt Enable Register
    pub TIMIEN: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Shifter Status DMA Enable
    pub SHIFTSDEN: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Shifter State Register
    pub SHIFTSTATE: RWRegister<u32>,

    _reserved4: [u32; 15],

    /// Shifter Control N Register
    pub SHIFTCTL0: RWRegister<u32>,

    /// Shifter Control N Register
    pub SHIFTCTL1: RWRegister<u32>,

    /// Shifter Control N Register
    pub SHIFTCTL2: RWRegister<u32>,

    /// Shifter Control N Register
    pub SHIFTCTL3: RWRegister<u32>,

    _reserved5: [u32; 28],

    /// Shifter Configuration N Register
    pub SHIFTCFG0: RWRegister<u32>,

    /// Shifter Configuration N Register
    pub SHIFTCFG1: RWRegister<u32>,

    /// Shifter Configuration N Register
    pub SHIFTCFG2: RWRegister<u32>,

    /// Shifter Configuration N Register
    pub SHIFTCFG3: RWRegister<u32>,

    _reserved6: [u32; 60],

    /// Shifter Buffer N Register
    pub SHIFTBUF0: RWRegister<u32>,

    /// Shifter Buffer N Register
    pub SHIFTBUF1: RWRegister<u32>,

    /// Shifter Buffer N Register
    pub SHIFTBUF2: RWRegister<u32>,

    /// Shifter Buffer N Register
    pub SHIFTBUF3: RWRegister<u32>,

    _reserved7: [u32; 28],

    /// Shifter Buffer N Bit Swapped Register
    pub SHIFTBUFBIS0: RWRegister<u32>,

    /// Shifter Buffer N Bit Swapped Register
    pub SHIFTBUFBIS1: RWRegister<u32>,

    /// Shifter Buffer N Bit Swapped Register
    pub SHIFTBUFBIS2: RWRegister<u32>,

    /// Shifter Buffer N Bit Swapped Register
    pub SHIFTBUFBIS3: RWRegister<u32>,

    _reserved8: [u32; 28],

    /// Shifter Buffer N Byte Swapped Register
    pub SHIFTBUFBYS0: RWRegister<u32>,

    /// Shifter Buffer N Byte Swapped Register
    pub SHIFTBUFBYS1: RWRegister<u32>,

    /// Shifter Buffer N Byte Swapped Register
    pub SHIFTBUFBYS2: RWRegister<u32>,

    /// Shifter Buffer N Byte Swapped Register
    pub SHIFTBUFBYS3: RWRegister<u32>,

    _reserved9: [u32; 28],

    /// Shifter Buffer N Bit Byte Swapped Register
    pub SHIFTBUFBBS0: RWRegister<u32>,

    /// Shifter Buffer N Bit Byte Swapped Register
    pub SHIFTBUFBBS1: RWRegister<u32>,

    /// Shifter Buffer N Bit Byte Swapped Register
    pub SHIFTBUFBBS2: RWRegister<u32>,

    /// Shifter Buffer N Bit Byte Swapped Register
    pub SHIFTBUFBBS3: RWRegister<u32>,

    _reserved10: [u32; 28],

    /// Timer Control N Register
    pub TIMCTL0: RWRegister<u32>,

    /// Timer Control N Register
    pub TIMCTL1: RWRegister<u32>,

    /// Timer Control N Register
    pub TIMCTL2: RWRegister<u32>,

    /// Timer Control N Register
    pub TIMCTL3: RWRegister<u32>,

    _reserved11: [u32; 28],

    /// Timer Configuration N Register
    pub TIMCFG0: RWRegister<u32>,

    /// Timer Configuration N Register
    pub TIMCFG1: RWRegister<u32>,

    /// Timer Configuration N Register
    pub TIMCFG2: RWRegister<u32>,

    /// Timer Configuration N Register
    pub TIMCFG3: RWRegister<u32>,

    _reserved12: [u32; 28],

    /// Timer Compare N Register
    pub TIMCMP0: RWRegister<u32>,

    /// Timer Compare N Register
    pub TIMCMP1: RWRegister<u32>,

    /// Timer Compare N Register
    pub TIMCMP2: RWRegister<u32>,

    /// Timer Compare N Register
    pub TIMCMP3: RWRegister<u32>,

    _reserved13: [u32; 92],

    /// Shifter Buffer N Nibble Byte Swapped Register
    pub SHIFTBUFNBS0: RWRegister<u32>,

    /// Shifter Buffer N Nibble Byte Swapped Register
    pub SHIFTBUFNBS1: RWRegister<u32>,

    /// Shifter Buffer N Nibble Byte Swapped Register
    pub SHIFTBUFNBS2: RWRegister<u32>,

    /// Shifter Buffer N Nibble Byte Swapped Register
    pub SHIFTBUFNBS3: RWRegister<u32>,

    _reserved14: [u32; 28],

    /// Shifter Buffer N Half Word Swapped Register
    pub SHIFTBUFHWS0: RWRegister<u32>,

    /// Shifter Buffer N Half Word Swapped Register
    pub SHIFTBUFHWS1: RWRegister<u32>,

    /// Shifter Buffer N Half Word Swapped Register
    pub SHIFTBUFHWS2: RWRegister<u32>,

    /// Shifter Buffer N Half Word Swapped Register
    pub SHIFTBUFHWS3: RWRegister<u32>,

    _reserved15: [u32; 28],

    /// Shifter Buffer N Nibble Swapped Register
    pub SHIFTBUFNIS0: RWRegister<u32>,

    /// Shifter Buffer N Nibble Swapped Register
    pub SHIFTBUFNIS1: RWRegister<u32>,

    /// Shifter Buffer N Nibble Swapped Register
    pub SHIFTBUFNIS2: RWRegister<u32>,

    /// Shifter Buffer N Nibble Swapped Register
    pub SHIFTBUFNIS3: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub CTRL: u32,
    pub PIN: u32,
    pub SHIFTSTAT: u32,
    pub SHIFTERR: u32,
    pub TIMSTAT: u32,
    pub SHIFTSIEN: u32,
    pub SHIFTEIEN: u32,
    pub TIMIEN: u32,
    pub SHIFTSDEN: u32,
    pub SHIFTSTATE: u32,
    pub SHIFTCTL0: u32,
    pub SHIFTCTL1: u32,
    pub SHIFTCTL2: u32,
    pub SHIFTCTL3: u32,
    pub SHIFTCFG0: u32,
    pub SHIFTCFG1: u32,
    pub SHIFTCFG2: u32,
    pub SHIFTCFG3: u32,
    pub SHIFTBUF0: u32,
    pub SHIFTBUF1: u32,
    pub SHIFTBUF2: u32,
    pub SHIFTBUF3: u32,
    pub SHIFTBUFBIS0: u32,
    pub SHIFTBUFBIS1: u32,
    pub SHIFTBUFBIS2: u32,
    pub SHIFTBUFBIS3: u32,
    pub SHIFTBUFBYS0: u32,
    pub SHIFTBUFBYS1: u32,
    pub SHIFTBUFBYS2: u32,
    pub SHIFTBUFBYS3: u32,
    pub SHIFTBUFBBS0: u32,
    pub SHIFTBUFBBS1: u32,
    pub SHIFTBUFBBS2: u32,
    pub SHIFTBUFBBS3: u32,
    pub TIMCTL0: u32,
    pub TIMCTL1: u32,
    pub TIMCTL2: u32,
    pub TIMCTL3: u32,
    pub TIMCFG0: u32,
    pub TIMCFG1: u32,
    pub TIMCFG2: u32,
    pub TIMCFG3: u32,
    pub TIMCMP0: u32,
    pub TIMCMP1: u32,
    pub TIMCMP2: u32,
    pub TIMCMP3: u32,
    pub SHIFTBUFNBS0: u32,
    pub SHIFTBUFNBS1: u32,
    pub SHIFTBUFNBS2: u32,
    pub SHIFTBUFNBS3: u32,
    pub SHIFTBUFHWS0: u32,
    pub SHIFTBUFHWS1: u32,
    pub SHIFTBUFHWS2: u32,
    pub SHIFTBUFHWS3: u32,
    pub SHIFTBUFNIS0: u32,
    pub SHIFTBUFNIS1: u32,
    pub SHIFTBUFNIS2: u32,
    pub SHIFTBUFNIS3: u32,
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
