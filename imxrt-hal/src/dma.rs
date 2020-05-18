//! Direct Memory Access (DMA)
//!
//! We support
//!
//! - DMA from memory to a peripheral. Transfers may be uni- or bi-directional.
//!   See the [`Peripheral`](struct.Peripheral.html) for details.
//! - DMA memory copy, or memory-to-memory transfers. See [`Memcpy`](struct.Memcpy.html)
//!  for details.
//!
//! DMA types support either [`Linear`](struct.Linear.html) or [`Circular`](struct.Circular.html)
//! memory buffers. Either may be used as a DMA transfer source or destination. Both are backed
//! by statically-allocated [`Buffer`s](struct.Buffer.html). A user will create a memory buffer,
//! then pass ownership to a DMA type that defines a transfer. When the transfer completes, the
//! DMA type will release ownership back to the user.
//!
//! # Terms
//!
//! - *Source* is a location in memory that provides data. A source may be a buffer
//!    of data, or a peripheral register.
//! - *Destination* is a location in memory that will receive data.
//!   A destination may be a buffer of data, or a peripheral register.
//! - *Transfer* is an overloaded term, meaning either a DMA transfer, or the movement
//!   of data out of software, through a peripheral, to an external device.
//! - *DMA Transfer* is an operation achieved by the DMA controller to move data from a
//!   source to a destination.
//! - *Receive* means that we're moving data into software, through a peripheral, from an
//!   external device.
//!
//! # Example: Full-Duplex SPI Peripheral
//!
//! In this example, we prepare a SPI peripheral (SPI4) with two DMA
//! channels. One channel will send data; the other will receive data.
//! The example assumes that the user has registered a DMA interrupt
//! handler, since we're enabling an interrupt when the receive completes.
//!
//! ```no_run
//! use imxrt_hal::dma::{Circular, Buffer, Linear, Peripheral, ConfigBuilder, bidirectional_u16};
//!
//! // Two buffers that can support maximum receive and transfer sizes of 256 elements
//! static RX_BUFFER: Buffer<[u16; 256]> = Buffer::new([0; 256]);
//! static TX_BUFFER: Buffer<[u16; 256]> = Buffer::new([0; 256]);
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! //
//! // SPI setup...
//! //
//!
//! let (_, _, _, spi4_builder) = peripherals.spi.clock(
//!     &mut peripherals.ccm.handle,
//!     imxrt_hal::ccm::spi::ClockSelect::Pll2,
//!     imxrt_hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
//! );
//!
//! let mut spi4 = spi4_builder.build(
//!     peripherals.iomuxc.gpio_b0_02.alt3(),
//!     peripherals.iomuxc.gpio_b0_01.alt3(),
//!     peripherals.iomuxc.gpio_b0_03.alt3(),
//! );
//!
//! spi4.enable_chip_select_0(peripherals.iomuxc.gpio_b0_00.alt3());
//!
//! // Set the SPI clock speed, if desired...
//!
//! //
//! // DMA setup
//! //
//!
//! let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
//!
//! // i.MX RT DMA interrupt handlers manage pairs of DMA channels. There's one
//! // interrupt for DMA channel 9 and channel 25. By selecting these two
//! // DMA channels, we can register one interrupt to handle both DMA channel
//! // completion.
//! let tx_channel = dma_channels[9].take().unwrap();
//! let rx_channel = dma_channels[25].take().unwrap();
//!
//! // We only want to interrupt when the receive completes. When
//! // the receive completes, we know that we're also done transferring
//! // data.
//! let rx_config = ConfigBuilder::new()
//!     .interrupt_on_completion(true)
//!     .build();
//!
//! // The peripheral will transfer and receive u16 elements.
//! // It takes ownership of the SPI object, and the two DMA channels.
//! let mut peripheral = bidirectional_u16(
//!     spi4,
//!     (tx_channel, ConfigBuilder::new().build()),
//!     (rx_channel, rx_config),
//! );
//!
//! // Create DMA memory adapters over the statically-allocated DMA memory.
//! // These adapters will 'own' the statically-allocated memory. See the
//! // Linear and Circular docs for more information.
//! let mut tx_buffer = Circular::new(&TX_BUFFER).unwrap();
//! let mut rx_buffer = Linear::new(&RX_BUFFER).unwrap();
//!
//! // Send 6 elements, and expect to receive 6 elements
//! for v in 1..=6 {
//!     tx_buffer.push(v);
//! }
//! rx_buffer.set_transfer_len(6);
//!
//! // Start the DMA transfers
//! peripheral.start_receive(rx_buffer).unwrap();
//! peripheral.start_transfer(tx_buffer).unwrap();
//!
//! // At this point, the DMA controller is transferring data from
//! // the SPI peripheral, and receiving data from the SPI peripheral.
//! // Received data appears in RX_BUFFER. The DMA controller will trigger
//! // an interrupt for DMA channel 25 when it has transferred 6 u16s.
//! //
//! // Your ISR should clear the interrupt and complete the transfers,
//! // as depicted below:
//!
//! while peripheral.is_receive_interrupt() {
//!     peripheral.receive_clear_interrupt();
//! }
//!
//! let mut rx_buffer = None;
//! if peripheral.is_receive_complete() {
//!     // Recover the receive buffer
//!     rx_buffer = peripheral.receive_complete();
//! }
//!
//! let mut tx_buffer = None;
//! if peripheral.is_transfer_complete() {
//!     // Recover the transfer buffer
//!     tx_buffer = peripheral.transfer_complete();
//! }
//! ```
//!
//! # Example: memcpy
//!
//! See the [`Memcpy`](struct.Memcpy.html#example) documentation for an example of DMA-powered memcpy.
//!
//! # Notes on Data Cache
//!
//! If your i.MX RT system is using a data cache (DCache), you're responsible for issuing memory barriers,
//! and flushing any cached buffers, for the DMA controller. More generally, you're responsible for placing
//! your DMA buffers into memory regions that the DMA controller can use.
//!
//! ## TODO
//!
//! - Channel arbitration modes
//! - Channel grouping
//! - Channel priority, and channel priority swapping
//! - Channel chaining

