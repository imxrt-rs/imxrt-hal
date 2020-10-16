#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// USB OTG1 Control Register
pub mod USB_OTG1_CTRL {

    /// Disable OTG1 Overcurrent Detection
    pub mod OVER_CUR_DIS {
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

            /// 0b0: Enables overcurrent detection
            pub const OVER_CUR_DIS_0: u32 = 0b0;

            /// 0b1: Disables overcurrent detection
            pub const OVER_CUR_DIS_1: u32 = 0b1;
        }
    }

    /// OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event
    pub mod OVER_CUR_POL {
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

            /// 0b0: High active (high on this signal represents an overcurrent condition)
            pub const OVER_CUR_POL_0: u32 = 0b0;

            /// 0b1: Low active (low on this signal represents an overcurrent condition)
            pub const OVER_CUR_POL_1: u32 = 0b1;
        }
    }

    /// OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity.
    pub mod PWR_POL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMIC Power Pin is Low active.
            pub const PWR_POL_0: u32 = 0b0;

            /// 0b1: PMIC Power Pin is High active.
            pub const PWR_POL_1: u32 = 0b1;
        }
    }

    /// OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt
    pub mod WIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt Disabled
            pub const WIE_0: u32 = 0b0;

            /// 0b1: Interrupt Enabled
            pub const WIE_1: u32 = 0b1;
        }
    }

    /// OTG1 Software Wake-up Enable
    pub mod WKUP_SW_EN {
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

            /// 0b0: Disable
            pub const WKUP_SW_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const WKUP_SW_EN_1: u32 = 0b1;
        }
    }

    /// OTG1 Software Wake-up
    pub mod WKUP_SW {
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

            /// 0b0: Inactive
            pub const WKUP_SW_0: u32 = 0b0;

            /// 0b1: Force wake-up
            pub const WKUP_SW_1: u32 = 0b1;
        }
    }

    /// OTG1 Wake-up on ID change enable
    pub mod WKUP_ID_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable
            pub const WKUP_ID_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const WKUP_ID_EN_1: u32 = 0b1;
        }
    }

    /// OTG1 wake-up on VBUS change enable
    pub mod WKUP_VBUS_EN {
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

            /// 0b0: Disable
            pub const WKUP_VBUS_EN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const WKUP_VBUS_EN_1: u32 = 0b1;
        }
    }

    /// Wake-up on DPDM change enable
    pub mod WKUP_DPDM_EN {
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

            /// 0b0: DPDM changes wake-up to be disabled only when VBUS is 0.
            pub const WKUP_DPDM_EN_0: u32 = 0b0;

            /// 0b1: (Default) DPDM changes wake-up to be enabled, it is for device only.
            pub const WKUP_DPDM_EN_1: u32 = 0b1;
        }
    }

    /// OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port
    pub mod WIR {
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

            /// 0b0: No wake-up interrupt request received
            pub const WIR_0: u32 = 0b0;

            /// 0b1: Wake-up Interrupt Request received
            pub const WIR_1: u32 = 0b1;
        }
    }
}

/// OTG1 UTMI PHY Control 0 Register
pub mod USB_OTG1_PHY_CTRL_0 {

    /// Indicating whether OTG1 UTMI PHY clock is valid
    pub mod UTMI_CLK_VLD {
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

            /// 0b0: Invalid
            pub const UTMI_CLK_VLD_0: u32 = 0b0;

            /// 0b1: Valid
            pub const UTMI_CLK_VLD_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 512],

    /// USB OTG1 Control Register
    pub USB_OTG1_CTRL: RWRegister<u32>,

    _reserved2: [u32; 5],

    /// OTG1 UTMI PHY Control 0 Register
    pub USB_OTG1_PHY_CTRL_0: RWRegister<u32>,
}
pub struct ResetValues {
    pub USB_OTG1_CTRL: u32,
    pub USB_OTG1_PHY_CTRL_0: u32,
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
