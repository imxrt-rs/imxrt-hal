//! Supporting traits for defining peripheral DMA
//! sources and destinations
//!
//! # How to add DMA capabilities to your peripheral
//!
//! 1. Make sure your device can support DMA transfers!
//! 2. If your device can either produce data for a DMA transfer, or
//!    accept data from a DMA transfer, implement the `private::Sealed`
//!    trait for your peripheral *in the `private` module*.
//! 3. If your device can produce data for a DMA transfer, implement the
//!    `Source` trait for your peripheral *in your peripheral's module*.
//! 4. If your device can accept data from a DMA transfer, implement the
//!    `Destination` trait for your peripheral *in your peripheral's module*.
//!
//! See the [Rust API guidelines on future-proofing](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
//! to learn about the 'Sealed' pattern. Use the UART peripheral as an example.

use super::Element;

/// Describes a peripheral that can be the source of DMA data
///
/// By 'source,' we mean that it provides data for a DMA transfer.
/// A 'source,' would be a hardware device sending data into our
/// memory.
///
/// This trait may only be implemented by HAL authors. Users will find
/// that [HAL peripherals already implement `Source`](trait.Source.html#implementors).
pub trait Source<E: Element>: private::Sealed {
    type Error;
    /// Peripheral source request signal
    ///
    /// See Table 4-3 of the reference manual. A source probably
    /// has something like 'receive' in the name.
    const SOURCE_REQUEST_SIGNAL: u32;
    /// Returns a pointer to the register from which the DMA channel
    /// reads data
    ///
    /// This is the register that software reads to acquire data from
    /// a device. The type of the pointer describes the type of reads
    /// the DMA channel performs when transferring data.
    ///
    /// This memory is assumed to be static.
    fn source(&self) -> *const E;
    /// Perform any actions necessary to enable DMA transfers
    ///
    /// Callers use this method to put the peripheral in a state where
    /// it can supply the DMA channel with data.
    fn enable_source(&mut self) -> Result<(), Self::Error>;
    /// Perform any actions necessary to disable or cancel DMA transfers
    ///
    /// This may include undoing the actions in `enable_source()`.
    fn disable_source(&mut self);
}

/// Describes a peripheral that can be the destination for DMA data
///
/// By 'destination,' we mean that it receives data from a DMA transfer.
/// Software is sending data from memory to a device using DMA.
///
/// The trait may only be implemented by HAL authors. Users will find
/// that [HAL peripherals already implement `Destination`](trait.Destination.html#implementors).
pub trait Destination<E: Element>: private::Sealed {
    type Error;
    /// Peripheral destination request signal
    ///
    /// See Table 4-3 of the reference manual. A destination probably
    /// has something like 'transfer' in the name.
    const DESTINATION_REQUEST_SIGNAL: u32;
    /// Returns a pointer to the register into which the DMA channel
    /// writes data
    ///
    /// This is the register that software writes to when sending data to a
    /// device. The type of the pointer describes the type of reads the
    /// DMA channel performs when transferring data.
    fn destination(&self) -> *const E;
    /// Perform any actions necessary to enable DMA transfers
    ///
    /// Callers use this method to put the peripheral into a state where
    /// it can accept transfers from a DMA channel.
    fn enable_destination(&mut self) -> Result<(), Self::Error>;
    /// Perform any actions necessary to disable or cancel DMA transfers
    ///
    /// This may include undoing the actions in `enable_destination()`.
    fn disable_destination(&mut self);
}

/// Preventing crate users from implementing `Source` and `Destination`
/// for arbitrary types.
mod private {
    pub trait Sealed {}

    use crate::uart;
    impl<M> Sealed for uart::UART<M> where M: uart::module::Module {}

    use crate::spi;
    impl<M> Sealed for spi::SPI<M> where M: spi::module::Module {}
}
