//! ADC pad configuration

/// An ADC input bank, like `ADC1` and `ADC2`
pub trait ADC: private::Sealed {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::ADC1 {}
    impl Sealed for super::ADC2 {}
}

/// Indicates an ADC1-compatible pin
pub enum ADC1 {}
impl ADC for ADC1 {}

/// Indicates an ADC2-compatible pin
pub enum ADC2 {}
impl ADC for ADC2 {}

/// Describes an ADC input pin
///
/// ADC pins are specialized GPIO pins. Some pads may be used in both `ADC1`
/// and `ADC2`, so implementations will indicate their compatibility by
/// supplying an identifier in place of `ADCx`.
pub trait Pin<ADCx: ADC>: super::gpio::Pin {
    /// The input pin identifier
    ///
    /// Starts at `U0`, and increments up.
    type Input: super::consts::Unsigned;
}

/// Prepare an ADC pin
///
/// Due to a requirement in the ADC module, `prepare` will disable the pull/keeper
/// on the pin. The configuration change will not affect any other settings.
pub fn prepare_adc_pin<ADCx: ADC, P: Pin<ADCx>>(pin: &mut P) {
    // See the note in the ADC section of the reference manual
    // (using iMXRT1060, rev 2). ADC input signals connect to
    // GPIO, and we need to disable the keeper to prevent signal
    // jumps.
    super::alternate(pin, <P as super::gpio::Pin>::ALT);
    super::configure(
        pin,
        super::Config::modify().set_pull_keep(super::PullKeep::Disabled),
    );
}