//! DMA channel

use core::mem;

use crate::{
    element::Element,
    ral::{dma, dmamux, tcd::BandwidthControl, Static, DMA, MULTIPLEXER},
    ErrorStatus,
};

/// A DMA channel
///
/// You should rely on your HAL to allocate `Channel`s. If your HAL does not allocate channels,
/// or if you're desigining the HAL, use [`new`](#method.new) to create a new DMA channel.
pub struct Channel {
    /// Our channel number, expected to be between 0 to (CHANNEL_COUNT - 1)
    index: usize,
    /// Reference to the DMA registers
    registers: Static<dma::RegisterBlock>,
    /// Reference to the DMA multiplexer
    multiplexer: Static<dmamux::RegisterBlock>,
}

impl Channel {
    /// Set the channel's bandwidth control
    ///
    /// - `None` disables bandwidth control (default setting)
    /// - `Some(bwc)` sets the bandwidth control to `bwc`
    pub fn set_bandwidth_control(&mut self, bandwidth: Option<BandwidthControl>) {
        let raw = BandwidthControl::raw(bandwidth);
        let tcd = self.tcd();
        modify_reg!(crate::ral::tcd, tcd, CSR, BWC: raw);
    }

    /// Returns the DMA channel number
    ///
    /// Channels are unique and numbered within the half-open range `[0, CHANNEL_COUNT)`.
    pub fn channel(&self) -> usize {
        self.index
    }

    /// Creates a DMA channel
    ///
    /// # Safety
    ///
    /// This will create a handle that may alias global, mutable state.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to [`CHANNEL_COUNT`](constant.CHANNEL_COUNT.html).
    #[inline(always)]
    pub unsafe fn new(index: usize) -> Self {
        // TODO consider breaking the API and return `Option<Channel>`
        if index < super::CHANNEL_COUNT {
            Channel {
                index,
                registers: DMA,
                multiplexer: MULTIPLEXER,
            }
        } else {
            panic!("DMA channel index {} exceeds CHANNEL_COUNT", index);
        }
    }

    /// Reset the transfer control descriptor owned by the DMA channel
    ///
    /// `reset` should be called during channel initialization to put the
    /// channel into a known, good state.
    pub fn reset(&mut self) {
        self.tcd().reset();
    }

    /// Returns a handle to this channel's transfer control descriptor
    fn tcd(&self) -> &crate::ral::tcd::RegisterBlock {
        &self.registers.TCD[self.index]
    }

    /// Prepare the source of a transfer; see [`Transfer`](struct.Transfer.html) for details.
    ///
    /// # Safety
    ///
    /// User must ensure that the memory described by `Transfer` is valid for the lifetime of
    /// the DMA transaction.
    pub unsafe fn set_source_transfer<E: Element>(&mut self, transfer: &Transfer<E>) {
        let tcd = self.tcd();
        write_reg!(crate::ral::tcd, tcd, SADDR, transfer.address as u32);
        write_reg!(crate::ral::tcd, tcd, SOFF, transfer.offset);
        modify_reg!(crate::ral::tcd, tcd, ATTR, SSIZE: E::DATA_TRANSFER_ID, SMOD: transfer.modulo);
        write_reg!(
            crate::ral::tcd,
            tcd,
            SLAST,
            transfer.last_address_adjustment
        );
    }

    /// Prepare the destination for a transfer; see [`Transfer`](struct.Transfer.html) for details.
    ///
    /// # Safety
    ///
    /// User must ensure that the memory described by `Transfer` is valid for the lifetime of
    /// the DMA transaction.
    pub unsafe fn set_destination_transfer<E: Element>(&mut self, transfer: &Transfer<E>) {
        let tcd = self.tcd();
        write_reg!(crate::ral::tcd, tcd, DADDR, transfer.address as u32);
        write_reg!(crate::ral::tcd, tcd, DOFF, transfer.offset);
        modify_reg!(crate::ral::tcd, tcd, ATTR, DSIZE: E::DATA_TRANSFER_ID, DMOD: transfer.modulo);
        write_reg!(
            crate::ral::tcd,
            tcd,
            DLAST_SGA,
            transfer.last_address_adjustment
        );
    }

    /// Set the number of *bytes* to transfer per minor loop
    ///
    /// Describes how many bytes we should transfer for each DMA service request.
    pub fn set_minor_loop_bytes(&mut self, nbytes: u32) {
        let tcd = self.tcd();
        write_reg!(crate::ral::tcd, tcd, NBYTES, nbytes);
    }

    /// Se the number of elements to move in each minor loop
    ///
    /// Describes how many elements we should transfer for each DMA service request.
    pub fn set_minor_loop_elements<E: Element>(&mut self, len: usize) {
        self.set_minor_loop_bytes((mem::size_of::<E>() * len) as u32);
    }

