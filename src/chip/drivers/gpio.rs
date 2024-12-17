//! General purpose I/O.
//!
//! Create a [`Port`](Port) over a RAL GPIO instance. Then, use the `Port` to
//! allocate GPIO outputs and inputs.
//!
//! Use [`Output`](Output) to drive GPIO outputs. Use [`Input`](Input) to read
//! GPIO pin states, and trigger interrupts when GPIO states change.
//!
//! # Interior mutability
//!
//! Methods on `Output` and `Input` take immutable references, `&self`. The hardware
//! guarantees that these operations can occur without data races. Methods that
//! require multiple operations on a register are implemented on the `Port`, and
//! take the GPIO by reference.
//!
//! # Example
//!
//! ```no_run
//! use imxrt_hal::gpio::Port;
//! use imxrt_ral::gpio::GPIO2;
//!
//! let mut gpio2 = Port::new(unsafe { GPIO2::instance() });
//! let gpio_b0_04 = // Handle to GPIO_B0_04 IOMUXC pin, provided by BSP or higher-level HAL...
//!     # unsafe { imxrt_iomuxc::imxrt1060::gpio_b0::GPIO_B0_04::new() };
//!
//! let output = gpio2.output(gpio_b0_04);
//! output.set();
//! output.clear();
//! output.toggle();
//!
//! let input = gpio2.input(output.release());
//! assert!(input.is_set());
//! ```
//! # TODO
//!
//! - Fast GPIOs

use crate::{iomuxc, ral};

/// GPIO ports.
pub struct Port<const N: u8> {
    gpio: ral::gpio::Instance<N>,
}

impl<const N: u8> Port<N> {
    /// Create a GPIO port that can allocate and convert GPIOs.
    pub fn new(gpio: ral::gpio::Instance<N>) -> Self {
        Self { gpio }
    }

    fn register_block(&self) -> &'static ral::gpio::RegisterBlock {
        let register_block: &ral::gpio::RegisterBlock = &self.gpio;
        // Safety: points to peripheral memory, which is static.
        // Gpio implementation guarantees that memory which needs
        // mutable access to shared GPIO registers passes through
        // the Port type.
        let register_block: &'static ral::gpio::RegisterBlock =
            unsafe { core::mem::transmute(register_block) };
        register_block
    }

    /// Allocate an output GPIO.
    pub fn output<P>(&mut self, mut pin: P) -> Output<P>
    where
        P: iomuxc::gpio::Pin<N>,
    {
        iomuxc::gpio::prepare(&mut pin);
        Output::new(pin, self.register_block(), P::OFFSET)
    }

    /// Allocate an input GPIO.
    pub fn input<P>(&mut self, mut pin: P) -> Input<P>
    where
        P: iomuxc::gpio::Pin<N>,
    {
        iomuxc::gpio::prepare(&mut pin);
        Input::new(pin, self.register_block(), P::OFFSET)
    }

    /// Enable or disable GPIO input interrupts.
    ///
    /// Specify `None` to disable interrupts. Or, provide a trigger
    /// to configure the interrupt.
    pub fn set_interrupt<P>(&mut self, pin: &Input<P>, trigger: Option<Trigger>) {
        self.set_interrupt_enable(pin, false);
        if let Some(trigger) = trigger {
            self.set_interrupt_trigger(pin, trigger);
            self.set_interrupt_enable(pin, true);
        }
    }

    /// Set the GPIO input interrupt trigger for the provided input pin.
    fn set_interrupt_trigger<P>(&mut self, pin: &Input<P>, trigger: Trigger) {
        if Trigger::EitherEdge == trigger {
            ral::modify_reg!(ral::gpio, self.gpio, EDGE_SEL, |edge_sel| {
                edge_sel | pin.mask()
            });
        } else {
            ral::modify_reg!(ral::gpio, self.gpio, EDGE_SEL, |edge_sel| {
                edge_sel & !pin.mask()
            });
            let icr = trigger as u32;
            let icr_modify = |reg| reg & !(0b11 << pin.icr_offset()) | (icr << pin.icr_offset());
            if pin.offset < 16 {
                ral::modify_reg!(ral::gpio, self.gpio, ICR1, icr_modify);
            } else {
                ral::modify_reg!(ral::gpio, self.gpio, ICR2, icr_modify);
            }
        }
    }

    /// Enable (`true`) or disable (`false`) interrupt generation.
    fn set_interrupt_enable<P>(&mut self, pin: &Input<P>, enable: bool) {
        if enable {
            ral::modify_reg!(ral::gpio, self.gpio, IMR, |imr| imr | pin.mask());
        } else {
            ral::modify_reg!(ral::gpio, self.gpio, IMR, |imr| imr & !pin.mask());
        }
    }
}

