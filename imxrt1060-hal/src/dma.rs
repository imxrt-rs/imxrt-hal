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
//! use imxrt1060_hal as hal;
//! use hal::{dma::{Circular, Buffer, Linear, Peripheral, bidirectional_u16}, ral};
//!
//! // Circular buffers have alignment requirements
//! #[repr(align(512))]
//! struct Align(Buffer<[u16; 256]>);
//!
//! // Two buffers that can support maximum receive and transfer sizes of 256 elements
//! static RX_BUFFER: Buffer<[u16; 256]> = Buffer::new([0; 256]);
//! static TX_BUFFER: Align = Align(Buffer::new([0; 256]));
//!
//! //
//! // SPI setup...
//! //
//!
//! let pads = ral::iomuxc::IOMUXC::take().map(hal::iomuxc::new).unwrap();
//!
//! let mut ccm = ral::ccm::CCM::take().map(hal::ccm::CCM::from_ral).unwrap();
//!
//! let mut spi_clock = ccm.spi_clock.enable(&mut ccm.handle);
//! let spi_pins = hal::spi::Pins {
//!     sdo: pads.b0.p02,
//!     sdi: pads.b0.p01,
//!     sck: pads.b0.p03,
//!     pcs0: pads.b0.p00,
//! };
//! let mut spi4 = ral::lpspi::LPSPI4::take().unwrap();
//! spi_clock.set_clock_gate(&mut spi4, hal::ccm::ClockGate::On);
//! let mut spi = hal::spi::SPI::new(
//!     spi4,
//!     spi_pins,
//!     &spi_clock,
//! );
//!
//! // Set the SPI clock speed, if desired...
//!
//! //
//! // DMA setup
//! //
//!
//! let mut dma = ral::dma0::DMA0::take().unwrap();
//! ccm.handle.set_clock_gate_dma(&mut *dma, hal::ccm::ClockGate::On);
//! let mut dma_channels = hal::dma::channels(
//!     dma,
//!     ral::dmamux::DMAMUX::take().unwrap(),
//! );
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
//!     spi,
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
pub use imxrt_dma::{Channel, Element, ErrorStatus};

pub use buffer::{Buffer, Circular, CircularError, Drain, Linear, ReadHalf, WriteHalf};
pub use memcpy::Memcpy;
pub use peripheral::{helpers::*, Peripheral};

use crate::ral;

/// The number of DMA channels
pub const CHANNEL_COUNT: usize = 32;

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

/// Initialize and acquire the DMA channels
///
/// The return is 32 channels. However, **only the first [`CHANNEL_COUNT`] channels
/// are initialized to `Some(channel)`. The rest are `None`**.
///
/// You should enable the clock gates before calling `channels`. See
/// [`ccm::Handle::clock_gate_dma`](super::ccm::Handle::set_clock_gate_dma()) for more information.
///
/// # Example
///
/// Initialize and acquire the DMA channels, and move channel 7 to another function:
///
/// ```no_run
/// use imxrt1060_hal as hal;
/// use hal::{ccm::{CCM, ClockGate}, dma};
/// use hal::ral::{dma0, dmamux, ccm};
///
/// fn prepare_peripheral(channel: dma::Channel) { /* ... */ }
///
/// let mut ccm = ccm::CCM::take().map(CCM::from_ral).unwrap();
/// let mut dma = dma0::DMA0::take().unwrap();
/// ccm.handle.set_clock_gate_dma(&mut *dma, ClockGate::On);
/// let mut channels = dma::channels(
///     dma,
///     dmamux::DMAMUX::take().unwrap(),
/// );
///
/// prepare_peripheral(channels[7].take().unwrap());
/// ```
pub fn channels(dma: ral::dma0::Instance, mux: ral::dmamux::Instance) -> [Option<Channel>; 32] {
    drop(dma);
    drop(mux);

    let mut channels = [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None,
    ];

    for (idx, channel) in channels.iter_mut().take(CHANNEL_COUNT).enumerate() {
        let mut c = unsafe { Channel::new(idx) };
        c.reset();
        *channel = Some(c);
    }

    channels
}
