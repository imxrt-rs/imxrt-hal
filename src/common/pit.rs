//! Periodic interrupt timers.
//!
//! PITs are countdown timers that run on the PERCLK clock root.
//! When the timer elapses, it automatically restarts from the load value.
//! There are four PIT channels for independent time tracking. The four
//! channels share an interrupt.
//!
//! You can chain channels together by using [`Chained`](Chained).
//! This doubles the width of the timer.
//!
//! # Example
//!
//! Note that these examples do not demonstrate how to configure the PIT
//! clock gates, or PERCLK. For more information, see [the CCM peripheral clock
//! module](crate::ccm::perclk_clk).
//!
//! Acquire the four PIT timer channels:
//!
//! ```no_run
//! use imxrt_hal::pit;
//! use imxrt_ral::pit::PIT;
//!
//! let (mut pit0, mut pit1, _, _) = PIT::take().map(pit::new).unwrap();
//! ```
//!
//! Use `pit0` to implement a blocking delay:
//!
//! ```no_run
//! # use imxrt_hal::pit;
//! # use imxrt_ral::pit::PIT;
//! # let (mut pit0, pit1, _, _) = PIT::take().map(pit::new).unwrap();
//! # const DELAY_MS: u32 = 1;
//! pit0.set_load_timer_value(DELAY_MS);
//! pit0.enable();
//!
//! loop {
//!     while !pit0.is_elapsed() {}
//!     pit0.clear_elapsed();
//!     // Do work...
//! }
//! ```
//!
//! Chain the channels together to form a larger timer:
//!
//! ```no_run
//! # use imxrt_hal::pit;
//! # use imxrt_ral::pit::PIT;
//! # let (pit0, pit1, _, _) = PIT::take().map(pit::new).unwrap();
//!
//! let chained = pit::Chained01::new(pit0, pit1);
//! ```
//!
//! TODO
//!
//! - Add support for chips with multiple PITs, like the 1170. The
//!   types do not express this idea, and implementation details
//!   assume one PIT instance.

/// Channel 0.
pub type Pit0 = Pit<0>;
/// Channel 1.
pub type Pit1 = Pit<1>;
/// Channel 2.
pub type Pit2 = Pit<2>;
/// Channel 3.
pub type Pit3 = Pit<3>;

/// All four channels.
pub type Channels = (Pit0, Pit1, Pit2, Pit3);

/// A periodic interrupt timer (PIT) channel.
pub struct Pit<const CHAN: u8> {
    instance: &'static crate::ral::pit::RegisterBlock,
}

/// Convert the PIT peripheral instances into four timer channels.
///
/// `new` will reset all timer control registers before returning
/// the channels. It is is guaranteed to not touch the `FRZ` bit
/// in `MCR`.
pub fn new<const N: u8>(pit: crate::ral::pit::Instance<N>) -> Channels {
    crate::ral::modify_reg!(crate::ral::pit, pit, MCR, MDIS: MDIS_0);
    // Reset all PIT channels
    //
    // PIT channels may be used by a systems boot ROM, or another
    // user. Set them to a known, good state.
    crate::ral::write_reg!(crate::ral::pit::timer, &pit.TIMER[0], TCTRL, 0);
    crate::ral::write_reg!(crate::ral::pit::timer, &pit.TIMER[1], TCTRL, 0);
    crate::ral::write_reg!(crate::ral::pit::timer, &pit.TIMER[2], TCTRL, 0);
    crate::ral::write_reg!(crate::ral::pit::timer, &pit.TIMER[3], TCTRL, 0);

    unsafe {
        (
            Pit::new(&pit),
            Pit::new(&pit),
            Pit::new(&pit),
            Pit::new(&pit),
        )
    }
}

mod private {
    pub trait Sealed {}
}
pub trait Valid {}
pub enum Const<const N: u8> {}
impl private::Sealed for Const<0> {}
impl private::Sealed for Const<1> {}
impl private::Sealed for Const<2> {}
impl private::Sealed for Const<3> {}
impl Valid for Const<0> {}
impl Valid for Const<1> {}
impl Valid for Const<2> {}
impl Valid for Const<3> {}

impl<const CHAN: u8> Pit<CHAN> {
    /// Fabricate a PIT channel instance.
    ///
    /// # Safety
    ///
    /// This allows you to produce multiple PIT channels that mutably
    /// reference the same memory. You must ensure that writes to the
    /// peripheral memory are synchronized across instances.
    ///
    /// Use the free function [`new()`](crate::pit::new) to safely
    /// acquire the four PIT channels.
    pub unsafe fn new<const N: u8>(instance: &crate::ral::pit::Instance<N>) -> Self
    where
        Const<CHAN>: Valid,
    {
        let register_block: &'_ crate::ral::pit::RegisterBlock = &*instance;
        let register_block: &'static _ = core::mem::transmute(register_block);
        Self {
            instance: register_block,
        }
    }

