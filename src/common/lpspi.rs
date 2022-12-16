//! Low-power serial peripheral interface.
//!
//! [`Lpspi`] implements select embedded HAL SPI traits for coordinating SPI I/O.
//! When using the trait implementations, make sure to call [`set_endian`](Lpspi::set_endian)
//! and [`set_bit_order`](Lpspi::set_bit_order) based on your device. These settings
//! apply when the driver internally defines the transaction.
//!
//! This driver also exposes the peripheral's lower-level, hardware-dependent transaction interface.
//! Create a [`Transaction`], then [`enqueue_transaction`](Lpspi::enqueue_transaction) before
//! sending data with [`enqueue_data`](Lpspi::enqueue_data). When using the transaction interface,
//! you're responsible for serializing your data into `u32` SPI words.
//!
//! # Chip selects (CS) for SPI peripherals
//!
//! The iMXRT SPI peripherals have one or more peripheral-controlled chip selects (CS). Using
//! the peripheral-controlled CS means that you do not need a GPIO to coordinate SPI operations.
//! This driver only supports peripheral-controlled CSes.
//!
//! # Example
//!
//! Initialize an LPSPI with a 1MHz SCK. To understand how to configure the LPSPI
//! peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//! # use eh02 as embedded_hal;
//! use embedded_hal::blocking::spi::Transfer;
//! use hal::lpspi::{Lpspi, Pins};
//! use ral::lpspi::LPSPI4;
//!
//! let mut pads = // Handle to all processor pads...
//!     # unsafe { imxrt_iomuxc::imxrt1060::Pads::new() };
//!
//! # || -> Option<()> {
//! let spi_pins = Pins {
//!     sdo: pads.gpio_b0.p02,
//!     sdi: pads.gpio_b0.p01,
//!     sck: pads.gpio_b0.p03,
//!     pcs0: pads.gpio_b0.p00,
//! };
//!
//! let mut spi4 = unsafe { LPSPI4::instance() };
//! let mut spi = Lpspi::new(
//!     spi4,
//!     spi_pins,
//! );
//!
//! # const LPSPI_CLK_HZ: u32 = 1;
//! spi.disabled(|spi| {
//!     spi.set_clock_hz(LPSPI_CLK_HZ, 1_000_000);
//! });
//!
//! let mut buffer: [u8; 3] = [1, 2, 3];
//! spi.transfer(&mut buffer).ok()?;
//!
//! let (spi4, pins) = spi.release();
//!
//! // Re-construct without pins:
//! let mut spi = Lpspi::without_pins(spi4);
//! # Some(()) }();
//! ```
//!
//! # No continuous transfer or EH02 transactions
//!
//! Due to [a hardware defect][1], this driver does not support continuous transfers.
//! This means the driver does not support the EH02 SPI transaction API. An early iteration
//! of this driver reproduced the issue discussed in that forum.
//!
//! Since the transaction API provides all buffer information up front, the driver _could_
//! work around this hardware defect by modeling the multiple operations as one large transfer.
//! This requires some additional complexity in the driver to model the multiple buffers as one
//! larger buffer, and to handle cases when the driver needs to discard data. But beyond this
//! writing and some early prototyping, none of this has been explored further. Furthermore,
//! EH1 alpha development (as of alpha 8) shows that the transaction trait no longer exists.
//!
//! [1]: https://community.nxp.com/t5/i-MX-RT/RT1050-LPSPI-last-bit-not-completing-in-continuous-mode/m-p/898460

use crate::iomuxc::{consts, lpspi};
use crate::ral;

pub use eh02::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

/// Data direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Transmit direction (leaving the peripheral).
    Tx,
    /// Receive direction (entering the peripheral).
    Rx,
}

/// Data endianness for transmits and receives.
///
/// The LPSPI peripheral can perform byte swaps on
/// data input and output words. This allows you to
/// read and write in native endian regardless of
/// your hardware's behavior.
///
/// Note that this affects both transmit and receive
/// behaviors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Endian {
    /// Little endian (default).
    Little,
    /// Big endian.
    Big,
}

impl Default for Endian {
    fn default() -> Self {
        Endian::Little
    }
}

/// Bit order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BitOrder {
    /// Data is transferred most significant bit first (default).
    Msb,
    /// Data is transferred least significant bit first.
    Lsb,
}

impl Default for BitOrder {
    fn default() -> Self {
        BitOrder::Msb
    }
}

/// An LPSPI transaction definition.
///
/// The transaction defines how many bits the driver sends or recieves.
/// It also describes
///
/// - endianness (default: little)
/// - bit order (default: MSB)
/// - transmit masking (default: disabled)
/// - receive masking (default: disabled)
///
/// The LPSPI enqueues the transaction data inside the transmit
/// FIFO. When it pops the values from the FIFO, the values take
/// effect immediately. This may affect, or abort, any ongoing
/// transactions.
pub struct Transaction {
    endian: Endian,
    bit_order: BitOrder,
    receive_data_mask: bool,
    transmit_data_mask: bool,
    frame_size: u16,
}

