#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt Set Enable Register n
pub mod NVICISER0 {

    /// Interrupt set enable bits
    pub mod SETENA {
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

/// Interrupt Set Enable Register n
pub mod NVICISER1 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Set Enable Register n
pub mod NVICISER2 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Set Enable Register n
pub mod NVICISER3 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Set Enable Register n
pub mod NVICISER4 {
    pub use super::NVICISER0::SETENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER0 {

    /// Interrupt clear-enable bits
    pub mod CLRENA {
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

/// Interrupt Clear Enable Register n
pub mod NVICICER1 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER2 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER3 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Clear Enable Register n
pub mod NVICICER4 {
    pub use super::NVICICER0::CLRENA;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR0 {

    /// Interrupt set-pending bits
    pub mod SETPEND {
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

/// Interrupt Set Pending Register n
pub mod NVICISPR1 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR2 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR3 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Set Pending Register n
pub mod NVICISPR4 {
    pub use super::NVICISPR0::SETPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR0 {

    /// Interrupt clear-pending bits
    pub mod CLRPEND {
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

/// Interrupt Clear Pending Register n
pub mod NVICICPR1 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR2 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR3 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Clear Pending Register n
pub mod NVICICPR4 {
    pub use super::NVICICPR0::CLRPEND;
}

/// Interrupt Active bit Register n
pub mod NVICIABR0 {

    /// Interrupt active flags
    pub mod ACTIVE {
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

/// Interrupt Active bit Register n
pub mod NVICIABR1 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Active bit Register n
pub mod NVICIABR2 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Active bit Register n
pub mod NVICIABR3 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Active bit Register n
pub mod NVICIABR4 {
    pub use super::NVICIABR0::ACTIVE;
}

/// Interrupt Priority Register 0
pub mod NVICIP0 {

    /// Priority of the INT_DMA0_DMA16 interrupt 0
    pub mod PRI0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 1
pub mod NVICIP1 {

    /// Priority of the INT_DMA1_DMA17 interrupt 1
    pub mod PRI1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 2
pub mod NVICIP2 {

    /// Priority of the INT_DMA2_DMA18 interrupt 2
    pub mod PRI2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 3
pub mod NVICIP3 {

    /// Priority of the INT_DMA3_DMA19 interrupt 3
    pub mod PRI3 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 4
pub mod NVICIP4 {

    /// Priority of the INT_DMA4_DMA20 interrupt 4
    pub mod PRI4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 5
pub mod NVICIP5 {

    /// Priority of the INT_DMA5_DMA21 interrupt 5
    pub mod PRI5 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 6
pub mod NVICIP6 {

    /// Priority of the INT_DMA6_DMA22 interrupt 6
    pub mod PRI6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 7
pub mod NVICIP7 {

    /// Priority of the INT_DMA7_DMA23 interrupt 7
    pub mod PRI7 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 8
pub mod NVICIP8 {

    /// Priority of the INT_DMA8_DMA24 interrupt 8
    pub mod PRI8 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 9
pub mod NVICIP9 {

    /// Priority of the INT_DMA9_DMA25 interrupt 9
    pub mod PRI9 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 10
pub mod NVICIP10 {

    /// Priority of the INT_DMA10_DMA26 interrupt 10
    pub mod PRI10 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 11
pub mod NVICIP11 {

    /// Priority of the INT_DMA11_DMA27 interrupt 11
    pub mod PRI11 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 12
pub mod NVICIP12 {

    /// Priority of the INT_DMA12_DMA28 interrupt 12
    pub mod PRI12 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 13
pub mod NVICIP13 {

    /// Priority of the INT_DMA13_DMA29 interrupt 13
    pub mod PRI13 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 14
pub mod NVICIP14 {

    /// Priority of the INT_DMA14_DMA30 interrupt 14
    pub mod PRI14 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 15
pub mod NVICIP15 {

    /// Priority of the INT_DMA15_DMA31 interrupt 15
    pub mod PRI15 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 16
pub mod NVICIP16 {

    /// Priority of the INT_DMA_ERROR interrupt 16
    pub mod PRI16 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 17
pub mod NVICIP17 {

    /// Priority of the INT_CTI0_ERROR interrupt 17
    pub mod PRI17 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 18
pub mod NVICIP18 {

    /// Priority of the INT_CTI1_ERROR interrupt 18
    pub mod PRI18 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 19
pub mod NVICIP19 {

    /// Priority of the INT_CORE interrupt 19
    pub mod PRI19 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 20
pub mod NVICIP20 {

    /// Priority of the INT_LPUART1 interrupt 20
    pub mod PRI20 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 21
pub mod NVICIP21 {

    /// Priority of the INT_LPUART2 interrupt 21
    pub mod PRI21 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 22
pub mod NVICIP22 {

    /// Priority of the INT_LPUART3 interrupt 22
    pub mod PRI22 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 23
pub mod NVICIP23 {

    /// Priority of the INT_LPUART4 interrupt 23
    pub mod PRI23 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 24
pub mod NVICIP24 {

    /// Priority of the INT_LPUART5 interrupt 24
    pub mod PRI24 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 25
pub mod NVICIP25 {

    /// Priority of the INT_LPUART6 interrupt 25
    pub mod PRI25 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 26
pub mod NVICIP26 {

    /// Priority of the INT_LPUART7 interrupt 26
    pub mod PRI26 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 27
pub mod NVICIP27 {

    /// Priority of the INT_LPUART8 interrupt 27
    pub mod PRI27 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 28
pub mod NVICIP28 {

    /// Priority of the INT_LPI2C1 interrupt 28
    pub mod PRI28 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 29
pub mod NVICIP29 {

    /// Priority of the INT_LPI2C2 interrupt 29
    pub mod PRI29 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 30
pub mod NVICIP30 {

    /// Priority of the INT_LPI2C3 interrupt 30
    pub mod PRI30 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 31
pub mod NVICIP31 {

    /// Priority of the INT_LPI2C4 interrupt 31
    pub mod PRI31 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 32
pub mod NVICIP32 {

    /// Priority of the INT_LPSPI1 interrupt 32
    pub mod PRI32 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 33
pub mod NVICIP33 {

    /// Priority of the INT_LPSPI2 interrupt 33
    pub mod PRI33 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 34
pub mod NVICIP34 {

    /// Priority of the INT_LPSPI3 interrupt 34
    pub mod PRI34 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 35
pub mod NVICIP35 {

    /// Priority of the INT_LPSPI4 interrupt 35
    pub mod PRI35 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 36
pub mod NVICIP36 {

    /// Priority of the INT_CAN1 interrupt 36
    pub mod PRI36 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 37
pub mod NVICIP37 {

    /// Priority of the INT_CAN2 interrupt 37
    pub mod PRI37 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 38
pub mod NVICIP38 {

    /// Priority of the INT_FLEXRAM interrupt 38
    pub mod PRI38 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 39
pub mod NVICIP39 {

    /// Priority of the INT_KPP interrupt 39
    pub mod PRI39 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 40
pub mod NVICIP40 {

    /// Priority of the INT_Reserved56 interrupt 40
    pub mod PRI40 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 41
pub mod NVICIP41 {

    /// Priority of the INT_GPR_IRQ interrupt 41
    pub mod PRI41 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 42
pub mod NVICIP42 {

    /// Priority of the INT_Reserved58 interrupt 42
    pub mod PRI42 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 43
pub mod NVICIP43 {

    /// Priority of the INT_Reserved59 interrupt 43
    pub mod PRI43 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 44
pub mod NVICIP44 {

    /// Priority of the INT_Reserved60 interrupt 44
    pub mod PRI44 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 45
pub mod NVICIP45 {

    /// Priority of the INT_WDOG2 interrupt 45
    pub mod PRI45 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 46
pub mod NVICIP46 {

    /// Priority of the INT_SNVS_HP_WRAPPER interrupt 46
    pub mod PRI46 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 47
pub mod NVICIP47 {

    /// Priority of the INT_SNVS_HP_WRAPPER_TZ interrupt 47
    pub mod PRI47 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 48
pub mod NVICIP48 {

    /// Priority of the INT_SNVS_LP_WRAPPER interrupt 48
    pub mod PRI48 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 49
pub mod NVICIP49 {

    /// Priority of the INT_CSU interrupt 49
    pub mod PRI49 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 50
pub mod NVICIP50 {

    /// Priority of the INT_DCP interrupt 50
    pub mod PRI50 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 51
pub mod NVICIP51 {

    /// Priority of the INT_DCP_VMI interrupt 51
    pub mod PRI51 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 52
pub mod NVICIP52 {

    /// Priority of the INT_Reserved68 interrupt 52
    pub mod PRI52 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 53
pub mod NVICIP53 {

    /// Priority of the INT_TRNG interrupt 53
    pub mod PRI53 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 54
pub mod NVICIP54 {

    /// Priority of interrupt 54
    pub mod PRI54 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 55
pub mod NVICIP55 {

    /// Priority of the INT_BEE interrupt 55
    pub mod PRI55 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 56
pub mod NVICIP56 {

    /// Priority of the INT_SAI1 interrupt 56
    pub mod PRI56 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 57
pub mod NVICIP57 {

    /// Priority of the INT_SAI2 interrupt 57
    pub mod PRI57 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 58
pub mod NVICIP58 {

    /// Priority of the INT_SAI3_RX interrupt 58
    pub mod PRI58 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 59
pub mod NVICIP59 {

    /// Priority of the INT_SAI3_TX interrupt 59
    pub mod PRI59 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 60
pub mod NVICIP60 {

    /// Priority of the INT_SPDIF interrupt 60
    pub mod PRI60 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 61
pub mod NVICIP61 {

    /// Priority of the INT_PMU interrupt 61
    pub mod PRI61 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 62
pub mod NVICIP62 {

    /// Priority of the INT_Reserved78 interrupt 62
    pub mod PRI62 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 63
pub mod NVICIP63 {

    /// Priority of the INT_TEMP_LOW_HIGH interrupt 63
    pub mod PRI63 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 64
pub mod NVICIP64 {

    /// Priority of the INT_TEMP_PANIC interrupt 64
    pub mod PRI64 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 65
pub mod NVICIP65 {

    /// Priority of the INT_USB_PHY interrupt 65
    pub mod PRI65 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 66
pub mod NVICIP66 {

    /// Priority of the INT_Reserved82 interrupt 66
    pub mod PRI66 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 67
pub mod NVICIP67 {

    /// Priority of the INT_ADC1 interrupt 67
    pub mod PRI67 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 68
pub mod NVICIP68 {

    /// Priority of the INT_ADC2 interrupt 68
    pub mod PRI68 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 69
pub mod NVICIP69 {

    /// Priority of the INT_DCDC interrupt 69
    pub mod PRI69 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 70
pub mod NVICIP70 {

    /// Priority of the INT_Reserved86 interrupt 70
    pub mod PRI70 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 71
pub mod NVICIP71 {

    /// Priority of the INT_Reserved87 interrupt 71
    pub mod PRI71 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 72
pub mod NVICIP72 {

    /// Priority of the INT_GPIO1_INT0 interrupt 72
    pub mod PRI72 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 73
pub mod NVICIP73 {

    /// Priority of the INT_GPIO1_INT1 interrupt 73
    pub mod PRI73 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 74
pub mod NVICIP74 {

    /// Priority of the INT_GPIO1_INT2 interrupt 74
    pub mod PRI74 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 75
pub mod NVICIP75 {

    /// Priority of the INT_GPIO1_INT3 interrupt 75
    pub mod PRI75 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 76
pub mod NVICIP76 {

    /// Priority of the INT_GPIO1_INT4 interrupt 76
    pub mod PRI76 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 77
pub mod NVICIP77 {

    /// Priority of the INT_GPIO1_INT5 interrupt 77
    pub mod PRI77 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 78
pub mod NVICIP78 {

    /// Priority of the INT_GPIO1_INT6 interrupt 78
    pub mod PRI78 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 79
pub mod NVICIP79 {

    /// Priority of the INT_GPIO1_INT7 interrupt 79
    pub mod PRI79 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 80
pub mod NVICIP80 {

    /// Priority of the INT_GPIO1_Combined_0_15 interrupt 80
    pub mod PRI80 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 81
pub mod NVICIP81 {

    /// Priority of the INT_GPIO1_Combined_16_31 interrupt 81
    pub mod PRI81 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 82
pub mod NVICIP82 {

    /// Priority of the INT_GPIO2_Combined_0_15 interrupt 82
    pub mod PRI82 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 83
pub mod NVICIP83 {

    /// Priority of the INT_GPIO2_Combined_16_31 interrupt 83
    pub mod PRI83 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 84
pub mod NVICIP84 {

    /// Priority of the INT_GPIO3_Combined_0_15 interrupt 84
    pub mod PRI84 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 85
pub mod NVICIP85 {

    /// Priority of the INT_GPIO3_Combined_16_31 interrupt 85
    pub mod PRI85 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 86
pub mod NVICIP86 {

    /// Priority of the INT_Reserved102 interrupt 86
    pub mod PRI86 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 87
pub mod NVICIP87 {

    /// Priority of the INT_Reserved103 interrupt 87
    pub mod PRI87 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 88
pub mod NVICIP88 {

    /// Priority of the INT_GPIO5_Combined_0_15 interrupt 88
    pub mod PRI88 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 89
pub mod NVICIP89 {

    /// Priority of the INT_GPIO5_Combined_16_31 interrupt 89
    pub mod PRI89 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 90
pub mod NVICIP90 {

    /// Priority of the INT_FLEXIO1 interrupt 90
    pub mod PRI90 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 91
pub mod NVICIP91 {

    /// Priority of the INT_Reserved107 interrupt 91
    pub mod PRI91 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 92
pub mod NVICIP92 {

    /// Priority of the INT_WDOG1 interrupt 92
    pub mod PRI92 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 93
pub mod NVICIP93 {

    /// Priority of the INT_RTWDOG interrupt 93
    pub mod PRI93 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 94
pub mod NVICIP94 {

    /// Priority of the INT_EWM interrupt 94
    pub mod PRI94 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 95
pub mod NVICIP95 {

    /// Priority of the INT_CCM_1 interrupt 95
    pub mod PRI95 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 96
pub mod NVICIP96 {

    /// Priority of the INT_CCM_2 interrupt 96
    pub mod PRI96 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 97
pub mod NVICIP97 {

    /// Priority of the INT_GPC interrupt 97
    pub mod PRI97 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 98
pub mod NVICIP98 {

    /// Priority of the INT_SRC interrupt 98
    pub mod PRI98 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 99
pub mod NVICIP99 {

    /// Priority of the INT_Reserved115 interrupt 99
    pub mod PRI99 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 100
pub mod NVICIP100 {

    /// Priority of the INT_GPT1 interrupt 100
    pub mod PRI100 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 101
pub mod NVICIP101 {

    /// Priority of the INT_GPT2 interrupt 101
    pub mod PRI101 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 102
pub mod NVICIP102 {

    /// Priority of the INT_PWM1_0 interrupt 102
    pub mod PRI102 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 103
pub mod NVICIP103 {

    /// Priority of the INT_PWM1_1 interrupt 103
    pub mod PRI103 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 104
pub mod NVICIP104 {

    /// Priority of the INT_PWM1_2 interrupt 104
    pub mod PRI104 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 105
pub mod NVICIP105 {

    /// Priority of the INT_PWM1_3 interrupt 105
    pub mod PRI105 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 106
pub mod NVICIP106 {

    /// Priority of the INT_PWM1_FAULT interrupt 106
    pub mod PRI106 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 107
pub mod NVICIP107 {

    /// Priority of the INT_Reserved123 interrupt 107
    pub mod PRI107 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 108
pub mod NVICIP108 {

    /// Priority of the INT_FLEXSPI interrupt 108
    pub mod PRI108 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 109
pub mod NVICIP109 {

    /// Priority of the INT_SEMC interrupt 109
    pub mod PRI109 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 110
pub mod NVICIP110 {

    /// Priority of the INT_USDHC1 interrupt 110
    pub mod PRI110 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 111
pub mod NVICIP111 {

    /// Priority of the INT_USDHC2 interrupt 111
    pub mod PRI111 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 112
pub mod NVICIP112 {

    /// Priority of the INT_Reserved128 interrupt 112
    pub mod PRI112 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 113
pub mod NVICIP113 {

    /// Priority of the INT_USB_OTG1 interrupt 113
    pub mod PRI113 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 114
pub mod NVICIP114 {

    /// Priority of the INT_ENET interrupt 114
    pub mod PRI114 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 115
pub mod NVICIP115 {

    /// Priority of the INT_ENET_1588_Timer interrupt 115
    pub mod PRI115 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 116
pub mod NVICIP116 {

    /// Priority of the INT_XBAR1_IRQ_0_1 interrupt 116
    pub mod PRI116 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 117
pub mod NVICIP117 {

    /// Priority of the INT_XBAR1_IRQ_2_3 interrupt 117
    pub mod PRI117 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 118
pub mod NVICIP118 {

    /// Priority of the INT_ADC_ETC_IRQ0 interrupt 118
    pub mod PRI118 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 119
pub mod NVICIP119 {

    /// Priority of the INT_ADC_ETC_IRQ1 interrupt 119
    pub mod PRI119 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 120
pub mod NVICIP120 {

    /// Priority of the INT_ADC_ETC_IRQ2 interrupt 120
    pub mod PRI120 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 121
pub mod NVICIP121 {

    /// Priority of the INT_ADC_ETC_ERROR_IRQ interrupt 121
    pub mod PRI121 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 122
pub mod NVICIP122 {

    /// Priority of the INT_PIT interrupt 122
    pub mod PRI122 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 123
pub mod NVICIP123 {

    /// Priority of the INT_ACMP1 interrupt 123
    pub mod PRI123 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 124
pub mod NVICIP124 {

    /// Priority of the INT_ACMP2 interrupt 124
    pub mod PRI124 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 125
pub mod NVICIP125 {

    /// Priority of the INT_ACMP3 interrupt 125
    pub mod PRI125 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 126
pub mod NVICIP126 {

    /// Priority of the INT_ACMP4 interrupt 126
    pub mod PRI126 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 127
pub mod NVICIP127 {

    /// Priority of the INT_Reserved143 interrupt 127
    pub mod PRI127 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 128
pub mod NVICIP128 {

    /// Priority of the INT_Reserved144 interrupt 128
    pub mod PRI128 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 129
pub mod NVICIP129 {

    /// Priority of the INT_ENC1 interrupt 129
    pub mod PRI129 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 130
pub mod NVICIP130 {

    /// Priority of the INT_ENC2 interrupt 130
    pub mod PRI130 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 131
pub mod NVICIP131 {

    /// Priority of the INT_Reserved147 interrupt 131
    pub mod PRI131 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 132
pub mod NVICIP132 {

    /// Priority of the INT_Reserved148 interrupt 132
    pub mod PRI132 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 133
pub mod NVICIP133 {

    /// Priority of the INT_TMR1 interrupt 133
    pub mod PRI133 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 134
pub mod NVICIP134 {

    /// Priority of the INT_TMR2 interrupt 134
    pub mod PRI134 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 135
pub mod NVICIP135 {

    /// Priority of the INT_Reserved151 interrupt 135
    pub mod PRI135 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 136
pub mod NVICIP136 {

    /// Priority of the INT_Reserved152 interrupt 136
    pub mod PRI136 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 137
pub mod NVICIP137 {

    /// Priority of the INT_PWM2_0 interrupt 137
    pub mod PRI137 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 138
pub mod NVICIP138 {

    /// Priority of the INT_PWM2_1 interrupt 138
    pub mod PRI138 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 139
pub mod NVICIP139 {

    /// Priority of the INT_PWM2_2 interrupt 139
    pub mod PRI139 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 140
pub mod NVICIP140 {

    /// Priority of the INT_PWM2_3 interrupt 140
    pub mod PRI140 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register 141
pub mod NVICIP141 {

    /// Priority of the INT_PWM2_FAULT interrupt 141
    pub mod PRI141 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Software Trigger Interrupt Register
pub mod NVICSTIR {

    /// Interrupt ID of the interrupt to trigger, in the range 0-239. For example, a value of 0x03 specifies interrupt IRQ3.
    pub mod INTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Interrupt Set Enable Register n
    pub NVICISER0: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER1: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER2: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER3: RWRegister<u32>,

    /// Interrupt Set Enable Register n
    pub NVICISER4: RWRegister<u32>,

    _reserved1: [u32; 27],

    /// Interrupt Clear Enable Register n
    pub NVICICER0: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER1: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER2: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER3: RWRegister<u32>,

    /// Interrupt Clear Enable Register n
    pub NVICICER4: RWRegister<u32>,

    _reserved2: [u32; 27],

    /// Interrupt Set Pending Register n
    pub NVICISPR0: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR1: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR2: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR3: RWRegister<u32>,

    /// Interrupt Set Pending Register n
    pub NVICISPR4: RWRegister<u32>,

    _reserved3: [u32; 27],

    /// Interrupt Clear Pending Register n
    pub NVICICPR0: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR1: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR2: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR3: RWRegister<u32>,

    /// Interrupt Clear Pending Register n
    pub NVICICPR4: RWRegister<u32>,

    _reserved4: [u32; 27],

    /// Interrupt Active bit Register n
    pub NVICIABR0: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR1: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR2: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR3: RWRegister<u32>,

    /// Interrupt Active bit Register n
    pub NVICIABR4: RWRegister<u32>,

    _reserved5: [u32; 59],

    /// Interrupt Priority Register 0
    pub NVICIP0: RWRegister<u8>,

    /// Interrupt Priority Register 1
    pub NVICIP1: RWRegister<u8>,

    /// Interrupt Priority Register 2
    pub NVICIP2: RWRegister<u8>,

    /// Interrupt Priority Register 3
    pub NVICIP3: RWRegister<u8>,

    /// Interrupt Priority Register 4
    pub NVICIP4: RWRegister<u8>,

    /// Interrupt Priority Register 5
    pub NVICIP5: RWRegister<u8>,

    /// Interrupt Priority Register 6
    pub NVICIP6: RWRegister<u8>,

    /// Interrupt Priority Register 7
    pub NVICIP7: RWRegister<u8>,

    /// Interrupt Priority Register 8
    pub NVICIP8: RWRegister<u8>,

    /// Interrupt Priority Register 9
    pub NVICIP9: RWRegister<u8>,

    /// Interrupt Priority Register 10
    pub NVICIP10: RWRegister<u8>,

    /// Interrupt Priority Register 11
    pub NVICIP11: RWRegister<u8>,

    /// Interrupt Priority Register 12
    pub NVICIP12: RWRegister<u8>,

    /// Interrupt Priority Register 13
    pub NVICIP13: RWRegister<u8>,

    /// Interrupt Priority Register 14
    pub NVICIP14: RWRegister<u8>,

    /// Interrupt Priority Register 15
    pub NVICIP15: RWRegister<u8>,

    /// Interrupt Priority Register 16
    pub NVICIP16: RWRegister<u8>,

    /// Interrupt Priority Register 17
    pub NVICIP17: RWRegister<u8>,

    /// Interrupt Priority Register 18
    pub NVICIP18: RWRegister<u8>,

    /// Interrupt Priority Register 19
    pub NVICIP19: RWRegister<u8>,

    /// Interrupt Priority Register 20
    pub NVICIP20: RWRegister<u8>,

    /// Interrupt Priority Register 21
    pub NVICIP21: RWRegister<u8>,

    /// Interrupt Priority Register 22
    pub NVICIP22: RWRegister<u8>,

    /// Interrupt Priority Register 23
    pub NVICIP23: RWRegister<u8>,

    /// Interrupt Priority Register 24
    pub NVICIP24: RWRegister<u8>,

    /// Interrupt Priority Register 25
    pub NVICIP25: RWRegister<u8>,

    /// Interrupt Priority Register 26
    pub NVICIP26: RWRegister<u8>,

    /// Interrupt Priority Register 27
    pub NVICIP27: RWRegister<u8>,

    /// Interrupt Priority Register 28
    pub NVICIP28: RWRegister<u8>,

    /// Interrupt Priority Register 29
    pub NVICIP29: RWRegister<u8>,

    /// Interrupt Priority Register 30
    pub NVICIP30: RWRegister<u8>,

    /// Interrupt Priority Register 31
    pub NVICIP31: RWRegister<u8>,

    /// Interrupt Priority Register 32
    pub NVICIP32: RWRegister<u8>,

    /// Interrupt Priority Register 33
    pub NVICIP33: RWRegister<u8>,

    /// Interrupt Priority Register 34
    pub NVICIP34: RWRegister<u8>,

    /// Interrupt Priority Register 35
    pub NVICIP35: RWRegister<u8>,

    /// Interrupt Priority Register 36
    pub NVICIP36: RWRegister<u8>,

    /// Interrupt Priority Register 37
    pub NVICIP37: RWRegister<u8>,

    /// Interrupt Priority Register 38
    pub NVICIP38: RWRegister<u8>,

    /// Interrupt Priority Register 39
    pub NVICIP39: RWRegister<u8>,

    /// Interrupt Priority Register 40
    pub NVICIP40: RWRegister<u8>,

    /// Interrupt Priority Register 41
    pub NVICIP41: RWRegister<u8>,

    /// Interrupt Priority Register 42
    pub NVICIP42: RWRegister<u8>,

    /// Interrupt Priority Register 43
    pub NVICIP43: RWRegister<u8>,

    /// Interrupt Priority Register 44
    pub NVICIP44: RWRegister<u8>,

    /// Interrupt Priority Register 45
    pub NVICIP45: RWRegister<u8>,

    /// Interrupt Priority Register 46
    pub NVICIP46: RWRegister<u8>,

    /// Interrupt Priority Register 47
    pub NVICIP47: RWRegister<u8>,

    /// Interrupt Priority Register 48
    pub NVICIP48: RWRegister<u8>,

    /// Interrupt Priority Register 49
    pub NVICIP49: RWRegister<u8>,

    /// Interrupt Priority Register 50
    pub NVICIP50: RWRegister<u8>,

    /// Interrupt Priority Register 51
    pub NVICIP51: RWRegister<u8>,

    /// Interrupt Priority Register 52
    pub NVICIP52: RWRegister<u8>,

    /// Interrupt Priority Register 53
    pub NVICIP53: RWRegister<u8>,

    /// Interrupt Priority Register 54
    pub NVICIP54: RWRegister<u8>,

    /// Interrupt Priority Register 55
    pub NVICIP55: RWRegister<u8>,

    /// Interrupt Priority Register 56
    pub NVICIP56: RWRegister<u8>,

    /// Interrupt Priority Register 57
    pub NVICIP57: RWRegister<u8>,

    /// Interrupt Priority Register 58
    pub NVICIP58: RWRegister<u8>,

    /// Interrupt Priority Register 59
    pub NVICIP59: RWRegister<u8>,

    /// Interrupt Priority Register 60
    pub NVICIP60: RWRegister<u8>,

    /// Interrupt Priority Register 61
    pub NVICIP61: RWRegister<u8>,

    /// Interrupt Priority Register 62
    pub NVICIP62: RWRegister<u8>,

    /// Interrupt Priority Register 63
    pub NVICIP63: RWRegister<u8>,

    /// Interrupt Priority Register 64
    pub NVICIP64: RWRegister<u8>,

    /// Interrupt Priority Register 65
    pub NVICIP65: RWRegister<u8>,

    /// Interrupt Priority Register 66
    pub NVICIP66: RWRegister<u8>,

    /// Interrupt Priority Register 67
    pub NVICIP67: RWRegister<u8>,

    /// Interrupt Priority Register 68
    pub NVICIP68: RWRegister<u8>,

    /// Interrupt Priority Register 69
    pub NVICIP69: RWRegister<u8>,

    /// Interrupt Priority Register 70
    pub NVICIP70: RWRegister<u8>,

    /// Interrupt Priority Register 71
    pub NVICIP71: RWRegister<u8>,

    /// Interrupt Priority Register 72
    pub NVICIP72: RWRegister<u8>,

    /// Interrupt Priority Register 73
    pub NVICIP73: RWRegister<u8>,

    /// Interrupt Priority Register 74
    pub NVICIP74: RWRegister<u8>,

    /// Interrupt Priority Register 75
    pub NVICIP75: RWRegister<u8>,

    /// Interrupt Priority Register 76
    pub NVICIP76: RWRegister<u8>,

    /// Interrupt Priority Register 77
    pub NVICIP77: RWRegister<u8>,

    /// Interrupt Priority Register 78
    pub NVICIP78: RWRegister<u8>,

    /// Interrupt Priority Register 79
    pub NVICIP79: RWRegister<u8>,

    /// Interrupt Priority Register 80
    pub NVICIP80: RWRegister<u8>,

    /// Interrupt Priority Register 81
    pub NVICIP81: RWRegister<u8>,

    /// Interrupt Priority Register 82
    pub NVICIP82: RWRegister<u8>,

    /// Interrupt Priority Register 83
    pub NVICIP83: RWRegister<u8>,

    /// Interrupt Priority Register 84
    pub NVICIP84: RWRegister<u8>,

    /// Interrupt Priority Register 85
    pub NVICIP85: RWRegister<u8>,

    /// Interrupt Priority Register 86
    pub NVICIP86: RWRegister<u8>,

    /// Interrupt Priority Register 87
    pub NVICIP87: RWRegister<u8>,

    /// Interrupt Priority Register 88
    pub NVICIP88: RWRegister<u8>,

    /// Interrupt Priority Register 89
    pub NVICIP89: RWRegister<u8>,

    /// Interrupt Priority Register 90
    pub NVICIP90: RWRegister<u8>,

    /// Interrupt Priority Register 91
    pub NVICIP91: RWRegister<u8>,

    /// Interrupt Priority Register 92
    pub NVICIP92: RWRegister<u8>,

    /// Interrupt Priority Register 93
    pub NVICIP93: RWRegister<u8>,

    /// Interrupt Priority Register 94
    pub NVICIP94: RWRegister<u8>,

    /// Interrupt Priority Register 95
    pub NVICIP95: RWRegister<u8>,

    /// Interrupt Priority Register 96
    pub NVICIP96: RWRegister<u8>,

    /// Interrupt Priority Register 97
    pub NVICIP97: RWRegister<u8>,

    /// Interrupt Priority Register 98
    pub NVICIP98: RWRegister<u8>,

    /// Interrupt Priority Register 99
    pub NVICIP99: RWRegister<u8>,

    /// Interrupt Priority Register 100
    pub NVICIP100: RWRegister<u8>,

    /// Interrupt Priority Register 101
    pub NVICIP101: RWRegister<u8>,

    /// Interrupt Priority Register 102
    pub NVICIP102: RWRegister<u8>,

    /// Interrupt Priority Register 103
    pub NVICIP103: RWRegister<u8>,

    /// Interrupt Priority Register 104
    pub NVICIP104: RWRegister<u8>,

    /// Interrupt Priority Register 105
    pub NVICIP105: RWRegister<u8>,

    /// Interrupt Priority Register 106
    pub NVICIP106: RWRegister<u8>,

    /// Interrupt Priority Register 107
    pub NVICIP107: RWRegister<u8>,

    /// Interrupt Priority Register 108
    pub NVICIP108: RWRegister<u8>,

    /// Interrupt Priority Register 109
    pub NVICIP109: RWRegister<u8>,

    /// Interrupt Priority Register 110
    pub NVICIP110: RWRegister<u8>,

    /// Interrupt Priority Register 111
    pub NVICIP111: RWRegister<u8>,

    /// Interrupt Priority Register 112
    pub NVICIP112: RWRegister<u8>,

    /// Interrupt Priority Register 113
    pub NVICIP113: RWRegister<u8>,

    /// Interrupt Priority Register 114
    pub NVICIP114: RWRegister<u8>,

    /// Interrupt Priority Register 115
    pub NVICIP115: RWRegister<u8>,

    /// Interrupt Priority Register 116
    pub NVICIP116: RWRegister<u8>,

    /// Interrupt Priority Register 117
    pub NVICIP117: RWRegister<u8>,

    /// Interrupt Priority Register 118
    pub NVICIP118: RWRegister<u8>,

    /// Interrupt Priority Register 119
    pub NVICIP119: RWRegister<u8>,

    /// Interrupt Priority Register 120
    pub NVICIP120: RWRegister<u8>,

    /// Interrupt Priority Register 121
    pub NVICIP121: RWRegister<u8>,

    /// Interrupt Priority Register 122
    pub NVICIP122: RWRegister<u8>,

    /// Interrupt Priority Register 123
    pub NVICIP123: RWRegister<u8>,

    /// Interrupt Priority Register 124
    pub NVICIP124: RWRegister<u8>,

    /// Interrupt Priority Register 125
    pub NVICIP125: RWRegister<u8>,

    /// Interrupt Priority Register 126
    pub NVICIP126: RWRegister<u8>,

    /// Interrupt Priority Register 127
    pub NVICIP127: RWRegister<u8>,

    /// Interrupt Priority Register 128
    pub NVICIP128: RWRegister<u8>,

    /// Interrupt Priority Register 129
    pub NVICIP129: RWRegister<u8>,

    /// Interrupt Priority Register 130
    pub NVICIP130: RWRegister<u8>,

    /// Interrupt Priority Register 131
    pub NVICIP131: RWRegister<u8>,

    /// Interrupt Priority Register 132
    pub NVICIP132: RWRegister<u8>,

    /// Interrupt Priority Register 133
    pub NVICIP133: RWRegister<u8>,

    /// Interrupt Priority Register 134
    pub NVICIP134: RWRegister<u8>,

    /// Interrupt Priority Register 135
    pub NVICIP135: RWRegister<u8>,

    /// Interrupt Priority Register 136
    pub NVICIP136: RWRegister<u8>,

    /// Interrupt Priority Register 137
    pub NVICIP137: RWRegister<u8>,

    /// Interrupt Priority Register 138
    pub NVICIP138: RWRegister<u8>,

    /// Interrupt Priority Register 139
    pub NVICIP139: RWRegister<u8>,

    /// Interrupt Priority Register 140
    pub NVICIP140: RWRegister<u8>,

    /// Interrupt Priority Register 141
    pub NVICIP141: RWRegister<u8>,

    _reserved6: [u32; 668],
    _reserved7: [u16; 1],

    /// Software Trigger Interrupt Register
    pub NVICSTIR: RWRegister<u32>,
}
pub struct ResetValues {
    pub NVICISER0: u32,
    pub NVICISER1: u32,
    pub NVICISER2: u32,
    pub NVICISER3: u32,
    pub NVICISER4: u32,
    pub NVICICER0: u32,
    pub NVICICER1: u32,
    pub NVICICER2: u32,
    pub NVICICER3: u32,
    pub NVICICER4: u32,
    pub NVICISPR0: u32,
    pub NVICISPR1: u32,
    pub NVICISPR2: u32,
    pub NVICISPR3: u32,
    pub NVICISPR4: u32,
    pub NVICICPR0: u32,
    pub NVICICPR1: u32,
    pub NVICICPR2: u32,
    pub NVICICPR3: u32,
    pub NVICICPR4: u32,
    pub NVICIABR0: u32,
    pub NVICIABR1: u32,
    pub NVICIABR2: u32,
    pub NVICIABR3: u32,
    pub NVICIABR4: u32,
    pub NVICIP0: u8,
    pub NVICIP1: u8,
    pub NVICIP2: u8,
    pub NVICIP3: u8,
    pub NVICIP4: u8,
    pub NVICIP5: u8,
    pub NVICIP6: u8,
    pub NVICIP7: u8,
    pub NVICIP8: u8,
    pub NVICIP9: u8,
    pub NVICIP10: u8,
    pub NVICIP11: u8,
    pub NVICIP12: u8,
    pub NVICIP13: u8,
    pub NVICIP14: u8,
    pub NVICIP15: u8,
    pub NVICIP16: u8,
    pub NVICIP17: u8,
    pub NVICIP18: u8,
    pub NVICIP19: u8,
    pub NVICIP20: u8,
    pub NVICIP21: u8,
    pub NVICIP22: u8,
    pub NVICIP23: u8,
    pub NVICIP24: u8,
    pub NVICIP25: u8,
    pub NVICIP26: u8,
    pub NVICIP27: u8,
    pub NVICIP28: u8,
    pub NVICIP29: u8,
    pub NVICIP30: u8,
    pub NVICIP31: u8,
    pub NVICIP32: u8,
    pub NVICIP33: u8,
    pub NVICIP34: u8,
    pub NVICIP35: u8,
    pub NVICIP36: u8,
    pub NVICIP37: u8,
    pub NVICIP38: u8,
    pub NVICIP39: u8,
    pub NVICIP40: u8,
    pub NVICIP41: u8,
    pub NVICIP42: u8,
    pub NVICIP43: u8,
    pub NVICIP44: u8,
    pub NVICIP45: u8,
    pub NVICIP46: u8,
    pub NVICIP47: u8,
    pub NVICIP48: u8,
    pub NVICIP49: u8,
    pub NVICIP50: u8,
    pub NVICIP51: u8,
    pub NVICIP52: u8,
    pub NVICIP53: u8,
    pub NVICIP54: u8,
    pub NVICIP55: u8,
    pub NVICIP56: u8,
    pub NVICIP57: u8,
    pub NVICIP58: u8,
    pub NVICIP59: u8,
    pub NVICIP60: u8,
    pub NVICIP61: u8,
    pub NVICIP62: u8,
    pub NVICIP63: u8,
    pub NVICIP64: u8,
    pub NVICIP65: u8,
    pub NVICIP66: u8,
    pub NVICIP67: u8,
    pub NVICIP68: u8,
    pub NVICIP69: u8,
    pub NVICIP70: u8,
    pub NVICIP71: u8,
    pub NVICIP72: u8,
    pub NVICIP73: u8,
    pub NVICIP74: u8,
    pub NVICIP75: u8,
    pub NVICIP76: u8,
    pub NVICIP77: u8,
    pub NVICIP78: u8,
    pub NVICIP79: u8,
    pub NVICIP80: u8,
    pub NVICIP81: u8,
    pub NVICIP82: u8,
    pub NVICIP83: u8,
    pub NVICIP84: u8,
    pub NVICIP85: u8,
    pub NVICIP86: u8,
    pub NVICIP87: u8,
    pub NVICIP88: u8,
    pub NVICIP89: u8,
    pub NVICIP90: u8,
    pub NVICIP91: u8,
    pub NVICIP92: u8,
    pub NVICIP93: u8,
    pub NVICIP94: u8,
    pub NVICIP95: u8,
    pub NVICIP96: u8,
    pub NVICIP97: u8,
    pub NVICIP98: u8,
    pub NVICIP99: u8,
    pub NVICIP100: u8,
    pub NVICIP101: u8,
    pub NVICIP102: u8,
    pub NVICIP103: u8,
    pub NVICIP104: u8,
    pub NVICIP105: u8,
    pub NVICIP106: u8,
    pub NVICIP107: u8,
    pub NVICIP108: u8,
    pub NVICIP109: u8,
    pub NVICIP110: u8,
    pub NVICIP111: u8,
    pub NVICIP112: u8,
    pub NVICIP113: u8,
    pub NVICIP114: u8,
    pub NVICIP115: u8,
    pub NVICIP116: u8,
    pub NVICIP117: u8,
    pub NVICIP118: u8,
    pub NVICIP119: u8,
    pub NVICIP120: u8,
    pub NVICIP121: u8,
    pub NVICIP122: u8,
    pub NVICIP123: u8,
    pub NVICIP124: u8,
    pub NVICIP125: u8,
    pub NVICIP126: u8,
    pub NVICIP127: u8,
    pub NVICIP128: u8,
    pub NVICIP129: u8,
    pub NVICIP130: u8,
    pub NVICIP131: u8,
    pub NVICIP132: u8,
    pub NVICIP133: u8,
    pub NVICIP134: u8,
    pub NVICIP135: u8,
    pub NVICIP136: u8,
    pub NVICIP137: u8,
    pub NVICIP138: u8,
    pub NVICIP139: u8,
    pub NVICIP140: u8,
    pub NVICIP141: u8,
    pub NVICSTIR: u32,
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

/// Access functions for the NVIC peripheral instance
pub mod NVIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in NVIC
    pub const reset: ResetValues = ResetValues {
        NVICISER0: 0x00000000,
        NVICISER1: 0x00000000,
        NVICISER2: 0x00000000,
        NVICISER3: 0x00000000,
        NVICISER4: 0x00000000,
        NVICICER0: 0x00000000,
        NVICICER1: 0x00000000,
        NVICICER2: 0x00000000,
        NVICICER3: 0x00000000,
        NVICICER4: 0x00000000,
        NVICISPR0: 0x00000000,
        NVICISPR1: 0x00000000,
        NVICISPR2: 0x00000000,
        NVICISPR3: 0x00000000,
        NVICISPR4: 0x00000000,
        NVICICPR0: 0x00000000,
        NVICICPR1: 0x00000000,
        NVICICPR2: 0x00000000,
        NVICICPR3: 0x00000000,
        NVICICPR4: 0x00000000,
        NVICIABR0: 0x00000000,
        NVICIABR1: 0x00000000,
        NVICIABR2: 0x00000000,
        NVICIABR3: 0x00000000,
        NVICIABR4: 0x00000000,
        NVICIP0: 0x00000000,
        NVICIP1: 0x00000000,
        NVICIP2: 0x00000000,
        NVICIP3: 0x00000000,
        NVICIP4: 0x00000000,
        NVICIP5: 0x00000000,
        NVICIP6: 0x00000000,
        NVICIP7: 0x00000000,
        NVICIP8: 0x00000000,
        NVICIP9: 0x00000000,
        NVICIP10: 0x00000000,
        NVICIP11: 0x00000000,
        NVICIP12: 0x00000000,
        NVICIP13: 0x00000000,
        NVICIP14: 0x00000000,
        NVICIP15: 0x00000000,
        NVICIP16: 0x00000000,
        NVICIP17: 0x00000000,
        NVICIP18: 0x00000000,
        NVICIP19: 0x00000000,
        NVICIP20: 0x00000000,
        NVICIP21: 0x00000000,
        NVICIP22: 0x00000000,
        NVICIP23: 0x00000000,
        NVICIP24: 0x00000000,
        NVICIP25: 0x00000000,
        NVICIP26: 0x00000000,
        NVICIP27: 0x00000000,
        NVICIP28: 0x00000000,
        NVICIP29: 0x00000000,
        NVICIP30: 0x00000000,
        NVICIP31: 0x00000000,
        NVICIP32: 0x00000000,
        NVICIP33: 0x00000000,
        NVICIP34: 0x00000000,
        NVICIP35: 0x00000000,
        NVICIP36: 0x00000000,
        NVICIP37: 0x00000000,
        NVICIP38: 0x00000000,
        NVICIP39: 0x00000000,
        NVICIP40: 0x00000000,
        NVICIP41: 0x00000000,
        NVICIP42: 0x00000000,
        NVICIP43: 0x00000000,
        NVICIP44: 0x00000000,
        NVICIP45: 0x00000000,
        NVICIP46: 0x00000000,
        NVICIP47: 0x00000000,
        NVICIP48: 0x00000000,
        NVICIP49: 0x00000000,
        NVICIP50: 0x00000000,
        NVICIP51: 0x00000000,
        NVICIP52: 0x00000000,
        NVICIP53: 0x00000000,
        NVICIP54: 0x00000000,
        NVICIP55: 0x00000000,
        NVICIP56: 0x00000000,
        NVICIP57: 0x00000000,
        NVICIP58: 0x00000000,
        NVICIP59: 0x00000000,
        NVICIP60: 0x00000000,
        NVICIP61: 0x00000000,
        NVICIP62: 0x00000000,
        NVICIP63: 0x00000000,
        NVICIP64: 0x00000000,
        NVICIP65: 0x00000000,
        NVICIP66: 0x00000000,
        NVICIP67: 0x00000000,
        NVICIP68: 0x00000000,
        NVICIP69: 0x00000000,
        NVICIP70: 0x00000000,
        NVICIP71: 0x00000000,
        NVICIP72: 0x00000000,
        NVICIP73: 0x00000000,
        NVICIP74: 0x00000000,
        NVICIP75: 0x00000000,
        NVICIP76: 0x00000000,
        NVICIP77: 0x00000000,
        NVICIP78: 0x00000000,
        NVICIP79: 0x00000000,
        NVICIP80: 0x00000000,
        NVICIP81: 0x00000000,
        NVICIP82: 0x00000000,
        NVICIP83: 0x00000000,
        NVICIP84: 0x00000000,
        NVICIP85: 0x00000000,
        NVICIP86: 0x00000000,
        NVICIP87: 0x00000000,
        NVICIP88: 0x00000000,
        NVICIP89: 0x00000000,
        NVICIP90: 0x00000000,
        NVICIP91: 0x00000000,
        NVICIP92: 0x00000000,
        NVICIP93: 0x00000000,
        NVICIP94: 0x00000000,
        NVICIP95: 0x00000000,
        NVICIP96: 0x00000000,
        NVICIP97: 0x00000000,
        NVICIP98: 0x00000000,
        NVICIP99: 0x00000000,
        NVICIP100: 0x00000000,
        NVICIP101: 0x00000000,
        NVICIP102: 0x00000000,
        NVICIP103: 0x00000000,
        NVICIP104: 0x00000000,
        NVICIP105: 0x00000000,
        NVICIP106: 0x00000000,
        NVICIP107: 0x00000000,
        NVICIP108: 0x00000000,
        NVICIP109: 0x00000000,
        NVICIP110: 0x00000000,
        NVICIP111: 0x00000000,
        NVICIP112: 0x00000000,
        NVICIP113: 0x00000000,
        NVICIP114: 0x00000000,
        NVICIP115: 0x00000000,
        NVICIP116: 0x00000000,
        NVICIP117: 0x00000000,
        NVICIP118: 0x00000000,
        NVICIP119: 0x00000000,
        NVICIP120: 0x00000000,
        NVICIP121: 0x00000000,
        NVICIP122: 0x00000000,
        NVICIP123: 0x00000000,
        NVICIP124: 0x00000000,
        NVICIP125: 0x00000000,
        NVICIP126: 0x00000000,
        NVICIP127: 0x00000000,
        NVICIP128: 0x00000000,
        NVICIP129: 0x00000000,
        NVICIP130: 0x00000000,
        NVICIP131: 0x00000000,
        NVICIP132: 0x00000000,
        NVICIP133: 0x00000000,
        NVICIP134: 0x00000000,
        NVICIP135: 0x00000000,
        NVICIP136: 0x00000000,
        NVICIP137: 0x00000000,
        NVICIP138: 0x00000000,
        NVICIP139: 0x00000000,
        NVICIP140: 0x00000000,
        NVICIP141: 0x00000000,
        NVICSTIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut NVIC_TAKEN: bool = false;

    /// Safe access to NVIC
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN {
                None
            } else {
                NVIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to NVIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN && inst.addr == INSTANCE.addr {
                NVIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal NVIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        NVIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to NVIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const NVIC: *const RegisterBlock = 0xe000e100 as *const _;
