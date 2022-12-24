//! Demonstrates using DMA and UART with RTIC.
//!
//! The driver waits for a serial character, then sends the same
//! character back 32 times. All UART sends and receives are performed
//! with DMA.

#![no_std]
#![no_main]

use imxrt_hal as hal;

#[rtic::app(device = board, peripherals = false)]
mod app {
    use super::{dma_receive, dma_transfer, hal};

    /// What's our UART state?
    pub enum State {
        /// Waiting for the first character.
        Receiving,
        /// Sending back the results.
        Transfering,
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        console: board::Console,
        channel: hal::dma::channel::Channel,
        buffer: &'static mut [u8],
        state: State,
    }

    #[init(local = [buf: [u8; 32] = [0; 32]])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (
            board::Common { mut dma, .. },
            board::Specifics {
                led, mut console, ..
            },
        ) = board::new();
        let mut channel = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        channel.set_interrupt_on_completion(true);
        channel.set_disable_on_completion(true);

        unsafe {
            // Safety: buffer is static.
            dma_receive(&mut channel, &mut console, &mut cx.local.buf[..1]);
        }
        (
            Shared {},
            Local {
                led,
                console,
                channel,
                buffer: cx.local.buf,
                state: State::Receiving,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = BOARD_DMA_A, local = [led, console, channel, buffer, state])]
    fn dma_complete(cx: dma_complete::Context) {
        let dma_complete::LocalResources {
            channel,
            led,
            state,
            buffer,
            console,
        } = cx.local;

        while channel.is_interrupt() {
            channel.clear_interrupt();
        }

        if !channel.is_complete() || channel.is_error() {
            led.set();
            return;
        }
        channel.clear_complete();

        match state {
            State::Receiving => {
                // Completed receive operation.
                let recv = buffer[0];
                buffer.fill(recv);
                unsafe {
                    // Safety: buffer is static
                    dma_transfer(channel, console, buffer);
                }
                *state = State::Transfering;
            }
            State::Transfering => {
                unsafe {
                    // Safety: buffer is static.
                    dma_receive(channel, console, &mut buffer[..1]);
                }
                *state = State::Receiving;
                led.toggle();
            }
        }
    }
}

use hal::dma::{
    channel,
    peripheral::{Destination, Source},
};

/// Prepares and activates the DMA receive operation.
///
/// When this call completes, the DMA peripheral
/// is waiting for UART signals to indicate new data.
///
/// # Safety
///
/// `buffer` must have static lifetime and cannot be observed
/// while the transfer is active.
#[allow(unused_unsafe)]
unsafe fn dma_receive(
    channel: &mut channel::Channel,
    source: &mut board::Console,
    buffer: &mut [u8],
) {
    channel.disable();
    channel.clear_complete();
    channel.clear_error();
    channel.set_channel_configuration(channel::Configuration::enable(source.source_signal()));
    unsafe {
        // Safety: hardware address is valid.
        channel::set_source_hardware(channel, source.source_address());
        // Safety: buffer is static, so always valid.
        channel::set_destination_linear_buffer(channel, buffer);
        // Safety: combination of minor loop and transfer iterations prevent buffer overrun.
        channel.set_minor_loop_bytes(core::mem::size_of::<u8>() as u32);
        channel.set_transfer_iterations(buffer.len() as u16);
    }
    source.enable_source();
    unsafe {
        // Safety: channel is ready to go, and there's no stale configuration.
        channel.enable();
    }
}

/// Prepares and activates the DMA transfer operation.
///
/// When this call completes, the DMA peripheral is writing data from the buffer to
/// the UART peripheral.
///
/// # Safety
///
/// `buffer` must have static lifetime.
#[allow(unused_unsafe)]
unsafe fn dma_transfer(
    channel: &mut channel::Channel,
    destination: &mut board::Console,
    buffer: &[u8],
) {
    channel.disable();
    channel.clear_complete();
    channel.clear_error();
    channel.set_channel_configuration(channel::Configuration::enable(
        destination.destination_signal(),
    ));
    unsafe {
        // Safety: hardware address is valid.
        channel::set_destination_hardware(channel, destination.destination_address());
        // Safety: buffer is static, so always valid.
        channel::set_source_linear_buffer(channel, buffer);
        // Safety: combination of minor loop and transfer iterations prevent buffer overrun.
        channel.set_minor_loop_bytes(core::mem::size_of::<u8>() as u32);
        channel.set_transfer_iterations(buffer.len() as u16);
    }
    destination.enable_destination();
    unsafe {
        // Safety: channel is ready to go, and there's no stale configuration.
        channel.enable();
    }
}
