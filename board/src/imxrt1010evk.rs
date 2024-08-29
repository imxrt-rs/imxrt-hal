//! IMXRT1010EVK board configuration.
//!
//! Peripheral pins and instances are documented inline.
//!
//! # `"spi"` feature
//!
//! When activated, the PWM peripheral is disabled,
//! and the SPI peripheral takes its place. When not activated,
//! the SPI peripheral is the unit type `()`.
//!
//! This board repurposes the SPI pins for PWM instead of using the
//! hardware-allocated PWM pins. Hardware-allocated PWM pins require
//! that you populate and de-populate certain resistors. Compile-time
//! configurations are faster than working with 0402 resistors.

use defmt_rtt as _;

use crate::{hal, iomuxc::imxrt1010 as iomuxc, ral};

#[cfg(target_arch = "arm")]
use imxrt1010evk_fcb as _;

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

mod imxrt10xx {
    pub mod clock;
    pub mod power;

    #[path = "clock_tree/pll6_ahb.rs"]
    mod ahb;

    mod clock_tree;
}

pub use imxrt10xx::{clock::*, power::*};

/// You'll find log messages using the serial console, through the DAP.
pub(crate) const DEFAULT_LOGGING_BACKEND: crate::logging::Backend = crate::logging::Backend::Lpuart;

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio::GPIO_11>;

/// The SW4 "user" button.
pub type Button = hal::gpio::Input<ButtonPad>;
type ButtonPad = iomuxc::gpio_sd::GPIO_SD_05;

/// The UART console. Baud specified in lib.rs.
pub type Console = hal::lpuart::Lpuart<ConsolePins, 1>;

/// The debug serial console's pins.
///
/// The UART routes to the DAP coprocessor, so the specific pins are not
/// important. To interact with the console, attach to the serial interface of
/// your board's DAP coprocssor. The coprocessor shuttles the data between your
/// host and the MCU.
pub type ConsolePins = crate::hal::lpuart::Pins<iomuxc::gpio::GPIO_10, iomuxc::gpio::GPIO_09>;

pub type SpiPins = hal::lpspi::Pins<
    iomuxc::gpio_ad::GPIO_AD_04, // SDO, J57_8
    iomuxc::gpio_ad::GPIO_AD_03, // SDI, J57_10
    iomuxc::gpio_ad::GPIO_AD_06, // SCK, J57_12
    iomuxc::gpio_ad::GPIO_AD_05, // PCS0, J57_6
>;

#[cfg(feature = "spi")]
pub type Spi = hal::lpspi::Lpspi<SpiPins, 1>;

#[cfg(not(feature = "spi"))]
pub type Spi = ();

pub type I2cPins = hal::lpi2c::Pins<
    iomuxc::gpio::GPIO_02, // SCL, J57_20
    iomuxc::gpio::GPIO_01, // SDA, J57_18
>;

pub type I2c = hal::lpi2c::Lpi2c<I2cPins, 1>;

/// PWM components.
#[cfg(not(feature = "spi"))]
pub mod pwm {
    use super::iomuxc;
    use crate::hal::flexpwm;

    /// The PWM peripheral instance.
    ///
    /// The RAL qualifies this as "PWM 0," even if the board schematic and
    /// reference manual qualify this as "PWM 1." This is due to how the RAL
    /// auto-generated register definitions in the presence of multiple instances
    /// per peripheral.
    pub type Peripheral = flexpwm::Pwm<{ crate::ral::SOLE_INSTANCE }>;
    pub type Submodule = flexpwm::Submodule<{ Peripheral::N }, 2>;
    pub type Outputs = (
        flexpwm::Output<iomuxc::gpio_ad::GPIO_AD_04>, // A, J57_8
        flexpwm::Output<iomuxc::gpio_ad::GPIO_AD_03>, // B, J57_10
    );
}

#[cfg(feature = "spi")]
pub mod pwm {
    pub type Peripheral = ();
    pub type Submodule = ();
    pub type Outputs = ();
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

/// Test point 34.
///
/// Use this for measuring your application timing (as a GPIO).
/// Or, use it to evaluate clocks via `CCM_CLKO1`.
pub type Tp34 = iomuxc::gpio_sd::GPIO_SD_02;

/// Test point 31.
///
/// Use this for measuring your application timing (as a GPIO).
/// Or, use it to evaluate clocks via `CCM_CLKO2`.
pub type Tp31 = iomuxc::gpio_sd::GPIO_SD_01;

/// Opaque structure for managing GPIO ports.
///
/// Exposes methods to configure your board's specific GPIOs.
pub struct GpioPorts {
    gpio2: hal::gpio::Port<2>,
}

impl GpioPorts {
    /// Returns the GPIO port for the button.
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port<2> {
        &mut self.gpio2
    }
}

/// IMXRT1010EVK specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub button: Button,
    pub ports: GpioPorts,
    pub console: Console,
    pub spi: Spi,
    pub i2c: I2c,
    pub pwm: Pwm,
    pub tp34: Tp34,
    pub tp31: Tp31,
    pub trng: hal::trng::Trng,
    pub tempmon: hal::tempmon::TempMon,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
        let mut gpio1 = hal::gpio::Port::new(gpio1);
        let gpio2 = unsafe { ral::gpio::GPIO2::instance() };
        let mut gpio2 = hal::gpio::Port::new(gpio2);

