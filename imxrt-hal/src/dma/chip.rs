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
#[cfg(any(feature = "imxrt1011"))]
pub const DMA_CHANNEL_COUNT: usize = 16;

/// The number of DMA channels
#[cfg(any(feature = "imxrt1062"))]
pub const DMA_CHANNEL_COUNT: usize = 32;

/// When adding support for a new chip, make sure to either
///
/// - update an existing DMA_CHANNEL_COUNT above, or
/// - create a new DMA_CHANNEL_COUNT constant
///
/// When you add that new chip feature, you should also add
/// it to this list.
#[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062",)))]
pub const DMA_CHANNEL_COUNT: usize =
    compile_error!("No DMA_CHANNEL_COUNT specified for this chip!");


/// Helper symbol to support DMA channel initialization
#[cfg(any(feature = "imxrt1062"))]
pub(crate) const DMA_CHANNEL_INIT: [Option<super::Channel>; DMA_CHANNEL_COUNT] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

/// Helper symbol to support DMA channel initialization
#[cfg(any(feature = "imxrt1011"))]
pub(crate) const DMA_CHANNEL_INIT: [Option<super::Channel>; DMA_CHANNEL_COUNT] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

#[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062")))]
pub(crate) const DMA_CHANNEL_INIT: [Option<super::Channel>; DMA_CHANNEL_COUNT] =
    compile_error!("No DMA_CHANNEL_INIT specified for this chip!");

//
// Conditionally display group priority errors (GPE), and handle
// different masks for error channel (ERRCHN)
//

#[cfg(any(feature = "imxrt1011"))]
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

#[cfg(any(feature = "imxrt1062"))]
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

#[cfg(not(any(feature = "imxrt1011", feature = "imxrt1062",)))]
impl Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        compile_error!("No 'impl Display for ErrorStatus' specified for this chip!")
    }
}