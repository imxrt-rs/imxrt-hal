#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OCOTP
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTP Controller Control Register
pub mod CTRL {

    /// ADDR
    pub mod ADDR {
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

    /// RSVD0
    pub mod RSVD0 {
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

    /// BUSY
    pub mod BUSY {
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

    /// ERROR
    pub mod ERROR {
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

    /// RELOAD_SHADOWS
    pub mod RELOAD_SHADOWS {
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

    /// CRC_TEST
    pub mod CRC_TEST {
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

    /// CRC_FAIL
    pub mod CRC_FAIL {
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

    /// RSVD1
    pub mod RSVD1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_UNLOCK
    pub mod WR_UNLOCK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0011111001110111: Key needed to unlock HW_OCOTP_DATA register.
            pub const KEY: u32 = 0b0011111001110111;
        }
    }
}

/// OTP Controller Control Register
pub mod CTRL_SET {

    /// ADDR
    pub mod ADDR {
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

    /// RSVD0
    pub mod RSVD0 {
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

    /// BUSY
    pub mod BUSY {
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

    /// ERROR
    pub mod ERROR {
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

    /// RELOAD_SHADOWS
    pub mod RELOAD_SHADOWS {
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

    /// CRC_TEST
    pub mod CRC_TEST {
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

    /// CRC_FAIL
    pub mod CRC_FAIL {
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

    /// RSVD1
    pub mod RSVD1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_UNLOCK
    pub mod WR_UNLOCK {
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

/// OTP Controller Control Register
pub mod CTRL_CLR {
    pub use super::CTRL_SET::ADDR;
    pub use super::CTRL_SET::BUSY;
    pub use super::CTRL_SET::CRC_FAIL;
    pub use super::CTRL_SET::CRC_TEST;
    pub use super::CTRL_SET::ERROR;
    pub use super::CTRL_SET::RELOAD_SHADOWS;
    pub use super::CTRL_SET::RSVD0;
    pub use super::CTRL_SET::RSVD1;
    pub use super::CTRL_SET::WR_UNLOCK;
}

/// OTP Controller Control Register
pub mod CTRL_TOG {
    pub use super::CTRL_SET::ADDR;
    pub use super::CTRL_SET::BUSY;
    pub use super::CTRL_SET::CRC_FAIL;
    pub use super::CTRL_SET::CRC_TEST;
    pub use super::CTRL_SET::ERROR;
    pub use super::CTRL_SET::RELOAD_SHADOWS;
    pub use super::CTRL_SET::RSVD0;
    pub use super::CTRL_SET::RSVD1;
    pub use super::CTRL_SET::WR_UNLOCK;
}

/// OTP Controller Timing Register
pub mod TIMING {

    /// STROBE_PROG
    pub mod STROBE_PROG {
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

    /// RELAX
    pub mod RELAX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STROBE_READ
    pub mod STROBE_READ {
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

    /// WAIT
    pub mod WAIT {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (6 bits: 0x3f << 22)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RSRVD0
    pub mod RSRVD0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTP Controller Write Data Register
pub mod DATA {

    /// DATA
    pub mod DATA {
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

/// OTP Controller Write Data Register
pub mod READ_CTRL {

    /// READ_FUSE
    pub mod READ_FUSE {
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

    /// RSVD0
    pub mod RSVD0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (31 bits: 0x7fffffff << 1)
        pub const mask: u32 = 0x7fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTP Controller Read Data Register
pub mod READ_FUSE_DATA {
    pub use super::DATA::DATA;
}

/// Sticky bit Register
pub mod SW_STICKY {

    /// BLOCK_DTCP_KEY
    pub mod BLOCK_DTCP_KEY {
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

    /// SRK_REVOKE_LOCK
    pub mod SRK_REVOKE_LOCK {
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

    /// FIELD_RETURN_LOCK
    pub mod FIELD_RETURN_LOCK {
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

    /// BLOCK_ROM_PART
    pub mod BLOCK_ROM_PART {
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

    /// JTAG_BLOCK_RELEASE
    pub mod JTAG_BLOCK_RELEASE {
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

    /// RSVD0
    pub mod RSVD0 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (27 bits: 0x7ffffff << 5)
        pub const mask: u32 = 0x7ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Software Controllable Signals Register
pub mod SCS {

    /// HAB_JDE
    pub mod HAB_JDE {
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

