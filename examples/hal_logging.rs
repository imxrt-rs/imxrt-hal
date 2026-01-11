//! Demonstrates imxrt-log text and defmt logging.
//!
//! Modify the `FRONTEND` and `BACKEND` configurations to combine log / defmt
//! with a LPUART or USB CDC logger. When LPUART is selected, the log messages
//! are written over the board's console; see your board's `Console` documentation
//! for information on the settings. When a USB device is selected, the log messages
//! are written over a USB serial interface.
//!
//! See the `imxrt-log` documentation for more information on each logging backend.
//!
//! This example does not use interrupts. For an example that demonstrates interrupt-driven
//! loggers, see the rtic_logging example.

#![no_std]
#![no_main]

//
// Configure the demo below.
//

/// Change me to change how log messages are serialized
/// and transported.
const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
/// This is a function of your board. Want to change it? Change it right
/// here to explore different example code paths.
const BACKEND: board::logging::Backend = board::logging::BACKEND;

/// How frequently (milliseconds) should we make a log message?
///
/// Decrease this constant to log more frequently.
const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

//
// End configurations.
//

use hal::pit::Channel;
use imxrt_hal as hal;

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
        board::Specifics { led, console, .. },
    ) = board::new();

    // When should we generate a log message?
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

    let mut counter = 0;
    loop {
        poller.poll();
        if pit.is_elapsed(PIT_CHANNEL) {
            led.toggle();
            while pit.is_elapsed(PIT_CHANNEL) {
                pit.clear_elapsed(PIT_CHANNEL);
            }
            log::info!("Hello from the log framework over {BACKEND:?}! The count is {counter}");
            defmt::println!(
                "Hello from defmt over {}! The count is {=u32}",
                BACKEND,
                counter
            );
            counter += 1;
        }
    }
}
