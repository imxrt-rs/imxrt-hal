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
//! let mut ccm = unsafe { ral::ccm::CCM::instance() };
//! // OK: GPT1 is valid.
//! let setting = clock_gate::gpt_serial::<1>().get(&ccm);
//! # Some(()) }();
//! ```
//! ```compile_fail
//! # use imxrt_hal as hal;
//! # use imxrt_ral as ral;
//! # use hal::ccm::clock_gate;
//! # || -> Option<()> {
//! # let mut ccm = unsafe { ral::ccm::CCM::instance() };
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
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! use hal::ccm::clock_gate;
//!
//! # || -> Option<()> {
//! let mut ccm = unsafe { ral::ccm::CCM::instance() };
//!
//! // Turn on all clock gates downstream of the IPG clock.
//! clock_gate::IPG_CLOCK_GATES.iter().for_each(|clock_gate| {
//!     clock_gate.set(&mut ccm, clock_gate::ON);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    #[inline(always)]
    const fn shift(self) -> usize {
        self as usize * 2
    }
}

/// A clock gate locator.
///
/// These are reachable through the various function
/// provided in this module.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Locator {
    register: Register,
    gate: Gate,
}

const fn locator(register: Register, gate: Gate) -> Locator {
    Locator { register, gate }
}

/// Acquire the clock gate register for a clock gate locator.
#[inline(always)]
fn clock_gate_register(ccm: &CCM, register: Register) -> &ral::RWRegister<u32> {
    match register {
        Register::CCGR0 => &ccm.CCGR0,
        Register::CCGR1 => &ccm.CCGR1,
        Register::CCGR2 => &ccm.CCGR2,
        Register::CCGR3 => &ccm.CCGR3,
        Register::CCGR4 => &ccm.CCGR4,
        Register::CCGR5 => &ccm.CCGR5,
        Register::CCGR6 => &ccm.CCGR6,
        Register::CCGR7 => {
            // If we're compiling this code, it's because a chip feature
            // is activated.
            cfg_if::cfg_if! {
                if #[cfg(any(chip = "imxrt1060", chip = "imxrt1064"))] {
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
        let ccgr = clock_gate_register(ccm, self.register);
        let raw = (ccgr.read() >> self.gate.shift()) & 0b11;
        Setting::from_raw(raw)
    }

    /// Set the clock gate for this peripheral.
    pub fn set(&self, ccm: &mut CCM, setting: Setting) {
        let ccgr = clock_gate_register(ccm, self.register);
        let mut raw = ccgr.read();
        raw &= !(0b11 << self.gate.shift());
        raw |= (setting as u32) << self.gate.shift();
        ccgr.write(raw);
    }
}

/// Returns the PIT clock gate locator.
#[inline(always)]
pub const fn pit() -> Locator {
    locator(CCGR1, CG6)
}

/// Returns the GPT serial clock gate locator.
#[inline(always)]
pub const fn gpt_serial<const N: u8>() -> Locator
where
    ral::gpt::Instance<N>: ral::Valid,
{
    [locator(CCGR1, CG11), locator(CCGR0, CG13)][N as usize - 1]
}

/// Returns the GPT serial clock gate locator.
#[inline(always)]
pub const fn gpt_bus<const N: u8>() -> Locator
where
    ral::gpt::Instance<N>: ral::Valid,
{
    [locator(CCGR1, CG10), locator(CCGR0, CG12)][N as usize - 1]
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
pub const fn gpio<const N: u8>() -> Locator
where
    ral::gpio::Instance<N>: ral::Valid,
{
    [
        locator(CCGR1, CG13),
        locator(CCGR0, CG15),
        locator(CCGR2, CG13),
        locator(CCGR3, CG6),
        locator(CCGR1, CG15),
    ][N as usize - 1]
}

/// Returns the LPUART clock gate locator.
#[inline(always)]
pub const fn lpuart<const N: u8>() -> Locator
where
    ral::lpuart::Instance<N>: ral::Valid,
{
    [
        locator(CCGR5, CG12),
        locator(CCGR0, CG14),
        locator(CCGR0, CG6),
        locator(CCGR1, CG12),
        locator(CCGR3, CG1),
        locator(CCGR3, CG3),
        locator(CCGR5, CG13),
        locator(CCGR6, CG7),
    ][N as usize - 1]
}

/// Returns the DMA clock gate locator.
#[inline(always)]
pub const fn dma() -> Locator {
    locator(CCGR5, CG3)
}

/// Returns the LPI2C clock gate locator.
#[inline(always)]
pub const fn lpi2c<const N: u8>() -> Locator
where
    ral::lpi2c::Instance<N>: ral::Valid,
{
    [
        locator(CCGR2, CG3),
        locator(CCGR2, CG4),
        locator(CCGR2, CG5),
        locator(CCGR6, CG12),
    ][N as usize - 1]
}

/// Returns the LPSPI clock gate locator.
#[inline(always)]
pub const fn lpspi<const N: u8>() -> Locator
where
    ral::lpspi::Instance<N>: ral::Valid,
{
    [
        locator(CCGR1, CG0),
        locator(CCGR1, CG1),
        locator(CCGR1, CG2),
        locator(CCGR1, CG3),
    ][N as usize - 1]
}

#[allow(clippy::assertions_on_constants)]
const _: () = assert!(ral::SOLE_INSTANCE == 0u8);

/// Returns the FlexIO clock gate locator.
#[inline(always)]
pub const fn flexio<const N: u8>() -> Locator
where
    ral::flexio::Instance<N>: ral::Valid,
{
    [
        locator(CCGR5, CG1),
        locator(CCGR3, CG0),
        locator(CCGR7, CG6),
    ][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
}

/// Returns the FlexPWM clock gate locator.
#[inline(always)]
pub const fn flexpwm<const N: u8>() -> Locator
where
    ral::pwm::Instance<N>: ral::Valid,
{
    [
        locator(CCGR4, CG8),
        locator(CCGR4, CG9),
        locator(CCGR4, CG10),
        locator(CCGR4, CG11),
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
pub const fn trng() -> Locator {
    locator(CCGR6, CG6)
}

/// Returns the SNVS LP clock gate locator.
#[inline(always)]
pub const fn snvs_lp() -> Locator {
    locator(CCGR5, CG15)
}

/// Returns the SNVS HP clock gate locator.
#[inline(always)]
pub const fn snvs_hp() -> Locator {
    locator(CCGR5, CG14)
}

/// Returns the USB clock gate locator.
#[inline(always)]
pub const fn usb() -> Locator {
    locator(CCGR6, CG0)
}

/// Returns the ADC clock gate locators.
#[inline(always)]
pub const fn adc<const N: u8>() -> Locator
where
    ral::adc::Instance<N>: ral::Valid,
{
    [locator(CCGR1, CG8), locator(CCGR1, CG4)][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
}

/// Returns the SAI clock gate locators.
#[inline(always)]
pub const fn sai<const N: u8>() -> Locator
where
    ral::sai::Instance<N>: ral::Valid,
{
    [
        locator(CCGR5, CG9),
        locator(CCGR5, CG10),
        locator(CCGR5, CG11),
    ][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
}

pub use crate::chip::config::ccm::clock_gate::*;
