//! I2C pad configuration

/// Tag that indicates the SCL signal
pub enum SCL {}
/// Tag that indicates the SDA signal
pub enum SDA {}

/// An I2C signal; one of `SCL` or `SDA`
pub trait Signal: private::Sealed {}

impl Signal for SCL {}
impl Signal for SDA {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::SCL {}
    impl Sealed for super::SDA {}
}

/// An I2C pin
pub trait Pin: super::IOMUX {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: super::Daisy;
    /// I2C Signal
    type Signal: Signal;
    /// I2C module; `U2` for `I2C2`
    type Module: super::consts::Unsigned;
}

/// Prepare an I2C pin
///
/// If you do not call `prepare()` on your I2C pin, it might not work as a I2C
/// pin.
///
/// # Safety
///
/// `prepare()` inherits all the unsafety that comes from the `IOMUX` supertrait.
/// In particular, we cannot be sure that the implementation's pointers are correct.
/// It may also write a daisy configuration that's incorrect.
pub unsafe fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    P::DAISY.write();
}
