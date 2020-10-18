//! Direct Memory Access (DMA)
//!
//! We support
//!
//! - DMA from memory to a peripheral. Transfers may be uni- or bi-directional.
//!   See the [`Peripheral`](struct.Peripheral.html) for details.
//! - DMA memory copy, or memory-to-memory transfers. See [`Memcpy`](struct.Memcpy.html)
//!  for details.
//!
//! DMA types support either [`Linear`](struct.Linear.html) or [`Circular`](struct.Circular.html)
//! memory buffers. Either may be used as a DMA transfer source or destination. Both are backed
//! by statically-allocated [`Buffer`s](struct.Buffer.html). A user will create a memory buffer,
//! then pass ownership to a DMA type that defines a transfer. When the transfer completes, the
//! DMA type will release ownership back to the user.
//!
//! # Terms
//!
//! - *Source* is a location in memory that provides data. A source may be a buffer
//!    of data, or a peripheral register.
//! - *Destination* is a location in memory that will receive data.
//!   A destination may be a buffer of data, or a peripheral register.
//! - *Transfer* is an overloaded term, meaning either a DMA transfer, or the movement
//!   of data out of software, through a peripheral, to an external device.
//! - *DMA Transfer* is an operation achieved by the DMA controller to move data from a
//!   source to a destination.
//! - *Receive* means that we're moving data into software, through a peripheral, from an
//!   external device.
//!
//! # Example: Full-Duplex SPI Peripheral
//!
//! In this example, we prepare a SPI peripheral (SPI4) with two DMA
//! channels. One channel will send data; the other will receive data.
//! The example assumes that the user has registered a DMA interrupt
//! handler, since we're enabling an interrupt when the receive completes.
//!
//! ```no_run
//! use imxrt_hal::dma::{Circular, Buffer, Linear, Peripheral, bidirectional_u16};
//!
//! // Circular buffers have alignment requirements
//! #[repr(align(512))]
//! struct Align(Buffer<[u16; 256]>);
//!
//! // Two buffers that can support maximum receive and transfer sizes of 256 elements
//! static RX_BUFFER: Buffer<[u16; 256]> = Buffer::new([0; 256]);
//! static TX_BUFFER: Align = Align(Buffer::new([0; 256]));
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! //
//! // SPI setup...
//! //
//!
//! let (_, _, _, spi4_builder) = peripherals.spi.clock(
//!     &mut peripherals.ccm.handle,
//!     imxrt_hal::ccm::spi::ClockSelect::Pll2,
//!     imxrt_hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
//! );
//!
//! let mut spi4 = spi4_builder.build(
//!     peripherals.iomuxc.b0.p02,
//!     peripherals.iomuxc.b0.p01,
//!     peripherals.iomuxc.b0.p03,
//! );
//!
//! spi4.enable_chip_select_0(peripherals.iomuxc.b0.p00);
//!
//! // Set the SPI clock speed, if desired...
//!
//! //
//! // DMA setup
//! //
//!
//! let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
//!
//! // i.MX RT DMA interrupt handlers manage pairs of DMA channels. There's one
//! // interrupt for DMA channel 9 and channel 25. By selecting these two
//! // DMA channels, we can register one interrupt to handle both DMA channel
//! // completion.
//! let tx_channel = dma_channels[9].take().unwrap();
//! let mut rx_channel = dma_channels[25].take().unwrap();
//!
//! // We only want to interrupt when the receive completes. When
//! // the receive completes, we know that we're also done transferring
//! // data.
//! rx_channel.set_interrupt_on_completion(true);
//!
//! // The peripheral will transfer and receive u16 elements.
//! // It takes ownership of the SPI object, and the two DMA channels.
//! let mut peripheral = bidirectional_u16(
//!     spi4,
//!     tx_channel,
//!     rx_channel,
//! );
//!
//! // Create DMA memory adapters over the statically-allocated DMA memory.
//! // These adapters will 'own' the statically-allocated memory. See the
//! // Linear and Circular docs for more information.
//! let mut tx_buffer = Circular::new(&TX_BUFFER.0).unwrap();
//! let mut rx_buffer = Linear::new(&RX_BUFFER).unwrap();
//!
//! // Send 6 elements, and expect to receive 6 elements
//! for v in 1..=6 {
//!     tx_buffer.push(v);
//! }
//! rx_buffer.set_transfer_len(6);
//!
//! // Start the DMA transfers
//! peripheral.start_receive(rx_buffer).unwrap();
//! peripheral.start_transfer(tx_buffer).unwrap();
//!
//! // At this point, the DMA controller is transferring data from
//! // the SPI peripheral, and receiving data from the SPI peripheral.
//! // Received data appears in RX_BUFFER. The DMA controller will trigger
//! // an interrupt for DMA channel 25 when it has transferred 6 u16s.
//! //
//! // Your ISR should clear the interrupt and complete the transfers,
//! // as depicted below:
//!
//! while peripheral.is_receive_interrupt() {
//!     peripheral.receive_clear_interrupt();
//! }
//!
//! let mut rx_buffer = None;
//! if peripheral.is_receive_complete() {
//!     // Recover the receive buffer
//!     rx_buffer = peripheral.receive_complete();
//! }
//!
//! let mut tx_buffer = None;
//! if peripheral.is_transfer_complete() {
//!     // Recover the transfer buffer
//!     tx_buffer = peripheral.transfer_complete();
//! }
//! ```
//!
//! # Example: memcpy
//!
//! See the [`Memcpy`](struct.Memcpy.html#example) documentation for an example of DMA-powered memcpy.
//!
//! # Notes on Data Cache
//!
//! If your i.MX RT system is using a data cache (DCache), you're responsible for issuing memory barriers,
//! and flushing any cached buffers, for the DMA controller. More generally, you're responsible for placing
//! your DMA buffers into memory regions that the DMA controller can use.
//!
//! ## TODO
//!
//! - Channel arbitration modes
//! - Channel grouping
//! - Channel priority, and channel priority swapping
//! - Channel chaining

