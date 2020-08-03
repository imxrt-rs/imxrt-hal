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

    #[inline(always)]
    fn offset(&self) -> u32 {
        1u32 << <P as Pin>::Offset::USIZE
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
            .map(|gpr| unsafe { core::ptr::read_volatile(gpr) & self.offset() != 0 })
            .unwrap_or(false)
    }

    /// Configures the GPIO to fast mode. `true` indicates "fast," and `false` indicates "normal."
    ///
    /// Returns `false` if this pin does not support fast mode. Otherwise, returns `true`, indicating
    /// that the setting was respected.
    ///
    /// Note that the pin may be in a new state after transitioning into fast mode. Consider setting
    /// and maintaining the fast mode setting before relying on the pin.
    pub fn set_fast(&mut self, fast: bool) -> bool {
        self.gpr()
            .map(|gpr| {
                cortex_m::interrupt::free(|_| unsafe {
                    let v = core::ptr::read_volatile(gpr);

                    if fast {
                        core::ptr::write_volatile(gpr, v | self.offset());
                    } else {
                        core::ptr::write_volatile(gpr, v & !self.offset());
                    }

                    true
                })
            })
            .unwrap_or(false)
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

    /// Set the GPIO as an output
    pub fn output(self) -> GPIO<P, Output> {
        // Safety: critical section ensures consistency
        cortex_m::interrupt::free(|_| unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                | self.offset());
        });
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Returns `true` if this input pin is high
    pub fn is_set(&self) -> bool {
        // Safety: read is atomic
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), PSR) & self.offset() != 0 }
    }
}

impl<P> GPIO<P, Output>
where
    P: Pin,
{
    /// Transition the pin back to an input
    pub fn input(self) -> GPIO<P, Input> {
        // Safety: critical section ensures consistency
        cortex_m::interrupt::free(|_| unsafe {
            ral::modify_reg!(ral::gpio, self.register_block(), GDIR, |gdir| gdir
                & !self.offset());
        });
        GPIO {
            pin: self.pin,
            dir: PhantomData,
        }
    }

    /// Set the GPIO high
    pub fn set(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_SET, self.offset()) };
    }

    /// Set the GPIO low
    pub fn clear(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_CLEAR, self.offset()) };
    }

    /// Returns `true` if the pin is high
    pub fn is_set(&self) -> bool {
        // Safety: atomic read
        unsafe { ral::read_reg!(ral::gpio, self.register_block(), DR) & self.offset() != 0u32 }
    }

    /// Alternate the state of the pin
    pub fn toggle(&mut self) {
        // Safety: atomic write
        unsafe { ral::write_reg!(ral::gpio, self.register_block(), DR_TOGGLE, self.offset()) }
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