    /// SPARE
    pub mod SPARE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (30 bits: 0x3fffffff << 1)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
    pub mod LOCK {
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

/// Software Controllable Signals Register
pub mod SCS_SET {
    pub use super::SCS::HAB_JDE;
    pub use super::SCS::LOCK;
    pub use super::SCS::SPARE;
}

/// Software Controllable Signals Register
pub mod SCS_CLR {
    pub use super::SCS::HAB_JDE;
    pub use super::SCS::LOCK;
    pub use super::SCS::SPARE;
}

/// Software Controllable Signals Register
pub mod SCS_TOG {
    pub use super::SCS::HAB_JDE;
    pub use super::SCS::LOCK;
    pub use super::SCS::SPARE;
}

/// OTP Controller CRC test address
pub mod CRC_ADDR {

    /// DATA_START_ADDR
    pub mod DATA_START_ADDR {
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

    /// DATA_END_ADDR
    pub mod DATA_END_ADDR {
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

    /// CRC_ADDR
    pub mod CRC_ADDR {
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

    /// OTPMK_CRC
    pub mod OTPMK_CRC {
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

    /// RSVD0
    pub mod RSVD0 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (7 bits: 0x7f << 25)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTP Controller CRC Value Register
pub mod CRC_VALUE {
    pub use super::DATA::DATA;
}

/// OTP Controller Version Register
pub mod VERSION {

    /// STEP
    pub mod STEP {
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

    /// MINOR
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

    /// MAJOR
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

/// OTP Controller Timing Register
pub mod TIMING2 {

    /// RELAX_PROG
    pub mod RELAX_PROG {
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

    /// RSRVD0
    pub mod RSRVD0 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RELAX_READ
    pub mod RELAX_READ {
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

    /// RSRVD0
    pub mod RSRVD1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (10 bits: 0x3ff << 22)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value of OTP Bank0 Word0 (Lock controls)
pub mod LOCK {

    /// TESTER
    pub mod TESTER {
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

    /// BOOT_CFG
    pub mod BOOT_CFG {
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

    /// MEM_TRIM
    pub mod MEM_TRIM {
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

    /// SJC_RESP
    pub mod SJC_RESP {
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

    /// GP4_RLOCK
    pub mod GP4_RLOCK {
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

    /// MAC_ADDR
    pub mod MAC_ADDR {
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

    /// GP1
    pub mod GP1 {
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

    /// GP2
    pub mod GP2 {
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

    /// ROM_PATCH
    pub mod ROM_PATCH {
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

    /// SW_GP1
    pub mod SW_GP1 {
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

    /// OTPMK
    pub mod OTPMK {
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

    /// ANALOG
    pub mod ANALOG {
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

    /// OTPMK_CRC
    pub mod OTPMK_CRC {
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

    /// SW_GP2_LOCK
    pub mod SW_GP2_LOCK {
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

    /// MISC_CONF
    pub mod MISC_CONF {
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

    /// SW_GP2_RLOCK
    pub mod SW_GP2_RLOCK {
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

    /// GP4
    pub mod GP4 {
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

    /// GP3
    pub mod GP3 {
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

    /// FIELD_RETURN
    pub mod FIELD_RETURN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)
pub mod CFG0 {