mod buffer;
mod memcpy;
pub(crate) mod peripheral;

use imxrt_dma::Transfer;
pub use imxrt_dma::{Channel, Element, ErrorStatus, CHANNEL_COUNT};

pub use buffer::{Buffer, Circular, CircularError, Drain, Linear, ReadHalf, WriteHalf};
pub use memcpy::Memcpy;
pub use peripheral::{helpers::*, Peripheral};

use crate::{ccm, ral};

/// An error when preparing a transfer
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// There is already a scheduled transfer
    ///
    /// Cancel the transfer and try again.
    ScheduledTransfer,
    /// Error setting up the DMA transfer
    Setup(ErrorStatus),
}

/// Helper symbol to support DMA channel initialization
///
/// We always provide users with an array of 32 channels. But, only the first `CHANNEL_COUNT`
/// channels are initialized.
const DMA_CHANNEL_INIT: [Option<Channel>; 32] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

/// Unclocked, uninitialized DMA channels
///
/// Use [`clock()`](struct.Unclocked.html#method.clock) to initialize and acquire all DMA channels
///
/// ```no_run
/// let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
///
/// let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
/// let channel_27 = dma_channels[27].take().unwrap();
/// let channel_0 = dma_channels[0].take().unwrap();
/// ```
pub struct Unclocked([Option<Channel>; CHANNEL_COUNT]);
impl Unclocked {
    pub(crate) fn new(dma: ral::dma0::Instance, mux: ral::dmamux::Instance) -> Self {
        // Explicitly dropping instances
        //
        // Users should see these as "taken" by the HAL's DMA module, although they're not
        // used in the implementation.
        drop(dma);
        drop(mux);

        Unclocked(DMA_CHANNEL_INIT)
    }
    /// Enable the clocks for the DMA peripheral
    ///
    /// The return is an array of 32 channels. However, **only the first [`CHANNEL_COUNT`](constant.CHANNEL_COUNT.html) channels
    /// are initialized to `Some(channel)`. The rest are `None`.**
    ///
    /// Users may take channels as needed. The index in the array maps to the DMA channel number.
    pub fn clock(mut self, ccm: &mut ccm::Handle) -> [Option<Channel>; 32] {
        let (ccm, _) = ccm.raw();
        ral::modify_reg!(ral::ccm, ccm, CCGR5, CG3: 0x03);
        for (idx, channel) in self.0.iter_mut().take(CHANNEL_COUNT).enumerate() {
            // Safety: because we have the DMA instance, we assume that we own the DMA
            // peripheral. That means we own all the DMA channels.
            let mut chan = unsafe { Channel::new(idx) };
            chan.reset();
            *channel = Some(chan);
        }
        self.0
    }
}
