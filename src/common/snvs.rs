//! Secure non-volatile storage.
//!
//! The SNVS peripheral implements a few functions. This implementation
//! supports
//!
//! - the secure real-time counter (SRTC) in the low-power (LP) domain.

mod ral;
pub mod srtc;

pub use ral::lp::Core as LpCore;

/// SNVS low-power domain.
#[non_exhaustive]
pub struct LowPower {
    /// Core registers.
    ///
    /// This handled is passed into other SNVS low-power components
    /// when required.
    pub core: ral::lp::Core,
    /// Secure real-time counter.
    pub srtc: srtc::Disabled,
}

/// SNVS components.
///
/// Use [`new`](crate::snvs::new) to create the components.
#[non_exhaustive]
pub struct Snvs {
    /// Components for the low power domain.
    pub low_power: LowPower,
}

/// Decompose the SNVS peripheral into its components.
pub fn new(snvs: crate::ral::snvs::SNVS) -> Snvs {
    let components = ral::new(snvs);
    Snvs {
        low_power: LowPower {
            core: components.lp_core,
            srtc: srtc::Disabled::new(components.lp_srtc),
        },
    }
}
