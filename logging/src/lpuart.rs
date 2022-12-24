//! LPUART and DMA logging backend.
//!
//! There's two phases of initialization: `try_init` and `finish_init`.
//! See their documentation for more information. You must call `try_init`
//! before `finish_init`. They're separated so that you can start initialization
//! with a borrowed producer, then finish the initialization once everything
//! is OK from the first phase.

use core::mem::MaybeUninit;

use imxrt_hal::{
    dma::{channel, peripheral::Destination},
    lpuart::{Direction, Lpuart},
};

static mut CONSUMER: MaybeUninit<crate::Consumer> = MaybeUninit::uninit();
static mut CHANNEL: MaybeUninit<channel::Channel> = MaybeUninit::uninit();

pub(crate) const VTABLE: crate::PollerVTable = crate::PollerVTable { poll };

/// Drive the logging behavior.
///
/// # Safety
///
/// This may only be called from one execution context. It can only be called
/// after `CONSUMER` and `CHANNEL` are initialized.
///
/// By exposing this function through a [`Poller`](crate::Poller), we make both
/// of these guarantees. The `Poller` indirectly "owns" the static mut memory,
/// and the crate design ensures that there's only one `Poller` object in existence.
unsafe fn poll() {
    let consumer = CONSUMER.assume_init_mut();
    let channel = CHANNEL.assume_init_mut();

    // Could be high if the user enabled DMA interrupts.
    while channel.is_interrupt() {
        channel.clear_interrupt();
    }

    assert!(!channel.is_error(), "{:?}", channel.error_status());

    // Don't schedule another transfer while one is enabled.
    // DMA channel configuration will automatically disable
    // when the transfer completes (set_disable_on_completion).
    if channel.is_enabled() {
        return;
    }

    let complete = {
        let mut complete = false;
        while channel.is_complete() {
            channel.clear_complete();
            complete = true;
        }
        complete
    };

    // If we're at this point, there's no active transfer. So if
    // there's data available, we should try to schedule that transfer.
    //
    // The DMA controller references a slice of the buffer that we haven't
    // yet released. If the last transfer completed, it's time for us to
    // release that buffer for the producer.
    //
    // The goal is to only call read once, and handle cases where
    // transfers complete, and / or there's new data. It helps to decompose
    // into two branches, then studying the paths through these two branches.
    //
    //  if complete { /* Call read, and release grant based on last transfer size */}
    //  if let Ok(grant) = consumer.read() {
    //    /* Schedule next transfer with grant contents. */
    //  }
    if let Ok(grant) = consumer.read() {
        // completed holds whatever we previously transferred.
        // new either holds
        //
        // 1. the data accumulated since the start of the last transfer.
        // 2. all of the data accumulated since the last call to poll.
        let (completed, new) = if complete {
            let transferred: usize = channel.beginning_transfer_iterations().into();
            let buf = grant.buf();
            (&buf[..transferred], &buf[transferred..])
        } else {
            (&[][..], grant.buf())
        };

        if !new.is_empty() {
            channel::set_source_linear_buffer(channel, new);
            channel.set_transfer_iterations(new.len().min(u16::MAX as usize) as u16);
            channel.enable();
        }

        if !completed.is_empty() {
            let completed = completed.len();
            grant.release(completed);
        }
    }
}

/// Initialize the LPUART logger.
///
/// # Safety
///
/// This call must only be called once. This call must happen before `poll` is invoked.
pub(crate) unsafe fn init<P, const LPUART: u8>(
    mut lpuart: Lpuart<P, LPUART>,
    channel: channel::Channel,
    consumer: crate::Consumer,
    interrupts: crate::Interrupts,
) {
    channel.disable();
    channel.clear_complete();
    channel.clear_error();
    channel.set_minor_loop_bytes(core::mem::size_of::<u8>() as u32);

    CONSUMER.write(consumer);
    let channel = CHANNEL.write(channel);

    channel.set_disable_on_completion(true);
    channel.set_interrupt_on_completion(interrupts == crate::Interrupts::Enabled);

    channel.set_channel_configuration(channel::Configuration::enable(lpuart.destination_signal()));
    // Safety: size is appropriate for the buffer type.
    channel.set_minor_loop_bytes(core::mem::size_of::<u8>() as u32);

    // Safety: hardware address is valid.
    channel::set_destination_hardware(channel, lpuart.destination_address());

    lpuart.disable(|lpuart| {
        lpuart.disable_fifo(Direction::Tx);
    });
    lpuart.enable_destination(); // Note: this call is never undone.
}
