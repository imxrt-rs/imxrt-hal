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
//! let mut gpio2 = GPIO2::take().map(Port::new).unwrap();
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
        let register_block: &ral::gpio::RegisterBlock = &*self.gpio;
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
        P: iomuxc::gpio::Pin<Module = iomuxc::consts::Const<N>>,
    {
        iomuxc::gpio::prepare(&mut pin);
        ral::modify_reg!(ral::gpio, self.gpio, GDIR, |gdir| gdir | Output::<P>::MASK);
        Output {
            pin,
            gpio: self.register_block(),
        }
    }

    /// Allocate an input GPIO.
    pub fn input<P>(&mut self, mut pin: P) -> Input<P>
    where
        P: iomuxc::gpio::Pin<Module = iomuxc::consts::Const<N>>,
    {
        iomuxc::gpio::prepare(&mut pin);
        ral::modify_reg!(ral::gpio, self.gpio, GDIR, |gdir| gdir & !Input::<P>::MASK);
        Input {
            pin,
            gpio: self.register_block(),
        }
    }

    /// Enable or disable GPIO input interrupts.
    ///
    /// Specify `None` to disable interrupts. Or, provide a trigger
    /// to configure the interrupt.
    pub fn set_interrupt<P>(&mut self, pin: &Input<P>, trigger: Option<Trigger>)
    where
        P: iomuxc::gpio::Pin<Module = iomuxc::consts::Const<N>>,
    {
        self.set_interrupt_enable(pin, false);
        if let Some(trigger) = trigger {
            self.set_interrupt_trigger(pin, trigger);
            self.set_interrupt_enable(pin, true);
        }
    }

    /// Set the GPIO input interrupt trigger for the provided input pin.
    fn set_interrupt_trigger<P>(&mut self, _: &Input<P>, trigger: Trigger)
    where
        P: iomuxc::gpio::Pin<Module = iomuxc::consts::Const<N>>,
    {
        if Trigger::EitherEdge == trigger {
            ral::modify_reg!(ral::gpio, self.gpio, EDGE_SEL, |edge_sel| {
                edge_sel | Input::<P>::MASK
            });
        } else {
            ral::modify_reg!(ral::gpio, self.gpio, EDGE_SEL, |edge_sel| {
                edge_sel & !Input::<P>::MASK
            });
            let icr = trigger as u32;
            let icr_modify =
                |reg| reg & !(0b11 << Input::<P>::ICR_OFFSET) | (icr << Input::<P>::ICR_OFFSET);
            if Input::<P>::OFFSET < 16 {
                ral::modify_reg!(ral::gpio, self.gpio, ICR1, icr_modify);
            } else {
                ral::modify_reg!(ral::gpio, self.gpio, ICR2, icr_modify);
            }
        }
    }

    /// Enable (`true`) or disable (`false`) interrupt generation.
    fn set_interrupt_enable<P>(&mut self, _: &Input<P>, enable: bool)
    where
        P: iomuxc::gpio::Pin<Module = iomuxc::consts::Const<N>>,
    {
        if enable {
            ral::modify_reg!(ral::gpio, self.gpio, IMR, |imr| imr | Input::<P>::MASK);
        } else {
            ral::modify_reg!(ral::gpio, self.gpio, IMR, |imr| imr & !Input::<P>::MASK);
        }
    }
}

/// An output GPIO.
pub struct Output<P> {
    pin: P,
    // Logical ownership:
    // - DR: read only
    // - DR_SET, DR_CLEAR, DR_TOGGLE: write 1 to set value in DR
    gpio: &'static ral::gpio::RegisterBlock,
}

// Safety: an output pin is safe to send across execution contexts,
// because it points to static memory.
unsafe impl<P: Send> Send for Output<P> {}

impl<P> Output<P>
where
    P: iomuxc::gpio::Pin,
{
    // TODO(iomuxc) make this a constant?
    const OFFSET: u32 = <P::Offset as iomuxc::consts::Unsigned>::USIZE as u32;
    const MASK: u32 = 1 << Self::OFFSET;

    /// Set the GPIO high.
    pub fn set(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_SET, Self::MASK);
    }

    /// Set the GPIO low.
    pub fn clear(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_CLEAR, Self::MASK);
    }

    /// Alternate the GPIO pin output.
    ///
    /// `toggle` is implemented in hardware, so it will be more efficient
    /// than implementing in software.
    pub fn toggle(&self) {
        // Atomic write, OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, DR_TOGGLE, Self::MASK);
    }

    /// Returns `true` if the GPIO is set.
    pub fn is_set(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, DR) & Self::MASK != 0
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

/// An input GPIO.
pub struct Input<P> {
    pin: P,
    // Logical ownership:
    // - PSR: read only
    // - ISR: read, W1C
    gpio: &'static ral::gpio::RegisterBlock,
}

// Safety: see impl Send for Output.
unsafe impl<P: Send> Send for Input<P> {}

/// Input interrupt triggers.
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

impl<P> Input<P>
where
    P: iomuxc::gpio::Pin,
{
    const OFFSET: u32 = Output::<P>::OFFSET;
    const MASK: u32 = Output::<P>::MASK;
    const ICR_OFFSET: u32 = (Self::OFFSET % 16) * 2;

    /// Returns `true` if the GPIO is set high.
    pub fn is_set(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, PSR) & Self::MASK != 0
    }

    /// Returns `true` if the GPIO interrupt has triggered.
    pub fn is_triggered(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, ISR) & Self::MASK != 0
    }

    /// Clear the interrupt triggered flag.
    pub fn clear_triggered(&self) {
        // Atomic write; OK to take immutable reference.
        ral::write_reg!(ral::gpio, self.gpio, ISR, Self::MASK);
    }

    /// Indicates if interrupts are enabled for this input.
    pub fn is_interrupt_enabled(&self) -> bool {
        ral::read_reg!(ral::gpio, self.gpio, IMR) & Input::<P>::MASK != 0
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

impl<P> eh02::digital::v2::OutputPin for Output<P>
where
    P: iomuxc::gpio::Pin,
{
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
impl<P> eh02::digital::v2::StatefulOutputPin for Output<P>
where
    P: iomuxc::gpio::Pin,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set())
    }
}

#[cfg(feature = "eh02-unproven")]
impl<P> eh02::digital::v2::ToggleableOutputPin for Output<P>
where
    P: iomuxc::gpio::Pin,
{
    type Error = core::convert::Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Output::<P>::toggle(self);
        Ok(())
    }
}

#[cfg(feature = "eh02-unproven")]
impl<P> eh02::digital::v2::InputPin for Input<P>
where
    P: iomuxc::gpio::Pin,
{
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set())
    }
}