    /// Tells the DMA channel how many transfer iterations to perform
    ///
    /// A 'transfer iteration' is a read from a source, and a write to a destination, with
    /// read and write sizes described by a minor loop. Each iteration requires a DMA
    /// service request, either from hardware or from software.
    pub fn set_transfer_iterations(&mut self, iterations: u16) {
        let tcd = self.tcd();
        write_reg!(crate::ral::tcd, tcd, CITER, iterations);
        write_reg!(crate::ral::tcd, tcd, BITER, iterations);
    }

    /// Enable or disabling triggering from hardware
    ///
    /// If source is `Some(value)`, we trigger from hardware identified by the source identifier.
    /// If `source` is `None`, we disable hardware triggering.
    pub fn set_trigger_from_hardware(&mut self, source: Option<u32>) {
        let chcfg = &self.multiplexer.chcfg[self.index];
        chcfg.write(0);
        if let Some(source) = source {
            chcfg.write(dmamux::RegisterBlock::ENBL | source);
        }
    }

    /// Set this DMA channel as always on
    ///
    /// Use `set_always_on()` so that the DMA multiplexer drives the transfer with no
    /// throttling. Specifically, an "always-on" transfer will not need explicit re-activiation
    /// between major loops.
    ///
    /// Use an always-on channel for memory-to-memory transfers, so that you don't need explicit
    /// software re-activation to maintain the transfer. On the other hand, most peripheral transfers
    /// should not use an always-on channel, since the peripheral should control the data flow through
    /// explicit activation.
    pub fn set_always_on(&mut self) {
        let chcfg = &self.multiplexer.chcfg[self.index];
        chcfg.write(0);
        chcfg.write(dmamux::RegisterBlock::ENBL | dmamux::RegisterBlock::A_ON);
    }

    /// Returns `true` if the DMA channel is receiving a service signal from hardware
    pub fn is_hardware_signaling(&self) -> bool {
        self.registers.HRS.read() & (1 << self.index) != 0
    }

    /// Enable or disable the DMA's multiplexer request
    ///
    /// In this DMA implementation, all peripheral transfers and memcpy requests
    /// go through the DMA multiplexer. So, this needs to be set for the multiplexer
    /// to service the channel.
    pub fn set_enable(&mut self, enable: bool) {
        if enable {
            self.registers.SERQ.write(self.index as u8);
        } else {
            self.registers.CERQ.write(self.index as u8);
        }
    }

    /// Returns `true` if this DMA channel generated an interrupt
    pub fn is_interrupt(&self) -> bool {
        self.registers.INT.read() & (1 << self.index) != 0
    }

    /// Clear the interrupt flag from this DMA channel
    pub fn clear_interrupt(&mut self) {
        self.registers.CINT.write(self.index as u8);
    }

    /// Enable or disable 'disable on completion'
    ///
    /// 'Disable on completion' lets the DMA channel automatically clear the request signal
    /// when it completes a transfer.
    pub fn set_disable_on_completion(&mut self, dreq: bool) {
        let tcd = self.tcd();
        modify_reg!(crate::ral::tcd, tcd, CSR, DREQ: dreq as u16);
    }

    /// Enable or disable interrupt generation when the transfer completes
    ///
    /// You're responsible for registering your interrupt handler.
    pub fn set_interrupt_on_completion(&mut self, intr: bool) {
        let tcd = self.tcd();
        modify_reg!(crate::ral::tcd, tcd, CSR, INTMAJOR: intr as u16);
    }

    /// Indicates if the DMA transfer has completed
    pub fn is_complete(&self) -> bool {
        let tcd = self.tcd();
        read_reg!(crate::ral::tcd, tcd, CSR, DONE == 1)
    }

    /// Clears completion indication
    pub fn clear_complete(&mut self) {
        self.registers.CDNE.write(self.index as u8);
    }

    /// Indicates if the DMA channel is in an error state
    pub fn is_error(&self) -> bool {
        self.registers.ERR.read() & (1 << self.index) != 0
    }

    /// Clears the error flag
    pub fn clear_error(&mut self) {
        self.registers.CERR.write(self.index as u8);
    }

    /// Indicates if this DMA channel is actively transferring data
    pub fn is_active(&self) -> bool {
        let tcd = self.tcd();
        read_reg!(crate::ral::tcd, tcd, CSR, ACTIVE == 1)
    }

    /// Indicates if this DMA channel is enabled
    pub fn is_enabled(&self) -> bool {
        self.registers.ERQ.read() & (1 << self.index) != 0
    }

    /// Returns the value from the **global** error status register
    ///
    /// It may reflect the last channel that produced an error, and that
    /// may not be related to this channel.
    pub fn error_status(&self) -> ErrorStatus {
        ErrorStatus::new(self.registers.ES.read())
    }

    /// Start a DMA transfer
    ///
    /// `start()` should be used to request service from the DMA controller. It's
    /// necessary for in-memory DMA transfers. Do not use it for hardware-initiated
    /// DMA transfers. DMA transfers that involve hardware will rely on the hardware
    /// to request DMA service.
    ///
    /// Flag is automatically cleared by hardware after it's asserted.
    pub fn start(&mut self) {
        self.registers.SSRT.write(self.index as u8);
    }
}

