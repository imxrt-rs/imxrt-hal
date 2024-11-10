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
//! The iMXRT SPI peripherals have one or more hardware-controlled chip selects (CS). Using
//! the hardware-controlled CS means that you do not need a GPIO to coordinate SPI operations.
//! The driver nevertheless works with software-managed CS.
//!
//! Generally, an [`Lpspi`] is capable of using hardware-controlled chip select. However, it only
//! does that if you mux your hardware chip select to the LPSPI instance. The device abstractions
//! provided by this module can do that for you.
//!
//! Here's how [`Lpspi`] implements the various embedded-hal traits, when considering hardware-
//! and software-managed CS.
//!
//! - `Lpspi` implements the blocking traits defined in embedded-hal 0.2. If you're using
//!   software-managed CS, then you're responsible for toggling your pin as required. If you're
//!   using hardware-managed CS, then the transactions performed on the bus use [`Pcs::Pcs0`]
//!   by default, or the value you supply to [`Lpspi::set_chip_select`].
//! - `Lpspi` implements the blocking *bus* traits defined in embedded-hal 1.0. If you're using
//!   software-managed CS, you can use `embedded-hal-bus` to pair `Lpspi` with a GPIO. If you're
//!   using hardware-managed CS, use [`ExclusiveDevice`] or [`RefCellDevice`] to pair `Lpspi`
//!   with a chip select.
//!
//! # Device support
//!
//! By default, the driver behaves as a SPI controller, coordinating I/O for other SPI peripherals.
//! To behave like a peripheral, use [`set_peripheral_enable`](Disabled::set_peripheral_enable).
//!
//! As of this writing, you're expected to use the lower-level interface to perform device I/O.
//!
//! # Example
//!
//! Initialize an LPSPI controller with a 1MHz SCK. To understand how to configure the LPSPI
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
//!
//! Since we cannot reliably use continuous transactions, we're forced to use a single transaction to
//! realize `SpiDevice` transactions. This limits the total number of bytes transacted to 512.

use core::cell::RefCell;
use core::marker::PhantomData;
use core::task::Poll;

use crate::iomuxc::{consts, lpspi};
use crate::ral;

pub use eh02::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use eh1::spi::Operation;

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
}

/// Peripheral chip select.
///
/// Use this in [`Transaction`] to select the hardware-managed
/// chip select. Note that the hardware only uses the hardware-managed
/// chip select if the pin is muxed for its PCS alternate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum Pcs {
    /// Use PCS0 (default) for the next transaction.
    #[default]
    Pcs0,
    /// Use PCS1 for the next transaction.
    Pcs1,
    /// Use PCS2 for the next transaction.
    Pcs2,
    /// Use PCS3 for the next transaction.
    Pcs3,
}

/// The hardware chip select polarity.
///
/// Use [`Disabled::set_chip_select_polarity`] to configure
/// each chip select's polarity. Consult your peripheral's
/// documentation to understand which polarity is expected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum PcsPolarity {
    /// The chip select is active low.
    ///
    /// When idle, the chip select is high. This is
    /// the default state.
    #[default]
    ActiveLow,
    /// The chip select is active high.
    ///
    /// When idle, the chip select is low.
    ActiveHigh,
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
/// - the hardware-managed peripheral chip select, [`Pcs`]
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
    /// The SPI mode for the transaction.
    ///
    /// By default, this is [`MODE_0`].
    pub mode: Mode,
    /// Selects the hardware-managed peripheral chip select for the transaction.
    ///
    /// See [`Pcs`] for more information.
    pub pcs: Pcs,
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
        let bits = frame_size(data)?;
        Transaction::new(bits)
    }

    fn frame_size_valid(frame_size: u16) -> bool {
        const MIN_FRAME_SIZE: u16 = 8;
        const MAX_FRAME_SIZE: u16 = 1 << 12;
        const WORD_SIZE: u16 = 32;

        let last_frame_size = frame_size % WORD_SIZE;

        (MIN_FRAME_SIZE..=MAX_FRAME_SIZE).contains(&frame_size) && (1 != last_frame_size)
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
                mode: MODE_0,
                pcs: Default::default(),
            })
        } else {
            Err(LpspiError::FrameSize)
        }
    }
}

fn frame_size<W>(buffer: &[W]) -> Result<u16, LpspiError> {
    let bits = size_of_val(buffer)
        .checked_mul(8)
        .ok_or(LpspiError::FrameSize)?;

    bits.try_into().map_err(|_| LpspiError::FrameSize)
}

/// Sets the clock speed parameters.
///
/// This should only happen when the LPSPI peripheral is disabled.
fn compute_spi_clock(source_clock_hz: u32, spi_clock_hz: u32) -> ClockConfigs {
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

    ClockConfigs {
        // Delay between two clock transitions of two consecutive transfers
        // is exactly sckdiv/2, which causes the transfer to be seamless.
        dbt: (half_div - 1) as u8,
        // Add one sckdiv/2 setup and hold time before and after the transfer,
        // to make sure the signal is stable at sample time
        pcssck: (half_div - 1) as u8,
        sckpcs: (half_div - 1) as u8,
        sckdiv: sckdiv as u8,
    }
}

/// LPSPI clock configurations.
///
/// This is a low-level API. You should prefer [`set_clock_hz`](Disabled::set_clock_hz)
/// if you're not sure how to use these configurations.
///
/// All delays and dividers are in terms of the LPSPI functional clock cycles. They're
/// written directly to the corresponding clock configuration register fields. See inline
/// documentation to understand what values of zero represent.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClockConfigs {
    /// SCK-to-PCS delay.
    ///
    /// This value is off-by-one: a value of zero indicates one cycle.
    pub sckpcs: u8,
    /// PCS-to-SCK delay.
    ///
    /// This value is off-by-one: a value of zero indicates one cycle.
    pub pcssck: u8,
    /// Delay between transfers.
    ///
    /// For normal transactions, this affects the PCS negation duration. In this
    /// configuration, this value is off-by-two: a value of zero indicates two cycle.
    ///
    /// For continuous transactions, this affects the clock delay between words.
    /// In this configuration, this value is off-by-one.
    pub dbt: u8,
    /// SCK divider.
    ///
    /// This value is off-by-two: a value of zero indicates two cycles.
    pub sckdiv: u8,
}

impl ClockConfigs {
    const fn as_raw(self) -> u32 {
        use ral::lpspi::CCR;
        (self.sckpcs as u32) << CCR::SCKPCS::offset
            | (self.pcssck as u32) << CCR::PCSSCK::offset
            | (self.dbt as u32) << CCR::DBT::offset
            | (self.sckdiv as u32) << CCR::SCKDIV::offset
    }
}

/// In-memory clock configuration register values.
///
/// Newer LPSPI IP blocks, like those found on the 1180, have `CCR[DBT]`
/// and `CCR[SCKDIV]` fields that are WO/RAZ. RAZ is incompatible
/// with older IP blocks, like those found on all the other MCUs.
///
/// If we cache these values, all RMW on CCR will behave as if we
/// read those values through the register.
struct CcrCache {
    dbt: u8,
    sckdiv: u8,
}

