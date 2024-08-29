//! IMXRT1060EVK (revision A3) board configuration.
//!
//! Peripheral pins and instances are documented inline.
//! Note that all pinouts correspond to board revision A3.
//! Identifiers *may not* match for board revision B1. For
//! instance, header J24 on A3 is header J17 on B1. (Thankfully,
//! when comparing J24 and J17 across revisions, the pinout is
//! exactly the same. I'm not sure if this is generally
//! true.)

use defmt_rtt as _;

use crate::{hal, iomuxc::imxrt1060 as iomuxc, ral};

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

#[cfg(target_arch = "arm")]
use imxrt1060evk_fcb as _;

mod imxrt10xx {
    pub(crate) mod clock;
    pub(crate) mod power;

    #[path = "clock_tree/pll1_ahb.rs"]
    mod ahb;

    mod clock_tree;
}

pub use imxrt10xx::clock::*;

/// You'll find log messages using the serial console, through the DAP.
pub(crate) const DEFAULT_LOGGING_BACKEND: crate::logging::Backend = crate::logging::Backend::Lpuart;

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio_ad_b0::GPIO_AD_B0_09>;

/// The board's "user button". Could also be used as a wake up source.
pub type Button = hal::gpio::Input<()>;

/// The UART console. Baud specified in lib.rs.
pub type Console = hal::lpuart::Lpuart<ConsolePins, 1>;

/// The debug serial console's pins.
///
/// The UART routes to the DAP coprocessor, so the specific pins are not
/// important. To interact with the console, attach to the serial interface of
/// your board's DAP coprocssor. The coprocessor shuttles the data between your
/// host and the MCU.
pub type ConsolePins = hal::lpuart::Pins<
    iomuxc::gpio_ad_b0::GPIO_AD_B0_12, // TX
    iomuxc::gpio_ad_b0::GPIO_AD_B0_13, // RX
>;

/// The SPI bus is available as part of the arduino header J24
pub type SpiPins = hal::lpspi::Pins<
    iomuxc::gpio_sd_b0::GPIO_SD_B0_02, // SDO, J24_4
    iomuxc::gpio_sd_b0::GPIO_SD_B0_03, // SDI, J24_5
    iomuxc::gpio_sd_b0::GPIO_SD_B0_00, // SCK, J24_6
    iomuxc::gpio_sd_b0::GPIO_SD_B0_01, // PCS0, J24_3
>;

#[cfg(not(feature = "spi"))]
/// Activate the `"spi"` feature to configure the SPI peripheral.
pub type Spi = ();

#[cfg(feature = "spi")]
/// SPI peripheral.
pub type Spi = hal::lpspi::Lpspi<SpiPins, 1>;

type I2cScl = iomuxc::gpio_ad_b1::GPIO_AD_B1_00; // J24_10
type I2cSda = iomuxc::gpio_ad_b1::GPIO_AD_B1_01; // J24_9
pub type I2cPins = hal::lpi2c::Pins<I2cScl, I2cSda>;

/// I2C peripheral.
///
/// Routed to numerous places on this board:
///
/// - Arduino header J24.
/// - Wolfson codec (WM8960).
/// - LCD touch interface.
/// - FXOS8700 (typically DNP).
/// - CSI camera interface.
pub type I2c = hal::lpi2c::Lpi2c<I2cPins, 1>;

/// PWM components.
pub mod pwm {
    use super::iomuxc;
    use crate::hal::flexpwm;