/// Possible errors when interfacing the LPSPI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpspiError {
    /// The transaction frame size is incorrect.
    ///
    /// The frame size, in bits, must be between 8 bits and
    /// 4095 bits.
    FrameSize,
    /// FIFO error in the given direction.
    Fifo(Direction),
    /// Bus is busy at the start of a transfer.
    Busy,
    /// Caller provided no data.
    NoData,
}

impl Transaction {
    /// Defines a transaction for a `u8` buffer.
    ///
    /// After successfully defining a transaction of this buffer,
    /// supply it to the LPSPI driver, then start sending the
    /// data.
    ///
    /// Returns an error if any are true:
    ///
    /// - the buffer is empty.
    /// - there's more than 512 elements in the buffer.
    pub fn new_u8s(data: &[u8]) -> Result<Self, LpspiError> {
        Transaction::new_words(data)
    }

    /// Defines a transaction for a `u16` buffer.
    ///
    /// After successfully defining a transaction of this buffer,
    /// supply it to the LPSPI driver, then start sending the
    /// data.
    ///
    /// Returns an error if any are true:
    ///
    /// - the buffer is empty.
    /// - there's more than 256 elements in the buffer.
    pub fn new_u16s(data: &[u16]) -> Result<Self, LpspiError> {
        Transaction::new_words(data)
    }

    /// Defines a transaction for a `u32` buffer.
    ///
    /// After successfully defining a transaction of this buffer,
    /// supply it to the LPSPI driver, then start sending the
    /// data.
    ///
    /// Returns an error if any are true:
    ///
    /// - the buffer is empty.
    /// - there's more than 128 elements in the buffer.
    pub fn new_u32s(data: &[u32]) -> Result<Self, LpspiError> {
        Transaction::new_words(data)
    }

    fn new_words<W>(data: &[W]) -> Result<Self, LpspiError> {
        Transaction::new(8 * core::mem::size_of_val(data) as u16)
    }

    /// Define a transaction by specifying the frame size, in bits.
    ///
    /// The frame size describes the number of bits that will be transferred and
    /// received during the next transaction. Specifically, it describes the number
    /// of bits for which the PCS pin signals a transaction.
    ///
    /// # Requirements
    ///
    /// - `frame_size` fits within 12 bits; the implementation enforces this maximum value.
    /// - The minimum value for `frame_size` is 8; the implementation enforces this minimum
    ///   value.
    pub fn new(frame_size: u16) -> Result<Self, LpspiError> {
        const MIN_FRAME_SIZE: u16 = 8;
        const MAX_FRAME_SIZE: u16 = 1 << 12;
        if (MIN_FRAME_SIZE..MAX_FRAME_SIZE).contains(&frame_size) {
            Ok(Self {
                endian: Default::default(),
                bit_order: Default::default(),
                receive_data_mask: false,
                transmit_data_mask: false,
                frame_size: frame_size - 1,
            })
        } else {
            Err(LpspiError::FrameSize)
        }
    }

    /// Change the endianness.
    ///
    /// By default, the endianness is `Endian::Little`.
    pub fn set_endianness(&mut self, endianness: Endian) -> &mut Self {
        self.endian = endianness;
        self
    }

    /// Change the bit order.
    ///
    /// By default, the bit order is `BitOrder::Msb`.
    pub fn set_bit_order(&mut self, bit_order: BitOrder) -> &mut Self {
        self.bit_order = bit_order;
        self
    }

    /// Prevent data from entering the transmit FIFO.
    ///
    /// Useful if you're only receiving data.
    pub fn disable_transmit(&mut self) -> &mut Self {
        self.transmit_data_mask = true;
        self
    }

    /// Prevent data from entering the receive FIFO.
    ///
    /// Useful if you're only transmitting data.
    pub fn disable_receive(&mut self) -> &mut Self {
        self.receive_data_mask = true;
        self
    }
}

/// Sets the clock speed parameters.
///
/// This should only happen when the LPSPI peripheral is disabled.
fn set_spi_clock(source_clock_hz: u32, spi_clock_hz: u32, reg: &ral::lpspi::RegisterBlock) {
    let mut div = source_clock_hz / spi_clock_hz;

    if source_clock_hz / div > spi_clock_hz {
        div += 1;
    }

    // 0 <= div <= 255, and the true coefficient is really div + 2
    let div = div.saturating_sub(2).min(255).max(0);
    ral::write_reg!(
        ral::lpspi,
        reg,
        CCR,
        SCKDIV: div,
        // Both of these delays are arbitrary choices, and they should
        // probably be configurable by the end-user.
        DBT: div / 2,
        SCKPCS: 0x1F,
        PCSSCK: 0x1F
    );
}

