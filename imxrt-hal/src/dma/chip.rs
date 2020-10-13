//! The DMA chip module contains any DMA configurations that are
//! particular for a i.MX RT chip or chip family
//!
//! Things that are known to differ:
//!
//! - DMA channels
//! - Availability of DMA channel grouping, group scheduling, and error indications. As
//!   of this writing, DMA channel grouping and group scheduling is not implemented, so
//!   we can ignore that for now. We vary the way we display `ErrorStatus` messages.

use super::ErrorStatus;
use core::fmt::{self, Display};

//
// Specify the DMA channel count
//

/// The number of DMA channels
#[cfg(feature = "imxrt1011")]
pub const DMA_CHANNEL_COUNT: usize = 16;

/// The number of DMA channels
#[cfg(not(feature = "imxrt1011"))]
pub const DMA_CHANNEL_COUNT: usize = 32;

/// Helper symbol to support DMA channel initialization
#[cfg(not(feature = "imxrt1011"))]
pub(crate) const DMA_CHANNEL_INIT: [Option<super::Channel>; DMA_CHANNEL_COUNT] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

/// Helper symbol to support DMA channel initialization
#[cfg(feature = "imxrt1011")]
pub(crate) const DMA_CHANNEL_INIT: [Option<super::Channel>; DMA_CHANNEL_COUNT] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

//
// Conditionally display group priority errors (GPE), and handle
// different masks for error channel (ERRCHN)
//

#[cfg(feature = "imxrt1011")]
impl Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "DMA_ES: VLD {vld} ECX {ecx} CPE {cpe} ERRCHN {errchn} SAE {sae} SOE {soe} DAE {dae} DOE {doe} NCE {nce} SGE {sge} SBE {sbe} DBE {dbe}",
            vld = (self.es >> 31) & 0x1,
            ecx = (self.es >> 16) & 0x1,
            // No GPE
            cpe = (self.es >> 14) & 0x1,
            errchn = (self.es >> 8) & 0x0F, // Four bits for error channel
            sae = (self.es >> 7) & 0x1,
            soe = (self.es >> 6) & 0x1,
            dae = (self.es >> 5) & 0x1,
            doe = (self.es >> 4) & 0x1,
            nce = (self.es >> 3) & 0x1,
            sge = (self.es >> 2) & 0x1,
            sbe = (self.es >> 1) & 0x1,
            dbe = self.es & 0x1
        )
    }
}

#[cfg(not(feature = "imxrt1011"))]
impl Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "DMA_ES: VLD {vld} ECX {ecx} GPE {gpe} CPE {cpe} ERRCHN {errchn} SAE {sae} SOE {soe} DAE {dae} DOE {doe} NCE {nce} SGE {sge} SBE {sbe} DBE {dbe}",
            vld = (self.es >> 31) & 0x1,
            ecx = (self.es >> 16) & 0x1,
            gpe = (self.es >> 15) & 0x1,
            cpe = (self.es >> 14) & 0x1,
            errchn = (self.es >> 8) & 0x1F, // Five bits for error channel
            sae = (self.es >> 7) & 0x1,
            soe = (self.es >> 6) & 0x1,
            dae = (self.es >> 5) & 0x1,
            doe = (self.es >> 4) & 0x1,
            nce = (self.es >> 3) & 0x1,
            sge = (self.es >> 2) & 0x1,
            sbe = (self.es >> 1) & 0x1,
            dbe = self.es & 0x1
        )
    }
}

//
// These don't vary across chips.
// They're only in this module to
// show that we thought about that.
//

/// Address to the DMA multiplexer registers
pub(crate) const DMA_MULTIPLEXER_ADDRESS: u32 = 0x400E_C000;
/// Address to the DMA peripheral registers
pub(crate) const DMA_ADDRESS: u32 = 0x400E_8000;
