//! PWM pad configuration

/// A PWM output identified; one of `A` or `B`
pub trait Output: private::Sealed {}
/// PWM output A
pub enum A {}
/// PWM output B
pub enum B {}

impl Output for A {}
impl Output for B {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::A {}
    impl Sealed for super::B {}
}

/// A PWM pin
pub trait Pin: super::IOMUX {
    /// The alternate mode for the PWM pin
    const ALT: u32;
    /// The output identifier
    type Output: Output;
    /// The PWM module; `U2` is `PWM2`
    type Module: super::consts::Unsigned;
    /// The PWM submodule; `U3` for `PWM2_SM3`
    type Submodule: super::consts::Unsigned;
}

/// Prepare a PWM pin
///
/// # Safety
///
/// `prepare()` inherits all the unsafety of the `IOMUX` supertrait.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
}
