//! Teensy 4.0 and 4.1 board configuration.

use crate::{hal, iomuxc::imxrt1060 as iomuxc, RUN_MODE};

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_b0::GPIO_B0_03>;

/// Prepare all board resources, and return them.
pub fn new<P: Into<super::Peripherals>>(peripherals: P) -> super::Board {
    let super::Peripherals {
        gpio2,
        iomuxc,
        pit,
        gpt1,
        gpt2,
        mut ccm,
        mut ccm_analog,
        mut dcdc,
        ..
    } = peripherals.into();

    let iomuxc = super::convert_iomuxc(iomuxc);
    hal::ccm::set_low_power_mode(&mut ccm, hal::ccm::LowPowerMode::RemainInRun);
    hal::set_target_power(&mut dcdc, RUN_MODE);
    hal::ccm::clock_tree::configure(RUN_MODE, &mut ccm, &mut ccm_analog);

    let mut gpio2 = super::configure_gpio(gpio2, &mut ccm);

    let led = gpio2.output(iomuxc.gpio_b0.p03);
    let pit = super::configure_pit(pit, &mut ccm);

    let gpt1 = super::configure_gpt(gpt1, super::GPT1_DIVIDER, &mut ccm);
    let gpt2 = super::configure_gpt(gpt2, super::GPT2_DIVIDER, &mut ccm);

    super::Board {
        led,
        pit,
        gpt1,
        gpt2,
    }
}

#[cfg(target_arch = "arm")]
use teensy4_fcb as _;
