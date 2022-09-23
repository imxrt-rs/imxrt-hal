//! Chip-specific DMA APIs.

use crate::ral;

use crate::common::dma::channel::Channel;

/// The total number of DMA channels.
///
/// This is 16 the minumum number of DMA channels available for all
/// i.MX RT processors. However, if you've enabled a chip family feature
/// and that chip family has more than 16 DMA channels, this value may
/// increase.
pub const CHANNEL_COUNT: usize = crate::chip::config::DMA_CHANNEL_COUNT;

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
