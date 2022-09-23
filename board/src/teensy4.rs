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

#[cfg(not(feature = "spi"))]
/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_b0::GPIO_B0_03>;
#[cfg(feature = "spi")]
/// LED output repurposed for SPI SCLK.
pub type Led = ();

/// The UART console. Baud specified in lib.rs.
pub type Console = hal::lpuart::Lpuart<ConsolePins, 2>;
pub type ConsolePins = hal::lpuart::Pins<
    iomuxc::gpio_ad_b1::GPIO_AD_B1_02, // TX, P14
    iomuxc::gpio_ad_b1::GPIO_AD_B1_03, // RX, P15
>;

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

/// Teensy 4 specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub console: Console,
    pub spi: Spi,
    pub i2c: I2c,
    pub pwm: Pwm,
}

impl Specifics {
    pub(crate) fn new() -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let _gpio2 = unsafe { ral::gpio::GPIO2::instance() };
        let mut _gpio2 = hal::gpio::Port::new(_gpio2);

        #[cfg(not(feature = "spi"))]
        let led = _gpio2.output(iomuxc.gpio_b0.p03);
        #[cfg(feature = "spi")]
        let led = ();

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
        let spi = ();

        let lpi2c3 = unsafe { ral::lpi2c::LPI2C3::instance() };
        let i2c = I2c::new(
            lpi2c3,
            I2cPins {
                scl: iomuxc.gpio_ad_b1.p07,
                sda: iomuxc.gpio_ad_b1.p06,
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

        Self {
            led,
            console,
            spi,
            i2c,
            pwm,
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

    pub const CLKO1_SELECTIONS: [Clko1; 0] = [];

    pub const CLKO2_SELECTIONS: [Clko2; 0] = [];
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART2;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI4;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM2_2;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_USB1: Interrupt = Interrupt::USB_OTG1;
    pub const BOARD_SWTASK0: Interrupt = Interrupt::KPP;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_CONSOLE, syms::BOARD_CONSOLE),
        (BOARD_SPI, syms::BOARD_SPI),
        (BOARD_PWM, syms::BOARD_PWM),
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_PIT, syms::BOARD_PIT),
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}
pub use interrupt as Interrupt;
