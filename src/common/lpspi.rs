//! Low-power serial peripheral interface.
//!
//! [`Lpspi`] implements select embedded HAL SPI traits for coordinating SPI I/O.
//! When using the trait implementations, make sure that [`set_bit_order`](Lpspi::set_bit_order)
//! is correct for your device. These settings apply when the driver internally defines the transaction.
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
//! Blocking full-duplex transfers and writes will observe an asserted chip select while data
//! frames are exchanged / written.
//!
//! This driver generally assumes that you're using the peripheral-controlled chip select. If
//! you instead want to manage chip select in software, you should be able to multiplex your own
//! pins, then construct the driver [`without_pins`](Lpspi::without_pins).
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
//! use hal::lpspi::{Lpspi, Pins, SamplePoint};
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
//!     spi.set_sample_point(SamplePoint::Edge);
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
//! # Limitations
//!
//! Due to [a hardware defect][1], this driver does not yet support the EH02 SPI transaction API.
//! An early iteration of this driver reproduced the issue discussed in that forum. This driver may
//! be able to work around the defect in software, but it hasn't been explored.
//!
//! [1]: https://community.nxp.com/t5/i-MX-RT/RT1050-LPSPI-last-bit-not-completing-in-continuous-mode/m-p/898460
//!
//! [`Transaction`] exposes the continuous / continuing flags, so you're free to model advanced
//! transactions. However, keep in mind that disabling the receiver during a continuous transaction
//! may not work as expected.

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

/// Bit order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BitOrder {
    /// Data is transferred most significant bit first (default).
    #[default]
    Msb,
    /// Data is transferred least significant bit first.
    Lsb,
}

/// Receive sample point behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplePoint {
    /// Input data is sampled on SCK edge.
    Edge,
    /// Input data is sampled on delayed SCK edge.
    DelayedEdge,
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

/// An LPSPI transaction definition.
///
/// The transaction defines how many bits the driver sends or recieves.
/// It also describes
///
/// - endianness
/// - bit order
/// - transmit and receive masking
/// - continuous and continuing transfers (default: both disabled)
///
/// The LPSPI enqueues the transaction data into the transmit
/// FIFO. When it pops the values from the FIFO, the values take
/// effect immediately. This may affect, or abort, any ongoing
/// transactions. Consult the reference manual to understand when
/// you should enqueue transaction definitions, since it may only
/// be supported on word / frame boundaries.
///
/// Construct `Transaction` with [`new`](Self::new), and supply
/// the number of **bits** to transmit per frame.
///
/// ```
/// use imxrt_hal as hal;
/// use hal::lpspi::Transaction;
///
/// // Send one u32.
/// let mut transaction
///     = Transaction::new(8 * core::mem::size_of::<u32>() as u16);
/// ```
///
/// Once constructed, manipulate the public members to change the
/// configuration.
///
/// # Continuous transactions
///
/// The pseudo-code below shows how to set [`continuous`](Self::continuous) and
/// [`continuing`](Self::continuing) to model a continuous transaction. Keep in
/// mind the hardware limitations; see the [module-level docs](crate::lpspi#limitations) for
/// details.
///
/// ```
/// use imxrt_hal as hal;
/// use hal::lpspi::Transaction;
///
/// // Skipping LPSPI initialization; see module-level example.
///
/// // Start byte exchange as a continuous transaction. Each frame
/// // exchanges one byte (eight bits) with a device.
/// # || -> Result<(), hal::lpspi::LpspiError> {
/// let mut transaction = Transaction::new(8)?;
/// transaction.continuous = true;
/// // Enqueue transaction with LPSPI...
/// // Enqueue one byte with LPSPI...   <-- PCS asserts here.
///
/// # let buffer: [u8; 5] = [0; 5];
/// for byte in buffer {
///     // Set 'continuing' to indicate that the next
///     // transaction continues the previous one...
///     transaction.continuing = true;
///
///     // Enqueue transaction with LPSPI...
///     // Enqueue byte with LPSPI...
/// }
///
/// transaction.continuous = false;
/// transaction.continuing = false;
/// // Enqueue transaction with LPSPI... <-- PCS de-asserts here.
/// # Ok(()) }().unwrap();
/// ```
pub struct Transaction {
    /// Enable byte swap.
    ///
    /// When enabled (`true`), swap bytes within the `u32` word. This allows
    /// you to change the endianness of the 32-bit word transfer. The
    /// default is `false`.
    pub byte_swap: bool,
    /// Bit order.
    ///
    /// See [`BitOrder`] for details. The default is [`BitOrder::Msb`].
    pub bit_order: BitOrder,
    /// Mask the received data.
    ///
    /// If `true`, the peripheral discards received data. Use this
    /// when you only care about sending data. The default is `false`;
    /// the peripheral puts received data in the receive FIFO.
    pub receive_data_mask: bool,
    /// Mask the transmit data.
    ///
    /// If `true`, the peripheral doesn't send any data. Use this when
    /// you only care about receiving data. The default is `false`;
    /// the peripheral expects to send data using the transmit FIFO.
    pub transmit_data_mask: bool,
    /// Indicates (`true`) the start of a continuous transfer.
    ///
    /// If set, the peripherals chip select will remain asserted after
    /// exchanging the frame. This allows you to enqueue new commands
    /// and data words within the same transaction. Those new commands
    /// should have [`continuing`](Self::continuing) set to `true`.
    ///
    /// The default is `false`; chip select de-asserts after exchanging
    /// the frame. To stop a continuous transfer, enqueue a new `Transaction`
    /// in which this flag, and `continuing`, is false.
    pub continuous: bool,
    /// Indicates (`true`) that this command belongs to a previous transaction.
    ///
    /// Set this to indicate that this new `Transaction` belongs to a previous
    /// `Transaction`, one that had [`continuous`](Self::continuous) set.
    /// The default value is `false`.
    pub continuing: bool,

