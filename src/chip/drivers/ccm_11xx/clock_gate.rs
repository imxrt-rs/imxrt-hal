//! Functions for reading and manipulating clock gates.
//!
//! This module exposes a similar API as the `clock_gate` API
//! for the 10xx MCUs. Consult that module's documentation for
//! more information.

use crate::ral::{self, ccm::CCM};

/// A clock gate setting.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Setting {
    /// Clock is off.
    Off = 0,
    /// Clock is on.
    On = 1,
}

/// Helper constant to turn off clock gates.
pub const OFF: Setting = Setting::Off;
/// Helper constant to turn on clock gates.
pub const ON: Setting = Setting::On;

impl Setting {
    fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Setting::Off,
            1 => Setting::On,
            _ => unreachable!(), // Only one bit wide.
        }
    }
}

/// A clock gate locator.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Locator {
    offset: usize,
}

impl Locator {
    const fn new(offset: usize) -> Self {
        Self { offset }
    }

    /// Get the clock gate setting for the peripheral.
    pub fn get(&self, ccm: &CCM) -> Setting {
        let lpcg_direct = lpcg_direct_register(ccm, *self);
        let raw = lpcg_direct.read() & 0b1;
        Setting::from_raw(raw)
    }

    /// Set the clock gate for this peripheral.
    pub fn set(&self, ccm: &mut CCM, setting: Setting) {
        let lpcg_direct = lpcg_direct_register(ccm, *self);
        // Performing a RMW just to keep all other reserved fields intact. Who
        // knows what could be there.
        let mut raw = lpcg_direct.read();
        raw &= !0b1;
        raw |= setting as u32;
        lpcg_direct.write(raw);
    }
}

/// Acquire the LPCGx_DIRECT register for the locator.
///
/// Verbose so that we can easily see the type casting.
#[inline(always)]
fn lpcg_direct_register(ccm: &CCM, locator: Locator) -> &ral::RWRegister<u32> {
    use core::ops::Deref;
    let ccm: *const ral::ccm::RegisterBlock = ccm.deref();
    let ccm: *const u8 = ccm.cast();
    // Safety: formula for offset follows reference manual, so it's producing a pointer
    // to a valid LPCG_DIRECT register. We control all locator values. Produced pointer
    // is in range of allocated object.
    let lpcg_direct: *const u8 = unsafe { ccm.add(0x6000 + locator.offset * 0x20) };
    let lpcg_direct: *const ral::RWRegister<u32> = lpcg_direct.cast();
    // Safety: pointer is valid given the above discussion. RWRegister is transparent.
    unsafe { &*lpcg_direct }
}

/// Returns the PIT clock gate locator.
#[inline(always)]
pub const fn pit<const N: u8>() -> Locator
where
    ral::pit::Instance<N>: ral::Valid,
{
    [Locator::new(62), Locator::new(63)][N as usize - 1]
}

/// Returns the GPIO clock gate locator.
///
/// Note that all GPIOs are controlled by the same gate.
#[inline(always)]
pub const fn gpio() -> Locator {
    Locator::new(51)
}

/// Returns the Cortex M7 DMA clock gate locator.
///
/// Note that this is the locator for the Cortex
/// M7's DMA engine. Use [`dma_lpsr`] for the Cortex
/// M4's DMA engine.
#[inline(always)]
pub const fn dma() -> Locator {
    Locator::new(22)
}

/// Returns the Cortex M4 DMA clock gate locator.
///
/// Note that this is the locator for the Cortex
/// M4's DMA engine. Use [`dma`] for the Cortex
/// M7's DMA engine.
#[inline(always)]
pub const fn dma_lpsr() -> Locator {
    Locator::new(23)
}

/// Returns the GPT clock gate locator.
#[inline(always)]
pub const fn gpt<const N: u8>() -> Locator
where
    ral::gpt::Instance<N>: ral::Valid,
{
    [
        Locator::new(64),
        Locator::new(65),
        Locator::new(66),
        Locator::new(67),
        Locator::new(68),
        Locator::new(69),
    ][N as usize - 1]
}

/// Returns the USB clock gate locator.
///
/// There's only one clock gate for all USBs.
#[inline(always)]
pub const fn usb() -> Locator {
    Locator::new(115)
}

/// Returns the LPUART clock gate locator.
#[inline(always)]
pub const fn lpuart<const N: u8>() -> Locator
where
    ral::lpuart::Instance<N>: ral::Valid,
{
    // LPUART1 -> LPCG86
    // LPUART12 -> LPCG97
    Locator::new(N as usize + 85)
}

/// Returns the LPSPI clock gate locator.
#[inline(always)]
pub const fn lpspi<const N: u8>() -> Locator
where
    ral::lpspi::Instance<N>: ral::Valid,
{
    // LPSPI1 -> LPCG104
    // LPSPI6 -> LPCG109
    Locator::new(N as usize + 103)
}

/// Returns the LPI2C clock gate locator.
#[inline(always)]
pub const fn lpi2c<const N: u8>() -> Locator
where
    ral::lpi2c::Instance<N>: ral::Valid,
{
    // LPI2C1 -> LPCG98
    // LPI2C6 -> LPCG103
    Locator::new(N as usize + 97)
}

/// Returns the FlexPWM clock gate locator.
#[inline(always)]
pub const fn flexpwm<const N: u8>() -> Locator
where
    ral::pwm::Instance<N>: ral::Valid,
{
    // FlexPWM1 -> LPCG79
    // FlexPWM4 -> LPCG82
    Locator::new(N as usize + 78)
}

/// Returns the SVNS clock gate locator.
#[inline(always)]
pub const fn snvs() -> Locator {
    Locator::new(38)
}

/// Returns the FlexIO clock gate locator.
#[inline]
pub const fn flexio<const N: u8>() -> Locator
where
    ral::pwm::Instance<N>: ral::Valid,
{
    // FlexIO1 -> LPCG53
    // FlexIO2 -> LPCG54
    Locator::new(N as usize + 52)
}
