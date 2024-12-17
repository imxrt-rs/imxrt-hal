//! Temperature monitor.
//!
//! ## IMPORTANT NOTE:
//!
//! On 10xx MCUs, the temperature sensor uses and assumes that the bandgap
//! reference, 480MHz PLL and 32KHz RTC modules are properly programmed and fully
//! settled for correct operation.
//!
//! ## Example 1
//!
//! Manually triggered read
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! let inst = unsafe { ral::tempmon::TEMPMON::instance() };
//! let mut temp_mon = hal::tempmon::TempMon::new(inst);
//! loop {
//!     if let Ok(temperature) = nb::block!(temp_mon.measure_temp()) {
//!         // Temperature in mC (1°C = 1000°mC)
//!     }
//! }
//! ```
//!
//! ## Example 2
//!
//! Non-blocking reading
//!
//! ```no_run
//! use imxrt_hal::tempmon::TempMon;
//! use imxrt_ral as ral;
//!
//! let inst = unsafe { ral::tempmon::TEMPMON::instance() };
//!
//! // Init temperature monitor with 8Hz measure freq
//! // 0xffff = 2 Sec. Read more at `measure_freq()`
//! let mut temp_mon = TempMon::with_measure_freq(inst, 0x1000);
//! temp_mon.start();
//!
//! let mut last_temp = 0_i32;
//! loop {
//!     // Get the last temperature read by the measure_freq
//!     if let Ok(temp) = temp_mon.get_temp() {
//!         if last_temp != temp {
//!             // Temperature changed
//!             last_temp = temp;
//!         }
//!         // Do something else
//!     }
//! }
//! ```
//!
//! ## Example 3
//!
//! Low and high temperature Interrupt
//!
//! *NOTE*: TEMP_LOW_HIGH is triggered for `TempSensor low` and `TempSensor high`
//!
//! ```no_run
//! use imxrt_hal::tempmon::TempMon;
//! use imxrt_ral as ral;
//!
//! let inst = unsafe { ral::tempmon::TEMPMON::instance() };
//!
//! // Init temperature monitor with 8Hz measure freq
//! // 0xffff = 2 Sec. Read more at `measure_freq()`
//! let mut temp_mon = TempMon::with_measure_freq(inst, 0x1000);
//!
//! // Set low_alarm, high_alarm, and panic_alarm temperature
//! temp_mon.set_alarm_values(-5_000, 65_000, 95_000);
//!
//! // Use values from registers if you like to compare it somewhere
//! let (low_alarm, high_alarm, panic_alarm) = temp_mon.alarm_values();
//!
//! // Enables interrupts for low_high_alarm
//! unsafe {
//!     cortex_m::peripheral::NVIC::unmask(ral::interrupt::TEMP_LOW_HIGH);
//! }
//!
//! // Start could fail if the module is not powered up
//! if temp_mon.start().is_err() {
//!     temp_mon.power_up();
//!     temp_mon.start();
//! }
//!
//! // #[cortex_m_rt::interrupt]
//! fn TEMP_LOW_HIGH() {
//!     // disable the interrupt to avoid endless triggers
//!     cortex_m::peripheral::NVIC::mask(ral::interrupt::TEMP_LOW_HIGH);
//!
//!     // don't forget to enable it after the temperature is back to normal
//! }
//! ```

use crate::ral;

/// Indicates that the temperature monitor is powered down.
///
/// If you receive this error, `power_up()` the temperature monitor first,
/// and try again.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerDownError(());

/// A Temperature Monitor (TEMPMON)
///
/// See the [module-level documentation](crate::tempmon) for important notes.
///
/// # Example
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_ral as ral;
///
/// let inst = unsafe { ral::tempmon::TEMPMON::instance() };
/// let mut temp_mon = hal::tempmon::TempMon::new(inst);
/// loop {
///     if let Ok(_temperature) = nb::block!(temp_mon.measure_temp()) {
///         // _temperature in mC (1°C = 1000°mC)
///     }
/// }
/// ```
pub struct TempMon {
    base: ral::tempmon::TEMPMON,
    /// Scaler
    scaler: i32,
    /// Hot_count
    hot_count: i32,
    /// Hot_temp * 1000
    hot_temp: i32,
}

