use crate::{
    chip::ccm::clock_tree::lpi2c_frequency,
    common::lpi2c::{ClockConfiguration, ClockSpeed, Prescaler, Timing},
    RunMode,
};

/// Computes timing parameters for a given clock speed and run mode.
///
/// This function tries to approximate the best values given the expected
/// root clock frequency. It assumes that the data and clock rise times
/// are negligible. Rise times depend on your circuit, and they may not
/// be negligible at high clock frequencies. For best performance, consider
/// defining your own `Timing` parameters.
///
/// This function assumes that you're using the CCM clock tree API
/// to specify your LPI2C root clock frequency. If you're not using
/// the clock tree API, this function may not work properly.
///
/// This function is only available when a chip family feature is enabled.
pub const fn timing(clock_speed: ClockSpeed, run_mode: RunMode) -> Timing {
    // See the "LPI2C Example Timing Configurations" in your chip's
    // reference manual for more information on these values.
    const _: () = assert!(lpi2c_frequency(RunMode::Overdrive) == 60_000_000);

    match (run_mode, clock_speed) {
        (RunMode::Overdrive, ClockSpeed::KHz400) => Timing::new(
            ClockConfiguration {
                clkhi: 0x1F,
                clklo: 0x28,
                sethold: 0x11,
                datavd: 0x08,
                filtscl: 0x2,
                filtsda: 0x2,
            },
            Prescaler::Prescaler2,
        ),
        (RunMode::Overdrive, ClockSpeed::KHz100) => Timing::new(
            ClockConfiguration {
                clkhi: 0x1F,
                clklo: 0x28,
                sethold: 0x11,
                datavd: 0x08,
                filtscl: 0x2,
                filtsda: 0x2,
            },
            Prescaler::Prescaler8,
        ),
        (RunMode::Overdrive, ClockSpeed::MHz1) => Timing::new(
            ClockConfiguration {
                clkhi: 0x0B,
                clklo: 0x0F,
                sethold: 0x07,
                datavd: 0x01,
                filtscl: 0x2,
                filtsda: 0x2,
            },
            Prescaler::Prescaler2,
        ),
    }
}
