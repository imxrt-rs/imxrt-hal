use super::Element;

/// A peripheral that can be the source of DMA data
///
/// By 'source,' we mean that it provides data for a DMA transfer.
/// A source would be a hardware device writing data into memory,
/// like a UART receiver.
///
/// # Safety
///
/// `Source` should only be implemented on peripherals that are
/// DMA capable. This trait should be implemented by HAL authors
/// who are exposing DMA capable peripherals.
///
/// The `enable_source` and `disable_source` methods may have
/// interior mutability. The implementer must ensure that any modifications
/// are atomic. The `Source` consumer will assume that `enable_source` and
/// `disable_source` are correct, and that they are safe to call.
pub unsafe trait Source<E: Element> {
    /// Peripheral source request signal
    ///
    /// See Table 4-3 of the reference manual. A source may
    /// has a qualifier like 'receive' in the name.
    fn source_signal(&self) -> u32;
    /// Returns a pointer to the register from which the DMA channel
    /// reads data
    ///
    /// This is the register that software reads to acquire data from
    /// a device. The type of the pointer describes the type of reads
    /// the DMA channel performs when transferring data.
    ///
    /// This memory is assumed to be static. Repeated `source` calls
    /// should always return the same address.
    fn source(&self) -> *const E;
    /// Perform any actions necessary to enable DMA transfers
    ///
    /// Callers use this method to put the peripheral in a state where
    /// it can supply the DMA channel with data. This method may change
    /// the peripheral's state, and the implementation will ensure that
    /// any mutations are atomic.
    fn enable_source(&self);
    /// Perform any actions necessary to disable or cancel DMA transfers
    ///
    /// This may include undoing the actions in `enable_source`. This
    /// method may change the peripheral's state, and the implementation
    /// will ensure that any mutations are atomic.
    fn disable_source(&self);
}

/// A peripheral that can be the destination for DMA data
///
/// By 'destination,' we mean that it receives data from a DMA transfer.
/// A destination would be a peripheral that could send data out of
/// processor memory, like a UART transmitter.
///
/// # Safety
///
/// `Destination` should only be implemented on peripherals that are
/// DMA capable. This trait should be implemented by HAL authors
/// who are exposing DMA capable peripherals.
///
/// The `enable_destination` and `disable_destination` methods may have
/// interior mutability. The implementer must ensure that any modifications
/// are atomic. The `Source` consumer will assume that `enable_destination` and
/// `disable_destination` are correct, and that they are safe to call.
pub unsafe trait Destination<E: Element> {
    /// Peripheral destination request signal
    ///
    /// See Table 4-3 of the reference manual. A destination mave
    /// has a qualifier like 'transfer' in the name.
    fn destination_signal(&self) -> u32;
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
    /// it can accept transfers from a DMA channel. This method may change
    /// the peripheral's state, and the implementation will ensure that
    /// any mutations are atomic.
    fn enable_destination(&self);
    /// Perform any actions necessary to disable or cancel DMA transfers
    ///
    /// This may include undoing the actions in `enable_destination`.  This
    /// method may change the peripheral's state, and the implementation
    /// will ensure that any mutations are atomic.
    fn disable_destination(&self);
}
