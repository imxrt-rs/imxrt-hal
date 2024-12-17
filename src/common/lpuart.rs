//! Low-power universal asynchronous receiver / transmitter.
//!
//! Use the LPUART peripheral to perform reads and writes with a serial
//! device. Features include
//!
//! - configurable baud rates (depends on input clock frequency)
//! - parity bits: none, even, odd
//! - inverted TX and RX lines
//! - TX and RX FIFOs with configurable watermarks
//! - DMA transfers and receives
//! - Non-blocking and blocking implementations of `embedded-hal` serial
//!   traits.
//!
//! # Example
//!
//! Demonstrates how to create and configure an LPUART peripheral. To see an example
//! of LPUART clock configuration, see the [`ccm::uart_clk`](crate::ccm::uart_clk) documentation.
//! For more information on the DMA API, see the [`dma`](crate::dma) examples.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use hal::lpuart::{Baud, Direction, Lpuart, Parity, Pins, Status, Watermark};
//! use imxrt_ral as ral;
//! # use imxrt_iomuxc::imxrt1060 as iomuxc;
//!
//! # async fn opt() -> Option<()> {
//! let (gpio_ad_b1_02, gpio_ad_b1_03) = // Handle to LPUART2 TX and RX pins...
//!     # unsafe { (iomuxc::gpio_ad_b1::GPIO_AD_B1_02::new(), iomuxc::gpio_ad_b1::GPIO_AD_B1_03::new()) };
//! # const UART_CLKC_HZ: u32 = 1;
//!
//! let registers = unsafe { ral::lpuart::LPUART2::instance() };
//! let pins = Pins { tx: gpio_ad_b1_02, rx: gpio_ad_b1_03 };
//! let mut lpuart2 = Lpuart::new(registers, pins);
//!
//! const BAUD: Baud = Baud::compute(UART_CLKC_HZ, 115200);
//! lpuart2.disable(|lpuart2| {
//!     lpuart2.set_baud(&BAUD);
//!     lpuart2.set_parity(Parity::ODD);
//!     lpuart2.enable_fifo(Watermark::tx(4));
//!     lpuart2.disable_fifo(Direction::Rx);
//!     lpuart2.set_inversion(Direction::Rx, true);
//! });
//!
//! // Fill the transmit FIFO with 0xAA...
//! while lpuart2.status().contains(Status::TRANSMIT_EMPTY) {
//!     lpuart2.write_byte(0xAA);
//! }
//!
//! // Schedule a DMA receive...
//! # let mut dma_channel = unsafe { hal::dma::DMA.channel(13) };
//! let mut buffer = [0u8; 64];
//! lpuart2.dma_read(&mut dma_channel, &mut buffer)
//!     .await.ok()?;
//!
//! // Release the peripheral instance...
//! let (lpuart2, pins) = lpuart2.release();
//!
//! // Reconstruct without the pins...
//! let mut lpuart2 = Lpuart::without_pins(lpuart2);
//! # Some(()) }
//! ```

use crate::iomuxc;
use crate::ral::{self, lpuart::Instance};

/// LPUART pins.
pub struct Pins<TX, RX>
where
    TX: iomuxc::lpuart::Pin<Direction = iomuxc::lpuart::Tx>,
    RX: iomuxc::lpuart::Pin<Module = TX::Module, Direction = iomuxc::lpuart::Rx>,
{
    /// Transfer pin.
    pub tx: TX,
    /// Receive pin.
    pub rx: RX,
}

/// LPUART peripheral.
///
/// `Lpuart` lets you configure the LPUART peripheral, and perform I/O.
/// See the [module-level documentation](crate::lpuart) for an example.
///
/// `Lpuart` implements serial traits from `embedded-hal`. It models
/// DMA transfers as futures. The type exposes a lower-level API for
/// coordinating DMA transfers. However, you may find it easier to use
/// the [`dma`](crate::dma) interface.
pub struct Lpuart<P, const N: u8> {
    pins: P,
    pub(crate) lpuart: Instance<N>,
}

/// Serial direction.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Transfer direction (leaving the peripheral).
    Tx,
    /// Receiver direction (entering the peripheral).
    Rx,
}

impl<TX, RX, const N: u8> Lpuart<Pins<TX, RX>, N>
where
    TX: iomuxc::lpuart::Pin<Module = iomuxc::consts::Const<N>, Direction = iomuxc::lpuart::Tx>,
    RX: iomuxc::lpuart::Pin<Module = iomuxc::consts::Const<N>, Direction = iomuxc::lpuart::Rx>,
{
    /// Create a new LPUART peripheral from its peripheral registers
    /// and TX / RX pins.
    ///
    /// When `new` returns, the peripheral is reset, the pins are
    /// configured for their LPUART functions, and the TX and RX
    /// halves are enabled.
    pub fn new(lpuart: Instance<N>, mut pins: Pins<TX, RX>) -> Self {
        iomuxc::lpuart::prepare(&mut pins.tx);
        iomuxc::lpuart::prepare(&mut pins.rx);
        Self::init(lpuart, pins)
    }
}

