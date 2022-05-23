//! Demonstrates using DMA and UART with RTIC.
//!
//! The driver waits for a serial character, then sends the same
//! character back 32 times. All UART sends and receives are performed
//! with DMA.
//!
//! # Board compatibility
//!
//! Due to cortex-m-rtic #197 [1], this example is a non-traditional
//! RTIC application. The example conditionally compiles the
//! 'mod app' item in order to generalize platforms. The only thing
//! that changes is the "binds" hardware task attribute.
//!
//! [1]: https://github.com/rtic-rs/cortex-m-rtic/issues/197

#![no_std]
#![no_main]

use imxrt_hal as hal;

/// Application when the platform only supports 16 DMA channels.
#[cfg(feature = "imxrt1010evk")]
mod chan16 {
    #[rtic::app(device = imxrt_ral, peripherals = true)]
    mod app {
        #[shared]
        struct Shared {}

        #[local]
        struct Local {
            app: crate::App,
        }

        #[init(local = [buf: [u8; 32] = [0; 32]])]
        fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
            let app = crate::App::init(cx.device, cx.local.buf);
            (Shared {}, Local { app }, init::Monotonics())
        }

        #[task(binds = DMA7, local = [app])]
        fn dma_complete(cx: dma_complete::Context) {
            cx.local.app.dma_complete();
        }
    }
}

/// Application when the platform supports 32 DMA channels.
#[cfg(not(feature = "imxrt1010evk"))]
mod chan32 {
    #[rtic::app(device = imxrt_ral, peripherals = true)]
    mod app {
        #[shared]
        struct Shared {}

        #[local]
        struct Local {
            app: crate::App,
        }

        #[init(local = [buf: [u8; 32] = [0; 32]])]
        fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
            let app = crate::App::init(cx.device, cx.local.buf);
            (Shared {}, Local { app }, init::Monotonics())
        }

        #[task(binds = DMA7_DMA23, local = [app])]
        fn dma_complete(cx: dma_complete::Context) {
            cx.local.app.dma_complete();
        }
    }
}

/// What's our UART state?
enum State {
    /// Waiting for the first character.
    Receiving,
    /// Sending back the results.
    Transfering,
}

use hal::dma::{
    channel,
    peripheral::{Destination, Source},
};

/// The application resources.
pub struct App {
    led: board::Led,
    console: board::Console,
    channel: hal::dma::channel::Channel,
    buffer: &'static mut [u8],
    state: State,
}

impl App {
    /// Called by RTIC's init routine to prepare resources.
    fn init(peripherals: imxrt_ral::Peripherals, buffer: &'static mut [u8]) -> Self {
        let board::Board {
            led,
            mut console,
            mut dma,
            ..
        } = board::new(peripherals);

        let mut channel = dma[7].take().unwrap();
        channel.set_interrupt_on_completion(true);
        channel.set_disable_on_completion(true);

        unsafe {
            // Safety: buffer is static.
            dma_receive(&mut channel, &mut console, &mut buffer[..1]);
        }

        Self {
            led,
            console,
            channel,
            buffer,
            state: State::Receiving,
        }
    }

    /// Invoked by the DMA interrupt handler when an operation completes.
    fn dma_complete(&mut self) {
        while self.channel.is_interrupt() {
            self.channel.clear_interrupt();
        }

        if !self.channel.is_complete() || self.channel.is_error() {
            self.led.set();
            return;
        }
        self.channel.clear_complete();

        match self.state {
            State::Receiving => {
                // Completed receive operation.
                let recv = self.buffer[0];
                self.buffer.fill(recv);
                unsafe {
                    // Safety: buffer is static
                    dma_transfer(&mut self.channel, &mut self.console, self.buffer);
                }
                self.state = State::Transfering;
            }
            State::Transfering => {
                unsafe {
                    // Safety: buffer is static.
                    dma_receive(&mut self.channel, &mut self.console, &mut self.buffer[..1]);
                }
                self.state = State::Receiving;
                self.led.toggle();
            }
        }
    }
}

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
