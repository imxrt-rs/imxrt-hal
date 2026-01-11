//! Demonstrates random numbers from the TRNG.
//!
//! Connect your device's default logging BACKEND to observe log messages
//! with random numbers.

#![no_std]
#![no_main]

/// How frequently (milliseconds) should we make a random number?
const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

use hal::pit::Channel;
use imxrt_hal as hal;

const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
const BACKEND: board::logging::Backend = board::logging::BACKEND;
const PIT_CHANNEL: Channel = Channel::Chan2;

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common {
            mut pit,
            usb1,
            usbnc1,
            usbphy1,
            mut dma,
            ..
        },
        board::Specifics {
            led,
            console,
            mut trng,
            ..
        },
    ) = board::new();

    pit.set_load_timer_value(PIT_CHANNEL, MAKE_LOG_INTERVAL_MS);
    pit.set_interrupt_enable(PIT_CHANNEL, false);
    pit.enable(PIT_CHANNEL);

    let usbd = hal::usbd::Instances {
        usb: usb1,
        usbnc: usbnc1,
        usbphy: usbphy1,
    };
    let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    let mut poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

    loop {
        poller.poll();
        if pit.is_elapsed(PIT_CHANNEL) {
            led.toggle();
            while pit.is_elapsed(PIT_CHANNEL) {
                pit.clear_elapsed(PIT_CHANNEL);
            }

            let random = trng.next_u32();
            log::info!("Random number: {random:?}");
        }
    }
}