impl CcrCache {
    /// Reac the clock configuration values, considering the cached values.
    fn read_ccr(&self, lpspi: &ral::lpspi::RegisterBlock) -> ClockConfigs {
        let (sckpcs, pcssck) = ral::read_reg!(ral::lpspi, lpspi, CCR, SCKPCS, PCSSCK);
        ClockConfigs {
            sckpcs: sckpcs as u8,
            pcssck: pcssck as u8,
            dbt: self.dbt,
            sckdiv: self.sckdiv,
        }
    }
    /// Update the cached values.
    fn update(&mut self, clock_configs: ClockConfigs) {
        self.dbt = clock_configs.dbt;
        self.sckdiv = clock_configs.sckdiv;
    }
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
    mode: Mode,
    ccr_cache: CcrCache,
    pcs: Pcs,
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
        let spi = Lpspi {
            lpspi,
            pins,
            bit_order: BitOrder::default(),
            mode: MODE_0,
            // Once we issue a reset, below, these are zero.
            ccr_cache: CcrCache { dbt: 0, sckdiv: 0 },
            pcs: Pcs::default(),
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

        let tx_fifo_size = spi.max_watermark(Direction::Tx);
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

    /// Returns the hardware chip select used by the LPSPI bus.
    ///
    /// If you have not multiplexed a PCS pin to this LPSPI instance,
    /// then the value is meaningless.
    pub fn chip_select(&self) -> Pcs {
        self.pcs
    }

    /// Set the hardware chip select used by this LPSPI bus.
    ///
    /// This affects all subsequent transactions. In-flight transactions,
    /// if any, are not affected. If you have not multiplexed a PCS pin
    /// to this LPSPI instance, then this configuration is meaningless.
    pub fn set_chip_select(&mut self, pcs: Pcs) {
        self.pcs = pcs;
    }

    /// Temporarily disable the LPSPI peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpspi::Disabled) driver lets you modify
    /// LPSPI settings that require a fully disabled peripheral.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        let mut disabled = Disabled::new(&mut self.lpspi, &mut self.ccr_cache);
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

    /// Place `word` into the transmit FIFO.
    ///
    /// This will result in the value being sent from the LPSPI.
    /// You're responsible for making sure that the transmit FIFO can
    /// fit this word.
    pub fn enqueue_data(&self, word: u32) {
        ral::write_reg!(ral::lpspi, self.lpspi, TDR, word);
    }

    /// Wait for transmit FIFO space in a (concurrent) spin loop.
    ///
    /// This future does not care about the TX FIFO watermark. Instead, it
    /// checks the FIFO's size with an additional read.
    pub(crate) async fn spin_for_fifo_space(&self) -> Result<(), LpspiError> {
        core::future::poll_fn(|_| {
            let status = self.status();
            if status.intersects(Status::TRANSMIT_ERROR) {
                return Poll::Ready(Err(LpspiError::Fifo(Direction::Tx)));
            }
            let fifo_status = self.fifo_status();
            if !fifo_status.is_full(Direction::Tx) {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Spin until the transmit FIFO is empty.
    ///
    /// Check for both receive and transmit errors, allowing us to realize
    /// a flush operation.
    async fn spin_for_fifo_exhaustion(&self) -> Result<(), LpspiError> {
        core::future::poll_fn(|_| {
            let status = self.status();

            if status.intersects(Status::RECEIVE_ERROR) {
                return Poll::Ready(Err(LpspiError::Fifo(Direction::Rx)));
            }
            if status.intersects(Status::TRANSMIT_ERROR) {
                return Poll::Ready(Err(LpspiError::Fifo(Direction::Tx)));
            }

            // Contributor testing reveals that the busy flag may not set once
            // the TX FIFO is filled. This means a sequence like
            //
            //     lpspi.write(&[...])?;
            //     lpspi.flush()?;
            //
            // could pass through flush without observing busy. Therefore, we
            // also check the FIFO contents. Even if the peripheral isn't
            // busy, the FIFO should be non-empty.
            //
            // We can't just rely on the FIFO contents, since the FIFO could be
            // empty while the transaction is completing. (There's data in the
            // shift register, and PCS is still asserted.)
            if !status.intersects(Status::BUSY) && self.fifo_status().is_empty(Direction::Tx) {
                return Poll::Ready(Ok(()));
            }

            Poll::Pending
        })
        .await
    }

    pub(crate) fn wait_for_transmit_fifo_space(&self) -> Result<(), LpspiError> {
        crate::spin_on(self.spin_for_fifo_space())
    }

    /// Wait for receive data in a (concurrent) spin loop.
    ///
    /// This future does not care about the RX FIFO watermark. Instead, it
    /// checks the FIFO's size with an additional read.
    async fn spin_for_word(&self) -> Result<u32, LpspiError> {
        core::future::poll_fn(|_| {
            let status = self.status();
            if status.intersects(Status::RECEIVE_ERROR) {
                return Poll::Ready(Err(LpspiError::Fifo(Direction::Rx)));
            }

            let fifo_status = self.fifo_status();
            if !fifo_status.is_empty(Direction::Rx) {
                let data = self.read_data_unchecked();
                Poll::Ready(Ok(data))
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Send `len` LPSPI words (u32s) out of the peripheral.
    ///
    /// Expected to run in a (concurrent) spin loop, possibly with
    /// `spin_receive`.
    async fn spin_transmit(
        &self,
        mut data: impl TransmitData,
        len: usize,
    ) -> Result<(), LpspiError> {
        for _ in 0..len {
            self.spin_for_fifo_space().await?;
            let word = data.next_word(self.bit_order);
            self.enqueue_data(word);
        }
        Ok(())
    }

    /// Accept `len` LPSPI words (u32s) from the peripheral.
    ///
    /// Expected to run in a (concurrent) spin loop, possibly with
    /// `spin_transmit`.
    async fn spin_receive(&self, mut data: impl ReceiveData, len: usize) -> Result<(), LpspiError> {
        for _ in 0..len {
            let word = self.spin_for_word().await?;
            data.next_word(self.bit_order, word);
        }
        Ok(())
    }

    /// Set the SPI mode for the peripheral.
    ///
    /// This only affects the next transfer; ongoing transfers
    /// will not be influenced.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// Place a transaction definition into the transmit FIFO.
    ///
    /// Once this definition is popped from the transmit FIFO, this may
    /// affect, or abort, any ongoing transactions.
    ///
    /// You're responsible for making sure there's space in the transmit
    /// FIFO for this transaction command.
    pub fn enqueue_transaction(&self, transaction: &Transaction) {
        ral::write_reg!(ral::lpspi, self.lpspi, TCR,
            CPOL: if transaction.mode.polarity == Polarity::IdleHigh { CPOL_1 } else { CPOL_0 },
            CPHA: if transaction.mode.phase == Phase::CaptureOnSecondTransition { CPHA_1 } else { CPHA_0 },
            PRESCALE: PRESCALE_0,
            PCS: PCS_0,
            WIDTH: WIDTH_0,
            LSBF: transaction.bit_order as u32,
            BYSW: transaction.byte_swap as u32,
            RXMSK: transaction.receive_data_mask as u32,
            TXMSK: transaction.transmit_data_mask as u32,
            FRAMESZ: transaction.frame_size as u32,
            CONT: transaction.continuous as u32,
            CONTC: transaction.continuing as u32,
            PCS: transaction.pcs as u32
        );
    }

    /// Wait for all ongoing transactions to be finished.
    pub fn flush(&mut self) -> Result<(), LpspiError> {
        crate::spin_on(self.spin_for_fifo_exhaustion())
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

    fn max_watermark(&self, direction: Direction) -> u8 {
        (match direction {
            Direction::Rx => 1 << ral::read_reg!(ral::lpspi, self.lpspi, PARAM, RXFIFO),
            Direction::Tx => 1 << ral::read_reg!(ral::lpspi, self.lpspi, PARAM, TXFIFO),
        }) as u8
    }

    /// Reset all internal logic while preserving the driver's configurations.
    ///
    /// Unlike [`reset()`](Self::reset), this preserves all peripheral registers.
    pub fn soft_reset(&mut self) {
        // Backup previous registers
        let ier = ral::read_reg!(ral::lpspi, self.lpspi, IER);
        let der = ral::read_reg!(ral::lpspi, self.lpspi, DER);
        let cfgr0 = ral::read_reg!(ral::lpspi, self.lpspi, CFGR0);
        let cfgr1 = ral::read_reg!(ral::lpspi, self.lpspi, CFGR1);
        let dmr0 = ral::read_reg!(ral::lpspi, self.lpspi, DMR0);
        let dmr1 = ral::read_reg!(ral::lpspi, self.lpspi, DMR1);
        let ccr = self.clock_configs().as_raw();
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
        set_watermark(&self.lpspi, direction, watermark)
    }

    /// Recover from a transaction error.
    fn recover_from_error(&mut self) {
        // Resets the peripheral and flushes whatever is in the FIFOs.
        self.soft_reset();

        // Reset the status flags, clearing the error condition for the next use.
        self.clear_status(Status::TRANSMIT_ERROR | Status::RECEIVE_ERROR);
    }

    /// Return the driver's clock configurations.
    ///
    /// These values are decided by calls to [`set_clock_hz`](Disabled::set_clock_hz)
    /// and [`set_clock_configs`](Disabled::set_clock_configs).
    pub fn clock_configs(&self) -> ClockConfigs {
        self.ccr_cache.read_ccr(&self.lpspi)
    }

    /// Produce a transaction that considers bus-managed software state.
    pub(crate) fn bus_transaction<W>(&self, words: &[W]) -> Result<Transaction, LpspiError> {
        self.transaction_for_frame_size(frame_size(words)?)
    }

    fn transaction_for_frame_size(&self, bits: u16) -> Result<Transaction, LpspiError> {
        let mut transaction = Transaction::new(bits)?;
        transaction.bit_order = self.bit_order();
        transaction.mode = self.mode;
        transaction.pcs = self.chip_select();
        Ok(transaction)
    }

    fn operation_transaction<W>(
        &self,
        op: &Operation<W>,
    ) -> Result<Option<Transaction>, LpspiError> {
        let buffer: &[W] = match op {
            Operation::DelayNs(_) => return Ok(None),
            Operation::Read(buffer) => buffer,
            Operation::Write(buffer) => buffer,
            Operation::TransferInPlace(buffer) => buffer,
            Operation::Transfer(recv, send) => {
                if recv.len() > send.len() {
                    recv
                } else {
                    send
                }
            }
        };
        Transaction::new_words(buffer).map(Some)
    }

    /// Perform one or more operations within an LPSPI transaction.
    ///
    /// If you know there's no delay operations, you may pass in a no-op / panicking
    /// `delay_ns`. Any operation with an empty buffer is skipped.
    ///
    /// The implementation executes all operations within a single, non-continuous
    /// transaction. We can't use a continuous transaction without risking the
    /// undocumented hardware errata described in this module's documentation.
    /// This means users are limited to transacting 512 bytes across all operations.
    ///
    /// This will not flush when complete. Callers may need to do that themselves.
    fn transact<W: Word>(
        &mut self,
        operations: &mut [Operation<W>],
        mut delay_ns: impl FnMut(u32),
    ) -> Result<(), LpspiError> {
        if operations.is_empty() {
            return Ok(());
        }

        for op in operations.iter() {
            self.operation_transaction(op)?;
        }

        let len = operations.len();
        let (tx_schedule, rx_schedule) = schedule(operations);

        let tx_state_machine = async {
            let mut continuing = false;
            for mut desc in tx_schedule {
                // This delay is blocking, so it stalls the RX state machine.
                // We need to prevent an RX FIFO overrun while we're stalled.
                // To prevent that, we'll flush the transmit FIFO before delaying.
                // This increases the transaction latency when writes operations
                // preceded delay operations, but this is acceptable; our delay
                // is "at least" best effort.
                if let Some(ns) = desc.delay_ns() {
                    self.spin_for_fifo_exhaustion().await?;
                    delay_ns(ns);
                    continue;
                }

                let mut transaction = self.transaction_for_frame_size(desc.frame_size)?;
                transaction.continuous = len > 1;
                transaction.continuing = continuing;

                self.spin_for_fifo_space().await?;
                self.enqueue_transaction(&transaction);

                let total_words = desc.total_words();
                if let Some(tx_buffer) = desc.tx_buffer.take() {
                    self.spin_transmit(tx_buffer, desc.tx_words).await?;
                }

                self.spin_transmit(TransmitDummies, total_words.saturating_sub(desc.tx_words))
                    .await?;

                continuing = true;
            }

            let fin = self.transaction_for_frame_size(8).unwrap();
            self.spin_for_fifo_space().await?;
            self.enqueue_transaction(&fin);

            Ok(())
        };

        let rx_state_machine = async {
            for mut desc in rx_schedule {
                if desc.is_delay_ns() {
                    continue;
                }
                let total_words = desc.total_words();
                if let Some(rx_buffer) = desc.rx_buffer.take() {
                    self.spin_receive(rx_buffer, desc.rx_words).await?;
                }

                self.spin_receive(ReceiveDummies, total_words.saturating_sub(desc.rx_words))
                    .await?;
            }
            Ok(())
        };

        // Run those transmit and receive state machines together.
        crate::spin_on(futures::future::try_join(
            tx_state_machine,
            rx_state_machine,
        ))
        .inspect_err(|_| self.recover_from_error())?;

        Ok(())
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
    /// Indicates if the FIFO is empty for the given direction.
    #[inline]
    const fn is_empty(self, direction: Direction) -> bool {
        0 == match direction {
            Direction::Tx => self.txcount,
            Direction::Rx => self.rxcount,
        }
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

#[inline]
fn set_watermark(lpspi: &ral::lpspi::RegisterBlock, direction: Direction, watermark: u8) -> u8 {
    let max_watermark = match direction {
        Direction::Rx => 1 << ral::read_reg!(ral::lpspi, lpspi, PARAM, RXFIFO),
        Direction::Tx => 1 << ral::read_reg!(ral::lpspi, lpspi, PARAM, TXFIFO),
    };

    let watermark = watermark.min(max_watermark - 1);

    match direction {
        Direction::Rx => {
            ral::modify_reg!(ral::lpspi, lpspi, FCR, RXWATER: watermark as u32)
        }
        Direction::Tx => {
            ral::modify_reg!(ral::lpspi, lpspi, FCR, TXWATER: watermark as u32)
        }
    }

    watermark
}

/// An LPSPI peripheral which is temporarily disabled.
pub struct Disabled<'a, const N: u8> {
    lpspi: &'a ral::lpspi::Instance<N>,
    men: bool,
    ccr_cache: &'a mut CcrCache,
}

impl<'a, const N: u8> Disabled<'a, N> {
    fn new(lpspi: &'a mut ral::lpspi::Instance<N>, ccr_cache: &'a mut CcrCache) -> Self {
        let men = ral::read_reg!(ral::lpspi, lpspi, CR, MEN == MEN_1);

        // Request disable
        ral::modify_reg!(ral::lpspi, lpspi, CR, MEN: MEN_0);
        // Wait for the driver to finish its current transfer
        // and enter disabled state
        while ral::read_reg!(ral::lpspi, lpspi, CR, MEN == MEN_1) {}
        Self {
            lpspi,
            men,
            ccr_cache,
        }
    }

    /// Set the LPSPI clock speed (Hz).
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn set_clock_hz(&mut self, source_clock_hz: u32, clock_hz: u32) {
        let clock_configs = compute_spi_clock(source_clock_hz, clock_hz);
        self.set_clock_configs(clock_configs);
    }

    /// Set LPSPI timing configurations.
    ///
    /// If you're not sure how to select these timing values, prefer
    /// [`set_clock_hz`](Self::set_clock_hz).
    pub fn set_clock_configs(&mut self, timing: ClockConfigs) {
        self.ccr_cache.update(timing);
        ral::write_reg!(ral::lpspi, self.lpspi, CCR,
            SCKPCS: timing.sckpcs as u32,
            PCSSCK: timing.pcssck as u32,
            DBT: timing.dbt as u32,
            SCKDIV: timing.sckdiv as u32,
        );
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

    /// Become an LPSPI peripheral.
    ///
    /// By default, the LPSPI driver acts as a controller, driving I/O.
    /// By enabling peripheral functions (`true`), you can accept
    /// and react to another controller's I/O. When you're acting as a
    /// peripheral, you don't control the clock and chip select lines.
    #[inline]
    pub fn set_peripheral_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CFGR1, MASTER: !enable as u32);
    }

    /// Set the polarity for the `pcs` hardware chip select.
    ///
    /// By default, all polarities are active low.
    #[inline]
    pub fn set_chip_select_polarity(&mut self, pcs: Pcs, polarity: PcsPolarity) {
        let pcspol = ral::read_reg!(ral::lpspi, self.lpspi, CFGR1, PCSPOL);
        let mask = 1 << pcs as u32;
        let pcspol = if polarity == PcsPolarity::ActiveHigh {
            pcspol | mask
        } else {
            pcspol & !mask
        };
        ral::modify_reg!(ral::lpspi, self.lpspi, CFGR1, PCSPOL: pcspol);
    }
}

fn no_delay_needed(_: u32) {
    unreachable!()
}

impl<const N: u8> Drop for Disabled<'_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi, CR, MEN: self.men as u32);
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u8> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u16> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u16]) -> Result<&'a [u16], Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Transfer<u32> for Lpspi<P, N> {
    type Error = LpspiError;

    fn transfer<'a>(&mut self, words: &'a mut [u32]) -> Result<&'a [u32], Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)?;
        Ok(words)
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u8> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)?;
        self.flush()?;
        Ok(())
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u16> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)?;
        self.flush()?;
        Ok(())
    }
}

impl<P, const N: u8> eh02::blocking::spi::Write<u32> for Lpspi<P, N> {
    type Error = LpspiError;

    fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)?;
        self.flush()?;
        Ok(())
    }
}

impl eh1::spi::Error for LpspiError {
    fn kind(&self) -> eh1::spi::ErrorKind {
        use eh1::spi::ErrorKind;

        match self {
            LpspiError::Fifo(Direction::Rx) => ErrorKind::Overrun,
            _ => ErrorKind::Other,
        }
    }
}

impl<P, const N: u8> eh1::spi::ErrorType for Lpspi<P, N> {
    type Error = LpspiError;
}

impl<P, const N: u8> eh1::spi::SpiBus<u8> for Lpspi<P, N> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Read(words)], no_delay_needed)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Transfer(read, write)], no_delay_needed)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Lpspi::flush(self)
    }
}

