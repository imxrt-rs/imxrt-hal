#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! no description available

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTP Controller Control Register
pub mod HW_OCOTP_CTRL {

    /// OTP write and read access address register
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

    /// OTP controller status bit
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

    /// Set by the controller when an access to a locked region(OTP or shadow register) is requested
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

    /// Set to force re-loading the shadow registers (HW/SW capability and LOCK)
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

    /// Write 0x3E77 to enable OTP write accesses
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
pub mod HW_OCOTP_CTRL_SET {
    pub use super::HW_OCOTP_CTRL::ADDR;
    pub use super::HW_OCOTP_CTRL::BUSY;
    pub use super::HW_OCOTP_CTRL::ERROR;
    pub use super::HW_OCOTP_CTRL::RELOAD_SHADOWS;
    pub use super::HW_OCOTP_CTRL::WR_UNLOCK;
}

/// OTP Controller Control Register
pub mod HW_OCOTP_CTRL_CLR {
    pub use super::HW_OCOTP_CTRL::ADDR;
    pub use super::HW_OCOTP_CTRL::BUSY;
    pub use super::HW_OCOTP_CTRL::ERROR;
    pub use super::HW_OCOTP_CTRL::RELOAD_SHADOWS;
    pub use super::HW_OCOTP_CTRL::WR_UNLOCK;
}

/// OTP Controller Control Register
pub mod HW_OCOTP_CTRL_TOG {
    pub use super::HW_OCOTP_CTRL::ADDR;
    pub use super::HW_OCOTP_CTRL::BUSY;
    pub use super::HW_OCOTP_CTRL::ERROR;
    pub use super::HW_OCOTP_CTRL::RELOAD_SHADOWS;
    pub use super::HW_OCOTP_CTRL::WR_UNLOCK;
}

/// OTP Controller Timing Register
pub mod HW_OCOTP_TIMING {

    /// This count value specifies the strobe period in one time write OTP
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

    /// This count value specifies the time to add to all default timing parameters other than the Tpgm and Trd
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

    /// This count value specifies the strobe period in one time read OTP
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

    /// This count value specifies time interval between auto read and write access in one time program
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
}

/// OTP Controller Write Data Register
pub mod HW_OCOTP_DATA {

    /// Used to initiate a write to OTP
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
pub mod HW_OCOTP_READ_CTRL {

    /// Used to initiate a read to OTP
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
}

/// OTP Controller Read Data Register
pub mod HW_OCOTP_READ_FUSE_DATA {
    pub use super::HW_OCOTP_DATA::DATA;
}

/// Sticky bit Register
pub mod HW_OCOTP_SW_STICKY {

    /// Shadow register write and OTP write lock for SRK_REVOKE region
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

    /// Shadow register write and OTP write lock for FIELD_RETURN region
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
}

/// Software Controllable Signals Register
pub mod HW_OCOTP_SCS {

    /// HAB JTAG Debug Enable
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

    /// Unallocated read/write bits for implementation specific software use.
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

    /// When set, all of the bits in this register are locked and can not be changed through SW programming
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
pub mod HW_OCOTP_SCS_SET {
    pub use super::HW_OCOTP_SCS::HAB_JDE;
    pub use super::HW_OCOTP_SCS::LOCK;
    pub use super::HW_OCOTP_SCS::SPARE;
}

/// Software Controllable Signals Register
pub mod HW_OCOTP_SCS_CLR {
    pub use super::HW_OCOTP_SCS::HAB_JDE;
    pub use super::HW_OCOTP_SCS::LOCK;
    pub use super::HW_OCOTP_SCS::SPARE;
}

/// Software Controllable Signals Register
pub mod HW_OCOTP_SCS_TOG {
    pub use super::HW_OCOTP_SCS::HAB_JDE;
    pub use super::HW_OCOTP_SCS::LOCK;
    pub use super::HW_OCOTP_SCS::SPARE;
}

/// OTP Controller Version Register
pub mod HW_OCOTP_VERSION {

    /// Fixed read-only value reflecting the stepping of the RTL version.
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

    /// Fixed read-only value reflecting the MINOR field of the RTL version.
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

    /// Fixed read-only value reflecting the MAJOR field of the RTL version.
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

/// OTP Controller Timing Register 2
pub mod HW_OCOTP_TIMING2 {

    /// This count value specifies the strobe period in one time write OTP
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

    /// This count value specifies the strobe period in one time read OTP
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