/// An LPSPI peripheral.
///
/// The peripheral exposes low-level methods for coordinating
/// DMA transfers. However, you may find it easier to use the
/// [`dma`](crate::dma) interface to coordinate DMA transfers.
pub struct Lpspi<P, const N: u8> {
    lpspi: ral::lpspi::Instance<N>,
    pins: P,
    bit_order: BitOrder,
    endian: Endian,
}

/// Pins for a LPSPI device.
///
/// Consider using type aliases to simplify your usage:
///
/// ```no_run
/// use imxrt_hal as hal;
/// use imxrt_iomuxc::imxrt1060::gpio_b0::*;
///
/// // SPI pins used in my application
/// type LpspiPins = hal::lpspi::Pins<
///     GPIO_B0_02,
///     GPIO_B0_01,
///     GPIO_B0_03,
///     GPIO_B0_00,
/// >;
///
/// // Helper type for your SPI peripheral
/// type Lpspi<const N: u8> = hal::lpspi::Lpspi<LpspiPins, N>;
/// ```
pub struct Pins<SDO, SDI, SCK, PCS0> {
    /// Serial data out
    ///
    /// Data travels from the SPI host controller to the SPI device.
    pub sdo: SDO,
    /// Serial data in
    ///
    /// Data travels from the SPI device to the SPI host controller.
    pub sdi: SDI,
    /// Serial clock
    pub sck: SCK,
    /// Chip select 0
    ///
    /// (PCSx) convention matches the hardware.
    pub pcs0: PCS0,
}

impl<SDO, SDI, SCK, PCS0, const N: u8> Lpspi<Pins<SDO, SDI, SCK, PCS0>, N>
where
    SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
    SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
    SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    PCS0: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs0>,
{
    /// Create a new LPSPI driver from the RAL LPSPI instance and a set of pins.
    ///
    /// The LPSPI clock speed is unspecified.
    pub fn new(lpspi: ral::lpspi::Instance<N>, mut pins: Pins<SDO, SDI, SCK, PCS0>) -> Self {
        lpspi::prepare(&mut pins.sdo);
        lpspi::prepare(&mut pins.sdi);
        lpspi::prepare(&mut pins.sck);
        lpspi::prepare(&mut pins.pcs0);
        Self::init(lpspi, pins)
    }
}

impl<const N: u8> Lpspi<(), N> {
    /// Create a new LPSPI driver from the RAL LPSPI instance.
    ///
    /// This is similar to [`new()`](Self::new), but it does not configure
    /// pins. You're responsible for configuring pins, and for making sure
    /// the pin configuration doesn't change while this driver is in use.
    pub fn without_pins(lpspi: ral::lpspi::Instance<N>) -> Self {
        Self::init(lpspi, ())
    }
}

impl<P, const N: u8> Lpspi<P, N> {
    pub const N: u8 = N;

    fn init(lpspi: ral::lpspi::Instance<N>, pins: P) -> Self {
        let mut spi = Lpspi {
            lpspi,
            pins,
            endian: Endian::default(),
            bit_order: BitOrder::default(),
        };
        ral::write_reg!(ral::lpspi, spi.lpspi, CR, RST: RST_1);
        ral::write_reg!(ral::lpspi, spi.lpspi, CR, RST: RST_0);
        ral::write_reg!(
            ral::lpspi,
            spi.lpspi,
            CFGR1,
            MASTER: MASTER_1,
            SAMPLE: SAMPLE_1
        );
        Disabled::new(&mut spi.lpspi).set_mode(MODE_0);
        ral::write_reg!(ral::lpspi, spi.lpspi, FCR, RXWATER: 0xF, TXWATER: 0xF);
        ral::write_reg!(ral::lpspi, spi.lpspi, CR, MEN: MEN_1);
        spi
    }

    /// Release the SPI peripheral components
    pub fn release(self) -> (ral::lpspi::Instance<N>, P) {
        (self.lpspi, self.pins)
    }

    /// Returns the endian configuration.
    ///
    /// See notes in [`set_endian`](Lpspi::set_endian) to understand
    /// when this configuration takes effect.
    pub fn endian(&self) -> Endian {
        self.endian
    }

    /// Set the endian configuration.
    ///
    /// This applies to all higher-level write and transfer operations.
    /// If you're using the [`Transaction`] API with manual word reads
    /// and writes, set the configuration as part of the transaction.
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    /// Returns the bit order configuration.
    ///
    /// See notes in [`set_bit_order`](Lpspi::set_bit_order) to
    /// understand when this configuration takes effect.
    pub fn bit_order(&self) -> BitOrder {
        self.bit_order
    }

    /// Set the bit order configuration.
    ///
    /// This applies to all higher-level write and transfer operations.
    /// If you're using the [`Transaction`] API with manual word reads
    /// and writes, set the configuration as part of the transaction.
    pub fn set_bit_order(&mut self, bit_order: BitOrder) {
        self.bit_order = bit_order;
    }

