//! Teensy 4.0 and 4.1 board configuration.
//!
//! Peripheral pins and instances are documented inline.
//!
//! # `"spi"` feature
//!
//! When activated, [`Led`] is the unit type, `()`. The
//! SPI peripheral uses pin 13 as the clock output. When
//! not activated, the SPI peripheral is the unit type `()`.

use crate::{hal, iomuxc::imxrt1060 as iomuxc, ral};

mod imxrt10xx {
    pub(crate) mod clock;
    pub(crate) mod power;

    #[path = "clock_tree/pll1_ahb.rs"]
    mod ahb;

    mod clock_tree;
}

pub use imxrt10xx::clock::*;

/// You'll find log messages using USB1, the same USB that you use for programming.
pub(crate) const DEFAULT_LOGGING_BACKEND: crate::logging::Backend = crate::logging::Backend::Usbd;

#[cfg(not(feature = "spi"))]
/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_b0::GPIO_B0_03>;
#[cfg(feature = "spi")]
/// LED output repurposed for SPI SCLK.
pub type Led = ();

/// The board's "button" on pin 0.
///
/// Connect a normally-open switch from pin 0 to GND.
pub type Button = hal::gpio::Input<ButtonPad>;
type ButtonPad = iomuxc::gpio_ad_b0::GPIO_AD_B0_03;

/// The UART console. Baud specified in lib.rs.
pub type Console = hal::lpuart::Lpuart<ConsolePins, 2>;
pub type ConsolePins = hal::lpuart::Pins<
    iomuxc::gpio_ad_b1::GPIO_AD_B1_02, // TX, P14
    iomuxc::gpio_ad_b1::GPIO_AD_B1_03, // RX, P15
>;

pub type Sai1MclkPin = iomuxc::gpio_ad_b1::GPIO_AD_B1_09;

pub type Sai1TxPins = hal::sai::Pins<
    iomuxc::gpio_ad_b1::GPIO_AD_B1_15,
    iomuxc::gpio_ad_b1::GPIO_AD_B1_14,
    iomuxc::gpio_b1::GPIO_B1_01,
>;

pub type Sai1RxPins = hal::sai::Pins<
    iomuxc::gpio_ad_b1::GPIO_AD_B1_10,
    iomuxc::gpio_ad_b1::GPIO_AD_B1_11,
    iomuxc::gpio_b1::GPIO_B1_00,
>;

pub type Sai1 = hal::sai::Sai<1, Sai1MclkPin, Sai1TxPins, Sai1RxPins>;

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
pub type Spi = hal::lpspi::Lpspi<SpiPins, 4>;

type I2cScl = iomuxc::gpio_ad_b1::GPIO_AD_B1_00; // P19
type I2cSda = iomuxc::gpio_ad_b1::GPIO_AD_B1_01; // P18
pub type I2cPins = hal::lpi2c::Pins<I2cScl, I2cSda>;

pub type I2c = hal::lpi2c::Lpi2c<I2cPins, 1>;

/// PWM components.
pub mod pwm {
    use super::iomuxc;
    use crate::hal::flexpwm;

    pub type Peripheral = flexpwm::Pwm<2>;
    pub type Submodule = flexpwm::Submodule<{ Peripheral::N }, 2>;
    pub type Outputs = (
        flexpwm::Output<iomuxc::gpio_b0::GPIO_B0_10>, // A, P6
        flexpwm::Output<iomuxc::gpio_b0::GPIO_B0_11>, // B, P9
    );
}

/// The board's PWM components.
pub struct Pwm {
    /// Core PWM peripheral.
    pub module: pwm::Peripheral,
    /// PWM submodule control registers.
    pub submodule: pwm::Submodule,
    /// The output pairs (tuple of A, B outputs).
    pub outputs: pwm::Outputs,
}

/// Opaque structure for managing GPIO ports.
///
/// Exposes methods to configure your board's GPIOs.
pub struct GpioPorts {
    gpio1: hal::gpio::Port<1>,
}

impl GpioPorts {
    /// Returns the GPIO port for the button.
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port<1> {
        &mut self.gpio1
    }
}

