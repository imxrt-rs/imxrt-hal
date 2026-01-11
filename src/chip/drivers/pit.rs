//! Periodic interrupt timers.
//!
//! Each PIT has four channels, all running at the same frequency.
//! PIT channels count down to zero from a starting load value.
//! When a channel elapses, it automatically restarts from the load value.
//! All four channels share an interrupt.
//!
//! You can chain channels together using [`Pit::enable_chaining`].
//! This increases the width of the timer by having a channel count down
//! each time the previous channel expires. You can chain more than two
//! channels together. Keep in mind that you'll need to handle cases of
//! timer overflow in software.
//!
//! When channel 0 and 1 are chained together, the lifetime register is
//! enabled. Reads from the lifetime registers will automatically handle
//! overflow without a software loop.
//!
//! # Example
//!
//! Note that these examples do not demonstrate how to configure the PIT
//! clock gates, or PERCLK. For more information, see [the CCM peripheral clock
//! module](crate::ccm::perclk_clk).
//!
//! Acquire the PIT driver:
//!
//! ```no_run
//! use imxrt_hal::pit::{Pit, Channel};
//! use imxrt_ral::pit::PIT;
//!
//! let mut pit = Pit::new(unsafe { PIT::instance() });
//! ```
//!
//! Use channel 0 to implement a blocking delay:
//!
//! ```no_run
//! # use imxrt_hal::pit::{Pit, Channel};
//! # use imxrt_ral::pit::PIT;
//! # let mut pit = Pit::new(unsafe { PIT::instance() });
//! # const DELAY_MS: u32 = 1;
//! pit.set_load_timer_value(Channel::Chan0, DELAY_MS);
//! pit.enable(Channel::Chan0);
//!
//! loop {
//!     while !pit.is_elapsed(Channel::Chan0) {}
//!     pit.clear_elapsed(Channel::Chan0);
//!     // Do work...
//! }
//! ```
//!
//! Chain channels 0 and 1 together for a 64-bit timer:
//!
//! ```no_run
//! # use imxrt_hal::pit::{Pit, Channel};
//! # use imxrt_ral::pit::PIT;
//! # let mut pit = Pit::new(unsafe { PIT::instance() });
//! // Channel 1 will decrement when Channel 0 expires.
//! pit.enable_chaining(Channel::Chan1).unwrap();
//! ```

use crate::ral;

/// Any PIT instance.
type AnyPitInstance = crate::AnyInstance<ral::pit::RegisterBlock>;

/// A PIT channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(usize)]
pub enum Channel {
    /// Channel 0.
    Chan0 = 0,
    /// Channel 1.
    Chan1 = 1,
    /// Channel 2.
    Chan2 = 2,
    /// Channel 3.
    Chan3 = 3,
}

/// Error returned when a channel cannot be chained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CannotChainError(());

/// A periodic interrupt timer (PIT) driver.
///
/// The PIT has four independent timer channels that share a single
/// interrupt. Use the [`Channel`] enum to select which channel to operate on.
pub struct Pit {
    pit: AnyPitInstance,
}

impl Pit {
    /// Create a new PIT driver from the RAL's PIT instance.
    ///
    /// When `new` returns, all channels are disabled and reset.
    /// The `FRZ` bit in `MCR` is not modified.
    pub fn new<const N: u8>(pit: ral::pit::Instance<N>) -> Self {
        new(crate::into_any(pit))
    }

    fn timer(&self, channel: Channel) -> &ral::pit::timer::RegisterBlock {
        &self.pit.TIMER[channel as usize]
    }

    /// Enable (true) or disable (false) interrupt generation for a channel.
    pub fn set_interrupt_enable(&mut self, channel: Channel, enable: bool) {
        ral::modify_reg!(ral::pit::timer, self.timer(channel), TCTRL, TIE: enable as u32);
    }

    /// Indicates if timeouts will (true) or will not (false) generate interrupts.
    pub fn is_interrupt_enabled(&self, channel: Channel) -> bool {
        ral::read_reg!(ral::pit::timer, self.timer(channel), TCTRL, TIE == 1)
    }

    /// Reads the current timer value, in clock ticks.
    ///
    /// Returns `0` if the channel is disabled.
    pub fn current_timer_value(&self, channel: Channel) -> u32 {
        if self.is_enabled(channel) {
            // Note in CVAL register docs: don't read CVAL if the timer
            // is disabled "because the value is unreliable."
            ral::read_reg!(ral::pit::timer, self.timer(channel), CVAL)
        } else {
            0
        }
    }

    /// Loads the timer value for the next timer run.
    ///
    /// `ticks` is in clock ticks.
    pub fn set_load_timer_value(&self, channel: Channel, ticks: u32) {
        ral::write_reg!(
            ral::pit::timer,
            self.timer(channel),
            LDVAL,
            ticks.saturating_sub(1)
        );
    }

    /// Returns the load timer value for the next timer run, in clock ticks.
    pub fn load_timer_value(&self, channel: Channel) -> u32 {
        ral::read_reg!(ral::pit::timer, self.timer(channel), LDVAL).saturating_add(1)
    }

    /// Enable a timer channel.
    pub fn enable(&mut self, channel: Channel) {
        ral::modify_reg!(ral::pit::timer, self.timer(channel), TCTRL, TEN: 1);
    }