    /// Temporarily disable the LPSPI peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpspi::Disabled) driver lets you modify
    /// LPSPI settings that require a fully disabled peripheral. This will clear the transmit
    /// and receive FIFOs.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        self.clear_fifos();
        let mut disabled = Disabled::new(&mut self.lpspi);
        func(&mut disabled)
    }

    /// Read the status register.
    pub fn status(&self) -> Status {
        Status::from_bits_truncate(ral::read_reg!(ral::lpspi, self.lpspi, SR))
    }

    /// Clear the status flags.
    ///
    /// To clear status flags, set them high, then call `clear_status()`.
    ///
    /// The implementation will ensure that only the W1C bits are written, so it's
    /// OK to supply `Status::all()` to clear all bits.
    pub fn clear_status(&self, flags: Status) {
        let flags = flags & Status::W1C;
        ral::write_reg!(ral::lpspi, self.lpspi, SR, flags.bits());
    }

    /// Read the interrupt enable bits.
    pub fn interrupts(&self) -> Interrupts {
        Interrupts::from_bits_truncate(ral::read_reg!(ral::lpspi, self.lpspi, IER))
    }

    /// Set the interrupt enable bits.
    ///
    /// This writes the bits described by `interrupts` as is to the register.
    /// To modify the existing interrupts flags, you should first call [`interrupts`](Lpspi::interrupts)
    /// to get the current state, then modify that state.
    pub fn set_interrupts(&self, interrupts: Interrupts) {
        ral::write_reg!(ral::lpspi, self.lpspi, IER, interrupts.bits());
    }

    /// Clear any existing data in the SPI receive or transfer FIFOs.
    #[inline]
    pub fn clear_fifo(&mut self, direction: Direction) {
        match direction {
            Direction::Tx => ral::modify_reg!(ral::lpspi, self.lpspi, CR, RTF: RTF_1),
            Direction::Rx => ral::modify_reg!(ral::lpspi, self.lpspi, CR, RRF: RRF_1),
        }
    }

    /// Clear both FIFOs.
    pub fn clear_fifos(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, RTF: RTF_1, RRF: RRF_1);
    }

    /// Returns the watermark level for the given direction.
    #[inline]
    pub fn watermark(&self, direction: Direction) -> u8 {
        (match direction {
            Direction::Rx => ral::read_reg!(ral::lpspi, self.lpspi, FCR, RXWATER),
            Direction::Tx => ral::read_reg!(ral::lpspi, self.lpspi, FCR, TXWATER),
        }) as u8
    }

    /// Returns the FIFO status.
    #[inline]
    pub fn fifo_status(&self) -> FifoStatus {
        let (rxcount, txcount) = ral::read_reg!(ral::lpspi, self.lpspi, FSR, RXCOUNT, TXCOUNT);
        FifoStatus {
            rxcount: rxcount as u16,
            txcount: txcount as u16,
        }
    }

    /// Simply read whatever is in the receiver data register.
    fn read_data_unchecked(&self) -> u32 {
        ral::read_reg!(ral::lpspi, self.lpspi, RDR)
    }

    /// Read the data register.
    ///
    /// Returns `None` if the receive FIFO is empty. Otherwise, returns the complete
    /// read of the register. You're reponsible for interpreting the raw value as
    /// a data word, depending on the frame size.
    pub fn read_data(&mut self) -> Option<u32> {
        if ral::read_reg!(ral::lpspi, self.lpspi, RSR, RXEMPTY == RXEMPTY_0) {
            Some(self.read_data_unchecked())
        } else {
            None
        }
    }

    /// Check for any receiver errors.
    fn recv_ok(&self) -> Result<(), LpspiError> {
        let status = self.status();
        if status.intersects(Status::RECEIVE_ERROR) {
            Err(LpspiError::Fifo(Direction::Rx))
        } else {
            Ok(())
        }
    }

    /// Place `word` into the transmit FIFO.
    ///
    /// This will result in the value being sent from the LPSPI.
    /// You're responsible for making sure that the transmit FIFO can
    /// fit this word.
    pub fn enqueue_data(&self, word: u32) {
        ral::write_reg!(ral::lpspi, self.lpspi, TDR, word);
    }

    pub(crate) fn wait_for_transmit_fifo_space(&mut self) -> Result<(), LpspiError> {
        loop {
            let status = self.status();
            if status.intersects(Status::TRANSMIT_ERROR) {
                return Err(LpspiError::Fifo(Direction::Tx));
            }
            let fifo_status = self.fifo_status();
            if !fifo_status.is_full(Direction::Tx) {
                return Ok(());
            }
        }
    }

    /// Place a transaction definition into the transmit FIFO.
    ///
    /// Once this definition is popped from the transmit FIFO, this may
    /// affect, or abort, any ongoing transactions.
    ///
    /// You're responsible for making sure there's space in the transmit
    /// FIFO for this transaction command.
    pub fn enqueue_transaction(&mut self, transaction: &Transaction) {
        ral::modify_reg!(ral::lpspi, self.lpspi, TCR,
            LSBF: transaction.bit_order as u32,
            BYSW: transaction.endian as u32,
            RXMSK: transaction.receive_data_mask as u32,
            TXMSK: transaction.transmit_data_mask as u32,
            FRAMESZ: transaction.frame_size as u32
        );
    }

    /// Prepare common properties of the transaction for this driver.
    pub(crate) fn prepare_transaction<W>(&self, buffer: &[W]) -> Result<Transaction, LpspiError> {
        Transaction::new_words(buffer).map(|mut transaction| {
            transaction
                .set_bit_order(self.bit_order)
                .set_endianness(self.endian);
            transaction
        })
    }

    /// Exchanges data with the SPI device.
    fn exchange<T>(&mut self, buffer: &mut [T]) -> Result<(), LpspiError>
    where
        [T]: Word,
    {
        if self.status().intersects(Status::BUSY) {
            return Err(LpspiError::Busy);
        } else if buffer.is_empty() {
            return Err(LpspiError::NoData);
        }

        let transaction = self.prepare_transaction(buffer)?;
        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);

        let mut exchange = exchange::Exchange::new(buffer);
        while let Some(send_word) = exchange.read() {
            self.wait_for_transmit_fifo_space()?;
            self.enqueue_data(send_word);

            self.recv_ok()?;
            if let Some(recv_word) = self.read_data() {
                exchange.write(recv_word);
            }
        }

        while !exchange.is_write_empty() {
            self.recv_ok()?;
            if let Some(recv_word) = self.read_data() {
                exchange.write(recv_word);
            }
        }

        Ok(())
    }

    /// Write data to the transmit queue without subsequently reading
    /// the receive queue.
    ///
    /// Use this method when you know that the receiver queue is disabled
    /// (RXMASK high in TCR).
    fn write_no_read<W>(&mut self, buffer: &[W]) -> Result<(), LpspiError>
    where
        [W]: Word,
    {
        if self.status().intersects(Status::BUSY) {
            return Err(LpspiError::Busy);
        } else if buffer.is_empty() {
            return Err(LpspiError::NoData);
        }

        let mut transaction = self.prepare_transaction(buffer)?;
        transaction.disable_receive();
        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);

        let chunks = buffer.chunks(core::mem::size_of::<u32>() / core::mem::size_of::<W>());
        for slice in chunks {
            self.wait_for_transmit_fifo_space()?;
            self.enqueue_data(slice.read_word());
        }
        Ok(())
    }

    /// Let the peripheral act as a DMA source.
    ///
    /// After this call, the peripheral will signal to the DMA engine whenever
    /// it has data available to read.
    pub fn enable_dma_receive(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, FCR, RXWATER: 0); // No watermarks; affects DMA signaling
        ral::modify_reg!(ral::lpspi, self.lpspi, DER, RDDE: 1);
    }

    /// Stop the peripheral from acting as a DMA source.
    ///
    /// See the DMA chapter in the reference manual to understand when this
    /// should be called in the DMA transfer lifecycle.
    pub fn disable_dma_receive(&mut self) {
        while ral::read_reg!(ral::lpspi, self.lpspi, DER, RDDE == 1) {
            ral::modify_reg!(ral::lpspi, self.lpspi, DER, RDDE: 0);
        }
    }

    /// Let the peripheral act as a DMA destination.
    ///
    /// After this call, the peripheral will signal to the DMA engine whenever
    /// it has free space in its transfer buffer.
    pub fn enable_dma_transmit(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, FCR, TXWATER: 0); // No watermarks; affects DMA signaling
        ral::modify_reg!(ral::lpspi, self.lpspi, DER, TDDE: 1);
    }

    /// Stop the peripheral from acting as a DMA destination.
    ///
    /// See the DMA chapter in the reference manual to understand when this
    /// should be called in the DMA transfer lifecycle.
    pub fn disable_dma_transmit(&mut self) {
        while ral::read_reg!(ral::lpspi, self.lpspi, DER, TDDE == 1) {
            ral::modify_reg!(ral::lpspi, self.lpspi, DER, TDDE: 0);
        }
    }

    /// Produces a pointer to the receiver data register.
    ///
    /// You should use this pointer when coordinating a DMA transfer.
    /// You're not expected to read from this pointer in software.
    pub fn rdr(&self) -> *const ral::RORegister<u32> {
        core::ptr::addr_of!(self.lpspi.RDR)
    }

    /// Produces a pointer to the transfer data register.
    ///
    /// You should use this pointer when coordinating a DMA transfer.
    /// You're not expected to read from this pointer in software.
    pub fn tdr(&self) -> *const ral::WORegister<u32> {
        core::ptr::addr_of!(self.lpspi.TDR)
    }
}

