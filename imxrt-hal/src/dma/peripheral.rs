//! Supporting traits for defining peripheral DMA
//! sources and destinations

use super::Element;

/// Describes a peripheral that can be the source of DMA data
pub trait Source<E: Element>: source::Sealed<E> {
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
pub trait Destination<E: Element>: destination::Sealed<E> {
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

pub(crate) mod source {
    use super::{Element, Source};
    pub trait Sealed<E> {}
    impl<S, E> Sealed<E> for S
    where
        S: Source<E>,
        E: Element,
    {
    }
}

pub(crate) mod destination {
    use super::{Destination, Element};
    pub trait Sealed<E> {}
    impl<D, E> Sealed<E> for D
    where
        D: Destination<E>,
        E: Element,
    {
    }
}