    /// This count value specifies time interval between auto read and write access in one time program
    pub mod RELAX1 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (8 bits: 0xff << 22)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value of OTP Bank0 Word0 (Lock controls)
pub mod HW_OCOTP_LOCK {

    /// Status of shadow register and OTP write lock for tester region
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

    /// Status of shadow register and OTP write lock for boot_cfg region
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

    /// Status of shadow register and OTP write lock for mem_trim region
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

    /// Status of shadow register read and write, OTP read and write lock for sjc_resp region
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

    /// Status of shadow register and OTP write lock for mac_addr region
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

    /// Status of shadow register and OTP write lock for gp1 region
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

    /// Status of shadow register and OTP write lock for gp2 region
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

    /// Status of shadow register read and write, OTP read and write lock for otpmk region (MSB)
    pub mod OTPMK_MSB {
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

    /// Status of shadow register and OTP write lock for sw_gp1 region
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

    /// Status of shadow register read and write, OTP read and write lock for otpmk region (LSB)
    pub mod OTPMK_LSB {
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

    /// Status of shadow register and OTP write lock for analog region
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

    /// Status of shadow register and OTP write lock for otpmk_crc region
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

    /// Status of shadow register and OTP write lock for sw_gp2 region
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

    /// Status of shadow register and OTP write lock for misc_conf region
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

    /// Status of shadow register and OTP read lock for sw_gp2 region
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

    /// Status of shadow register and OTP write lock for gp3 region
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

    /// Reserved
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
pub mod HW_OCOTP_CFG0 {

    /// This register contains 32 bits of the Unique ID and SJC_CHALLENGE field
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
pub mod HW_OCOTP_CFG1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)
pub mod HW_OCOTP_CFG2 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)
pub mod HW_OCOTP_CFG3 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)
pub mod HW_OCOTP_CFG4 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)
pub mod HW_OCOTP_CFG5 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)
pub mod HW_OCOTP_CFG6 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word0 (Memory Related Info.)
pub mod HW_OCOTP_MEM0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word1 (Memory Related Info.)
pub mod HW_OCOTP_MEM1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word2 (Memory Related Info.)
pub mod HW_OCOTP_MEM2 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word3 (Memory Related Info.)
pub mod HW_OCOTP_MEM3 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word4 (Memory Related Info.)
pub mod HW_OCOTP_MEM4 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word5 (Analog Info.)
pub mod HW_OCOTP_ANA0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word6 (Analog Info.)
pub mod HW_OCOTP_ANA1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank1 Word7 (Analog Info.)
pub mod HW_OCOTP_ANA2 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word0 (SRK Hash)
pub mod HW_OCOTP_SRK0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word1 (SRK Hash)
pub mod HW_OCOTP_SRK1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word2 (SRK Hash)
pub mod HW_OCOTP_SRK2 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word3 (SRK Hash)
pub mod HW_OCOTP_SRK3 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word4 (SRK Hash)
pub mod HW_OCOTP_SRK4 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word5 (SRK Hash)
pub mod HW_OCOTP_SRK5 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word6 (SRK Hash)
pub mod HW_OCOTP_SRK6 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Shadow Register for OTP Bank3 Word7 (SRK Hash)
pub mod HW_OCOTP_SRK7 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word0 (Secure JTAG Response Field)
pub mod HW_OCOTP_SJC_RESP0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word1 (Secure JTAG Response Field)
pub mod HW_OCOTP_SJC_RESP1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word2 (MAC Address)
pub mod HW_OCOTP_MAC0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word3 (MAC Address)
pub mod HW_OCOTP_MAC1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word4 (MAC Address)
pub mod HW_OCOTP_GP3 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)
pub mod HW_OCOTP_GP1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)
pub mod HW_OCOTP_GP2 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word0 (SW GP1)
pub mod HW_OCOTP_SW_GP1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word1 (SW GP2)
pub mod HW_OCOTP_SW_GP20 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word2 (SW GP2)
pub mod HW_OCOTP_SW_GP21 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word3 (SW GP2)
pub mod HW_OCOTP_SW_GP22 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word4 (SW GP2)
pub mod HW_OCOTP_SW_GP23 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word5 (Misc Conf)
pub mod HW_OCOTP_MISC_CONF0 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word6 (Misc Conf)
pub mod HW_OCOTP_MISC_CONF1 {
    pub use super::HW_OCOTP_CFG0::BITS;
}

/// Value of OTP Bank5 Word7 (SRK Revoke)
pub mod HW_OCOTP_SRK_REVOKE {
    pub use super::HW_OCOTP_CFG0::BITS;
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTP Controller Control Register
    pub HW_OCOTP_CTRL: RWRegister<u32>,