impl<P, const N: u8> eh1::spi::SpiBus<u16> for Lpspi<P, N> {
    fn read(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Read(words)], no_delay_needed)
    }

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)
    }

    fn transfer(&mut self, read: &mut [u16], write: &[u16]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Transfer(read, write)], no_delay_needed)
    }

    fn transfer_in_place(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Lpspi::flush(self)
    }
}

impl<P, const N: u8> eh1::spi::SpiBus<u32> for Lpspi<P, N> {
    fn read(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Read(words)], no_delay_needed)
    }

    fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Write(words)], no_delay_needed)
    }

    fn transfer(&mut self, read: &mut [u32], write: &[u32]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::Transfer(read, write)], no_delay_needed)
    }

    fn transfer_in_place(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        self.transact(&mut [Operation::TransferInPlace(words)], no_delay_needed)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Lpspi::flush(self)
    }
}

// Not supporting WriteIter right now. Since we don't know how many bytes we're
// going to write, we can't specify the frame size. There might be ways around
// this by playing with CONTC and CONT bits, but we can evaluate that later.

/// Describes SPI words that can participate in transactions.
trait Word: Copy + Into<u32> + TryFrom<u32> {
    /// Repeatedly call `provider` to produce yourself,
    /// then turn yourself into a LPSPI word.
    fn pack_word(bit_order: BitOrder, provider: impl FnMut() -> Option<Self>) -> u32;

