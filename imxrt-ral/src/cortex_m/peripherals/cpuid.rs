#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CPUID
//!
//! Used by: armv7em, armv7m

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Provides identification information for the processor
pub mod Base {

    /// This field defines the implementer
    pub mod IMPLEMENTER {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1000001: ARM Limited
            pub const ARM: u32 = 0b1000001;
        }
    }

    /// Implementation defined
    pub mod VARIANT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field defines the architecture
    pub mod ARCHITECTURE {
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

            /// 0b1100: ARMv6-M
            pub const ARMv6M: u32 = 0b1100;
        }
    }

    /// Implementation defined.
    pub mod PARTNO {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Implementation defined.
    pub mod REVISION {
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

/// Processor Feature Register 0
pub mod PFR0 {}

/// Processor Feature Register 1
pub mod PFR1 {}

/// Debug Feature Register 0
pub mod DFR0 {}

/// Auxiliary Feature Register 0
pub mod AFR0 {}

/// Memory Model Feature Register 0
pub mod MMFR0 {}

/// Memory Model Feature Register 1
pub mod MMFR1 {}

/// Memory Model Feature Register 2
pub mod MMFR2 {}

/// Memory Model Feature Register 3
pub mod MMFR3 {}

/// Instruction Set Attribute Register 0
pub mod ISAR0 {}

/// Instruction Set Attribute Register 1
pub mod ISAR1 {}

/// Instruction Set Attribute Register 2
pub mod ISAR2 {}

/// Instruction Set Attribute Register 3
pub mod ISAR3 {}

/// Instruction Set Attribute Register 4
pub mod ISAR4 {}

/// Cache Level ID Register
pub mod CLIDR {}

/// Cache Type Register
pub mod CTR {}

/// Cache Size ID Register
pub mod CCSIDR {}

/// Cache Size Selection Register
pub mod CSSELR {}
#[repr(C)]
pub struct RegisterBlock {
    /// Provides identification information for the processor
    pub Base: RORegister<u32>,

    _reserved1: [u32; 15],

    /// Processor Feature Register 0
    pub PFR0: RORegister<u32>,

    /// Processor Feature Register 1
    pub PFR1: RORegister<u32>,

    /// Debug Feature Register 0
    pub DFR0: RORegister<u32>,

    /// Auxiliary Feature Register 0
    pub AFR0: RORegister<u32>,

    /// Memory Model Feature Register 0
    pub MMFR0: RORegister<u32>,

    /// Memory Model Feature Register 1
    pub MMFR1: RORegister<u32>,

    /// Memory Model Feature Register 2
    pub MMFR2: RORegister<u32>,

    /// Memory Model Feature Register 3
    pub MMFR3: RORegister<u32>,

    /// Instruction Set Attribute Register 0
    pub ISAR0: RORegister<u32>,

    /// Instruction Set Attribute Register 1
    pub ISAR1: RORegister<u32>,

    /// Instruction Set Attribute Register 2
    pub ISAR2: RORegister<u32>,

    /// Instruction Set Attribute Register 3
    pub ISAR3: RORegister<u32>,

    /// Instruction Set Attribute Register 4
    pub ISAR4: RORegister<u32>,

    _reserved2: [u32; 1],

    /// Cache Level ID Register
    pub CLIDR: RORegister<u32>,

    /// Cache Type Register
    pub CTR: RORegister<u32>,

    /// Cache Size ID Register
    pub CCSIDR: RORegister<u32>,

    /// Cache Size Selection Register
    pub CSSELR: RWRegister<u32>,
}
pub struct ResetValues {
    pub Base: u32,
    pub PFR0: u32,
    pub PFR1: u32,
    pub DFR0: u32,
    pub AFR0: u32,
    pub MMFR0: u32,
    pub MMFR1: u32,
    pub MMFR2: u32,
    pub MMFR3: u32,
    pub ISAR0: u32,
    pub ISAR1: u32,
    pub ISAR2: u32,
    pub ISAR3: u32,
    pub ISAR4: u32,
    pub CLIDR: u32,
    pub CTR: u32,
    pub CCSIDR: u32,
    pub CSSELR: u32,
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