    /// BITS
    pub mod BITS {
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

/// Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)
pub mod CFG1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)
pub mod CFG2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)
pub mod CFG3 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)
pub mod CFG4 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)
pub mod CFG5 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)
pub mod CFG6 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word0 (Memory Related Info.)
pub mod MEM0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word1 (Memory Related Info.)
pub mod MEM1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word2 (Memory Related Info.)
pub mod MEM2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word3 (Memory Related Info.)
pub mod MEM3 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word4 (Memory Related Info.)
pub mod MEM4 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word5 (Memory Related Info.)
pub mod ANA0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)
pub mod ANA1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)
pub mod ANA2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word0 (OTPMK Key)
pub mod OTPMK0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word1 (OTPMK Key)
pub mod OTPMK1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word2 (OTPMK Key)
pub mod OTPMK2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word3 (OTPMK Key)
pub mod OTPMK3 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word4 (OTPMK Key)
pub mod OTPMK4 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word5 (OTPMK Key)
pub mod OTPMK5 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word6 (OTPMK Key)
pub mod OTPMK6 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank2 Word7 (OTPMK Key)
pub mod OTPMK7 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word0 (SRK Hash)
pub mod SRK0 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word1 (SRK Hash)
pub mod SRK1 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word2 (SRK Hash)
pub mod SRK2 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word3 (SRK Hash)
pub mod SRK3 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word4 (SRK Hash)
pub mod SRK4 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word5 (SRK Hash)
pub mod SRK5 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word6 (SRK Hash)
pub mod SRK6 {
    pub use super::CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word7 (SRK Hash)
pub mod SRK7 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word0 (Secure JTAG Response Field)
pub mod SJC_RESP0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word1 (Secure JTAG Response Field)
pub mod SJC_RESP1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word2 (MAC Address)
pub mod MAC0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word3 (MAC Address)
pub mod MAC1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word4 (MAC2 Address)
pub mod MAC2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word5 (CRC Key)
pub mod OTPMK_CRC32 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)
pub mod GP1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)
pub mod GP2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word0 (SW GP1)
pub mod SW_GP1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word1 (SW GP2)
pub mod SW_GP20 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word2 (SW GP2)
pub mod SW_GP21 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word3 (SW GP2)
pub mod SW_GP22 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word4 (SW GP2)
pub mod SW_GP23 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word5 (Misc Conf)
pub mod MISC_CONF0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word6 (Misc Conf)
pub mod MISC_CONF1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank5 Word7 (SRK Revoke)
pub mod SRK_REVOKE {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word0 (ROM Patch)
pub mod ROM_PATCH0 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word1 (ROM Patch)
pub mod ROM_PATCH1 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word2 (ROM Patch)
pub mod ROM_PATCH2 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word3 (ROM Patch)
pub mod ROM_PATCH3 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word4 (ROM Patch)
pub mod ROM_PATCH4 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word5 (ROM Patch)
pub mod ROM_PATCH5 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word6 (ROM Patch)
pub mod ROM_PATCH6 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank6 Word7 (ROM Patch)
pub mod ROM_PATCH7 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word0 (GP3)
pub mod GP30 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word1 (GP3)
pub mod GP31 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word2 (GP3)
pub mod GP32 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word3 (GP3)
pub mod GP33 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word4 (GP4)
pub mod GP40 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word5 (GP4)
pub mod GP41 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word6 (GP4)
pub mod GP42 {
    pub use super::CFG0::BITS;
}

/// Value of OTP Bank7 Word7 (GP4)
pub mod GP43 {
    pub use super::CFG0::BITS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTP Controller Control Register
    pub CTRL: RWRegister<u32>,

    /// OTP Controller Control Register
    pub CTRL_SET: RWRegister<u32>,

    /// OTP Controller Control Register
    pub CTRL_CLR: RWRegister<u32>,

    /// OTP Controller Control Register
    pub CTRL_TOG: RWRegister<u32>,

    /// OTP Controller Timing Register
    pub TIMING: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// OTP Controller Write Data Register
    pub DATA: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// OTP Controller Write Data Register
    pub READ_CTRL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// OTP Controller Read Data Register
    pub READ_FUSE_DATA: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// Sticky bit Register
    pub SW_STICKY: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Software Controllable Signals Register
    pub SCS: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub SCS_SET: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub SCS_CLR: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub SCS_TOG: RWRegister<u32>,

    /// OTP Controller CRC test address
    pub CRC_ADDR: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// OTP Controller CRC Value Register
    pub CRC_VALUE: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// OTP Controller Version Register
    pub VERSION: RORegister<u32>,

    _reserved8: [u32; 27],

    /// OTP Controller Timing Register
    pub TIMING2: RWRegister<u32>,

    _reserved9: [u32; 191],

    /// Value of OTP Bank0 Word0 (Lock controls)
    pub LOCK: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)
    pub CFG0: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)
    pub CFG1: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)
    pub CFG2: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)
    pub CFG3: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)
    pub CFG4: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)
    pub CFG5: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)
    pub CFG6: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// Value of OTP Bank1 Word0 (Memory Related Info.)
    pub MEM0: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Value of OTP Bank1 Word1 (Memory Related Info.)
    pub MEM1: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// Value of OTP Bank1 Word2 (Memory Related Info.)
    pub MEM2: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// Value of OTP Bank1 Word3 (Memory Related Info.)
    pub MEM3: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// Value of OTP Bank1 Word4 (Memory Related Info.)
    pub MEM4: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// Value of OTP Bank1 Word5 (Memory Related Info.)
    pub ANA0: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// Value of OTP Bank1 Word6 (General Purpose Customer Defined Info.)
    pub ANA1: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// Value of OTP Bank1 Word7 (General Purpose Customer Defined Info.)
    pub ANA2: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// Value of OTP Bank2 Word0 (OTPMK Key)
    pub OTPMK0: RWRegister<u32>,

    _reserved26: [u32; 3],

    /// Value of OTP Bank2 Word1 (OTPMK Key)
    pub OTPMK1: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// Value of OTP Bank2 Word2 (OTPMK Key)
    pub OTPMK2: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// Value of OTP Bank2 Word3 (OTPMK Key)
    pub OTPMK3: RWRegister<u32>,

    _reserved29: [u32; 3],

