//! An interface for defining and configuring i.MX RT pads
//!
//! `imxrt-iomuxc` provides traits for defining and configuring i.MX RT
//! processor pads. A 'pad' is the physical input / output on an i.MX RT processor.
//! Pads may be configured for various functions. A pad may act as a UART pin, an I2C
//! pin, or other types of pins. A 'pin' is a pad that's configured for a functional
//! purpose. The traits let us say which pad can be used for which peripheral pin.
//!
//! Developers who write hardware abstraction layers (HALs) for i.MX RT processors may
//! use the `imxrt-iomuxc` traits in their APIs. HAL implementers may also expose all
//! the processor's pads for HAL users. The approach lets users treat pads as resources
//! which will be consumed and used by processor peripherals.
//!
//! Processor pads are defined in one or more processor-specific crates. The `imxrt106x-iomuxc`
//! crate is one example of an processor-specific crate. The crate defines all processor pads,
//! and provides implementations of the `imxrt-iomuxc` traits.
//!
//! # Safety
//!
//! Many traits, methods, and functions exposed by the `imxrt-iomuxc` crate are `unsafe`. See
//! the trait and function documentation for more details. Most end-users aren't expected to
//! use this `unsafe` interface. Rather, HAL implementers should build a safer inteface that
//! focuses on peripheral-level configuration, rather than pad- and pin-level configuration.
//!
//! # Public API
//!
//! Any accessible *and documented* symbol is considered part of this crate's public, user-facing
//! API. Any public *but not documented* symbol is exposed for i.MX RT IOMUXC crate implementers, and is
//! not for typical users. If a symbol is marked as `#[doc(hidden)]`, or explicitly
//! documented as "do not use," it should not be used by typical users.
//!
//! # Design Guidance
//!
//! For recommendations on how you can use these traits, see the module-level documentation. The
//! rest of this section describes general guidance for designing APIs with these traits.
//!
//! ## Type-Erased Pads
//!
//! At the expense of requiring `unsafe`, users may favor type-erased pads over strongly-typed pads.
//! When creating APIs that consume strongly-typed pads, or pads that conform to peripheral pin interfaces,
//! consider supporting an `unsafe` API to create the peripheral without requiring the strongly-typed pads.
//! The API will expect that the user is responsible for manually configuring the type-erased pad.
//!
//! ```no_run
//! # imxrt_iomuxc::define_pad!();
//! use imxrt_iomuxc::uart::{Pin, TX, RX};
//! # pub struct UART;
//!
//! impl UART {
//!     pub fn new<T, R>(mut tx: T, mut rx: R, /* ... */) -> UART
//!     where
//!         T: Pin<Direction = TX>,
//!         R: Pin<Direction = RX, Module = <T as Pin>::Module>,
//!     {
//!         // Peripheral implementer resposible for calling unsafe functions
//!         unsafe {
//!             imxrt_iomuxc::uart::prepare(&mut tx);
//!             imxrt_iomuxc::uart::prepare(&mut rx);
//!         }
//!         // ...
//!         # UART
//!     }
//!
//!     pub unsafe fn new_unchecked(tx: ErasedPad, rx: ErasedPad, /* ... */) -> UART {
//!         // ...
//!         # UART
//!     }
//! }
//! # struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
//! # type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
//! # type AD_B0_04 = Pad<AD_B0, imxrt_iomuxc::consts::U4>;
//! # impl imxrt_iomuxc::uart::Pin for AD_B0_03 { const ALT: u32 = 0; type Direction = imxrt_iomuxc::uart::TX; type Module = imxrt_iomuxc::consts::U1; const DAISY: Option<imxrt_iomuxc::Daisy> = None; }
//! # impl imxrt_iomuxc::uart::Pin for AD_B0_04 { const ALT: u32 = 0; type Direction = imxrt_iomuxc::uart::RX; type Module = imxrt_iomuxc::consts::U1; const DAISY: Option<imxrt_iomuxc::Daisy> = None; }
//!
//! // Preferred: create a UART peripheral with strongly-typed pads...
//! let ad_b0_03 = unsafe { AD_B0_03::new() };
//! let ad_b0_04 = unsafe { AD_B0_04::new() };
//! let uart1 = UART::new(ad_b0_03, ad_b0_04);
//!
//! // Optional: create a UART peripheral from type-erased pads...
//! let ad_b0_03 = unsafe { AD_B0_03::new() };
//! let ad_b0_04 = unsafe { AD_B0_04::new() };
//!
//! let mut tx_pad = ad_b0_03.erase();
//! let mut rx_pad = ad_b0_04.erase();
//!
//! // User is responsible for configuring the pad,
//! // since we can't call `prepare()` on the pad...
//! unsafe {
//!     // Daisy registers and values aren't attached
//!     // to erased pads, so we have to reference this
//!     // manually.
//!     <AD_B0_03 as imxrt_iomuxc::uart::Pin>::DAISY.map(|daisy| daisy.write());
//!     <AD_B0_04 as imxrt_iomuxc::uart::Pin>::DAISY.map(|daisy| daisy.write());
//!     imxrt_iomuxc::alternate(&mut tx_pad, 2);
//!     imxrt_iomuxc::alternate(&mut rx_pad, 2);
//!     imxrt_iomuxc::clear_sion(&mut tx_pad);
//!     imxrt_iomuxc::clear_sion(&mut rx_pad);
//! }
//! // Pads are configured for UART settings
//! let uart1 = unsafe { UART::new_unchecked(tx_pad, rx_pad) };
//! ```

