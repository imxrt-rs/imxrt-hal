//! Teensy 4.0 and 4.1 board configuration.
//!
//! # `"spi"` feature
//!
//! When activated, [`Led`] is the unit type, `()`. The
//! SPI peripheral uses pin 13 as the clock output. When
//! not activated, the SPI peripheral is the unit type `()`.

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
/// Activate the `"spi"` feature to configure the SPI peripheral.
pub type Spi = ();

#[cfg(feature = "spi")]
/// SPI peripheral.
pub type Spi = hal::lpspi::LpspiMaster<SpiPins, 4>;

pub type I2cPins = hal::lpi2c::Pins<
    iomuxc::gpio_ad_b1::GPIO_AD_B1_07, // SCL, P16
    iomuxc::gpio_ad_b1::GPIO_AD_B1_06, // SDA, P17
>;

pub type I2c = hal::lpi2c::Lpi2cMaster<I2cPins, 3>;

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
        lpi2c3,
        mut ccm,
        mut ccm_analog,
        mut dcdc,
        ..
    } = peripherals.into();

    let mut iomuxc = super::convert_iomuxc(iomuxc);
    hal::ccm::set_low_power_mode(&mut ccm, hal::ccm::LowPowerMode::RemainInRun);
    hal::set_target_power(&mut dcdc, RUN_MODE);
    super::prepare_clock_tree(&mut ccm, &mut ccm_analog);

    CLOCK_GATES
        .into_iter()
        .for_each(|locator| locator.set(&mut ccm, clock_gate::ON));
    configure_pins(&mut iomuxc);

    let mut _gpio2 = hal::gpio::Port::new(_gpio2);

    #[cfg(not(feature = "spi"))]
    let led = _gpio2.output(iomuxc.gpio_b0.p03);
    #[cfg(feature = "spi")]
    let led = ();

    let pit = super::configure_pit(pit);

    let gpt1 = super::configure_gpt(gpt1, super::GPT1_DIVIDER);
    let gpt2 = super::configure_gpt(gpt2, super::GPT2_DIVIDER);

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

    let i2c = I2c::new(
        lpi2c3,
        I2cPins {
            scl: iomuxc.gpio_ad_b1.p07,
            sda: iomuxc.gpio_ad_b1.p06,
        },
        &super::I2C_BAUD_RATE,
    );

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
        i2c,
        ccm,
        specifics,
    }
}

use hal::ccm::clock_gate;

/// The clock gates for this board's peripherals.
const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::pit(),
    clock_gate::gpt_bus::<1>(),
    clock_gate::gpt_bus::<2>(),
    clock_gate::gpt_serial::<1>(),
    clock_gate::gpt_serial::<2>(),
    clock_gate::gpio::<2>(),
    clock_gate::lpuart::<{ Console::N }>(),
    clock_gate::dma(),
    #[cfg(feature = "spi")]
    clock_gate::lpspi::<{ Spi::N }>(),
    clock_gate::lpi2c::<{ I2c::N }>(),
];

/// Configure board pins.
///
/// Peripherals are responsible for pin muxing, so there's no need to
/// set alternates here.
fn configure_pins(
    super::Pads {
        ref mut gpio_ad_b1, ..
    }: &mut super::Pads,
) {
    use crate::iomuxc;
    const I2C_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_open_drain(iomuxc::OpenDrain::Enabled)
        .set_slew_rate(iomuxc::SlewRate::Fast)
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_speed(iomuxc::Speed::Fast)
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup22k));

    iomuxc::configure(&mut gpio_ad_b1.p07, I2C_PIN_CONFIG);
    iomuxc::configure(&mut gpio_ad_b1.p06, I2C_PIN_CONFIG);
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
