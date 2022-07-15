//! A thin board support package for `imxrt-hal` hardware examples.
//!
//! The top-level module exposes configurations and APIs that are common across
//! boards. For board specific information, like which LPUART is the console and
//! which pins are I2C, see the board-specific modules.

#![no_std]

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

mod ral_shim;
mod rt;

pub use ral_shim::{interrupt, Interrupt, BOARD_DMA_A_INDEX, NVIC_PRIO_BITS};

#[cfg(board = "imxrt1010evk")]
#[path = "imxrt1010evk.rs"]
mod board;

#[cfg(board = "teensy4")]
#[path = "teensy4.rs"]
mod board;

pub use board::*;

/// Components that are common to all boards.
///
/// This includes timers, DMA channels, and things
/// that don't necessarily depend on a pinout.
pub struct Common {
    /// PIT channels.
    pub pit: hal::pit::Channels,
    /// GPT1 timer.
    ///
    /// Use [`GPT1_FREQUENCY`] to understand its frequency.
    pub gpt1: hal::gpt::Gpt<1>,
    /// GPT2 timer.
    ///
    /// Use [`GPT2_FREQUENCY`] to understand its frequency.
    pub gpt2: hal::gpt::Gpt<2>,
    /// DMA channels.
    pub dma: [Option<hal::dma::channel::Channel>; hal::dma::CHANNEL_COUNT],
    /// True random number generator.
    pub trng: hal::trng::Trng,
    /// USB1 core registers.
    pub usb1: Usb1,
    /// USB1 non-core registers.
    pub usbnc1: UsbNc1,
    /// USBPHY1 registers.
    pub usbphy1: UsbPhy1,
    /// USB_ANALOG registers.
    pub usb_analog: UsbAnalog,
}

impl Common {
    /// Take common resources.
    ///
    /// Returns `None` if _any_ resource is already taken.
    fn take() -> Option<Self> {
        let pit = ral::pit::PIT::take()?;
        // Stop timers in debug mode.
        ral::modify_reg!(ral::pit, pit, MCR, FRZ: FRZ_1);
        let pit = hal::pit::new(pit);

        let gpt1 = configure_gpt(ral::gpt::GPT1::take()?, GPT1_DIVIDER);
        let gpt2 = configure_gpt(ral::gpt::GPT2::take()?, GPT2_DIVIDER);

        let dma = hal::dma::channels(ral::dma0::DMA0::take()?, ral::dmamux::DMAMUX::take()?);
        let trng = hal::trng::Trng::new(
            ral::trng::TRNG::take()?,
            Default::default(),
            Default::default(),
        );
        Some(Self {
            pit,
            gpt1,
            gpt2,
            dma,
            trng,
            usb1: Usb1::take()?,
            usbnc1: UsbNc1::take()?,
            usbphy1: UsbPhy1::take()?,
            usb_analog: UsbAnalog::take()?,
        })
    }
}

/// Configure board clocks and power.
///
/// This should be the first call to every example. This call takes
/// RAL resources, and releases them before exit.
///
/// # Panics
///
/// Panics if any of the RAL resources are already taken.
fn configure() {
    let mut ccm = ral::ccm::CCM::take().unwrap();
    let mut ccm_analog = ral::ccm_analog::CCM_ANALOG::take().unwrap();
    let mut dcdc = ral::dcdc::DCDC::take().unwrap();

    hal::ccm::set_low_power_mode(&mut ccm, hal::ccm::LowPowerMode::RemainInRun);
    hal::set_target_power(&mut dcdc, RUN_MODE);
    prepare_clock_tree(&mut ccm, &mut ccm_analog);

    COMMON_CLOCK_GATES
        .into_iter()
        .chain(board::CLOCK_GATES.into_iter())
        .for_each(|locator: &clock_gate::Locator| {
            locator.set(&mut ccm, hal::ccm::clock_gate::ON);
        });

    ral::dcdc::DCDC::release(dcdc);
    ral::ccm_analog::CCM_ANALOG::release(ccm_analog);
    ral::ccm::CCM::release(ccm);
}

use hal::ccm::clock_gate;
const COMMON_CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::pit(),
    clock_gate::gpt_bus::<1>(),
    clock_gate::gpt_bus::<2>(),
    clock_gate::gpt_serial::<1>(),
    clock_gate::gpt_serial::<2>(),
    clock_gate::dma(),
    clock_gate::usb(),
    clock_gate::trng(),
];