#![no_std]

mod config;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod uart;

use core::ptr;

pub use config::{
    configure, Config, DriveStrength, Hysteresis, OpenDrain, PullKeep, PullKeepSelect, PullUpDown,
    SlewRate, Speed,
};

/// Type-level constants and traits
///
/// Re-exported from the [`typenum` crate](https://crates.io/crates/typenum), but scoped for the requirements
/// of the IOMUXC peripheral.
pub mod consts {
    pub use typenum::consts::{
        U0, U1, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U2, U20, U21, U22, U23, U24, U25,
        U26, U27, U28, U29, U3, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39, U4, U40, U41, U5,
        U6, U7, U8, U9,
    };
    pub use typenum::Unsigned;
}

/// A pad group base
///
/// A 'Base' defines the start of similarly-named pads, like `AD_B0`. `Base`s
/// provide access to a multiplexer register base and a pad configuration register
/// base.
///
/// This trait is for developers who are preparing processor-specific crates that implement
/// the `imxrt-iomuxc` traits. **Do not** implement this trait if you are an end user.
///
/// # Safety
///
/// You must ensure that the two pointers are correct for your processor.
#[doc(hidden)]
pub unsafe trait Base {
    /// Starting register for a multiplexer register
    ///
    /// For the `AD_B0` base, this would be the MUX register of `AD_B0_00`.
    fn mux_base() -> *mut u32;
    /// Starting register for a pad configuration register
    ///
    /// For the `AD_B0` base, this would be the PAD register of `AD_B0_00`.
    fn pad_base() -> *mut u32;
}

/// Define an IOMUXC base
///
/// `base_name` is the name of the IOMUXC register base. For something like
/// `AD_B0_03`, the base is `AD_B0`.
///
/// `mux_base` is a `u32` that represents the base's mux address. For the IOMUXC
/// registers starting with `AD_B0`, this is the mux address of `AD_B0_00`.
///
/// `pad_base` is a `u32` that represents the base's pad address. For the IOMUXC
/// registers starting with `AD_B0`, this is the pad address of `AD_B0_00`.
///
/// For use only by iomuxc crate implementers.
#[doc(hidden)]
#[macro_export]
macro_rules! define_base {
    ($base_name: ident, $mux_base: expr, $pad_base: expr) => {
        #[allow(non_camel_case_types)] // Conform with reference manual
        #[derive(Debug)]
        pub struct $base_name;

        unsafe impl imxrt_iomuxc::Base for $base_name {
            fn mux_base() -> *mut u32 {
                $mux_base as *mut u32
            }
            fn pad_base() -> *mut u32 {
                $pad_base as *mut u32
            }
        }
    };
}

/// An IOMUXC-capable pad which can support I/O multiplexing
///
/// **Do not** implement this trait yourself. The crate will provide a blanket implementation
/// for all pads.
pub unsafe trait IOMUX {
    /// Returns the absolute address of the multiplex register
    ///
    /// # Safety
    ///
    /// Returns a pointer to an address that may be mutably aliased elsewhere.
    unsafe fn mux(&mut self) -> *mut u32;
    /// Returns the absolute address of the pad configuration register
    ///
    /// # Safety
    ///
    /// Returns a pointer to an address that may be mutably aliased elsewhere.
    unsafe fn pad(&mut self) -> *mut u32;
}

