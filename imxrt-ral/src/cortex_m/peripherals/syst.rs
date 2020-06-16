#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System Timer
//!
//! Used by: armv6m, armv7em, armv7m

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SysTick Control and Status Register
pub mod CSR {}

/// SysTick Reload Value Register
pub mod RVR {}

/// SysTick Current Value Register
pub mod CVR {}

/// SysTick Calibration Value Register
pub mod CALIB {}
#[repr(C)]
pub struct RegisterBlock {
    /// SysTick Control and Status Register
    pub CSR: RWRegister<u32>,

    /// SysTick Reload Value Register
    pub RVR: RWRegister<u32>,

    /// SysTick Current Value Register
    pub CVR: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// SysTick Calibration Value Register
    pub CALIB: RORegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
    pub RVR: u32,
    pub CVR: u32,
    pub CALIB: u32,
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
