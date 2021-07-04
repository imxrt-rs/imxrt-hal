//! GPIOs
//!
//! This GPIO driver supports the `embedded_hal`'s `v2` digital traits.
//!
//! # Fast and Normal GPIOs
//!
//! High speed, or "fast," GPIOs are GPIOs that run on the AHB clock. Normal GPIOs
//! run on the IPG clock.
//!
//! # Example
//!
//! ```no_run
//! use imxrt_hal::{self, gpio::GPIO};
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//! let input = GPIO::new(peripherals.iomuxc.ad_b0.p11);
//!
//! assert!(!input.is_set());
//! let mut output = input.output();
//!
//! output.set();
//! assert!(output.is_set());
//!
//! output.toggle();
//! assert!(!output.is_set());
//!
//! assert!(output.set_fast(true));
//! output.toggle();
//! assert!(output.is_set());
//! ```

use crate::iomuxc::{consts::Unsigned, gpio::Pin};
use crate::ral::{
    self,
    gpio::{self, RegisterBlock},
};
use core::marker::PhantomData;

/// Denotes that a pin is configured as an input
pub enum Input {}
/// Denotes that a pin is configured as an output
pub enum Output {}

pub struct GPIO<P, D> {
    pin: P,
    dir: PhantomData<D>,
}

impl<P, D> GPIO<P, D>
where
    P: Pin,
{
    fn register_block(&self) -> *const RegisterBlock {
        const REGISTER_BLOCKS: [*const RegisterBlock; 9] = [
            gpio::GPIO1,
            gpio::GPIO2,
            gpio::GPIO3,
            gpio::GPIO4,
            gpio::GPIO5,
            gpio::GPIO6,
            gpio::GPIO7,
            gpio::GPIO8,
            gpio::GPIO9,
        ];
        REGISTER_BLOCKS[self.module().wrapping_sub(1)]
    }

    /// Returns the bitmask for this GPIO
    #[inline(always)]
    fn mask(&self) -> u32 {
        1u32 << <P as Pin>::Offset::USIZE
    }

    /// Returns the ICR field offset for this GPIO
    ///
    /// ICR is "Interrupt Configuration Register"
    fn icr_offset(&self) -> usize {
        (<P as Pin>::Offset::USIZE % 16) * 2
    }

    /// The return is a non-zero number, since the GPIO identifiers
    /// start with '1.'
    #[inline(always)]
    fn module(&self) -> usize {
        let fast_offset = if self.is_fast() { 5 } else { 0 };
        <P as Pin>::Module::USIZE + fast_offset
    }

    fn gpr(&self) -> Option<*mut u32> {
        // GPR register for GPIO1
        const GPR26: *mut u32 = 0x400A_C068 as *mut u32;
        if <P as Pin>::Module::USIZE < 5 {
            // Safety: GPR registers in range for GPIO1, 2, 3, and 4
            Some(unsafe { GPR26.add(<P as Pin>::Module::USIZE.wrapping_sub(1)) })
        } else {
            None
        }
    }

    /// Returns `true` if the GPIO is configured for fast mode
    ///
    /// If the GPIO cannot support fast mode, `is_fast()` always returns `false`
    pub fn is_fast(&self) -> bool {
        // Safety: MMIO valid per gpr() function.
        // Read is atomic
        self.gpr()
            .map(|gpr| unsafe { core::ptr::read_volatile(gpr) & self.mask() != 0 })
            .unwrap_or(false)
    }

    /// Configures the GPIO to fast mode. `true` indicates "fast," and `false` indicates "normal."
    ///
    /// Returns `false` if this pin does not support fast mode. Otherwise, returns `true`, indicating
    /// that the setting was respected.
    ///
    /// If you transition an output pin into fast mode, the GPIO output state may *not* be maintained.
    /// That is, if your GPIO pin was high, a transition into fast mode may set the pin low. Consider
    /// setting fast mode before driving the GPIO output to avoid inconsistencies.
    pub fn set_fast(&mut self, fast: bool) -> bool {
        self.gpr()
            .map(|gpr| {
                cortex_m::interrupt::free(|_| unsafe {
                    let v = core::ptr::read_volatile(gpr);

                    let from = self.register_block();

                    if fast {
                        core::ptr::write_volatile(gpr, v | self.mask());
                    } else {
                        core::ptr::write_volatile(gpr, v & !self.mask());
                    }

                    let to = self.register_block();
                    self.copy_settings(from, to);

                    true
                })
            })
            .unwrap_or(false)
    }

    /// Copies the settings for one GPIO register block, `from`, to another register block, `to`.
    ///
    /// This method runs when changing from a normal to a fast GPIO, or a fast to a normal GPIO.
    /// The goal is to make the switch seamless for the end user. You must only copy the settings
    /// for the current GPIO pin.
    ///
    /// # Safety
    ///
    /// This method must run in a critical section, since it performs reads and writes across
    /// multiple register blocks which are shared across multiple pins.
    unsafe fn copy_settings(&self, from: *const RegisterBlock, to: *const RegisterBlock) {
        macro_rules! copy_bits {
            ($REG:ident, $mask:expr) => {{
                let source_register = ral::read_reg!(ral::gpio, from, $REG);
                let target_bits = $mask & source_register;

                ral::modify_reg!(ral::gpio, to, $REG, |mut destination| {
                    // Set the bit low...
                    destination &= !$mask;
                    // OR in the previous setting...
                    destination | target_bits
                });
            }};
        }

        // The input / output direction. When switching across fast / normal, keep the
        // same direction.
        copy_bits!(GDIR, self.mask());

        // Interrupt configuration is preserved when switching fast / normal.
        if <P as Pin>::Offset::USIZE < 16 {
            copy_bits!(ICR1, self.mask());
        } else {
            copy_bits!(ICR2, self.mask());
        }
        copy_bits!(EDGE_SEL, self.mask());
        copy_bits!(IMR, self.mask());
    }

    /// Configure the GPIO as an output
    fn set_output(&self, _: &cortex_m::interrupt::CriticalSection) {
        // Safety: critical section, enforced by API, ensures consistency
        unsafe {
            // Turn off interrupts for pin.
            ral::modify_reg!(ral::gpio, self.register_block(), IMR, |imr| imr
                & !self.mask());

            // Change direction
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                | self.mask());
        }
    }

    /// Configure the GPIO as an input
    fn set_input(&self, _: &cortex_m::interrupt::CriticalSection) {
        // Safety: critical section, enforced by API, ensures consistency
        unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                & !self.mask());
        }
    }
}