impl<const N: u8> Lpuart<(), N> {
    /// Create a new LPUART peripheral from its peripheral registers
    /// without any pins.
    ///
    /// This is similar to [`new()`](Self::new), but it does not configure
    /// pins to function as inputs and outputs. You're responsible
    /// for configuring TX and RX pins and for making sure the pin state
    /// doesn't change.
    pub fn without_pins(lpuart: Instance<N>) -> Self {
        Self::init(lpuart, ())
    }
}

impl<P, const N: u8> Lpuart<P, N> {
    /// The peripheral instance.
    pub const N: u8 = N;

    fn init(lpuart: Instance<N>, pins: P) -> Self {
        ral::write_reg!(ral::lpuart, lpuart, GLOBAL, RST: 1);
        ral::write_reg!(ral::lpuart, lpuart, GLOBAL, RST: 0);
        ral::modify_reg!(ral::lpuart, lpuart, CTRL, TE: TE_1, RE: RE_1);
        Self { pins, lpuart }
    }

    /// Indicates if the transmit / receive functions are
    /// (`true`) or are not (`false`) enabled.
    pub fn is_enabled(&self, direction: Direction) -> bool {
        match direction {
            Direction::Rx => ral::read_reg!(ral::lpuart, self.lpuart, CTRL, RE == RE_1),
            Direction::Tx => ral::read_reg!(ral::lpuart, self.lpuart, CTRL, TE == TE_1),
        }
    }

    /// Enable (`true`) or disable (`false`) the transmit / receive
    /// functions.
    pub fn set_enable(&mut self, direction: Direction, enable: bool) {
        match direction {
            Direction::Rx => ral::modify_reg!(ral::lpuart, self.lpuart, CTRL, RE: enable as u32),
            Direction::Tx => ral::modify_reg!(ral::lpuart, self.lpuart, CTRL, TE: enable as u32),
        }
    }

    /// Resets all internal logic and registers.
    ///
    /// Note that this may not reset all peripheral state, like the state
    /// in the peripheral's global register.
    pub fn reset(&mut self) {
        ral::write_reg!(ral::lpuart, self.lpuart, GLOBAL, RST: 1);
        ral::write_reg!(ral::lpuart, self.lpuart, GLOBAL, RST: 0);
    }

    /// Release all components of the LPUART driver.
    ///
    /// This does not change any component state; it releases the components as-is.
    /// If you need to obtain the registers in a known, good state, consider calling
    /// methods like [`reset()`](Self::reset) before releasing the registers.
    pub fn release(self) -> (Instance<N>, P) {
        (self.lpuart, self.pins)
    }

    /// Borrow the LPUART pins.
    pub fn pins(&self) -> &P {
        &self.pins
    }

    /// Exclusively borrow the LPUART pins.
    pub fn pins_mut(&mut self) -> &mut P {
        &mut self.pins
    }