    /// OTP Controller Control Register
    pub HW_OCOTP_CTRL_SET: RWRegister<u32>,

    /// OTP Controller Control Register
    pub HW_OCOTP_CTRL_CLR: RWRegister<u32>,

    /// OTP Controller Control Register
    pub HW_OCOTP_CTRL_TOG: RWRegister<u32>,

    /// OTP Controller Timing Register
    pub HW_OCOTP_TIMING: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// OTP Controller Write Data Register
    pub HW_OCOTP_DATA: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// OTP Controller Write Data Register
    pub HW_OCOTP_READ_CTRL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// OTP Controller Read Data Register
    pub HW_OCOTP_READ_FUSE_DATA: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// Sticky bit Register
    pub HW_OCOTP_SW_STICKY: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Software Controllable Signals Register
    pub HW_OCOTP_SCS: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub HW_OCOTP_SCS_SET: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub HW_OCOTP_SCS_CLR: RWRegister<u32>,

    /// Software Controllable Signals Register
    pub HW_OCOTP_SCS_TOG: RWRegister<u32>,

    _reserved6: [u32; 8],

    /// OTP Controller Version Register
    pub HW_OCOTP_VERSION: RORegister<u32>,

    _reserved7: [u32; 27],

    /// OTP Controller Timing Register 2
    pub HW_OCOTP_TIMING2: RWRegister<u32>,

    _reserved8: [u32; 191],

    /// Value of OTP Bank0 Word0 (Lock controls)
    pub HW_OCOTP_LOCK: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG0: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG1: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG2: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG3: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG4: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG5: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)
    pub HW_OCOTP_CFG6: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// Value of OTP Bank1 Word0 (Memory Related Info.)
    pub HW_OCOTP_MEM0: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// Value of OTP Bank1 Word1 (Memory Related Info.)
    pub HW_OCOTP_MEM1: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Value of OTP Bank1 Word2 (Memory Related Info.)
    pub HW_OCOTP_MEM2: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// Value of OTP Bank1 Word3 (Memory Related Info.)
    pub HW_OCOTP_MEM3: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// Value of OTP Bank1 Word4 (Memory Related Info.)
    pub HW_OCOTP_MEM4: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// Value of OTP Bank1 Word5 (Analog Info.)
    pub HW_OCOTP_ANA0: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// Value of OTP Bank1 Word6 (Analog Info.)
    pub HW_OCOTP_ANA1: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// Value of OTP Bank1 Word7 (Analog Info.)
    pub HW_OCOTP_ANA2: RWRegister<u32>,

    _reserved24: [u32; 35],

    /// Shadow Register for OTP Bank3 Word0 (SRK Hash)
    pub HW_OCOTP_SRK0: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// Shadow Register for OTP Bank3 Word1 (SRK Hash)
    pub HW_OCOTP_SRK1: RWRegister<u32>,

    _reserved26: [u32; 3],

    /// Shadow Register for OTP Bank3 Word2 (SRK Hash)
    pub HW_OCOTP_SRK2: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// Shadow Register for OTP Bank3 Word3 (SRK Hash)
    pub HW_OCOTP_SRK3: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// Shadow Register for OTP Bank3 Word4 (SRK Hash)
    pub HW_OCOTP_SRK4: RWRegister<u32>,

    _reserved29: [u32; 3],

    /// Shadow Register for OTP Bank3 Word5 (SRK Hash)
    pub HW_OCOTP_SRK5: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// Shadow Register for OTP Bank3 Word6 (SRK Hash)
    pub HW_OCOTP_SRK6: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// Shadow Register for OTP Bank3 Word7 (SRK Hash)
    pub HW_OCOTP_SRK7: RWRegister<u32>,

    _reserved32: [u32; 3],

    /// Value of OTP Bank4 Word0 (Secure JTAG Response Field)
    pub HW_OCOTP_SJC_RESP0: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// Value of OTP Bank4 Word1 (Secure JTAG Response Field)
    pub HW_OCOTP_SJC_RESP1: RWRegister<u32>,

    _reserved34: [u32; 3],

    /// Value of OTP Bank4 Word2 (MAC Address)
    pub HW_OCOTP_MAC0: RWRegister<u32>,