#![allow(non_snake_case)] // Compatibility with RAL

mod buffer;
mod element;
mod memcpy;
pub(crate) mod peripheral;
mod register;

pub use buffer::{Buffer, Circular, CircularError, Linear, ReadHalf, WriteHalf};
pub use element::Element;
pub use memcpy::Memcpy;
pub use peripheral::{helpers::*, Config, ConfigBuilder, Peripheral};

use crate::{ccm, ral};
use core::{
    fmt::{self, Debug, Display},
    mem,
};
use register::{DMARegisters, MultiplexerRegisters, Static, DMA, MULTIPLEXER};

/// A DMA channel
///
/// DMA channels provide one-way transfers of data. They accept a source of data,
/// and a destination of data. They copy data from the source to the destination.
/// When the transfer is complete, a DMA channel signals completion by changing a
/// value in a register, or triggering an interrupt.
///
/// DMA channels have very little public interface. They're best used when paired with a
/// [`Peripheral`](struct.Peripheral.html) or a [`Memcpy`](struct.Memcpy.html).
pub struct Channel {
    /// Our channel number, expected to be between 0 to 31
    index: usize,
    /// Reference to the DMA registers
    registers: Static<DMARegisters>,
    /// Reference to the DMA multiplexer
    multiplexer: Static<MultiplexerRegisters>,
}

impl Channel {
    /// Allocates a DMA channel, and sets the initial state for
    /// the channel.
    fn new(index: usize) -> Self {
        let channel = Channel {
            index,
            registers: DMA,
            multiplexer: MULTIPLEXER,
        };
        channel.registers.TCD[channel.index].reset();
        channel
    }

    /// Returns a handle to this channel's transfer control descriptor
    fn tcd(&self) -> &register::TransferControlDescriptor {
        &self.registers.TCD[self.index]
    }