    /// Temporarily disable the LPUART peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpuart::Disabled) driver lets you modify
    /// LPUART settings that require a fully disabled peripheral. This will flush
    /// TX and RX buffers.
    pub fn disable<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        let mut disabled = Disabled::new(&self.lpuart);
        func(&mut disabled)
    }

    /// Return the baud-specific timing values for this UART peripheral.
    pub fn baud(&self) -> Baud {
        let (osr, sbr, bothedge) =
            ral::read_reg!(ral::lpuart, self.lpuart, BAUD, OSR, SBR, BOTHEDGE);
        Baud {
            osr: osr + 1,
            sbr,
            bothedge: bothedge != 0,
        }
    }

    /// Return the parity seting for the UART peripheral.
    ///
    /// Result is `None` if there is no parity setting.
    pub fn parity(&self) -> Option<Parity> {
        let (pe, pt) = ral::read_reg!(ral::lpuart, self.lpuart, CTRL, PE, PT);
        const PARITY_ODD: u32 = Parity::Odd as u32;
        const PARITY_EVEN: u32 = Parity::Even as u32;
        match pt {
            PARITY_ODD if pe != 0 => Parity::ODD,
            PARITY_EVEN if pe != 0 => Parity::EVEN,
            _ => Parity::NONE,
        }
    }

    /// Indicates if the bits are inverted.
    #[inline]
    pub fn is_inverted(&self, direction: Direction) -> bool {
        match direction {
            Direction::Rx => ral::read_reg!(ral::lpuart, self.lpuart, STAT, RXINV == 1),
            Direction::Tx => ral::read_reg!(ral::lpuart, self.lpuart, CTRL, TXINV == 1),
        }
    }

    /// Indicates if the FIFO is enabled.
    #[inline]
    pub fn is_fifo_enabled(&self, direction: Direction) -> bool {
        match direction {
            Direction::Rx => ral::read_reg!(ral::lpuart, self.lpuart, FIFO, RXFE == 1),
            Direction::Tx => ral::read_reg!(ral::lpuart, self.lpuart, FIFO, TXFE == 1),
        }
    }

    /// Returns the FIFO watermark value.
    #[inline]
    pub fn fifo_watermark(&self, direction: Direction) -> u32 {
        match direction {
            Direction::Rx => ral::read_reg!(ral::lpuart, self.lpuart, WATER, RXWATER),
            Direction::Tx => ral::read_reg!(ral::lpuart, self.lpuart, WATER, TXWATER),
        }
    }

    /// Read the data register.
    pub fn read_data(&self) -> ReadData {
        ReadData(ral::read_reg!(ral::lpuart, self.lpuart, DATA))
    }

    /// Write a byte.
    ///
    /// This does not perform any checks for space in the transmit
    /// buffer. To check transmit buffer space, use `status`, and
    /// check for the transmit data register empty.
    pub fn write_byte(&self, byte: u8) {
        ral::write_reg!(ral::lpuart, self.lpuart, DATA, byte as u32);
    }

    /// Check the peripheral status register.
    pub fn status(&self) -> Status {
        let stat = ral::read_reg!(ral::lpuart, self.lpuart, STAT);
        let fifo = ral::read_reg!(ral::lpuart, self.lpuart, FIFO);
        Status::from_registers(stat, fifo)
    }

    /// Clear the status flags.
    ///
    /// Bits that are read-only will be cleared by the implementation, so it's
    /// safe to call with `Status::all()`.
    #[inline]
    pub fn clear_status(&mut self, status: Status) {
        let stat_flags = status & Status::W1C & Status::stat_mask();
        let fifo_flags = status & Status::W1C & Status::fifo_mask();
        ral::modify_reg!(ral::lpuart, self.lpuart, STAT, |stat| {
            let stat = stat & !Status::stat_mask().stat_bits();
            stat | stat_flags.stat_bits()
        });
        ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, |fifo| {
            let fifo = fifo & !Status::fifo_mask().fifo_bits();
            fifo | fifo_flags.fifo_bits()
        });
    }

    /// Flush data from the FIFO.
    ///
    /// This does not flush anything that's already in the transmit or receive register.
    #[inline]
    pub fn flush_fifo(&mut self, direction: Direction) {
        flush_fifo(&self.lpuart, direction);
    }

    /// Return the interrupt flags.
    ///
    /// The interrupt flags indicate the reasons that this peripheral may generate an
    /// interrupt.
    pub fn interrupts(&self) -> Interrupts {
        let ctrl = ral::read_reg!(ral::lpuart, self.lpuart, CTRL);
        let fifo = ral::read_reg!(ral::lpuart, self.lpuart, FIFO);
        Interrupts::from_bits_truncate(ctrl | fifo)
    }

    /// Let the peripheral act as a DMA destination.
    ///
    /// After this call, the peripheral will signal to the DMA engine whenever
    /// it has free space in its transfer buffer.
    pub fn enable_dma_transmit(&mut self) {
        ral::modify_reg!(ral::lpuart, self.lpuart, BAUD, TDMAE: 1);
    }

    /// Stop the peripheral from acting as a DMA destination.
    ///
    /// See the DMA chapter in the reference manual to understand when this
    /// should be called in the DMA transfer lifecycle.
    pub fn disable_dma_transmit(&mut self) {
        while ral::read_reg!(ral::lpuart, self.lpuart, BAUD, TDMAE == 1) {
            ral::modify_reg!(ral::lpuart, self.lpuart, BAUD, TDMAE: 0);
        }
    }

    /// Produces a pointer to the data register.
    ///
    /// You should use this pointer when coordinating a DMA transfer.
    /// You're not expected to read from this pointer in software.
    pub fn data(&self) -> *const ral::RWRegister<u32> {
        core::ptr::addr_of!(self.lpuart.DATA)
    }

    /// Let the peripheral act as a DMA source.
    ///
    /// After this call, the peripheral will signal to the DMA engine whenever
    /// it has data available to read.
    pub fn enable_dma_receive(&mut self) {
        self.clear_status(Status::W1C);
        ral::modify_reg!(ral::lpuart, self.lpuart, BAUD, RDMAE: 1);
    }

    /// Stop the peripheral from acting as a DMA source.
    ///
    /// See the DMA chapter in the reference manual to understand when this
    /// should be called in the DMA transfer lifecycle.
    pub fn disable_dma_receive(&mut self) {
        while ral::read_reg!(ral::lpuart, self.lpuart, BAUD, RDMAE == 1) {
            ral::modify_reg!(ral::lpuart, self.lpuart, BAUD, RDMAE: 0);
        }
    }

    /// Attempts to write a single byte to the bus.
    ///
    /// Returns `false` if the fifo was already full.
    pub fn try_write(&mut self, byte: u8) -> bool {
        ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, TXOF: TXOF_1);
        self.write_byte(byte);
        ral::read_reg!(ral::lpuart, self.lpuart, FIFO, TXOF == TXOF_0)
    }

    /// Attempts to read a single byte from the bus.
    ///
    /// Returns:
    ///   - `Ok(Some(u8))` if data was read
    ///   - `Ok(None)` if the fifo was empty
    ///   - `Err(..)` if a read error happened
    pub fn try_read(&mut self) -> Result<Option<u8>, ReadFlags> {
        let data = self.read_data();
        if data.flags().contains(ReadFlags::RXEMPT) {
            Ok(None)
        } else if data
            .flags()
            .intersects(ReadFlags::PARITY_ERROR | ReadFlags::FRAME_ERROR | ReadFlags::NOISY)
        {
            Err(data.flags())
        } else {
            Ok(Some(data.into()))
        }
    }
}