/// Teensy 4 specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub button: Button,
    pub ports: GpioPorts,
    pub console: Console,
    pub spi: Spi,
    pub i2c: I2c,
    pub sai1: Sai1,
    pub pwm: Pwm,
    pub trng: hal::trng::Trng,
    pub tempmon: hal::tempmon::TempMon,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let iomuxc_gpr = unsafe { ral::iomuxc_gpr::IOMUXC_GPR::instance() };
        ral::modify_reg!(ral::iomuxc_gpr, iomuxc_gpr, GPR1, SAI1_MCLK_DIR: 1);

        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
        let mut gpio1 = hal::gpio::Port::new(gpio1);

        #[cfg(not(feature = "spi"))]
        let led = {
            let gpio2 = unsafe { ral::gpio::GPIO2::instance() };
            let mut gpio2 = hal::gpio::Port::new(gpio2);
            gpio2.output(iomuxc.gpio_b0.p03)
        };
        #[cfg(feature = "spi")]
        let led = ();

        let button = gpio1.input(iomuxc.gpio_ad_b0.p03);

        let lpuart2 = unsafe { ral::lpuart::LPUART2::instance() };
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

        let sai1 = {
            let sai1 = unsafe { ral::sai::SAI1::instance() };
            let rxpins = Sai1RxPins {
                sync: iomuxc.gpio_ad_b1.p10,
                bclk: iomuxc.gpio_ad_b1.p11,
                data: iomuxc.gpio_b1.p00,
            };

            let txpins = Sai1TxPins {
                sync: iomuxc.gpio_ad_b1.p15,
                bclk: iomuxc.gpio_ad_b1.p14,
                data: iomuxc.gpio_b1.p01,
            };
            Sai1::new(sai1, iomuxc.gpio_ad_b1.p09, txpins, rxpins)
        };

        #[cfg(feature = "spi")]
        let spi = {
            let lpspi4 = unsafe { ral::lpspi::LPSPI4::instance() };
            let pins = SpiPins {
                sdo: iomuxc.gpio_b0.p02,
                sdi: iomuxc.gpio_b0.p01,
                sck: iomuxc.gpio_b0.p03,
                pcs0: iomuxc.gpio_b0.p00,
            };
            let mut spi = Spi::new(lpspi4, pins);
            spi.disabled(|spi| {
                spi.set_clock_hz(super::LPSPI_CLK_FREQUENCY, super::SPI_BAUD_RATE_FREQUENCY);
            });
            spi
        };
        #[cfg(not(feature = "spi"))]
        #[allow(clippy::let_unit_value)]
        let spi = ();

        let lpi2c1 = unsafe { ral::lpi2c::LPI2C1::instance() };
        let i2c = I2c::new(
            lpi2c1,
            I2cPins {
                scl: iomuxc.gpio_ad_b1.p00,
                sda: iomuxc.gpio_ad_b1.p01,
            },
            &super::I2C_BAUD_RATE,
        );

        let flexpwm2 = unsafe { ral::pwm::PWM2::instance() };
        let pwm = {
            let (pwm, (_, _, sm, _)) = hal::flexpwm::new(flexpwm2);

            let out_a = hal::flexpwm::Output::new_a(iomuxc.gpio_b0.p10);
            let out_b = hal::flexpwm::Output::new_b(iomuxc.gpio_b0.p11);

            Pwm {
                module: pwm,
                submodule: sm,
                outputs: (out_a, out_b),
            }
        };
        let trng = hal::trng::Trng::new(
            unsafe { ral::trng::TRNG::instance() },
            Default::default(),
            Default::default(),
        );
        let tempmon = hal::tempmon::TempMon::with_measure_freq(
            unsafe { ral::tempmon::TEMPMON::instance() },
            0x1000,
        );
        Self {
            led,
            button,
            ports: GpioPorts { gpio1 },
            console,
            spi,
            i2c,
            sai1,
            pwm,
            trng,
            tempmon,
        }
    }
}

use hal::ccm::clock_gate;

/// The clock gates for this board's peripherals.
pub(crate) const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio::<2>(),
    clock_gate::lpuart::<{ Console::N }>(),
    #[cfg(feature = "spi")]
    clock_gate::lpspi::<{ Spi::N }>(),
    clock_gate::lpi2c::<{ I2c::N }>(),
    clock_gate::flexpwm::<{ pwm::Peripheral::N }>(),
];

/// Configure board pins.
///
/// Peripherals are responsible for pin muxing, so there's no need to
/// set alternates here.
fn configure_pins(
    super::Pads {
        ref mut gpio_ad_b0,
        ref mut gpio_ad_b1,
        ref mut gpio_b0,
        ..
    }: &mut super::Pads,
) {
    use crate::iomuxc;
    const I2C_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_open_drain(iomuxc::OpenDrain::Enabled)
        .set_slew_rate(iomuxc::SlewRate::Fast)
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_speed(iomuxc::Speed::Fast)
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup22k));

    let scl: &mut I2cScl = &mut gpio_ad_b1.p00;
    iomuxc::configure(scl, I2C_PIN_CONFIG);
    let sda: &mut I2cSda = &mut gpio_ad_b1.p01;
    iomuxc::configure(sda, I2C_PIN_CONFIG);

    const BUTTON_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k))
        .set_hysteresis(iomuxc::Hysteresis::Enabled);

    let button: &mut ButtonPad = &mut gpio_ad_b0.p03;
    iomuxc::configure(button, BUTTON_CONFIG);

    const SPI_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_open_drain(iomuxc::OpenDrain::Disabled)
        .set_hysteresis(iomuxc::Hysteresis::Disabled)
        .set_pull_keeper(None);

    iomuxc::configure(&mut gpio_b0.p02, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_b0.p01, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_b0.p03, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_b0.p00, SPI_PIN_CONFIG);
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

    pub const CLKO1_SELECTIONS: [Clko1; 0] = [];

    pub const CLKO2_SELECTIONS: [Clko2; 0] = [];

    pub const MAX_DIVIDER_VALUE: u32 = 8;
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART2;
    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO2_COMBINED_16_31;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI4;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM2_2;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA11_DMA27;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT;
    pub const BOARD_SAI1: Interrupt = Interrupt::SAI1;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_USB1: Interrupt = Interrupt::USB_OTG1;
    pub const BOARD_SWTASK0: Interrupt = Interrupt::KPP;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_CONSOLE, syms::BOARD_CONSOLE),
        (BOARD_BUTTON, syms::BOARD_BUTTON),
        (BOARD_SPI, syms::BOARD_SPI),
        (BOARD_PWM, syms::BOARD_PWM),
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_DMA_B, syms::BOARD_DMA_B),
        (BOARD_PIT, syms::BOARD_PIT),
        (BOARD_SAI1, syms::BOARD_SAI1),
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}
pub use interrupt as Interrupt;
