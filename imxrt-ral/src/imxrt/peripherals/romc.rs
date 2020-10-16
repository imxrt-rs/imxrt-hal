#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ROMC
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ROMC Data Registers
pub mod ROMPATCH7D {

    /// Data Fix Registers - Stores the data used for 1-word data fix operations
    pub mod DATAX {
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

/// ROMC Data Registers
pub mod ROMPATCH6D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH5D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH4D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH3D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH2D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH1D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Data Registers
pub mod ROMPATCH0D {
    pub use super::ROMPATCH7D::DATAX;
}

/// ROMC Control Register
pub mod ROMPATCHCNTL {

    /// Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine
    pub mod DATAFIX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Address comparator triggers a opcode patch
            pub const DATAFIX_0: u32 = 0b00000000;

            /// 0b00000001: Address comparator triggers a data fix
            pub const DATAFIX_1: u32 = 0b00000001;
        }
    }

    /// ROMC Disable -- This bit, when set, disables all ROMC operations
    pub mod DIS {
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

            /// 0b0: Does not affect any ROMC functions (default)
            pub const DIS_0: u32 = 0b0;

            /// 0b1: Disable all ROMC functions: data fixing, and opcode patching
            pub const DIS_1: u32 = 0b1;
        }
    }
}

/// ROMC Enable Register High
pub mod ROMPATCHENH {}

/// ROMC Enable Register Low
pub mod ROMPATCHENL {

    /// Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event
    pub mod ENABLE {
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

            /// 0b0000000000000000: Address comparator disabled
            pub const ENABLE_0: u32 = 0b0000000000000000;

            /// 0b0000000000000001: Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address
            pub const ENABLE_1: u32 = 0b0000000000000001;
        }
    }
}

/// ROMC Address Registers
pub mod ROMPATCH0A {

    /// THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch
    pub mod THUMBX {
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

            /// 0b0: Arm patch
            pub const THUMBX_0: u32 = 0b0;

            /// 0b1: THUMB patch (ignore if data fix)
            pub const THUMBX_1: u32 = 0b1;
        }
    }

    /// Address Comparator Registers - Indicates the memory address to be watched
    pub mod ADDRX {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (22 bits: 0x3fffff << 1)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ROMC Address Registers
pub mod ROMPATCH1A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH2A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH3A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH4A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH5A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH6A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH7A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH8A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH9A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH10A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH11A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH12A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH13A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH14A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Address Registers
pub mod ROMPATCH15A {
    pub use super::ROMPATCH0A::ADDRX;
    pub use super::ROMPATCH0A::THUMBX;
}

/// ROMC Status Register
pub mod ROMPATCHSR {

    /// ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB
    pub mod SOURCE {
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

            /// 0b000000: Address Comparator 0 matched
            pub const SOURCE_0: u32 = 0b000000;

            /// 0b000001: Address Comparator 1 matched
            pub const SOURCE_1: u32 = 0b000001;

            /// 0b001111: Address Comparator 15 matched
            pub const SOURCE_15: u32 = 0b001111;
        }
    }

    /// ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred
    pub mod SW {
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

            /// 0b0: no event or comparator collisions
            pub const SW_0: u32 = 0b0;

            /// 0b1: a collision has occurred
            pub const SW_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 53],

    /// ROMC Data Registers
    pub ROMPATCH7D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH6D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH5D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH4D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH3D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH2D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH1D: RWRegister<u32>,

    /// ROMC Data Registers
    pub ROMPATCH0D: RWRegister<u32>,

    /// ROMC Control Register
    pub ROMPATCHCNTL: RWRegister<u32>,

    /// ROMC Enable Register High
    pub ROMPATCHENH: RORegister<u32>,

    /// ROMC Enable Register Low
    pub ROMPATCHENL: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH0A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH1A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH2A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH3A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH4A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH5A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH6A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH7A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH8A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH9A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH10A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH11A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH12A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH13A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH14A: RWRegister<u32>,

    /// ROMC Address Registers
    pub ROMPATCH15A: RWRegister<u32>,

    _reserved2: [u32; 50],

    /// ROMC Status Register
    pub ROMPATCHSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ROMPATCH7D: u32,
    pub ROMPATCH6D: u32,
    pub ROMPATCH5D: u32,
    pub ROMPATCH4D: u32,
    pub ROMPATCH3D: u32,
    pub ROMPATCH2D: u32,
    pub ROMPATCH1D: u32,
    pub ROMPATCH0D: u32,
    pub ROMPATCHCNTL: u32,
    pub ROMPATCHENH: u32,
    pub ROMPATCHENL: u32,
    pub ROMPATCH0A: u32,
    pub ROMPATCH1A: u32,
    pub ROMPATCH2A: u32,
    pub ROMPATCH3A: u32,
    pub ROMPATCH4A: u32,
    pub ROMPATCH5A: u32,
    pub ROMPATCH6A: u32,
    pub ROMPATCH7A: u32,
    pub ROMPATCH8A: u32,
    pub ROMPATCH9A: u32,
    pub ROMPATCH10A: u32,
    pub ROMPATCH11A: u32,
    pub ROMPATCH12A: u32,
    pub ROMPATCH13A: u32,
    pub ROMPATCH14A: u32,
    pub ROMPATCH15A: u32,
    pub ROMPATCHSR: u32,
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
