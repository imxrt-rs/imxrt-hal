//! DMA-powered memory copy

use super::{buffer, Channel, Element, Error};
use core::{
    marker::PhantomData,
    sync::atomic::{compiler_fence, Ordering},
};

/// A type that can peform memory-to-memory
/// DMA transfers
///
/// Methods that start transfers will
/// return immediately. Then, you may query for DMA completion.
///
/// A `Memcpy` accepts either a [`Linear`](struct.Linear.html)
/// or a [`Circular`](struct.Circular.html) buffer.
///
/// # Example
///
/// ```no_run
/// use imxrt_hal::dma;
///
/// static SOURCE: dma::Buffer<[u8; 32]> = dma::Buffer::new([0; 32]);
/// static DESTINATION: dma::Buffer<[u8; 64]> = dma::Buffer::new([0; 64]);
///
/// let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
/// let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
/// let mut dma_channel = dma_channels[7].take().unwrap();
/// dma_channel.set_interrupt_on_completion(false);
///
/// let mut memcpy = dma::Memcpy::new(dma_channel);
///
/// let mut source = dma::Linear::new(&SOURCE).unwrap();
/// let mut destination = dma::Linear::new(&DESTINATION).unwrap();
///
/// source.as_mut_elements()[..14].copy_from_slice(&[8; 14]);
/// source.set_transfer_len(14);
/// destination.set_transfer_len(12);
/// // Total 12 elements transferred
///
/// // Begin the transfer
/// memcpy.transfer(source, destination).unwrap();
///
/// // Wait for the transfer...
/// while !memcpy.is_complete() {}
///
/// // Transfer complete! It's safe to look at the destination.
/// // Don't forget to clear the complete signal.
/// let (source, destination) = memcpy.complete().unwrap().unwrap();
/// ```
pub struct Memcpy<E, S, D> {
    channel: Channel,
    buffers: Option<(S, D)>,
    _element: PhantomData<E>,
}

impl<E: Element, S, D> Memcpy<E, S, D>
where
    S: buffer::Source<E>,
    D: buffer::Destination<E>,
{
    /// Create a type that can perform memory-to-memory DMA transfers
    pub fn new(mut channel: Channel) -> Self {
        channel.set_always_on();
        channel.set_disable_on_completion(true);
        Memcpy {
            channel,
            buffers: None,
            _element: PhantomData,
        }
    }

    /// Take the underlying DMA channel, and destroy the `Memcpy`
    pub fn take(self) -> Channel {
        self.channel
    }

    /// Transfer data from the `source` buffer to the `destination` buffer
    ///
    /// If `transfer()` returns `Ok(())`, the transfer is in progress. Use [`is_complete()`](struct.Memcpy.html#method.is_complete)
    /// to check on the transfer status.
    ///
    /// The number of elements transferred is the minimum size of the two
    /// buffers.
    pub fn transfer(&mut self, mut source: S, mut destination: D) -> Result<(), (S, D, Error)> {
        if self.buffers.is_some() || self.channel.is_enabled() {
            return Err((source, destination, Error::ScheduledTransfer));
        }

        let src = source.source();
        let dst = destination.destination();

        unsafe {
            self.channel.set_source_transfer(&src);
            self.channel.set_destination_transfer(&dst);
        }

        source.prepare_source();
        destination.prepare_destination();

        let length = source.source_len().min(destination.destination_len()) as u16;

        self.channel.set_minor_loop_elements::<E>(1);
        self.channel.set_transfer_iterations(length);

        compiler_fence(Ordering::Release);
        self.channel.set_enable(true);
        self.channel.start();
        if self.channel.is_error() {
            let es = self.channel.error_status();
            self.channel.clear_error();
            Err((source, destination, Error::Setup(es)))
        } else {
            self.buffers = Some((source, destination));
            Ok(())
        }
    }

    /// Returns `true` if the transfer is complete, or `false` if the
    /// transfer is not complete
    ///
    /// Once `is_complete()` returns `true`, you should finish the transfer
    /// by calling [`complete()`](struct.Memcpy.html#method.complete).
    pub fn is_complete(&self) -> bool {
        self.channel.is_complete()
    }

    /// Returns `true` if this transfer has generated an interrupt
    pub fn is_interrupt(&self) -> bool {
        self.channel.is_interrupt()
    }

    /// Clears the interrupt flag on the channel
    ///
    /// Users are **required** to clear the interrupt flag, or the hardware
    /// may continue to generate interrupts for the channel. This must be called
    /// for completion interrupts and half-transfer interrupts.
    pub fn clear_interrupt(&mut self) {
        self.channel.clear_interrupt();
    }

    /// Complete the memory-to-memory the DMA transfer
    ///
    /// If `complete()` is called before the transfer is complete,
    /// the transfer is canceled. If the transfer is cancelled, the contents of the receive
    /// buffer are unspecified. Await `is_complete()` before calling `complete()` to avoid
    /// early transfer cancellation.
    ///
    /// - `None` indicates that there's no scheduled transfer; we have no buffers
    /// - `Some(Ok(..))` indicates that the transfer was complete when `complete()` was called
    /// - `Some(Err(..))` indicates that the transfer was in progress, but was cancelled
    pub fn complete(&mut self) -> Option<Result<(S, D), (S, D)>> {
        self.buffers.take().map(|(mut source, mut destination)| {
            if self.is_complete() {
                self.channel.clear_complete();
                source.complete_source();
                destination.complete_destination();
                Ok((source, destination))
            } else {
                self.channel.set_enable(false);
                self.channel.clear_complete();
                Err((source, destination))
            }
        })
    }
}
