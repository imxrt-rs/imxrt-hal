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
pub(crate) unsafe fn poll() {
    let consumer = CONSUMER.assume_init_mut();
    let channel = CHANNEL.assume_init_mut();

    // Could be high if the user enabled DMA interrupts.
    while channel.is_interrupt() {
        channel.clear_interrupt();
    }

    assert!(!channel.is_error());

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

    // If we're at this point, there's no active transfer. So, if
    // there's data available, we should try to schedule that transfer.
    //
    // The goal was to only call split_read once, and handle cases where
    // transfers complete, and / or there's new data. It helps to decompose
    // into two branches, then studying the paths through these two branches.
    //
    //  if is_complete { /* Call split read and release grant based on last transfer size */}
    //  if let Ok(remaining_grant) = consumer.split_read() {
    //    /* Schedule next transfer */
    //  }
    if let Ok(grant) = consumer.split_read() {
        // We have some data. There could be new data. Or, we could be holding
        // data in the buffer and are pending a release. This routine needs to
        // handle both cases.
        let available = if complete {
            // What's remaining in the buffer is the difference between the total
            // available, and what we last transferred. We need to release this
            // data, and schedule the remaining bytes. The DMA channel is already
            // pointing at the next element in the circular queue.
            let transferred: usize = channel.beginning_transfer_iterations().into();
            let remaining = grant.combined_len() - transferred;
            grant.release(transferred);
            remaining
        } else {
            // Otherwise, transfer whatever is available. This is the path
            // taken when the caller first calls poll(), and when new data
            // appears (and no transfer completed).
            grant.combined_len()
        };

        // Could be false if a transfer completes, and if there was no new data
        // in the buffer. In this case, wait for data to appear.
        if available > 0 {
            channel.set_transfer_iterations(available.min(u16::MAX as usize) as u16);
            channel.enable();
        }
    }
}

/// This initialization ensures that the DMA channel is inactive, then does trickery
/// with bbqueue to access the "circular buffer."
///
/// # Panics
///
/// Panics if the producer cannot grant us the entire buffer. This is unlikely, based
/// on package design.
///
/// Also panics if the underlying buffer does not meet the alignment requirements. This
/// is also unlikely, because we took care in dependency selection.
pub(crate) fn try_init(
    channel: &mut channel::Channel,
    producer: &mut bbqueue::Producer<'static, { crate::BUFFER_SIZE }>,
) {
    channel.disable();
    channel.clear_complete();
    channel.clear_error();

    const _: () = assert!(
        crate::BUFFER_SIZE.is_power_of_two(),
        "BUFFER_SIZE must be power of two for correct circular DMA transfers."
    );

    let mut grant = producer
        .grant_exact(crate::BUFFER_SIZE)
        .unwrap_or_else(|_| panic!("Could not obtain grant for DMA sizing"));
    let buffer = grant.buf();

    // Safety: buffer is static. Call panics if the buffer's alignment isn't correct.
    unsafe { channel::set_source_circular_buffer(channel, buffer) };
}

/// Finish the LPUART logging initialization.
///
/// # Safety
///
/// This call must only be called once. This call must happen before `poll` is invoked.
pub(crate) unsafe fn finish_init<P, const LPUART: u8>(
    mut lpuart: Lpuart<P, LPUART>,
    channel: channel::Channel,
    consumer: crate::Consumer,
    interrupts: crate::Interrupts,
) {
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
