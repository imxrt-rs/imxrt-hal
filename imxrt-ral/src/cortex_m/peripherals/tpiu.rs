#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Trace Port Interface Unit
//!
//! Used by: armv7em, armv7m

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Supported Parallel Port Sizes Register
pub mod SSPSR {}

/// Current Parallel Port Size Register
pub mod CSPSR {}

/// Asynchronous Clock Prescaler Register
pub mod ACPR {}

/// Selected Pin Protocol Register
pub mod SPPR {}

/// TPIU Type Register
pub mod TYPE {}

/// Lock Status Register
pub mod LSR {}

/// Lock Access Register
pub mod LAR {}
#[repr(C)]
pub struct RegisterBlock {
    /// Supported Parallel Port Sizes Register
    pub SSPSR: RORegister<u32>,

    /// Current Parallel Port Size Register
    pub CSPSR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Asynchronous Clock Prescaler Register
    pub ACPR: RWRegister<u32>,

    _reserved2: [u32; 55],

    /// Selected Pin Protocol Register
    pub SPPR: RWRegister<u32>,

    _reserved3: [u32; 943],

    /// Lock Access Register
    pub LAR: WORegister<u32>,

    /// Lock Status Register
    pub LSR: RORegister<u32>,

    _reserved4: [u32; 4],

    /// TPIU Type Register
    pub TYPE: RORegister<u32>,
}
pub struct ResetValues {
    pub SSPSR: u32,
    pub CSPSR: u32,
    pub ACPR: u32,
    pub SPPR: u32,
    pub LAR: u32,
    pub LSR: u32,
    pub TYPE: u32,
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
