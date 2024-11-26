//! Clock control module for 11xx MCUs.
//!
//! The implementation assumes that the CCM operates in
//! "unassigned mode." See the section on CCM modes in
//! the reference manual (15.5.1.) for more information.
//! The API mimics the high-level clock gate and tree APIs
//!  for the 10xx family.

pub mod clock_gate;
pub mod output_source;

pub use crate::common::ccm::XTAL_OSCILLATOR_HZ;

pub use crate::gpc::{ControlMode, Setpoint};
use crate::ral::{self, ccm::CCM};

/// A clock source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
#[non_exhaustive]
pub enum ClockSource {
    /// The voltage controlled oscillator for PLL1.
    ///
    /// This output isn't exposed by the clock tree.
    /// However, the [`Pll1Clk`] is exposed.
    Pll1 = 21,
    /// The PLL1 output into the clock tree.
    Pll1Clk = 22,
    /// PLL1 with a fixed divide-by-2.
    Pll1Div2 = 23,
    /// PLL1 with a fixed divide-by-5.
    Pll1Div5 = 24,
}

/// Indicates that the clock does not support setpoint control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetpointNotImplementedError(());

/// Signals if the clock source supports setpoint control.
#[inline]
pub fn has_setpoint(ccm: &CCM, clock_source: ClockSource) -> bool {
    let oscpll = &ccm.OSCPLL[clock_source as usize];
    ral::read_reg!(
        ral::ccm::oscpll,
        oscpll,
        OSCPLL_CONFIG,
        SETPOINT_PRESENT == 1
    )
}

/// Set the GPC setpoint control mode for the clock source.
///
/// Returns an error if [`has_setpoint`] does not report support
/// for setpoints.
#[inline]
pub fn set_gpc_control_mode(
    ccm: &mut CCM,
    clock_source: ClockSource,
    control_mode: ControlMode,
) -> Result<(), SetpointNotImplementedError> {
    if control_mode == ControlMode::Gpc && !has_setpoint(ccm, clock_source) {
        return Err(SetpointNotImplementedError(()));
    }

    let oscpll = &ccm.OSCPLL[clock_source as usize];
    ral::modify_reg!(ral::ccm::oscpll, oscpll, OSCPLL_AUTHEN, SETPOINT_MODE: control_mode.is_gpc() as u32);
    Ok(())
}

/// Configure the setpoints for the clock.
///
/// Note: this does not affect the standby setpoints.
#[inline]
pub fn set_setpoints(ccm: &mut CCM, clock_source: ClockSource, setpoint: Setpoint) {
    let oscpll = &ccm.OSCPLL[clock_source as usize];
    ral::modify_reg!(ral::ccm::oscpll, oscpll, OSCPLL_SETPOINT, SETPOINT: setpoint.bits() as u32);
}
