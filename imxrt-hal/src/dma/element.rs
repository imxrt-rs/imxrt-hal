//! Trait to define acceptable DMA transfer elements

/// Describes a transferrable DMA element; basically, an unsigned
/// integer of any size.
pub trait Element: private::Sealed {
    /// An identifier describing the data transfer size
    ///
    /// Part of the TCD API; see documentation on TCD[SSIZE]
    /// and TCD[DSIZE] for more information.
    const DATA_TRANSFER_ID: u16;
}

impl Element for u8 {
    const DATA_TRANSFER_ID: u16 = 0;
}

impl Element for u16 {
    const DATA_TRANSFER_ID: u16 = 1;
}

impl Element for u32 {
    const DATA_TRANSFER_ID: u16 = 2;
}

impl Element for u64 {
    const DATA_TRANSFER_ID: u16 = 3;
}

mod private {
    pub trait Sealed {}
    use super::Element;
    impl<E> Sealed for E where E: Element {}
}