bitflags::bitflags! {
    /// Status flags for the LPSPI interface.
    pub struct Status : u32 {
        /// Module busy flag.
        ///
        /// This flag is read only.
        const BUSY = 1 << 24;

        //
        // Start W1C bits.
        //

        /// Data match flag.
        ///
        /// Indicates that received data has matched one or both of the match
        /// fields. To clear this flag, write this bit to the status register
        /// (W1C).
        const DATA_MATCH = 1 << 13;
        /// Receive error flag.
        ///
        /// Set when the receive FIFO has overflowed. Before clearing this bit,
        /// empty the receive FIFO. Then, write this bit to clear the flag (W1C).
        const RECEIVE_ERROR = 1 << 12;
        /// Transmit error flag.
        ///
        /// Set when the transmit FIFO has underruns. Before clearing this bit,
        /// end the transfer. Then, write this bit to clear the flag (W1C).
        const TRANSMIT_ERROR = 1 << 11;
        /// Transfer complete flag.
        ///
        /// Set when the LPSPI returns to an idle state, and the transmit FIFO
        /// is empty. To clear this flag, write this bit (W1C).
        const TRANSFER_COMPLETE = 1 << 10;
        /// Frame complete flag.
        ///
        /// Set at the end of each frame transfer, when PCS negates. To clear this
        /// flag, write this bit (W1C).
        const FRAME_COMPLETE = 1 << 9;
        /// Word complete flag.
        ///
        /// Set when the last bit of a received word is sampled. To clear this flag, write
        /// this bit (W1C).
        const WORD_COMPLETE = 1 << 8;

        //
        // End W1C bits.
        //

        /// Receive data flag.
        ///
        /// Set when the number of words in the receive FIFO is greater than the watermark.
        /// This flag is read only. To clear the flag, exhaust the receive FIFO.
        const RECEIVE_DATA = 1 << 1;
        /// Transmit data flag.
        ///
        /// Set when the number of words in the transmit FIFO is less than or equal to the
        /// watermark. This flag is read only. TO clear the flag, fill the transmit FIFO.
        const TRANSMIT_DATA = 1 << 0;
    }
}

