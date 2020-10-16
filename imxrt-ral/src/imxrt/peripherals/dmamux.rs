#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Channel 0 Configuration Register
pub mod CHCFG0 {

    /// DMA Channel Source (Slot Number)
    pub mod SOURCE {
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

    /// DMA Channel Always Enable
    pub mod A_ON {
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

            /// 0b0: DMA Channel Always ON function is disabled
            pub const A_ON_0: u32 = 0b0;

            /// 0b1: DMA Channel Always ON function is enabled
            pub const A_ON_1: u32 = 0b1;
        }
    }

    /// DMA Channel Trigger Enable
    pub mod TRIG {
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

            /// 0b0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)
            pub const TRIG_0: u32 = 0b0;

            /// 0b1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode.
            pub const TRIG_1: u32 = 0b1;
        }
    }

    /// DMA Mux Channel Enable
    pub mod ENBL {
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

            /// 0b0: DMA Mux channel is disabled
            pub const ENBL_0: u32 = 0b0;

            /// 0b1: DMA Mux channel is enabled
            pub const ENBL_1: u32 = 0b1;
        }
    }
}

/// Channel 0 Configuration Register
pub mod CHCFG1 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG2 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG3 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG4 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG5 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG6 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG7 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG8 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG9 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG10 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG11 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG12 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG13 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG14 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG15 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG16 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG17 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG18 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG19 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG20 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG21 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG22 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG23 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG24 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG25 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG26 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG27 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG28 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG29 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG30 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG31 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Channel 0 Configuration Register
    pub CHCFG0: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG1: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG2: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG3: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG4: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG5: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG6: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG7: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG8: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG9: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG10: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG11: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG12: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG13: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG14: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG15: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG16: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG17: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG18: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG19: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG20: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG21: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG22: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG23: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG24: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG25: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG26: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG27: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG28: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG29: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG30: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG31: RWRegister<u32>,
}
pub struct ResetValues {
    pub CHCFG0: u32,
    pub CHCFG1: u32,
    pub CHCFG2: u32,
    pub CHCFG3: u32,
    pub CHCFG4: u32,
    pub CHCFG5: u32,
    pub CHCFG6: u32,
    pub CHCFG7: u32,
    pub CHCFG8: u32,
    pub CHCFG9: u32,
    pub CHCFG10: u32,
    pub CHCFG11: u32,
    pub CHCFG12: u32,
    pub CHCFG13: u32,
    pub CHCFG14: u32,
    pub CHCFG15: u32,
    pub CHCFG16: u32,
    pub CHCFG17: u32,
    pub CHCFG18: u32,
    pub CHCFG19: u32,
    pub CHCFG20: u32,
    pub CHCFG21: u32,
    pub CHCFG22: u32,
    pub CHCFG23: u32,
    pub CHCFG24: u32,
    pub CHCFG25: u32,
    pub CHCFG26: u32,
    pub CHCFG27: u32,
    pub CHCFG28: u32,
    pub CHCFG29: u32,
    pub CHCFG30: u32,
    pub CHCFG31: u32,
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
