//! Demonstrates the secure real-time counter.
//!
//! This example tries to set the SRTC counter value. If the SRTC
//! is already counting (from a previous execution, for instance),
//! then `was_enabled` is `true`, and the SRTC continues counting from
//! its current value. Otherwise, `was_enabled` is `false`, and the
//! SRTC starts counting from a new value.
//!
//! This example requires that your board supports logging. It uses the
//! board's recommended logger by default.
//!
//! The `was_enabled` state retention depends on how you program your board,
//! and how your board is powered. This specifically includes power to your
//! chip's low-power domains.

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

//
// End configurations.
//

use hal::snvs::srtc::EnabledState;
use imxrt_hal as hal;

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common {
            usb1,
            usbnc1,
            usbphy1,
            mut dma,

            srtc,
            mut snvs_lp_core,
            ..
        },
        board::Specifics { led, console, .. },
    ) = board::new();

    let usbd = hal::usbd::Instances {
        usb: usb1,
        usbnc: usbnc1,
        usbphy: usbphy1,
    };

    let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    let mut poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

    let (srtc, was_enabled) = match srtc.try_enable(&mut snvs_lp_core, 1600000000, 0) {
        EnabledState::AlreadyCounting { srtc, .. } => (srtc, true),
        EnabledState::SetTime(srtc) => (srtc, false),
    };

    let mut then = 0;
    loop {
        poller.poll();
        let now = srtc.get();
        if now != then {
            then = now;
            led.toggle();
            log::info!("SRTC time: {now}. Was enabled? {was_enabled}");
            defmt::println!("SRTC time: {=u32}. Was enable? {=bool}", now, was_enabled);
        }
    }
}
