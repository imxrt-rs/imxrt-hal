//! Clock control module (CCM)
//!
//! The clocks and types exposed in `ccm` support clock control and peripheral clock
//! gating. Use [`CCM::from_ral`](CCM) to acquire the clock roots and the
//! CCM handle. Then, enable your clocks.
//!
//! ```no_run
//! use imxrt1060_hal as hal;
//! use hal::{ccm, ral};
//!
//! let ccm::CCM {
//!     mut handle,
//!     uart_clock,
//!     ..
//! } = ral::ccm::CCM::take().map(ccm::CCM::from_ral).unwrap();
//!
//! let mut uart_clock = uart_clock.enable(&mut handle);
//! ```
//!
//! Clocks can enable peripheral clock gates, and they may be used in APIs that require
//! you to first initialize clocks.
//!
//! ```no_run
//! # use imxrt1060_hal as hal;
//! # use hal::{ccm, ral};
//! # let ccm::CCM {
//! #     mut handle,
//! #     uart_clock,
//! #     ..
//! # } = ral::ccm::CCM::take().map(ccm::CCM::from_ral).unwrap();
//! # let mut uart_clock = uart_clock.enable(&mut handle);
//! let mut lpuart2 = ral::lpuart::LPUART2::take().unwrap();
//!
//! // Enable the clock gate:
//! uart_clock.set_clock_gate(&mut lpuart2, ccm::ClockGate::On);
//!
//! // Create the peripheral... see UART documentation for more information.
//! ```
pub use imxrt_ccm::{
    ral::{I2CClock, PerClock, SPIClock, UARTClock, CCM},
    ClockGate, Handle,
};

/// Compute the number of clock ticks that represent the microsecond delay
pub(crate) fn ticks(delay_us: u32, clock_hz: u32) -> u32 {
    1_000_000_000u32
        .checked_div(clock_hz)
        .and_then(|period_ns| {
            let delay_ns = delay_us.saturating_mul(1_000);
            delay_ns
                .checked_div(period_ns)
                .map(|ticks| ticks.saturating_sub(1))
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::ticks;

    #[test]
    fn ticks_zero_clock() {
        assert_eq!(ticks(1, 0), 0);
    }

    #[test]
    fn ticks_gpt() {
        const HZ: u32 = 8_000_000; // 8MHz
        assert_eq!(ticks(0, HZ), 0);
        assert_eq!(ticks(1, HZ), 7);
        assert_eq!(ticks(1_000, HZ), 7_999);
        assert_eq!(ticks(1_000_000, HZ), 7_999_999);
        // Saturating mul
        assert_eq!(ticks(1_000_000_000, HZ), ticks(0xFFFF_FFFF, HZ));
    }
}
