//! Exposes routines to configure the AHB clock from PLL1.
//!
//! It supports the 1060. It should also support the 1050.

use crate::ral;
use crate::{hal::ccm, RunMode};

/// Specify the PLL1 DIV_SEL for a given run mode.
const fn div_sel(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 100,
    }
}

/// Specify the PLL1 divider for a given run mode.
const fn arm_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 2,
    }
}

/// Specify the AHB divider for a given run mode.
const fn ahb_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 1,
    }
}

/// Returns the target AHB frequency for the provided run mode.
pub const fn ahb_frequency(run_mode: RunMode) -> u32 {
    ccm::analog::pll1::frequency(div_sel(run_mode)) / arm_divider(run_mode) / ahb_divider(run_mode)
}

const _: () = assert!(600_000_000 == ahb_frequency(RunMode::Overdrive));

/// Configure the AHB and IPG clock.
///
/// When this call returns, the AHB and IPG clocks frequencies match the
/// values returned by the [`ahb_frequency()`] and
/// [`ipg_frequency()`](super::clock_tree::ipg_frequency) functions.
///
/// This function will disable the clock gates for various peripherals. It may
/// leave these clock gates disabled.
pub fn configure_ahb_ipg(
    run_mode: RunMode,
    ccm: &mut ral::ccm::CCM,
    ccm_analog: &mut ral::ccm_analog::CCM_ANALOG,
) {
    ccm::clock_gate::IPG_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, ccm::clock_gate::OFF));

    if ccm::ahb_clk::Selection::PeriphClk2Sel == ccm::ahb_clk::selection(ccm) {
        // Switch to the pre-peripheral clock before changing
        // peripheral clock 2...
        ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PrePeriphClkSel);
    }

    // Temporarily switch to the crystal oscillator.
    ccm::periph_clk2::set_divider(ccm, 1);
    ccm::periph_clk2::set_selection(ccm, ccm::periph_clk2::Selection::Osc);
    ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PeriphClk2Sel);

    match run_mode {
        RunMode::Overdrive => {
            // Prepare PLL1.
            ccm::analog::pll1::restart(ccm_analog, div_sel(run_mode));
            ccm::arm_divider::set_divider(ccm, arm_divider(run_mode));
            ccm::ahb_clk::set_divider(ccm, ahb_divider(run_mode));

            // Switch back to PLL1.
            ccm::pre_periph_clk::set_selection(ccm, ccm::pre_periph_clk::Selection::Pll1);
            ccm::ahb_clk::set_selection(ccm, ccm::ahb_clk::Selection::PrePeriphClkSel);
        }
    }

    ccm::ipg_clk::set_divider(ccm, super::clock_tree::ipg_divider(run_mode));
}