    /// Given a word, deconstruct the word and call the
    /// `sink` with those components. `valid_bytes` conveys
    /// how many bytes in `word` are valid. It's never more
    /// than four, and it's never zero.
    fn unpack_word(word: u32, bit_order: BitOrder, valid_bytes: usize, sink: impl FnMut(Self));
}

impl Word for u8 {
    fn pack_word(bit_order: BitOrder, mut provider: impl FnMut() -> Option<Self>) -> u32 {
        let mut word = 0;
        match bit_order {
            BitOrder::Msb => {
                for _ in 0..4 {
                    if let Some(byte) = provider() {
                        word <<= 8;
                        word |= u32::from(byte);
                    }
                }
            }
            BitOrder::Lsb => {
                for offset in 0..4 {
                    if let Some(byte) = provider() {
                        word |= u32::from(byte) << (8 * offset);
                    }
                }
            }
        }

        word
    }
    fn unpack_word(word: u32, bit_order: BitOrder, valid_bytes: usize, mut sink: impl FnMut(Self)) {
        let mut offsets = [0usize, 8, 16, 24];
        let valid = &mut offsets[..valid_bytes];
        if matches!(bit_order, BitOrder::Msb) {
            valid.reverse();
        }
        for offset in valid {
            sink((word >> *offset) as u8);
        }
    }
}

impl Word for u16 {
    fn pack_word(bit_order: BitOrder, mut provider: impl FnMut() -> Option<Self>) -> u32 {
        let mut word = 0;
        match bit_order {
            BitOrder::Msb => {
                for _ in 0..2 {
                    if let Some(half) = provider() {
                        word <<= 16;
                        word |= u32::from(half);
                    }
                }
            }
            BitOrder::Lsb => {
                for offset in 0..2 {
                    if let Some(half) = provider() {
                        word |= u32::from(half) << (16 * offset);
                    }
                }
            }
        }

        word
    }
    fn unpack_word(word: u32, bit_order: BitOrder, valid_bytes: usize, mut sink: impl FnMut(Self)) {
        let mut offsets = [0usize, 16];
        let valid = &mut offsets[..valid_bytes / 2];

        if matches!(bit_order, BitOrder::Msb) {
            valid.reverse();
        }

        for offset in valid {
            sink((word >> *offset) as u16);
        }
    }
}

impl Word for u32 {
    fn pack_word(_: BitOrder, mut provider: impl FnMut() -> Option<Self>) -> u32 {
        provider().unwrap_or(0)
    }
    fn unpack_word(word: u32, _: BitOrder, _: usize, mut sink: impl FnMut(Self)) {
        sink(word)
    }
}

/// Low-level description of an LPSPI operation.
///
/// Interpretation depends on the TX or RX state machine.
/// The TX state machine interprets a delay using `delay_ns`.
struct Descriptor<'w, W> {
    frame_size: u16,
    tx_buffer: Option<TransmitBuffer<'w, W>>,
    tx_words: usize,
    rx_buffer: Option<ReceiveBuffer<'w, W>>,
    rx_words: usize,
}

impl<W> Descriptor<'_, W> {
    /// The number of 32-bit LPSPI words transacted by this descriptor.
    fn total_words(&self) -> usize {
        self.tx_words.max(self.rx_words)
    }

    fn is_delay_ns(&self) -> bool {
        self.tx_buffer.is_none() && self.rx_buffer.is_none()
    }

    /// Returns the delay, in nanoseconds, if this descriptor describes
    /// a delay operation.
    fn delay_ns(&self) -> Option<u32> {
        if self.is_delay_ns() {
            Some(self.tx_words as u32)
        } else {
            None
        }
    }
}

/// The schedule splits the user's SPI operations for transmit
/// and receive state machines.
///
/// Use [`schedule`] to produce two of these, one per state machine.
/// Then, execute the [`Descriptor`]s produced by the schedule.
struct Schedule<'o, 'w, W: 'static> {
    /// Our position in the user's (slice of) operations.
    ptr: *mut Operation<'w, W>,
    /// The address at the end of the slice.
    end: *mut Operation<'w, W>,
    /// Capture the lifetime of the (slice of) operations.
    _lt: PhantomData<&'o mut [Operation<'w, W>]>,
}