    frame_size: u16,
}

impl Transaction {
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
        if let Ok(frame_size) = u16::try_from(8 * core::mem::size_of_val(data)) {
            Transaction::new(frame_size)
        } else {
            Err(LpspiError::FrameSize)
        }
    }

    fn frame_size_valid(frame_size: u16) -> bool {
        const MIN_FRAME_SIZE: u16 = 8;
        const MAX_FRAME_SIZE: u16 = 1 << 12;
        const MIN_WORD_SIZE: u16 = 2;
        const WORD_SIZE: u16 = 32;

        let last_word_size = frame_size % WORD_SIZE;

        (MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&frame_size) && (last_word_size >= MIN_WORD_SIZE)
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
    /// - The last 32-bit word in the frame is at least 2 bits long.
    pub fn new(frame_size: u16) -> Result<Self, LpspiError> {
        if Self::frame_size_valid(frame_size) {
            Ok(Self {
                byte_swap: false,
                bit_order: Default::default(),
                receive_data_mask: false,
                transmit_data_mask: false,
                frame_size: frame_size - 1,
                continuing: false,
                continuous: false,
            })
        } else {
            Err(LpspiError::FrameSize)
        }
    }
}

/// Sets the clock speed parameters.
///
/// This should only happen when the LPSPI peripheral is disabled.
fn set_spi_clock(source_clock_hz: u32, spi_clock_hz: u32, reg: &ral::lpspi::RegisterBlock) {
    // Round up, so we always get a resulting SPI clock that is
    // equal or less than the requested frequency.
    let half_div =
        u32::try_from(1 + u64::from(source_clock_hz - 1) / (u64::from(spi_clock_hz) * 2)).unwrap();

    // Make sure SCKDIV is between 0 and 255
    // For some reason SCK starts to misbehave in between frames
    // if half_div is less than 3.
    let half_div = half_div.clamp(3, 128);
    // Because half_div is in range [3,128], sckdiv is in range [4, 254].
    let sckdiv = 2 * (half_div - 1);

    ral::write_reg!(ral::lpspi, reg, CCR,
        // Delay between two clock transitions of two consecutive transfers
        // is exactly sckdiv/2, which causes the transfer to be seamless.
        DBT: half_div - 1,
        // Add one sckdiv/2 setup and hold time before and after the transfer,
        // to make sure the signal is stable at sample time
        PCSSCK: half_div - 1,
        SCKPCS: half_div - 1,
        SCKDIV: sckdiv
    );
}

