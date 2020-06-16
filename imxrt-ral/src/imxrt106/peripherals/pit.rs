#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PIT
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// PIT Module Control Register
pub mod MCR {

    /// Freeze
    pub mod FRZ {
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

            /// 0b0: Timers continue to run in Debug mode.
            pub const FRZ_0: u32 = 0b0;

            /// 0b1: Timers are stopped in Debug mode.
            pub const FRZ_1: u32 = 0b1;
        }
    }

    /// Module Disable - (PIT section)
    pub mod MDIS {
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

            /// 0b0: Clock for standard PIT timers is enabled.
            pub const MDIS_0: u32 = 0b0;

            /// 0b1: Clock for standard PIT timers is disabled.
            pub const MDIS_1: u32 = 0b1;
        }
    }
}

/// PIT Upper Lifetime Timer Register
pub mod LTMR64H {

    /// Life Timer value
    pub mod LTH {
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

/// PIT Lower Lifetime Timer Register
pub mod LTMR64L {

    /// Life Timer value
    pub mod LTL {
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

/// Timer Load Value Register
pub mod LDVAL0 {

    /// Timer Start Value
    pub mod TSV {
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

/// Current Timer Value Register
pub mod CVAL0 {

    /// Current Timer Value
    pub mod TVL {
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

/// Timer Control Register
pub mod TCTRL0 {

    /// Timer Enable
    pub mod TEN {
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

            /// 0b0: Timer n is disabled.
            pub const TEN_0: u32 = 0b0;

            /// 0b1: Timer n is enabled.
            pub const TEN_1: u32 = 0b1;
        }
    }

    /// Timer Interrupt Enable
    pub mod TIE {
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

            /// 0b0: Interrupt requests from Timer n are disabled.
            pub const TIE_0: u32 = 0b0;

            /// 0b1: Interrupt will be requested whenever TIF is set.
            pub const TIE_1: u32 = 0b1;
        }
    }

    /// Chain Mode
    pub mod CHN {
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

            /// 0b0: Timer is not chained.
            pub const CHN_0: u32 = 0b0;

            /// 0b1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1.
            pub const CHN_1: u32 = 0b1;
        }
    }
}

/// Timer Flag Register
pub mod TFLG0 {

    /// Timer Interrupt Flag
    pub mod TIF {
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

            /// 0b0: Timeout has not yet occurred.
            pub const TIF_0: u32 = 0b0;

            /// 0b1: Timeout has occurred.
            pub const TIF_1: u32 = 0b1;
        }
    }
}

/// Timer Load Value Register
pub mod LDVAL1 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL1 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL1 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG1 {
    pub use super::TFLG0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL2 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL2 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL2 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG2 {
    pub use super::TFLG0::TIF;
}

/// Timer Load Value Register
pub mod LDVAL3 {
    pub use super::LDVAL0::TSV;
}

/// Current Timer Value Register
pub mod CVAL3 {
    pub use super::CVAL0::TVL;
}

/// Timer Control Register
pub mod TCTRL3 {
    pub use super::TCTRL0::CHN;
    pub use super::TCTRL0::TEN;
    pub use super::TCTRL0::TIE;
}

/// Timer Flag Register
pub mod TFLG3 {
    pub use super::TFLG0::TIF;
}
#[repr(C)]
pub struct RegisterBlock {
    /// PIT Module Control Register
    pub MCR: RWRegister<u32>,

    _reserved1: [u32; 55],

    /// PIT Upper Lifetime Timer Register
    pub LTMR64H: RORegister<u32>,

    /// PIT Lower Lifetime Timer Register
    pub LTMR64L: RORegister<u32>,

    _reserved2: [u32; 6],

    /// Timer Load Value Register
    pub LDVAL0: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL0: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL0: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG0: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL1: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL1: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL1: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG1: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL2: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL2: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL2: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG2: RWRegister<u32>,

    /// Timer Load Value Register
    pub LDVAL3: RWRegister<u32>,

    /// Current Timer Value Register
    pub CVAL3: RORegister<u32>,

    /// Timer Control Register
    pub TCTRL3: RWRegister<u32>,

    /// Timer Flag Register
    pub TFLG3: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub LTMR64H: u32,
    pub LTMR64L: u32,
    pub LDVAL0: u32,
    pub CVAL0: u32,
    pub TCTRL0: u32,
    pub TFLG0: u32,
    pub LDVAL1: u32,
    pub CVAL1: u32,
    pub TCTRL1: u32,
    pub TFLG1: u32,
    pub LDVAL2: u32,
    pub CVAL2: u32,
    pub TCTRL2: u32,
    pub TFLG2: u32,
    pub LDVAL3: u32,
    pub CVAL3: u32,
    pub TCTRL3: u32,
    pub TFLG3: u32,
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
