#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Data Watchpoint and Trace Unit
//!
//! Used by: armv6m, armv7em, armv7m

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control Register
pub mod CTRL {}

/// Cycle Count Register
pub mod CYCCNT {}

/// CPI Count Register
pub mod CPICNT {}

/// Exception Overhead Count Register
pub mod EXCCNT {}

/// Sleep Count Register
pub mod SLEEPCNT {}

/// LSU Count Register
pub mod LSUCNT {}

/// Folded-instruction Count Register
pub mod FOLDCNT {}

/// Program Counter Sample Register
pub mod PCSR {}

/// Comparator Register 0
pub mod COMP0 {}

/// Comparator Mask Register 0
pub mod MASK0 {}

/// Comparator Function Register 0
pub mod FUNCTION0 {}

/// Comparator Register 1
pub mod COMP1 {}

/// Comparator Mask Register 1
pub mod MASK1 {}

/// Comparator Function Register 1
pub mod FUNCTION1 {}

/// Comparator Register 2
pub mod COMP2 {}

/// Comparator Mask Register 2
pub mod MASK2 {}

/// Comparator Function Register 2
pub mod FUNCTION2 {}

/// Comparator Register 3
pub mod COMP3 {}

/// Comparator Mask Register 3
pub mod MASK3 {}

/// Comparator Function Register 3
pub mod FUNCTION3 {}

/// Comparator Register 4
pub mod COMP4 {}

/// Comparator Mask Register 4
pub mod MASK4 {}

/// Comparator Function Register 4
pub mod FUNCTION4 {}

/// Comparator Register 5
pub mod COMP5 {}

/// Comparator Mask Register 5
pub mod MASK5 {}

/// Comparator Function Register 5
pub mod FUNCTION5 {}

/// Comparator Register 6
pub mod COMP6 {}

/// Comparator Mask Register 6
pub mod MASK6 {}

/// Comparator Function Register 6
pub mod FUNCTION6 {}

/// Comparator Register 7
pub mod COMP7 {}

/// Comparator Mask Register 7
pub mod MASK7 {}

/// Comparator Function Register 7
pub mod FUNCTION7 {}

/// Comparator Register 8
pub mod COMP8 {}

/// Comparator Mask Register 8
pub mod MASK8 {}

/// Comparator Function Register 8
pub mod FUNCTION8 {}

/// Comparator Register 9
pub mod COMP9 {}

/// Comparator Mask Register 9
pub mod MASK9 {}

/// Comparator Function Register 9
pub mod FUNCTION9 {}

/// Comparator Register 10
pub mod COMP10 {}

/// Comparator Mask Register 10
pub mod MASK10 {}

/// Comparator Function Register 10
pub mod FUNCTION10 {}

/// Comparator Register 11
pub mod COMP11 {}

/// Comparator Mask Register 11
pub mod MASK11 {}

/// Comparator Function Register 11
pub mod FUNCTION11 {}

/// Comparator Register 12
pub mod COMP12 {}

/// Comparator Mask Register 12
pub mod MASK12 {}

/// Comparator Function Register 12
pub mod FUNCTION12 {}

/// Comparator Register 13
pub mod COMP13 {}

/// Comparator Mask Register 13
pub mod MASK13 {}

/// Comparator Function Register 13
pub mod FUNCTION13 {}

/// Comparator Register 14
pub mod COMP14 {}

/// Comparator Mask Register 14
pub mod MASK14 {}

/// Comparator Function Register 14
pub mod FUNCTION14 {}

/// Comparator Register 15
pub mod COMP15 {}

/// Comparator Mask Register 15
pub mod MASK15 {}

/// Comparator Function Register 15
pub mod FUNCTION15 {}

/// Lock Status Register
pub mod LSR {}

