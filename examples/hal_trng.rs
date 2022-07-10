//! Demonstrates random numbers from the TRNG.
//!
//! Connect to the USB serial interface to observe log messages
//! with random numbers.

#![no_std]
#![no_main]

/// How frequently (milliseconds) should we make a random number?
const MAKE_LOG_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

use imxrt_hal as hal;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Board {
        led,
        pit: (_, _, mut make_log, _),
        usb1,
        usbnc1,
        usbphy1,
        mut usb_analog,
        mut trng,
        ..
    } = board::prepare();

    make_log.set_load_timer_value(MAKE_LOG_INTERVAL_MS);
    make_log.set_interrupt_enable(false);
    make_log.enable();

    let usb_instances = hal::usbd::Instances {
        usb: usb1,
        usbnc: usbnc1,
        usbphy: usbphy1,
        usb_analog: &mut usb_analog,
    };
    let mut poller = imxrt_log::log::usbd(usb_instances, imxrt_log::Interrupts::Disabled).unwrap();

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