impl Status {
    const W1C: Self = Self::from_bits_truncate(
        Self::DATA_MATCH.bits()
            | Self::RECEIVE_ERROR.bits()
            | Self::TRANSMIT_ERROR.bits()
            | Self::TRANSFER_COMPLETE.bits()
            | Self::FRAME_COMPLETE.bits()
            | Self::WORD_COMPLETE.bits(),
    );
}

/// The number of words in each FIFO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FifoStatus {
    /// Number of words in the receive FIFO.
    pub rxcount: u16,
    /// Number of words in the transmit FIFO.
    pub txcount: u16,
}

impl FifoStatus {
    /// Indicates if the FIFO is full for the given direction.
    #[inline]
    pub const fn is_full(self, direction: Direction) -> bool {
        /// See PARAM register docs.
        const MAX_FIFO_SIZE: u16 = 16;
        let count = match direction {
            Direction::Tx => self.txcount,
            Direction::Rx => self.rxcount,
        };
        count >= MAX_FIFO_SIZE
    }
}

bitflags::bitflags! {
    /// Interrupt flags.
    ///
    /// A high bit indicates that the condition generates an interrupt.
    /// See the status bits for more information.
    pub struct Interrupts : u32 {
        /// Data match interrupt enable.
        const DATA_MATCH = 1 << 13;
        /// Receive error interrupt enable.
        const RECEIVE_ERROR = 1 << 12;
        /// Transmit error interrupt enable.
        const TRANSMIT_ERROR = 1 << 11;
        /// Transmit complete interrupt enable.
        const TRANSMIT_COMPLETE = 1 << 10;
        /// Frame complete interrupt enable.
        const FRAME_COMPLETE = 1 << 9;
        /// Word complete interrupt enable.
        const WORD_COMPLETE = 1 << 8;

        /// Receive data interrupt enable.
        const RECEIVE_DATA = 1 << 1;
        /// Transmit data interrupt enable.
        const TRANSMIT_DATA = 1 << 0;
    }
}

/// An LPSPI peripheral which is temporarily disabled.
pub struct Disabled<'a, const N: u8> {
    lpspi: &'a ral::lpspi::Instance<N>,
    men: bool,
}

impl<'a, const N: u8> Disabled<'a, N> {
    fn new(lpspi: &'a mut ral::lpspi::Instance<N>) -> Self {
        let men = ral::read_reg!(ral::lpspi, lpspi, CR, MEN == MEN_1);
        ral::modify_reg!(ral::lpspi, lpspi, CR, MEN: MEN_0);
        Self { lpspi, men }
    }

