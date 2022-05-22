//! Synchronous buck mode converter.
//!
//! The `dcdc` module only provides a thin API over the RAL.

use crate::ral::{self, dcdc::DCDC};

/// Set the target value of `VDD_SOC`, in millivolts
///
/// Values are clamped between 800mV and 1575mV, with 25mV step
/// sizes.
pub fn set_target_vdd_soc(dcdc: &mut DCDC, millivolts: u32) {
    let mv = millivolts.clamp(800, 1575);
    let trg = (mv - 800) / 25;
    ral::modify_reg!(ral::dcdc, dcdc, REG3, TRG: trg);
    while ral::read_reg!(ral::dcdc, dcdc, REG0, STS_DC_OK == 0) {}
}

/// Returns the target value of `VDD_SOC`, in millivolts.
pub fn target_vdd_soc(dcdc: &DCDC) -> u32 {
    let trg = ral::read_reg!(ral::dcdc, dcdc, REG3, TRG);
    trg * 25 + 800
}