    _reserved35: [u32; 3],

    /// Value of OTP Bank4 Word3 (MAC Address)
    pub HW_OCOTP_MAC1: RWRegister<u32>,

    _reserved36: [u32; 3],

    /// Value of OTP Bank4 Word4 (MAC Address)
    pub HW_OCOTP_GP3: RWRegister<u32>,

    _reserved37: [u32; 7],

    /// Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)
    pub HW_OCOTP_GP1: RWRegister<u32>,

    _reserved38: [u32; 3],

    /// Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)
    pub HW_OCOTP_GP2: RWRegister<u32>,

    _reserved39: [u32; 3],

    /// Value of OTP Bank5 Word0 (SW GP1)
    pub HW_OCOTP_SW_GP1: RWRegister<u32>,

    _reserved40: [u32; 3],

    /// Value of OTP Bank5 Word1 (SW GP2)
    pub HW_OCOTP_SW_GP20: RWRegister<u32>,

    _reserved41: [u32; 3],

    /// Value of OTP Bank5 Word2 (SW GP2)
    pub HW_OCOTP_SW_GP21: RWRegister<u32>,

    _reserved42: [u32; 3],

    /// Value of OTP Bank5 Word3 (SW GP2)
    pub HW_OCOTP_SW_GP22: RWRegister<u32>,

    _reserved43: [u32; 3],

    /// Value of OTP Bank5 Word4 (SW GP2)
    pub HW_OCOTP_SW_GP23: RWRegister<u32>,

    _reserved44: [u32; 3],

    /// Value of OTP Bank5 Word5 (Misc Conf)
    pub HW_OCOTP_MISC_CONF0: RWRegister<u32>,

    _reserved45: [u32; 3],

    /// Value of OTP Bank5 Word6 (Misc Conf)
    pub HW_OCOTP_MISC_CONF1: RWRegister<u32>,

    _reserved46: [u32; 3],

