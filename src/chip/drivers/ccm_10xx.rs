//! Chip-specific CCM APIs.
//!
//! This module and its submodules should work across all i.MX RT10xx processors
//! (with proper family configuration).

pub use crate::chip::config::ccm::*;

pub mod ahb_clk;
pub mod analog;
pub mod clock_gate;
pub mod output_source;

use crate::ral;

pub use crate::common::ccm::XTAL_OSCILLATOR_HZ;

/// PERCLK clock.
///
/// The PERCLK clock controls GPT and PIT timers.
///
/// # Example
///
/// Use the CCM to set the PERCLK clock selection and frequency.
/// After this snippet runs, the PERCLK clock runs at 8MHz.
/// To safely perform this switch, disable all clock gates to the
/// PIT and GPT peripherals.
///
/// ```no_run
/// use imxrt_ral as ral;
/// use imxrt_hal as hal;
///
/// use hal::ccm::{self, clock_gate};
///
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
///
/// clock_gate::PERCLK_CLOCK_GATES
///     .iter()
///     .for_each(|clock_gate| clock_gate.set(&mut ccm, clock_gate::OFF));
///
/// // 24MHz...
/// ccm::perclk_clk::set_selection(&mut ccm, ccm::perclk_clk::Selection::Oscillator);
/// // ...divided by 3.
/// ccm::perclk_clk::set_divider(&mut ccm, 3);
///
/// clock_gate::PERCLK_CLOCK_GATES
///     .iter()
///     .for_each(|clock_gate| clock_gate.set(&mut ccm, clock_gate::ON));
/// ```
pub mod perclk_clk {
    use crate::ral::{self, ccm::CCM};

    /// PERCLK clock selection.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Selection {
        /// Derive from the IPG clock root.
        Ipg = 0,
        /// Derive from the oscillator clock.
        Oscillator = 1,
    }

    /// Set the PERCLK clock selection.
    #[inline(always)]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CSCMR1, PERCLK_CLK_SEL: selection as u32);
    }

    /// Returns the PERCLK clock selection.
    #[inline(always)]
    pub fn selection(ccm: &CCM) -> Selection {
        if ral::read_reg!(ral::ccm, ccm, CSCMR1, PERCLK_CLK_SEL == 1) {
            Selection::Oscillator
        } else {
            Selection::Ipg
        }
    }

    /// The smallest PERCLK divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest PERCLK divider.
    pub const MAX_DIVIDER: u32 = 64;

    /// Set the PERCLK clock divider.
    ///
    /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CSCMR1, PERCLK_PODF: podf);
    }

    /// Returns the PERCLK clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CSCMR1, PERCLK_PODF) + 1
    }
}

/// IPG clock.
///
/// The IPG clock is divided from the core clock.
pub mod ipg_clk {
    use crate::ral::{self, ccm::CCM};

    /// Returns the IPG clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CBCDR, IPG_PODF) + 1
    }

    /// The smallest IPG divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest IPG divider.
    pub const MAX_DIVIDER: u32 = 4;

    /// Sets the IPG clock divider.
    ///
    /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CBCDR, IPG_PODF: podf);
    }
}

/// Wait for all handshake bits to deassert.
pub(crate) fn wait_handshake(ccm: &crate::ral::ccm::CCM) {
    while crate::ral::read_reg!(crate::ral::ccm, ccm, CDHIPR) != 0 {}
}

/// Low power mode.
///
/// From the reference manual,
///
/// > Setting the low power mode that system will enter on next assertion of dsm_request signal.
///
/// Practically, this affects the processor behavior when you use WFI, WFE, or enter another
/// low-power state. Low-power settings that aren't "run" halt the ARM SYSTICK peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LowPowerMode {
    /// Remain in run mode when entering low power.
    RemainInRun = 0,
    /// Move to wait mode when entering low power.
    TransferToWait = 1,
    /// Stop when entering low power.
    TransferToStop = 2,
}

/// Set the CCM low power mode.
pub fn set_low_power_mode(ccm: &mut ral::ccm::CCM, mode: LowPowerMode) {
    ral::modify_reg!(ral::ccm, ccm, CLPCR, LPM: mode as u32);
}

