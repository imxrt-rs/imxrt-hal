//! Power Management Unit.
//!
//! The PMu also provides some control for the PHY LDO.
//! Note that the terms "PHY LDO" and `LDO_PLL` represent
//! the same thing, and this API prefers the PHY LDO term.

pub use crate::gpc::{ControlMode, Setpoint};
use crate::ral::{self, anadig_pmu::ANADIG_PMU as PMU};

/// Set the control mode for PHY LDO.
#[inline]
pub fn set_phy_ldo_control(pmu: &mut PMU, control_mode: ControlMode) {
    ral::modify_reg!(ral::anadig_pmu, pmu, PMU_LDO_PLL, LDO_PLL_CONTROL_MODE: control_mode.is_gpc() as u32);
}

/// Specify with setpoints should turn on the PHY_LDO.
///
/// Each high bit indicates that the PHY LDO is on for that setpoint.
/// See [`Setpoint`] documentation for more information.
#[inline]
pub fn set_phy_ldo_setpoints(pmu: &PMU, setpoint: Setpoint) {
    // Low bit == on.
    // High bit == off.
    //
    // Flip the bits, since each software setpoint bit
    // suggests "on."
    ral::write_reg!(
        ral::anadig_pmu,
        pmu,
        LDO_PLL_ENABLE_SP,
        (!setpoint).bits() as u32
    );
}

/// Enable or disable the PLL bandgap reference voltage.
#[inline]
pub fn enable_pll_reference_voltage(pmu: &mut PMU, enable: bool) {
    ral::modify_reg!(ral::anadig_pmu, pmu, PMU_REF_CTRL, EN_PLL_VOL_REF_BUFFER: enable as u32);
}

/// Set the control mode for the PLL bandgap reference voltage.
#[inline]
pub fn set_pll_reference_control(pmu: &mut PMU, control_mode: ControlMode) {
    ral::modify_reg!(ral::anadig_pmu, pmu, PMU_REF_CTRL, REF_CONTROL_MODE: control_mode.is_gpc() as u32);
}
