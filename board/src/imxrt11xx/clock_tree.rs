//! CCM clock tree for 11xx MCUs.
//!
//! Use `configure` to simply configure the clock tree for a given
//! run mode. After `configure`, the system clocks run at the frequencies
//! described by each `*_frequency` function. The frequencies for a given
//! run mode are less than or equal to the maximum allowed for the given
//! run mode. Consult your MCU's reference manual for more information.
//!
//! Use `*_frequency` functions to understand the target system clock frequencies.
//! Note that these functions are `const`, and should be usable in constant
//! contexts.
//!
//! See the 10xx documentation for an example.

use crate::{
    hal::ccm::XTAL_OSCILLATOR_HZ,
    ral::{self, ccm::CCM},
    RunMode,
};

/// A clock source.
///
/// Used to represent possible clock sources for clock roots.
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(unused)] // TODO remove once there's more development here.
enum ClockSource {
    /// ARM_PLL, for the Cortex M7.
    ArmCm7,
    /// SYS_PLL1, with dedicated 1GHz frequency
    /// for ETH.
    SysPll1,
    /// SYS_PLL2, main PLL (no PFDs).
    SysPll2,
    /// SYS_PLL3, main PLL (no PDFs).
    SysPll3,
    /// 24MHz XTAL.
    ///
    /// Primary source for all PLLs.
    XtalOsc24MHz,
    /// 400 MHz RCOSC.
    ///
    /// Power source before any other PLLs are enabled, or while
    /// in low power mode.
    RcOsc400MHz,
}

impl ClockSource {
    /// Returns the frequency (Hz) of the PLL for a given run mode.
    const fn frequency(self, run_mode: RunMode) -> u32 {
        match (self, run_mode) {
            (ClockSource::ArmCm7, RunMode::Overdrive) => 1_000_000_000, // Brr...
            (ClockSource::SysPll1, _) => 1_000_000_000,
            (ClockSource::SysPll2, _) => 528_000_000,
            (ClockSource::SysPll3, _) => 480_000_000,
            (ClockSource::XtalOsc24MHz, _) => XTAL_OSCILLATOR_HZ,
            (ClockSource::RcOsc400MHz, _) => 400_000_000,
        }
    }
}

/// A clock selection.
///
/// Pairs a clock source with a MUX value for a particular clock root.
/// It includes a non-zero divider, so that a selection can fully describe
/// the frequency of a clock root.
#[derive(Clone, Copy)]
struct Selection {
    mux: u32,
    source: ClockSource,
    divider: u32,
}

impl Selection {
    const fn frequency(self, run_mode: RunMode) -> u32 {
        self.source.frequency(run_mode) / self.divider
    }
}

const fn bus_selection(run_mode: RunMode) -> Selection {
    match run_mode {
        RunMode::Overdrive => Selection {
            mux: 0b010,
            source: ClockSource::RcOsc400MHz,
            divider: 2,
        },
    }
}

/// Returns the target bus clock frequency for the run mode.
pub const fn bus_frequency(run_mode: RunMode) -> u32 {
    bus_selection(run_mode).frequency(run_mode)
}

const _: () = assert!(bus_frequency(RunMode::Overdrive) == 200_000_000); // 240MHz max.

#[inline(always)]
fn configure_clock_root(offset: usize, selection: &Selection, ccm: &mut CCM) {
    let clock_root = &ccm.CLOCK_ROOT[offset];
    ral::modify_reg!(ral::ccm::clockroot, clock_root, CLOCK_ROOT_CONTROL,
        DIV: selection.divider - 1,
        MUX: selection.mux);
    while ral::read_reg!(
        ral::ccm::clockroot,
        clock_root,
        CLOCK_ROOT_STATUS0,
        CHANGING == 1
    ) {}
}

/// Set the bus clock (IPG) configuration for the Cortex M7.
///
/// When this call returns, the bus clock frequency matches the value returned
/// by [`bus_frequency()`].
///
/// This function may disable clock gates for various peripherals. It may leave
/// these clock gates disabled.
pub fn configure_bus(run_mode: RunMode, ccm: &mut CCM) {
    configure_clock_root(2, &bus_selection(run_mode), ccm);
}

const fn gpt_selection<const N: u8>(run_mode: RunMode) -> Selection {
    match run_mode {
        // Same for all N.
        RunMode::Overdrive => Selection {
            mux: 0b010,
            source: ClockSource::RcOsc400MHz,
            divider: 4,
        },
    }
}

/// Returns the target GPTn clock frequency for the run mode.
pub const fn gpt_frequency<const N: u8>(run_mode: RunMode) -> u32
where
    ral::gpt::Instance<N>: ral::Valid,
{
    gpt_selection::<N>(run_mode).frequency(run_mode)
}