/// Returns the CCM low power mode.
pub fn low_power_mode(ccm: &ral::ccm::CCM) -> LowPowerMode {
    match ral::read_reg!(ral::ccm, ccm, CLPCR, LPM) {
        0 => LowPowerMode::RemainInRun,
        1 => LowPowerMode::TransferToWait,
        2 => LowPowerMode::TransferToStop,
        _ => unreachable!(),
    }
}

/// UART clock root.
///
/// `uart_clk` provides the clock source for all LPUART peripherals.
/// You must disable LPUART clock gates before selecting the clock
/// and divider.
///
/// # Example
///
/// Select a 24MHz clock for the LPUART peripherals. This would affect
/// how baud rate is computed. Since we're only using the second LPUART
/// peripheral, we only disable and enable its clock gates.
///
/// ```no_run
/// use imxrt_hal as hal;
/// use hal::ccm::{uart_clk, clock_gate};
///
/// use imxrt_ral as ral;
///
/// const UART_CLK_DIVIDER: u32 = 1;
/// const UART_CLK_HZ: u32 = hal::ccm::XTAL_OSCILLATOR_HZ / UART_CLK_DIVIDER;
///
/// # fn opt() -> Option<()> {
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
/// clock_gate::lpuart::<2>().set(&mut ccm, clock_gate::OFF);
/// uart_clk::set_selection(&mut ccm, uart_clk::Selection::Oscillator);
/// uart_clk::set_divider(&mut ccm, UART_CLK_DIVIDER);
///
/// clock_gate::lpuart::<2>().set(&mut ccm, clock_gate::ON);
/// # Some(()) }
/// ```
pub mod uart_clk {
    use crate::ral::{self, ccm::CCM};

    /// Returns the UART clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CSCDR1, UART_CLK_PODF) + 1
    }

    /// The smallest UART clock divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest UART clock divider.
    pub const MAX_DIVIDER: u32 = 1 << 6;

    /// Set the UART clock divider.
    ///
    /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CSCDR1, UART_CLK_PODF: podf);
    }

    /// UART clock selection.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Selection {
        /// PLL 3 divided by 6.
        ///
        /// This is typically 480MHz / 6 == 80MHz.
        Pll3Div6 = 0,
        /// 24MHz oscillator.
        Oscillator = 1,
    }

    /// Return the UART clock selection.
    #[inline(always)]
    pub fn selection(ccm: &CCM) -> Selection {
        match ral::read_reg!(ral::ccm, ccm, CSCDR1, UART_CLK_SEL) {
            0 => Selection::Pll3Div6,
            1 => Selection::Oscillator,
            _ => unreachable!(),
        }
    }

    /// Set the UART clock selection.
    #[inline(always)]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CSCDR1, UART_CLK_SEL: selection as u32);
    }
}

/// LPI2C clock root.
///
/// `lpi2c_clk` provides the clock source for all LPI2C peripherals.
/// You must disable LPI2C clock gates before selecting the clock
/// and divider.
///
/// # Example
///
/// ```no_run
/// use imxrt_hal as hal;
/// use hal::ccm::{lpi2c_clk, clock_gate};
///
/// use imxrt_ral as ral;
///
/// const LPI2C_CLK_DIVIDER: u32 = 3;
/// const LPI2C_CLK_HZ: u32 = hal::ccm::XTAL_OSCILLATOR_HZ / LPI2C_CLK_DIVIDER;
///
/// # fn opt() -> Option<()> {
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
/// clock_gate::lpi2c::<2>().set(&mut ccm, clock_gate::OFF);
/// lpi2c_clk::set_selection(&mut ccm, lpi2c_clk::Selection::Oscillator);
/// lpi2c_clk::set_divider(&mut ccm, LPI2C_CLK_DIVIDER);
/// clock_gate::lpi2c::<2>().set(&mut ccm, clock_gate::ON);
/// # Some(()) }
/// ```
pub mod lpi2c_clk {
    use crate::ral::{self, ccm::CCM};