impl<P> GPIO<P, Input>
where
    P: Pin,
{
    /// Create a GPIO from a pad that supports a GPIO configuration
    ///
    /// All pads may be used as a GPIO, so this should always work.
    pub fn new(mut pin: P) -> Self {
        crate::iomuxc::gpio::prepare(&mut pin);
        Self {
            pin,
            dir: PhantomData,
        }
    }

    /// Set the GPIO as an output.
    ///
    /// Any interrupt configuration will be cleared and needs redoing if the pin is transitioned
    /// back to an input.
    pub fn output(self) -> GPIO<P, Output> {
        cortex_m::interrupt::free(|cs| self.set_output(cs));
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Returns `true` if this input pin is high
    pub fn is_set(&self) -> bool {
        // Safety: read is atomic
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), PSR) & self.mask() != 0 }
    }
}

/// GPIO input interrupt configurations.
///
/// These configurations do not take effect until
/// GPIO input interrupts are enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InterruptConfiguration {
    LowLevel = 0,
    HighLevel = 1,
    RisingEdge = 2,
    FallingEdge = 3,
    EitherEdge = 4,
}

impl<P> GPIO<P, Input>
where
    P: Pin,
{
    /// Enable (`true`) or disable (`false`) interrupts for this GPIO input.
    pub fn set_interrupt_enable(&mut self, enable: bool) {
        cortex_m::interrupt::free(|_| unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), IMR, |imr| if enable {
                imr | self.mask()
            } else {
                imr & !self.mask()
            })
        });
    }

    /// Indicates if interrupts are (`true`) or are not (`false`) enabled for this GPIO input.
    pub fn is_interrupt_enabled(&self) -> bool {
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), IMR) & self.mask() != 0u32 }
    }

    /// Set the interrupt configuration for this GPIO input.
    pub fn set_interrupt_configuration(&mut self, interrupt_configuration: InterruptConfiguration) {
        // Safety: These modify_reg! must be completed as one unit, or we get an inconsistent state.
        cortex_m::interrupt::free(|_| unsafe {
            if InterruptConfiguration::EitherEdge == interrupt_configuration {
                ral::modify_reg!(ral::gpio, self.register_block(), EDGE_SEL, |edge_sel| {
                    edge_sel | self.mask()
                });
            } else {
                ral::modify_reg!(ral::gpio, self.register_block(), EDGE_SEL, |edge_sel| {
                    edge_sel & !self.mask()
                });
                let icr = interrupt_configuration as u32;
                let icr_offset = self.icr_offset();
                let icr_modify = |reg| reg & !(0b11 << icr_offset) | (icr << icr_offset);
                if <P as Pin>::Offset::USIZE < 16 {
                    ral::modify_reg!(ral::gpio, self.register_block(), ICR1, icr_modify);
                } else {
                    ral::modify_reg!(ral::gpio, self.register_block(), ICR2, icr_modify);
                }
            }
        });
    }

    /// Indicates whether this GPIO input triggered an interrupt.
    pub fn is_interrupt_status(&self) -> bool {
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), ISR) & self.mask() != 032 }
    }

    /// Clear the interrupt status flag.
    pub fn clear_interrupt_status(&mut self) {
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), ISR, self.mask()) }
    }
}

impl<P> GPIO<P, Output>
where
    P: Pin,
{
    /// Transition the pin back to an input
    pub fn input(self) -> GPIO<P, Input> {
        cortex_m::interrupt::free(|cs| self.set_input(cs));
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Set the GPIO high
    pub fn set(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_SET, self.mask()) };
    }

    /// Set the GPIO low
    pub fn clear(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_CLEAR, self.mask()) };
    }

    /// Returns `true` if the pin is high
    pub fn is_set(&self) -> bool {
        // Safety: atomic read
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), DR) & self.mask() != 0u32 }
    }

    /// Alternate the state of the pin
    pub fn toggle(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_TOGGLE, self.mask()) }
    }
}

use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

impl<P> OutputPin for GPIO<P, Output>
where
    P: Pin,
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

impl<P> StatefulOutputPin for GPIO<P, Output>
where
    P: Pin,
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_set_high().map(|res| !res)
    }
}

impl<P> ToggleableOutputPin for GPIO<P, Output>
where
    P: Pin,
{
    type Error = core::convert::Infallible;
    fn toggle(&mut self) -> Result<(), Self::Error> {
        GPIO::<P, Output>::toggle(self);
        Ok(())
    }
}

impl<P> InputPin for GPIO<P, Input>
where
    P: Pin,
{
    type Error = core::convert::Infallible;
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|res| !res)
    }
}