fn flush_fifo<const N: u8>(lpuart: &Instance<N>, direction: Direction) {
    match direction {
        Direction::Rx => ral::modify_reg!(ral::lpuart, lpuart, FIFO, RXFLUSH: RXFLUSH_1),
        Direction::Tx => ral::modify_reg!(ral::lpuart, lpuart, FIFO, TXFLUSH: TXFLUSH_1),
    }
}

/// A temporarily-disabled LPUART peripheral.
///
/// The disabled peripheral lets you changed
/// settings that require a disabled peripheral.
pub struct Disabled<'a, const N: u8> {
    lpuart: &'a Instance<N>,
    te: bool,
    re: bool,
}

impl<const N: u8> Drop for Disabled<'_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpuart, self.lpuart, CTRL, TE: self.te as u32, RE: self.re as u32);
    }
}

impl<'a, const N: u8> Disabled<'a, N> {
    fn new(lpuart: &'a Instance<N>) -> Self {
        let (te, re) = ral::read_reg!(ral::lpuart, lpuart, CTRL, TE, RE);
        ral::modify_reg!(ral::lpuart, lpuart, CTRL, TE: TE_0, RE: RE_0);
        for direction in [Direction::Rx, Direction::Tx] {
            flush_fifo(lpuart, direction);
        }
        Self {
            lpuart,
            te: te != 0,
            re: re != 0,
        }
    }

    /// Set baud-specific timing values for this UART peripheral.
    ///
    /// The timing values are used to set a baud rate. To compute
    /// a baud rate, see [`Baud::compute`](crate::lpuart::Baud::compute). Or,
    /// you may compute your own timing values.
    pub fn set_baud(&mut self, baud: &Baud) {
        ral::modify_reg!(ral::lpuart, self.lpuart,
            BAUD,
            OSR: baud.osr.clamp(4, 32) - 1,
            SBR: baud.sbr.min((1 << 13) - 1),
            BOTHEDGE: baud.bothedge as u32)
    }

    /// Specify parity bit settings. If there is no parity, use `None`.
    pub fn set_parity(&mut self, parity: Option<Parity>) {
        ral::modify_reg!(
            ral::lpuart,
            self.lpuart,
            CTRL,
            PE: parity.is_some() as u32,
            M: parity.is_some() as u32,
            PT: parity.map(|p| p as u32).unwrap_or(0u32)
        );
    }

    /// Reverse the polarity of data, affecting all data bits, start
    /// and stop bits, and polarity bits.
    ///
    /// The default inversion state is `false`.
    #[inline]
    pub fn set_inversion(&mut self, direction: Direction, inverted: bool) {
        match direction {
            Direction::Rx => {
                ral::modify_reg!(ral::lpuart, self.lpuart, STAT, RXINV: inverted as u32)
            }
            Direction::Tx => {
                ral::modify_reg!(ral::lpuart, self.lpuart, CTRL, TXINV: inverted as u32)
            }
        }
    }

    /// Disable the FIFO for the given direction.
    #[inline]
    pub fn disable_fifo(&mut self, direction: Direction) {
        match direction {
            Direction::Rx => ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, RXFE: RXFE_0),
            Direction::Tx => ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, TXFE: TXFE_0),
        }
    }

    /// Enable the FIFO, and set the FIFO watermark.
    ///
    /// `watermark` describes the serial direction, and the point at which the hardware signals a full
    /// or empty FIFO. Use [`Watermark::tx`](crate::lpuart::Watermark::tx)
    /// to enable the transfer FIFO, and [`Watermark::rx`](crate::lpuart::Watermark::rx) to enable the
    /// receive FIFO.
    ///
    /// The actual watermark value is limited by the hardware. `enable_fifo` returns the
    /// actual watermark value.
    #[inline]
    pub fn enable_fifo(&mut self, watermark: Watermark) -> u32 {
        let size = match watermark.direction {
            Direction::Rx => 1 << ral::read_reg!(ral::lpuart, self.lpuart, PARAM, RXFIFO),
            Direction::Tx => 1 << ral::read_reg!(ral::lpuart, self.lpuart, PARAM, TXFIFO),
        };
        let size = watermark.size.min(size - 1);
        match watermark.direction {
            Direction::Rx => {
                ral::modify_reg!(ral::lpuart, self.lpuart, WATER, RXWATER: size);
                ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, RXFE: RXFE_1);
            }
            Direction::Tx => {
                ral::modify_reg!(ral::lpuart, self.lpuart, WATER, TXWATER: size);
                ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, TXFE: TXFE_1);
            }
        };
        size
    }

    /// Set the interrupt flags for this LPUART peripheral.
    ///
    /// Use `set_interrupts` to enable or disable interrupt generation for
    /// this peripheral.
    pub fn set_interrupts(&mut self, interrupts: Interrupts) {
        let ctrl_flags = interrupts & Interrupts::ctrl_mask();
        let fifo_flags = interrupts & Interrupts::fifo_mask();
        ral::modify_reg!(ral::lpuart, self.lpuart, CTRL, |ctrl| {
            let ctrl = ctrl & !Interrupts::ctrl_mask().bits();
            ctrl | ctrl_flags.bits()
        });
        ral::modify_reg!(ral::lpuart, self.lpuart, FIFO, |fifo| {
            let fifo = fifo & !Interrupts::fifo_mask().bits();
            fifo | fifo_flags.bits()
        });
    }
}

