//! Clock and timing support for 10xx processors.
//!
//! This module was separated by the common board items during an 11xx board
//! bringup. As of this writing, it's unclear if we'll be able to support a common
//! API for 10xx and 11xx clocking. It's likely that we will be able to build a common
//! API, so we'll revisit this separation later.

use super::clock_tree;
use crate::{board_impl, hal, ral, GPT1_DIVIDER, GPT2_DIVIDER, RUN_MODE};

/// Configure board clocks and power.
///
/// # Safety
///
/// Pokes at MMIO. Should only be done once.
pub(crate) unsafe fn configure() {
    let mut ccm = ral::ccm::CCM::instance();
    let mut ccm_analog = ral::ccm_analog::CCM_ANALOG::instance();
    let mut dcdc = ral::dcdc::DCDC::instance();

    hal::ccm::set_low_power_mode(&mut ccm, hal::ccm::LowPowerMode::RemainInRun);
    super::power::set_target_power(&mut dcdc, RUN_MODE);
    prepare_clock_tree(&mut ccm, &mut ccm_analog);

    COMMON_CLOCK_GATES
        .iter()
        .chain(board_impl::CLOCK_GATES.iter())
        .for_each(|locator: &clock_gate::Locator| {
            locator.set(&mut ccm, hal::ccm::clock_gate::ON);
        });
}

fn prepare_clock_tree(
    ccm: &mut crate::ral::ccm::CCM,
    ccm_analog: &mut crate::ral::ccm_analog::CCM_ANALOG,
) {
    use crate::hal::ccm;
    clock_tree::configure_ahb_ipg(RUN_MODE, ccm, ccm_analog);
    clock_tree::configure_lpi2c(RUN_MODE, ccm);
    clock_tree::configure_lpspi(RUN_MODE, ccm);
    clock_tree::configure_perclk(RUN_MODE, ccm);
    clock_tree::configure_uart(RUN_MODE, ccm);
    ccm::analog::pll3::restart(ccm_analog);
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
    clock_gate::snvs_lp(),
    clock_gate::snvs_hp(),
];

/// The PIT clock frequency (Hz).
pub const PIT_FREQUENCY: u32 = clock_tree::perclk_frequency(RUN_MODE);

/// The GPT1 clock frequency (Hz).
pub const GPT1_FREQUENCY: u32 = clock_tree::perclk_frequency(RUN_MODE) / GPT1_DIVIDER;
/// The GPT2 clock frequency (Hz).
pub const GPT2_FREQUENCY: u32 = clock_tree::perclk_frequency(RUN_MODE) / GPT2_DIVIDER;

/// The UART clock frequency (Hz).
pub const UART_CLK_FREQUENCY: u32 = clock_tree::uart_frequency(RUN_MODE);

/// The LPSPI clock frequency (Hz).
pub const LPSPI_CLK_FREQUENCY: u32 = clock_tree::lpspi_frequency(RUN_MODE);

/// The LPI2C clock frequency (Hz).
pub const LPI2C_CLK_FREQUENCY: u32 = clock_tree::lpi2c_frequency(RUN_MODE);

pub const PWM_PRESCALER: hal::flexpwm::Prescaler = hal::flexpwm::Prescaler::Prescaler8;
pub const PWM_FREQUENCY: u32 = clock_tree::ipg_frequency(RUN_MODE) / PWM_PRESCALER.divider();
