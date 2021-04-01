//! Secure Real-Time Clock
//!
//! Basic support only: Supports enabling and setting the clock, but not any of its more
//! advanced features (alarm, calibration, stop on security violation, etc).
//!
//! The Secure Real-Time Clock continues tracking time until it is specifically disabled or loses
//! power, even through a system reboot (and potentially a loss of power to the system, if the
//! SRTC is connected to a battery).
//!
//! # Example
//!
//! ```no_run
//! use imxrt_hal;
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! let mut srtc = peripherals.srtc.enable_and_set(&mut peripherals.ccm.handle, 1600000000, 0);
//! // Interpreted as Unix time: Sep 13 2020 12:26:40.000
//!
//! let now = srtc.get();
//! ```
//!
//! # Teensy 4.x Note
//!
//! When the SRTC is enabled, setting the board into program mode then using the Teensy Loader
//! application (GUI) to reboot it will set the current time (Unix epoch, but time in local
//! timezone). This will overwrite whatever time you may have previously set and is ambiguous
//! around the backwards daylight savings transition point.

use core::fmt;

use crate::ccm;
use crate::ral;
use crate::ral::snvs::Instance;

const MR_SHIFT: u8 = 17;
const LR_SHIFT: u8 = 15;

/// The SRTC, disabled.
///
/// Note: The SRTC may actually be enabled, as (unless disabled) it stays enabled across reboots
/// as long as it has power.
pub struct Unclocked {
    reg: Instance,
}

impl Unclocked {
    pub(crate) fn new(snvs: Instance) -> Self {
        Unclocked { reg: snvs }
    }

    /// Enable the Secure Real-Time Clock and set it to the provided time. The time provided is
    /// [a count of seconds since some epoch](https://en.wikipedia.org/wiki/Epoch_(computing))
    /// and the sub-second 32768Hz ticks.
    ///
    /// Usually that epoch is the Unix epoch, but as an example setting `0` would create a simple
    /// 'seconds since SRTC enabled' clock.
    pub fn enable_and_set(mut self, ccm: &mut ccm::Handle, time: u32, ticks: u16) -> SRTC {
        // a whole disable-set-enable cycle appears to take ~900us
        clock(ccm);
        disable(&mut self.reg);
        set(&mut self.reg, time, ticks);
        self.enable(ccm)
    }

    /// Enable the Secure Real-Time Clock without setting the time.
    /// The SRTC keeps track of time as long as it is enabled and has power, so this function will
    /// avoid overwriting the old time.
    ///
    /// If no time had previously been set, the RTC will start counting from `0`.
    pub fn enable(mut self, ccm: &mut ccm::Handle) -> SRTC {
        clock(ccm);
        enable(&mut self.reg);
        SRTC { reg: self.reg }
    }

    /// Enable the SRTC.
    ///
    /// If the SRTC isn't already running, `try_enable` uses `seconds` and `ticks` as the starting
    /// count for the RTC.
    /// If the SRTC is currently running, the return indicates the current RTC time.
    pub fn try_enable(mut self, ccm: &mut ccm::Handle, seconds: u32, ticks: u16) -> EnabledState {
        clock(ccm);
        if is_enabled(&self.reg) {
            let (seconds, ticks) = get(&self.reg);
            EnabledState::AlreadyCounting {
                srtc: SRTC { reg: self.reg },
                seconds,
                ticks,
            }
        } else {
            set(&mut self.reg, seconds, ticks);
            enable(&mut self.reg);
            EnabledState::SetTime(SRTC { reg: self.reg })
        }
    }
}

/// Indicates the result of the `try_enable` method
#[derive(Debug)]
pub enum EnabledState {
    /// The SRTC was already enabled, and it's currently counting from `seconds`
    AlreadyCounting {
        srtc: SRTC,
        /// The current whole-second time on the SRTC
        seconds: u32,
        /// The current 32768Hz ticks of the SRTC
        ticks: u16,
    },
    /// The SRTC was not previously enabled, and it's now counting from the
    /// `seconds` and `ticks` supplied to `try_enable`
    SetTime(SRTC),
}

/// Returns whether the SRTC is enabled.
#[inline(always)]
fn is_enabled(snvs: &Instance) -> bool {
    ral::read_reg!(ral::snvs, snvs, LPCR, SRTC_ENV) != 0
}

/// Number of quarter-nanoseconds per tick. Rounded up from 122,070.3125 to avoid overflowing
/// the 15-bit result of `micros_to_ticks`.
const QUARTER_NANOS_PER_TICK: u32 = 122_071;

