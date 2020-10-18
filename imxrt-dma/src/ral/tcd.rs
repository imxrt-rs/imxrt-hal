//! Transfer Control Descriptor

#![allow(non_snake_case, non_upper_case_globals)]

use super::register::RWRegister;

/// DMA Transfer Control Descriptor (TCD)
#[repr(C, align(32))]
pub struct RegisterBlock {
    pub SADDR: RWRegister<u32>,
    // Signed numbers for offsets / 'last' members intentional.
    // The hardware treats them as signed numbers.
    pub SOFF: RWRegister<i16>,
    pub ATTR: RWRegister<u16>,
    pub NBYTES: RWRegister<u32>,
    pub SLAST: RWRegister<i32>,
    pub DADDR: RWRegister<u32>,
    pub DOFF: RWRegister<i16>,
    pub CITER: RWRegister<u16>,
    pub DLAST_SGA: RWRegister<i32>,
    pub CSR: RWRegister<u16>,
    pub BITER: RWRegister<u16>,
}

const _STATIC_ASSERT_TCD_32_BYTES: [u32; 1] =
    [0; (32 == core::mem::size_of::<RegisterBlock>()) as usize];

impl RegisterBlock {
    /// TCDs are uninitialized after reset. Set them to a known,
    /// good state here.
    pub fn reset(&self) {
        self.SADDR.write(0);
        self.SOFF.write(0);
        self.ATTR.write(0);
        self.NBYTES.write(0);
        self.SLAST.write(0);
        self.DADDR.write(0);
        self.DOFF.write(0);
        self.CITER.write(0);
        self.DLAST_SGA.write(0);
        self.CSR.write(0);
        self.BITER.write(0);
    }
}

pub mod ATTR {
    /// Destination data transfer size
    pub mod DSIZE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Destination Address Modulo
    pub mod DMOD {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source data transfer size
    pub mod SSIZE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {}
    }

    pub mod SMOD {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {}
    }
}
pub mod CSR {

    /// Enable an interrupt when major iteration count completes.
    pub mod INTMAJOR {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {}
    }

    /// Disable Request
    pub mod DREQ {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {}
    }

    /// Channel Done
    pub mod DONE {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bandwidth Control
    pub mod BWC {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No eDMA engine stalls.
            pub const BWC_0: u16 = 0b00;

            /// 0b10: eDMA engine stalls for 4 cycles after each R/W.
            pub const BWC_2: u16 = 0b10;

            /// 0b11: eDMA engine stalls for 8 cycles after each R/W.
            pub const BWC_3: u16 = 0b11;
        }
    }

    /// Channel Active
    pub mod ACTIVE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Throttles the amount of bus bandwidth consumed by the eDMA
///
/// Defines the number of stalls that the DMA engine will insert
/// between most element transfers.
///
/// Some stalls may not occur to minimize startup latency. See the
/// reference manual for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum BandwidthControl {
    /// DMA engine stalls for 4 cycles after each R/W.
    Stall4Cycles = CSR::BWC::RW::BWC_2,
    /// DMA engine stalls for 8 cycles after each R/W.
    Stall8Cycles = CSR::BWC::RW::BWC_3,
}

impl BandwidthControl {
    pub(crate) fn raw(bwc: Option<Self>) -> u16 {
        match bwc {
            None => CSR::BWC::RW::BWC_0,
            Some(bwc) => bwc as u16,
        }
    }
}
