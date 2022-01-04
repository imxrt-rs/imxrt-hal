#![cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]

//! Temperature Monitor (TEMPMON)
//!
//! IMPORTANT NOTE: The temperature sensor uses and assumes that the bandgap
//! reference, 480MHz PLL and 32KHz RTC modules are properly programmed and fully
//! settled for correct operation.
//!
//! # Example 1
//!
//! Manually triggered read
//!
//! ```no_run
//! use imxrt1060_hal;
//! use embedded_hal::timer::CountDown;
//!
//! let mut peripherals = imxrt1060_hal::Peripherals::take().unwrap();
//!
//! let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
//!     imxrt1060_hal::ccm::PLL1::ARM_HZ,
//!     &mut peripherals.ccm.handle,
//!     &mut peripherals.dcdc,
//! );
//!
//! let mut cfg = peripherals.ccm.perclk.configure(
//!     &mut peripherals.ccm.handle,
//!     imxrt1060_hal::ccm::perclk::PODF::DIVIDE_3,
//!     imxrt1060_hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
//! );
//!
//! // init temperature monitor
//! let mut temp_mon = peripherals.tempmon.init();
//! loop {
//!     let temperature = temp_mon.measure_temp();
//! }
//! ```
//!
//! # Example 2
//!
//! Non-blocking reading
//!
//! ```no_run
//! // [Example 1]
//!
//! // Init temperature monitor with 8Hz measure freq
//! // 0xffff = 2 Sec. Read more at `measure_freq()`
//! let mut temp_mon = peripherals.tempmon.init_with_measure_freq(0x1000);
//! temp_mon.start();
//!
//! let last_temp = 0.0_f32
//! loop {
//!     // Get the last temperature read by the measure_freq
//!     let temp = t.get_temp();
//!     if last_temp != temp {
//!         // temperature changed
//!         last_temp = temp;
//!     }
//!     // do something else
//! }
//! ```
//!
//! # Example 3
//!
//! Low and high temperature Interrupt
//!
//! ```no_run
//! // [Example 1]
//!
//! // init temperature monitor with 8Hz measure freq
//! // 0xffff = 2 Sec. Read more at `measure_freq()`
//! let mut temp_mon = peripherals.tempmon.init_with_measure_freq(0x1000);
//!
//! // set low_alarm, high_alarm, and panic_alarm
//! temp_mon.set_alarm_values(-5.0, 65.0, 95.0);
//!
//! // use values from registers if you like to compare it somewhere
//! let (low_alarm, high_alarm, panic_alarm) = t.alarm_values();
//!
//! // enables interrupts for low_high_alarm and not for panic_alarm
//! temp_mon.enable_interrupts(true, false);
//! temp_mon.start();
//!
//! #[cortex_m_rt::interrupt]
//! fn TEMP_LOW_HIGH() {
//!     // disable the interrupt to avoid endless triggers
//!     TempMon::disable_interrupts(true, false);
//!
//!     // don't forget to enable it after the temperature is back to normal
//! }
//! ```

use crate::ral;

/// An Uninitialized temperature monitor module
///
/// The temperature sensor uses and assumes that the bandgap
/// reference, 480MHz PLL and 32KHz RTC modules are properly
/// programmed and fully settled for correct operation.
pub struct Uninitialized(ral::tempmon::Instance);

impl Uninitialized {
    /// assign the tempmon Instance to this temperature monitor wrapper.
    pub fn new(base: ral::tempmon::Instance) -> Self {
        Self(base)
    }

    /// Initialize the temperature monitor.
    pub fn init(self) -> TempMon {
        let calibration = unsafe { ral::read_reg!(ral::ocotp, OCOTP, ANA1) };

        let n1_room_count = (calibration >> 20) as u16;
        let t1_room_temp = 25_u16;
        let n2_hot_count = ((calibration >> 8) & 0xFFF) as u16;
        let t2_hot_temp = (calibration & 0xFF) as u16;

        // Tmeas = T2 - (Nmeas - N2) * ((T2 – T1) / (N1 – N2))
        let t_dif: f32 = (t2_hot_temp - t1_room_temp).into();
        let n_dif: f32 = (n1_room_count - n2_hot_count).into();

        let scaler = t_dif / n_dif;

        // Tmeas = HOT_TEMP - (Nmeas - HOT_COUNT) * ((HOT_TEMP - 25.0) / (ROOM_COUNT – HOT_COUNT))

        let t = TempMon {
            base: self.0,
            scaler,
            hot_count: n2_hot_count,
            hot_temp: t2_hot_temp.into(),
        };
        t
    }

    /// Initialize the temperature monitor.
    ///
    /// The `measure_freq` determines how many RTC clocks to wait before automatically repeating a temperature
    /// measurement. The pause time before remeasuring is the field value multiplied by the RTC period.
    ///
    /// Find more details `TempMon.set_measure_frequency`
    pub fn init_with_measure_freq(self, measure_freq: u16) -> TempMon {
        let mut t = self.init();
        t.set_measure_frequency(measure_freq);
        t
    }
}

