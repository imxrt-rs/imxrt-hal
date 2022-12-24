//! Demonstrates random numbers from the TRNG.
//!
//! Connect to the USB serial interface to observe log messages
//! with random numbers.

#![no_std]
#![no_main]

/// How frequently (milliseconds) should we make a random number?
const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

use imxrt_hal as hal;

const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
const BACKEND: board::logging::Backend = board::logging::BACKEND;

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common {
            pit: (_, _, mut make_log, _),
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

    make_log.set_load_timer_value(MAKE_LOG_INTERVAL_MS);
    make_log.set_interrupt_enable(false);
    make_log.enable();

    let usbd = hal::usbd::Instances {
        usb: usb1,
        usbnc: usbnc1,
        usbphy: usbphy1,
    };
    let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    let mut poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

    loop {
        poller.poll();
        if make_log.is_elapsed() {
            led.toggle();
            while make_log.is_elapsed() {
                make_log.clear_elapsed();
            }

            let random = trng.next_u32();
            log::info!("Random number: {random:?}");
        }
    }
}