const SION_BIT: u32 = 1 << 4;

/// Set the SION bit in a pad's MUX register
///
/// Users who are using strongly-typed pads should not call `set_sion()` directly.
/// Instead, `set_sion()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `set_sion()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
///
/// # Safety
///
/// We can't guarantee that the pointer to the pad's mux register is correct.
#[inline(always)]
pub unsafe fn set_sion<I: IOMUX>(pad: &mut I) {
    let mut mux = ptr::read_volatile(pad.mux());
    mux |= SION_BIT;
    ptr::write_volatile(pad.mux(), mux);
}

/// Clear the SION bit in a pad's MUX register
///
/// Users who are using strongly-typed pads should not call `clear_sion()` directly.
/// Instead, `clear_sion()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `clear_sion()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
///
/// # Safety
///
/// We can't guarantee that the pointer to the pad's mux register is correct.
#[inline(always)]
pub unsafe fn clear_sion<I: IOMUX>(pad: &mut I) {
    let mut mux = ptr::read_volatile(pad.mux());
    mux &= !SION_BIT;
    ptr::write_volatile(pad.mux(), mux);
}

/// Set an alternate value for the pad
///
/// Users who are using strongly-typed pads should not call `alternate()` directly.
/// Instead, `alternate()` will be used in a peripheral's `prepare()` function as needed,
/// so that you don't have to call it.
///
/// However, you should use `alternate()` if you're using any type-erased pads, since those
/// pads cannot be used with a peripheral's `prepare()` function.
///
/// # Safety
///
/// We can't guarantee that the pointer to the pad's mux register is correct.
#[inline(always)]
pub unsafe fn alternate<I: IOMUX>(pad: &mut I, alt: u32) {
    const ALT_MASK: u32 = 0b1111;
    let mut mux = ptr::read_volatile(pad.mux());
    mux = (mux & !ALT_MASK) | (alt & ALT_MASK);
    ptr::write_volatile(pad.mux(), mux);
}

