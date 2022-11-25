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
    #[cfg(family = "imxrt10xx")]
    const _: () = assert!(lpi2c_frequency(RunMode::Overdrive) == 8_000_000);

    // Assuming all the LPI2C root clocks use the same underlying config.
    #[cfg(family = "imxrt11xx")]
    const _: () = assert!(lpi2c_frequency::<1>(RunMode::Overdrive) == 8_000_000);

    match (run_mode, clock_speed) {
        (RunMode::Overdrive, ClockSpeed::KHz400) => Timing::new(
            ClockConfiguration {
                clkhi: 0x05,
                clklo: 0x0B,
                sethold: 0x04,
                datavd: 0x02,
                filtscl: 0x0,
                filtsda: 0x0,
            },
            Prescaler::Prescaler1,
        ),
        (RunMode::Overdrive, ClockSpeed::KHz100) => Timing::new(
            ClockConfiguration {
                clkhi: 0x05,
                clklo: 0x0B,
                sethold: 0x04,
                datavd: 0x02,
                filtscl: 0x0,
                filtsda: 0x0,
            },
            Prescaler::Prescaler4,
        ),
        (RunMode::Overdrive, ClockSpeed::MHz1) => Timing::new(
            ClockConfiguration {
                clkhi: 0x01,
                clklo: 0x03,
                sethold: 0x02,
                datavd: 0x01,
                filtscl: 0x0,
                filtsda: 0x0,
            },
            Prescaler::Prescaler1,
        ),
    }
}
