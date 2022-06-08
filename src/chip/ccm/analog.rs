//! Analog clock control module.
//!
//! Each module describes a system PLL. The symbols in this module depend on the
//! chip selection.

pub use crate::chip::config::plls::*;

/// The system PLL.
pub mod pll2 {
    /// PLL2 frequency (Hz).
    ///
    /// The reference manual notes that PLL2 should always run at 528MHz,
    /// so this constant assumes that PLL2's DIV_SELECT field isn't
    /// changed at runtime.
    pub const FREQUENCY: u32 = 528_000_000;

    /// The smallest PLL2_PFD divider.
    pub const MIN_FRAC: u8 = super::pll3::MIN_FRAC;
    /// The largest PLL2_PFD divider.
    pub const MAX_FRAC: u8 = super::pll3::MAX_FRAC;
}

/// The USB PLL.
///
/// When an implementation has multiple USB peripherals, this
/// PLL is associated with USB1.
pub mod pll3 {
    /// PLL3 frequency (Hz).
    ///
    /// The reference manual notes that PLL3 should always run at 480MHz,
    /// so this constant assumes that PLL3's DIV_SELECT field isn't
    /// changed at runtime.
    pub const FREQUENCY: u32 = 480_000_000;

    /// The smallest PLL3_PFD divider.
    pub const MIN_FRAC: u8 = 12;
    /// The largest PLL3_PFD divider.
    pub const MAX_FRAC: u8 = 35;
}