    /// Value of OTP Bank2 Word4 (OTPMK Key)
    pub OTPMK4: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// Value of OTP Bank2 Word5 (OTPMK Key)
    pub OTPMK5: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// Value of OTP Bank2 Word6 (OTPMK Key)
    pub OTPMK6: RWRegister<u32>,

    _reserved32: [u32; 3],

    /// Value of OTP Bank2 Word7 (OTPMK Key)
    pub OTPMK7: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// Shadow Register for OTP Bank3 Word0 (SRK Hash)
    pub SRK0: RWRegister<u32>,

    _reserved34: [u32; 3],

    /// Shadow Register for OTP Bank3 Word1 (SRK Hash)
    pub SRK1: RWRegister<u32>,

    _reserved35: [u32; 3],

    /// Shadow Register for OTP Bank3 Word2 (SRK Hash)
    pub SRK2: RWRegister<u32>,

    _reserved36: [u32; 3],

    /// Shadow Register for OTP Bank3 Word3 (SRK Hash)
    pub SRK3: RWRegister<u32>,

    _reserved37: [u32; 3],

    /// Shadow Register for OTP Bank3 Word4 (SRK Hash)
    pub SRK4: RWRegister<u32>,

    _reserved38: [u32; 3],

    /// Shadow Register for OTP Bank3 Word5 (SRK Hash)
    pub SRK5: RWRegister<u32>,

    _reserved39: [u32; 3],

    /// Shadow Register for OTP Bank3 Word6 (SRK Hash)
    pub SRK6: RWRegister<u32>,

    _reserved40: [u32; 3],

    /// Shadow Register for OTP Bank3 Word7 (SRK Hash)
    pub SRK7: RWRegister<u32>,

    _reserved41: [u32; 3],

    /// Value of OTP Bank4 Word0 (Secure JTAG Response Field)
    pub SJC_RESP0: RWRegister<u32>,

    _reserved42: [u32; 3],

    /// Value of OTP Bank4 Word1 (Secure JTAG Response Field)
    pub SJC_RESP1: RWRegister<u32>,

    _reserved43: [u32; 3],

    /// Value of OTP Bank4 Word2 (MAC Address)
    pub MAC0: RWRegister<u32>,

    _reserved44: [u32; 3],

    /// Value of OTP Bank4 Word3 (MAC Address)
    pub MAC1: RWRegister<u32>,

    _reserved45: [u32; 3],

    /// Value of OTP Bank4 Word4 (MAC2 Address)
    pub MAC2: RWRegister<u32>,

    _reserved46: [u32; 3],

    /// Value of OTP Bank4 Word5 (CRC Key)
    pub OTPMK_CRC32: RWRegister<u32>,

    _reserved47: [u32; 3],

    /// Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)
    pub GP1: RWRegister<u32>,

    _reserved48: [u32; 3],

    /// Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)
    pub GP2: RWRegister<u32>,

    _reserved49: [u32; 3],

    /// Value of OTP Bank5 Word0 (SW GP1)
    pub SW_GP1: RWRegister<u32>,

    _reserved50: [u32; 3],

    /// Value of OTP Bank5 Word1 (SW GP2)
    pub SW_GP20: RWRegister<u32>,

    _reserved51: [u32; 3],

    /// Value of OTP Bank5 Word2 (SW GP2)
    pub SW_GP21: RWRegister<u32>,

    _reserved52: [u32; 3],

    /// Value of OTP Bank5 Word3 (SW GP2)
    pub SW_GP22: RWRegister<u32>,

    _reserved53: [u32; 3],

    /// Value of OTP Bank5 Word4 (SW GP2)
    pub SW_GP23: RWRegister<u32>,

    _reserved54: [u32; 3],

    /// Value of OTP Bank5 Word5 (Misc Conf)
    pub MISC_CONF0: RWRegister<u32>,

    _reserved55: [u32; 3],

    /// Value of OTP Bank5 Word6 (Misc Conf)
    pub MISC_CONF1: RWRegister<u32>,

    _reserved56: [u32; 3],

    /// Value of OTP Bank5 Word7 (SRK Revoke)
    pub SRK_REVOKE: RWRegister<u32>,

    _reserved57: [u32; 67],

    /// Value of OTP Bank6 Word0 (ROM Patch)
    pub ROM_PATCH0: RWRegister<u32>,

    _reserved58: [u32; 3],

    /// Value of OTP Bank6 Word1 (ROM Patch)
    pub ROM_PATCH1: RWRegister<u32>,

    _reserved59: [u32; 3],

    /// Value of OTP Bank6 Word2 (ROM Patch)
    pub ROM_PATCH2: RWRegister<u32>,

