//! Board configurations for `imxrt-hal` examples.

#![no_std]

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

mod ral_shim;
mod rt;

pub use ral_shim::{interrupt, Interrupt, Peripherals, BOARD_DMA_A_INDEX, NVIC_PRIO_BITS};

#[cfg(board = "imxrt1010evk")]
#[path = "imxrt1010evk.rs"]
mod board;

#[cfg(board = "teensy4")]
#[path = "teensy4.rs"]
mod board;

pub use board::*;

/// Resources available for all boards.
pub struct Board {
    /// GPIO output for the board's LED.
    pub led: Led,
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
    /// UART console.
    pub console: Console,
    /// DMA channels.
    pub dma: [Option<hal::dma::channel::Channel>; hal::dma::CHANNEL_COUNT],
    /// SPI peripheral.
    pub spi: Spi,
    /// I2C peripheral.
    pub i2c: I2c,
    /// CCM registers.
    pub ccm: ral::ccm::CCM,
    /// Any board-specific resouces.
    ///
    /// For example portability, try to minimize these.
    pub specifics: board::Specifics,
}

/// Peripheral instances required by the board.
///
/// This is an opaque structure. If it exists, the board
/// has ownership of all `imxrt-ral` peripheral register
/// blocks that it requires. Use [`take()`](Instances::take)
/// to safely acquire those peripherals.
#[allow(dead_code)] // Might not be used by all boards.
pub struct Instances {
    gpio1: ral::gpio::GPIO1,
    gpio2: ral::gpio::GPIO2,
    iomuxc: ral::iomuxc::IOMUXC,
    gpt1: ral::gpt::GPT1,
    gpt2: ral::gpt::GPT2,
    lpuart1: ral::lpuart::LPUART1,
    lpuart2: ral::lpuart::LPUART2,
    pit: ral::pit::PIT,
    dma: ral::dma0::DMA0,
    dma_mux: ral::dmamux::DMAMUX,
    ccm: ral::ccm::CCM,
    ccm_analog: ral::ccm_analog::CCM_ANALOG,
    dcdc: ral::dcdc::DCDC,
    lpspi1: ral::lpspi::LPSPI1,
    lpi2c1: ral::lpi2c::LPI2C1,
    #[cfg(family = "imxrt1060")]
    lpspi4: ral::lpspi::LPSPI4,
    #[cfg(family = "imxrt1060")]
    lpi2c3: ral::lpi2c::LPI2C3,
}

impl Instances {
    pub fn take() -> Option<Self> {
        Some(Self {
            gpio1: ral::gpio::GPIO1::take()?,
            gpio2: ral::gpio::GPIO2::take()?,
            iomuxc: ral::iomuxc::IOMUXC::take()?,
            gpt1: ral::gpt::GPT1::take()?,
            gpt2: ral::gpt::GPT2::take()?,
            lpuart1: ral::lpuart::LPUART1::take()?,
            lpuart2: ral::lpuart::LPUART2::take()?,
            pit: ral::pit::PIT::take()?,
            dma: ral::dma0::DMA0::take()?,
            dma_mux: ral::dmamux::DMAMUX::take()?,
            ccm: ral::ccm::CCM::take()?,
            ccm_analog: ral::ccm_analog::CCM_ANALOG::take()?,
            dcdc: ral::dcdc::DCDC::take()?,
            lpspi1: ral::lpspi::LPSPI1::take()?,
            lpi2c1: ral::lpi2c::LPI2C1::take()?,
            #[cfg(family = "imxrt1060")]
            lpspi4: ral::lpspi::LPSPI4::take()?,
            #[cfg(family = "imxrt1060")]
            lpi2c3: ral::lpi2c::LPI2C3::take()?,
        })
    }
}

impl From<ral::Peripherals> for Instances {
    fn from(p: ral::Peripherals) -> Self {
        Self {
            gpio1: p.GPIO1,
            gpio2: p.GPIO2,
            iomuxc: p.IOMUXC,
            gpt1: p.GPT1,
            gpt2: p.GPT2,
            lpuart1: p.LPUART1,
            lpuart2: p.LPUART2,
            pit: p.PIT,
            dma: p.DMA0,
            dma_mux: p.DMAMUX,
            ccm: p.CCM,
            ccm_analog: p.CCM_ANALOG,
            dcdc: p.DCDC,
            lpspi1: p.LPSPI1,
            lpi2c1: p.LPI2C1,
            #[cfg(family = "imxrt1060")]
            lpspi4: p.LPSPI4,
            #[cfg(family = "imxrt1060")]
            lpi2c3: p.LPI2C3,
        }
    }
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
pub const I2C_BAUD_RATE: hal::lpi2c::timing::ClockSpeed = hal::lpi2c::timing::ClockSpeed::KHz400;

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

/// Configure PIT channels.
fn configure_pit(pit: ral::pit::PIT) -> hal::pit::Channels {
    // Stop timers in debug mode.
    ral::modify_reg!(ral::pit, pit, MCR, FRZ: FRZ_1);
    hal::pit::new(pit)
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

/// Prepare all board components.
///
/// # Panics
///
/// Panics if any board component is already taken from the RAL.
pub fn prepare() -> Board {
    Instances::take()
        .map(board::new)
        .expect("Board component already taken")
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
}
