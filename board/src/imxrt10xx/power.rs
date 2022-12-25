use crate::{hal::dcdc, ral, RunMode};

/// Set the target power for the provided `run_mode`.
pub fn set_target_power(dcdc: &mut ral::dcdc::DCDC, run_mode: RunMode) {
    let millivolts = match run_mode {
        RunMode::Overdrive => 1250,
    };
    dcdc::set_target_vdd_soc(dcdc, millivolts);
}
