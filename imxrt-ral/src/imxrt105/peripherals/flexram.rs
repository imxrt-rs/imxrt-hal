#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXRAM
//!
//! Used by: imxrt1051, imxrt1052

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// TCM CRTL Register
pub mod TCM_CTRL {

    /// TCM Write Wait Mode Enable
    pub mod TCM_WWAIT_EN {
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

            /// 0b0: TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_WWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_WWAIT_EN_1: u32 = 0b1;
        }
    }

    /// TCM Read Wait Mode Enable
    pub mod TCM_RWAIT_EN {
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

            /// 0b0: TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_RWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_RWAIT_EN_1: u32 = 0b1;
        }
    }

    /// Force RAM Clock Always On
    pub mod FORCE_CLK_ON {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Register
pub mod INT_STATUS {

    /// ITCM Access Error Status
    pub mod ITCM_ERR_STATUS {
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

            /// 0b0: ITCM access error does not happen
            pub const ITCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: ITCM access error happens.
            pub const ITCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status
    pub mod DTCM_ERR_STATUS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTCM access error does not happen
            pub const DTCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: DTCM access error happens.
            pub const DTCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status
    pub mod OCRAM_ERR_STATUS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCRAM access error does not happen
            pub const OCRAM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens.
            pub const OCRAM_ERR_STATUS_1: u32 = 0b1;
        }
    }
}

/// Interrupt Status Enable Register
pub mod INT_STAT_EN {

    /// ITCM Access Error Status Enable
    pub mod ITCM_ERR_STAT_EN {
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

            /// 0b0: Masked
            pub const ITCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status Enable
    pub mod DTCM_ERR_STAT_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable
    pub mod OCRAM_ERR_STAT_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod INT_SIG_EN {

    /// ITCM Access Error Interrupt Enable
    pub mod ITCM_ERR_SIG_EN {
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

            /// 0b0: Masked
            pub const ITCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Interrupt Enable
    pub mod DTCM_ERR_SIG_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable
    pub mod OCRAM_ERR_SIG_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// TCM CRTL Register
    pub TCM_CTRL: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// Interrupt Status Register
    pub INT_STATUS: RWRegister<u32>,

    /// Interrupt Status Enable Register
    pub INT_STAT_EN: RWRegister<u32>,

    /// Interrupt Enable Register
    pub INT_SIG_EN: RWRegister<u32>,
}
pub struct ResetValues {
    pub TCM_CTRL: u32,
    pub INT_STATUS: u32,
    pub INT_STAT_EN: u32,
    pub INT_SIG_EN: u32,
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
