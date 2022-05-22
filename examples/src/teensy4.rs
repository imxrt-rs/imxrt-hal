//! Teensy 4.0 and 4.1 board configuration.

use crate::{hal, iomuxc::imxrt1060 as iomuxc, ral};

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_b0::GPIO_B0_03>;

/// Prepare all board resources, and return them.
///
/// # Panics
///
/// Panics if a board resource is already taken.
pub fn prepare() -> super::Board {
    let iomuxc = super::take_iomuxc();

    let mut gpio2 = ral::gpio::GPIO2::take()
        .map(hal::gpio::Port::new)
        .expect("Unable to take GPIO2 peripheral");

    let led = gpio2.output(iomuxc.gpio_b0.p03);

    super::Board { led }
}

#[cfg(target_arch = "arm")]
use teensy4_fcb as _;