/// Defines the `Pad` struct, and provides an implementation of the
/// `IOMUX` trait
///
/// All implementation crates must call `define_pad!()` in their top-level module.
#[doc(hidden)]
#[macro_export]
macro_rules! define_pad {
    () => {
        /// An i.MXT RT pad
        ///
        /// The `Base` is the pad tag, like `AD_B0`. The `Offset` is the
        /// constant (type) that describes the pad number.
        ///
        /// `Pad`s have no size.
        #[derive(Debug)]
        pub struct Pad<Base, Offset> {
            base: ::core::marker::PhantomData<Base>,
            offset: ::core::marker::PhantomData<Offset>,
            // Block auto-implement of Send / Sync. We'll manually implement
            // the traits as necessary.
            _not_send_sync: ::core::marker::PhantomData<*const ()>,
        }

        impl<Base, Offset> Pad<Base, Offset> {
            /// Creates a handle to the pad
            ///
            /// # Safety
            ///
            /// `new()` may be called anywhere, by anyone. This could lead to multiple objects that
            /// mutate the same memory. Consider calling `new()` once, near startup, then passing objects
            /// and references throughout your program.
            #[inline(always)]
            pub const unsafe fn new() -> Self {
                Self {
                    base: ::core::marker::PhantomData,
                    offset: ::core::marker::PhantomData,
                    _not_send_sync: ::core::marker::PhantomData,
                }
            }
        }

        unsafe impl<Base, Offset> Send for Pad<Base, Offset>
        where
            Base: Send,
            Offset: Send,
        {
        }

        impl<Base, Offset> Pad<Base, Offset>
        where
            Base: imxrt_iomuxc::Base,
            Offset: imxrt_iomuxc::consts::Unsigned,
        {
            /// Erase the pad's type, returning an `ErasedPad`
            #[inline(always)]
            pub fn erase(self) -> ErasedPad {
                ErasedPad {
                    mux_base: Base::mux_base(),
                    pad_base: Base::pad_base(),
                    offset: Offset::USIZE,
                }
            }
        }

        unsafe impl<Base, Offset> imxrt_iomuxc::IOMUX for Pad<Base, Offset>
        where
            Base: imxrt_iomuxc::Base,
            Offset: imxrt_iomuxc::consts::Unsigned,
        {
            /// # Safety
            ///
            /// Returns a pointer to an address that may be mutably aliased elsewhere.
            #[inline(always)]
            unsafe fn mux(&mut self) -> *mut u32 {
                Base::mux_base().add(Offset::USIZE)
            }

            /// # Safety
            ///
            /// Returns a pointer to an address that may be mutably aliased elsewhere.
            #[inline(always)]
            unsafe fn pad(&mut self) -> *mut u32 {
                Base::pad_base().add(Offset::USIZE)
            }
        }

        /// A pad that has its type erased
        ///
        /// `ErasedPad` moves the pad state to run time, rather than compile time.
        /// The type may provide more flexibility for some APIs. Each `ErasedPad` is
        /// three pointers large.
        ///
        /// `ErasedPad` may be converted back into their strongly-typed analogs using
        /// `TryFrom` and `TryInto` conversions.
        ///
        /// ```no_run
        /// use imxrt_iomuxc as iomuxc;
        /// # iomuxc::define_pad!();
        /// # struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
        /// # type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
        /// let ad_b0_03 = unsafe { AD_B0_03::new() };
        /// let mut erased = ad_b0_03.erase();
        ///
        /// // Erased pads may be manually manipulated
        /// unsafe { iomuxc::alternate(&mut erased, 7) };
        /// unsafe { iomuxc::set_sion(&mut erased) };
        ///
        /// // Try to convert the erased pad back to its strongly-typed counterpart
        /// use core::convert::TryFrom;
        /// let ad_b0_03 = AD_B0_03::try_from(erased).unwrap();
        /// ```
        #[derive(Debug)]
        pub struct ErasedPad {
            mux_base: *mut u32,
            pad_base: *mut u32,
            offset: usize,
        }

        unsafe impl imxrt_iomuxc::IOMUX for ErasedPad {
            /// # Safety
            ///
            /// Returns a pointer to an address that may be mutably aliased elsewhere.
            #[inline(always)]
            unsafe fn mux(&mut self) -> *mut u32 {
                self.mux_base.add(self.offset)
            }

            /// # Safety
            ///
            /// Returns a pointer to an address that may be mutably aliased elsewhere.
            #[inline(always)]
            unsafe fn pad(&mut self) -> *mut u32 {
                self.pad_base.add(self.offset)
            }
        }

        unsafe impl Send for ErasedPad {}

        /// An error that indicates the conversion from an `ErasedPad` to a
        /// strongly-typed pad failed.
        ///
        /// Failure happens when trying to convert an `ErasedPad` into the incorrect
        /// pad. The error indicator wraps the pad that failed to convert.
        #[derive(Debug)]
        pub struct WrongPadError(pub ErasedPad);

        impl<Base, Offset> ::core::convert::TryFrom<ErasedPad> for Pad<Base, Offset>
        where
            Base: imxrt_iomuxc::Base,
            Offset: imxrt_iomuxc::consts::Unsigned,
        {
            type Error = WrongPadError;
            fn try_from(erased_pad: ErasedPad) -> Result<Self, Self::Error> {
                if erased_pad.mux_base == Base::mux_base()
                    && erased_pad.pad_base == Base::pad_base()
                    && erased_pad.offset == Offset::USIZE
                {
                    Ok(unsafe { Self::new() })
                } else {
                    Err(WrongPadError(erased_pad))
                }
            }
        }
    };
}

/// A daisy selection
///
/// A daisy chain specifies which pad will be used for a peripheral's
/// input. Call `write()` to commit the settings described by a `Daisy`
/// value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Daisy {
    reg: *mut u32,
    value: u32,
}

impl Daisy {
    /// Create a new select input that, when utilized, will write
    /// `value` into `reg`
    #[doc(hidden)]
    pub const fn new(reg: *mut u32, value: u32) -> Self {
        Daisy { reg, value }
    }

    /// Commit the settings defined by this `Daisy` value to the hardware
    ///
    /// # Safety
    ///
    /// We cannot be sure that the `reg` pointer created during construction is
    /// correct. This modifies a global, processor register, so the typical
    /// rules around mutable static memory apply.
    #[inline(always)]
    pub unsafe fn write(self) {
        ptr::write_volatile(self.reg, self.value);
    }
}