    _reserved60: [u32; 3],

    /// Value of OTP Bank6 Word3 (ROM Patch)
    pub ROM_PATCH3: RWRegister<u32>,

    _reserved61: [u32; 3],

    /// Value of OTP Bank6 Word4 (ROM Patch)
    pub ROM_PATCH4: RWRegister<u32>,

    _reserved62: [u32; 3],

    /// Value of OTP Bank6 Word5 (ROM Patch)
    pub ROM_PATCH5: RWRegister<u32>,

    _reserved63: [u32; 3],

    /// Value of OTP Bank6 Word6 (ROM Patch)
    pub ROM_PATCH6: RWRegister<u32>,

    _reserved64: [u32; 3],

    /// Value of OTP Bank6 Word7 (ROM Patch)
    pub ROM_PATCH7: RWRegister<u32>,

    _reserved65: [u32; 3],

    /// Value of OTP Bank7 Word0 (GP3)
    pub GP30: RWRegister<u32>,

    _reserved66: [u32; 3],

    /// Value of OTP Bank7 Word1 (GP3)
    pub GP31: RWRegister<u32>,

    _reserved67: [u32; 3],

    /// Value of OTP Bank7 Word2 (GP3)
    pub GP32: RWRegister<u32>,

    _reserved68: [u32; 3],

    /// Value of OTP Bank7 Word3 (GP3)
    pub GP33: RWRegister<u32>,

    _reserved69: [u32; 3],

    /// Value of OTP Bank7 Word4 (GP4)
    pub GP40: RWRegister<u32>,

    _reserved70: [u32; 3],

    /// Value of OTP Bank7 Word5 (GP4)
    pub GP41: RWRegister<u32>,

    _reserved71: [u32; 3],

    /// Value of OTP Bank7 Word6 (GP4)
    pub GP42: RWRegister<u32>,

    _reserved72: [u32; 3],

    /// Value of OTP Bank7 Word7 (GP4)
    pub GP43: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub TIMING: u32,
    pub DATA: u32,
    pub READ_CTRL: u32,
    pub READ_FUSE_DATA: u32,
    pub SW_STICKY: u32,
    pub SCS: u32,
    pub SCS_SET: u32,
    pub SCS_CLR: u32,
    pub SCS_TOG: u32,
    pub CRC_ADDR: u32,
    pub CRC_VALUE: u32,
    pub VERSION: u32,
    pub TIMING2: u32,
    pub LOCK: u32,
    pub CFG0: u32,
    pub CFG1: u32,
    pub CFG2: u32,
    pub CFG3: u32,
    pub CFG4: u32,
    pub CFG5: u32,
    pub CFG6: u32,
    pub MEM0: u32,
    pub MEM1: u32,
    pub MEM2: u32,
    pub MEM3: u32,
    pub MEM4: u32,
    pub ANA0: u32,
    pub ANA1: u32,
    pub ANA2: u32,
    pub OTPMK0: u32,
    pub OTPMK1: u32,
    pub OTPMK2: u32,
    pub OTPMK3: u32,
    pub OTPMK4: u32,
    pub OTPMK5: u32,
    pub OTPMK6: u32,
    pub OTPMK7: u32,
    pub SRK0: u32,
    pub SRK1: u32,
    pub SRK2: u32,
    pub SRK3: u32,
    pub SRK4: u32,
    pub SRK5: u32,
    pub SRK6: u32,
    pub SRK7: u32,
    pub SJC_RESP0: u32,
    pub SJC_RESP1: u32,
    pub MAC0: u32,
    pub MAC1: u32,
    pub MAC2: u32,
    pub OTPMK_CRC32: u32,
    pub GP1: u32,
    pub GP2: u32,
    pub SW_GP1: u32,
    pub SW_GP20: u32,
    pub SW_GP21: u32,
    pub SW_GP22: u32,
    pub SW_GP23: u32,
    pub MISC_CONF0: u32,
    pub MISC_CONF1: u32,
    pub SRK_REVOKE: u32,
    pub ROM_PATCH0: u32,
    pub ROM_PATCH1: u32,
    pub ROM_PATCH2: u32,
    pub ROM_PATCH3: u32,
    pub ROM_PATCH4: u32,
    pub ROM_PATCH5: u32,
    pub ROM_PATCH6: u32,
    pub ROM_PATCH7: u32,
    pub GP30: u32,
    pub GP31: u32,
    pub GP32: u32,
    pub GP33: u32,
    pub GP40: u32,
    pub GP41: u32,
    pub GP42: u32,
    pub GP43: u32,
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
