//! (Secure) Real-Time Clock
//!
//! Basic support only: Supports enabling and setting the clocks, but not any of their more
//! advanced features (alarm, interrupt, power glitch detection, etc).
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
//! let mut rtc = peripherals.rtc.enable_and_set(1600000000);
//! // Interpreted as Unix time: Sep 13 2020 12:26:40
//!
//! let now = rtc.get();
//! ```
//!
//! # Teensy 4.x Note
//!
//! When the SRTC is enabled, setting the board into program mode then using the Teensy Loader
//! to reboot it will set the current time (Unix epoch, but time in local timezone). This will
//! overwrite whatever time you may have previously set.

use core::fmt;

use crate::ral;
use crate::ral::snvs::Instance;

const MR_SHIFT: u8 = 17;
const LR_SHIFT: u8 = 15;

/// A disabled RTC.
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

    /// Enable both the Secure Real-Time Clock and the Real-Time Clock and set them to the
    /// provided time. The time provided is
    /// [a count of seconds since some epoch](https://en.wikipedia.org/wiki/Epoch_(computing)).
    /// Usually that will be the Unix epoch, but as an example setting `0` would create a simple
    /// 'seconds since RTC enabled' clock.
    pub fn enable_and_set(mut self, time: u32) -> RTC {
        disable(&mut self.reg);
        set(&mut self.reg, time);
        self.enable()
        // a whole disable-set-enable cycle appears to take ~950us
    }

    /// Enable the Real-Time Clock without setting the time. The SRTC keeps track of time as long
    /// as it is enabled and has power, so this function will avoid overwriting the old time.
    ///
    /// If no time had previously been set, the RTC will start counting from `0`.
    pub fn enable(mut self) -> RTC {
        enable(&mut self.reg);
        RTC { reg: self.reg }
    }
}

/// Disable the RTC and SRTC.
fn disable(snvs: &mut Instance) {
    // If the SRTC is locked this function will loop forever.
    // SRTC locking is not implemented, so if it's locked then the user did it manually.
    ral::modify_reg!(ral::snvs, snvs, HPCR, RTC_EN: RTC_EN_0); // disable RTC
    ral::modify_reg!(ral::snvs, snvs, LPCR, SRTC_ENV: SRTC_ENV_0); // disable SRTC
    while ral::read_reg!(ral::snvs, snvs, HPCR, RTC_EN) != 0
        || ral::read_reg!(ral::snvs, snvs, LPCR, SRTC_ENV) != 0
    {
        core::sync::atomic::spin_loop_hint();
    } // wait until RTC and SRTC turn off
}

/// Sets the SRTC time.
fn set(snvs: &mut Instance, time: u32) {
    ral::modify_reg!(ral::snvs, snvs, LPSRTCMR, SRTC: time >> MR_SHIFT);
    ral::write_reg!(ral::snvs, snvs, LPSRTCLR, time << LR_SHIFT);
    // The lowest 15 bits of MR are the MSB, upper are reserved. §20.6.6
    // The upper 17 bits of LR are the LSB down to seconds.
    // The remaining 15 bits of LR are slowly incrementing and appear to relate to the RTC's
    // optional periodic interrupt. As we are changing the time, they are reset to 0.
}

/// Enable the RTC and SRTC, setting RTC to sync from SRTC.
fn enable(snvs: &mut Instance) {
    ral::modify_reg!(ral::snvs, snvs, LPCR, SRTC_ENV: SRTC_ENV_1); // enable SRTC
    ral::modify_reg!(ral::snvs, snvs, HPCR, RTC_EN: RTC_EN_1, HP_TS: HP_TS_1); // enable RTC and sync
    while ral::read_reg!(ral::snvs, snvs, HPCR, RTC_EN) == 0
        || ral::read_reg!(ral::snvs, snvs, LPCR, SRTC_ENV) == 0
    {
        core::sync::atomic::spin_loop_hint();
    } // wait until RTC and SRTC turn on
}

/// The Real-Time Clock, enabled.
pub struct RTC {
    reg: Instance,
}

impl RTC {
    /// Get the current time as a count of seconds since some point in the past.
    pub fn get(&self) -> u32 {
        let mut time = 0;
        let mut check = 0;
        for _ in 0..6 {
            // reference manual says this should take no more than 3 sessions of 2 reads; do 6
            time = check;
            // Safety: Reading from unprivileged readable registers.
            unsafe {
                check = ral::read_reg!(ral::snvs, SNVS, HPRTCMR, RTC) << MR_SHIFT;
                check |= ral::read_reg!(ral::snvs, SNVS, HPRTCLR) >> LR_SHIFT;
            }
            if time == check {
                break;
            }
        }
        time
    }

    /// Set the current time as a count of seconds since some point in the past.
    pub fn set(&mut self, time: u32) {
        disable(&mut self.reg);
        set(&mut self.reg, time);
        enable(&mut self.reg);
    }
}

impl fmt::Debug for RTC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RTC").finish()
        // very basic, just to prevent compile errors if user puts it in a struct
    }
}