    /// Returns the LPI2C clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CSCDR2, LPI2C_CLK_PODF) + 1
    }

    /// The smallest LPI2C clock divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest LPI2C clock divider.
    pub const MAX_DIVIDER: u32 = 64;

    /// Set the LPI2C clock divider.
    ///
    /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CSCDR2, LPI2C_CLK_PODF: podf);
    }

    /// LPI2C clock selections.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Selection {
        /// Derive from PLL3 divided by 8.
        Pll3Div8 = 0,
        /// Derive from the crystal oscillator.
        Oscillator = 1,
    }

    /// Returns the LPI2C clock selection.
    #[inline(always)]
    pub fn selection(ccm: &CCM) -> Selection {
        match ral::read_reg!(ral::ccm, ccm, CSCDR2, LPI2C_CLK_SEL) {
            0 => Selection::Pll3Div8,
            1 => Selection::Oscillator,
            _ => unreachable!(),
        }
    }

    /// Set the LPI2C clock selection.
    #[inline(always)]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CSCDR2, LPI2C_CLK_SEL: selection as u32);
    }
}

/// LPSPI clock root.
///
/// `lpspi_clk` provides the clock source for all LPSPI peripherals.
/// You must disable LPSPI clock gates before selecting the clock
/// and divider.
///
/// # Example
///
/// ```no_run
/// use imxrt_hal as hal;
/// use hal::ccm::{lpspi_clk, clock_gate};
/// use hal::ccm::analog::pll2;
///
/// use imxrt_ral as ral;
///
/// const LPSPI_CLK_DIVIDER: u32 = 8;
/// const LPSPI_CLK_HZ: u32 = pll2::FREQUENCY / LPSPI_CLK_DIVIDER;
///
/// # fn opt() -> Option<()> {
/// let mut ccm = unsafe { ral::ccm::CCM::instance() };
/// clock_gate::lpspi::<2>().set(&mut ccm, clock_gate::OFF);
/// lpspi_clk::set_selection(&mut ccm, lpspi_clk::Selection::Pll2);
/// lpspi_clk::set_divider(&mut ccm, LPSPI_CLK_DIVIDER);
///
/// clock_gate::lpspi::<2>().set(&mut ccm, clock_gate::ON);
/// # Some(()) }
/// ```
pub mod lpspi_clk {
    use crate::ral::{self, ccm::CCM};

    /// Returns the LPSPI clock divider.
    #[inline(always)]
    pub fn divider(ccm: &CCM) -> u32 {
        ral::read_reg!(ral::ccm, ccm, CBCMR, LPSPI_PODF) + 1
    }

    /// The smallest LPSPI clock divider.
    pub const MIN_DIVIDER: u32 = 1;
    /// The largest LPSPI clock divider.
    pub const MAX_DIVIDER: u32 = 8;

    /// Set the LPSPI clock divider.
    ///
    /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
    #[inline(always)]
    pub fn set_divider(ccm: &mut CCM, divider: u32) {
        // 1010 MCUs support an extra bit in this field, so this
        // could be a max of 16 for those chips.
        let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
        ral::modify_reg!(ral::ccm, ccm, CBCMR, LPSPI_PODF: podf);
    }

    /// LPSPI clock selections.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum Selection {
        /// Derive from PLL3_PFD1.
        Pll3Pfd1 = 0,
        /// Derive from the PLL3_PFD0.
        Pll3Pfd0 = 1,
        /// Derive from PLL2.
        Pll2 = 2,
        /// Derive from PLL2_PFD2.
        Pll2Pfd2 = 3,
    }

    /// Returns the LPSPI clock selection.
    #[inline(always)]
    pub fn selection(ccm: &CCM) -> Selection {
        match ral::read_reg!(ral::ccm, ccm, CBCMR, LPSPI_CLK_SEL) {
            0 => Selection::Pll3Pfd1,
            1 => Selection::Pll3Pfd0,
            2 => Selection::Pll2,
            3 => Selection::Pll2Pfd2,
            _ => unreachable!(),
        }
    }

    /// Set the LPSPI clock selection.
    #[inline(always)]
    pub fn set_selection(ccm: &mut CCM, selection: Selection) {
        ral::modify_reg!(ral::ccm, ccm, CBCMR, LPSPI_CLK_SEL: selection as u32);
    }
}

