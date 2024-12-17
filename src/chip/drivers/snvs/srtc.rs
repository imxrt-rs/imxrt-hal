//! Secure real-time clock.
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
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! # || -> Option<()> {
//! let mut ccm = unsafe { ral::ccm::CCM::instance() };
//! hal::ccm::clock_gate::snvs_lp().set(&mut ccm, hal::ccm::clock_gate::ON);
//!
//! let hal::snvs::Snvs { low_power: hal::snvs::LowPower {
//!     mut core, srtc, ..
//! }, .. } = hal::snvs::new(unsafe { ral::snvs::SNVS::instance() });
//! let srtc = srtc.enable_and_set(&mut core, 1600000000, 0);
//! let now = srtc.get();
//! # Some(())}();
//! ```

use core::fmt;

use super::ral::lp::{Core, Srtc as Instance};

const MR_SHIFT: u8 = 17;
const LR_SHIFT: u8 = 15;

/// The SRTC, disabled.
///
/// Note: The SRTC may actually be enabled, as (unless disabled) it stays enabled across reboots
/// as long as it has power.
pub struct Disabled {
    reg: Instance,
}

impl Disabled {
    pub(super) fn new(reg: Instance) -> Self {
        Self { reg }
    }

    /// Enable the Secure Real-Time Clock and set it to the provided time. The time provided is
    /// [a count of seconds since some epoch](https://en.wikipedia.org/wiki/Epoch_(computing))
    /// and the sub-second 32768Hz ticks.
    ///
    /// Usually that epoch is the Unix epoch, but as an example setting `0` would create a simple
    /// 'seconds since SRTC enabled' clock.
    pub fn enable_and_set(mut self, core: &mut Core, time: u32, ticks: u16) -> Srtc {
        // a whole disable-set-enable cycle appears to take ~900us
        disable(core);
        set(&mut self.reg, time, ticks);
        self.enable(core)
    }

    /// Enable the Secure Real-Time Clock without setting the time.
    /// The SRTC keeps track of time as long as it is enabled and has power, so this function will
    /// avoid overwriting the old time.
    ///
    /// If no time had previously been set, the RTC will start counting from `0`.
    pub fn enable(self, core: &mut Core) -> Srtc {
        enable(core);
        Srtc { reg: self.reg }
    }

    /// Enable the SRTC.
    ///
    /// If the SRTC isn't already running, `try_enable` uses `seconds` and `ticks` as the starting
    /// count for the RTC.
    /// If the SRTC is currently running, the return indicates the current RTC time.
    pub fn try_enable(mut self, core: &mut Core, seconds: u32, ticks: u16) -> EnabledState {
        if is_enabled(core) {
            let (seconds, ticks) = get(&self.reg);
            EnabledState::AlreadyCounting {
                srtc: Srtc { reg: self.reg },
                seconds,
                ticks,
            }
        } else {
            set(&mut self.reg, seconds, ticks);
            enable(core);
            EnabledState::SetTime(Srtc { reg: self.reg })
        }
    }
}

/// Indicates the result of the `try_enable` method
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum EnabledState {
    /// The SRTC was already enabled, and it's currently counting from `seconds`
    AlreadyCounting {
        /// The enabled and counting SRTC
        srtc: Srtc,
        /// The current whole-second time on the SRTC
        seconds: u32,
        /// The current 32768Hz ticks of the SRTC
        ticks: u16,
    },
    /// The SRTC was not previously enabled, and it's now counting from the
    /// `seconds` and `ticks` supplied to `try_enable`
    SetTime(Srtc),
}

/// Returns whether the SRTC is enabled.
#[inline(always)]
fn is_enabled(snvs: &Core) -> bool {
    crate::ral::read_reg!(super::ral::lp::core, snvs, LPCR, SRTC_ENV) != 0
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

/// Disable the SRTC.
fn disable(snvs: &mut Core) {
    // If the SRTC is locked this function will loop forever.
    // SRTC locking is not implemented, so if it's locked then the user did it manually.
    crate::ral::modify_reg!(super::ral::lp::core, snvs, LPCR, SRTC_ENV: SRTC_ENV_0); // disable SRTC
    while is_enabled(snvs) {
        core::hint::spin_loop()
    } // wait until SRTC turns off
}

/// Sets the SRTC time.
fn set(snvs: &mut Instance, time: u32, ticks: u16) {
    let low_time = (time << LR_SHIFT) | (ticks as u32);
    crate::ral::modify_reg!(super::ral::lp::srtc, snvs, LPSRTCMR, SRTC: time >> MR_SHIFT);
    crate::ral::write_reg!(super::ral::lp::srtc, snvs, LPSRTCLR, low_time);
    // The lowest 15 bits of MR are the MSB, upper are reserved. §20.6.6
    // The upper 17 bits of LR are the LSB down to seconds.
    // The lower 15 bits of LR are the sub-second ticks of the 32768Hz clock.
    // (A 47-bit counter of a 32768Hz clock.)
}

/// Enable the SRTC.
fn enable(snvs: &mut Core) {
    crate::ral::modify_reg!(super::ral::lp::core, snvs, LPCR, SRTC_ENV: SRTC_ENV_1); // enable SRTC
    while !is_enabled(snvs) {
        core::hint::spin_loop()
    } // wait until SRTC turns on
}

/// Get the current time from the SRTC as `(seconds, ticks)`.
fn get(snvs: &Instance) -> (u32, u16) {
    let mut msb = 0;
    let mut lsb = 0;
    for _ in 0..6 {
        // reference manual says this should take no more than 3 sessions of 2 reads; do 6
        let msb2 = crate::ral::read_reg!(super::ral::lp::srtc, snvs, LPSRTCMR, SRTC);
        let lsb2 = crate::ral::read_reg!(super::ral::lp::srtc, snvs, LPSRTCLR);
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
pub struct Srtc {
    reg: Instance,
}

impl Srtc {
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
    pub fn set(&mut self, core: &mut Core, time: u32, ticks: u16) {
        disable(core);
        set(&mut self.reg, time, ticks);
        enable(core);
    }
}

impl fmt::Debug for Srtc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SRTC").finish()
        // very basic, just to prevent compile errors if user puts it in a struct
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Srtc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SRTC {{ ... }}")
    }
}