/// A Temperature Monitor (TEMPMON)
///
/// # Example 1
///
/// ```no_run
/// use imxrt1060_hal;
/// use embedded_hal::timer::CountDown;
///
/// let mut peripherals = imxrt1060_hal::Peripherals::take().unwrap();
/// let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
///     imxrt1060_hal::ccm::PLL1::ARM_HZ,
///     &mut peripherals.ccm.handle,
///     &mut peripherals.dcdc,
/// );
/// let mut cfg = peripherals.ccm.perclk.configure(
///     &mut peripherals.ccm.handle,
///     imxrt1060_hal::ccm::perclk::PODF::DIVIDE_3,
///     imxrt1060_hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
/// );
///
/// // init temperature monitor
/// // consider using init_with_measure_freq
/// let mut temp_mon = peripherals.tempmon.init();
/// loop {
///     let temperature = temp_mon.measure_temp();
/// }
/// ```
pub struct TempMon {
    base: ral::tempmon::Instance,
    scaler: f32,
    hot_count: u16,
    hot_temp: f32,
}

impl TempMon {
    /// converts the temp_cnt into a human readable temperature [C°]
    fn convert(&self, temp_cnt: u16) -> f32 {
        let n_meas: f32 = (temp_cnt - self.hot_count).into();
        self.hot_temp - n_meas * self.scaler
    }

    /// decode the temp_value into measurable bytes (f32 -> u32)
    fn decode(&self, temp_value: f32) -> u32 {
        let v: i32 = ((temp_value - self.hot_temp) / self.scaler) as i32;
        (self.hot_count as i32 - v) as u32
    }

    /// triggers a new measurement and waits until it's finished
    ///
    /// If you configured automatically repeating, this will trigger additional measurement.
    /// Use get_temp instate to get the last read value
    pub fn measure_temp(&self) -> f32 {
        ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0, MEASURE_TEMP: 1);

        while ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, FINISHED == 1) {}
        let temp_cnt = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, TEMP_CNT) as u16;
        self.convert(temp_cnt)
    }

    /// Returns the last read value from the temperature sensor
    pub fn get_temp(&self) -> f32 {
        while ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, FINISHED == 1) {}
        let temp_cnt = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, TEMP_CNT) as u16;
        self.convert(temp_cnt)
    }

    /// Starts the measurement process. If the measurement frequency is zero, this
    /// results in a single conversion.
    pub fn start(&mut self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0,
            MEASURE_TEMP: 1
        );
    }

    /// Stops the measurement process. This only has an effect If the measurement
    /// frequency is not zero.
    pub fn stop(&mut self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0,
            MEASURE_TEMP: 0
        );
    }

    /// This powers down the temperature sensor.
    pub fn power_down(&mut self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0,
            POWER_DOWN: 1
        );
    }

    /// This powers up the temperature sensor.
    pub fn power_up(&mut self) {
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE0,
            POWER_DOWN: 0
        );
    }

    /// Set the temperature that will generate a low alarm, high alarm, and panic alarm interrupt
    /// when the temperature exceeded this values.
    pub fn set_alarm_values(&mut self, low_alarm: f32, high_alarm: f32, panic_alarm: f32) {
        let low_alarm = self.decode(low_alarm);
        let high_alarm = self.decode(high_alarm);
        let panic_alarm = self.decode(panic_alarm);
        ral::write_reg!(ral::tempmon, self.base, TEMPSENSE0, ALARM_VALUE: high_alarm);
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
    /// returns (low_alarm_temp, high_alarm_temp, panic_alarm_temp)
    pub fn alarm_values(&mut self) -> (f32, f32, f32) {
        let high_alarm = ral::read_reg!(ral::tempmon, self.base, TEMPSENSE0, ALARM_VALUE);
        let (low_alarm, panic_alarm) = ral::read_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE2,
            LOW_ALARM_VALUE,
            PANIC_ALARM_VALUE
        );
        (
            self.convert(low_alarm as u16),
            self.convert(high_alarm as u16),
            self.convert(panic_alarm as u16),
        )
    }

    /// enables interrupts for the selected alarm
    ///
    /// NOTE: Make sure that you implemented the TEMP_LOW_HIGH or TEMP_PANIC interrupt handler
    ///
    /// NOTE: TEMP_LOW_HIGH is triggered for `TempSensor low` and `TempSensor high`
    pub fn enable_interrupts(&mut self, low_high_alarm: bool, panic_alarm: bool) {
        if low_high_alarm {
            unsafe { cortex_m::peripheral::NVIC::unmask(ral::interrupt::TEMP_LOW_HIGH) };
        }
        if panic_alarm {
            unsafe { cortex_m::peripheral::NVIC::unmask(ral::interrupt::TEMP_PANIC) };
        }
    }

    /// Disable interrupts for the selected alarm
    pub fn disable_interrupts(low_high_alarm: bool, panic_alarm: bool) {
        if low_high_alarm {
            cortex_m::peripheral::NVIC::mask(ral::interrupt::TEMP_LOW_HIGH);
        }
        if panic_alarm {
            cortex_m::peripheral::NVIC::mask(ral::interrupt::TEMP_PANIC);
        }
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
        ral::write_reg!(
            ral::tempmon,
            self.base,
            TEMPSENSE1,
            MEASURE_FREQ: measure_freq as u32
        );
    }
}
