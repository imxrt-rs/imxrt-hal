//! SPI pad configurations

/// A SPI signal
pub trait Signal: private::Sealed {}

/// A tag that indicates a SPI clock pad
pub enum SCK {}
/// A tag that indicates a SPI data out pad
pub enum SDO {}
/// A tag that indicates a SPI data in pad
pub enum SDI {}
/// A tag that indicates a SPI chip select pad
pub enum PCS0 {}

impl Signal for SCK {}
impl Signal for SDO {}
impl Signal for SDI {}
impl Signal for PCS0 {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::SCK {}
    impl Sealed for super::SDO {}
    impl Sealed for super::SDI {}
    impl Sealed for super::PCS0 {}
}

/// A SPI pin
pub trait Pin: super::IOMUX {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: super::Daisy;
    /// SPI signal
    type Signal: Signal;
    /// SPI module; `U3` for `SPI3`
    type Module: super::consts::Unsigned;
}

/// Prepare a SPI pin
///
/// If you do not call `prepare()` on your SPI pin, it might work as
/// a SPI pin.
///
/// # Safety
///
/// `prepare()` inherits all the unsafety that comes from the `IOMUX` supertrait.
pub unsafe fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    P::DAISY.write();
}