/// Use these snippets in documentation to create a fake example pad, `AD_B0_03.
///
/// You should use `no_run` to prevent execution, or you'll probably derefence a null pointer.
/// ```
/// # use imxrt_iomuxc::IOMUX; #[allow(non_camel_case_types)] pub struct AD_B0_03;
/// # impl AD_B0_03 { unsafe fn new() -> Self { Self } fn ptr(&self) -> *mut u32 { core::ptr::null_mut() }}
/// # unsafe impl IOMUX for AD_B0_03 { unsafe fn mux(&mut self) -> *mut u32 { self.ptr() } unsafe fn pad(&mut self) -> *mut u32 { self.ptr() }}
/// ```
#[cfg(doctest)]
pub struct DocPadSnippet;

/// GPIO pad configuration
pub mod gpio {
    /// A GPIO pin
    pub trait Pin: super::IOMUX {
        /// The alternate value for this pad
        const ALT: u32;
        /// The GPIO module; `U5` for `GPIO5`
        type Module: super::consts::Unsigned;
        /// The offset; `U13` for `GPIO5_IO13`
        type Offset: super::consts::Unsigned;
    }

    /// Prepare a pad to be used as a GPIO pin
    ///
    /// # Safety
    ///
    /// `prepare()` inherits all the unsafety of the `IOMUX` supertrait.
    pub unsafe fn prepare<P: Pin>(pin: &mut P) {
        super::alternate(pin, P::ALT);
    }
}

#[cfg(test)]
mod tests {
    use crate as imxrt_iomuxc;
    super::define_pad!();

    use crate::consts::{U0, U1};

    #[derive(Debug)]
    struct TestBase;

    unsafe impl crate::Base for TestBase {
        fn mux_base() -> *mut u32 {
            static mut MEM: u32 = 0;
            unsafe { &mut MEM as *mut u32 }
        }
        fn pad_base() -> *mut u32 {
            static mut MEM: u32 = 0;
            unsafe { &mut MEM as *mut u32 }
        }
    }

    type TestPad = Pad<TestBase, U0>;

    #[test]
    fn erased_pad_convert_success() {
        let pad = unsafe { TestPad::new() };
        let erased = pad.erase();

        use core::convert::TryFrom;
        TestPad::try_from(erased).expect("This is the test pad");
    }

    #[test]
    fn erased_pad_convert_fail() {
        let pad = unsafe { TestPad::new() };
        let erased = pad.erase();

        use core::convert::TryFrom;
        type OtherPad = Pad<TestBase, U1>;
        OtherPad::try_from(erased).expect_err("This is a different pad");
    }
}

/// ```
/// imxrt_iomuxc::define_pad!();
/// fn is_send<S: Send>(s: S) {}
/// struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
/// type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
/// is_send(unsafe { AD_B0_03::new() }.erase());
/// ```
#[cfg(doctest)]
struct ErasedPadsAreSend;

/// ```
/// imxrt_iomuxc::define_pad!();
/// fn is_send<S: Send>(s: S) {}
/// struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
/// type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
/// is_send(unsafe { AD_B0_03::new() });
/// is_send(unsafe { AD_B0_03::new() }.erase());
/// ```
#[cfg(doctest)]
struct PadsAreSend;

/// ```compile_fail
/// imxrt_iomuxc::define_pad!();
/// fn is_sync<S: Sync>(s: S) {}
/// struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
/// type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
/// is_sync(unsafe { AD_B0_03::new() }.erase())
/// ```
#[cfg(doctest)]
struct ErasedPadsAreNotSync;

/// ```compile_fail
/// imxrt_iomuxc::define_pad!();
/// fn is_sync<S: Sync>(s: S) {}
/// struct AD_B0; unsafe impl imxrt_iomuxc::Base for AD_B0 { fn mux_base() -> *mut u32 { 0 as *mut u32 } fn pad_base() -> *mut u32 { 0 as *mut u32 } }
/// type AD_B0_03 = Pad<AD_B0, imxrt_iomuxc::consts::U3>;
/// is_sync(unsafe { AD_B0_03::new() })
/// ```
#[cfg(doctest)]
struct PadsAreNotSync;
