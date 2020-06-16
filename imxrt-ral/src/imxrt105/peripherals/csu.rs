#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSU registers
//!
//! Used by: imxrt1051, imxrt1052

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Config security level register
pub mod CSL0 {

    /// Secure user read access control for the second slave
    pub mod SUR_S2 {
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

            /// 0b0: The secure user read access is disabled for the second slave.
            pub const SUR_S2_0: u32 = 0b0;

            /// 0b1: The secure user read access is enabled for the second slave.
            pub const SUR_S2_1: u32 = 0b1;
        }
    }

    /// Secure supervisor read access control for the second slave
    pub mod SSR_S2 {
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

            /// 0b0: The secure supervisor read access is disabled for the second slave.
            pub const SSR_S2_0: u32 = 0b0;

            /// 0b1: The secure supervisor read access is enabled for the second slave.
            pub const SSR_S2_1: u32 = 0b1;
        }
    }

    /// Non-secure user read access control for the second slave
    pub mod NUR_S2 {
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

            /// 0b0: The non-secure user read access is disabled for the second slave.
            pub const NUR_S2_0: u32 = 0b0;

            /// 0b1: The non-secure user read access is enabled for the second slave.
            pub const NUR_S2_1: u32 = 0b1;
        }
    }

    /// Non-secure supervisor read access control for the second slave
    pub mod NSR_S2 {
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

            /// 0b0: The non-secure supervisor read access is disabled for the second slave.
            pub const NSR_S2_0: u32 = 0b0;

            /// 0b1: The non-secure supervisor read access is enabled for the second slave.
            pub const NSR_S2_1: u32 = 0b1;
        }
    }

    /// Secure user write access control for the second slave
    pub mod SUW_S2 {
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

            /// 0b0: The secure user write access is disabled for the second slave.
            pub const SUW_S2_0: u32 = 0b0;

            /// 0b1: The secure user write access is enabled for the second slave.
            pub const SUW_S2_1: u32 = 0b1;
        }
    }

    /// Secure supervisor write access control for the second slave
    pub mod SSW_S2 {
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

            /// 0b0: The secure supervisor write access is disabled for the second slave.
            pub const SSW_S2_0: u32 = 0b0;

            /// 0b1: The secure supervisor write access is enabled for the second slave.
            pub const SSW_S2_1: u32 = 0b1;
        }
    }

    /// Non-secure user write access control for the second slave
    pub mod NUW_S2 {
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

            /// 0b0: The non-secure user write access is disabled for the second slave.
            pub const NUW_S2_0: u32 = 0b0;

            /// 0b1: The non-secure user write access is enabled for the second slave.
            pub const NUW_S2_1: u32 = 0b1;
        }
    }

    /// Non-secure supervisor write access control for the second slave
    pub mod NSW_S2 {
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

            /// 0b0: The non-secure supervisor write access is disabled for the second slave.
            pub const NSW_S2_0: u32 = 0b0;

            /// 0b1: The non-secure supervisor write access is enabled for the second slave.
            pub const NSW_S2_1: u32 = 0b1;
        }
    }

    /// The lock bit corresponding to the second slave. It is written by the secure software.
    pub mod LOCK_S2 {
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

            /// 0b0: Not locked. Bits 7-0 can be written by the software.
            pub const LOCK_S2_0: u32 = 0b0;

            /// 0b1: Bits 7-0 are locked and cannot be written by the software
            pub const LOCK_S2_1: u32 = 0b1;
        }
    }

    /// Secure user read access control for the first slave
    pub mod SUR_S1 {
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

            /// 0b0: The secure user read access is disabled for the first slave.
            pub const SUR_S1_0: u32 = 0b0;

            /// 0b1: The secure user read access is enabled for the first slave.
            pub const SUR_S1_1: u32 = 0b1;
        }
    }

    /// Secure supervisor read access control for the first slave
    pub mod SSR_S1 {
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

            /// 0b0: The secure supervisor read access is disabled for the first slave.
            pub const SSR_S1_0: u32 = 0b0;

            /// 0b1: The secure supervisor read access is enabled for the first slave.
            pub const SSR_S1_1: u32 = 0b1;
        }
    }

    /// Non-secure user read access control for the first slave
    pub mod NUR_S1 {
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

            /// 0b0: The non-secure user read access is disabled for the first slave.
            pub const NUR_S1_0: u32 = 0b0;

            /// 0b1: The non-secure user read access is enabled for the first slave.
            pub const NUR_S1_1: u32 = 0b1;
        }
    }

    /// Non-secure supervisor read access control for the first slave
    pub mod NSR_S1 {
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

            /// 0b0: The non-secure supervisor read access is disabled for the first slave.
            pub const NSR_S1_0: u32 = 0b0;

            /// 0b1: The non-secure supervisor read access is enabled for the first slave.
            pub const NSR_S1_1: u32 = 0b1;
        }
    }

    /// Secure user write access control for the first slave
    pub mod SUW_S1 {
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

            /// 0b0: The secure user write access is disabled for the first slave.
            pub const SUW_S1_0: u32 = 0b0;

            /// 0b1: The secure user write access is enabled for the first slave.
            pub const SUW_S1_1: u32 = 0b1;
        }
    }

    /// Secure supervisor write access control for the first slave
    pub mod SSW_S1 {
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

            /// 0b0: The secure supervisor write access is disabled for the first slave.
            pub const SSW_S1_0: u32 = 0b0;

            /// 0b1: The secure supervisor write access is enabled for the first slave.
            pub const SSW_S1_1: u32 = 0b1;
        }
    }

    /// Non-secure user write access control for the first slave
    pub mod NUW_S1 {
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

            /// 0b0: The non-secure user write access is disabled for the first slave.
            pub const NUW_S1_0: u32 = 0b0;

            /// 0b1: The non-secure user write access is enabled for the first slave.
            pub const NUW_S1_1: u32 = 0b1;
        }
    }

    /// Non-secure supervisor write access control for the first slave
    pub mod NSW_S1 {
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

            /// 0b0: The non-secure supervisor write access is disabled for the first slave.
            pub const NSW_S1_0: u32 = 0b0;

            /// 0b1: The non-secure supervisor write access is enabled for the first slave
            pub const NSW_S1_1: u32 = 0b1;
        }
    }

    /// The lock bit corresponding to the first slave. It is written by the secure software.
    pub mod LOCK_S1 {
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

            /// 0b0: Not locked. The bits 16-23 can be written by the software.
            pub const LOCK_S1_0: u32 = 0b0;

            /// 0b1: The bits 16-23 are locked and can't be written by the software.
            pub const LOCK_S1_1: u32 = 0b1;
        }
    }
}

