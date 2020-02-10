#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Debug Control Block
//!
//! Used by: armv6m, armv7em, armv7m

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Debug Halting Control and Status Register
pub mod DHCSR {}

/// Debug Core Register Selector Register
pub mod DCRSR {}

/// Debug Core Register Data Register
pub mod DCRDR {}

/// Debug Exception and Monitor Control Register
pub mod DEMCR {}
pub struct RegisterBlock {
    /// Debug Halting Control and Status Register
    pub DHCSR: RWRegister<u32>,

    /// Debug Core Register Selector Register
    pub DCRSR: WORegister<u32>,

    /// Debug Core Register Data Register
    pub DCRDR: RWRegister<u32>,

    /// Debug Exception and Monitor Control Register
    pub DEMCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub DHCSR: u32,
    pub DCRSR: u32,
    pub DCRDR: u32,
    pub DEMCR: u32,
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
