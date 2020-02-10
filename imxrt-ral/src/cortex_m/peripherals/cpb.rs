#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cache and branch predictor maintenance operations
//!
//! Used by: armv7em, armv7m

use crate::UnsafeWORegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// I-cache invalidate all to PoU
pub mod ICIALLU {}

/// I-cache invalidate by MVA to PoU
pub mod ICIMVAU {

    /// Memory address
    pub mod MVA {
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

/// D-cache invalidate by MVA to PoC
pub mod DCIMVAC {
    pub use super::ICIMVAU::MVA;
}

/// D-cache invaldiate by set-way
pub mod DCISW {}

/// D-cache clean by MVA to PoU
pub mod DCCMVAU {
    pub use super::ICIMVAU::MVA;
}

/// D-cache clean by MVA to PoC
pub mod DCCMVAC {
    pub use super::ICIMVAU::MVA;
}

/// D-cache clean by set-way
pub mod DCCSW {}

/// D-cache clean and invalidate by MVA to PoC
pub mod DCCIMVAC {
    pub use super::ICIMVAU::MVA;
}

/// D-cache clean and invalidate by set-way
pub mod DCCISW {}

/// Branch predictor invalidate all
pub mod BPIALL {}
pub struct RegisterBlock {
    /// I-cache invalidate all to PoU
    pub ICIALLU: UnsafeWORegister<u32>,

    _reserved1: [u32; 1],

    /// I-cache invalidate by MVA to PoU
    pub ICIMVAU: UnsafeWORegister<u32>,

    /// D-cache invalidate by MVA to PoC
    pub DCIMVAC: UnsafeWORegister<u32>,

    /// D-cache invaldiate by set-way
    pub DCISW: UnsafeWORegister<u32>,

    /// D-cache clean by MVA to PoU
    pub DCCMVAU: UnsafeWORegister<u32>,

    /// D-cache clean by MVA to PoC
    pub DCCMVAC: UnsafeWORegister<u32>,

    /// D-cache clean by set-way
    pub DCCSW: UnsafeWORegister<u32>,

    /// D-cache clean and invalidate by MVA to PoC
    pub DCCIMVAC: UnsafeWORegister<u32>,

    /// D-cache clean and invalidate by set-way
    pub DCCISW: UnsafeWORegister<u32>,

    /// Branch predictor invalidate all
    pub BPIALL: UnsafeWORegister<u32>,
}
pub struct ResetValues {
    pub ICIALLU: u32,
    pub ICIMVAU: u32,
    pub DCIMVAC: u32,
    pub DCISW: u32,
    pub DCCMVAU: u32,
    pub DCCMVAC: u32,
    pub DCCSW: u32,
    pub DCCIMVAC: u32,
    pub DCCISW: u32,
    pub BPIALL: u32,
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