/// Describes a DMA transfer
///
/// `Transfer` describes a source or a destination of a DMA transfer. A source or destination
/// could be
///
/// - a hardware register
/// - an element buffer that's treated as linear memory
/// - an element buffer that's treated as a circular buffer
///
/// A transfer that uses a circular buffer requires that the buffer size is a power of two.
///
/// It's always safe to create a `Transfer`, because the struct is inert. But, it's generally
/// unsafe to use `Transfer` in other methods. You must make sure that the memory described by
/// `Transfer` is valid for the lifetime of the DMA transaction.
#[derive(Debug)]
pub struct Transfer<E: Element> {
    /// The starting address for the DMA transfer
    ///
    /// If this describes a source, `address` will be the first
    /// address read. If this describes a destination, `address`
    /// will be the first address written.
    address: *const E,

    /// Offsets to perform for each read / write of a memory address.
    ///
    /// When defining a transfer for a peripheral source or destination,
    /// `offset` should be zero. Otherwise, `offset` should represent the
    /// size of the data element, `E`.
    ///
    /// Negative (backwards) adjustments are permitted, if you'd like to read
    /// a buffer backwards or something.
    offset: i16,

    /* size: u16, // Not needed; captured in E: Element type */
    /// Defines the strategy for reading / writing linear or circular buffers
    ///
    /// `modulo` should be zero if this definition defines a transfer from linear
    /// memory or a peripheral. `modulo` will be non-zero when defining a transfer
    /// from a circular buffer. The non-zero value is the number of high bits to freeze
    /// when performing address offsets (see `offset`). Given that we're only supporting
    /// power-of-two buffer sizes, `modulo` will be `31 - clz(cap * sizeof(E))`, where `cap` is the
    /// total size of the circular buffer, `clz` is "count leading zeros," and `sizeof(E)` is
    /// the size of the element, in bytes.
    modulo: u16,

    /// Perform any last-address adjustments when we complete the transfer
    ///
    /// Once we complete moving data from a linear buffer, we should set our pointer back to the
    /// initial address. For this case, `last_address_adjustment` should be a negative number that
    /// describes how may *bytes* to move backwards from our current address to reach our starting
    /// address. Alternatively, it could describe how to move to a completely new address, like
    /// a nearby buffer that we're using for a double-buffer. Or, set it to zero, which means "keep
    /// your current position." "Keep your current position" is important when working with a
    /// peripheral address!
    last_address_adjustment: i32,
}

/// Describes an error when creating a transfer for a circular buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircularError {
    /// The size of the memory is not a power of two
    NotPowerOfTwo,
    /// The alignment of the buffer must be a multiple of the buffer's
    /// size, which includes both element type, and the length of the buffer.
    IncorrectAlignment,
}

impl<E: Element> Transfer<E> {
    /// Defines a transfer that reads from a hardware register at `address`
    pub fn hardware(address: *const E) -> Self {
        Transfer {
            address,
            // Don't move the address pointer
            offset: 0,
            // We're not a circular buffer
            modulo: 0,
            // Don't move the address pointer
            last_address_adjustment: 0,
        }
    }

    /// Defines a transfer that can read from or write to `buffer`
    ///
    /// `ptr` points to the starting element of the buffer. `len` indicates how many elements
    /// you intend on transferring.
    pub fn buffer_linear(ptr: *const E) -> Self {
        Transfer {
            address: ptr,
            offset: core::mem::size_of::<E>() as i16,
            modulo: 0,
            // This crate's API doesn't guarantee that we'll reset the pointer.
            // Callers will always need to re-call `set_[source|destination]_transfer`
            // to specify a starting addess.
            last_address_adjustment: 0,
        }
    }

    /// Defines a transfer that can read from or write to the circular buffer
    ///
    /// `start` points to the first element that will be used in the transfer. `capacity`
    /// is the total size of the allocated memory region for the transfer; it is **not**
    /// the number of elements to transfer.
    pub fn buffer_circular(start: *const E, capacity: usize) -> Result<Self, CircularError> {
        if !capacity.is_power_of_two() {
            return Err(CircularError::NotPowerOfTwo);
        } else if (start as usize) % (capacity * mem::size_of::<E>()) != 0 {
            return Err(CircularError::IncorrectAlignment);
        }
        let modulo = 31 - (capacity * mem::size_of::<E>()).leading_zeros() as u16;
        Ok(Transfer {
            address: start,
            offset: core::mem::size_of::<E>() as i16,
            modulo,
            last_address_adjustment: 0,
        })
    }
}

// It's OK to send a channel across an execution context.
// They can't be cloned or copied, so there's no chance of
// them being (mutably) shared.
unsafe impl Send for Channel {}