    /// Enable (true) or disable (false) interrupt generation.
    pub fn set_interrupt_enable(&mut self, enable: bool) {
        crate::ral::modify_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TCTRL,
            TIE: enable as u32
        )
    }

    /// Indicates if timeouts will (true) or will not (false) generate interrupts.
    pub fn is_interrupt_enabled(&self) -> bool {
        crate::ral::read_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TCTRL,
            TIE == 1
        )
    }

    /// Reads the current time value, in clock ticks.
    ///
    /// Returns `0` if the timer is disabled.
    pub fn current_timer_value(&self) -> u32 {
        if self.is_enabled() {
            // Note in CVAL register docs: don't read CVAL if the timer
            // is disable "because the value is unreliable."
            crate::ral::read_reg!(
                crate::ral::pit::timer,
                &self.instance.TIMER[CHAN as usize],
                CVAL
            )
        } else {
            0
        }
    }

    /// Loads the timer value for the next timer run.
    ///
    /// `ticks` is in clock ticks.
    pub fn set_load_timer_value(&self, ticks: u32) {
        crate::ral::write_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            LDVAL,
            ticks.saturating_sub(1)
        );
    }

    /// Returns the load timer value for the next timer run, in clock ticks.
    pub fn load_timer_value(&self) -> u32 {
        crate::ral::read_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            LDVAL
        )
        .saturating_add(1)
    }

    /// Enable the timer.
    pub fn enable(&mut self) {
        crate::ral::modify_reg!(crate::ral::pit::timer, &self.instance.TIMER[CHAN as usize], TCTRL, TEN: 1);
    }

    /// Disable the timer.
    pub fn disable(&mut self) {
        crate::ral::modify_reg!(crate::ral::pit::timer, &self.instance.TIMER[CHAN as usize], TCTRL, TEN: 0);
    }

    /// Returns `true` if the PIT channel is enabled.
    pub fn is_enabled(&self) -> bool {
        crate::ral::read_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TCTRL,
            TEN == 1
        )
    }

    /// Returns `true` if the timer has elapsed.
    pub fn is_elapsed(&self) -> bool {
        crate::ral::read_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TFLG,
            TIF == 1
        )
    }

    /// Clear the elapsed flag.
    pub fn clear_elapsed(&self) {
        crate::ral::write_reg!(crate::ral::pit::timer, &self.instance.TIMER[CHAN as usize], TFLG, TIF: 1)
    }

    /// Specify that this channel is chained to the previous channel.
    ///
    /// This affects how the timer counts. If you're looking to chain timers
    /// easily, see [`Chained`](crate::pit::Chained).
    pub fn set_chained(&mut self, chained: bool) {
        crate::ral::modify_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TCTRL,
            CHN: chained as u32
        );
    }

    /// Returns true if this channel is chained to the previous channel.
    pub fn is_chained(&self) -> bool {
        crate::ral::read_reg!(
            crate::ral::pit::timer,
            &self.instance.TIMER[CHAN as usize],
            TCTRL,
            CHN == 1
        )
    }
}

unsafe impl<const CHAN: u8> Send for Pit<CHAN> {}

/// Two chained PIT timer channels.
///
/// When the low timer counts down to zero, the high timer is decremented
/// by one. This doubles the width of the timer.
///
/// Timers must be chained in sequence. For example, timer 2 (high) can be
/// chained to timer 1 (low). But, timer 3 cannot be chained to timer 1.
///
/// Chaining channel 1 and 0 enables the lifetime register. The lifetime
/// register allows us to read two 32-bit registers without rollover.
/// Otherwise, the implementation handles rollovers in software with a small
/// loop and comparison.
pub struct Chained<const L: u8, const H: u8> {
    low: Pit<L>,
    high: Pit<H>,
}

/// Chained channels 0 and 1.
pub type Chained01 = Chained<0, 1>;
/// Chained channels 1 and 2.
pub type Chained12 = Chained<1, 2>;
/// Chained channels 2 and 3.
pub type Chained23 = Chained<2, 3>;

impl Chained<0, 1> {
    /// Chain together channels 0 and 1.
    ///
    /// This creates the lifetime timer.
    pub fn new(low: Pit<0>, high: Pit<1>) -> Self {
        chain(low, high)
    }