    pub type Peripheral = flexpwm::Pwm<1>;
    pub type Submodule = flexpwm::Submodule<{ Peripheral::N }, 3>;
    pub type Outputs = (
        flexpwm::Output<iomuxc::gpio_ad_b0::GPIO_AD_B0_10>, // A, J22_6
        flexpwm::Output<iomuxc::gpio_ad_b0::GPIO_AD_B0_11>, // B, J22_3
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
    gpio5: hal::gpio::Port<5>,
}

impl GpioPorts {
    /// Returns the GPIO port for the button.
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port<5> {
        &mut self.gpio5
    }
}

/// IMXRT1060EVK specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub button: Button,
    pub ports: GpioPorts,
    pub console: Console,
    pub spi: Spi,
    pub i2c: I2c,
    pub pwm: Pwm,
    pub trng: hal::trng::Trng,
    pub tempmon: hal::tempmon::TempMon,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        // Manually configuring IOMUXC_SNVS pads, since there's no
        // equivalent API in imxrt-iomuxc.
        let iomuxc_snvs = unsafe { ral::iomuxc_snvs::IOMUXC_SNVS::instance() };
        // ALT5 => GPIO5[0]
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_MUX_CTL_PAD_WAKEUP, MUX_MODE: 5);
        // Pull up the pin to be brought to GND on switch press. No need for a high drive.
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_PAD_CTL_PAD_WAKEUP, PUS: 1, PUE: 1, DSE: 0);

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
        let mut gpio1 = hal::gpio::Port::new(gpio1);
        let led = gpio1.output(iomuxc.gpio_ad_b0.p09);

        let gpio5 = unsafe { ral::gpio::GPIO5::instance() };
        let mut gpio5 = hal::gpio::Port::new(gpio5);
        let button = hal::gpio::Input::without_pin(&mut gpio5, 0);

        let lpuart1 = unsafe { ral::lpuart::LPUART1::instance() };
        let mut console = hal::lpuart::Lpuart::new(
            lpuart1,
            hal::lpuart::Pins {
                tx: iomuxc.gpio_ad_b0.p12,
                rx: iomuxc.gpio_ad_b0.p13,
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
                sdo: iomuxc.gpio_sd_b0.p02,
                sdi: iomuxc.gpio_sd_b0.p03,
                sck: iomuxc.gpio_sd_b0.p00,
                pcs0: iomuxc.gpio_sd_b0.p01,
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
                scl: iomuxc.gpio_ad_b1.p00,
                sda: iomuxc.gpio_ad_b1.p01,
            },
            &super::I2C_BAUD_RATE,
        );

        let flexpwm1 = unsafe { ral::pwm::PWM1::instance() };
        let pwm = {
            let (pwm, (_, _, _, sm)) = hal::flexpwm::new(flexpwm1);

            let out_a = hal::flexpwm::Output::new_a(iomuxc.gpio_ad_b0.p10);
            let out_b = hal::flexpwm::Output::new_b(iomuxc.gpio_ad_b0.p11);

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
            ports: GpioPorts { gpio5 },
            console,
            spi,
            i2c,
            pwm,
            trng,
            tempmon,
        }
    }
}

use hal::ccm::clock_gate;

/// The clock gates for this board's peripherals.
pub(crate) const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio::<1>(),
    clock_gate::gpio::<5>(),
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
        ref mut gpio_ad_b1,
        ref mut gpio_sd_b0,
        ..
    }: &mut super::Pads,
) {
    use crate::iomuxc;
    const I2C_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_open_drain(iomuxc::OpenDrain::Enabled)
        .set_slew_rate(iomuxc::SlewRate::Slow)
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_speed(iomuxc::Speed::Low)
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup22k));

    let i2c_scl: &mut I2cScl = &mut gpio_ad_b1.p00;
    let i2c_sda: &mut I2cSda = &mut gpio_ad_b1.p01;
    iomuxc::configure(i2c_scl, I2C_PIN_CONFIG);
    iomuxc::configure(i2c_sda, I2C_PIN_CONFIG);

    const SPI_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_open_drain(iomuxc::OpenDrain::Disabled)
        .set_hysteresis(iomuxc::Hysteresis::Disabled)
        .set_pull_keeper(None);

    iomuxc::configure(&mut gpio_sd_b0.p02, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_sd_b0.p03, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_sd_b0.p00, SPI_PIN_CONFIG);
    iomuxc::configure(&mut gpio_sd_b0.p01, SPI_PIN_CONFIG);
}

/// Helpers for the clock_out example.
///
/// TODO add the CLKO1 and CLKO2 selections for the 1060. See
/// the corresponding TODO in the imxrt1060 chip configuration
/// modules in the HAL.
///
/// TODO add CCM pin support for 1060 pads in imxrt-iomuxc.
/// Then, configure those pins as CCM outputs. See the 1010EVK
/// and 1170EVK board modules for an example.
pub mod clock_out {
    use crate::hal::ccm::output_source::{clko1::Selection as Clko1, clko2::Selection as Clko2};

    pub const CLKO1_SELECTIONS: [Clko1; 0] = [];

    pub const CLKO2_SELECTIONS: [Clko2; 0] = [];

    pub const MAX_DIVIDER_VALUE: u32 = 8;
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART1;
    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO5_COMBINED_0_15;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI1;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM1_3;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA11_DMA27;
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
