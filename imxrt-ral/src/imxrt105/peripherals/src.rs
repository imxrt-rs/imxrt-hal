#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRC
//!
//! Used by: imxrt1051, imxrt1052

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SRC Control Register
pub mod SCR {

    /// Mask wdog_rst_b source
    pub mod mask_wdog_rst {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (4 bits: 0b1111 << 7)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0101: wdog_rst_b is masked
            pub const mask_wdog_rst_5: u32 = 0b0101;

            /// 0b1010: wdog_rst_b is not masked (default)
            pub const mask_wdog_rst_10: u32 = 0b1010;
        }
    }

    /// Software reset for core0 only
    pub mod core0_rst {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: do not assert core0 reset
            pub const core0_rst_0: u32 = 0b0;

            /// 0b1: assert core0 reset
            pub const core0_rst_1: u32 = 0b1;
        }
    }

    /// Software reset for core0 debug only
    pub mod core0_dbg_rst {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: do not assert core0 debug reset
            pub const core0_dbg_rst_0: u32 = 0b0;

            /// 0b1: assert core0 debug reset
            pub const core0_dbg_rst_1: u32 = 0b1;
        }
    }

    /// Do not assert debug resets after power gating event of core
    pub mod dbg_rst_msk_pg {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: do not mask core debug resets (debug resets will be asserted after power gating event)
            pub const dbg_rst_msk_pg_0: u32 = 0b0;

            /// 0b1: mask core debug resets (debug resets won't be asserted after power gating event)
            pub const dbg_rst_msk_pg_1: u32 = 0b1;
        }
    }

    /// Mask wdog3_rst_b source
    pub mod mask_wdog3_rst {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0101: wdog3_rst_b is masked
            pub const mask_wdog3_rst_5: u32 = 0b0101;

            /// 0b1010: wdog3_rst_b is not masked
            pub const mask_wdog3_rst_10: u32 = 0b1010;
        }
    }
}

/// SRC Boot Mode Register 1
pub mod SBMR1 {

    /// Refer to fusemap.
    pub mod BOOT_CFG1 {
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

    /// Refer to fusemap.
    pub mod BOOT_CFG2 {
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

    /// Refer to fusemap.
    pub mod BOOT_CFG3 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Refer to fusemap.
    pub mod BOOT_CFG4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRC Reset Status Register
pub mod SRSR {

    /// Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)
    pub mod ipp_reset_b {
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

            /// 0b0: Reset is not a result of ipp_reset_b pin.
            pub const ipp_reset_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of ipp_reset_b pin.
            pub const ipp_reset_b_1: u32 = 0b1;
        }
    }

    /// Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core
    pub mod lockup_sysresetreq {
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

            /// 0b0: Reset is not a result of the mentioned case.
            pub const lockup_sysresetreq_0: u32 = 0b0;

            /// 0b1: Reset is a result of the mentioned case.
            pub const lockup_sysresetreq_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the csu_reset_b input.
    pub mod csu_reset_b {
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

            /// 0b0: Reset is not a result of the csu_reset_b event.
            pub const csu_reset_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of the csu_reset_b event.
            pub const csu_reset_b_1: u32 = 0b1;
        }
    }

    /// Indicates whether the reset was the result of the ipp_user_reset_b qualified reset.
    pub mod ipp_user_reset_b {
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

            /// 0b0: Reset is not a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const ipp_user_reset_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of the ipp_user_reset_b qualified as COLD reset event.
            pub const ipp_user_reset_b_1: u32 = 0b1;
        }
    }

    /// IC Watchdog Time-out reset
    pub mod wdog_rst_b {
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

            /// 0b0: Reset is not a result of the watchdog time-out event.
            pub const wdog_rst_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog time-out event.
            pub const wdog_rst_b_1: u32 = 0b1;
        }
    }

    /// HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG.
    pub mod jtag_rst_b {
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

