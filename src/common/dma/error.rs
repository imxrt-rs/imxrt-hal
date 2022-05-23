//! DMA error status

use core::fmt::{self, Debug, Display};

/// A wrapper around a DMA error status value
///
/// The wrapper contains a copy of the DMA controller's
/// error status register at the point of an error. The
/// wrapper implements both `Debug` and `Display`. Format
/// the error to see a summary of the error bits.
#[derive(Clone, Copy)]
pub struct Error {
    /// The raw error status
    es: u32,
}

impl Error {
    #[inline(always)]
    pub(crate) const fn new(es: u32) -> Self {
        Error { es }
    }
    /// Returns the raw error status value
    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.es
    }
    #[inline(always)]
    const fn is_bit(self, bit: u32) -> bool {
        (self.es >> bit) & 1 != 0
    }
    /// Logical OR of all DMA channel error status bits
    ///
    /// If you have an `Error` this should always be true.
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        self.is_bit(31)
    }
    /// Indicates if the transfer was cancelled
    #[inline(always)]
    pub const fn is_cancelled(self) -> bool {
        self.is_bit(16)
    }
    /// Indicates a group priority error
    #[inline(always)]
    pub const fn is_group_priority(self) -> bool {
        self.is_bit(15)
    }
    /// Indicates a channel priority error
    #[inline(always)]
    pub const fn is_channel_priority(self) -> bool {
        self.is_bit(14)
    }
    /// Indicates the channel number
    #[inline(always)]
    pub const fn channel_number(self) -> u32 {
        (self.es >> 8) & 0x1F
    }
    /// Indicates a source address error
    #[inline(always)]
    pub const fn is_source_address(self) -> bool {
        self.is_bit(7)
    }
    /// Indicates a source offset error
    #[inline(always)]
    pub const fn is_source_offset(self) -> bool {
        self.is_bit(6)
    }
    /// Indicates a destination address error
    #[inline(always)]
    pub const fn is_destination_address(self) -> bool {
        self.is_bit(5)
    }
    /// Indicates a destination offset error
    #[inline(always)]
    pub const fn is_destination_offset(self) -> bool {
        self.is_bit(4)
    }
    /// Indicates a minor / major loop configuration error
    #[inline(always)]
    pub const fn is_loop_configuration(self) -> bool {
        self.is_bit(3)
    }
    /// Indicates a scatter / gather configuration error
    #[inline(always)]
    pub const fn is_scatter_gather(self) -> bool {
        self.is_bit(2)
    }
    /// Indicates a source bus error
    #[inline(always)]
    pub const fn is_source_bus(self) -> bool {
        self.is_bit(1)
    }
    /// Indicates a destination bus error
    #[inline(always)]
    pub const fn is_destination_bus(self) -> bool {
        self.is_bit(0)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DMA_ES({:#010X})", self.es)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
             "DMA_ES: VLD {vld} ECX {ecx} GPE {gpe} CPE {cpe} ERRCHN {errchn} SAE {sae} SOE {soe} DAE {dae} DOE {doe} NCE {nce} SGE {sge} SBE {sbe} DBE {dbe}",
             vld = self.is_valid() as u32,
             ecx = self.is_cancelled() as u32,
             gpe = self.is_group_priority() as u32,
             cpe = self.is_channel_priority() as u32,
             errchn = self.channel_number(),
             sae = self.is_source_address() as u32,
             soe = self.is_source_offset() as u32,
             dae = self.is_destination_address() as u32,
             doe = self.is_destination_offset() as u32,
             nce = self.is_loop_configuration() as u32,
             sge = self.is_scatter_gather() as u32,
             sbe = self.is_source_bus() as u32,
             dbe = self.is_destination_bus() as u32,
         )
    }
}
