//! DMA-powered memory copy

use super::{buffer, Channel, Element, Error, ErrorStatus};
use core::{
    marker::PhantomData,
    sync::atomic::{compiler_fence, Ordering},
};

/// A type that can peform memory-to-memory
/// DMA transfers
///
/// Methods that start transfers will
/// return immediately. Then, you may query for DMA completion. Unlike the peripheral
/// DMA support, the memory copy interface **does not** enable interrupts on completion.
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
/// let mut memcpy = dma::Memcpy::new(dma_channels[7].take().unwrap());
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
        channel.set_interrupt_on_completion(false);
        channel.set_interrupt_on_half(false);
        channel.set_trigger_from_hardware(None);
        channel.set_disable_on_completion(false);
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
    pub fn transfer(
        &mut self,
        mut source: S,
        mut destination: D,
    ) -> Result<(), (S, D, Error<void::Void>)> {
        if self.buffers.is_some() || self.channel.is_complete() {
            return Err((source, destination, Error::ScheduledTransfer));
        }

        let src = source.source();
        let dst = destination.destination();

        unsafe {
            self.channel.set_source_transfer(src);
            self.channel.set_destination_transfer(dst);
        }

        source.prepare_source();
        destination.prepare_destination();

        let length = src.len().min(dst.len());

        self.channel.set_minor_loop_elements::<E>(length);
        self.channel.set_transfer_iterations(1);

        compiler_fence(Ordering::Release);

        self.channel.start();
        if self.channel.is_error() {
            let es = ErrorStatus::new(self.channel.error_status());
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
                super::halted(|halt| {
                    self.channel.cancel(halt);
                });
                self.channel.clear_complete();
                Err((source, destination))
            }
        })
    }
}