            /// 0b0: Reset is not a result of HIGH-Z reset from JTAG.
            pub const jtag_rst_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of HIGH-Z reset from JTAG.
            pub const jtag_rst_b_1: u32 = 0b1;
        }
    }

    /// JTAG software reset. Indicates whether the reset was the result of software reset from JTAG.
    pub mod jtag_sw_rst {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of software reset from JTAG.
            pub const jtag_sw_rst_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from JTAG.
            pub const jtag_sw_rst_1: u32 = 0b1;
        }
    }

    /// IC Watchdog3 Time-out reset
    pub mod wdog3_rst_b {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset is not a result of the watchdog3 time-out event.
            pub const wdog3_rst_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of the watchdog3 time-out event.
            pub const wdog3_rst_b_1: u32 = 0b1;
        }
    }

    /// Temper Sensor software reset
    pub mod tempsense_rst_b {
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

            /// 0b0: Reset is not a result of software reset from Temperature Sensor.
            pub const tempsense_rst_b_0: u32 = 0b0;

            /// 0b1: Reset is a result of software reset from Temperature Sensor.
            pub const tempsense_rst_b_1: u32 = 0b1;
        }
    }
}

/// SRC Boot Mode Register 2
pub mod SBMR2 {

    /// SECONFIG\[1\] shows the state of the SECONFIG\[1\] fuse
    pub mod SEC_CONFIG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DIR_BT_DIS shows the state of the DIR_BT_DIS fuse
    pub mod DIR_BT_DIS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse
    pub mod BT_FUSE_SEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BMOD\[1:0\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B
    pub mod BMOD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SRC General Purpose Register 1
pub mod GPR1 {

    /// Holds entry function for core0 for waking-up from low power mode
    pub mod PERSISTENT_ENTRY0 {
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

/// SRC General Purpose Register 2
pub mod GPR2 {

    /// Holds argument of entry function for core0 for waking-up from low power mode
    pub mod PERSISTENT_ARG0 {
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

/// SRC General Purpose Register 3
pub mod GPR3 {}

/// SRC General Purpose Register 4
pub mod GPR4 {}

/// SRC General Purpose Register 5
pub mod GPR5 {}

/// SRC General Purpose Register 6
pub mod GPR6 {}

/// SRC General Purpose Register 7
pub mod GPR7 {}

/// SRC General Purpose Register 8
pub mod GPR8 {}

/// SRC General Purpose Register 9
pub mod GPR9 {}

/// SRC General Purpose Register 10
pub mod GPR10 {}
pub struct RegisterBlock {
    /// SRC Control Register
    pub SCR: RWRegister<u32>,

    /// SRC Boot Mode Register 1
    pub SBMR1: RORegister<u32>,

    /// SRC Reset Status Register
    pub SRSR: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// SRC Boot Mode Register 2
    pub SBMR2: RORegister<u32>,

    /// SRC General Purpose Register 1
    pub GPR1: RWRegister<u32>,

    /// SRC General Purpose Register 2
    pub GPR2: RWRegister<u32>,

    /// SRC General Purpose Register 3
    pub GPR3: RWRegister<u32>,

    /// SRC General Purpose Register 4
    pub GPR4: RWRegister<u32>,

    /// SRC General Purpose Register 5
    pub GPR5: RWRegister<u32>,

    /// SRC General Purpose Register 6
    pub GPR6: RWRegister<u32>,

    /// SRC General Purpose Register 7
    pub GPR7: RWRegister<u32>,

    /// SRC General Purpose Register 8
    pub GPR8: RWRegister<u32>,

    /// SRC General Purpose Register 9
    pub GPR9: RORegister<u32>,

    /// SRC General Purpose Register 10
    pub GPR10: RWRegister<u32>,
}
pub struct ResetValues {
    pub SCR: u32,
    pub SBMR1: u32,
    pub SRSR: u32,
    pub SBMR2: u32,
    pub GPR1: u32,
    pub GPR2: u32,
    pub GPR3: u32,
    pub GPR4: u32,
    pub GPR5: u32,
    pub GPR6: u32,
    pub GPR7: u32,
    pub GPR8: u32,
    pub GPR9: u32,
    pub GPR10: u32,
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
