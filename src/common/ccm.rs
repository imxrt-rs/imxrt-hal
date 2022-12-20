use crate::iomuxc;

/// Frequency (Hz) of the crystal oscillator.
pub const XTAL_OSCILLATOR_HZ: u32 = 24_000_000;

/// A CCM clock output pin.
///
/// This adapter configures the IOMUXC pad, and protects the pad
/// from accidentally being used for anything else. It does not
/// enable / disable anything in the CCM to enable the output. Use the functions
/// in the [`output_source`](crate::ccm::output_source) module
/// to the CLKO settings.
pub struct Output<P> {
    pin: P,
}

impl<P, const N: u8> Output<P>
where
    P: iomuxc::ccm::Pin<Function = iomuxc::ccm::Observable<N>>,
{
    /// Create the output pin.
    #[inline]
    pub fn new(mut pin: P) -> Self {
        iomuxc::ccm::prepare(&mut pin);
        Self { pin }
    }

    /// Release the output pin.
    #[inline]
    pub fn release(self) -> P {
        self.pin
    }
}