    /// Indicates that `source` will supply data for a DMA tranfser
    ///
    /// `set_source()` prepares the DMA channel to perform `E`-sized reads
    /// from the memory pointed at by `source`. When the transfer completes,
    /// the DMA controller will continue pointing at `source`.
    ///
    /// # Safety
    ///
    /// Lifetime of `source` must be greater than the lifetime
    /// of the DMA transfer.
    unsafe fn set_source<E: Element>(&mut self, source: *const E) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, SADDR, source as u32);
        ral::write_reg!(register::tcd, tcd, SOFF, 0);
        ral::modify_reg!(register::tcd, tcd, ATTR, SSIZE: E::DATA_TRANSFER_ID, SMOD: 0);
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(register::tcd, tcd, SLAST, 0);
    }

    /// Indiates that `destination` will receive data from a DMA transfer
    ///
    /// `set_destination()` prepares the DMA channel to perform `E`-sized writes
    /// on the memory pointed at by `destination`. When the transfer completes,
    /// the DMA channel will continue pointing at `destination`.
    ///
    /// # Safety
    ///
    /// Lifetime of 'destination' must be greater than the lifetime
    /// of the DMA transfer.
    unsafe fn set_destination<E: Element>(&mut self, destination: *const E) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, DADDR, destination as u32);
        ral::write_reg!(register::tcd, tcd, DOFF, 0);
        ral::modify_reg!(register::tcd, tcd, ATTR, DSIZE: E::DATA_TRANSFER_ID, DMOD: 0);
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(register::tcd, tcd, DLAST_SGA, 0);
    }

    /// Indicates that the `source` buffer will supply data for a DMA transfer
    ///
    /// `set_source_buffer()` prepares the DMA channel to perform `E`-sized reads
    /// of the elements in `source`. When the transfer completes, the DMA channel will
    /// point at the beginning of `source`.
    ///
    /// # Safety
    ///
    /// Lifetime of 'source' must be greater than the lifetime of the
    /// DMA transfer.
    unsafe fn set_source_buffer<E: Element>(&mut self, source: &[E]) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, SADDR, source.as_ptr() as u32);
        ral::write_reg!(register::tcd, tcd, SOFF, mem::size_of::<E>() as i16);
        ral::modify_reg!(register::tcd, tcd, ATTR, SSIZE: E::DATA_TRANSFER_ID, SMOD: 0);
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(
            register::tcd,
            tcd,
            SLAST,
            (source.len() as i32).wrapping_neg()
        );
    }

    /// Indicates that `source` will provide data for a DMA transfer as if it
    /// were a circular buffer
    ///
    /// `set_source_circular()` prepares the DMA channel to perform `E`-sized reads from
    /// the memory pointed at by `source`. `size` indicates the total capacity of the circular
    /// buffer, and it's expected to be a multiple of two. `source` should be a pointer to
    /// a readable element. It's expected that the source buffer's alignment is a multiple
    /// of the buffer size. When the transfer completes, the DMA channel will be pointing at the next
    /// readable element.
    ///
    /// # Safety
    ///
    /// Lifetime of `source` must be greater than the lifetime of the transfer. All of the size
    /// and alignment guarantees must hold.
    unsafe fn set_source_circular<E: Element>(&mut self, source: *const E, size: usize) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, SADDR, source as u32);
        ral::write_reg!(register::tcd, tcd, SOFF, mem::size_of::<E>() as i16);
        ral::modify_reg!(
            register::tcd,
            tcd,
            ATTR,
            SSIZE: E::DATA_TRANSFER_ID,
            SMOD: (31 - size.leading_zeros()) as u16
        );
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(register::tcd, tcd, SLAST, 0);
    }

    /// Indicates that the `destination` buffer will receive data from a DMA transfer
    ///
    /// `set_destination_buffer()` prepares the DMA channel to perform `E`-sized writes
    /// of elements starting at the head of `destination`. When the transfer completes, the DMA
    /// channel will point at the beginning of `destination`.
    ///
    /// # Safety
    ///
    /// Lifetime of 'destination' must be greater than the lifetime of the
    /// DMA transfer.
    unsafe fn set_destination_buffer<E: Element>(&mut self, destination: &mut [E]) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, DADDR, destination.as_mut_ptr() as u32);
        ral::write_reg!(register::tcd, tcd, DOFF, mem::size_of::<E>() as i16);
        ral::modify_reg!(register::tcd, tcd, ATTR, DSIZE: E::DATA_TRANSFER_ID, DMOD: 0);
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(
            register::tcd,
            tcd,
            DLAST_SGA,
            (destination.len() as i32).wrapping_neg()
        );
    }

    /// Indicates that `destination` will recieve data from a DMA transfer as if it
    /// were a circular buffer
    ///
    /// `set_destination_circular()` prepares the DMA channel to perform `E`-sized writes to
    /// the memory pointed at by `destination`. `size` indicates the total capacity of the circular
    /// buffer, and it's expected to be a multiple of two. `destination` should be a pointer to
    /// a writable element. It's expected that the destination buffer's alignment is a multiple
    /// of the buffer size. When the transfer completes, the DMA channel will be pointing at the next
    /// writeable element.
    ///
    /// # Safety
    ///
    /// Lifetime of `destination` must be greater than the lifetime of the transfer. All of the size
    /// and alignment guarantees must hold.
    unsafe fn set_destination_circular<E: Element>(&mut self, destination: *mut E, size: usize) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, DADDR, destination as u32);
        ral::write_reg!(register::tcd, tcd, DOFF, mem::size_of::<E>() as i16);
        ral::modify_reg!(
            register::tcd,
            tcd,
            ATTR,
            DSIZE: E::DATA_TRANSFER_ID,
            DMOD: (31 - size.leading_zeros()) as u16
        );
        ral::write_reg!(register::tcd, tcd, NBYTES, mem::size_of::<E>() as u32);
        ral::write_reg!(register::tcd, tcd, DLAST_SGA, 0);
    }

    /// Tells the DMA channel how many transfer iterations to perform
    ///
    /// A 'transfer iteration' is a read from a source, and a write to a destination.
    fn set_transfer_iterations(&mut self, iterations: u16) {
        let tcd = self.tcd();
        ral::write_reg!(register::tcd, tcd, CITER, iterations);
        ral::write_reg!(register::tcd, tcd, BITER, iterations);
    }

    /// Enable or disabling triggering from hardware
    ///
    /// If source is `Some(value)`, we trigger from hardware identified by the source identifier.
    /// If `source` is `None`, we disable hardware triggering.
    fn set_trigger_from_hardware(&mut self, source: Option<u32>) {
        let chcfg = &self.multiplexer.chcfg[self.index];
        chcfg.write(0);
        if let Some(source) = source {
            const ENBL: u32 = 1 << 31;
            chcfg.write(ENBL | source);
        }
    }

    /// Returns `true` if the DMA channel is receiving a service signal from hardware
    fn is_hardware_signaling(&self) -> bool {
        self.registers.HRS.read() & (1 << self.index) != 0
    }

    /// Enable or disable the DMA channel
    fn set_enable(&mut self, enable: bool) {
        if enable {
            self.registers.SERQ.write(self.index as u8);
        } else {
            self.registers.CERQ.write(self.index as u8);
        }
    }

    /// Returns `true` if this DMA channel generated an interrupt
    fn is_interrupt(&self) -> bool {
        self.registers.INT.read() & (1 << self.index) != 0
    }

    /// Clear the interrupt flag from this DMA channel
    fn clear_interrupt(&mut self) {
        self.registers.CINT.write(self.index as u8);
    }

    /// Enable or disable 'disable on completion'
    ///
    /// 'Disable on completion' lets the DMA channel automatically clear the request signal
    /// when it completes a transfer.
    fn set_disable_on_completion(&mut self, dreq: bool) {
        let tcd = self.tcd();
        ral::modify_reg!(register::tcd, tcd, CSR, DREQ: dreq as u16);
    }

    /// Enable or disable interrupt generation when the transfer completes
    fn set_interrupt_on_completion(&mut self, intr: bool) {
        let tcd = self.tcd();
        ral::modify_reg!(register::tcd, tcd, CSR, INTMAJOR: intr as u16);
    }

    /// Returns `true` if this channel's completion will generate an interrupt
    fn is_interrupt_on_completion(&self) -> bool {
        let tcd = self.tcd();
        ral::read_reg!(register::tcd, tcd, CSR, INTMAJOR == 1)
    }

    /// Enable or disable interrupt generation when the transfer is half complete
    fn set_interrupt_on_half(&mut self, intr: bool) {
        let tcd = self.tcd();
        ral::modify_reg!(register::tcd, tcd, CSR, INTHALF: intr as u16);
    }

    /// Returns `true` if this channel will generate an interrupt halfway through transfer
    fn is_interrupt_on_half(&self) -> bool {
        let tcd = self.tcd();
        ral::read_reg!(register::tcd, tcd, CSR, INTHALF == 1)
    }

    /// Indicates if the DMA transfer has completed
    fn is_complete(&self) -> bool {
        let tcd = self.tcd();
        ral::read_reg!(register::tcd, tcd, CSR, DONE == 1)
    }

    /// Clears completion indication
    fn clear_complete(&mut self) {
        self.registers.CDNE.write(self.index as u8);
    }

    /// Indicates if the DMA channel is in an error state
    fn is_error(&self) -> bool {
        self.registers.ERR.read() & (1 << self.index) != 0
    }

    /// Clears the error flag
    fn clear_error(&mut self) {
        self.registers.CERR.write(self.index as u8);
    }

    /// Indicates if this DMA channel is actively transferring data
    fn is_active(&self) -> bool {
        let tcd = self.tcd();
        ral::read_reg!(register::tcd, tcd, CSR, ACTIVE == 1)
    }

    /// Indicates if this DMA channel is enabled and requesting service
    ///
    /// 'Enabled' and 'active' are different:
    ///
    /// - 'enabled' means "there's a transfer defined, and we're ready to be serviced"
    /// - 'active' implies 'enabled,' and also adds "we're actively transferring data"
    fn is_enabled(&self) -> bool {
        self.registers.ERQ.read() & (1 << self.index) != 0
    }

    /// Returns the value from the **global** error status register
    ///
    /// It may reflect the last channel that produced an error, and that
    /// may not be related to this channel.
    fn error_status(&self) -> u32 {
        self.registers.ES.read()
    }

    /// Start a DMA transfer
    ///
    /// `start()` should be used to request service from the DMA controller. It's
    /// necessary for in-memory DMA transfers. Do not use it for hardware-initiated
    /// DMA transfers. DMA transfers that involve hardware will rely on the hardware
    /// to request DMA service.
    ///
    /// Flag is automatically cleared by hardware after it's asserted.
    fn start(&mut self) {
        self.registers.SSRT.write(self.index as u8);
    }
}

