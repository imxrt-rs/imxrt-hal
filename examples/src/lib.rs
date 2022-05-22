//! Board configurations for `imxrt-hal` examples.

#![no_std]

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;

mod rt;

#[cfg(feature = "imxrt1010evk")]
#[path = "imxrt1010evk.rs"]
mod board;

#[cfg(feature = "teensy4")]
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
}

/// Peripheral register blocks required by the board.
///
/// This is an opaque structure. If it exists, the board
/// has ownership of all `imxrt-ral` peripheral register
/// blocks that it requires. Use [`take()`](Peripherals::take)
/// to safely acquire those peripherals.
#[allow(dead_code)] // TODO remove.
pub struct Peripherals {
    gpio1: ral::gpio::GPIO1,
    gpio2: ral::gpio::GPIO2,
    iomuxc: ral::iomuxc::IOMUXC,
    gpt1: ral::gpt::GPT1,
    gpt2: ral::gpt::GPT2,
    lpuart1: ral::lpuart::LPUART1,
    pit: ral::pit::PIT,
    ccm: ral::ccm::CCM,
    ccm_analog: ral::ccm_analog::CCM_ANALOG,
    dcdc: ral::dcdc::DCDC,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        Some(Self {
            gpio1: ral::gpio::GPIO1::take()?,
            gpio2: ral::gpio::GPIO2::take()?,
            iomuxc: ral::iomuxc::IOMUXC::take()?,
            gpt1: ral::gpt::GPT1::take()?,
            gpt2: ral::gpt::GPT2::take()?,
            lpuart1: ral::lpuart::LPUART1::take()?,
            pit: ral::pit::PIT::take()?,
            ccm: ral::ccm::CCM::take()?,
            ccm_analog: ral::ccm_analog::CCM_ANALOG::take()?,
            dcdc: ral::dcdc::DCDC::take()?,
        })
    }
}

#[cfg(feature = "rtic")]
impl From<ral::Peripherals> for Peripherals {
    fn from(p: ral::Peripherals) -> Self {
        Self {
            gpio1: p.GPIO1,
            gpio2: p.GPIO2,
            iomuxc: p.IOMUXC,
            gpt1: p.GPT1,
            gpt2: p.GPT2,
            lpuart1: p.LPUART1,
            pit: p.PIT,
            ccm: p.CCM,
            ccm_analog: p.CCM_ANALOG,
            dcdc: p.DCDC,
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

#[cfg(imxrt1010)]
use iomuxc::imxrt1010::Pads;

#[cfg(imxrt1060)]
use iomuxc::imxrt1060::Pads;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

/// Configure PIT channels.
fn configure_pit(pit: ral::pit::PIT, ccm: &mut ral::ccm::CCM) -> hal::pit::Channels {
    hal::ccm::clock_gate::pit().set(ccm, hal::ccm::clock_gate::ON);
    // Stop timers in debug mode.
    ral::modify_reg!(ral::pit, pit, MCR, FRZ: FRZ_1);
    hal::pit::new(pit)
}

/// Configure a GPIO port.
fn configure_gpio<const N: u8>(
    gpio: ral::gpio::Instance<N>,
    ccm: &mut ral::ccm::CCM,
) -> hal::gpio::Port<N>
where
    ral::gpio::Instance<N>: ral::Valid,
{
    hal::ccm::clock_gate::gpio::<N>().set(ccm, hal::ccm::clock_gate::ON);
    hal::gpio::Port::new(gpio)
}

fn configure_gpt<const N: u8>(
    gpt: ral::gpt::Instance<N>,
    divider: u32,
    ccm: &mut ral::ccm::CCM,
) -> hal::gpt::Gpt<N>
where
    ral::gpt::Instance<N>: ral::Valid,
{
    hal::ccm::clock_gate::gpt_serial::<N>().set(ccm, hal::ccm::clock_gate::ON);
    hal::ccm::clock_gate::gpt_bus::<N>().set(ccm, hal::ccm::clock_gate::ON);
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
    Peripherals::take()
        .map(board::new)
        .expect("Board component already taken")
}

/// TODO featureful panic handler
#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