/// An output GPIO.
pub struct Output<P> {
    pin: P,
    // Logical ownership:
    // - DR: read only
    // - PSR: read only
    // - DR_SET, DR_CLEAR, DR_TOGGLE: write 1 to set value in DR
    gpio: &'static ral::gpio::RegisterBlock,
    offset: u32,
}

// Safety: an output pin is safe to send across execution contexts,
// because it points to static memory.
unsafe impl<P: Send> Send for Output<P> {}

impl<P> Output<P> {
    fn new(pin: P, gpio: &'static ral::gpio::RegisterBlock, offset: u32) -> Self {
        let output = Self { pin, gpio, offset };
        ral::modify_reg!(ral::gpio, gpio, GDIR, |gdir| gdir | output.mask());
        output
    }

    const fn mask(&self) -> u32 {
        1 << self.offset
    }

    /// Set the GPIO high.
    pub fn set(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_SET, self.mask());
    }

    /// Set the GPIO low.
    pub fn clear(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_CLEAR, self.mask());
    }

    /// Alternate the GPIO pin output.
    ///
    /// `toggle` is implemented in hardware, so it will be more efficient
    /// than implementing in software.
    pub fn toggle(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_TOGGLE, self.mask());
    }

    /// Returns `true` if the GPIO is set.
    pub fn is_set(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, DR) & self.mask() != 0
    }

    /// Returns `true` if the value of the pad is high.
    ///
    /// Can differ from [`is_set()`](Self::is_set), especially in an open drain config.
    pub fn is_pad_high(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, PSR) & self.mask() != 0
    }

    /// Release the underlying pin object.
    pub fn release(self) -> P {
        self.pin
    }

    /// Access the underlying pin.
    pub fn pin(&self) -> &P {
        &self.pin
    }

    /// Mutably access the underling pin.
    pub fn pin_mut(&mut self) -> &mut P {
        &mut self.pin
    }
}

impl Output<()> {
    /// Allocate an output GPIO without a pin.
    ///
    /// Prefer using [`Port::output`](Port::output) to create a GPIO output with a
    /// pin resource. That method ensures that pin resources are managed throughout
    /// your program, and that the pin is configured to operate as a GPIO output.
    ///
    /// You may use this method to allocate duplicate `Output` object for the same
    /// physical GPIO output. This is considered safe, since the `Output` API is
    /// reentrant.
    ///
    /// If you use this constructor, you're responsible for configuring the IOMUX
    /// multiplexer register.
    pub fn without_pin<const N: u8>(port: &mut Port<N>, offset: u32) -> Self {
        Self::new((), port.register_block(), offset)
    }
}

/// An input GPIO.
pub struct Input<P> {
    pin: P,
    // Logical ownership:
    // - PSR: read only
    // - ISR: read, W1C
    gpio: &'static ral::gpio::RegisterBlock,
    offset: u32,
}

// Safety: see impl Send for Output.
unsafe impl<P: Send> Send for Input<P> {}

