#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPC Interface control register
pub mod CNTR {

    /// MEGA domain power down request
    pub mod MEGA_PDN_REQ {
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

            /// 0b0: No Request
            pub const MEGA_PDN_REQ_0: u32 = 0b0;

            /// 0b1: Request power down sequence
            pub const MEGA_PDN_REQ_1: u32 = 0b1;
        }
    }

    /// MEGA domain power up request
    pub mod MEGA_PUP_REQ {
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

            /// 0b0: No Request
            pub const MEGA_PUP_REQ_0: u32 = 0b0;

            /// 0b1: Request power up sequence
            pub const MEGA_PUP_REQ_1: u32 = 0b1;
        }
    }

    /// FlexRAM PDRAM0 Power Gate Enable
    pub mod PDRAM0_PGE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FlexRAM PDRAM0 domain will keep power on even if CPU core is power down.
            pub const PDRAM0_PGE_0: u32 = 0b0;

            /// 0b1: FlexRAM PDRAM0 domain will be power down once when CPU core is power down.
            pub const PDRAM0_PGE_1: u32 = 0b1;
        }
    }
}

/// IRQ masking register 1
pub mod IMR1 {

    /// IRQ\[31:0\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR1 {
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

/// IRQ masking register 2
pub mod IMR2 {

    /// IRQ\[63:32\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR2 {
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

/// IRQ masking register 3
pub mod IMR3 {

    /// IRQ\[95:64\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR3 {
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

/// IRQ masking register 4
pub mod IMR4 {

    /// IRQ\[127:96\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR4 {
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

/// IRQ status resister 1
pub mod ISR1 {

    /// IRQ\[31:0\] status, read only
    pub mod ISR1 {
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

/// IRQ status resister 2
pub mod ISR2 {

    /// IRQ\[63:32\] status, read only
    pub mod ISR2 {
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

/// IRQ status resister 3
pub mod ISR3 {

    /// IRQ\[95:64\] status, read only
    pub mod ISR3 {
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

/// IRQ status resister 4
pub mod ISR4 {

    /// IRQ\[127:96\] status, read only
    pub mod ISR4 {
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

/// IRQ masking register 5
pub mod IMR5 {

    /// IRQ\[159:128\] masking bits: 1-irq masked, 0-irq is not masked
    pub mod IMR5 {
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

/// IRQ status resister 5
pub mod ISR5 {
    pub use super::ISR4::ISR4;
}
pub struct RegisterBlock {
    /// GPC Interface control register
    pub CNTR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// IRQ masking register 1
    pub IMR1: RWRegister<u32>,

    /// IRQ masking register 2
    pub IMR2: RWRegister<u32>,

    /// IRQ masking register 3
    pub IMR3: RWRegister<u32>,

    /// IRQ masking register 4
    pub IMR4: RWRegister<u32>,

    /// IRQ status resister 1
    pub ISR1: RORegister<u32>,

    /// IRQ status resister 2
    pub ISR2: RORegister<u32>,

    /// IRQ status resister 3
    pub ISR3: RORegister<u32>,

    /// IRQ status resister 4
    pub ISR4: RORegister<u32>,

    _reserved2: [u32; 3],

    /// IRQ masking register 5
    pub IMR5: RWRegister<u32>,

    /// IRQ status resister 5
    pub ISR5: RORegister<u32>,
}
pub struct ResetValues {
    pub CNTR: u32,
    pub IMR1: u32,
    pub IMR2: u32,
    pub IMR3: u32,
    pub IMR4: u32,
    pub ISR1: u32,
    pub ISR2: u32,
    pub ISR3: u32,
    pub ISR4: u32,
    pub IMR5: u32,
    pub ISR5: u32,
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