    /// Value of OTP Bank5 Word7 (SRK Revoke)
    pub HW_OCOTP_SRK_REVOKE: RWRegister<u32>,
}
pub struct ResetValues {
    pub HW_OCOTP_CTRL: u32,
    pub HW_OCOTP_CTRL_SET: u32,
    pub HW_OCOTP_CTRL_CLR: u32,
    pub HW_OCOTP_CTRL_TOG: u32,
    pub HW_OCOTP_TIMING: u32,
    pub HW_OCOTP_DATA: u32,
    pub HW_OCOTP_READ_CTRL: u32,
    pub HW_OCOTP_READ_FUSE_DATA: u32,
    pub HW_OCOTP_SW_STICKY: u32,
    pub HW_OCOTP_SCS: u32,
    pub HW_OCOTP_SCS_SET: u32,
    pub HW_OCOTP_SCS_CLR: u32,
    pub HW_OCOTP_SCS_TOG: u32,
    pub HW_OCOTP_VERSION: u32,
    pub HW_OCOTP_TIMING2: u32,
    pub HW_OCOTP_LOCK: u32,
    pub HW_OCOTP_CFG0: u32,
    pub HW_OCOTP_CFG1: u32,
    pub HW_OCOTP_CFG2: u32,
    pub HW_OCOTP_CFG3: u32,
    pub HW_OCOTP_CFG4: u32,
    pub HW_OCOTP_CFG5: u32,
    pub HW_OCOTP_CFG6: u32,
    pub HW_OCOTP_MEM0: u32,
    pub HW_OCOTP_MEM1: u32,
    pub HW_OCOTP_MEM2: u32,
    pub HW_OCOTP_MEM3: u32,
    pub HW_OCOTP_MEM4: u32,
    pub HW_OCOTP_ANA0: u32,
    pub HW_OCOTP_ANA1: u32,
    pub HW_OCOTP_ANA2: u32,
    pub HW_OCOTP_SRK0: u32,
    pub HW_OCOTP_SRK1: u32,
    pub HW_OCOTP_SRK2: u32,
    pub HW_OCOTP_SRK3: u32,
    pub HW_OCOTP_SRK4: u32,
    pub HW_OCOTP_SRK5: u32,
    pub HW_OCOTP_SRK6: u32,
    pub HW_OCOTP_SRK7: u32,
    pub HW_OCOTP_SJC_RESP0: u32,
    pub HW_OCOTP_SJC_RESP1: u32,
    pub HW_OCOTP_MAC0: u32,
    pub HW_OCOTP_MAC1: u32,
    pub HW_OCOTP_GP3: u32,
    pub HW_OCOTP_GP1: u32,
    pub HW_OCOTP_GP2: u32,
    pub HW_OCOTP_SW_GP1: u32,
    pub HW_OCOTP_SW_GP20: u32,
    pub HW_OCOTP_SW_GP21: u32,
    pub HW_OCOTP_SW_GP22: u32,
    pub HW_OCOTP_SW_GP23: u32,
    pub HW_OCOTP_MISC_CONF0: u32,
    pub HW_OCOTP_MISC_CONF1: u32,
    pub HW_OCOTP_SRK_REVOKE: u32,
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

/// Access functions for the OCOTP peripheral instance
pub mod OCOTP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401f4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCOTP
    pub const reset: ResetValues = ResetValues {
        HW_OCOTP_CTRL: 0x00000000,
        HW_OCOTP_CTRL_SET: 0x00000000,
        HW_OCOTP_CTRL_CLR: 0x00000000,
        HW_OCOTP_CTRL_TOG: 0x00000000,
        HW_OCOTP_TIMING: 0x060D9755,
        HW_OCOTP_DATA: 0x00000000,
        HW_OCOTP_READ_CTRL: 0x00000000,
        HW_OCOTP_READ_FUSE_DATA: 0x00000000,
        HW_OCOTP_SW_STICKY: 0x00000000,
        HW_OCOTP_SCS: 0x00000000,
        HW_OCOTP_SCS_SET: 0x00000000,
        HW_OCOTP_SCS_CLR: 0x00000000,
        HW_OCOTP_SCS_TOG: 0x00000000,
        HW_OCOTP_VERSION: 0x06000000,
        HW_OCOTP_TIMING2: 0x01C30092,
        HW_OCOTP_LOCK: 0x00000000,
        HW_OCOTP_CFG0: 0x00000000,
        HW_OCOTP_CFG1: 0x00000000,
        HW_OCOTP_CFG2: 0x00000000,
        HW_OCOTP_CFG3: 0x00000000,
        HW_OCOTP_CFG4: 0x00000000,
        HW_OCOTP_CFG5: 0x00000000,
        HW_OCOTP_CFG6: 0x00000000,
        HW_OCOTP_MEM0: 0x00000000,
        HW_OCOTP_MEM1: 0x00000000,
        HW_OCOTP_MEM2: 0x00000000,
        HW_OCOTP_MEM3: 0x00000000,
        HW_OCOTP_MEM4: 0x00000000,
        HW_OCOTP_ANA0: 0x00000000,
        HW_OCOTP_ANA1: 0x00000000,
        HW_OCOTP_ANA2: 0x00000000,
        HW_OCOTP_SRK0: 0x00000000,
        HW_OCOTP_SRK1: 0x00000000,
        HW_OCOTP_SRK2: 0x00000000,
        HW_OCOTP_SRK3: 0x00000000,
        HW_OCOTP_SRK4: 0x00000000,
        HW_OCOTP_SRK5: 0x00000000,
        HW_OCOTP_SRK6: 0x00000000,
        HW_OCOTP_SRK7: 0x00000000,
        HW_OCOTP_SJC_RESP0: 0x00000000,
        HW_OCOTP_SJC_RESP1: 0x00000000,
        HW_OCOTP_MAC0: 0x00000000,
        HW_OCOTP_MAC1: 0x00000000,
        HW_OCOTP_GP3: 0x00000000,
        HW_OCOTP_GP1: 0x00000000,
        HW_OCOTP_GP2: 0x00000000,
        HW_OCOTP_SW_GP1: 0x00000000,
        HW_OCOTP_SW_GP20: 0x00000000,
        HW_OCOTP_SW_GP21: 0x00000000,
        HW_OCOTP_SW_GP22: 0x00000000,
        HW_OCOTP_SW_GP23: 0x00000000,
        HW_OCOTP_MISC_CONF0: 0x00000000,
        HW_OCOTP_MISC_CONF1: 0x00000000,
        HW_OCOTP_SRK_REVOKE: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCOTP_TAKEN: bool = false;

    /// Safe access to OCOTP
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
            if OCOTP_TAKEN {
                None
            } else {
                OCOTP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCOTP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCOTP_TAKEN && inst.addr == INSTANCE.addr {
                OCOTP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCOTP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCOTP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCOTP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCOTP: *const RegisterBlock = 0x401f4000 as *const _;
