//! Direct Memory Access (DMA) driver for i.MX RT processors.
//!
//! This module provides
//!
//! - an unsafe API for defining and scheduling transfers with DMA `Channel`s
//!   and `Transfer`s
//! - safe DMA futures for memcpy, peripheral-to-memory, and memory-to-peripheral
//!   transfers
//!
//! This DMA driver may be re-exported from a HAL. If it is, you should consider
//! using the safer APIs provided by your HAL.
//!
//! # Portability
//!
//! This DMA driver works across all considered i.MX RT variants (1010 and 1060
//! family). You must make sure that the DMA channel you're creating is valid for
//! your i.MX RT processor. This only matters on i.MX RT 1010 processors, which
//! only support 16 DMA channels. Creating an invalid channel for your 1010 processor
//! will result in a channel that references reserved memory.

pub mod channel;
mod element;
mod error;
mod interrupt;
pub mod memcpy;
pub mod peripheral;
mod ral;

pub(crate) use channel::Channel;
pub use element::Element;
pub use error::Error;
pub use interrupt::{on_error, on_interrupt, Transfer};
pub use ral::tcd::BandwidthControl;

/// A DMA result
pub type Result<T> = core::result::Result<T, Error>;

use core::{future::Future, pin::Pin, task::Poll};

/// Poll a future with a dummy waker.
///
/// Use `poll_no_wake` when you want to drive a future to completion, but you
/// don't care about the future waking an executor. It may be used to initiate
/// a DMA transfer that will later be awaited with [`block`].
///
/// Do not use `poll_no_wake` if you want an executor to be woken when the DMA
/// transfer completes.
pub fn poll_no_wake<F>(future: Pin<&mut F>) -> Poll<F::Output>
where
    F: Future,
{
    use core::task::{Context, RawWaker, RawWakerVTable, Waker};
    const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});

    const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
    // Safety: raw waker meets documented requirements.
    let waker = unsafe { Waker::from_raw(RAW_WAKER) };
    let mut context = Context::from_waker(&waker);
    future.poll(&mut context)
}

/// Block until the future returns a result.
///
/// `block` invokes [`poll_no_wake`] in a loop until the future
/// returns a result. Consider using `block` after starting a transfer
/// with `poll_no_wake`, and after doing other work.
pub fn block<F>(mut future: Pin<&mut F>) -> F::Output
where
    F: Future,
{
    loop {
        match poll_no_wake(future.as_mut()) {
            Poll::Ready(result) => return result,
            Poll::Pending => {}
        }
    }
}