/// Board entrypoint.
///
/// Use this to configure the hardware and acquire peripherals.
///
/// # Panics
///
/// This should only be called once, at the top of your `main()` routine.
/// It panics if any hardware resource is already taken.
pub fn new() -> (Common, Specifics) {
    configure();
    (Common::take().unwrap(), Specifics::take().unwrap())
}

/// The board's run mode.
pub const RUN_MODE: hal::RunMode = hal::RunMode::Overdrive;

/// The PIT clock frequency (Hz).
pub const PIT_FREQUENCY: u32 = hal::ccm::clock_tree::perclk_frequency(RUN_MODE);

const GPT1_DIVIDER: u32 = 10;
const GPT2_DIVIDER: u32 = 100;
const GPT_SELECTION: hal::gpt::ClockSource = hal::gpt::ClockSource::HighFrequencyReferenceClock;
/// The GPT1 clock frequency (Hz).
pub const GPT1_FREQUENCY: u32 = hal::ccm::clock_tree::perclk_frequency(RUN_MODE) / GPT1_DIVIDER;
/// The GPT2 clock frequency (Hz).
pub const GPT2_FREQUENCY: u32 = hal::ccm::clock_tree::perclk_frequency(RUN_MODE) / GPT2_DIVIDER;

/// The UART clock frequency (Hz).
pub const UART_CLK_FREQUENCY: u32 = hal::ccm::clock_tree::uart_frequency(RUN_MODE);
/// The console baud rate: 115200bps.
pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);

/// The LPSPI clock frequency (Hz).
pub const LPSPI_CLK_FREQUENCY: u32 = hal::ccm::clock_tree::lpspi_frequency(RUN_MODE);
/// Target SPI baud rate (Hz).
pub const SPI_BAUD_RATE_FREQUENCY: u32 = 1_000_000;

/// The LPI2C clock frequency (Hz).
pub const LPI2C_CLK_FREQUENCY: u32 = hal::ccm::clock_tree::lpi2c_frequency(RUN_MODE);
/// Target I2C baud rate (Hz).
pub const I2C_BAUD_RATE: hal::lpi2c::Timing =
    hal::lpi2c::timing(hal::lpi2c::ClockSpeed::KHz400, RUN_MODE);

#[cfg(family = "imxrt1010")]
use iomuxc::imxrt1010::Pads;

#[cfg(family = "imxrt1060")]
use iomuxc::imxrt1060::Pads;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

fn configure_gpt<const N: u8>(gpt: ral::gpt::Instance<N>, divider: u32) -> hal::gpt::Gpt<N>
where
    ral::gpt::Instance<N>: ral::Valid,
{
    let mut gpt = hal::gpt::Gpt::new(gpt);
    gpt.disable();
    gpt.set_wait_mode_enable(true);
    gpt.set_clock_source(GPT_SELECTION);
    gpt.set_divider(divider);
    gpt
}

fn prepare_clock_tree(
    ccm: &mut crate::ral::ccm::CCM,
    ccm_analog: &mut crate::ral::ccm_analog::CCM_ANALOG,
) {
    use crate::hal::ccm;
    ccm::clock_tree::configure_ahb_ipg(RUN_MODE, ccm, ccm_analog);
    ccm::clock_tree::configure_lpi2c(RUN_MODE, ccm);
    ccm::clock_tree::configure_lpspi(RUN_MODE, ccm);
    ccm::clock_tree::configure_perclk(RUN_MODE, ccm);
    ccm::clock_tree::configure_uart(RUN_MODE, ccm);
    ccm::analog::pll3::restart(ccm_analog);
}

#[cfg(family = "imxrt1010")]
mod usb1 {
    use crate::ral;

    pub type Usb1 = ral::usb::USB;
    pub type UsbPhy1 = ral::usbphy::USBPHY;
    pub type UsbNc1 = ral::usbnc::USBNC;
    pub type UsbAnalog = ral::usb_analog::USB_ANALOG;
}

#[cfg(not(family = "imxrt1010"))]
mod usb1 {
    use crate::ral;

    pub type Usb1 = ral::usb::USB1;
    pub type UsbPhy1 = ral::usbphy::USBPHY1;
    pub type UsbNc1 = ral::usbnc::USBNC1;
    pub type UsbAnalog = ral::usb_analog::USB_ANALOG;
}

use usb1::*;