/// Config security level register
pub mod CSL1 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL2 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL3 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL4 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL5 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL6 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL7 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL8 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL9 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL10 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL11 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL12 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL13 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL14 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL15 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL16 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL17 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL18 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL19 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL20 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL21 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL22 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL23 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL24 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL25 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL26 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL27 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL28 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL29 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL30 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// Config security level register
pub mod CSL31 {
    pub use super::CSL0::LOCK_S1;
    pub use super::CSL0::LOCK_S2;
    pub use super::CSL0::NSR_S1;
    pub use super::CSL0::NSR_S2;
    pub use super::CSL0::NSW_S1;
    pub use super::CSL0::NSW_S2;
    pub use super::CSL0::NUR_S1;
    pub use super::CSL0::NUR_S2;
    pub use super::CSL0::NUW_S1;
    pub use super::CSL0::NUW_S2;
    pub use super::CSL0::SSR_S1;
    pub use super::CSL0::SSR_S2;
    pub use super::CSL0::SSW_S1;
    pub use super::CSL0::SSW_S2;
    pub use super::CSL0::SUR_S1;
    pub use super::CSL0::SUR_S2;
    pub use super::CSL0::SUW_S1;
    pub use super::CSL0::SUW_S2;
}

/// HP0 register
pub mod HP0 {

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the eDMA
    pub mod HP_DMA {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_DMA_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_DMA_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the eDMA
    pub mod L_DMA {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DMA_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_DMA_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the LCDIF
    pub mod HP_LCDIF {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_LCDIF_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_LCDIF_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the LCDIF
    pub mod L_LCDIF {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_LCDIF_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_LCDIF_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the CSI
    pub mod HP_CSI {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_CSI_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_CSI_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the CSI
    pub mod L_CSI {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_CSI_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_CSI_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the PXP
    pub mod HP_PXP {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_PXP_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_PXP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the PXP
    pub mod L_PXP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_PXP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_PXP_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the DCP
    pub mod HP_DCP {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_DCP_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_DCP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the DCP
    pub mod L_DCP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DCP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit cannot be written by the software.
            pub const L_DCP_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the ENET
    pub mod HP_ENET {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_ENET_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_ENET_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the ENET
    pub mod L_ENET {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_ENET_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_ENET_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the USDHC1
    pub mod HP_USDHC1 {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USDHC1_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USDHC1_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC1
    pub mod L_USDHC1 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC1_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC1_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the USDHC2
    pub mod HP_USDHC2 {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USDHC2_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USDHC2_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC2
    pub mod L_USDHC2 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC2_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC2_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the TPSMP
    pub mod HP_TPSMP {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_TPSMP_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_TPSMP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the TPSMP
    pub mod L_TPSMP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_TPSMP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_TPSMP_1: u32 = 0b1;
        }
    }