    /// Disable a timer channel.
    pub fn disable(&mut self, channel: Channel) {
        ral::modify_reg!(ral::pit::timer, self.timer(channel), TCTRL, TEN: 0);
    }

    /// Returns `true` if the channel is enabled.
    pub fn is_enabled(&self, channel: Channel) -> bool {
        ral::read_reg!(ral::pit::timer, self.timer(channel), TCTRL, TEN == 1)
    }

    /// Returns `true` if the timer has elapsed.
    pub fn is_elapsed(&self, channel: Channel) -> bool {
        ral::read_reg!(ral::pit::timer, self.timer(channel), TFLG, TIF == 1)
    }

    /// Clear the elapsed flag.
    pub fn clear_elapsed(&self, channel: Channel) {
        ral::write_reg!(ral::pit::timer, self.timer(channel), TFLG, TIF: 1);
    }

    /// Chain adjacent channels together, forming a larger timer.
    ///
    /// A chained timer will decrement by one each time the previous channel
    /// expires. The previous channel is the channel with a lower number.
    ///
    /// For example, to chain `Chan2` to `Chan1`, supply `Chan2` as an argument.
    /// Then, every time `Chan1` expires, `Chan2` decrements by one.
    ///
    /// You may chain multiple channels together, forming 96-bit and 128-bit
    /// timers. When reading the timer values, take care to handle overflows.
    ///
    /// If you're generating interrupts from a chained timer, the channel with
    /// the larger number should generate that interrupt.
    ///
    /// When querying for the time tracked by chained timers, make sure you
    /// account for rollover in software. Keep in mind that the lifetime timer
    /// will handle rollover automatically; see [`lifetime_value`](Self::lifetime_value)
    /// for more information.
    ///
    /// # Errors
    ///
    /// Returns [`CannotChainError`] if `channel` is `Chan0`, since there is
    /// no previous channel to chain to.
    pub fn enable_chaining(&mut self, channel: Channel) -> Result<(), CannotChainError> {
        if channel == Channel::Chan0 {
            return Err(CannotChainError(()));
        }
        ral::modify_reg!(ral::pit::timer, self.timer(channel), TCTRL, CHN: 1);
        Ok(())
    }

    /// Disable chaining for a channel.
    ///
    /// See [`enable_chaining`](Self::enable_chaining) for more information.
    ///
    /// # Errors
    ///
    /// Returns [`CannotChainError`] if `channel` is `Chan0`, since `Chan0`
    /// cannot be chained.
    pub fn disable_chaining(&mut self, channel: Channel) -> Result<(), CannotChainError> {
        if channel == Channel::Chan0 {
            return Err(CannotChainError(()));
        }
        ral::modify_reg!(ral::pit::timer, self.timer(channel), TCTRL, CHN: 0);
        Ok(())
    }

    /// Returns `true` if a channel is chained to the previous channel.
    ///
    /// See [`enable_chaining`](Self::enable_chaining) for more information.
    ///
    /// Always returns `false` for `Chan0`.
    pub fn is_chained(&self, channel: Channel) -> bool {
        if channel == Channel::Chan0 {
            return false;
        }
        ral::read_reg!(ral::pit::timer, self.timer(channel), TCTRL, CHN == 1)
    }

    /// Read the lifetime register value.
    ///
    /// The lifetime register is a 64-bit register that combines channels 0 and 1.
    /// It is only valid when channel 1 is chained to channel 0.
    ///
    /// The method assumes that channel 1 is chained to channel 0. See
    /// [`enable_chaining`](Self::enable_chaining) for more information. Additionally,
    /// the method assumes both channels 0 and 1 are enabled.
    ///
    /// Given the hardware support, this call does not require a loop to account for
    /// rollover.
    ///
    /// This method implements the recommended fix for errata ERR050130.
    pub fn lifetime_value(&self) -> u64 {
        let mut high = self.pit.LTMR64H.read();
        let mut low = self.pit.LTMR64L.read();

        let ldval0 = self.pit.TIMER[0].LDVAL.read();
        if low == ldval0 {
            high = self.pit.LTMR64H.read();
            low = self.pit.LTMR64L.read();
        }

        (u64::from(high) << 32) + u64::from(low)
    }
}

fn new(pit: AnyPitInstance) -> Pit {
    ral::modify_reg!(ral::pit, pit, MCR, MDIS: MDIS_0);
    // Reset all PIT channels
    //
    // PIT channels may be used by a system's boot ROM, or another
    // user. Set them to a known, good state.
    ral::write_reg!(ral::pit::timer, &pit.TIMER[0], TCTRL, 0);
    ral::write_reg!(ral::pit::timer, &pit.TIMER[1], TCTRL, 0);
    ral::write_reg!(ral::pit::timer, &pit.TIMER[2], TCTRL, 0);
    ral::write_reg!(ral::pit::timer, &pit.TIMER[3], TCTRL, 0);

    Pit { pit }
}

/// ```compile_fail
/// use imxrt_hal::pit::Pit;
/// fn not_sync<T: Sync>() {}
///
/// not_sync::<Pit>();
/// ```
#[cfg(doctest)]
struct PitNotSync;

/// ```send
/// use imxrt_hal::pit::Pit;
/// fn is_send<T: Send>() {}
///
/// is_send::<Pit>();
/// ```
#[cfg(doctest)]
struct PitSend;