/// Values specific to the baud rate.
///
/// To compute the values for a given baud rate,
/// use [`compute`](Baud::compute). To understand
/// the actual baud rate, use [`value`](Baud::value).
///
/// Advanced users may choose to set the OSR, SBR, and
/// BOTHEDGE values directly.
///
/// ```no_run
/// use imxrt_hal::lpuart::Baud;
///
/// // Assume UART clock is driven from the crystal
/// // oscillator...
/// const UART_CLOCK_HZ: u32 = 24_000_000;
/// const BAUD: Baud = Baud::compute(UART_CLOCK_HZ, 115200);
/// ```
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Baud {
    /// Oversampling rate.
    ///
    /// This should be set between 4 and 32.
    /// The driver clamps the `osr` value within
    /// this range.
    pub osr: u32,
    /// Baud rate modulo divisor.
    ///
    /// The driver commits this value directly.
    /// A value of zero is allowed, but will disable
    /// baud rate generation in hardware. The max
    /// value is `(2^13) - 1`. The implementation
    /// limits the max value.
    pub sbr: u32,
    /// Both edge sampling.
    ///
    /// Should be set when the oversampling
    /// rate is between 4 and 7. Optional
    /// for higher sampling rates. The driver
    /// will commit this value directly.
    pub bothedge: bool,
}

impl Baud {
    /// Returns the baud value in bits per second.
    ///
    /// `source_clock_hz` is the UART clock frequency (Hz).
    ///
    /// # Panics
    ///
    /// Panics if `sbr` or `osr` is zero.
    pub const fn value(self, source_clock_hz: u32) -> u32 {
        source_clock_hz / (self.sbr * self.osr)
    }

    /// Computes a timings struct that represents a baud rate.
    ///
    /// `source_clock_hz` is the UART clock frequency (Hz). `baud`
    /// is the intended baud rate.
    pub const fn compute(source_clock_hz: u32, baud: u32) -> Baud {
        const fn max(left: u32, right: u32) -> u32 {
            if left > right {
                left
            } else {
                right
            }
        }
        const fn min(left: u32, right: u32) -> u32 {
            if left < right {
                left
            } else {
                right
            }
        }

        let mut err = u32::MAX;
        let mut best_osr = 0;
        let mut best_sbr = 0;

        let mut osr = if baud > 3_000_000 { 4 } else { 8 };
        while osr <= 32 {
            let mut sbr = 1;
            while sbr < 8192 {
                let b = source_clock_hz / (sbr * osr);
                let e = max(baud, b) - min(baud, b);
                if e < err {
                    err = e;
                    best_osr = osr;
                    best_sbr = sbr;
                }
                sbr += 1;
            }
            osr += 1;
        }
        Baud {
            osr: best_osr,
            sbr: best_sbr,
            bothedge: 4 <= best_osr && best_osr <= 7,
        }
    }
}

/// Parity bit selection.
///
/// See [`Disabled::set_parity`](crate::lpuart::Disabled::set_parity) and
/// [`Lpuart::parity`](crate::lpuart::Lpuart::parity) for more information.
/// Consider using the associated constants to quickly specify
/// parity bits.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Parity {
    /// Even parity.
    Even = 0,
    /// Odd parity.
    Odd = 1,
}

impl Parity {
    /// No parity.
    pub const NONE: Option<Parity> = None;
    /// Even parity.
    pub const EVEN: Option<Parity> = Some(Parity::Even);
    /// Odd parity.
    pub const ODD: Option<Parity> = Some(Parity::Odd);
}

bitflags::bitflags! {
    /// Errors that may occur when reading data.
    pub struct ReadFlags : u32 {
        /// Data was received with noise.
        const NOISY = 1 << 15;
        /// Parity error when receiving data.
        const PARITY_ERROR = 1 << 14;
        /// Framing error when receiving data.
        const FRAME_ERROR = 1 << 13;
        /// Receive buffer is empty.
        ///
        /// Asserts when there is no data in the receive buffer.
        const RXEMPT = 1 << 12;
        /// Idle Line.
        ///
        /// Indicates the receiver line was idle before receiving the character.
        /// Overrun occured, and we lost data in the shift register.
        const IDLINE = 1 << 11;
    }
}