macro_rules! ccm_flexio {
    (
        $name:ident, $desc:literal,
        divider: ($divider_reg:ident, $divider_field:ident),
        predivider: ($predivider_reg:ident, $predivider_field:ident),
        selection: ($sel_reg:ident, $sel_field:ident)$(,)?
    ) => {
        #[doc = concat!($desc, " clock root.")]
        pub mod $name {
            use crate::ral::{self, ccm::CCM};

            #[doc = concat!("Returns the ", $desc, " clock divider.")]
            #[inline(always)]
            pub fn divider(ccm: &CCM) -> u32 {
                ral::read_reg!(ral::ccm, ccm, $divider_reg, $divider_field) + 1
            }

            #[doc = concat!("The smallest ", $desc, " clock divider.")]
            pub const MIN_DIVIDER: u32 = 1;
            #[doc = concat!("The largest ", $desc, " clock divider.")]
            pub const MAX_DIVIDER: u32 = 8;

            #[doc = concat!("Set the ", $desc, " clock divider.")]
            ///
            /// The implementation clamps `divider` between [`MIN_DIVIDER`] and [`MAX_DIVIDER`].
            #[inline(always)]
            pub fn set_divider(ccm: &mut CCM, divider: u32) {
                // 1010 MCUs support an extra bit in this field, so this
                // could be a max of 16 for those chips.
                let podf = divider.clamp(MIN_DIVIDER, MAX_DIVIDER) - 1;
                ral::modify_reg!(ral::ccm, ccm, $divider_reg, $divider_field: podf);
            }

            #[doc = concat!("Returns the ", $desc, " clock predivider.")]
            #[inline(always)]
            pub fn predivider(ccm: &CCM) -> u32 {
                ral::read_reg!(ral::ccm, ccm, $predivider_reg, $predivider_field) + 1
            }

            #[doc = concat!("The smallest ", $desc, " clock predivider.")]
            pub const MIN_PREDIVIDER: u32 = 1;
            #[doc = concat!("The largest ", $desc, " clock predivider.")]
            pub const MAX_PREDIVIDER: u32 = 8;

            #[doc = concat!("Set the ", $desc, " clock predivider.")]
            ///
            /// The implementation clamps `predivider` between [`MIN_PREDIVIDER`] and [`MAX_PREDIVIDER`].
            #[inline(always)]
            pub fn set_predivider(ccm: &mut CCM, predivider: u32) {
                let podf = predivider.clamp(MIN_PREDIVIDER, MAX_PREDIVIDER) - 1;
                ral::modify_reg!(ral::ccm, ccm, $predivider_reg, $predivider_field: podf);
            }

            #[doc = concat!($desc, " clock selections.")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(u32)]
            pub enum Selection {
                /// Derive from PLL4.
                Pll4 = 0,
                /// Derive from PLL3_PFD2.
                Pll3Pfd2 = 1,

                #[cfg(any(feature = "imxrt1060", feature = "imxrt1064"))]
                /// Derive from PLL5.
                Pll5 = 2,
                #[cfg(feature = "imxrt1010")]
                /// Derive from PLL2.
                Pll2 = 2,

                //
                // '2' reserved on 1020.
                //

                /// Derive from pll3_sw_clk.
                Pll3SwClk = 3,
            }

            #[doc = concat!("Returns the ", $desc, " clock selections.")]
            #[inline(always)]
            pub fn selection(ccm: &CCM) -> Selection {
                match ral::read_reg!(ral::ccm, ccm, $sel_reg, $sel_field) {
                    0 => Selection::Pll4,
                    1 => Selection::Pll3Pfd2,
                    #[cfg(any(feature = "imxrt1060", feature = "imxrt1064"))]
                    2 => Selection::Pll5,
                    #[cfg(feature = "imxrt1010")]
                    2 => Selection::Pll2,
                    3 => Selection::Pll3SwClk,
                    _ => unreachable!(),
                }
            }

            #[doc = concat!("Set the ", $desc, " clock selections.")]
            #[inline(always)]
            pub fn set_selection(ccm: &mut CCM, selection: Selection) {
                ral::modify_reg!(ral::ccm, ccm, $sel_reg, $sel_field: selection as u32);
            }
        }
    };
}