// We have to impl Debug by hand because the codegen'ed
// ral::tempmon::TEMPMON doesn't impl Debug...
impl core::fmt::Debug for TempMon {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Note: omit `base` which doesn't impl Debug or Display
        fmt.debug_struct("TempMon")
            .field("scaler", &format_args!("{}", self.scaler))
            .field("hot_count", &format_args!("{}", self.hot_count))
            .field("hot_temp", &format_args!("{}", self.hot_temp))
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for TempMon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TempMon {{ scaler: {}, hot_count: {}, hot_temp: {} }}",
            self.scaler,
            self.hot_count,
            self.hot_temp,
        )
    }
}

impl TempMon {
    /// Initialize and create the temperature monitor.
    pub fn new(tempmon: ral::tempmon::TEMPMON) -> Self {
        // Safety: This value is read-only and set by the manufacturer. imxrt-ral
        // is constructed to always point at a valid OCOTP instance.
        let calibration = unsafe { ral::read_reg!(ral::ocotp, OCOTP, ANA1) };

        // The ral doesn't provide direct access to the values.
        let n1_room_count = (calibration >> 20) as i32;
        let t1_room_temp = 25_000_i32;
        let n2_hot_count = ((calibration >> 8) & 0xFFF) as i32;
        let t2_hot_temp = (calibration & 0xFF) as i32 * 1_000;

        // Tmeas = HOT_TEMP - (Nmeas - HOT_COUNT) * ((HOT_TEMP - 25.0) / (ROOM_COUNT – HOT_COUNT))
        let scaler = (t2_hot_temp - t1_room_temp) / (n1_room_count - n2_hot_count);
        // Tmeas = HOT_TEMP - (Nmeas - HOT_COUNT) * scaler

        let t = Self {
            base: tempmon,
            scaler,
            hot_count: n2_hot_count,
            hot_temp: t2_hot_temp,
        };
        t.power_up();
        t
    }
    /// Initialize the temperature monitor.
    ///
    /// The `measure_freq` determines how many RTC clocks to wait before automatically repeating a temperature
    /// measurement. The pause time before remeasuring is the field value multiplied by the RTC period.
    ///
    /// Find more details [`set_measure_frequency`](TempMon::set_measure_frequency).
    pub fn with_measure_freq(tempmon: ral::tempmon::TEMPMON, measure_freq: u16) -> Self {
        let mut t = Self::new(tempmon);
        t.set_measure_frequency(measure_freq);
        t
    }
    /// Converts the temp_cnt into a human readable temperature [°mC] (1/1000 °C)
    ///
    /// param **temp_cnt**: measurement value from the tempmon module
    ///
    /// return: Temperature in °mC (1/1000°C)
    fn convert(&self, temp_cnt: i32) -> i32 {
        let n_meas = temp_cnt - self.hot_count;
        self.hot_temp - n_meas * self.scaler
    }

    /// Decode the temp_value into measurable bytes
    ///
    /// param **temp_value_mc**: temperature value in °mC (1/1000°C)
    ///
    /// return: decoded temperature, compatible to the module internal measurements
    fn decode(&self, temp_value_mc: i32) -> u32 {
        let v = (temp_value_mc - self.hot_temp) / self.scaler;
        (self.hot_count - v) as u32
    }

    /// Triggers a new measurement
    ///
    /// If you configured automatically repeating, this will trigger additional measurement.
    /// Use get_temp instate to get the last read value
    ///
    /// The returning temperature in 1/1000 Celsius (°mC)
    ///
    /// Example: 25500°mC -> 25.5°C
    pub fn measure_temp(&mut self) -> nb::Result<i32, PowerDownError> {
        if !self.is_powered_up() {
            Err(nb::Error::from(PowerDownError(())))
        } else {
            // If no measurement is active, trigger new measurement
            let active = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, MEASURE_TEMP == START);
            if !active {
                ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0_SET, MEASURE_TEMP: START);
            }

