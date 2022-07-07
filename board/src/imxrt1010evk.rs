//! IMXRT1010EVK board configuration.
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

use crate::{hal, iomuxc::imxrt1010 as iomuxc, ral, RUN_MODE};

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio::GPIO_11>;
/// The UART console.
pub type Console = hal::lpuart::Lpuart<ConsolePins, 1>;
pub type ConsolePins = crate::hal::lpuart::Pins<iomuxc::gpio::GPIO_10, iomuxc::gpio::GPIO_09>;

pub type SpiPins = hal::lpspi::Pins<
    iomuxc::gpio_ad::GPIO_AD_04, // SDO, J57_8
    iomuxc::gpio_ad::GPIO_AD_03, // SDI, J57_10
    iomuxc::gpio_ad::GPIO_AD_06, // SCK, J57_12
    iomuxc::gpio_ad::GPIO_AD_05, // PCS0, J57_6
>;

#[cfg(feature = "spi")]
pub type Spi = hal::lpspi::LpspiMaster<SpiPins, 1>;

#[cfg(not(feature = "spi"))]
pub type Spi = ();

pub type I2cPins = hal::lpi2c::Pins<
    iomuxc::gpio::GPIO_02, // SCL, J57_20
    iomuxc::gpio::GPIO_01, // SDA, J57_18
>;

pub type I2c = hal::lpi2c::Lpi2cMaster<I2cPins, 1>;

/// PWM components.
#[cfg(not(feature = "spi"))]
pub mod pwm {
    use super::iomuxc;
    use crate::hal::flexpwm;

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

/// Board-specific resources.
pub struct Specifics {
    pub tp31: Tp31,
    pub tp34: Tp34,
}

pub type Usb1 = ral::usb::USB;
pub type UsbPhy1 = ral::usbphy::USBPHY;
pub type UsbNc1 = ral::usbnc::USBNC;
pub type UsbAnalog = ral::usb_analog::USB_ANALOG;

/// Prepare all board resources, and return them.
pub fn new<P: Into<super::Instances>>(peripherals: P) -> super::Board {
    #[cfg(target_arch = "arm")]
    rtt_target::rtt_init_print!();

    let super::Instances {
        gpio1,
        iomuxc,
        pit,
        gpt1,
        gpt2,
        lpuart1,
        dma,
        dma_mux,
        lpspi1: _lpspi1,
        lpi2c1,
        flexpwm: _flexpwm,
        mut ccm,
        mut ccm_analog,
        mut dcdc,
        usb1,
        usbnc1,
        usb_analog,
        usbphy1,
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

    let mut gpio1 = hal::gpio::Port::new(gpio1);
    let led = gpio1.output(iomuxc.gpio.p11);

    let pit = super::configure_pit(pit);

    let gpt1 = super::configure_gpt(gpt1, super::GPT1_DIVIDER);
    let gpt2 = super::configure_gpt(gpt2, super::GPT2_DIVIDER);

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
        let pins = SpiPins {
            sdo: iomuxc.gpio_ad.p04,
            sdi: iomuxc.gpio_ad.p03,
            sck: iomuxc.gpio_ad.p06,
            pcs0: iomuxc.gpio_ad.p05,
        };
        let mut spi = Spi::new(_lpspi1, pins);
        spi.disabled(|spi| {
            spi.set_clock_hz(super::LPSPI_CLK_FREQUENCY, super::SPI_BAUD_RATE_FREQUENCY);
        });
        spi
    };

    #[cfg(not(feature = "spi"))]
    let spi = ();

    let i2c = I2c::new(
        lpi2c1,
        I2cPins {
            scl: iomuxc.gpio.p02,
            sda: iomuxc.gpio.p01,
        },
        &super::I2C_BAUD_RATE,
    );

    let dma = hal::dma::channels(dma, dma_mux);

    #[cfg(not(feature = "spi"))]
    let pwm = {
        let (pwm, (_, _, sm, _)) = hal::flexpwm::new(_flexpwm);

        let out_a = hal::flexpwm::Output::new_a(iomuxc.gpio_ad.p04);
        let out_b = hal::flexpwm::Output::new_b(iomuxc.gpio_ad.p03);

        super::Pwm {
            module: pwm,
            submodule: sm,
            outputs: (out_a, out_b),
        }
    };

    #[cfg(feature = "spi")]
    let pwm = super::Pwm {
        module: (),
        submodule: (),
        outputs: (),
    };

    let specifics = Specifics {
        tp31: iomuxc.gpio_sd.p01,
        tp34: iomuxc.gpio_sd.p02,
    };
    super::Board {
        led,
        pit,
        gpt1,
        gpt2,
        console,
        dma,
        spi,
        i2c,
        pwm,
        ccm,
        usb1,
        usbnc1,
        usb_analog,
        usbphy1,
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
    clock_gate::gpio::<1>(),
    clock_gate::lpuart::<{ Console::N }>(),
    clock_gate::dma(),
    #[cfg(feature = "spi")]
    clock_gate::lpspi::<{ Spi::N }>(),
    clock_gate::lpi2c::<{ I2c::N }>(),
    #[cfg(not(feature = "spi"))]
    clock_gate::flexpwm::<{ pwm::Peripheral::N }>(),
    clock_gate::usb(),
];

/// Configure board pins.
///
/// Peripherals are responsible for pin muxing, so there's no need to
/// set alternates here.
fn configure_pins(super::Pads { ref mut gpio, .. }: &mut super::Pads) {
    use crate::iomuxc;
    const I2C_PIN_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_open_drain(iomuxc::OpenDrain::Enabled)
        .set_slew_rate(iomuxc::SlewRate::Fast)
        .set_drive_strength(iomuxc::DriveStrength::R0_4)
        .set_speed(iomuxc::Speed::Fast)
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup22k));

