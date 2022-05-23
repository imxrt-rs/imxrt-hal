//! DMA interrupt support

use super::{channel::Channel, Error};
use core::{
    cell::RefCell,
    future::Future,
    marker::PhantomPinned,
    pin::Pin,
    sync::atomic,
    task::{Context, Poll, Waker},
};

use cortex_m::interrupt::{self, Mutex};

/// Handle a DMA interrupt
///
/// Checks the interrupt status for the channel identified by `channel`.
/// If the channel completed its transfer, or it's in an error state,
/// `on_interrupt` wakes the channel's waker.
///
/// Consider calling `on_interrupt` in a DMA channel's interrupt handler:
///
/// ```
/// use imxrt_hal as hal;
/// use hal::dma::on_interrupt;
///
/// // #[cortex_m_rt::interrupt]
/// fn DMA7_DMA23() {
///     // Safety: only checking channels 7 and 23, which
///     // are both valid on an i.MX RT 1060 chip.
///     unsafe {
///         on_interrupt(7);
///         on_interrupt(23);
///     }
/// }
/// ```
///
/// # Safety
///
/// Caller must ensure that `on_interrupt` is called in the correct interrupt
/// handler. Caller must ensure that `channel` is valid for the given system,
/// and for the interrupt handler.
///
/// # Panics
///
/// Panics if `channel` is greater than or equal to 32.
#[inline(always)]
pub unsafe fn on_interrupt(channel: usize) {
    let channel = Channel::new(channel);
    if channel.is_interrupt() {
        channel.clear_interrupt();
    }

    if channel.is_complete() | channel.is_error() {
        interrupt::free(|cs| {
            let waker = WAKERS[channel.channel()].borrow(cs);
            let mut waker = waker.borrow_mut();
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        });
    }
}

/// Handle a DMA error on one or more channels
///
/// `on_error` will find all DMA channels below `max_channel` that have
/// an error. `on_error` then wakes that channel's waker, if it exists.
///
/// `max_channel` is the total number of channels, starting from channel 0,
/// that you want to check for errors. This should be 32, or 16 on a system
/// with half the normal DMA channels. However, you may specify fewer channels
/// if you only use a handful of channels.
///
/// Consider calling `on_error` in the DMA error interrupt:
///
/// ```
/// use imxrt_hal as hal;
/// use hal::dma::on_error;
///
/// // #[cortex_m_rt::interrupt]
/// fn DMA_ERROR() {
///     // Safety: on an i.MX RT 1062 chip, there are
///     // 32 DMA channels. This usage is valid.
///     // This usage is NOT valid for an i.MX RT 1011
///     // chip, which only has 16 DMA channels.
///     unsafe { on_error(32) };
/// }
/// ```
///
/// # Safety
///
/// `max_channel` must be valid for your system. Specifying a value greater than
/// 16 on a system that only has 16 channels results in undefined behavior.
///
/// # Panics
///
/// Panics if `max_channel` is greater than 32.
#[inline(always)]
pub unsafe fn on_error(max_channel: usize) {
    interrupt::free(|cs| {
        (0..max_channel)
            .map(|chan| Channel::new(chan))
            .filter(|chan| chan.is_error())
            .flat_map(|channel| {
                let waker = WAKERS[channel.channel()].borrow(cs);
                let mut waker = waker.borrow_mut();
                waker.take()
            })
            .for_each(|waker| {
                waker.wake();
            })
    });
}

type SharedWaker = Mutex<RefCell<Option<Waker>>>;
#[allow(clippy::declare_interior_mutable_const)] // Very convenient, and usage for static init deemed OK in clippy docs
const NO_WAKER: SharedWaker = Mutex::new(RefCell::new(None));
static WAKERS: [SharedWaker; 32] = [NO_WAKER; 32];

/// The core DMA transfer future
///
/// `Transfer` is a future that drives the DMA transfer. `Transfer` will
/// initiate a DMA transfer when it is first polled. You may then poll it
/// to understand when the transfer completes.
///
/// To cancel a transfer, drop the `Transfer`.
///
/// If you've enabled DMA interrupts, consider using [`on_interrupt`](super::on_interrupt)
/// and [`on_error`](super::on_error) to wake an executor when the DMA transfer completes,
/// or the DMA controller observes an error. The interrupt interface assumes that you've
///
/// - configured your channel to generate interrupts
/// - registered a DMA ISR with your embedded runtime
/// - unmasked the DMA interrupt in the NVIC
///
/// `Transfer` calls the unsafe [`enable`](super::channel::Channel::enable) method to enable a
/// DMA transfer. To properly use `Transfer`, make sure that you've configured your DMA
/// channel for a valid transfer.
///
/// `Transfer` is the core DMA future used in `imxrt_dma`. For safe DMA transfers, consider
/// using
///
/// - [`Memcpy`](super::memcpy::Memcpy) for buffer-to-buffer DMA transfers
/// - [`Rx`](super::peripheral::Rx) for peripheral-to-memory DMA transfers
/// - [`Tx`](super::peripheral::Tx) for memory-to-peripheral DMA transfers
///
/// `Transfer` is designed to the DMA `Channel` public interface. If you need to implement
/// your own transfer future, you may do so.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use hal::dma::{channel::Channel, Transfer};
///
/// let my_channel: Channel = // Acquire your channel...
///     # unsafe { Channel::new(0) };
/// // Properly prepare your transfer...
/// // Safety: transfer properly prepared
/// let my_transfer = unsafe { Transfer::new(&my_channel) };
/// // Execute your transfer with a blocking executor...
/// # mod executor { pub fn block<F: core::future::Future>(_: F) {} }
/// executor::block(my_transfer);
/// ```
pub struct Transfer<'channel> {
    channel: &'channel Channel,
    _pinned: PhantomPinned,
}

impl<'channel> Transfer<'channel> {
    /// Create a new `Transfer` that performs the DMA transfer described by `channel`
    ///
    /// # Safety
    ///
    /// Assumes that the transfer is correctly defined in the DMA channel memory.
    /// The transfer enables after the first call to `poll()`.
    pub unsafe fn new(channel: &'channel Channel) -> Self {
        Transfer {
            channel,
            _pinned: PhantomPinned,
        }
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        interrupt::free(|cs| {
            let waker = WAKERS[self.channel.channel()].borrow(cs);
            let mut waker = waker.borrow_mut();
            *waker = Some(cx.waker().clone());
        });

        loop {
            if self.channel.is_error() {
                let es = self.channel.error_status();
                self.channel.clear_error();
                return Poll::Ready(Err(es));
            } else if self.channel.is_complete() {
                self.channel.clear_complete();
                return Poll::Ready(Ok(()));
            } else if self.channel.is_enabled() {
                return Poll::Pending;
            } else {
                atomic::fence(atomic::Ordering::SeqCst);
                unsafe { self.channel.enable() };
            }
        }
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        self.channel.disable();
        self.channel.clear_complete();
        self.channel.clear_error();
        interrupt::free(|cs| {
            let waker = WAKERS[self.channel.channel()].borrow(cs);
            let mut waker = waker.borrow_mut();
            *waker = None;
        });
    }
}
