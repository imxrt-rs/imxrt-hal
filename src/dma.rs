//! Direct memory access.
//!
//! Use the `dma` APIs to perform memory operations without processor intervention.
//! The API supports the following transfers:
//!
//! - peripheral to memory
//! - memory to peripheral
//! - memory to memory
//!
//! Peripheral support depends on the peripheral. See your peripheral's API for details.
//! Methods that use DMA are typically prefixed with `dma`.
//!
//! DMA transfers are modeled as futures. The examples below demonstrate a simple way
//! to start a transfer, then block until completion. But, since these are futures, you
//! may use these futures in `async` code.
//!
//! # DMA channels
//!
//! The API provides access to at least 16 DMA channels. If you've enabled an optional chip
//! family feature, this number may change. See [`CHANNEL_COUNT`] for more information.
//!
//! # Visibility
//!
//! This module is only available when a chip family feature is activated.
//!
//! # Example
//!
//! Use [`channels()`] to access all DMA channels for your processor.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! # fn doc() -> Option<()> {
//! let mut ccm = ral::ccm::CCM::take()?;
//! hal::ccm::clock_gate::dma().set(&mut ccm, hal::ccm::clock_gate::ON);
//!
//! let mut channels = hal::dma::channels(
//!     ral::dma0::DMA0::take()?,
//!     ral::dmamux::DMAMUX::take()?,
//! );
//!
//! // Selecting the 13th DMA channel for our examples...
//! let mut channel = channels[13].take()?;
//! # Some(()) }
//! ```
//!
//! Construct and poll a [`Memcpy`](memcpy::Memcpy) to perform a memory-to-memory transfer.
//!
//! ```no_run
//! # use imxrt_hal as hal;
//! # use imxrt_ral as ral;
//! # let mut channel = unsafe { hal::dma::channel::Channel::new(13) };
//! let source = [4u32, 5, 6, 7];
//! let mut destination = [0u32; 4];
//!
//! let memcpy = hal::dma::memcpy::memcpy(&source, &mut destination, &mut channel);
//! pin_utils::pin_mut!(memcpy);
//! // This poll call begins the transfer.
//! let poll = hal::dma::poll_no_wake(memcpy.as_mut());
//! assert!(poll.is_pending());
//!
//! // Do other work here while the transfer happens...
//!
//! // Wait for the copy to complete.
//! let result = hal::dma::block(memcpy);
//! assert!(result.is_ok());
//! ```
//!
//! For examples of using DMA with a peripheral, see the peripheral's documentation.

use crate::ral;

pub use crate::dma_common::*;

/// The total number of DMA channels.
///
/// This is 16 the minumum number of DMA channels available for all
/// i.MX RT processors. However, if you've enabled a chip family feature
/// and that chip family has more than 16 DMA channels, this value may
/// increase.
pub const CHANNEL_COUNT: usize = crate::chip::DMA_CHANNEL_COUNT;

/// Allocate all DMA channels.
///
/// The number of channels depends on [`CHANNEL_COUNT`], which may change
/// depending on feature selection.
///
/// When `channels` returns, each element is guaranteed to hold `Some` channel.
/// You may then `take()` the channel, leaving `None` in its place.
pub fn channels(_: ral::dma0::DMA0, _: ral::dmamux::DMAMUX) -> [Option<Channel>; CHANNEL_COUNT] {
    const NO_CHANNEL: Option<Channel> = None;
    let mut channels: [Option<Channel>; CHANNEL_COUNT] = [NO_CHANNEL; CHANNEL_COUNT];

    for (idx, channel) in channels.iter_mut().enumerate() {
        // Safety: own the DMA instances, so we're OK to fabricate the channels.
        // It would be unsafe for the user to subsequently access the DMA instances.
        let mut chan = unsafe { Channel::new(idx) };
        chan.reset();
        *channel = Some(chan);
    }
    channels
}
