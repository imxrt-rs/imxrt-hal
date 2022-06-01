//! Teensy 4.0 and 4.1 board configuration.
//!
//! # `"board-spi"` feature
//!
//! When activated, [`Led`] is the unit type, `()`. The
//! SPI peripheral uses pin 13 as the clock output.

use crate::{hal, iomuxc::imxrt1060 as iomuxc, RUN_MODE};

#[cfg(not(feature = "spi"))]
/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_b0::GPIO_B0_03>;
#[cfg(feature = "spi")]
/// LED output repurposed for SPI SCLK.
pub type Led = ();

pub type Console = hal::lpuart::Lpuart<ConsolePins, 2>;
pub type ConsolePins =
    hal::lpuart::Pins<iomuxc::gpio_ad_b1::GPIO_AD_B1_02, iomuxc::gpio_ad_b1::GPIO_AD_B1_03>;

pub type SpiPins = hal::lpspi::Pins<
    iomuxc::gpio_b0::GPIO_B0_02, // SDO, P11
    iomuxc::gpio_b0::GPIO_B0_01, // SDI, P12
    iomuxc::gpio_b0::GPIO_B0_03, // SCK, P13
    iomuxc::gpio_b0::GPIO_B0_00, // PCS0, P10
>;

#[cfg(not(feature = "spi"))]
/// Activate the `"board-spi"` feature to configure the SPI peripheral.
pub type Spi = ();

#[cfg(feature = "spi")]
/// SPI peripheral.
pub type Spi = hal::lpspi::LpspiMaster<SpiPins, 4>;

/// Teensy 4 specific peripherals.
pub struct Specifics {}

/// Prepare all board resources, and return them.
pub fn new<P: Into<super::Instances>>(peripherals: P) -> super::Board {
    let super::Instances {
        gpio2: _gpio2,
        iomuxc,
        pit,
        gpt1,
        gpt2,
        lpuart2,
        dma,
        dma_mux,
        lpspi4: _lpspi4,
        mut ccm,
        mut ccm_analog,
        mut dcdc,
        ..
    } = peripherals.into();

    let iomuxc = super::convert_iomuxc(iomuxc);
    hal::ccm::set_low_power_mode(&mut ccm, hal::ccm::LowPowerMode::RemainInRun);
    hal::set_target_power(&mut dcdc, RUN_MODE);
    hal::ccm::clock_tree::configure(RUN_MODE, &mut ccm, &mut ccm_analog);

    let mut _gpio2 = super::configure_gpio(_gpio2, &mut ccm);

    #[cfg(not(feature = "spi"))]
    let led = _gpio2.output(iomuxc.gpio_b0.p03);
    #[cfg(feature = "spi")]
    let led = ();

    let pit = super::configure_pit(pit, &mut ccm);

    let gpt1 = super::configure_gpt(gpt1, super::GPT1_DIVIDER, &mut ccm);
    let gpt2 = super::configure_gpt(gpt2, super::GPT2_DIVIDER, &mut ccm);

    hal::ccm::clock_gate::lpuart::<{ Console::N }>().set(&mut ccm, hal::ccm::clock_gate::ON);
    let mut console = hal::lpuart::Lpuart::new(
        lpuart2,
        hal::lpuart::Pins {
            tx: iomuxc.gpio_ad_b1.p02,
            rx: iomuxc.gpio_ad_b1.p03,
        },
    );
    console.disable(|console| {
        console.set_baud(&super::CONSOLE_BAUD);
        console.set_parity(None);
    });

    #[cfg(feature = "spi")]
    let spi = {
        hal::ccm::clock_gate::lpspi::<{ Spi::N }>().set(&mut ccm, hal::ccm::clock_gate::ON);
        let pins = SpiPins {
            sdo: iomuxc.gpio_b0.p02,
            sdi: iomuxc.gpio_b0.p01,
            sck: iomuxc.gpio_b0.p03,
            pcs0: iomuxc.gpio_b0.p00,
        };
        let mut spi = Spi::new(_lpspi4, pins);
        spi.disabled(|spi| {
            spi.set_clock_hz(super::LPSPI_CLK_FREQUENCY, super::SPI_BAUD_RATE_FREQUENCY);
        });
        spi
    };
    #[cfg(not(feature = "spi"))]
    let spi = ();

    hal::ccm::clock_gate::dma().set(&mut ccm, hal::ccm::clock_gate::ON);
    let dma = hal::dma::channels(dma, dma_mux);
    let specifics = Specifics {};
    super::Board {
        led,
        pit,
        gpt1,
        gpt2,
        console,
        dma,
        spi,
        ccm,
        specifics,
    }
}

#[cfg(target_arch = "arm")]
use teensy4_fcb as _;
#[cfg(target_arch = "arm")]
use teensy4_panic as _;

/// Helpers for the clock_out example.
///
/// The Teensy 4 does not have these outputs, so the configuration enables
/// no functionality.
pub mod clock_out {
    use crate::hal::ccm::output_source::{clko1::Selection as Clko1, clko2::Selection as Clko2};

    pub fn prepare_outputs(_: &mut super::Specifics) {}

    pub const CLKO1_SELECTIONS: [Clko1; 0] = [];

    pub const CLKO2_SELECTIONS: [Clko2; 0] = [];
}
