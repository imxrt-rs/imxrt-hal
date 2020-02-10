#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! KPP Registers
//!
//! Used by: imxrt1051, imxrt1052

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Keypad Control Register
pub mod KPCR {

    /// Keypad Row Enable
    pub mod KRE {
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

            /// 0b00000000: Row is not included in the keypad key press detect.
            pub const KRE_0: u32 = 0b00000000;

            /// 0b00000001: Row is included in the keypad key press detect.
            pub const KRE_1: u32 = 0b00000001;
        }
    }

    /// Keypad Column Strobe Open-Drain Enable
    pub mod KCO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Column strobe output is totem pole drive.
            pub const TOTEM_POLE: u32 = 0b00000000;

            /// 0b00000001: Column strobe output is open drain.
            pub const OPEN_DRAIN: u32 = 0b00000001;
        }
    }
}

/// Keypad Status Register
pub mod KPSR {

    /// Keypad Key Depress
    pub mod KPKD {
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

            /// 0b0: No key presses detected
            pub const KPKD_0: u32 = 0b0;

            /// 0b1: A key has been depressed
            pub const KPKD_1: u32 = 0b1;
        }
    }

    /// Keypad Key Release
    pub mod KPKR {
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

            /// 0b0: No key release detected
            pub const KPKR_0: u32 = 0b0;

            /// 0b1: All keys have been released
            pub const KPKR_1: u32 = 0b1;
        }
    }

    /// Key Depress Synchronizer Clear
    pub mod KDSC {
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

            /// 0b0: No effect
            pub const KDSC_0: u32 = 0b0;

            /// 0b1: Set bits that clear the keypad depress synchronizer chain
            pub const KDSC_1: u32 = 0b1;
        }
    }

    /// Key Release Synchronizer Set
    pub mod KRSS {
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

            /// 0b0: No effect
            pub const KRSS_0: u32 = 0b0;

            /// 0b1: Set bits which sets keypad release synchronizer chain
            pub const KRSS_1: u32 = 0b1;
        }
    }

    /// Keypad Key Depress Interrupt Enable
    pub mod KDIE {
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

            /// 0b0: No interrupt request is generated when KPKD is set.
            pub const KDIE_0: u32 = 0b0;

            /// 0b1: An interrupt request is generated when KPKD is set.
            pub const KDIE_1: u32 = 0b1;
        }
    }

    /// Keypad Release Interrupt Enable
    pub mod KRIE {
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

            /// 0b0: No interrupt request is generated when KPKR is set.
            pub const KRIE_0: u32 = 0b0;

            /// 0b1: An interrupt request is generated when KPKR is set.
            pub const KRIE_1: u32 = 0b1;
        }
    }
}

/// Keypad Data Direction Register
pub mod KDDR {

    /// Keypad Row Data Direction
    pub mod KRDD {
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

            /// 0b00000000: ROWn pin configured as an input.
            pub const INPUT: u32 = 0b00000000;

            /// 0b00000001: ROWn pin configured as an output.
            pub const OUTPUT: u32 = 0b00000001;
        }
    }

    /// Keypad Column Data Direction Register
    pub mod KCDD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: COLn pin is configured as an input.
            pub const INPUT: u32 = 0b00000000;

            /// 0b00000001: COLn pin is configured as an output.
            pub const OUTPUT: u32 = 0b00000001;
        }
    }
}

/// Keypad Data Register
pub mod KPDR {

    /// Keypad Row Data
    pub mod KRD {
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

    /// Keypad Column Data
    pub mod KCD {
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
}
pub struct RegisterBlock {
    /// Keypad Control Register
    pub KPCR: RWRegister<u16>,

    /// Keypad Status Register
    pub KPSR: RWRegister<u16>,

    /// Keypad Data Direction Register
    pub KDDR: RWRegister<u16>,

    /// Keypad Data Register
    pub KPDR: RWRegister<u16>,
}
pub struct ResetValues {
    pub KPCR: u16,
    pub KPSR: u16,
    pub KDDR: u16,
    pub KPDR: u16,
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