/// An LPSPI driver.
///
/// The driver exposes low-level methods for coordinating
/// DMA transfers. However, you may find it easier to use the
/// [`dma`](crate::dma) interface to coordinate DMA transfers.
///
/// The driver implements `embedded-hal` SPI traits. You should prefer
/// these implementations for their ease of use.
///
/// See the [module-level documentation](crate::lpspi) for an example
/// of how to construct this driver.
pub struct Lpspi<P, const N: u8> {
    lpspi: ral::lpspi::Instance<N>,
    pins: P,
    bit_order: BitOrder,
    tx_fifo_size: u16,
    rx_fifo_size: u16,
    mode: Mode,
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
/// >;
///
/// // Helper type for your SPI peripheral
/// type Lpspi<const N: u8> = hal::lpspi::Lpspi<LpspiPins, N>;
/// ```
pub struct Pins<SDO, SDI, SCK> {
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
}

impl<SDO, SDI, SCK, const N: u8> Lpspi<Pins<SDO, SDI, SCK>, N>
where
    SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
    SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
    SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
{
    /// Create a new LPSPI driver from the RAL LPSPI instance and a set of pins.
    ///
    /// When this call returns, the LPSPI pins are configured for their function.
    /// The peripheral is enabled after reset. The LPSPI clock speed is unspecified.
    /// The mode is [`MODE_0`]. The sample point is [`SamplePoint::DelayedEdge`].
    pub fn new(lpspi: ral::lpspi::Instance<N>, mut pins: Pins<SDO, SDI, SCK>) -> Self {
        lpspi::prepare(&mut pins.sdo);
        lpspi::prepare(&mut pins.sdi);
        lpspi::prepare(&mut pins.sck);

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
    /// The peripheral instance.
    pub const N: u8 = N;

    fn init(lpspi: ral::lpspi::Instance<N>, pins: P) -> Self {
        let (tx_fifo_size_exp, rx_fifo_size_exp) =
            ral::read_reg!(ral::lpspi, lpspi, PARAM, TXFIFO, RXFIFO);
        let tx_fifo_size = 1 << tx_fifo_size_exp;
        let rx_fifo_size = 1 << rx_fifo_size_exp;

        let spi = Lpspi {
            lpspi,
            pins,
            bit_order: BitOrder::default(),
            tx_fifo_size,
            rx_fifo_size,
            mode: MODE_0,
        };

        // Reset and disable
        ral::modify_reg!(ral::lpspi, spi.lpspi, CR, MEN: MEN_0, RST: RST_1);
        while spi.is_enabled() {}
        ral::modify_reg!(ral::lpspi, spi.lpspi, CR, RST: RST_0);

        // Reset Fifos
        ral::modify_reg!(ral::lpspi, spi.lpspi, CR, RTF: RTF_1, RRF: RRF_1);

        // Configure master mode
        ral::write_reg!(
            ral::lpspi,
            spi.lpspi,
            CFGR1,
            MASTER: MASTER_1,
            SAMPLE: SAMPLE_1
        );

        // Configure watermarks
        ral::write_reg!(ral::lpspi, spi.lpspi, FCR,
            RXWATER: 0,               // Notify when we have any data available
            TXWATER: u32::from(tx_fifo_size) / 2 // Nofify when we have at least tx_fifo_size/2 space available
        );

        ral::write_reg!(ral::lpspi, spi.lpspi, CR, MEN: MEN_1);

        spi
    }

    /// Indicates if the driver is (`true`) or is not (`false`) enabled.
    pub fn is_enabled(&self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi, CR, MEN == MEN_1)
    }

    /// Enable (`true`) or disable (`false`) the peripheral.
    ///
    /// Note that disabling does not take effect immediately; instead the
    /// peripheral finishes the current transfer and then disables itself.
    /// It is required to check [`is_enabled()`](Self::is_enabled) repeatedly until the
    /// peripheral is actually disabled.
    pub fn set_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, MEN: enable as u32)
    }

    /// Reset the driver.
    ///
    /// Note that this may not not reset all peripheral state, like the
    /// enabled state.
    pub fn reset(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, RST: RST_1);
        while ral::read_reg!(ral::lpspi, self.lpspi, CR, RST == RST_1) {
            ral::modify_reg!(ral::lpspi, self.lpspi, CR, RST: RST_0);
        }
    }

    /// Cancel the current transfer and force the driver back into idle
    /// state.
    ///
    /// This should be done whenever an error occurred.
    pub fn soft_reset(&mut self) {
        // Backup previous registers
        let ier = ral::read_reg!(ral::lpspi, self.lpspi, IER);
        let der = ral::read_reg!(ral::lpspi, self.lpspi, DER);
        let cfgr0 = ral::read_reg!(ral::lpspi, self.lpspi, CFGR0);
        let cfgr1 = ral::read_reg!(ral::lpspi, self.lpspi, CFGR1);
        let dmr0 = ral::read_reg!(ral::lpspi, self.lpspi, DMR0);
        let dmr1 = ral::read_reg!(ral::lpspi, self.lpspi, DMR1);
        let ccr = ral::read_reg!(ral::lpspi, self.lpspi, CCR);
        let fcr = ral::read_reg!(ral::lpspi, self.lpspi, FCR);

        // Backup enabled state
        let enabled = self.is_enabled();

        // Reset and disable
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, MEN: MEN_0, RST: RST_1);
        while self.is_enabled() {}
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, RST: RST_0);

        // Reset fifos
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, RTF: RTF_1, RRF: RRF_1);

        // Restore settings
        ral::write_reg!(ral::lpspi, self.lpspi, IER, ier);
        ral::write_reg!(ral::lpspi, self.lpspi, DER, der);
        ral::write_reg!(ral::lpspi, self.lpspi, CFGR0, cfgr0);
        ral::write_reg!(ral::lpspi, self.lpspi, CFGR1, cfgr1);
        ral::write_reg!(ral::lpspi, self.lpspi, DMR0, dmr0);
        ral::write_reg!(ral::lpspi, self.lpspi, DMR1, dmr1);
        ral::write_reg!(ral::lpspi, self.lpspi, CCR, ccr);
        ral::write_reg!(ral::lpspi, self.lpspi, FCR, fcr);

        // Restore enabled state
        self.set_enable(enabled);
    }

    /// Release the SPI driver components.
    ///
    /// This does not change any component state; it releases the components as-is.
    /// If you need to obtain the registers in a known, good state, consider calling
    /// methods like [`reset()`](Self::reset) before releasing the registers.
    pub fn release(self) -> (ral::lpspi::Instance<N>, P) {
        (self.lpspi, self.pins)
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
    /// LPSPI settings that require a fully disabled peripheral.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
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
    ///
    /// Be aware that a critical section might be required to avoid a read-modify-write race condition
    /// between [`interrupts`](Self::interrupts) and [`set_interrupts`](Self::set_interrupts).
    pub fn set_interrupts(&self, interrupts: Interrupts) {
        ral::write_reg!(ral::lpspi, self.lpspi, IER, interrupts.bits());
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
    pub fn set_watermark(&mut self, direction: Direction, watermark: u16) -> u16 {
        let max_watermark = match direction {
            Direction::Rx => self.rx_fifo_size - 1,
            Direction::Tx => self.tx_fifo_size - 1,
        };

        let watermark = watermark.min(max_watermark);

        match direction {
            Direction::Rx => {
                ral::modify_reg!(ral::lpspi, self.lpspi, FCR, RXWATER: u32::from(watermark))
            }
            Direction::Tx => {
                ral::modify_reg!(ral::lpspi, self.lpspi, FCR, TXWATER: u32::from(watermark))
            }
        }

        watermark
    }

    /// Set the SPI mode for the peripheral.
    ///
    /// This only affects the next transfer; ongoing transfers
    /// will not be influenced.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// Clear any existing data in the SPI receive or transfer FIFOs.
    ///
    /// Note that this will **not** cancel a running transfer.
    /// Use [`soft_reset()`](Self::soft_reset) for that usecase instead.
    #[inline]
    pub fn clear_fifo(&mut self, direction: Direction) {
        match direction {
            Direction::Tx => ral::modify_reg!(ral::lpspi, self.lpspi, CR, RTF: RTF_1),
            Direction::Rx => ral::modify_reg!(ral::lpspi, self.lpspi, CR, RRF: RRF_1),
        }
    }

    /// Clear both FIFOs.
    ///
    /// Note that this will **not** cancel a running transfer.
    /// Use [`soft_reset()`](Self::soft_reset) for that usecase instead.
    pub fn clear_fifos(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, RTF: RTF_1, RRF: RRF_1);
    }

    /// Returns the watermark level for the given direction.
    #[inline]
    pub fn watermark(&self, direction: Direction) -> u16 {
        (match direction {
            Direction::Rx => ral::read_reg!(ral::lpspi, self.lpspi, FCR, RXWATER),
            Direction::Tx => ral::read_reg!(ral::lpspi, self.lpspi, FCR, TXWATER),
        }) as u16
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
            if fifo_status.txcount < self.tx_fifo_size {
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
        ral::write_reg!(ral::lpspi, self.lpspi, TCR,
            CPOL: if self.mode.polarity == Polarity::IdleHigh {CPOL_1} else {CPOL_0},
            CPHA: if self.mode.phase == Phase::CaptureOnSecondTransition {CPHA_1} else {CPHA_0},
            PRESCALE: PRESCALE_0,
            PCS: PCS_0,
            WIDTH: WIDTH_0,
            LSBF: transaction.bit_order as u32,
            BYSW: transaction.byte_swap as u32,
            RXMSK: transaction.receive_data_mask as u32,
            TXMSK: transaction.transmit_data_mask as u32,
            FRAMESZ: transaction.frame_size as u32,
            CONT: transaction.continuous as u32,
            CONTC: transaction.continuing as u32
        );
    }

    /// Wait for all ongoing transactions to be finished.
    pub fn flush(&mut self) -> Result<(), LpspiError> {
        loop {
            let status = self.status();

            if status.intersects(Status::RECEIVE_ERROR) {
                return Err(LpspiError::Fifo(Direction::Rx));
            }
            if status.intersects(Status::TRANSMIT_ERROR) {
                return Err(LpspiError::Fifo(Direction::Tx));
            }

            if !status.intersects(Status::BUSY) {
                return Ok(());
            }
        }
    }

    /// Exchanges data with the SPI device.
    ///
    /// This routine uses continuous transfers to perform the transaction, no matter the
    /// primitive type. There's an optimization for &[u32] that we're missing; in this case,
    /// we don't necessarily need to use continuous transfers. The frame size could be set to
    /// 8 * buffer.len() * sizeof(u32), and we copy user words into the transmit queue as-is.
    /// But handling the packing of u8s and u16s into the u32 transmit queue in software is
    /// extra work, work that's effectively achieved when we use continuous transfers.
    /// We're guessing that the time to pop a transmit command from the queue is much faster
    /// than the time taken to pop from the data queue, so the extra queue utilization shouldn't
    /// matter.
    fn exchange<W>(&mut self, buffer: &mut [W]) -> Result<(), LpspiError>
    where
        W: Word,
    {
        if self.status().intersects(Status::BUSY) {
            return Err(LpspiError::Busy);
        } else if buffer.is_empty() {
            return Err(LpspiError::NoData);
        }

        self.clear_fifos();

        let mut transaction = Transaction::new(8 * core::mem::size_of::<W>() as u16)?;
        transaction.bit_order = self.bit_order();
        transaction.continuous = true;

        let mut tx_idx = 0usize;
        let mut rx_idx = 0usize;

        // Continue looping while there is either tx OR rx remaining
        while tx_idx < buffer.len() || rx_idx < buffer.len() {
            if tx_idx < buffer.len() {
                let word = buffer[tx_idx];

                // Turn off TCR CONT on last tx as a workaround so that the final
                // falling edge comes through:
                // https://community.nxp.com/t5/i-MX-RT/RT1050-LPSPI-last-bit-not-completing-in-continuous-mode/m-p/898460
                if tx_idx + 1 == buffer.len() {
                    transaction.continuous = false;
                }

                self.wait_for_transmit_fifo_space()?;
                self.enqueue_transaction(&transaction);

                self.wait_for_transmit_fifo_space()?;
                self.enqueue_data(word.into());
                transaction.continuing = true;
                tx_idx += 1;
            }

            if rx_idx < buffer.len() {
                self.recv_ok()?;
                if let Some(word) = self.read_data() {
                    buffer[rx_idx] = word.try_into().unwrap_or(W::MAX);
                    rx_idx += 1;
                }
            }
        }

        Ok(())
    }

    /// Write data to the transmit queue without subsequently reading
    /// the receive queue.
    ///
    /// Use this method when you know that the receiver queue is disabled
    /// (RXMASK high in TCR).
    ///
    /// Similar to `exchange`, this is using continuous transfers for all supported primitives.
    fn write_no_read<W>(&mut self, buffer: &[W]) -> Result<(), LpspiError>
    where
        W: Word,
    {
        if self.status().intersects(Status::BUSY) {
            return Err(LpspiError::Busy);
        } else if buffer.is_empty() {
            return Err(LpspiError::NoData);
        }

        self.clear_fifos();

        let mut transaction = Transaction::new(8 * core::mem::size_of::<W>() as u16)?;
        transaction.bit_order = self.bit_order();
        transaction.continuous = true;
        transaction.receive_data_mask = true;

        for word in buffer {
            self.wait_for_transmit_fifo_space()?;
            self.enqueue_transaction(&transaction);

            self.wait_for_transmit_fifo_space()?;
            self.enqueue_data((*word).into());
            transaction.continuing = true;
        }

        transaction.continuing = false;
        transaction.continuous = false;

        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);

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

        // Request disable
        ral::modify_reg!(ral::lpspi, lpspi, CR, MEN: MEN_0);
        // Wait for the driver to finish its current transfer
        // and enter disabled state
        while ral::read_reg!(ral::lpspi, lpspi, CR, MEN == MEN_1) {}

        Self { lpspi, men }
    }

    /// Set the LPSPI clock speed (Hz).
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn set_clock_hz(&mut self, source_clock_hz: u32, clock_hz: u32) {
        set_spi_clock(source_clock_hz, clock_hz, self.lpspi);
    }

    /// Set the sampling point of the LPSPI peripheral.
    ///
    /// When set to `SamplePoint::DelayedEdge`, the LPSPI will sample the input data
    /// on a delayed LPSPI_SCK edge, which improves the setup time when sampling data.
    #[inline]
    pub fn set_sample_point(&mut self, sample_point: SamplePoint) {
        match sample_point {
            SamplePoint::Edge => ral::modify_reg!(ral::lpspi, self.lpspi, CFGR1, SAMPLE: SAMPLE_0),
            SamplePoint::DelayedEdge => {
                ral::modify_reg!(ral::lpspi, self.lpspi, CFGR1, SAMPLE: SAMPLE_1)
            }
        }
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

/// Describes SPI words that can participate in transactions.
trait Word: Copy + Into<u32> + TryFrom<u32> {
    const MAX: Self;
}

impl Word for u8 {
    const MAX: u8 = u8::MAX;
}

impl Word for u16 {
    const MAX: u16 = u16::MAX;
}

impl Word for u32 {
    const MAX: u32 = u32::MAX;
}
