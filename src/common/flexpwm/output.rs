//! PWM output pins.

use super::Channel;
use crate::iomuxc;

/// A PWM output pin.
///
/// Provides a simple interface for configuring a PWM pin, and for defining
/// the turn on and turn off count values. Use `new_a` or `new_b` to create
/// your PWM outputs (depending on the PWM pin's function).
///
/// The behaviors of the output pin depend on the submodule's configurations.
/// Make sure you configure your PWM output pair operations with
/// [`Submodule::set_pair_operation`](crate::flexpwm::Submodule::set_pair_operation).
/// to ensure this output functions as expected.
pub struct Output<P> {
    pin: P,
    channel: Channel,
}

impl<P, const N: u8, const M: u8> Output<P>
where
    P: iomuxc::flexpwm::Pin<
        Module = iomuxc::consts::Const<N>,
        Output = iomuxc::flexpwm::A,
        Submodule = iomuxc::consts::Const<M>,
    >,
{
    /// Create a PWM A channel output.
    pub fn new_a(pin: P) -> Self {
        Output::new(pin, Channel::A)
    }
}

impl<P, const N: u8, const M: u8> Output<P>
where
    P: iomuxc::flexpwm::Pin<
        Module = iomuxc::consts::Const<N>,
        Output = iomuxc::flexpwm::B,
        Submodule = iomuxc::consts::Const<M>,
    >,
{
    /// Create a PWM B channel output.
    pub fn new_b(pin: P) -> Self {
        Output::new(pin, Channel::B)
    }
}

impl<P, const N: u8, const M: u8> Output<P>
where
    P: iomuxc::flexpwm::Pin<
        Module = iomuxc::consts::Const<N>,
        Submodule = iomuxc::consts::Const<M>,
    >,
{
    const MASK: super::Mask = super::Submodule::<N, M>::MASK;
    fn new(mut pin: P, channel: Channel) -> Self {
        iomuxc::flexpwm::prepare(&mut pin);
        Output { pin, channel }
    }
    /// Release the PWM pin.
    ///
    /// This call disables the PWM output on the pin.
    pub fn release(self, pwm: &mut super::Pwm<N>) -> P {
        self.set_output_enable(pwm, false);
        self.pin
    }
    /// Indicates if this ouput is enabled.
    pub fn output_enable(&self, pwm: &super::Pwm<N>) -> bool {
        pwm.output_enable(self.channel).intersects(Self::MASK)
    }
    /// Enable or disable this output.
    pub fn set_output_enable(&self, pwm: &mut super::Pwm<N>, enable: bool) {
        let mut output_enable = pwm.output_enable(self.channel);
        output_enable.set(Self::MASK, enable);
        pwm.set_output_enable(self.channel, output_enable);
    }
    /// Returns the turn on counter value.
    ///
    /// When the PWM counter reaches this value, the output sets.
    pub fn turn_on(&self, sm: &super::Submodule<N, M>) -> i16 {
        sm.value(super::turn_on(self.channel))
    }
    /// Returns the turn off counter value.
    ///
    /// When the PWM counter reaches this value, the output clears.
    pub fn turn_off(&self, sm: &super::Submodule<N, M>) -> i16 {
        sm.value(super::turn_off(self.channel))
    }
    /// Set the turn on counter value.
    pub fn set_turn_on(&self, sm: &super::Submodule<N, M>, compare: i16) {
        sm.set_value(super::turn_on(self.channel), compare)
    }
    /// Set the turn off counter value.
    pub fn set_turn_off(&self, sm: &super::Submodule<N, M>, compare: i16) {
        sm.set_value(super::turn_off(self.channel), compare)
    }
}