impl<'w, W: Word + 'static> Iterator for Schedule<'_, 'w, W> {
    type Item = Descriptor<'w, W>;
    fn next(&mut self) -> Option<Self::Item> {
        // Safety: We do not form multiple mutable references to either
        // the Operation variants, or the mutable buffers wrapped by Operation
        // variants. The pointer-len bounds are in bounds, and their lifetimes
        // do not exceed the lifetimes of the derived slices.
        unsafe {
            if self.ptr == self.end {
                return None;
            }

            /// Access a pointer to the inner slice.
            fn decay_ref_ref_slice<W>(buffer: &&[W]) -> *const W {
                buffer.as_ptr()
            }

            /// Access the pointer to the inner slice.
            fn decay_ref_mut_slice<W>(buffer: &&mut [W]) -> *mut W {
                // Safety: see inline notes.
                unsafe {
                    // Dispell the shared reference which conveys that the
                    // inner exclusive reference cannot be mutated.
                    let buffer: *const &mut [W] = buffer;

                    // We know that &mut [T] -> *mut [T] is OK.
                    // Perform the same cast through the outer pointer.
                    let buffer: *const *mut [W] = buffer.cast();

                    // Get the fat pointer. We're reading initialized
                    // and aligned memory, since the outer shared
                    // reference must be initialized and aligned.
                    let buffer: *mut [W] = buffer.read();

                    // This is
                    //
                    //  impl *mut [T] { pub fn as_ptr(self) }
                    //
                    // which isn't yet stable.
                    buffer as *mut W
                }
            }

            // Shared reference to an operation from
            // a mutable pointer. This shared reference
            // can exist with other shared references.
            // (We'll also never form multiple shared
            // references, since iteration of all schedules
            // occurs in the same execution context).
            let op: &Operation<'w, W> = &*self.ptr;
            let descriptor = match op {
                // Sentinel handled by the transmit state machine.
                Operation::DelayNs(ns) => Descriptor {
                    frame_size: 0,
                    tx_buffer: None,
                    tx_words: *ns as _,
                    rx_buffer: None,
                    rx_words: 0,
                },
                Operation::Read(buffer) => {
                    let frame_size = frame_size(buffer).unwrap();
                    let len = buffer.len();
                    let word_count = word_count(buffer);
                    let buffer: *mut W = decay_ref_mut_slice(buffer);
                    let rx = ReceiveBuffer::from_raw(buffer, len);
                    Descriptor {
                        frame_size,
                        tx_buffer: None,
                        tx_words: 0,
                        rx_buffer: Some(rx),
                        rx_words: word_count,
                    }
                }
                Operation::Write(buffer) => {
                    let frame_size = frame_size(buffer).unwrap();
                    let len = buffer.len();
                    let word_count = word_count(buffer);
                    let buffer = decay_ref_ref_slice(buffer);
                    let tx = TransmitBuffer::from_raw(buffer, len);
                    Descriptor {
                        frame_size,
                        tx_buffer: Some(tx),
                        tx_words: word_count,
                        rx_buffer: None,
                        rx_words: 0,
                    }
                }
                Operation::TransferInPlace(buffer) => {
                    let frame_size = frame_size(buffer).unwrap();
                    let len = buffer.len();
                    let word_count = word_count(buffer);
                    let buffer: *mut W = decay_ref_mut_slice(buffer);
                    let tx = TransmitBuffer::from_raw(buffer, len);
                    let rx = ReceiveBuffer::from_raw(buffer, len);
                    Descriptor {
                        frame_size,
                        tx_buffer: Some(tx),
                        tx_words: word_count,
                        rx_buffer: Some(rx),
                        rx_words: word_count,
                    }
                }
                Operation::Transfer(recv, send) => {
                    let frame_size = if recv.len() > send.len() {
                        frame_size(recv)
                    } else {
                        frame_size(send)
                    }
                    .unwrap();

                    let recv_words = word_count(recv);
                    let send_words = word_count(send);

                    let rx_len = recv.len();
                    let tx_len = send.len();

                    let recv: *mut W = decay_ref_mut_slice(recv);
                    let send: *const W = decay_ref_ref_slice(send);

                    let rx = ReceiveBuffer::from_raw(recv, rx_len);
                    let tx = TransmitBuffer::from_raw(send, tx_len);

                    Descriptor {
                        frame_size,
                        tx_buffer: Some(tx),
                        tx_words: send_words,
                        rx_buffer: Some(rx),
                        rx_words: recv_words,
                    }
                }
            };

            // Remains in bounds, or points to the element at the
            // end of the slice. Bounds check is at the top.
            self.ptr = self.ptr.add(1);

            Some(descriptor)
        }
    }
}

/// Produce a schedule, one for each TX and RX state machine.
fn schedule<'ops, 'w, W: Word + 'static>(
    ops: &mut [Operation<'w, W>],
) -> (Schedule<'ops, 'w, W>, Schedule<'ops, 'w, W>) {
    // Safety: We track the lifetime of all memory through
    // the schedule. Inspection of the Schedule interation
    // shows that we never form an exclusive reference to
    // any memory.
    unsafe {
        let len = ops.len();
        let ptr = ops.as_mut_ptr();
        let end = ptr.add(len);

        (
            Schedule {
                ptr,
                end,
                _lt: PhantomData,
            },
            Schedule {
                ptr,
                end,
                _lt: PhantomData,
            },
        )
    }
}

/// Generalizes how we prepare LPSPI words for transmit.
trait TransmitData {
    /// Get the next word for the transmit FIFO.
    ///
    /// If you're out of words, return 0.
    fn next_word(&mut self, bit_order: BitOrder) -> u32;
}

/// Generalizes how we save LPSPI data into memory.
trait ReceiveData {
    /// Invoked each time we read data from the queue.
    fn next_word(&mut self, bit_order: BitOrder, word: u32);
}

/// Transmit data from a buffer.
struct TransmitBuffer<'a, W> {
    /// The read position.
    ptr: *const W,
    /// At the end of the buffer.
    end: *const W,
    _buffer: PhantomData<&'a [W]>,
}

impl<W> TransmitBuffer<'_, W>
where
    W: Word,
{
    /// # Safety
    ///
    /// `ptr + len` must be in bounds, or at the end of the
    /// allocation.
    unsafe fn from_raw(ptr: *const W, len: usize) -> Self {
        Self {
            ptr,
            // Safety: caller upholds contract that ptr + len
            // must be in bounds, or at the end.
            end: unsafe { ptr.add(len) },
            _buffer: PhantomData,
        }
    }

    /// Read the next element from the buffer.
    fn next_read(&mut self) -> Option<W> {
        // Safety: read the next word only if we're in bounds.
        unsafe {
            (self.ptr != self.end).then(|| {
                let word = self.ptr.read();
                self.ptr = self.ptr.add(1);
                word
            })
        }
    }
}

impl<W> TransmitData for TransmitBuffer<'_, W>
where
    W: Word,
{
    fn next_word(&mut self, bit_order: BitOrder) -> u32 {
        W::pack_word(bit_order, || self.next_read())
    }
}

/// Transmits dummy values.
struct TransmitDummies;

impl TransmitData for TransmitDummies {
    fn next_word(&mut self, _: BitOrder) -> u32 {
        u32::MAX
    }
}

/// Receive data into a buffer.
struct ReceiveBuffer<'a, W> {
    /// The write position.
    ptr: *mut W,
    /// At the end of the buffer.
    end: *const W,
    _buffer: PhantomData<&'a [W]>,
}

impl<W> ReceiveBuffer<'_, W>
where
    W: Word,
{
    /// # Safety
    ///
    /// `ptr + len` must be in bounds, or at the end of the
    /// allocation.
    unsafe fn from_raw(ptr: *mut W, len: usize) -> Self {
        Self {
            ptr,
            // Safety: caller upholds contract that ptr + len
            // must be in bounds, or at the end.
            end: unsafe { ptr.cast_const().add(len) },
            _buffer: PhantomData,
        }
    }

    /// Put the next element into the buffer.
    fn next_write(&mut self, elem: W) {
        // Safety: write the next word only if we're in bounds.
        // Words are primitive types; we don't need to execute
        // a drop when we overwrite a value in memory.
        unsafe {
            if self.ptr.cast_const() != self.end {
                self.ptr.write(elem);
                self.ptr = self.ptr.add(1);
            }
        }
    }

    fn array_len(&self) -> usize {
        // Safety: end and ptr derive from the same allocation.
        // We always update ptr in multiples of it's object type.
        // The end pointer is always at a higher address in memory.
        unsafe { self.end.byte_offset_from(self.ptr) as _ }
    }
}