            // If the measurement is not finished or not started
            // i.MX Docs: This bit should be cleared by the sensor after the start of each measurement
            if ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, FINISHED == INVALID) {
                // measure_temp could be triggered again without any effect
                Err(nb::Error::WouldBlock)
            } else {
                // Clear MEASURE_TEMP to trigger a new measurement at the next call
                ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0_CLR, MEASURE_TEMP: START);

                let temp_cnt = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, TEMP_CNT) as i32;
                Ok(self.convert(temp_cnt))
            }
        }
    }

    /// Returns the last read value from the temperature sensor
    ///
    /// The returning temperature in 1/1000 Celsius (°mC)
    ///
    /// Example: 25500°mC -> 25.5°C
    pub fn get_temp(&self) -> nb::Result<i32, PowerDownError> {
        if self.is_powered_up() {
            let temp_cnt = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, TEMP_CNT) as i32;
            Ok(self.convert(temp_cnt))
        } else {
            Err(nb::Error::from(PowerDownError(())))
        }
    }

    /// Starts the measurement process. If the measurement frequency is zero, this
    /// results in a single conversion.
    pub fn start(&mut self) -> nb::Result<(), PowerDownError> {
        if self.is_powered_up() {
            ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0_SET, MEASURE_TEMP: START);
            Ok(())
        } else {
            Err(nb::Error::from(PowerDownError(())))
        }
    }

    /// Stops the measurement process. This only has an effect If the measurement
    /// frequency is not zero.
    pub fn stop(&self) {
        ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0_CLR, MEASURE_TEMP: START);
    }

    /// Returns the true if the tempmon module is powered up.
    pub fn is_powered_up(&self) -> bool {
        ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, POWER_DOWN == POWER_UP)
    }

    /// This powers down the temperature sensor.
    pub fn power_down(&self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0_SET,
            POWER_DOWN: POWER_DOWN
        );
    }

    /// This powers up the temperature sensor.
    pub fn power_up(&self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0_CLR,
            POWER_DOWN: POWER_DOWN
        );
    }

    /// Set the temperature that will generate a low alarm, high alarm, and panic alarm interrupt
    /// when the temperature exceeded this values.
    ///
    /// ## Note:
    /// low_alarm_mc, high_alarm_mc, and panic_alarm_mc are in milli Celsius (1/1000 °C)
    pub fn set_alarm_values(&mut self, low_alarm_mc: i32, high_alarm_mc: i32, panic_alarm_mc: i32) {
        let low_alarm = self.decode(low_alarm_mc);
        let high_alarm = self.decode(high_alarm_mc);
        let panic_alarm = self.decode(panic_alarm_mc);
        ral::modify_reg!(ral::tempmon, self.base, TEMPSENSE0, ALARM_VALUE: high_alarm);
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE2,
            LOW_ALARM_VALUE: low_alarm,
            PANIC_ALARM_VALUE: panic_alarm
        );
    }

    /// Queries the temperature that will generate a low alarm, high alarm, and panic alarm interrupt.
    ///
    /// Returns (low_alarm_temp, high_alarm_temp, panic_alarm_temp)
    pub fn alarm_values(&self) -> (i32, i32, i32) {
        let high_alarm = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, ALARM_VALUE);
        let (low_alarm, panic_alarm) = ral::read_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE2,
            LOW_ALARM_VALUE,
            PANIC_ALARM_VALUE
        );
        (
            self.convert(low_alarm as i32),
            self.convert(high_alarm as i32),
            self.convert(panic_alarm as i32),
        )
    }

    /// This bits determines how many RTC clocks to wait before automatically repeating a temperature
    /// measurement. The pause time before remeasuring is the field value multiplied by the RTC period.
    ///
    /// | value  | note |
    /// | ------ | ----------------------------------------------------- |
    /// | 0x0000 | Defines a single measurement with no repeat.          |
    /// | 0x0001 | Updates the temperature value at a RTC clock rate.    |
    /// | 0x0002 | Updates the temperature value at a RTC/2 clock rate.  |
    /// | ...    | ... |
    /// | 0xFFFF | Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock.|
    ///
    pub fn set_measure_frequency(&mut self, measure_freq: u16) {
        ral::modify_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE1,
            MEASURE_FREQ: measure_freq as u32
        );
    }
}
