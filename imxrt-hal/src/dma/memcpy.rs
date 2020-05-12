//! DMA-powered memory copy
//!
//! The `memcpy` module lets users execute memory-to-memory DMA transfers. It's like
//! a thread to copy memory between two buffers. Methods that start transfers will
//! return immediately. Then, you may query for DMA completion. Unlike the peripheral
//! DMA support, the memory copy interface does not enable interrupts on completion.
//!
//! # Example
//!
//! There's an `unsafe` interface that requires you to ensure the two source and destination
//! buffers are alive for the lifetime of the transfer:
//!
//! ```no_run
//! use imxrt_hal::dma;
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//! let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
//! let mut memcpy = dma::memcpy::Memcpy::new(dma_channels[7].take().unwrap());
//!
//! let source: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
//! let mut destination: [u8; 4] = [0; 4];
//!
//! // Begin the transfer
//! //
//! // The call returns immediately, so we must ensure that the
//! // source and destination buffers remain alive for the life
//! // of the transfer. We also should not look at destination
//! // until the transfer completes.
//! unsafe { memcpy.transfer(&source, &mut destination); }
//! while !memcpy.complete() {}
//!
//! // Transfer complete! It's safe to look at the destination.
//! // Don't forget to clear the complete signal.
//! memcpy.clear_complete();
//! assert_eq!(destination, source);
//! ```

use super::{Channel, Element, Error, ErrorStatus};
use core::convert::TryFrom;
use core::marker::PhantomData;

/// A type that can peform memory-to-memory
/// DMA transfers
pub struct Memcpy<E> {
    channel: Channel,
    _element: PhantomData<E>,
}

impl<E: Element> Memcpy<E> {
    /// Create a type that can perform memory-to-memory DMA transfers
    pub fn new(mut channel: Channel) -> Self {
        channel.set_interrupt_on_completion(false);
        channel.set_trigger_from_hardware(None);
        Memcpy {
            channel,
            _element: PhantomData,
        }
    }

    /// Take the underlying DMA channel, and destroy the `Memcpy`
    pub fn take(self) -> Channel {
        self.channel
    }

    /// Transfer data from the `source` buffer to the `destination` buffer
    ///
    /// The number of elements transferred is the minimum size of the two
    /// buffers.
    ///
    /// # Safety
    ///
    /// The lifetime of both `source` and `destination` must be greater than
    /// the lifetime of the transfer.
    pub unsafe fn transfer(
        &mut self,
        source: &[E],
        destination: &mut [E],
    ) -> Result<(), Error<void::Void>> {
        if self.active() {
            return Err(Error::ActiveTransfer);
        }

        self.channel.set_source_buffer(source);
        self.channel.set_destination_buffer(destination);
        let iterations = source.len().min(destination.len());
        self.channel.set_transfer_iterations(
            i16::try_from(iterations)
                .map(|iterations| iterations as u16)
                .map_err(|_| Error::TooManyElements(iterations))?,
        );

        self.channel.set_enable(true);
        self.channel.start();
        if self.channel.error() {
            let es = ErrorStatus::new(self.channel.error_status());
            self.channel.clear_error();
            Err(Error::Setup(es))
        } else {
            Ok(())
        }
    }

    /// Returns `true` if the transfer is complete, or `false` if the
    /// transfer is not complete
    pub fn complete(&self) -> bool {
        self.channel.complete()
    }

    /// Clear the completion indication for the DMA transfer
    ///
    /// Users are *required* to clear the completion flag before
    /// starting another transfer.
    pub fn clear_complete(&mut self) {
        self.channel.clear_complete();
    }

    /// Cancel an active transfer
    ///
    /// Does nothing if there is not an active transfer
    pub fn cancel(&mut self) {
        self.channel.set_enable(false);
    }

    /// Returns `true` if there is an active transfer
    pub fn active(&self) -> bool {
        self.channel.active()
    }
}
