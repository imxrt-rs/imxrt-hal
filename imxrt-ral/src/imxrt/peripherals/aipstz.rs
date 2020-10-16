#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AIPSTZ Control Registers
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Master Priviledge Registers
pub mod MPR {

    /// Master 5 Priviledge, Buffer, Read, Write Control.
    pub mod MPROT5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\[1\] access attribute.
            pub const MPL0: u32 = 0b0000;

            /// 0b0000: Accesses from this master are not forced to user-mode. The hprot\[1\] access attribute is used directly to determine ips_supervisor_access.
            pub const MPL1: u32 = 0b0000;
        }
    }

    /// Master 3 Priviledge, Buffer, Read, Write Control.
    pub mod MPROT3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 2 Priviledge, Buffer, Read, Write Control
    pub mod MPROT2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 1 Priviledge, Buffer, Read, Write Control
    pub mod MPROT1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }

    /// Master 0 Priviledge, Buffer, Read, Write Control
    pub mod MPROT0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MPROT5::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR {

    /// Off-platform Peripheral Access Control 7
    pub mod OPAC7 {
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

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 6
    pub mod OPAC6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 5
    pub mod OPAC5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 4
    pub mod OPAC4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 3
    pub mod OPAC3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 2
    pub mod OPAC2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 1
    pub mod OPAC1 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }

    /// Off-platform Peripheral Access Control 0
    pub mod OPAC0 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC7::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR1 {

    /// Off-platform Peripheral Access Control 15
    pub mod OPAC15 {
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

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 14
    pub mod OPAC14 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 13
    pub mod OPAC13 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 12
    pub mod OPAC12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 11
    pub mod OPAC11 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 10
    pub mod OPAC10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 9
    pub mod OPAC9 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }

    /// Off-platform Peripheral Access Control 8
    pub mod OPAC8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC15::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR2 {

    /// Off-platform Peripheral Access Control 23
    pub mod OPAC23 {
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

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 22
    pub mod OPAC22 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 21
    pub mod OPAC21 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 20
    pub mod OPAC20 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 19
    pub mod OPAC19 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 18
    pub mod OPAC18 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 17
    pub mod OPAC17 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }

    /// Off-platform Peripheral Access Control 16
    pub mod OPAC16 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC23::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR3 {

    /// Off-platform Peripheral Access Control 31
    pub mod OPAC31 {
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

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 30
    pub mod OPAC30 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 29
    pub mod OPAC29 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 28
    pub mod OPAC28 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 27
    pub mod OPAC27 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 26
    pub mod OPAC26 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 25
    pub mod OPAC25 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }

    /// Off-platform Peripheral Access Control 24
    pub mod OPAC24 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC31::RW;
    }
}

/// Off-Platform Peripheral Access Control Registers
pub mod OPACR4 {

    /// Off-platform Peripheral Access Control 33
    pub mod OPAC33 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Accesses from an untrusted master are allowed.
            pub const TP0: u32 = 0b0000;

            /// 0b0000: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus.
            pub const TP1: u32 = 0b0000;
        }
    }

    /// Off-platform Peripheral Access Control 32
    pub mod OPAC32 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OPAC33::RW;
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Master Priviledge Registers
    pub MPR: RWRegister<u32>,

    _reserved1: [u32; 15],

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR1: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR2: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR3: RWRegister<u32>,

    /// Off-Platform Peripheral Access Control Registers
    pub OPACR4: RWRegister<u32>,
}
pub struct ResetValues {
    pub MPR: u32,
    pub OPACR: u32,
    pub OPACR1: u32,
    pub OPACR2: u32,
    pub OPACR3: u32,
    pub OPACR4: u32,
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
