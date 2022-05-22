//! Functions for reading and manipulating clock gates.
//!
//! Use these functions to acquire [`Locator`]s to peripheral clock
//! gates. Once you have a locator, use `set` to change the gate, and
//! `get` to read the gate.
//!
//! Some locator-returning functions require a constant `N`, representing
//! the Nth peripheral (the '4' in GPIO4). An invalid `N` for a given
//! peripheral generates a compile-time error.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! use hal::ccm::clock_gate;
//!
//! # || -> Option<()> {
//! let mut ccm = ral::ccm::CCM::take()?;
//! // OK: GPT1 is valid.
//! let setting = clock_gate::gpt_serial::<1>().get(&ccm);
//! # Some(()) }();
//! ```
//! ```compile_fail
//! # use imxrt_hal as hal;
//! # use imxrt_ral as ral;
//! # use hal::ccm::clock_gate;
//! # || -> Option<()> {
//! # let mut ccm = ral::ccm::CCM::take()?;
//! // ERROR: GPT33 is NOT a real peripheral.
//! let setting = clock_gate::gpt_serial::<33>().get(&ccm);
//! # Some(()) }();
//! ```
//!
//! # Aggregate clock gates
//!
//! The module exposes collections of aggregate clock gates. It forms
//! collections based on the root clock. The ordering of clock gates
//! within these collections are unspecified.
//!
//! Additionally, the size of each collection may vary by chip. Since
//! the collections are `const`, their size is known at compile time.
//!
//! Use [`all()`] to manipulate all implemented clock gates.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! use hal::ccm::clock_gate;
//!
//! # || -> Option<()> {
//! let mut ccm = ral::ccm::CCM::take()?;
//!
//! // Turn on all clock gates downstream of the IPG clock.
//! clock_gate::IPG_CLOCK_GATES.iter().for_each(|clock_gate| {
//!     clock_gate.set(&mut ccm, clock_gate::ON);
//! });
//!
//! // Turn off every clock gate in the system. This reverts
//! // the IPG clock gate operation from above.
//! clock_gate::all().for_each(|clock_gate| {
//!     clock_gate.set(&mut ccm, clock_gate::OFF);
//! });
//! # Some(()) }();
//! ```

use crate::ral::{self, ccm::CCM};

/// A clock gate setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Setting {
    /// Clock is off in all modes.
    Off = 0,
    /// Clock is on in run mode, but off in WAIT and STOP modes.
    OnlyRun = 1,
    /// Clock is on in all modes, except stop mode.
    On = 3,
}

/// Helper constant to turn off clock gates.
pub const OFF: Setting = Setting::Off;
/// Helper constant to turn on clock gates.
pub const ON: Setting = Setting::On;

impl Setting {
    fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Setting::Off,
            1 => Setting::OnlyRun,
            3 => Setting::On,
            _ => unreachable!(), // Reserved value in two bit field.
        }
    }
}

/// Clock gating register.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
#[repr(u8)]
enum Register {
    CCGR0,
    CCGR1,
    CCGR2,
    CCGR3,
    CCGR4,
    CCGR5,
    CCGR6,
    CCGR7,
}

use Register::*;

/// Clock gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
#[repr(u8)]
enum Gate {
    CG0,
    CG1,
    CG2,
    CG3,
    CG4,
    CG5,
    CG6,
    CG7,
    CG8,
    CG9,
    CG10,
    CG11,
    CG12,
    CG13,
    CG14,
    CG15,
}

use Gate::*;

impl Gate {
    /// Compute the bitshift required to access this gate field
    /// in the register.
    const fn shift(self) -> usize {
        self as usize * 2
    }
}

/// A clock gate locator.
///
/// These are reachable through the various function
/// provided in this module.
#[derive(Debug, PartialEq, Eq)]
pub struct Locator {
    register: Register,
    gate: Gate,
}

macro_rules! locator {
    ($register:ident, $gate:ident) => {
        Locator {
            register: $register,
            gate: $gate,
        }
    };
}

/// Acquire the clock gate register for a clock gate locator.
fn clock_gate_register<'a>(ccm: &'a CCM, locator: &'_ Locator) -> &'a ral::RWRegister<u32> {
    match locator.register {
        Register::CCGR0 => &ccm.CCGR0,
        Register::CCGR1 => &ccm.CCGR1,
        Register::CCGR2 => &ccm.CCGR2,
        Register::CCGR3 => &ccm.CCGR3,
        Register::CCGR4 => &ccm.CCGR4,
        Register::CCGR5 => &ccm.CCGR5,
        Register::CCGR6 => &ccm.CCGR6,
        Register::CCGR7 => {
            // If we're compiling this code, it's because a chip family feature
            // is activated.
            cfg_if::cfg_if! {
                if #[cfg(any(feature = "imxrt1060", feature = "imxrt1064"))] {
                    // Only certain i.MX RT 10xx chips have CCGR7.
                    &ccm.CCGR7
                } else {
                    // The RAL's 'Valid' trait ensures that no clock gate locator
                    // with this clock gate register is reachable.
                    unreachable!()
                }
            }
        }
    }
}

impl Locator {
    /// Get the clock gate setting for the peripheral.
    pub fn get(&self, ccm: &CCM) -> Setting {
        let ccgr = clock_gate_register(ccm, self);
        let raw = (ccgr.read() >> self.gate.shift()) & 0b11;
        Setting::from_raw(raw)
    }

    /// Set the clock gate for this peripheral.
    pub fn set(&self, ccm: &mut CCM, setting: Setting) {
        let ccgr = clock_gate_register(ccm, self);
        let mut raw = ccgr.read();
        raw &= !(0b11 << self.gate.shift());
        raw |= (setting as u32) << self.gate.shift();
        ccgr.write(raw);
    }

