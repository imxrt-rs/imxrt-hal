//! A RAL-like module to support DMA register access
//!
//! The RAL has TONS of symbols for DMA. The script that auto-generates
//! the RAL from a SVD file doesn't represent register clusters as an array
//! of structs. The transfer control descriptions, in particularly, could
//! conveniently be represented by 32 TCD structs. Same with the multiplexer
//! registers. Same with the priority registers...
//!
//! This module lets us hit those ideals. At the same time, we can expose an
//! interface that lets us use the RAL macros, where applicable.

use crate::ral::{RORegister, RWRegister};
use core::ops::Index;

/// DMA multiplexer configuration registers
#[repr(C)]
pub(super) struct MultiplexerRegisters {
    /// Multiplexer configuration registers, one per channel
    pub chcfg: [RWRegister<u32>; 32],
}

pub(super) const MULTIPLEXER: Static<MultiplexerRegisters> = Static(0x400E_C000 as *const _);

impl MultiplexerRegisters {
    pub const ENBL: u32 = 1 << 31;
    pub const A_ON: u32 = 1 << 29;
}

/// DMA registers
#[repr(C)]
pub(super) struct DMARegisters {
    /// Control Register
    pub CR: RWRegister<u32>,
    /// Error Status Register
    pub ES: RORegister<u32>,
    _reserved1: [u32; 1],
    /// Enable Request Register
    pub ERQ: RWRegister<u32>,
    _reserved2: [u32; 1],
    /// Enable Error Interrupt Register
    pub EEI: RWRegister<u32>,
    /// Clear Enable Error Interrupt Register
    pub CEEI: RWRegister<u8>,
    /// Set Enable Error Interrupt Register
    pub SEEI: RWRegister<u8>,
    /// Clear Enable Request Register
    pub CERQ: RWRegister<u8>,
    /// Set Enable Request Register
    pub SERQ: RWRegister<u8>,
    /// Clear DONE Status Bit Register
    pub CDNE: RWRegister<u8>,
    /// Set START Bit Register
    pub SSRT: RWRegister<u8>,
    /// Clear Error Register
    pub CERR: RWRegister<u8>,
    /// Clear Interrupt Request Register
    pub CINT: RWRegister<u8>,
    _reserved3: [u32; 1],
    /// Interrupt Request Register
    pub INT: RWRegister<u32>,
    _reserved4: [u32; 1],
    /// Error Register
    pub ERR: RWRegister<u32>,
    _reserved5: [u32; 1],
    /// Hardware Request Status Register
    pub HRS: RORegister<u32>,
    _reserved6: [u32; 3],
    /// Enable Asynchronous Request in Stop Register
    pub EARS: RWRegister<u32>,
    _reserved7: [u32; 46],
    /// Channel Priority Registers
    pub DCHPRI: ChannelPriorityRegisters,
    _reserved8: [u32; 952],
    /// Transfer Control Descriptors
    pub TCD: [TransferControlDescriptor; 32],
}

pub(super) const DMA: Static<DMARegisters> = Static(0x400E_8000 as *const _);

/// Wrapper for channel priority registers
///
/// Channel priority registers cannot be accessed with
/// normal channel indexes. This adapter makes it so that
/// we *can* access them with channel indexes by converting
/// the channel number to a reference to the priority
/// register.
#[repr(transparent)]
pub(super) struct ChannelPriorityRegisters([RWRegister<u8>; 32]);

impl Index<usize> for ChannelPriorityRegisters {
    type Output = RWRegister<u8>;
    fn index(&self, channel: usize) -> &RWRegister<u8> {
        // Pattern follows
        //
        //   3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, ...
        //
        // for all channels < 32. NXP keeping us on our toes.
        let idx = 4 * (channel / 4) + (3 - (channel % 4));
        &self.0[idx]
    }
}

/// DMA Transfer Control Descriptor (TCD)
#[repr(C, align(32))]
pub(super) struct TransferControlDescriptor {
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
    [0; (32 == core::mem::size_of::<TransferControlDescriptor>()) as usize];

impl TransferControlDescriptor {
    /// TCDs are uninitialized after reset. Set them to a known,
    /// good state here.
    pub(super) fn reset(&self) {
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

//
// Re-export all TCD register fields in our own namespace.
// It lets us abuse the RAL's macros while using our own
// naming convention. We're only exporting those fields for
// registers that have multiple fields.
//
// Arbitrary selection of TCD_*0.
//

pub(super) mod tcd {
    pub mod ATTR {
        pub use crate::ral::dma0::TCD_ATTR0::*;
    }
    pub mod CSR {
        pub use crate::ral::dma0::TCD_CSR0::*;
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
        pub(in crate::dma) fn raw(bwc: Option<Self>) -> u16 {
            match bwc {
                None => CSR::BWC::RW::BWC_0,
                Some(bwc) => bwc as u16,
            }
        }
    }
}

//
// Helper types for static memory
//
// Similar to the RAL's `Instance` type, but more copy.
//

pub(super) struct Static<T>(*const T);
impl<T> core::ops::Deref for Static<T> {
    type Target = T;
    fn deref(&self) -> &'static Self::Target {
        // Safety: pointer points to static memory (peripheral memory)
        unsafe { &*self.0 }
    }
}
impl<T> Clone for Static<T> {
    fn clone(&self) -> Self {
        Static(self.0)
    }
}
impl<T> Copy for Static<T> {}