bitflags::bitflags! {
    /// Interrupt settings.
    ///
    /// A set bit indicates that the interrupt is enabled.
    pub struct Interrupts : u32 {
        /// Overrun interrupt enable.
        const OVERRUN = 1 << 27;
        /// Noise error interrupt enable.
        const NOISE_ERROR = 1 << 26;
        /// Framing error interrupt enable.
        const FRAMING_ERROR = 1 << 25;
        /// Parity error interrupt enable.
        const PARITY_ERROR = 1 << 24;
        /// Transmit empty interrupt enable.
        ///
        /// Triggers when the `TRANSMIT_EMPTY` _status_ bit is high.
        const TRANSMIT_EMPTY = 1 << 23;
        /// Transmit complete interrupt enable.
        ///
        /// Triggers an interrupt when the `TRANSMIT_COMPLETE` _status_ bit is high.
        const TRANSMIT_COMPLETE = 1 << 22;
        /// Receiver interrupt enable.
        ///
        /// Triggers when the `RECEIVE_FULL` _status_ bit is high.
        const RECEIVE_FULL = 1 << 21;

        // All of the above flags pertain to the CTRL
        // register. These flags pertain to the FIFO
        // interrupts. They can be written directly.

        /// Transmit FIFO Overflow Interrupt Enable.
        ///
        /// If set, a transmit FIFO overrun event generates an
        /// interrupt.
        const TRANSMIT_OVERFLOW = 1 << 9;
        /// Receive FIFO Underflow Interrupt Enable.
        ///
        /// If set, a receive FIFO underflow event generates an
        /// interrupt.
        const RECEIVE_UNDERFLOW = 1 << 8;
    }
}

impl Interrupts {
    /// Mask for only the FIFO bits.
    const fn fifo_mask() -> Self {
        Self::from_bits_truncate(
            Interrupts::TRANSMIT_OVERFLOW.bits() | Interrupts::RECEIVE_UNDERFLOW.bits(),
        )
    }
    /// Mask for only the CTRL bits.
    const fn ctrl_mask() -> Self {
        // Safety: bits are valid for this bitflags instance.
        Self::from_bits_truncate(Self::all().bits() & !Self::fifo_mask().bits())
    }
}

/// The result of reading from the receiver.
///
/// The data contains flags, which may indicate errors
/// in the received data. If the flags indicate value data,
/// use `u8::from` to convert the data into its raw byte.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ReadData(u32);

impl ReadData {
    /// Access the read flags, which indicate results of the
    /// read operation.
    #[inline]
    pub fn flags(self) -> ReadFlags {
        ReadFlags::from_bits_truncate(self.0)
    }

    /// Access the raw value.
    #[inline]
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl From<ReadData> for u8 {
    #[inline]
    fn from(read_data: ReadData) -> u8 {
        read_data.0 as u8
    }
}

bitflags::bitflags! {
    /// Status flags.
    pub struct Status : u32 {
        /// Receiver active flag.
        ///
        /// Set when the receiver detects a start bit. Cleared when
        /// the line is idle.
        const RECEIVE_ACTIVE = 1 << 24;
        /// Transmit data register empty.
        ///
        /// This bit is set when the transmit FIFO can accept data.
        /// - if the FIFO is enabled, this is set when the FIFO hits the watermark.
        /// - if the FIFO is disabled, this is set when there's nothing in
        ///   the transmit data register.
        const TRANSMIT_EMPTY = 1 << 23;
        /// Transmit complete.
        ///
        /// TC is cleared when there's a transmission in progress, or when a preamble /
        /// break character is loaded. It's set when the transmit buffer is empty.
        ///
        /// To clear TC, perform a write.
        const TRANSMIT_COMPLETE = 1 << 22;
        /// Receiver data register full.
        ///
        /// This bit is set when the receive FIFO is full.
        /// - if the FIFO is enabled, this is set when the FIFO hits the watermark.
        /// - if the FIFO is disabled, this is set when there's something in the
        ///   receiver register.
        const RECEIVE_FULL = 1 << 21;
        /// Idle line flag.
        ///
        /// IDLE is set when the LPUART receive line becomes idle for a full character
        /// time after a period of activity.
        const IDLE = 1 << 20;
        /// Receiver overrun.
        ///
        /// Set when software fails to prevent the receive data register
        /// from overflowing with data. The OR bit is set immediately after the
        /// stop bit has been completely received for the dataword that overflows
        /// the buffer and all the other error flags (FE, NF, and PF) are prevented
        /// from setting.
        const OVERRUN = 1 << 19;
        /// Noise flag.
        ///
        /// This is also available in the read flags. However, setting it here
        /// allows you to clear the noise flag in the status register.
        const NOISY = 1 << 18;
        /// Framing error.
        ///
        /// This is also available in the read flags. However, setting it here
        /// allows you to clear the noise flag in the status register.
        const FRAME_ERROR = 1 << 17;
        /// Parity error.
        ///
        /// This is also available in the read flags. However, setting it here
        /// allows you to clear the noise flag in the status register.
        const PARITY_ERROR = 1 << 16;

        // All flags up to and including bit 13 are marked 'reserved'
        // in the status register. We're using these for other 'status'
        // functions.

        // These two flags relate to the FIFOs. They can only be written
        // to the FIFO register after a left shift of FIFO_SHIFT.

        /// Transmitter Buffer Overflow Flag
        ///
        /// Indicates that more data has been written to the transmit buffer than it can hold.
        const TRANSMIT_OVERFLOW = 1 << 13;
        /// Receiver Buffer Underflow Flag
        ///
        /// Indicates that more data has been read from the receive buffer than was present.
        const RECEIVE_UNDERFLOW = 1 << 12;
    }
}

impl Status {
    /// The number of left shifts required to move the FIFO
    /// status bits into position for the FIFO register.
    const FIFO_SHIFT: u32 = 4;