const _: () = assert!(gpt_frequency::<1>(RunMode::Overdrive) == 100_000_000); // 240MHz max.

/// Set the GPTn clock configuration.
///
/// When this call returns, the GPTn clock frequency matches the value
/// returned by [`gpt_frequency`].
///
/// This function may disable clock gates for various peripherals. It may leave
/// these clock gates disabled.
pub fn configure_gpt<const N: u8>(run_mode: RunMode, ccm: &mut CCM)
where
    ral::gpt::Instance<N>: ral::Valid,
{
    // GPT 1 at CLOCK_ROOT14: 14 - 1 = 13
    // GPT 6 at CLOCK_ROOT19: 19 - 6 = 13
    configure_clock_root(N as usize + 13, &gpt_selection::<N>(run_mode), ccm);
}

const fn lpuart_selection<const N: u8>(run_mode: RunMode) -> Selection {
    match run_mode {
        // Same for all N.
        RunMode::Overdrive => Selection {
            mux: 0b010,
            source: ClockSource::RcOsc400MHz,
            divider: 5,
        },
    }
}

/// Returns the target LPUARTn clock frequency for the run mode.
pub const fn lpuart_frequency<const N: u8>(run_mode: RunMode) -> u32
where
    ral::lpuart::Instance<N>: ral::Valid,
{
    lpuart_selection::<N>(run_mode).frequency(run_mode)
}

const _: () = assert!(lpuart_frequency::<1>(RunMode::Overdrive) == 80_000_000); // Max allowed.

/// Set the LPUARTn clock configuration.
///
/// When this call returns, the LPUARTn clock frequency matches the value
/// returned by [`lpuart_frequency`].
///
/// This function may disable clock gates for various peripherals. It may leave
/// these clock gates disabled.
pub fn configure_lpuart<const N: u8>(run_mode: RunMode, ccm: &mut CCM)
where
    ral::lpuart::Instance<N>: ral::Valid,
{
    // LPUART1 -> CLOCK_ROOT25
    // LPUART12 -> CLOCK_ROOT36
    configure_clock_root(N as usize + 24, &lpuart_selection::<N>(run_mode), ccm);
}

const fn lpi2c_selection<const N: u8>(run_mode: RunMode) -> Selection {
    match run_mode {
        RunMode::Overdrive => Selection {
            mux: 0b001,
            source: ClockSource::XtalOsc24MHz,
            divider: 3,
        },
    }
}

/// Returns the target LPI2Cn clock frequency for the run mode.
pub const fn lpi2c_frequency<const N: u8>(run_mode: RunMode) -> u32
where
    ral::lpi2c::Instance<N>: ral::Valid,
{
    lpi2c_selection::<N>(run_mode).frequency(run_mode)
}

const _: () = assert!(lpi2c_frequency::<1>(RunMode::Overdrive) == 8_000_000); // Max is 66MHz.

/// Set the LPI2Cn clock configuration.
///
/// When this call returns, the LPI2Cn clock frequency matches the value
/// returned by [`lpi2c_frequency`].
///
/// This function may disable clock gates for various peripherals. It may leave
/// these clock gates disabled.
pub fn configure_lpi2c<const N: u8>(run_mode: RunMode, ccm: &mut CCM)
where
    ral::lpi2c::Instance<N>: ral::Valid,
{
    // LPI2C1 -> CLOCK_ROOT37
    // LPI2C6 -> CLOCK_ROOT42
    configure_clock_root(N as usize + 36, &lpi2c_selection::<N>(run_mode), ccm);
}

const fn lpspi_selection<const N: u8>(run_mode: RunMode) -> Selection {
    match run_mode {
        RunMode::Overdrive => Selection {
            mux: 0b010,
            source: ClockSource::RcOsc400MHz,
            divider: 4,
        },
    }
}

/// Returns the target LPSPIn clock frequency for the run mode.
pub const fn lpspi_frequency<const N: u8>(run_mode: RunMode) -> u32
where
    ral::lpspi::Instance<N>: ral::Valid,
{
    lpspi_selection::<N>(run_mode).frequency(run_mode)
}

const _: () = assert!(lpspi_frequency::<1>(RunMode::Overdrive) == 100_000_000); // Max is 135MHz.

/// Set the LPSPICn clock configuration.
///
/// When this call returns, the LPSPICn clock frequency matches the value
/// returned by [`lpspi_frequency`].
///
/// This function may disable clock gates for various peripherals. It may leave
/// these clock gates disabled.
pub fn configure_lpspi<const N: u8>(run_mode: RunMode, ccm: &mut CCM)
where
    ral::lpspi::Instance<N>: ral::Valid,
{
    // LPSPI1 -> CLOCK_ROOT43
    // LPSPI6 -> CLOCK_ROOT48
    configure_clock_root(N as usize + 42, &lpspi_selection::<N>(run_mode), ccm);
}