/// Lock Access Register
pub mod LAR {}
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register
    pub CTRL: RWRegister<u32>,

    /// Cycle Count Register
    pub CYCCNT: RWRegister<u32>,

    /// CPI Count Register
    pub CPICNT: RWRegister<u32>,

    /// Exception Overhead Count Register
    pub EXCCNT: RWRegister<u32>,

    /// Sleep Count Register
    pub SLEEPCNT: RWRegister<u32>,

    /// LSU Count Register
    pub LSUCNT: RWRegister<u32>,

    /// Folded-instruction Count Register
    pub FOLDCNT: RWRegister<u32>,

    /// Program Counter Sample Register
    pub PCSR: RORegister<u32>,

    /// Comparator Register 0
    pub COMP0: RWRegister<u32>,

    /// Comparator Mask Register 0
    pub MASK0: RWRegister<u32>,

    /// Comparator Function Register 0
    pub FUNCTION0: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Comparator Register 1
    pub COMP1: RWRegister<u32>,

    /// Comparator Mask Register 1
    pub MASK1: RWRegister<u32>,

    /// Comparator Function Register 1
    pub FUNCTION1: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Comparator Register 2
    pub COMP2: RWRegister<u32>,

    /// Comparator Mask Register 2
    pub MASK2: RWRegister<u32>,

    /// Comparator Function Register 2
    pub FUNCTION2: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Comparator Register 3
    pub COMP3: RWRegister<u32>,

    /// Comparator Mask Register 3
    pub MASK3: RWRegister<u32>,

    /// Comparator Function Register 3
    pub FUNCTION3: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Comparator Register 4
    pub COMP4: RWRegister<u32>,

    /// Comparator Mask Register 4
    pub MASK4: RWRegister<u32>,

    /// Comparator Function Register 4
    pub FUNCTION4: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Comparator Register 5
    pub COMP5: RWRegister<u32>,

    /// Comparator Mask Register 5
    pub MASK5: RWRegister<u32>,

    /// Comparator Function Register 5
    pub FUNCTION5: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// Comparator Register 6
    pub COMP6: RWRegister<u32>,

    /// Comparator Mask Register 6
    pub MASK6: RWRegister<u32>,

    /// Comparator Function Register 6
    pub FUNCTION6: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// Comparator Register 7
    pub COMP7: RWRegister<u32>,

    /// Comparator Mask Register 7
    pub MASK7: RWRegister<u32>,

    /// Comparator Function Register 7
    pub FUNCTION7: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Comparator Register 8
    pub COMP8: RWRegister<u32>,

    /// Comparator Mask Register 8
    pub MASK8: RWRegister<u32>,

    /// Comparator Function Register 8
    pub FUNCTION8: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// Comparator Register 9
    pub COMP9: RWRegister<u32>,

    /// Comparator Mask Register 9
    pub MASK9: RWRegister<u32>,

    /// Comparator Function Register 9
    pub FUNCTION9: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// Comparator Register 10
    pub COMP10: RWRegister<u32>,

    /// Comparator Mask Register 10
    pub MASK10: RWRegister<u32>,

    /// Comparator Function Register 10
    pub FUNCTION10: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// Comparator Register 11
    pub COMP11: RWRegister<u32>,

    /// Comparator Mask Register 11
    pub MASK11: RWRegister<u32>,

    /// Comparator Function Register 11
    pub FUNCTION11: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// Comparator Register 12
    pub COMP12: RWRegister<u32>,

    /// Comparator Mask Register 12
    pub MASK12: RWRegister<u32>,

    /// Comparator Function Register 12
    pub FUNCTION12: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// Comparator Register 13
    pub COMP13: RWRegister<u32>,

    /// Comparator Mask Register 13
    pub MASK13: RWRegister<u32>,

    /// Comparator Function Register 13
    pub FUNCTION13: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// Comparator Register 14
    pub COMP14: RWRegister<u32>,

    /// Comparator Mask Register 14
    pub MASK14: RWRegister<u32>,

    /// Comparator Function Register 14
    pub FUNCTION14: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// Comparator Register 15
    pub COMP15: RWRegister<u32>,

    /// Comparator Mask Register 15
    pub MASK15: RWRegister<u32>,

    /// Comparator Function Register 15
    pub FUNCTION15: RWRegister<u32>,

    _reserved16: [u32; 933],

    /// Lock Access Register
    pub LAR: WORegister<u32>,

    /// Lock Status Register
    pub LSR: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CYCCNT: u32,
    pub CPICNT: u32,
    pub EXCCNT: u32,
    pub SLEEPCNT: u32,
    pub LSUCNT: u32,
    pub FOLDCNT: u32,
    pub PCSR: u32,
    pub COMP0: u32,
    pub MASK0: u32,
    pub FUNCTION0: u32,
    pub COMP1: u32,
    pub MASK1: u32,
    pub FUNCTION1: u32,
    pub COMP2: u32,
    pub MASK2: u32,
    pub FUNCTION2: u32,
    pub COMP3: u32,
    pub MASK3: u32,
    pub FUNCTION3: u32,
    pub COMP4: u32,
    pub MASK4: u32,
    pub FUNCTION4: u32,
    pub COMP5: u32,
    pub MASK5: u32,
    pub FUNCTION5: u32,
    pub COMP6: u32,
    pub MASK6: u32,
    pub FUNCTION6: u32,
    pub COMP7: u32,
    pub MASK7: u32,
    pub FUNCTION7: u32,
    pub COMP8: u32,
    pub MASK8: u32,
    pub FUNCTION8: u32,
    pub COMP9: u32,
    pub MASK9: u32,
    pub FUNCTION9: u32,
    pub COMP10: u32,
    pub MASK10: u32,
    pub FUNCTION10: u32,
    pub COMP11: u32,
    pub MASK11: u32,
    pub FUNCTION11: u32,
    pub COMP12: u32,
    pub MASK12: u32,
    pub FUNCTION12: u32,
    pub COMP13: u32,
    pub MASK13: u32,
    pub FUNCTION13: u32,
    pub COMP14: u32,
    pub MASK14: u32,
    pub FUNCTION14: u32,
    pub COMP15: u32,
    pub MASK15: u32,
    pub FUNCTION15: u32,
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
