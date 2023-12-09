//! General Power Controller.

use crate::ral::{self, gpc_cpu_mode_ctrl_::RegisterBlock as GpcCtrl};

/// Describes the power / clock control mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ControlMode {
    /// Software, not the GPC, controls the element.
    Software,
    /// The GPC controls the element.
    Gpc,
}

impl ControlMode {
    pub(crate) const fn is_gpc(self) -> bool {
        matches!(self, Self::Gpc)
    }
}

bitflags::bitflags! {
    /// A bitmask for specifying setpoints.
    ///
    /// No matter the API, a high bit indicates that the setpoint
    /// is "enabled,", "compatible," or "on" for the given configuration.
    /// A lower bit indicates that the setpoint is "disabled," "not
    /// compatible," or "off" for the configuration. (Some registers have
    /// inverted interpretation; nevertheless, the APIs hide this for
    /// consistency.)
    pub struct Setpoint: u16 {
        /// Setpoint 0.
        const SP0 = 1 << 0;
        /// Setpoint 1.
        const SP1 = 1 << 1;
        /// Setpoint 2.
        const SP2 = 1 << 2;
        /// Setpoint 3.
        const SP3 = 1 << 3;
        /// Setpoint 4.
        const SP4 = 1 << 4;
        /// Setpoint 5.
        const SP5 = 1 << 5;
        /// Setpoint 6.
        const SP6 = 1 << 6;
        /// Setpoint 7.
        const SP7 = 1 << 7;
        /// Setpoint 8.
        const SP8 = 1 << 8;
        /// Setpoint 9.
        const SP9 = 1 << 9;
        /// Setpoint 10.
        const SP10 = 1 << 10;
        /// Setpoint 11.
        const SP11 = 1 << 11;
        /// Setpoint 12.
        const SP12 = 1 << 12;
        /// Setpoint 13.
        const SP13 = 1 << 13;
        /// Setpoint 14.
        const SP14 = 1 << 14;
        /// Setpoint 15.
        const SP15 = 1 << 15;
    }
}

/// The setpoint is out of range.
///
/// Setpoints are bound between 0 and 15, inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetpointOutOfRangeError(pub u32);

/// Request a run mode setpoint transition.
///
/// Blocks until the transition completes.
pub fn request_setpoint_transition(
    gpc: &GpcCtrl,
    setpoint: u32,
) -> Result<(), SetpointOutOfRangeError> {
    if setpoint < 16 {
        ral::modify_reg!(ral::gpc_cpu_mode_ctrl_, gpc, CM_SP_CTRL,
            CPU_SP_RUN: setpoint,
            CPU_SP_RUN_EN: 1
        );
        while ral::read_reg!(ral::gpc_cpu_mode_ctrl_, gpc, CM_SP_CTRL, CPU_SP_RUN_EN == 1) {}
        Ok(())
    } else {
        Err(SetpointOutOfRangeError(setpoint))
    }
}