    /// Determines whether the register value of the corresponding HP field is passed as the hprot\[1\] of the USB
    pub mod HP_USB {
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

            /// 0b0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USB_0: u32 = 0b0;

            /// 0b1: The HP register bit is routed to the csu_hprot1 output for the corresponding master.
            pub const HP_USB_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USB
    pub mod L_USB {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USB_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USB_1: u32 = 0b1;
        }
    }
}

/// Secure access register
pub mod SA {

    /// Non-secure access policy indicator bit
    pub mod NSA_DMA {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_DMA_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_DMA_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the eDMA
    pub mod L_DMA {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DMA_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_DMA_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_LCDIF {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_LCDIF_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_LCDIF_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the LCDIF
    pub mod L_LCDIF {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_LCDIF_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_LCDIF_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_CSI {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_CSI_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_CSI_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the CSI
    pub mod L_CSI {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_CSI_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_CSI_1: u32 = 0b1;
        }
    }

    /// Non-Secure Access Policy indicator bit
    pub mod NSA_PXP {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_PXP_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_PXP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the PXP
    pub mod L_PXP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_PXP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_PXP_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_DCP {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_DCP_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_DCP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the DCP
    pub mod L_DCP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DCP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_DCP_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_ENET {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_ENET_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_ENET_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the ENET1 and ENET2
    pub mod L_ENET {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_ENET_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_ENET_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_USDHC1 {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_USDHC1_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_USDHC1_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC1
    pub mod L_USDHC1 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC1_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC1_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_USDHC2 {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_USDHC2_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_USDHC2_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC2
    pub mod L_USDHC2 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC2_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC2_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_TPSMP {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_TPSMP_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_TPSMP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the TPSMP
    pub mod L_TPSMP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_TPSMP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_TPSMP_1: u32 = 0b1;
        }
    }

    /// Non-secure access policy indicator bit
    pub mod NSA_USB {
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

            /// 0b0: Secure access for the corresponding type-1 master
            pub const NSA_USB_0: u32 = 0b0;

            /// 0b1: Non-secure access for the corresponding type-1 master
            pub const NSA_USB_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USB
    pub mod L_USB {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USB_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USB_1: u32 = 0b1;
        }
    }
}

/// HPCONTROL0 register
pub mod HPCONTROL0 {

    /// Indicates the privilege/user mode for the eDMA
    pub mod HPC_DMA {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_DMA_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_DMA_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the eDMA
    pub mod L_DMA {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DMA_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_DMA_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the LCDIF
    pub mod HPC_LCDIF {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_LCDIF_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_LCDIF_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the LCDIF
    pub mod L_LCDIF {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_LCDIF_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_LCDIF_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the CSI
    pub mod HPC_CSI {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_CSI_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_CSI_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the CSI
    pub mod L_CSI {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_CSI_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_CSI_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the PXP
    pub mod HPC_PXP {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_PXP_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_PXP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the PXP
    pub mod L_PXP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_PXP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_PXP_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the DCP
    pub mod HPC_DCP {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_DCP_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_DCP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the DCP
    pub mod L_DCP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_DCP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_DCP_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the ENET
    pub mod HPC_ENET {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_ENET_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_ENET_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the ENET
    pub mod L_ENET {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_ENET_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_ENET_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the USDHC1
    pub mod HPC_USDHC1 {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_USDHC1_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_USDHC1_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC1
    pub mod L_USDHC1 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC1_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC1_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the USDHC2
    pub mod HPC_USDHC2 {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_USDHC2_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_USDHC2_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USDHC2.
    pub mod L_USDHC2 {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USDHC2_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USDHC2_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the TPSMP
    pub mod HPC_TPSMP {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_TPSMP_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_TPSMP_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the TPSMP.
    pub mod L_TPSMP {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_TPSMP_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_TPSMP_1: u32 = 0b1;
        }
    }

