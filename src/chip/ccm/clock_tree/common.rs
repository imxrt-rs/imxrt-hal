//! Default clock tree frequencies and configuration.
//!
//! All symbols exported in the `clock_tree` parent module for a given
//! chip. It depends on an `ahb_frequency` symbol, which varies by the
//! chip family.

pub use crate::ccm::clock_tree::ahb_frequency;
use crate::{
    ccm::{
        clock_gate, lpi2c_clk::Selection as Lpi2cSelection, lpspi_clk::Selection as LpspiSelection,
        perclk_clk::Selection as PerclkSelection, uart_clk::Selection as UartSelection,
    },
    ral, RunMode,
};

const fn ipg_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 4,
    }
}

/// Returns the target IPG frequency (Hz) for the run mode.
pub const fn ipg_frequency(run_mode: RunMode) -> u32 {
    ahb_frequency(run_mode) / ipg_divider(run_mode)
}

const fn perclk_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 2,
    }
}

const fn perclk_selection(run_mode: RunMode) -> PerclkSelection {
    match run_mode {
        RunMode::Overdrive => PerclkSelection::Ipg,
    }
}

/// Returns the target PERCLK frequency (Hz) for the run mode.
pub const fn perclk_frequency(run_mode: RunMode) -> u32 {
    let hz = match run_mode {
        RunMode::Overdrive => ipg_frequency(run_mode),
    };
    hz / perclk_divider(run_mode)
}

/// Specify the LPSPI clock divider for a given run mode.
const fn lpspi_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 4,
    }
}

/// Returns the target LPSPI clock frequency for the run mode.
pub const fn lpspi_frequency(run_mode: RunMode) -> u32 {
    let hz = match run_mode {
        RunMode::Overdrive => crate::ccm::analog::pll2::FREQUENCY,
    };
    hz / lpspi_divider(run_mode)
}

const _: () = assert!(lpspi_frequency(RunMode::Overdrive) == 132_000_000); // Max allowed

const fn uart_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 1,
    }
}

/// Returns the target UART clock frequency for the run mode.
pub const fn uart_frequency(run_mode: RunMode) -> u32 {
    let hz = match run_mode {
        RunMode::Overdrive => crate::ccm::analog::pll3::FREQUENCY / 6,
    };
    hz / uart_divider(run_mode)
}

const _: () = assert!(uart_frequency(RunMode::Overdrive) == 80_000_000); // Max allowed

const fn lpi2c_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 1,
    }
}

/// Returns the LPI2C clock frequency for the run mode.
pub const fn lpi2c_frequency(run_mode: RunMode) -> u32 {
    let hz = match run_mode {
        RunMode::Overdrive => crate::ccm::analog::pll3::FREQUENCY / 8,
    };
    hz / lpi2c_divider(run_mode)
}

const _: () = assert!(lpi2c_frequency(RunMode::Overdrive) == 60_000_000); // Max is 66MHz.

/// Configure the clock tree.
///
/// When this call returns, the system clock frequencies match the values
/// returned by [`clock_tree`](crate::ccm::clock_tree)'s various `*_frequency`
/// functions.
///
/// `configure` will disable the clock gates for various peripherals. It
/// may leave these clock gates disabled.
pub fn configure(
    run_mode: RunMode,
    ccm: &mut ral::ccm::CCM,
    ccm_analog: &mut ral::ccm_analog::CCM_ANALOG,
) {
    clock_gate::all().for_each(|cg| cg.set(ccm, clock_gate::OFF));

    super::configure_ahb(run_mode, ccm, ccm_analog);
    crate::ccm::ipg_clk::set_divider(ccm, ipg_divider(run_mode));

    crate::ccm::perclk_clk::set_selection(ccm, perclk_selection(run_mode));
    crate::ccm::perclk_clk::set_divider(ccm, perclk_divider(run_mode));

    match run_mode {
        RunMode::Overdrive => {
            crate::ccm::lpspi_clk::set_selection(ccm, LpspiSelection::Pll2);
            crate::ccm::uart_clk::set_selection(ccm, UartSelection::Pll3Div6);
            crate::ccm::lpi2c_clk::set_selection(ccm, Lpi2cSelection::Pll3Div8);
        }
    }

    crate::ccm::lpspi_clk::set_divider(ccm, lpspi_divider(run_mode));
    crate::ccm::uart_clk::set_divider(ccm, uart_divider(run_mode));
    crate::ccm::lpi2c_clk::set_divider(ccm, lpi2c_divider(run_mode));
}
