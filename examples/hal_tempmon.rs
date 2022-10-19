//! Demonstrates the temperature monitor on 10xx MCUs.
//!
//! This example uses the logging system to relay temperature
//! measurements.

#![no_std]
#![no_main]

//
// Configure the demo below.
//

/// Change me to change how log messages are serialized
/// and transported.
const FRONTEND: Frontend = Frontend::Log;
/// Change me to select the peripheral used for logging.
const BACKEND: Backend = Backend::Lpuart;

#[derive(Debug, defmt::Format)]
pub enum Frontend {
    Log,
    Defmt,
}

#[derive(Debug, defmt::Format)]
pub enum Backend {
    Usbd,
    Lpuart,
}

/// How frequently (milliseconds) should we make a log message?
///
/// Decrease this constant to log more frequently.
const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

//
// End configurations.
//

use imxrt_hal as hal;

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
            mut tempmon,
            ..
        },
    ) = board::new();

    // When should we generate a log message?
    make_log.set_load_timer_value(MAKE_LOG_INTERVAL_MS);
    make_log.set_interrupt_enable(false);
    make_log.enable();

    let usb_instances = hal::usbd::Instances {
        usb: usb1,
        usbnc: usbnc1,
        usbphy: usbphy1,
    };

    let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    let mut poller = match (FRONTEND, BACKEND) {
        // Logging frontends...
        (Frontend::Log, Backend::Lpuart) => {
            imxrt_log::log::lpuart(console, dma_a, imxrt_log::Interrupts::Disabled).unwrap()
        }
        (Frontend::Log, Backend::Usbd) => {
            imxrt_log::log::usbd(usb_instances, imxrt_log::Interrupts::Disabled).unwrap()
        }
        // Defmt frontends...
        (Frontend::Defmt, Backend::Lpuart) => {
            imxrt_log::defmt::lpuart(console, dma_a, imxrt_log::Interrupts::Disabled).unwrap()
        }
        (Frontend::Defmt, Backend::Usbd) => {
            imxrt_log::defmt::usbd(usb_instances, imxrt_log::Interrupts::Disabled).unwrap()
        }
    };

    tempmon.start().ok();
    loop {
        poller.poll();
        if make_log.is_elapsed() {
            led.toggle();
            while make_log.is_elapsed() {
                make_log.clear_elapsed();
            }

            if let Ok(temperature) = tempmon.get_temp() {
                log::info!("Temperature (mC'): {temperature}");
                defmt::println!("Temperature (mC'): {=i32}", temperature);
            }
        }
    }
}