    /// Read the lifetime register value.
    ///
    /// This is only supported when chaining channels 0 and
    /// channel 1. Returns `0` if the timer is disabled. The
    /// lifetime registers account for rollover possibility
    /// in hardware.
    ///
    /// This method implements the recommended fix for
    /// errata ERR050130.
    pub fn lifetime_value(&self) -> u64 {
        if !self.is_enabled() {
            return 0;
        }

        // Safety: there can only be one (safe) instance of this chained timer.
        // We effectively own these registers. These addresses are valid MMIO
        // registers.
        let mut high = self.low.instance.LTMR64H.read();
        let mut low = self.low.instance.LTMR64L.read();

        let ldval0 = self.low.load_timer_value();
        if low == ldval0 {
            high = self.low.instance.LTMR64H.read();
            low = self.low.instance.LTMR64L.read();
        }

        (u64::from(high) << 32) + u64::from(low)
    }
}

impl Chained<1, 2> {
    /// Chain together channels 1 and 2.
    pub fn new(low: Pit<1>, high: Pit<2>) -> Self {
        chain(low, high)
    }
}

impl Chained<2, 3> {
    /// Chain together channels 2 and 3.
    pub fn new(low: Pit<2>, high: Pit<3>) -> Self {
        chain(low, high)
    }
}

/// Chain the low and high timers together.
///
/// This has no type safety to ensure valid channel chaining.
fn chain<const L: u8, const H: u8>(mut low: Pit<L>, mut high: Pit<H>) -> Chained<L, H> {
    low.disable();
    high.disable();

    low.clear_elapsed();
    high.clear_elapsed();

    low.set_chained(false);
    low.set_interrupt_enable(false);
    high.set_chained(true);
    Chained { low, high }
}

impl<const L: u8, const H: u8> Chained<L, H> {
    /// Release the chained timers.
    ///
    /// When `release` returns, the timer chain is disabled, and the
    /// timer channels are disabled.
    pub fn release(mut self) -> (Pit<L>, Pit<H>) {
        self.low.disable();
        self.high.disable();

        self.high.set_chained(false);
        self.high.set_interrupt_enable(false);
        (self.low, self.high)
    }

    /// Enable (true) or disable (false) interrupt generation when
    /// the chained timer expires.
    pub fn set_interrupt_enable(&mut self, enable: bool) {
        self.high.set_interrupt_enable(enable);
    }

    /// Indicates if timeouts will (true) or will not (false) generate interrupts.
    pub fn is_interrupt_enabled(&self) -> bool {
        self.high.is_interrupt_enabled()
    }

    /// Loads the timer value for the next timer run.
    ///
    /// `ticks` is in clock ticks.
    pub fn set_load_timer_value(&mut self, ticks: u64) {
        self.low.set_load_timer_value((ticks & 0xFFFF_FFFF) as u32);
        self.high.set_load_timer_value((ticks >> 32) as u32);
    }

    /// Returns the load timer value for the next timer run, in ticks.
    pub fn load_timer_value(&self) -> u64 {
        let low = self.low.load_timer_value();
        let high = self.high.load_timer_value();
        (u64::from(high) << 32) + u64::from(low)
    }

    /// Reads the current timer value, in clock ticks.
    ///
    /// Returns `0` if the timer is disabled.
    pub fn current_timer_value(&self) -> u64 {
        if !self.is_enabled() {
            return 0;
        }

        loop {
            let tmp = self.high.current_timer_value();
            let low = self.low.current_timer_value();
            let high = self.high.current_timer_value();
            if high == tmp {
                // No rollover.
                return (u64::from(high) << 32) + u64::from(low);
            }
        }
    }

    /// Enable the chained timer.
    pub fn enable(&mut self) {
        self.high.enable();
        self.low.enable();
    }

    /// Disable the chained timer.
    pub fn disable(&mut self) {
        self.low.disable();
        self.high.disable();
    }

    /// Returns `true` if the chained timer is enabled.
    pub fn is_enabled(&self) -> bool {
        self.high.is_enabled()
    }

    /// Returns `true` if the chained timer has elapsed.
    pub fn is_elapsed(&self) -> bool {
        self.high.is_elapsed()
    }

    /// Clear the elapsed flag.
    pub fn clear_elapsed(&mut self) {
        self.high.clear_elapsed();
        self.low.clear_elapsed(); // Is this necessary?
    }
}

/// ```compile_fail
/// use imxrt_hal as hal;
/// use hal::pit::Pit;
///
/// let p: Pit<4> = unsafe { Pit::new() };
/// ```
#[cfg(doctest)]
struct InvalidChannel;