/// Converts the number of microseconds that have occurred since a second into clock ticks
/// (1/32768 of a second).
///
/// For example: for the time `2020-10-05 01:39:56.505`, `micros` is `505000`, and this gives
/// `16547` ticks.
///
/// The maximum valid value for `micros` is `999_999`.
pub fn micros_to_ticks(micros: u32) -> u16 {
    let quarter_nanos = micros * 4000;
    (quarter_nanos / QUARTER_NANOS_PER_TICK) as u16
}

// it is normal that these won't round trip

/// Converts sub-second clock ticks (1/32768 of a second) into microseconds.
///
/// For example: 32000 ticks works out to 976568 microseconds.
fn ticks_to_micros(ticks: u16) -> u32 {
    ticks as u32 * QUARTER_NANOS_PER_TICK / 4000
}

/// Enable the SNVS_LP clock (enabled by default)
fn clock(ccm: &mut ccm::Handle) {
    let (ccm, _) = ccm.raw();
    ral::modify_reg!(ral::ccm, ccm, CCGR5, CG15: 0b11);
}

/// Disable the SRTC.
fn disable(snvs: &mut Instance) {
    // If the SRTC is locked this function will loop forever.
    // SRTC locking is not implemented, so if it's locked then the user did it manually.
    ral::modify_reg!(ral::snvs, snvs, LPCR, SRTC_ENV: SRTC_ENV_0); // disable SRTC
    while is_enabled(snvs) {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    } // wait until SRTC turns off
}

/// Sets the SRTC time.
fn set(snvs: &mut Instance, time: u32, ticks: u16) {
    let low_time = (time << LR_SHIFT) | (ticks as u32);
    ral::modify_reg!(ral::snvs, snvs, LPSRTCMR, SRTC: time >> MR_SHIFT);
    ral::write_reg!(ral::snvs, snvs, LPSRTCLR, low_time);
    // The lowest 15 bits of MR are the MSB, upper are reserved. §20.6.6
    // The upper 17 bits of LR are the LSB down to seconds.
    // The lower 15 bits of LR are the sub-second ticks of the 32768Hz clock.
    // (A 47-bit counter of a 32768Hz clock.)
}

/// Enable the SRTC.
fn enable(snvs: &mut Instance) {
    ral::modify_reg!(ral::snvs, snvs, LPCR, SRTC_ENV: SRTC_ENV_1); // enable SRTC
    while !is_enabled(snvs) {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    } // wait until SRTC turns on
}

/// Get the current time from the SRTC as `(seconds, ticks)`.
fn get(snvs: &Instance) -> (u32, u16) {
    let mut msb = 0;
    let mut lsb = 0;
    for _ in 0..6 {
        // reference manual says this should take no more than 3 sessions of 2 reads; do 6
        let msb2 = ral::read_reg!(ral::snvs, snvs, LPSRTCMR, SRTC);
        let lsb2 = ral::read_reg!(ral::snvs, snvs, LPSRTCLR);
        if msb == msb2 && lsb == lsb2 {
            break;
        }
        msb = msb2;
        lsb = lsb2;
    }
    let seconds = (msb << MR_SHIFT) | (lsb >> LR_SHIFT);
    let ticks = (lsb & 0x7FFF) as u16;
    (seconds, ticks)
}

/// The Secure Real-Time Clock, enabled.
pub struct SRTC {
    reg: Instance,
}

impl SRTC {
    /// Get the current time as a count of seconds since some point in the past.
    pub fn get(&self) -> u32 {
        self.get_with_ticks().0
    }

    /// Gets the current time as a tuple containing the count of seconds since some point in the past
    /// and the sub-second time as 32768Hz ticks.
    pub fn get_with_ticks(&self) -> (u32, u16) {
        get(&self.reg)
    }

    /// Gets the current time as a tuple containing the count of seconds since some point in the past
    /// and the sub-second time as microseconds.
    pub fn get_with_micros(&self) -> (u32, u32) {
        let (seconds, ticks) = self.get_with_ticks();
        (seconds, ticks_to_micros(ticks))
    }

    /// Set the current time as a count of seconds since some point in the past and the sub-second
    /// time as 32768Hz ticks.
    pub fn set(&mut self, time: u32, ticks: u16) {
        disable(&mut self.reg);
        set(&mut self.reg, time, ticks);
        enable(&mut self.reg);
    }
}

impl fmt::Debug for SRTC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SRTC").finish()
        // very basic, just to prevent compile errors if user puts it in a struct
    }
}