    /// Set the SPI mode for the peripheral
    pub fn set_mode(&mut self, mode: Mode) {
        // This could probably be changed when we're not disabled.
        // However, there's rules about when you can read TCR.
        // Specifically, reading TCR while it's being loaded from
        // the transmit FIFO could result in an incorrect reading.
        // Only permitting this when we're disabled might help
        // us avoid something troublesome.
        ral::modify_reg!(
            ral::lpspi,
            self.lpspi,
            TCR,
            CPOL: ((mode.polarity == Polarity::IdleHigh) as u32),
            CPHA: ((mode.phase == Phase::CaptureOnSecondTransition) as u32)
        );
    }

    /// Set the LPSPI clock speed (Hz).
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn set_clock_hz(&mut self, source_clock_hz: u32, clock_hz: u32) {
        set_spi_clock(source_clock_hz, clock_hz, self.lpspi);
    }

    /// Set the watermark level for a given direction.
    ///
    /// Returns the watermark level committed to the hardware. This may be different
    /// than the supplied `watermark`, since it's limited by the hardware.
    ///
    /// When `direction == Direction::Rx`, the receive data flag is set whenever the
    /// number of words in the receive FIFO is greater than `watermark`.
    ///
    /// When `direction == Direction::Tx`, the transmit data flag is set whenever the
    /// the number of words in the transmit FIFO is less than, or equal, to `watermark`.
    #[inline]
    pub fn set_watermark(&mut self, direction: Direction, watermark: u8) -> u8 {
        let max_watermark = match direction {
            Direction::Rx => 1 << ral::read_reg!(ral::lpspi, self.lpspi, PARAM, RXFIFO),
            Direction::Tx => 1 << ral::read_reg!(ral::lpspi, self.lpspi, PARAM, TXFIFO),
        };

        let watermark = watermark.min(max_watermark - 1);

        match direction {
            Direction::Rx => {
                ral::modify_reg!(ral::lpspi, self.lpspi, FCR, RXWATER: watermark as u32)
            }
            Direction::Tx => {
                ral::modify_reg!(ral::lpspi, self.lpspi, FCR, TXWATER: watermark as u32)
            }
        }

        watermark
    }
}

impl<const N: u8> Drop for Disabled<'_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, MEN: self.men as u32);
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u8> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        self.exchange(words)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u16> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u16]) -> Result<&'a [u16], Self::Error> {
        self.exchange(words)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u32> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u32]) -> Result<&'a [u32], Self::Error> {
        self.exchange(words)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u8> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.write_no_read(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u16> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        self.write_no_read(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u32> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        self.write_no_read(words)
    }
}

// Not supporting WriteIter right now. Since we don't know how many bytes we're
// going to write, we can't specify the frame size. There might be ways around
// this by playing with CONTC and CONT bits, but we can evaluate that later.

/// Encapsulates the data exchange routine.
mod exchange {
    use super::Word;

    /// A type that can exchange SPI words with a slice.
    pub(super) struct Exchange<'a, T: ?Sized> {
        /// Source / sink of data.
        data: &'a mut T,
        /// Read position in the data slice.
        read_idx: usize,
        /// Write position in the data slice.
        write_idx: usize,
    }

    impl<'a, T> Exchange<'a, [T]> {
        /// Create a new exchange.
        pub fn new(data: &'a mut [T]) -> Self {
            Self {
                data,
                read_idx: 0,
                write_idx: 0,
            }
        }
    }

    impl<T> Exchange<'_, [T]>
    where
        [T]: Word,
    {
        fn end(&self, start: usize) -> usize {
            let elems = core::mem::size_of::<u32>() / core::mem::size_of::<T>();
            (start + elems).min(self.data.len())
        }
        /// Read the next word from the data.
        ///
        /// Returns `None` if there is no more data.
        pub fn read(&mut self) -> Option<u32> {
            let end = self.end(self.read_idx);
            let slice = &self.data[self.read_idx..end];
            if slice.is_empty() {
                return None;
            }
            let word = slice.read_word();
            self.read_idx = end;
            Some(word)
        }

        /// Write the next word into the data slice.
        ///
        /// Does nothing if the exchange cannot hold the word.
        pub fn write(&mut self, word: u32) {
            let end = self.end(self.write_idx);
            let slice = &mut self.data[self.write_idx..end];
            if slice.is_empty() {
                return;
            }
            slice.write_word(word);
            self.write_idx = end;
        }

        fn write_len(&self) -> usize {
            self.data.len() - self.write_idx
        }

        /// Check if the write half can hold more data (`false`)
        /// or if it's empty / out of space (`true`).
        pub fn is_write_empty(&self) -> bool {
            self.write_len() == 0
        }
    }
}

