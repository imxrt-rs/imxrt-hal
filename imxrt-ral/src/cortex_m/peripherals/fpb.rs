#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash Patch and Breakpoint Unit
//!
//! Used by: armv7em, armv7m

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// FlashPatch Control Register
pub mod CTRL {}

/// FlashPatch Remap Register
pub mod REMAP {}

/// FlashPatch Comparator Register 0
pub mod COMP0 {}

/// FlashPatch Comparator Register 1
pub mod COMP1 {}

/// FlashPatch Comparator Register 2
pub mod COMP2 {}

/// FlashPatch Comparator Register 3
pub mod COMP3 {}

/// FlashPatch Comparator Register 4
pub mod COMP4 {}

/// FlashPatch Comparator Register 5
pub mod COMP5 {}

/// FlashPatch Comparator Register 6
pub mod COMP6 {}

/// FlashPatch Comparator Register 7
pub mod COMP7 {}

/// FlashPatch Comparator Register 8
pub mod COMP8 {}

/// FlashPatch Comparator Register 9
pub mod COMP9 {}

/// FlashPatch Comparator Register 10
pub mod COMP10 {}

/// FlashPatch Comparator Register 11
pub mod COMP11 {}

/// FlashPatch Comparator Register 12
pub mod COMP12 {}

/// FlashPatch Comparator Register 13
pub mod COMP13 {}

/// FlashPatch Comparator Register 14
pub mod COMP14 {}

/// FlashPatch Comparator Register 15
pub mod COMP15 {}

/// Lock Status Register
pub mod LSR {}

/// Lock Access Register
pub mod LAR {}
#[repr(C)]
pub struct RegisterBlock {
    /// FlashPatch Control Register
    pub CTRL: RWRegister<u32>,

    /// FlashPatch Remap Register
    pub REMAP: RWRegister<u32>,

    /// FlashPatch Comparator Register 0
    pub COMP0: RWRegister<u32>,

    /// FlashPatch Comparator Register 1
    pub COMP1: RWRegister<u32>,

    /// FlashPatch Comparator Register 2
    pub COMP2: RWRegister<u32>,

    /// FlashPatch Comparator Register 3
    pub COMP3: RWRegister<u32>,

    /// FlashPatch Comparator Register 4
    pub COMP4: RWRegister<u32>,

    /// FlashPatch Comparator Register 5
    pub COMP5: RWRegister<u32>,

    /// FlashPatch Comparator Register 6
    pub COMP6: RWRegister<u32>,

    /// FlashPatch Comparator Register 7
    pub COMP7: RWRegister<u32>,

    /// FlashPatch Comparator Register 8
    pub COMP8: RWRegister<u32>,

    /// FlashPatch Comparator Register 9
    pub COMP9: RWRegister<u32>,

    /// FlashPatch Comparator Register 10
    pub COMP10: RWRegister<u32>,

    /// FlashPatch Comparator Register 11
    pub COMP11: RWRegister<u32>,

    /// FlashPatch Comparator Register 12
    pub COMP12: RWRegister<u32>,

    /// FlashPatch Comparator Register 13
    pub COMP13: RWRegister<u32>,

    /// FlashPatch Comparator Register 14
    pub COMP14: RWRegister<u32>,

    /// FlashPatch Comparator Register 15
    pub COMP15: RWRegister<u32>,

    _reserved1: [u32; 986],

    /// Lock Access Register
    pub LAR: WORegister<u32>,

    /// Lock Status Register
    pub LSR: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub REMAP: u32,
    pub COMP0: u32,
    pub COMP1: u32,
    pub COMP2: u32,
    pub COMP3: u32,
    pub COMP4: u32,
    pub COMP5: u32,
    pub COMP6: u32,
    pub COMP7: u32,
    pub COMP8: u32,
    pub COMP9: u32,
    pub COMP10: u32,
    pub COMP11: u32,
    pub COMP12: u32,
    pub COMP13: u32,
    pub COMP14: u32,
    pub COMP15: u32,
    pub LAR: u32,
    pub LSR: u32,
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