    /// Indicates the privilege/user mode for the USB
    pub mod HPC_USB {
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

            /// 0b0: User mode for the corresponding master
            pub const HPC_USB_0: u32 = 0b0;

            /// 0b1: Supervisor mode for the corresponding master
            pub const HPC_USB_1: u32 = 0b1;
        }
    }

    /// Lock bit set by the TZ software for the USB.
    pub mod L_USB {
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

            /// 0b0: No lock-the adjacent (next lower) bit can be written by the software.
            pub const L_USB_0: u32 = 0b0;

            /// 0b1: Lock-the adjacent (next lower) bit can't be written by the software.
            pub const L_USB_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Config security level register
    pub CSL0: RWRegister<u32>,

    /// Config security level register
    pub CSL1: RWRegister<u32>,

    /// Config security level register
    pub CSL2: RWRegister<u32>,

    /// Config security level register
    pub CSL3: RWRegister<u32>,

    /// Config security level register
    pub CSL4: RWRegister<u32>,

    /// Config security level register
    pub CSL5: RWRegister<u32>,

    /// Config security level register
    pub CSL6: RWRegister<u32>,

    /// Config security level register
    pub CSL7: RWRegister<u32>,

    /// Config security level register
    pub CSL8: RWRegister<u32>,

    /// Config security level register
    pub CSL9: RWRegister<u32>,

    /// Config security level register
    pub CSL10: RWRegister<u32>,

    /// Config security level register
    pub CSL11: RWRegister<u32>,

    /// Config security level register
    pub CSL12: RWRegister<u32>,

    /// Config security level register
    pub CSL13: RWRegister<u32>,

    /// Config security level register
    pub CSL14: RWRegister<u32>,

    /// Config security level register
    pub CSL15: RWRegister<u32>,

    /// Config security level register
    pub CSL16: RWRegister<u32>,

    /// Config security level register
    pub CSL17: RWRegister<u32>,

    /// Config security level register
    pub CSL18: RWRegister<u32>,

    /// Config security level register
    pub CSL19: RWRegister<u32>,

    /// Config security level register
    pub CSL20: RWRegister<u32>,

    /// Config security level register
    pub CSL21: RWRegister<u32>,

    /// Config security level register
    pub CSL22: RWRegister<u32>,

    /// Config security level register
    pub CSL23: RWRegister<u32>,

    /// Config security level register
    pub CSL24: RWRegister<u32>,

    /// Config security level register
    pub CSL25: RWRegister<u32>,

    /// Config security level register
    pub CSL26: RWRegister<u32>,

    /// Config security level register
    pub CSL27: RWRegister<u32>,

    /// Config security level register
    pub CSL28: RWRegister<u32>,

    /// Config security level register
    pub CSL29: RWRegister<u32>,

    /// Config security level register
    pub CSL30: RWRegister<u32>,

    /// Config security level register
    pub CSL31: RWRegister<u32>,

    _reserved1: [u32; 96],

    /// HP0 register
    pub HP0: RWRegister<u32>,

    _reserved2: [u32; 5],

    /// Secure access register
    pub SA: RWRegister<u32>,

    _reserved3: [u32; 79],

    /// HPCONTROL0 register
    pub HPCONTROL0: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSL0: u32,
    pub CSL1: u32,
    pub CSL2: u32,
    pub CSL3: u32,
    pub CSL4: u32,
    pub CSL5: u32,
    pub CSL6: u32,
    pub CSL7: u32,
    pub CSL8: u32,
    pub CSL9: u32,
    pub CSL10: u32,
    pub CSL11: u32,
    pub CSL12: u32,
    pub CSL13: u32,
    pub CSL14: u32,
    pub CSL15: u32,
    pub CSL16: u32,
    pub CSL17: u32,
    pub CSL18: u32,
    pub CSL19: u32,
    pub CSL20: u32,
    pub CSL21: u32,
    pub CSL22: u32,
    pub CSL23: u32,
    pub CSL24: u32,
    pub CSL25: u32,
    pub CSL26: u32,
    pub CSL27: u32,
    pub CSL28: u32,
    pub CSL29: u32,
    pub CSL30: u32,
    pub CSL31: u32,
    pub HP0: u32,
    pub SA: u32,
    pub HPCONTROL0: u32,
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