        let led = gpio1.output(iomuxc.gpio.p11);
        let button = gpio2.input(iomuxc.gpio_sd.p05);

        let lpuart1 = unsafe { ral::lpuart::LPUART1::instance() };
        let mut console = hal::lpuart::Lpuart::new(
            lpuart1,
            hal::lpuart::Pins {
                tx: iomuxc.gpio.p10,
                rx: iomuxc.gpio.p09,
            },
        );
        console.disable(|console| {
            console.set_baud(&super::CONSOLE_BAUD);
            console.set_parity(None);
        });

        #[cfg(feature = "spi")]
        let spi = {
            let lpspi1 = unsafe { ral::lpspi::LPSPI1::instance() };
            let pins = SpiPins {
                sdo: iomuxc.gpio_ad.p04,
                sdi: iomuxc.gpio_ad.p03,
                sck: iomuxc.gpio_ad.p06,
                pcs0: iomuxc.gpio_ad.p05,
            };
            let mut spi = Spi::new(lpspi1, pins);
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
                scl: iomuxc.gpio.p02,
                sda: iomuxc.gpio.p01,
            },
            &super::I2C_BAUD_RATE,
        );

        #[cfg(not(feature = "spi"))]
        let pwm = {
            let flexpwm = unsafe { ral::pwm::PWM::instance() };
            let (pwm, (_, _, sm, _)) = hal::flexpwm::new(flexpwm);

            let out_a = hal::flexpwm::Output::new_a(iomuxc.gpio_ad.p04);
            let out_b = hal::flexpwm::Output::new_b(iomuxc.gpio_ad.p03);

            super::Pwm {
                module: pwm,
                submodule: sm,
                outputs: (out_a, out_b),
            }
        };

        #[cfg(feature = "spi")]
        let pwm = Pwm {
            module: (),
            submodule: (),
            outputs: (),
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
            ports: GpioPorts { gpio2 },
            console,
            spi,
            i2c,
            pwm,
            tp34: iomuxc.gpio_sd.p02,
            tp31: iomuxc.gpio_sd.p01,
            trng,
            tempmon,
        }
    }
}

use hal::ccm::clock_gate;

/// The clock gates for this board's peripherals.
pub(crate) const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio::<1>(),
    clock_gate::lpuart::<{ Console::N }>(),
    #[cfg(feature = "spi")]
    clock_gate::lpspi::<{ Spi::N }>(),
    clock_gate::lpi2c::<{ I2c::N }>(),
    #[cfg(not(feature = "spi"))]
    clock_gate::flexpwm::<{ pwm::Peripheral::N }>(),
];

/// Configure board pins.
///
/// Peripherals are responsible for pin muxing, so there's no need to
/// set alternates here.
fn configure_pins(
    super::Pads {
        ref mut gpio,
        ref mut gpio_sd,
        ref mut gpio_ad,
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

    iomuxc::configure(&mut gpio.p02, I2C_PIN_CONFIG);
    iomuxc::configure(&mut gpio.p01, I2C_PIN_CONFIG);

    const BUTTON_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k))
        .set_hysteresis(iomuxc::Hysteresis::Enabled);

    let button: &mut ButtonPad = &mut gpio_sd.p05;
    iomuxc::configure(button, BUTTON_CONFIG);

    // Set the pin muxing for the two test points.
    crate::iomuxc::ccm::prepare(&mut gpio_sd.p01);
    crate::iomuxc::ccm::prepare(&mut gpio_sd.p02);

    const SPI_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_open_drain(iomuxc::OpenDrain::Disabled)
        .set_hysteresis(iomuxc::Hysteresis::Disabled)
        .set_pull_keeper(None);

    iomuxc::configure(&mut gpio_ad.p04, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_ad.p03, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_ad.p06, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_ad.p05, SPI_PIN_CONFIG);
}

/// Helpers for the clock_out example.
pub mod clock_out {
    use crate::hal::ccm::output_source::{clko1::Selection as Clko1, clko2::Selection as Clko2};

    pub const CLKO1_SELECTIONS: [Clko1; 7] = [
        Clko1::Pll3SwClkDiv2,
        Clko1::Pll2Div2,
        Clko1::EnetPllDiv2,
        Clko1::AhbClk,
        Clko1::IpgClk,
        Clko1::Perclk,
        Clko1::Pll4MainClk,
    ];

    pub const CLKO2_SELECTIONS: [Clko2; 9] = [
        Clko2::Lpi2cClk,
        Clko2::OscClk,
        Clko2::LpspiClk,
        Clko2::Sai1Clk,
        Clko2::Sai3Clk,
        Clko2::TraceClk,
        Clko2::FlexspiClk,
        Clko2::UartClk,
        Clko2::Spdif0Clk,
    ];

    pub const MAX_DIVIDER_VALUE: u32 = 8;
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART1;
    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO2_COMBINED_0_15;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI1;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM1_2;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA11;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT;
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
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}
pub use interrupt as Interrupt;