/// An error when preparing a transfer
#[derive(Debug)]
#[non_exhaustive]
pub enum Error<P> {
    /// There is already a scheduled transfer
    ///
    /// Cancel the transfer and try again.
    ScheduledTransfer,
    /// The peripheral returned an error
    Peripheral(P),
    /// Error setting up the DMA transfer
    Setup(ErrorStatus),
}

/// Unclocked, uninitialized DMA channels
///
/// Use [`clock()`](struct.Unclocked.html#method.clock) to initialize and acquire all DMA channels
///
/// ```no_run
/// let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
///
/// let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
/// let channel_27 = dma_channels[27].take().unwrap();
/// let channel_0 = dma_channels[0].take().unwrap();
/// ```
pub struct Unclocked([Option<Channel>; 32]);
impl Unclocked {
    pub(crate) fn new() -> Self {
        Unclocked([
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        ])
    }
    /// Enable the clocks for the DMA peripheral
    ///
    /// The return is 32 channels, each being initialized as `Some(Channel)`. Users may take channels as needed.
    /// The index in the array maps to the DMA channel number.
    pub fn clock(mut self, ccm: &mut ccm::Handle) -> [Option<Channel>; 32] {
        let (ccm, _) = ccm.raw();
        ral::modify_reg!(ral::ccm, ccm, CCGR5, CG3: 0x03);
        for (idx, channel) in self.0.iter_mut().enumerate() {
            *channel = Some(Channel::new(idx));
        }
        self.0
    }
}

