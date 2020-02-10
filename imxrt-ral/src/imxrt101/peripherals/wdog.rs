#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG
//!
//! Used by: imxrt1011, imxrt1015

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Watchdog Control Register
pub mod WCR {

    /// WDZST
    pub mod WDZST {
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

            /// 0b0: Continue timer operation (Default).
            pub const WDZST_0: u32 = 0b0;

            /// 0b1: Suspend the watchdog timer.
            pub const WDZST_1: u32 = 0b1;
        }
    }

    /// WDBG
    pub mod WDBG {
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

            /// 0b0: Continue WDOG timer operation (Default).
            pub const WDBG_0: u32 = 0b0;

            /// 0b1: Suspend the watchdog timer.
            pub const WDBG_1: u32 = 0b1;
        }
    }

    /// WDE
    pub mod WDE {
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

            /// 0b0: Disable the Watchdog (Default).
            pub const WDE_0: u32 = 0b0;

            /// 0b1: Enable the Watchdog.
            pub const WDE_1: u32 = 0b1;
        }
    }

    /// WDT
    pub mod WDT {
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

            /// 0b0: No effect on WDOG_B (Default).
            pub const WDT_0: u32 = 0b0;

            /// 0b1: Assert WDOG_B upon a Watchdog Time-out event.
            pub const WDT_1: u32 = 0b1;
        }
    }

    /// SRS
    pub mod SRS {
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

            /// 0b0: Assert system reset signal.
            pub const SRS_0: u32 = 0b0;

            /// 0b1: No effect on the system (Default).
            pub const SRS_1: u32 = 0b1;
        }
    }

    /// WDA
    pub mod WDA {
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

            /// 0b0: Assert WDOG_B output.
            pub const WDA_0: u32 = 0b0;

            /// 0b1: No effect on system (Default).
            pub const WDA_1: u32 = 0b1;
        }
    }

    /// software reset extension, an option way to generate software reset
    pub mod SRE {
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

            /// 0b0: using original way to generate software reset (default)
            pub const SRE_0: u32 = 0b0;

            /// 0b1: using new way to generate software reset.
            pub const SRE_1: u32 = 0b1;
        }
    }

    /// WDW
    pub mod WDW {
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

            /// 0b0: Continue WDOG timer operation (Default).
            pub const WDW_0: u32 = 0b0;

            /// 0b1: Suspend WDOG timer operation.
            pub const WDW_1: u32 = 0b1;
        }
    }

    /// WT
    pub mod WT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: - 0.5 Seconds (Default).
            pub const WT_0: u32 = 0b00000000;

            /// 0b00000001: - 1.0 Seconds.
            pub const WT_1: u32 = 0b00000001;

            /// 0b00000010: - 1.5 Seconds.
            pub const WT_2: u32 = 0b00000010;

            /// 0b00000011: - 2.0 Seconds.
            pub const WT_3: u32 = 0b00000011;

            /// 0b11111111: - 128 Seconds.
            pub const WT_255: u32 = 0b11111111;
        }
    }
}

/// Watchdog Service Register
pub mod WSR {

    /// WSR
    pub mod WSR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0101010101010101: Write to the Watchdog Service Register (WDOG_WSR).
            pub const WSR_21845: u32 = 0b0101010101010101;

            /// 0b1010101010101010: Write to the Watchdog Service Register (WDOG_WSR).
            pub const WSR_43690: u32 = 0b1010101010101010;
        }
    }
}

/// Watchdog Reset Status Register
pub mod WRSR {

    /// SFTW
    pub mod SFTW {
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

            /// 0b0: Reset is not the result of a software reset.
            pub const SFTW_0: u32 = 0b0;

            /// 0b1: Reset is the result of a software reset.
            pub const SFTW_1: u32 = 0b1;
        }
    }

    /// TOUT
    pub mod TOUT {
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

            /// 0b0: Reset is not the result of a WDOG timeout.
            pub const TOUT_0: u32 = 0b0;

            /// 0b1: Reset is the result of a WDOG timeout.
            pub const TOUT_1: u32 = 0b1;
        }
    }

    /// POR
    pub mod POR {
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

            /// 0b0: Reset is not the result of a power on reset.
            pub const POR_0: u32 = 0b0;

            /// 0b1: Reset is the result of a power on reset.
            pub const POR_1: u32 = 0b1;
        }
    }
}

/// Watchdog Interrupt Control Register
pub mod WICR {

    /// WICT
    pub mod WICT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: WICT\[7:0\] = Time duration between interrupt and time-out is 0 seconds.
            pub const WICT_0: u32 = 0b00000000;

            /// 0b00000001: WICT\[7:0\] = Time duration between interrupt and time-out is 0.5 seconds.
            pub const WICT_1: u32 = 0b00000001;

            /// 0b00000100: WICT\[7:0\] = Time duration between interrupt and time-out is 2 seconds (Default).
            pub const WICT_4: u32 = 0b00000100;

            /// 0b11111111: WICT\[7:0\] = Time duration between interrupt and time-out is 127.5 seconds.
            pub const WICT_255: u32 = 0b11111111;
        }
    }

    /// WTIS
    pub mod WTIS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt has occurred (Default).
            pub const WTIS_0: u32 = 0b0;

            /// 0b1: Interrupt has occurred
            pub const WTIS_1: u32 = 0b1;
        }
    }

    /// WIE
    pub mod WIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable Interrupt (Default).
            pub const WIE_0: u32 = 0b0;

            /// 0b1: Enable Interrupt.
            pub const WIE_1: u32 = 0b1;
        }
    }
}

/// Watchdog Miscellaneous Control Register
pub mod WMCR {

    /// PDE
    pub mod PDE {
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

            /// 0b0: Power Down Counter of WDOG is disabled.
            pub const PDE_0: u32 = 0b0;

            /// 0b1: Power Down Counter of WDOG is enabled (Default).
            pub const PDE_1: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    /// Watchdog Control Register
    pub WCR: RWRegister<u16>,

    /// Watchdog Service Register
    pub WSR: RWRegister<u16>,

    /// Watchdog Reset Status Register
    pub WRSR: RORegister<u16>,

    /// Watchdog Interrupt Control Register
    pub WICR: RWRegister<u16>,

    /// Watchdog Miscellaneous Control Register
    pub WMCR: RWRegister<u16>,
}
pub struct ResetValues {
    pub WCR: u16,
    pub WSR: u16,
    pub WRSR: u16,
    pub WICR: u16,
    pub WMCR: u16,
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