impl<W> ReceiveData for ReceiveBuffer<'_, W>
where
    W: Word,
{
    fn next_word(&mut self, bit_order: BitOrder, word: u32) {
        let valid_bytes = self.array_len().min(size_of_val(&word));
        W::unpack_word(word, bit_order, valid_bytes, |elem| self.next_write(elem));
    }
}

/// Receive dummy data.
struct ReceiveDummies;

impl ReceiveData for ReceiveDummies {
    fn next_word(&mut self, _: BitOrder, _: u32) {}
}

/// Computes how may Ws fit inside a LPSPI word.
const fn per_word<W: Word>() -> usize {
    core::mem::size_of::<u32>() / core::mem::size_of::<W>()
}

/// Computes how many u32 words we need to transact this buffer.
const fn word_count<W: Word>(words: &[W]) -> usize {
    words.len().div_ceil(per_word::<W>())
}

/// A SPI device with hardware-managed chip select.
///
/// `B` is the [`Lpspi`] bus type. The helper types
/// [`ExclusiveDevice`] and [`RefCellDevice`] impose
/// a specific kind of `B`. Prefer those types.
///
/// `CS` is your chip select pad. If you don't care about
/// pin type states, you can elide this type using the
/// various `without_pin` constructors.
///
/// `D` is a delay type. If you don't need a delay, you can
/// use a dummy type that panics or does nothing.
pub struct Device<B, CS, D: eh1::delay::DelayNs> {
    bus: B,
    cs: CS,
    delay: D,
    pcs: Pcs,
}

impl<B, CS, D: eh1::delay::DelayNs> Device<B, CS, D> {
    /// Borrow the bus.
    pub fn bus(&self) -> &B {
        &self.bus
    }
    /// Exclusively borrow the bus.
    ///
    /// Be careful when changing bus settings, especially the
    /// chip select.
    pub fn bus_mut(&mut self) -> &mut B {
        &mut self.bus
    }

    /// Release the device components.
    ///
    /// The components' states are unspecified.
    pub fn release(self) -> (B, CS, D) {
        (self.bus, self.cs, self.delay)
    }

    fn with_pin(bus: B, mut cs: CS, delay: D, pcs: Pcs) -> Self
    where
        CS: lpspi::Pin,
    {
        lpspi::prepare(&mut cs);
        Self::new(bus, cs, delay, pcs)
    }

    fn new(bus: B, cs: CS, delay: D, pcs: Pcs) -> Self {
        Self {
            bus,
            cs,
            delay,
            pcs,
        }
    }
}

impl<B, CS, D: eh1::delay::DelayNs> eh1::spi::ErrorType for Device<B, CS, D> {
    type Error = LpspiError;
}