/// A wrapper around a DMA error status value
///
/// The wrapper contains a copy of the DMA controller's
/// error status register at the point of an error. The
/// wrapper implements both `Debug` and `Display`. The
/// type may be printed to understand why there was a
/// DMA error.
#[derive(Clone, Copy)]
pub struct ErrorStatus {
    /// The raw error status
    es: u32,
}

impl ErrorStatus {
    const fn new(es: u32) -> Self {
        ErrorStatus { es }
    }
}

impl Debug for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DMA_ES({:#010X})", self.es)
    }
}

impl Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "DMA_ES: VLD {vld} ECX {ecx} GPE {gpe} CPE {cpe} ERRCHN {errchn} SAE {sae} SOE {soe} DAE {dae} DOE {doe} NCE {nce} SGE {sge} SBE {sbe} DBE {dbe}",
            vld = (self.es >> 31) & 0x1,
            ecx = (self.es >> 16) & 0x1,
            gpe = (self.es >> 15) & 0x1,
            cpe = (self.es >> 14) & 0x1,
            errchn = (self.es >> 8) & 0x1F,
            sae = (self.es >> 7) & 0x1,
            soe = (self.es >> 6) & 0x1,
            dae = (self.es >> 5) & 0x1,
            doe = (self.es >> 4) & 0x1,
            nce = (self.es >> 3) & 0x1,
            sge = (self.es >> 2) & 0x1,
            sbe = (self.es >> 1) & 0x1,
            dbe = self.es & 0x1
        )
    }
}