/// Input interrupt triggers.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Trigger {
    /// Interrupt when GPIO is low
    Low = 0,
    /// Interrupt when GPIO is high
    High = 1,
    /// Interrupt after GPIO rising edge
    RisingEdge = 2,
    /// Interrupt after GPIO falling edge
    FallingEdge = 3,
    /// Interrupt after either a rising or falling edge
    EitherEdge = 4,
}

impl<P> Input<P> {
    fn new(pin: P, gpio: &'static ral::gpio::RegisterBlock, offset: u32) -> Self {
        let input = Self { pin, gpio, offset };
        ral::modify_reg!(ral::gpio, gpio, GDIR, |gdir| gdir & !input.mask());
        input
    }

    const fn mask(&self) -> u32 {
        1 << self.offset
    }

    const fn icr_offset(&self) -> u32 {
        (self.offset % 16) * 2
    }

    /// Returns `true` if the GPIO is set high.
    pub fn is_set(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, PSR) & self.mask() != 0
    }

    /// Returns `true` if the GPIO interrupt has triggered.
    pub fn is_triggered(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, ISR) & self.mask() != 0
    }

    /// Clear the interrupt triggered flag.
    pub fn clear_triggered(&self) {
        // Atomic write; OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, ISR, self.mask());
    }

    /// Indicates if interrupts are enabled for this input.
    pub fn is_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, IMR) & self.mask() != 0
    }

    /// Release the underlying pin object.
    pub fn release(self) -> P {
        self.pin
    }

    /// Access the underlying pin.
    pub fn pin(&self) -> &P {
        &self.pin
    }

    /// Mutably access the underling pin.
    pub fn pin_mut(&mut self) -> &mut P {
        &mut self.pin
    }
}

impl Input<()> {
    /// Allocate an input GPIO without a pin.
    ///
    /// Prefer using [`Port::input`](Port::input) to create a GPIO input with a
    /// pin resource. That method ensures that pin resources are managed throughout
    /// your program, and that the pin is configured to operate as a GPIO input.
    ///
    /// You may use this method to allocate duplicate `Input` object for the same
    /// physical GPIO input. This is considered safe, since the `Input` API is
    /// reentrant. Any non-reentrant methods are attached to [`Port`], which cannot
    /// be constructed without an `unsafe` constructor of the register block.
    ///
    /// If you use this constructor, you're responsible for configuring the IOMUX
    /// multiplexer register.
    pub fn without_pin<const N: u8>(port: &mut Port<N>, offset: u32) -> Self {
        Self::new((), port.register_block(), offset)
    }
}

impl<P> eh02::digital::v2::OutputPin for Output<P> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set();
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.clear();
        Ok(())
    }
}

#[cfg(feature = "eh02-unproven")]
impl<P> eh02::digital::v2::StatefulOutputPin for Output<P> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set())
    }
}

#[cfg(feature = "eh02-unproven")]
impl<P> eh02::digital::v2::ToggleableOutputPin for Output<P> {
    type Error = core::convert::Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Output::<P>::toggle(self);
        Ok(())
    }
}

#[cfg(feature = "eh02-unproven")]
impl<P> eh02::digital::v2::InputPin for Input<P> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set())
    }
}

impl<P> eh1::digital::ErrorType for Output<P> {
    type Error = core::convert::Infallible;
}

impl<P> eh1::digital::OutputPin for Output<P> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set(self);
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::clear(self);
        Ok(())
    }
}

impl<P> eh1::digital::StatefulOutputPin for Output<P> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Output::is_set(self))
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Output::toggle(self);
        Ok(())
    }
}

// For open drain or simply reading back the actual state
// of the pin.
impl<P> eh1::digital::InputPin for Output<P> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_pad_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Output::is_pad_high(self))
    }
}

impl<P> eh1::digital::ErrorType for Input<P> {
    type Error = core::convert::Infallible;
}

impl<P> eh1::digital::InputPin for Input<P> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_set(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!Input::is_set(self))
    }
}