/// A device that owns the LPSPI bus.
///
/// Use this is you have no other SPI peripherals connected
/// to your bus.
pub type ExclusiveDevice<P, CS, D, const N: u8> = Device<Lpspi<P, N>, CS, D>;

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> ExclusiveDevice<P, CS, D, N> {
    /// Construct the device with its PCS0 hardware chip select.
    pub fn with_pcs0(mut bus: Lpspi<P, N>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs0>,
    {
        bus.set_chip_select(Pcs::Pcs0);
        Self::with_pin(bus, cs, delay, Pcs::Pcs0)
    }
    /// Construct the device with its PCS1 hardware chip select.
    pub fn with_pcs1(mut bus: Lpspi<P, N>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs1>,
    {
        bus.set_chip_select(Pcs::Pcs1);
        Self::with_pin(bus, cs, delay, Pcs::Pcs1)
    }
    /// Construct the device with its PCS2 hardware chip select.
    pub fn with_pcs2(mut bus: Lpspi<P, N>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs2>,
    {
        bus.set_chip_select(Pcs::Pcs2);
        Self::with_pin(bus, cs, delay, Pcs::Pcs2)
    }
    /// Construct the device with its PCS3 hardware chip select.
    pub fn with_pcs3(mut bus: Lpspi<P, N>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs3>,
    {
        bus.set_chip_select(Pcs::Pcs3);
        Self::with_pin(bus, cs, delay, Pcs::Pcs3)
    }
}

impl<P, D: eh1::delay::DelayNs, const N: u8> ExclusiveDevice<P, (), D, N> {
    /// Construct the device without a chip select type state.
    ///
    /// You're responsible for muxing the chip select pin to the hardware.
    pub fn without_pin(mut bus: Lpspi<P, N>, pcs: Pcs, delay: D) -> Self {
        bus.set_chip_select(pcs);
        Self::new(bus, (), delay, pcs)
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u8>
    for ExclusiveDevice<P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u16>
    for ExclusiveDevice<P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u16>]) -> Result<(), Self::Error> {
        self.bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u32>
    for ExclusiveDevice<P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u32>]) -> Result<(), Self::Error> {
        self.bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

/// A device that can share the LPSPI bus in a single context.
///
/// Use this if you can share all of your devices within a single
/// execution context.
pub type RefCellDevice<'a, P, CS, D, const N: u8> = Device<&'a RefCell<Lpspi<P, N>>, CS, D>;

impl<'a, P, CS, D: eh1::delay::DelayNs, const N: u8> RefCellDevice<'a, P, CS, D, N> {
    /// Construct the device with its PCS0 hardware chip select.
    pub fn with_pcs0(bus: &'a RefCell<Lpspi<P, N>>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs0>,
    {
        Self::with_pin(bus, cs, delay, Pcs::Pcs0)
    }
    /// Construct the device with its PCS1 hardware chip select.
    pub fn with_pcs1(bus: &'a RefCell<Lpspi<P, N>>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs1>,
    {
        Self::with_pin(bus, cs, delay, Pcs::Pcs1)
    }
    /// Construct the device with its PCS2 hardware chip select.
    pub fn with_pcs2(bus: &'a RefCell<Lpspi<P, N>>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs2>,
    {
        Self::with_pin(bus, cs, delay, Pcs::Pcs2)
    }
    /// Construct the device with its PCS3 hardware chip select.
    pub fn with_pcs3(bus: &'a RefCell<Lpspi<P, N>>, cs: CS, delay: D) -> Self
    where
        CS: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Pcs3>,
    {
        Self::with_pin(bus, cs, delay, Pcs::Pcs3)
    }
}

impl<'a, P, D: eh1::delay::DelayNs, const N: u8> RefCellDevice<'a, P, (), D, N> {
    /// Construct the device without a chip select type state.
    ///
    /// You're responsible for muxing the chip select pin to the hardware.
    pub fn without_pin(bus: &'a RefCell<Lpspi<P, N>>, pcs: Pcs, delay: D) -> Self {
        Self::new(bus, (), delay, pcs)
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u8>
    for RefCellDevice<'_, P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        let mut bus = self.bus.borrow_mut();
        bus.set_chip_select(self.pcs);
        bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u16>
    for RefCellDevice<'_, P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u16>]) -> Result<(), Self::Error> {
        let mut bus = self.bus.borrow_mut();
        bus.set_chip_select(self.pcs);
        bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

impl<P, CS, D: eh1::delay::DelayNs, const N: u8> eh1::spi::SpiDevice<u32>
    for RefCellDevice<'_, P, CS, D, N>
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u32>]) -> Result<(), Self::Error> {
        let mut bus = self.bus.borrow_mut();
        bus.set_chip_select(self.pcs);
        bus.transact(operations, |ns| self.delay.delay_ns(ns))
    }
}

/// Tests try to approximate the way we'll use TransmitBuffer and ReceiveBuffer
/// in firmware. Consider running these with miri to evaluate unsafe usages.
#[cfg(test)]
mod tests {
    use super::{Descriptor, Operation, ReceiveBuffer, TransmitBuffer, Word};

    impl<W> ReceiveBuffer<'_, W>
    where
        W: Word,
    {
        fn new(buffer: &mut [W]) -> Self {
            // Safety: pointer offset math meets expectations.
            unsafe { Self::from_raw(buffer.as_mut_ptr(), buffer.len()) }
        }
    }

    impl<W> TransmitBuffer<'_, W>
    where
        W: Word,
    {
        fn new(buffer: &[W]) -> Self {
            // Safety: pointer offset math meets expectations.
            unsafe { Self::from_raw(buffer.as_ptr(), buffer.len()) }
        }
    }

    /// Creates the transmit and receive buffer objects for an
    /// in-place transfer.
    fn transfer_in_place<W: Word>(
        buffer: &mut [W],
    ) -> (TransmitBuffer<'_, W>, ReceiveBuffer<'_, W>) {
        // Safety: pointer math meets expectation. This produces
        // a mutable and immutable pointer to the same mutable buffer.
        // Module inspection shows that these pointers never become
        // references. We maintain the lifetime across both objects,
        // so the buffer isn't dropped.
        unsafe {
            let len = buffer.len();
            let ptr = buffer.as_mut_ptr();
            (
                TransmitBuffer::from_raw(ptr, len),
                ReceiveBuffer::from_raw(ptr, len),
            )
        }
    }

    #[test]
    fn schedule_tranfers_in_place_interleaved_u32() {
        const FST: [u32; 7] = [3_u32, 4, 5, 6, 7, 8, 9];
        let mut fst = FST;

        const SND: [u32; 2] = [55_u32, 77];
        let mut snd = SND;

        const TRD: [u32; 9] = [87_u32, 88, 89, 90, 91, 92, 93, 94, 95];
        let mut trd = TRD;

        let mut operations = [
            Operation::TransferInPlace(&mut fst),
            Operation::TransferInPlace(&mut snd),
            Operation::TransferInPlace(&mut trd),
        ];

        let initials: [&[u32]; 3] = [&FST, &SND, &TRD];

        let (tx_sched, rx_sched) = super::schedule(&mut operations);
        let sched = tx_sched.zip(rx_sched);

        let update = |elem| elem + 1;
        for ((tx_desc, rx_desc), initial) in sched.zip(initials) {
            let mut tx = tx_desc.tx_buffer.unwrap();
            let mut rx = rx_desc.rx_buffer.unwrap();

            for elem in initial {
                assert_eq!(*elem, tx.next_read().unwrap());
                rx.next_write(update(*elem));
            }
        }

        assert_eq!(fst, FST.map(update));
        assert_eq!(snd, SND.map(update));
        assert_eq!(trd, TRD.map(update));
    }

    #[test]
    fn schedule_tranfers_scattered_u32() {
        const DATA: [u32; 7] = [3_u32, 4, 5, 6, 7, 8, 9];
        let mut incoming = [0; 13];
        let outgoing = DATA;

        let mut operations = [Operation::Transfer(&mut incoming, &outgoing)];

        let (tx_sched, rx_sched) = super::schedule(&mut operations);
        let sched = tx_sched.zip(rx_sched);

        let update = |elem| elem + 13;
        for (tx_desc, rx_desc) in sched {
            let mut tx = tx_desc.tx_buffer.unwrap();
            let mut rx = rx_desc.rx_buffer.unwrap();

            for elem in DATA {
                assert_eq!(elem, tx.next_read().unwrap());
                rx.next_write(update(elem));
            }

            for rest in 249..255 {
                rx.next_write(rest)
            }
        }

        assert_eq!(
            incoming,
            [16, 17, 18, 19, 20, 21, 22, 249, 250, 251, 252, 253, 254]
        );
    }

    #[test]
    fn schedule_writes_u32() {
        const DATA: [u32; 7] = [3_u32, 4, 5, 6, 7, 8, 9];
        let outgoing = DATA;

        let mut operations = [Operation::Write(&outgoing)];

        let (mut tx_sched, mut rx_sched) = super::schedule(&mut operations);

        fn assert_descriptor(desc: &mut Descriptor<u32>) {
            let mut tx_buffer = desc.tx_buffer.take().unwrap();
            for elem in DATA {
                assert_eq!(elem, tx_buffer.next_read().unwrap())
            }
            assert_eq!(desc.tx_words, 7);
            assert_eq!(desc.rx_words, 0);
            assert!(desc.rx_buffer.is_none());
        }

        let mut tx_desc = tx_sched.next().unwrap();
        assert_descriptor(&mut tx_desc);

        let mut rx_desc = rx_sched.next().unwrap();
        assert_descriptor(&mut rx_desc);

        assert!(tx_sched.next().is_none());
        assert!(rx_sched.next().is_none());
    }

    #[test]
    fn schedule_reads_u32() {
        let mut incoming = [0; 13];
        let mut operations = [Operation::Read(&mut incoming)];

        let (mut tx_sched, mut rx_sched) = super::schedule(&mut operations);

        fn assert_descriptor(desc: &mut Descriptor<u32>, fill: &[u32]) {
            let mut rx_buffer = desc.rx_buffer.take().unwrap();
            for &elem in fill {
                rx_buffer.next_write(elem);
            }
            assert_eq!(desc.rx_words, 13);
            assert_eq!(desc.tx_words, 0);
            assert!(desc.tx_buffer.is_none());
        }

        let mut tx_desc = tx_sched.next().unwrap();
        assert_descriptor(&mut tx_desc, &[22, 33, 44, 55, 66]);

        let mut rx_desc = rx_sched.next().unwrap();
        assert_descriptor(&mut rx_desc, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);

        assert!(tx_sched.next().is_none());
        assert!(rx_sched.next().is_none());

        assert_eq!(incoming, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn schedule_delay_ns() {
        let mut operations: [Operation<u32>; 1] = [Operation::DelayNs(314)];
        let (mut tx_sched, mut rx_sched) = super::schedule(&mut operations);

        let tx = tx_sched.next().unwrap();
        assert!(tx.rx_buffer.is_none());
        assert!(tx.tx_buffer.is_none());
        assert_eq!(tx.rx_words, 0);
        assert_eq!(tx.tx_words, 314);
        assert!(tx.is_delay_ns());
        assert_eq!(tx.delay_ns(), Some(314));

        let rx = rx_sched.next().unwrap();
        assert!(rx.rx_buffer.is_none());
        assert!(rx.tx_buffer.is_none());
        assert_eq!(rx.rx_words, 0);
        assert_eq!(rx.tx_words, 314);
        assert!(rx.is_delay_ns());
        assert_eq!(rx.delay_ns(), Some(314));

        assert!(tx_sched.next().is_none());
        assert!(rx_sched.next().is_none());
    }

    #[test]
    fn transfer_in_place_interleaved_read_write_u32() {
        const BUFFER: [u32; 9] = [42u32, 43, 44, 45, 46, 47, 48, 49, 50];
        let mut buffer = BUFFER;
        let (mut tx, mut rx) = transfer_in_place(&mut buffer);

        for elem in BUFFER {
            assert_eq!(elem, tx.next_read().unwrap());
            rx.next_write(elem + 1);
        }

        assert_eq!(buffer, [43, 44, 45, 46, 47, 48, 49, 50, 51]);
    }

    #[test]
    fn transfer_in_place_interleaved_write_read_u32() {
        const BUFFER: [u32; 9] = [42u32, 43, 44, 45, 46, 47, 48, 49, 50];
        let mut buffer = BUFFER;
        let (mut tx, mut rx) = transfer_in_place(&mut buffer);

        for elem in BUFFER {
            rx.next_write(elem + 1);
            assert_eq!(elem + 1, tx.next_read().unwrap());
        }

        assert_eq!(buffer, [43, 44, 45, 46, 47, 48, 49, 50, 51]);
    }

    #[test]
    fn transfer_in_place_bulk_read_write_u32() {
        const BUFFER: [u32; 9] = [42u32, 43, 44, 45, 46, 47, 48, 49, 50];
        let mut buffer = BUFFER;
        let (mut tx, mut rx) = transfer_in_place(&mut buffer);

        for elem in BUFFER {
            assert_eq!(elem, tx.next_read().unwrap());
        }
        for elem in BUFFER {
            rx.next_write(elem + 1);
        }

        assert_eq!(buffer, [43, 44, 45, 46, 47, 48, 49, 50, 51]);
    }

    #[test]
    fn transfer_in_place_bulk_write_read_u32() {
        const BUFFER: [u32; 9] = [42u32, 43, 44, 45, 46, 47, 48, 49, 50];
        let mut buffer = BUFFER;
        let (mut tx, mut rx) = transfer_in_place(&mut buffer);

        for elem in BUFFER {
            rx.next_write(elem + 1);
        }
        for elem in BUFFER {
            assert_eq!(elem + 1, tx.next_read().unwrap());
        }

        assert_eq!(buffer, [43, 44, 45, 46, 47, 48, 49, 50, 51]);
    }

    #[test]
    fn transmit_buffer() {
        use super::{BitOrder::*, TransmitBuffer, TransmitData};

        //
        // u32
        //
        // This is the easiest to understand w.r.t. the bit order, since this is the natural word
        // size of the peripheral. No matter the bit order, we produce the same word for the TX
        // FIFO. The hardware handles the MSB or LSB transform.

        let mut tx = TransmitBuffer::new(&[0xDEADBEEFu32, 0xAD1CAC1D]);
        assert_eq!(tx.next_word(Msb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Msb), 0xAD1CAC1D);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADBEEFu32, 0xAD1CAC1D]);
        assert_eq!(tx.next_word(Lsb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Lsb), 0xAD1CAC1D);
        assert_eq!(tx.next_word(Lsb), 0);

        //
        // u8
        //
        // If the user prefers u8 words, then we should pack the bytes into a u32 such that the
        // hardware's MSB/LSB transform maintains the (literal) byte order.

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE, 0xEF, 0xA5, 0x00, 0x1D]);
        assert_eq!(tx.next_word(Msb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Msb), 0x00A5001D);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE, 0xEF, 0xA5, 0x00, 0x1D]);
        assert_eq!(tx.next_word(Lsb), 0xEFBEADDE);
        assert_eq!(tx.next_word(Lsb), 0x001D00A5);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE, 0xEF]);
        assert_eq!(tx.next_word(Msb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE, 0xEF]);
        assert_eq!(tx.next_word(Lsb), 0xEFBEADDE);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE]);
        assert_eq!(tx.next_word(Msb), 0x00DEADBE);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEu8, 0xAD, 0xBE]);
        assert_eq!(tx.next_word(Lsb), 0x00BEADDE);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);

        //
        // u16
        //
        // Same goes here: we should combine u16s such that the hardware transfers elements
        // in order while applying the MSB/LSB transform on each u16.

        let mut tx = TransmitBuffer::new(&[0xDEADu16, 0xBEEF, 0xA5A5]);
        assert_eq!(tx.next_word(Msb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Msb), 0x0000A5A5);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADu16, 0xBEEF, 0xA5A5]);
        assert_eq!(tx.next_word(Lsb), 0xBEEFDEAD);
        assert_eq!(tx.next_word(Lsb), 0x0000A5A5);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADu16, 0xBEEF]);
        assert_eq!(tx.next_word(Msb), 0xDEADBEEF);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADu16, 0xBEEF]);
        assert_eq!(tx.next_word(Lsb), 0xBEEFDEAD);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADu16]);
        assert_eq!(tx.next_word(Msb), 0x0000DEAD);
        assert_eq!(tx.next_word(Msb), 0);
        assert_eq!(tx.next_word(Msb), 0);

        let mut tx = TransmitBuffer::new(&[0xDEADu16]);
        assert_eq!(tx.next_word(Lsb), 0x0000DEAD);
        assert_eq!(tx.next_word(Lsb), 0);
        assert_eq!(tx.next_word(Lsb), 0);
    }

    #[test]
    fn receive_buffer() {
        // See notes in transmit_buffer test to understand MSB and LSB
        // transformations.

        use super::{BitOrder::*, ReceiveBuffer, ReceiveData};

        //
        // u8
        //

        let mut buffer = [0u8; 9];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Msb, 0xDEADBEEF);
        rx.next_word(Msb, 0xAD1CAC1D);
        rx.next_word(Msb, 0x04030201);
        rx.next_word(Msb, 0x55555555);
        assert_eq!(
            buffer,
            [0xDE, 0xAD, 0xBE, 0xEF, 0xAD, 0x1C, 0xAC, 0x1D, 0x01]
        );

        let mut buffer = [0u8; 9];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Lsb, 0xDEADBEEF);
        rx.next_word(Lsb, 0xAD1CAC1D);
        rx.next_word(Lsb, 0x04030201);
        rx.next_word(Lsb, 0x55555555);
        assert_eq!(
            buffer,
            [0xEF, 0xBE, 0xAD, 0xDE, 0x1D, 0xAC, 0x1C, 0xAD, 0x01]
        );

        //
        // u16
        //

        let mut buffer = [0u16; 5];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Msb, 0xDEADBEEF);
        rx.next_word(Msb, 0xAD1CAC1D);
        rx.next_word(Msb, 0x04030201);
        rx.next_word(Msb, 0x55555555);
        assert_eq!(buffer, [0xDEAD, 0xBEEF, 0xAD1C, 0xAC1D, 0x0201]);

        let mut buffer = [0u16; 5];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Lsb, 0xDEADBEEF);
        rx.next_word(Lsb, 0xAD1CAC1D);
        rx.next_word(Lsb, 0x04030201);
        rx.next_word(Lsb, 0x55555555);
        assert_eq!(buffer, [0xBEEF, 0xDEAD, 0xAC1D, 0xAD1C, 0x0201]);

        //
        // u32
        //

        let mut buffer = [0u32; 3];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Msb, 0xDEADBEEF);
        rx.next_word(Msb, 0xAD1CAC1D);
        rx.next_word(Msb, 0x77777777);
        rx.next_word(Msb, 0x55555555);
        assert_eq!(buffer, [0xDEADBEEF, 0xAD1CAC1D, 0x77777777]);

        let mut buffer = [0u32; 3];
        let mut rx = ReceiveBuffer::new(&mut buffer);
        rx.next_word(Lsb, 0xDEADBEEF);
        rx.next_word(Lsb, 0xAD1CAC1D);
        rx.next_word(Lsb, 0x77777777);
        rx.next_word(Lsb, 0x55555555);
        assert_eq!(buffer, [0xDEADBEEF, 0xAD1CAC1D, 0x77777777]);
    }

    #[test]
    fn transaction_frame_sizes() {
        assert!(super::Transaction::new_words(&[1u8]).is_ok());
        assert!(super::Transaction::new_words(&[1u8, 2]).is_ok());
        assert!(super::Transaction::new_words(&[1u8, 2, 3]).is_ok());
        assert!(super::Transaction::new_words(&[1u8, 2, 3, 4]).is_ok());
        assert!(super::Transaction::new_words(&[1u8, 2, 3, 4, 5]).is_ok());

        assert!(super::Transaction::new_words(&[1u16]).is_ok());
        assert!(super::Transaction::new_words(&[1u16, 2]).is_ok());
        assert!(super::Transaction::new_words(&[1u16, 2, 3]).is_ok());
        assert!(super::Transaction::new_words(&[1u16, 2, 3, 4]).is_ok());
        assert!(super::Transaction::new_words(&[1u16, 2, 3, 4, 5]).is_ok());

        assert!(super::Transaction::new_words(&[1u32]).is_ok());
        assert!(super::Transaction::new_words(&[1u32, 2]).is_ok());
        assert!(super::Transaction::new_words(&[1u32, 2, 3]).is_ok());
        assert!(super::Transaction::new_words(&[1u32, 2, 3, 4]).is_ok());
        assert!(super::Transaction::new_words(&[1u32, 2, 3, 4, 5]).is_ok());

        assert!(super::Transaction::new(7).is_err());
        assert!(super::Transaction::new(8).is_ok());
        assert!(super::Transaction::new(9).is_ok());
        assert!(super::Transaction::new(31).is_ok());
        assert!(super::Transaction::new(32).is_ok());
        assert!(super::Transaction::new(33).is_err());
        assert!(super::Transaction::new(34).is_ok());
        assert!(super::Transaction::new(95).is_ok());
        assert!(super::Transaction::new(96).is_ok());
        assert!(super::Transaction::new(97).is_err());
        assert!(super::Transaction::new(98).is_ok());
    }
}