    iomuxc::configure(&mut gpio.p02, I2C_PIN_CONFIG);
    iomuxc::configure(&mut gpio.p01, I2C_PIN_CONFIG);
}

/// Flash configuration block.
///
/// Only supports QuadI/O read sequences, since that's
/// all that's necessary to get the board booted.
mod fcb {

    use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
    use imxrt_boot_gen::serial_flash::*;

    /// Adesto Technologies AT25SF128A.
    ///
    /// 128 Mbit Serial NOR Flash Memory
    /// w/ Dual & Quad I/O Support.
    mod at25sf128a {
        /// See 8.2.6 of the data sheet.
        pub const FAST_READ_QUAD_IO: u8 = 0xEB;

        const DENSITY_BITS: u32 = 128 * 1024 * 1024;
        pub const DENSITY_BYTES: u32 = DENSITY_BITS / 8;
    }

    use at25sf128a::*;

    const SEQ_READ: Sequence = SequenceBuilder::new()
        .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
        .instr(Instr::new(RADDR, Pads::Four, 0x18))
        .instr(Instr::new(DUMMY, Pads::Four, 0x06))
        .instr(Instr::new(READ, Pads::Four, 0x04))
        .build();

    const LUT: LookupTable = LookupTable::new().command(Command::Read, SEQ_READ);

    const COMMON_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
        flexspi::ConfigurationBlock::new(LUT)
            .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
            .cs_hold_time(0x03)
            .cs_setup_time(0x03)
            .column_address_width(ColumnAddressWidth::OtherDevices)
            .device_mode_configuration(DeviceModeConfiguration::Disabled)
            .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
            .flash_size(SerialFlashRegion::A1, at25sf128a::DENSITY_BYTES)
            .serial_clk_freq(SerialClockFrequency::MHz100)
            .serial_flash_pad_type(FlashPadType::Quad);

    /// Name is important; it's EXTERNed by the linker script.
    #[used]
    #[no_mangle]
    #[cfg_attr(target_arch = "arm", link_section = ".fcb")]
    pub static FLEXSPI_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
        nor::ConfigurationBlock::new(COMMON_CONFIGURATION_BLOCK)
            .page_size(256)
            .sector_size(4096)
            .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::NoChange);
}

/// Helpers for the clock_out example.
pub mod clock_out {
    use crate::hal::ccm::output_source::{clko1::Selection as Clko1, clko2::Selection as Clko2};

    pub fn prepare_outputs(specifics: &mut super::Specifics) {
        crate::iomuxc::ccm::prepare(&mut specifics.tp31);
        crate::iomuxc::ccm::prepare(&mut specifics.tp34);
    }

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
        Clko2::TracClk,
        Clko2::FlexspiClk,
        Clko2::UartClk,
        Clko2::Spdif0Clk,
    ];
}

#[cfg(target_arch = "arm")]
use panic_rtt_target as _;