    /// The set of status bits that are W1C.
    ///
    /// Use this to differentiate read-only bits from bits that are
    /// W1C.
    pub const W1C: Status =
        Self::from_bits_truncate(Self::all().bits() & !Self::read_only_mask().bits());

    /// Status bits that are read-only.
    ///
    /// Includes those bits in the FIFO register.
    const fn read_only_mask() -> Self {
        Self::from_bits_truncate(
            Self::RECEIVE_ACTIVE.bits()
                | Self::TRANSMIT_EMPTY.bits()
                | Self::TRANSMIT_COMPLETE.bits()
                | Self::RECEIVE_FULL.bits(),
        )
    }

    /// Return the bitflags that represent the FIFO bits.
    const fn fifo_mask() -> Self {
        Self::from_bits_truncate(Self::TRANSMIT_OVERFLOW.bits() | Self::RECEIVE_UNDERFLOW.bits())
    }
    /// Return the bitflags that represent the STAT bits.
    const fn stat_mask() -> Self {
        Self::from_bits_truncate(Self::all().bits() & !Self::fifo_mask().bits())
    }
    /// Returns the FIFO bits that may be written to the FIFO register.
    const fn fifo_bits(self) -> u32 {
        (self.bits & Self::fifo_mask().bits()) << Self::FIFO_SHIFT
    }
    /// Returns the STAT bits that may be writeen to the STAT register.
    const fn stat_bits(self) -> u32 {
        self.bits & Self::stat_mask().bits()
    }
    /// Compose status bitflags from raw STAT and FIFO register values.
    ///
    /// FIFO should only include `TXOF` and / or `RXUF` bits.
    const fn from_registers(stat: u32, fifo: u32) -> Self {
        Self::from_bits_truncate(stat | (fifo >> Self::FIFO_SHIFT))
    }
}

/// Watermark levels for TX and RX FIFOs.
///
/// See [`Lpuart::enable_fifo`](crate::lpuart::Disabled::enable_fifo) for more
/// information.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy)]
pub struct Watermark {
    direction: Direction,
    size: u32,
}

impl Watermark {
    /// Specify the transmit FIFO watermark.
    ///
    /// Note that the actual watermark value will be limited by the hardware.
    #[inline]
    pub const fn tx(size: u32) -> Self {
        Watermark {
            direction: Direction::Tx,
            size,
        }
    }
    /// Specify the receive FIFO watermark.
    ///
    /// Note that the actual watermark value with be limited by the hardware.
    #[inline]
    pub const fn rx(size: core::num::NonZeroU32) -> Self {
        Watermark {
            direction: Direction::Rx,
            size: size.get(),
        }
    }
}

impl<P, const N: u8> eh02::serial::Write<u8> for Lpuart<P, N> {
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.flush()?;
        self.write_byte(word);
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if !self.status().contains(Status::TRANSMIT_EMPTY) {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl<P, const N: u8> eh02::serial::Read<u8> for Lpuart<P, N> {
    type Error = ReadFlags;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let data = self.read_data();
        self.clear_status(Status::W1C);
        if data.flags().contains(ReadFlags::RXEMPT) {
            Err(nb::Error::WouldBlock)
        } else if data
            .flags()
            .intersects(ReadFlags::PARITY_ERROR | ReadFlags::FRAME_ERROR | ReadFlags::NOISY)
        {
            Err(nb::Error::Other(data.flags()))
        } else {
            Ok(data.into())
        }
    }
}

impl<P, const N: u8> eh02::blocking::serial::Write<u8> for Lpuart<P, N> {
    type Error = core::convert::Infallible;

    fn bwrite_all(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        for word in buffer {
            nb::block!(eh02::serial::Write::write(self, *word))?;
        }

        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        nb::block!(eh02::serial::Write::flush(self))?;
        Ok(())
    }
}

impl eio06::Error for ReadFlags {
    fn kind(&self) -> eio06::ErrorKind {
        eio06::ErrorKind::Other
    }
}

impl<P, const N: u8> eio06::ErrorType for Lpuart<P, N> {
    type Error = ReadFlags;
}

impl<P, const N: u8> eio06::WriteReady for Lpuart<P, N> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.status().contains(Status::TRANSMIT_EMPTY))
    }
}

impl<P, const N: u8> eio06::ReadReady for Lpuart<P, N> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.status().contains(Status::RECEIVE_FULL))
    }
}