/// Interface to read / write SPI words in memory.
///
/// Implemented on slices at and below `u32`.
trait Word {
    /// Read the word.
    fn read_word(&self) -> u32;
    /// Write the word.
    fn write_word(&mut self, word: u32);
}

impl Word for [u8] {
    fn read_word(&self) -> u32 {
        u32::from_le_bytes([
            #[allow(clippy::get_first)] // Syntactically consistent.
            self.get(0).copied().unwrap_or(0),
            self.get(1).copied().unwrap_or(0),
            self.get(2).copied().unwrap_or(0),
            self.get(3).copied().unwrap_or(0),
        ])
    }
    fn write_word(&mut self, word: u32) {
        self.iter_mut()
            .zip(word.to_le_bytes())
            .for_each(|(dst, src)| *dst = src);
    }
}

impl Word for [u16] {
    fn read_word(&self) -> u32 {
        #[allow(clippy::get_first)] // Syntactically consistent.
        let [a, b] = self
            .get(0)
            .copied()
            .map(u16::to_le_bytes)
            .unwrap_or([0u8, 0]);
        let [c, d] = self
            .get(1)
            .copied()
            .map(u16::to_le_bytes)
            .unwrap_or([0u8, 0]);
        u32::from_le_bytes([a, b, c, d])
    }
    fn write_word(&mut self, word: u32) {
        if let Some(halfword) = self.get_mut(0) {
            *halfword = word as u16;
        }
        if let Some(halfword) = self.get_mut(1) {
            *halfword = (word >> 16) as u16
        }
    }
}

impl Word for [u32] {
    fn read_word(&self) -> u32 {
        self.first().copied().unwrap_or(0)
    }
    fn write_word(&mut self, word: u32) {
        if let Some(w) = self.get_mut(0) {
            *w = word;
        }
    }
}

/// Describes the primitive types that can be transmitted and
/// received by DMA.
///
/// This is a subset of all possible DMA elements, since it matches
/// the [`Transaction`] support.
pub trait DmaElement: crate::common::dma::Element {}
impl DmaElement for u8 {}
impl DmaElement for u16 {}
impl DmaElement for u32 {}

#[cfg(test)]
mod tests {
    use super::{Direction, FifoStatus};

    #[test]
    fn fifo_status_is_full() {
        let mut status = FifoStatus {
            txcount: 0,
            rxcount: 0,
        };
        assert!(!status.is_full(Direction::Tx));
        assert!(!status.is_full(Direction::Rx));

        status.txcount = 15;
        status.rxcount = 15;
        assert!(!status.is_full(Direction::Tx));
        assert!(!status.is_full(Direction::Rx));

        status.txcount = 16;
        status.rxcount = 16;
        assert!(status.is_full(Direction::Tx));
        assert!(status.is_full(Direction::Rx));
    }

    use super::exchange::Exchange;

    #[test]
    fn byte_exchange() {
        let mut buffer: [u8; 5] = [1, 2, 3, 4, 5];
        let mut ex = Exchange::new(&mut buffer);
        assert!(!ex.is_write_empty());
        assert_eq!(ex.read(), Some(0x04030201));
        ex.write(0xAAAAAAAA);
        assert_eq!(ex.read(), Some(0x00000005));
        assert!(!ex.is_write_empty());
        ex.write(0x55555555);
        assert!(ex.is_write_empty());
        assert_eq!(ex.read(), None);
        ex.write(0x12345678);
        assert_eq!(buffer, [0xAA, 0xAA, 0xAA, 0xAA, 0x55]);
    }

    #[test]
    fn half_exchange() {
        let mut buffer: [u16; 5] = [1, 2, 3, 4, 5];
        let mut ex = Exchange::new(&mut buffer);
        assert_eq!(ex.read(), Some(0x00020001));
        ex.write(0xDEADBEEF);
        assert_eq!(ex.read(), Some(0x00040003));
        ex.write(0xad1cac1d);
        assert_eq!(ex.read(), Some(0x00000005));
        assert!(!ex.is_write_empty());
        ex.write(0xabcdef89);
        assert!(ex.is_write_empty());
        assert_eq!(ex.read(), None);
        ex.write(42);
        assert_eq!(buffer, [0xBEEF, 0xDEAD, 0xac1d, 0xad1c, 0xef89]);
    }

    #[test]
    fn word_exchange() {
        let mut buffer: [u32; 3] = [1, 2, 3];
        let mut ex = Exchange::new(&mut buffer);
        assert!(!ex.is_write_empty());
        assert_eq!(ex.read(), Some(1));
        ex.write(99);
        assert_eq!(ex.read(), Some(2));
        ex.write(88);
        assert_eq!(ex.read(), Some(3));
        assert!(!ex.is_write_empty());
        ex.write(77);
        assert!(ex.is_write_empty());
        assert_eq!(ex.read(), None);
        ex.write(66);
        assert_eq!(buffer, [99, 88, 77]);
    }
}