    /// Executes `func` while this clock gate is off.
    ///
    /// When this method returns, the clock gate is configured back to its
    /// previous setting before the call.
    pub fn while_off<R>(&self, ccm: &mut CCM, func: impl FnOnce() -> R) -> R {
        while_off([self], ccm, func)
    }
}

/// Executes `func` while all clock gates are turned off.
///
/// When this function returns, the clock gates are configured back
/// to their states before this call.
pub fn while_off<R, const N: usize>(
    locators: [&Locator; N],
    ccm: &mut CCM,
    func: impl FnOnce() -> R,
) -> R {
    let mut settings = [Setting::Off; N];
    settings
        .iter_mut()
        .zip(locators)
        .for_each(|(setting, locator)| {
            *setting = locator.get(ccm);
            locator.set(ccm, Setting::Off);
        });

    let result = func();

    settings
        .iter()
        .zip(locators)
        .for_each(|(setting, locator)| {
            locator.set(ccm, *setting);
        });

    result
}

/// Returns the PIT clock gate locator.
#[inline(always)]
pub const fn pit() -> &'static Locator {
    // TODO(mciantyre) there's multiple PITs on the 1170 chips...
    &locator!(CCGR1, CG6)
}

/// Returns the GPT serial clock gate locator.
#[inline(always)]
pub const fn gpt_serial<const N: u8>() -> &'static Locator
where
    ral::gpt::Instance<N>: ral::Valid,
{
    &[locator!(CCGR1, CG11), locator!(CCGR0, CG13)][N as usize - 1]
}

/// Returns the GPT serial clock gate locator.
#[inline(always)]
pub const fn gpt_bus<const N: u8>() -> &'static Locator
where
    ral::gpt::Instance<N>: ral::Valid,
{
    &[locator!(CCGR1, CG10), locator!(CCGR0, CG12)][N as usize - 1]
}

/// Returns the GPIO clock gate locator.
///
/// Note that fast GPIOs do not have a clock gate,
/// so `N` should not describe a fast GPIO port.
///
/// # Panics
///
/// Panics if `N` describes a fast GPIO port.
#[inline(always)]
pub const fn gpio<const N: u8>() -> &'static Locator
where
    ral::gpio::Instance<N>: ral::Valid,
{
    &[
        locator!(CCGR1, CG13),
        locator!(CCGR0, CG15),
        locator!(CCGR2, CG13),
        locator!(CCGR3, CG6),
        locator!(CCGR1, CG15),
    ][N as usize - 1]
}

/// Returns the LPUART clock gate locator.
#[inline(always)]
pub const fn lpuart<const N: u8>() -> &'static Locator
where
    ral::lpuart::Instance<N>: ral::Valid,
{
    &[
        locator!(CCGR5, CG12),
        locator!(CCGR0, CG14),
        locator!(CCGR0, CG6),
        locator!(CCGR1, CG12),
        locator!(CCGR3, CG1),
        locator!(CCGR3, CG3),
        locator!(CCGR5, CG13),
        locator!(CCGR6, CG7),
    ][N as usize - 1]
}

/// Returns the DMA clock gate locator.
#[inline(always)]
pub const fn dma() -> &'static Locator {
    &locator!(CCGR5, CG3)
}

/// Returns the LPI2C clock gate locator.
#[inline(always)]
pub const fn lpi2c<const N: u8>() -> &'static Locator
where
    ral::lpi2c::Instance<N>: ral::Valid,
{
    &[
        locator!(CCGR2, CG3),
        locator!(CCGR2, CG4),
        locator!(CCGR2, CG5),
        locator!(CCGR6, CG12),
    ][N as usize - 1]
}

/// Returns the LPSPI clock gate locator.
#[inline(always)]
pub const fn lpspi<const N: u8>() -> &'static Locator
where
    ral::lpspi::Instance<N>: ral::Valid,
{
    &[
        locator!(CCGR1, CG0),
        locator!(CCGR1, CG1),
        locator!(CCGR1, CG2),
        locator!(CCGR1, CG3),
    ][N as usize - 1]
}

/// Returns the FlexPWM clock gate locator.
#[inline(always)]
pub const fn flexpwm<const N: u8>() -> &'static Locator
where
    ral::pwm::Instance<N>: ral::Valid,
{
    const _: () = assert!(ral::SOLE_INSTANCE == 0u8);
    &[
        locator!(CCGR4, CG8),
        locator!(CCGR4, CG9),
        locator!(CCGR4, CG10),
        locator!(CCGR4, CG11),
    ][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
    // because 1010 has only one PWM instance, and that's
    // represented by ral::SOLE_INSTANCE (the number 0).
    // Consider this as the precedent approach when adding
    // support for 1170 (multiple PIT, DMA peripherals w.r.t
    // the 10xx families).
}

/// Returns the TRNG clock gate locator.
#[inline(always)]
pub const fn trng() -> &'static Locator {
    &locator!(CCGR6, CG6)
}

/// Returns the SNVS clock gate locator.
#[inline(always)]
pub const fn snvs() -> &'static Locator {
    &locator!(CCGR5, CG15)
}

pub use crate::ccm::root_clock_gates::*;

/// Produces an iterator for all implemented clock gate locators.
///
/// Clock gate locator ordering is unspecified. All locators are
/// unique. The number of locators may vary by chip family.
pub fn all() -> impl Iterator<Item = &'static Locator> {
    PERCLK_CLOCK_GATES
        .iter()
        .chain(UART_CLOCK_GATES)
        .chain(LPSPI_CLOCK_GATES)
        .chain(LPI2C_CLOCK_GATES)
        .chain(IPG_CLOCK_GATES)
        .cloned() // &&Locator -> &Locator
}