impl<P, const N: u8> eio06::Write for Lpuart<P, N> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut num_written = 0;
        for word in buf {
            if num_written == 0 {
                // For the first word, continue trying until we send.
                // This function is supposed to block until at least one word is
                // sent.
                while !self.try_write(*word) {}
            } else {
                // If we already sent at least one word, return once
                // the buffer is full
                if !self.try_write(*word) {
                    break;
                }
            }
            num_written += 1;
        }

        Ok(num_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.status().contains(Status::TRANSMIT_COMPLETE) {}

        Ok(())
    }
}

impl<P, const N: u8> eio06::Read for Lpuart<P, N> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut num_read = 0;
        for word in buf {
            let data = if num_read == 0 {
                // For the first word, continue querying until we receive something.
                // This function is supposed to block until at least one word is
                // received.
                loop {
                    if let Some(data) = self.try_read()? {
                        break data;
                    }
                }
            } else {
                // If we already read at least one word, return once
                // the buffer is empty
                if let Some(data) = self.try_read()? {
                    data
                } else {
                    break;
                }
            };

            *word = data;
            num_read += 1;
        }

        Ok(num_read)
    }
}

#[cfg(test)]
mod tests {
    use super::{Baud, ReadData, ReadFlags, Status};

    #[test]
    fn approximate_baud() {
        // Assume the 24MHz XTAL clock.
        const UART_CLOCK_HZ: u32 = 24_000_000;
        // The best baud rate we can get is
        const EXPECTED_BAUD: u32 = 115384;
        // for a target baud of
        const TARGET_BAUD: u32 = 115200;

        const BAUD: Baud = Baud::compute(UART_CLOCK_HZ, TARGET_BAUD);

        assert_eq!(BAUD.value(UART_CLOCK_HZ), EXPECTED_BAUD);

        // These values could switch, depending on the implementation...
        assert!(BAUD.sbr == 8 || BAUD.sbr == 26, "SBR: {}", BAUD.sbr);
        if BAUD.sbr == 8 {
            assert_eq!(BAUD.osr, 26);
        } else {
            assert_eq!(BAUD.osr, 8);
        }
        assert!(!BAUD.bothedge);
    }

    #[test]
    fn non_default_sbr_baud() {
        // Assume the 24MHz XTAL clock.
        const UART_CLOCK_HZ: u32 = 24_000_000;
        // The best baud rate we can get is
        const EXPECTED_BAUD: u32 = 9600;
        // for a target baud of
        const TARGET_BAUD: u32 = 9600;

        const BAUD: Baud = Baud::compute(UART_CLOCK_HZ, TARGET_BAUD);

        assert_eq!(BAUD.value(UART_CLOCK_HZ), EXPECTED_BAUD);

        assert_eq!(BAUD.osr, 10, "OSR: {}", BAUD.osr);
        assert_eq!(BAUD.sbr, 250, "SBR: {}", BAUD.sbr);
        assert!(!BAUD.bothedge);
    }

    #[test]
    fn max_baud() {
        // Assume the 24MHz XTAL clock.
        const UART_CLOCK_HZ: u32 = 24_000_000;
        // The best baud rate we can get is
        const EXPECTED_BAUD: u32 = 6_000_000;
        // for a target baud of
        const TARGET_BAUD: u32 = 6_000_000;

        const BAUD: Baud = Baud::compute(UART_CLOCK_HZ, TARGET_BAUD);

        assert_eq!(BAUD.value(UART_CLOCK_HZ), EXPECTED_BAUD);

        assert_eq!(BAUD.osr, 4, "OSR: {}", BAUD.osr);
        assert_eq!(BAUD.sbr, 1, "SBR: {}", BAUD.sbr);
        assert!(BAUD.bothedge);
    }

    #[test]
    fn read_data_flags() {
        let read_data = ReadData(1 << 15 | 1 << 13);
        let flags = read_data.flags();

        assert!(flags.contains(ReadFlags::NOISY));
        assert!(!flags.contains(ReadFlags::PARITY_ERROR));
        assert!(flags.contains(ReadFlags::FRAME_ERROR));
        assert!(!flags.contains(ReadFlags::RXEMPT));
        assert!(!flags.contains(ReadFlags::IDLINE));

        assert!(flags.intersects(ReadFlags::NOISY | ReadFlags::PARITY_ERROR));
        assert!(!flags.intersects(ReadFlags::RXEMPT | ReadFlags::PARITY_ERROR));
    }

    #[test]
    fn status_flags() {
        assert_eq!(Status::fifo_mask().bits(), (1 << 13) | (1 << 12));
        assert_eq!(Status::fifo_mask().fifo_bits(), (1 << 17) | (1 << 16));
        assert_eq!(Status::stat_mask().bits(), 0x01FF_0000);
        assert_eq!(Status::stat_mask().stat_bits(), 0x01FF_0000);
        assert_eq!(Status::W1C.bits(), 0x001F_3000);

        assert!(Status::from_registers(0, (1 << 17) | (1 << 16))
            .contains(Status::TRANSMIT_OVERFLOW | Status::RECEIVE_UNDERFLOW));
        assert!(Status::from_registers(u32::MAX, 0).contains(Status::stat_mask()));

        assert!(Status::all().contains(Status::TRANSMIT_EMPTY));
    }
}
