//! Low Power Universal Asynchronous Receiver/Transmit (LPUART)
//!
//! The UART module provides a serial peripheral that implements
//! the `embedded_hal::serial` traits. The peripheral is sufficient
//! for implementing basic serial communications.
//!
//! UARTs may also be used in bi-directional DMA transfers.
//!
//! # Example
//!
//! ```no_run
//! use imxrt1060_hal as hal;
//! use hal::{ccm::{self, ClockGate}, iomuxc};
//! use hal::ral::{
//!     ccm::CCM, lpuart::LPUART2,
//!     iomuxc::IOMUXC,
//! };
//! use embedded_hal::serial::{Read, Write};
//!
//! let pads = IOMUXC::take().map(iomuxc::new).unwrap();
//!
//! let mut ccm = CCM::take().map(ccm::CCM::from_ral).unwrap();
//! let mut uart2 = LPUART2::take().unwrap();
//!
//! let mut uart_clock = ccm.uart_clock.enable(&mut ccm.handle);
//! uart_clock.set_clock_gate(&mut uart2, ClockGate::On);
//!
//! let mut uart = hal::uart::UART::new(
//!     uart2,
//!     pads.ad_b1.p02, // TX
//!     pads.ad_b1.p03, // RX
//!     &uart_clock,
//! );
//!
//! uart.set_baud(9600).unwrap();
//!
//! uart.set_tx_fifo(core::num::NonZeroU8::new(3));
//! uart.set_rx_fifo(true);
//! uart.set_parity(Some(hal::uart::Parity::Even));
//! uart.set_rx_inversion(true);
//! uart.set_tx_inversion(false);
//!
//! uart.write(0xDE).unwrap();
//! let byte = uart.read().unwrap();
//!
//! // Split the peripheral into transfer and receive halves
//! let (mut tx, mut rx) = uart.split();
//! tx.write(0xAD).unwrap();
//! let byte = rx.read().unwrap();
//!
//! // Combine the two back together
//! let uart = hal::uart::UART::join(tx, rx);
//! ```

use crate::iomuxc::consts::Unsigned;
use crate::iomuxc::uart;
use crate::ral::{self, lpuart::RegisterBlock};
use core::marker::PhantomData;

/// An initialized UART peripheral
///
/// Call `read()` or `write()` to transmit bytes.
pub struct UART<M: Unsigned, TX, RX> {
    reg: ral::lpuart::Instance<M>,
    effective_clock: u32,
    _pins: (TX, RX),
}

/// A UART transfer half
///
/// `Tx` is capable of writing data, and nothing else. To configure
/// a transfer half, configure the [`UART`](struct.UART.html) peripheral
/// before calling [`split()`](struct.UART.html#method.split).
pub struct Tx<M: Unsigned, P> {
    reg: ral::lpuart::Instance<M>,
    effective_clock: u32,
    _tx: P,
}

/// A UART receive half
///
/// `Rx` is capable of receiving data, and nothing else. To configure
/// a receive half, configure the [`UART`](struct.UART.html) peripheral
/// before calling [`split()`](struct.UART.html#method.split).
pub struct Rx<M: Unsigned, P> {
    reg: *const RegisterBlock,
    _module: PhantomData<M>,
    _rx: P,
}

/// Parity selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// Even parity (the 'E' in 8E1, for example)
    Even,
    /// Odd parity (the 'O' in 8O1, for example)
    Odd,
}

impl Parity {
    fn bit(self) -> bool {
        self == Parity::Odd
    }
}

impl<M, TX, RX> UART<M, TX, RX>
where
    M: Unsigned,
{
    /// Create a new `UART` from a UART instance, TX and RX pins
    ///
    /// The baud rate of the returned `UART` is unspecified. Make sure you use [`set_baud`](UART::set_baud())
    /// to properly configure the driver.
    pub fn new(
        instance: ral::lpuart::Instance<M>,
        mut tx: TX,
        mut rx: RX,
        clock: &crate::ccm::UARTClock,
    ) -> UART<M, TX, RX>
    where
        TX: uart::Pin<Direction = uart::TX, Module = M>,
        RX: uart::Pin<Direction = uart::RX, Module = M>,
    {
        crate::iomuxc::uart::prepare(&mut tx);
        crate::iomuxc::uart::prepare(&mut rx);
        let uart = UART {
            reg: instance,
            effective_clock: clock.frequency(),
            _pins: (tx, rx),
        };
        ral::modify_reg!(ral::lpuart, uart.reg, CTRL, TE: TE_1, RE: RE_1);
        uart
    }
}

impl<M, TX, RX> UART<M, TX, RX>
where
    M: Unsigned,
{
    const DMA_SOURCE_REQUEST_SIGNAL: u32 = DMA_RX_REQUEST_LOOKUP[M::USIZE - 1];
    const DMA_DESTINATION_REQUEST_SIGNAL: u32 = DMA_TX_REQUEST_LOOKUP[M::USIZE - 1];

    /// Split the UART peripheral into its transfer and receive half
    ///
    /// Ensure your UART peripheral is configured before calling
    /// `split()`.
    pub fn split(self) -> (Tx<M, TX>, Rx<M, RX>) {
        let (tx_pin, rx_pin) = self._pins;
        let rx = Rx {
            reg: &*self.reg,
            _rx: rx_pin,
            _module: PhantomData,
        };
        let tx = Tx {
            reg: self.reg,
            _tx: tx_pin,
            effective_clock: self.effective_clock,
        };
        (tx, rx)
    }

    /// Re-combine the transfer and receive halves to create a full UART peripheral
    ///
    /// `join()` will let you re-configure a UART peripheral if theres a need to change
    /// settings.
    pub fn join(tx: Tx<M, TX>, rx: Rx<M, RX>) -> Self {
        UART {
            reg: tx.reg,
            effective_clock: tx.effective_clock,
            _pins: (tx._tx, rx._rx),
        }
    }

    /// Specify parity bit settings. If there is no parity, use `None`.
    ///
    /// Calling this method will temporarily disable the peripheral,
    /// flusing all data from all FIFOs.
    pub fn set_parity(&mut self, parity: Option<Parity>) {
        self.while_disabled(|this| {
            ral::modify_reg!(
                ral::lpuart,
                this.reg,
                CTRL,
                PE: u32::from(parity.is_some()),
                M: u32::from(parity.is_some()),
                PT: u32::from(parity.map(|p| p.bit()).unwrap_or(false))
            );
        });
    }

    /// Reverse the polarity of received data, affecting all data bits, start
    /// and stop bits, and polarity bits.
    ///
    /// The default inversion state is `false`. Note that calling this method
    /// will temporarily disable the peripheral, flusing all data from all FIFOs.
    pub fn set_rx_inversion(&mut self, inverted: bool) {
        self.while_disabled(|this| {
            ral::modify_reg!(ral::lpuart, this.reg, STAT, RXINV: u32::from(inverted));
        });
    }

    /// Reverse the polarity of transferred data, affecting all data bits,
    /// start and stop bits, and polarity bits.
    ///
    /// The default inversion state is `false`. Note that calling this method
    /// will temporarily disable the peripheral, flusing all data from all FIFOs.
    pub fn set_tx_inversion(&mut self, inverted: bool) {
        self.while_disabled(|this| {
            ral::modify_reg!(ral::lpuart, this.reg, CTRL, TXINV: u32::from(inverted));
        });
    }

    /// Controls the TX FIFO.
    ///
    /// If size is `Some(n)`, where `n > 0`, the method will enable the TX
    /// FIFO with a size `n`. The method returns size of the FIFO that was
    /// set, which is based on the hardware. On an iMXRT1062, the max size
    /// is 4.
    ///
    /// If size is `None`, the method disables the TX FIFO. The return is 0.
    ///
    /// The method temporarily disables the UART bus, flushing any data in
    /// the *both* TX and RX FIFOs.
    pub fn set_tx_fifo(&mut self, size: Option<core::num::NonZeroU8>) -> u8 {
        self.while_disabled(|this| {
            if let Some(requested_size) = size {
                // Maximum TX FIFO size supported by this device
                let max_size = 1 << ral::read_reg!(ral::lpuart, this.reg, PARAM, TXFIFO);
                let tx_fifo_size = max_size.min(requested_size.get());
                // Safety: max size is one less than PARAM[TXFIFO].
                // Assume an iMXRT1062. PARAM[TXFIFO] = 4, so
                // WATER[TXWATER] = 3. 3 == 0b11, which fits into
                // the two bit range of the field. We're assuming
                // that this scales for chips that might have a larger
                // PARAM[TXFIFO] size.
                ral::modify_reg!(
                    ral::lpuart,
                    this.reg,
                    WATER,
                    TXWATER: (tx_fifo_size.saturating_sub(1) as u32)
                );
                ral::modify_reg!(ral::lpuart, this.reg, FIFO, TXFE: TXFE_1);
                tx_fifo_size
            } else {
                ral::modify_reg!(ral::lpuart, this.reg, WATER, TXWATER: 0);
                ral::modify_reg!(ral::lpuart, this.reg, FIFO, TXFE: TXFE_0);
                0
            }
        })
    }

    /// Enable or disable the RX FIFO. The maximum size of the FIFO is based on
    /// the underlying hardware. An iMXRT1062's RX FIFO is 4 bytes.
    ///
    /// Calling this method temporarily disables the peripheral, flusing all data
    /// from *both* TX and RX FIFOs.
    pub fn set_rx_fifo(&mut self, enable: bool) {
        self.while_disabled(|this| {
            ral::modify_reg!(ral::lpuart, this.reg, FIFO, RXFE: u32::from(enable));
        })
    }

    fn while_disabled<F: FnMut(&mut Self) -> R, R>(&mut self, mut act: F) -> R {
        ral::modify_reg!(
            ral::lpuart,
            self.reg,
            FIFO,
            TXFLUSH: TXFLUSH_1,
            RXFLUSH: RXFLUSH_1
        );
        let (te, re) = ral::read_reg!(ral::lpuart, self.reg, CTRL, TE, RE);
        ral::modify_reg!(ral::lpuart, self.reg, CTRL, TE: TE_0, RE: RE_0);
        let res = act(self);
        ral::modify_reg!(ral::lpuart, self.reg, CTRL, TE: te, RE: re);
        res
    }

    /// Set the serial baud rate
    ///
    /// If there is an error, the error is [`Error::Clock`](Error::Clock).
    pub fn set_baud(&mut self, baud: u32) -> Result<(), Error> {
        let timings = timings(self.effective_clock, baud)?;
        self.while_disabled(|this| {
            ral::modify_reg!(
                ral::lpuart,
                this.reg,
                BAUD,
                OSR: u32::from(timings.osr),
                SBR: u32::from(timings.sbr),
                BOTHEDGE: u32::from(timings.both_edge)
            );
        });
        Ok(())
    }

    /// Enable the receiver interrupt associated with this UART
    ///
    /// The interrupt will trigger when there are at least `watermark` number of
    /// bytes in the RX FIFO. Returns the maximum-allowable watermark level that
    /// was set in hardware. A watermark of `Some(0)` means that we should interrupt
    /// as soon as a byte is read.
    ///
    /// If the watermark is greater than 0, ensure that you call `set_rx_fifo` before this
    /// method. Otherwise, the return will be 0 despite the supplied watermark.
    ///
    /// Disable receiver interrupt by setting `watermark` to `None`. The return is always 0
    /// when disabling the receiver interrupt.
    pub fn set_receiver_interrupt(&mut self, watermark: Option<u8>) -> u8 {
        self.while_disabled(|this| {
            if let Some(watermark) = watermark {
                let rx_fifo_size = if ral::read_reg!(ral::lpuart, this.reg, FIFO, RXFE == RXFE_1)
                    && watermark > 0
                {
                    // Use the FIFO watermark to define interrupt frequency.
                    let max_size = 1 << ral::read_reg!(ral::lpuart, this.reg, PARAM, RXFIFO);
                    let fifo_size = max_size.min(watermark);
                    // Safety: see justification in set_tx_fifo
                    ral::modify_reg!(ral::lpuart, this.reg, WATER, RXWATER: fifo_size as u32);
                    fifo_size
                } else {
                    // User has not enable the RX FIFO, or the watermark is zero.
                    0
                };
                ral::modify_reg!(ral::lpuart, this.reg, CTRL, RIE: RIE_1);
                rx_fifo_size
            } else {
                ral::modify_reg!(ral::lpuart, this.reg, WATER, RXWATER: 0);
                ral::modify_reg!(ral::lpuart, this.reg, CTRL, RIE: RIE_0);
                0
            }
        })
    }
}

use embedded_hal::serial;

impl<M, TX, RX> serial::Write<u8> for UART<M, TX, RX>
where
    M: Unsigned,
{
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        unsafe { write(&*self.reg, word) }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        flush(&*self.reg)
    }
}

/// Clear the UART status flags
///
/// # Safety
///
/// Performs writes behind an immutable receiver. Caller must ensure
/// that the operation is atomic.
#[inline(always)]
unsafe fn clear_status(reg: &RegisterBlock) {
    ral::modify_reg!(
        ral::lpuart,
        reg,
        STAT,
        IDLE: IDLE_1,
        OR: OR_1,
        NF: NF_1,
        FE: FE_1,
        PF: PF_1
    );
}

#[inline(always)]
unsafe fn read(reg: &RegisterBlock) -> nb::Result<u8, ReadError> {
    use ral::lpuart::DATA::*;
    let data = ral::read_reg!(ral::lpuart, reg, DATA);
    if data & RXEMPT::mask != 0 {
        Err(nb::Error::WouldBlock)
    } else {
        let mut flags = ReadErrorFlags::empty();
        flags.set(
            ReadErrorFlags::OVERRUN,
            ral::read_reg!(ral::lpuart, reg, STAT, OR == OR_1),
        );
        flags.set(ReadErrorFlags::PARITY, data & PARITYE::mask != 0);
        flags.set(ReadErrorFlags::FRAME_ERROR, data & FRETSC::mask != 0);
        flags.set(ReadErrorFlags::NOISY, data & NOISY::mask != 0);

        let raw = (data & 0xFF) as u8;
        clear_status(reg);

        if flags.is_empty() {
            Ok(raw)
        } else {
            Err(nb::Error::Other(ReadError { flags, raw }))
        }
    }
}

#[inline(always)]
unsafe fn write(reg: &RegisterBlock, word: u8) -> nb::Result<(), core::convert::Infallible> {
    flush(reg)?;
    ral::write_reg!(ral::lpuart, reg, DATA, word as u32);
    Ok(())
}

#[inline(always)]
fn flush(reg: &RegisterBlock) -> nb::Result<(), core::convert::Infallible> {
    if ral::read_reg!(ral::lpuart, reg, STAT, TDRE == TDRE_0) {
        Err(nb::Error::WouldBlock)
    } else {
        Ok(())
    }
}

impl<M, P> serial::Write<u8> for Tx<M, P>
where
    M: Unsigned,
{
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        unsafe { write(&*self.reg, word) }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        flush(&*self.reg)
    }
}

bitflags::bitflags! {
    /// Errors that may occur when reading data
    pub struct ReadErrorFlags : u8 {
        /// Data was received with noise
        const NOISY = 1 << 7;
        /// Parity error when receiving data
        const PARITY = 1 << 6;
        /// Framing error when receiving data
        const FRAME_ERROR = 1 << 5;
        /// Overrun occured, and we lost data in the shift register
        const OVERRUN = 1 << 4;
    }
}

/// Type that describes a read error
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadError {
    /// Decribes the reason for the error
    pub flags: ReadErrorFlags,
    /// The raw value read, if you'd like to consider it
    pub raw: u8,
}

impl<M, TX, RX> serial::Read<u8> for UART<M, TX, RX>
where
    M: Unsigned,
{
    type Error = ReadError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        unsafe { read(&*self.reg) }
    }
}

impl<M, P> serial::Read<u8> for Rx<M, P>
where
    M: Unsigned,
{
    type Error = ReadError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        unsafe { read(&*self.reg) }
    }
}

use crate::dma;

/// UART TX DMA Request signal
///
/// See table 4-3 of the iMXRT1060 Reference Manual (Rev 2)
const DMA_TX_REQUEST_LOOKUP: [u32; 8] = [2, 66, 4, 68, 6, 70, 8, 72];

/// UART RX DMA Request signal
///
/// See table 4-3 of the iMXRT1060 Reference Manual (Rev 2)
const DMA_RX_REQUEST_LOOKUP: [u32; 8] = [3, 67, 5, 69, 7, 71, 9, 73];

#[inline(always)]
fn dma_source(reg: &RegisterBlock) -> *const u8 {
    &reg.DATA as *const _ as *const u8
}

#[inline(always)]
fn dma_enable_source(reg: &RegisterBlock) {
    cortex_m::interrupt::free(|_| unsafe {
        // Safety: mutability is atomic
        clear_status(reg);
        ral::modify_reg!(ral::lpuart, reg, BAUD, RDMAE: 1);
    });
}

#[inline(always)]
fn dma_disable_source(reg: &RegisterBlock) {
    cortex_m::interrupt::free(|_| {
        while ral::read_reg!(ral::lpuart, reg, BAUD, RDMAE == 1) {
            ral::modify_reg!(ral::lpuart, reg, BAUD, RDMAE: 0);
        }
    });
}

unsafe impl<M, TX, RX> dma::peripheral::Source<u8> for UART<M, TX, RX>
where
    M: Unsigned,
{
    fn source_signal(&self) -> u32 {
        Self::DMA_SOURCE_REQUEST_SIGNAL
    }
    fn source(&self) -> *const u8 {
        dma_source(&*self.reg)
    }
    fn enable_source(&self) {
        dma_enable_source(&*self.reg);
    }
    fn disable_source(&self) {
        dma_disable_source(&*self.reg);
    }
}

unsafe impl<M, P> dma::peripheral::Source<u8> for Rx<M, P>
where
    M: Unsigned,
{
    fn source_signal(&self) -> u32 {
        UART::<M, P, P>::DMA_SOURCE_REQUEST_SIGNAL
    }
    fn source(&self) -> *const u8 {
        // Safety: deref is OK; memory valid
        unsafe { dma_source(&*self.reg) }
    }
    fn enable_source(&self) {
        // Safety: deref is OK; memory valid
        unsafe { dma_enable_source(&*self.reg) };
    }
    fn disable_source(&self) {
        // Safety: deref is OK; memory valid
        unsafe { dma_disable_source(&*self.reg) };
    }
}

#[inline(always)]
fn dma_destination(reg: &RegisterBlock) -> *const u8 {
    &reg.DATA as *const _ as *const u8
}

#[inline(always)]
fn dma_enable_destination(reg: &RegisterBlock) {
    cortex_m::interrupt::free(|_| {
        ral::modify_reg!(ral::lpuart, reg, BAUD, TDMAE: 1);
    });
}

#[inline(always)]
fn dma_disable_destination(reg: &RegisterBlock) {
    cortex_m::interrupt::free(|_| {
        while ral::read_reg!(ral::lpuart, reg, BAUD, TDMAE == 1) {
            ral::modify_reg!(ral::lpuart, reg, BAUD, TDMAE: 0);
        }
    });
}

unsafe impl<M, TX, RX> dma::peripheral::Destination<u8> for UART<M, TX, RX>
where
    M: Unsigned,
{
    fn destination_signal(&self) -> u32 {
        Self::DMA_DESTINATION_REQUEST_SIGNAL
    }
    fn destination(&self) -> *const u8 {
        dma_destination(&*self.reg)
    }
    fn enable_destination(&self) {
        dma_enable_destination(&*self.reg);
    }
    fn disable_destination(&self) {
        dma_disable_destination(&*self.reg);
    }
}

unsafe impl<M, P> dma::peripheral::Destination<u8> for Tx<M, P>
where
    M: Unsigned,
{
    fn destination_signal(&self) -> u32 {
        UART::<M, P, P>::DMA_DESTINATION_REQUEST_SIGNAL
    }
    fn destination(&self) -> *const u8 {
        dma_destination(&*self.reg)
    }
    fn enable_destination(&self) {
        dma_enable_destination(&*self.reg);
    }
    fn disable_destination(&self) {
        dma_disable_destination(&*self.reg);
    }
}

use embedded_hal::blocking::serial::write::Default as BlockingWrite;

impl<M, TX, RX> BlockingWrite<u8> for UART<M, TX, RX> where M: Unsigned {}
impl<M, P> BlockingWrite<u8> for Tx<M, P> where M: Unsigned {}

/// An opaque type that describes timing configurations
struct Timings {
    /// OSR register value. Accounts for the -1. May be written
    /// directly to the register
    osr: u8,
    /// True if we need to set BOTHEDGE given the OSR value
    both_edge: bool,
    /// SBR value;
    sbr: u16,
}

/// Errors propagated from a [`UART`] device
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// There was an error when preparing the baud rate or clocks
    Clock,
}

/// Compute timings for a UART peripheral. Returns the timings,
/// or a string describing an error.
fn timings(effective_clock: u32, baud: u32) -> Result<Timings, Error> {
    //        effective_clock
    // baud = ---------------
    //         (OSR+1)(SBR)
    //
    // Solve for SBR:
    //
    //       effective_clock
    // SBR = ---------------
    //        (OSR+1)(baud)
    //
    // After selecting SBR, calculate effective baud.
    // Minimize the error over all OSRs.

    let base_clock: u32 = effective_clock.checked_div(baud).ok_or(Error::Clock)?;
    let mut error = u32::max_value();
    let mut best_osr = 16;
    let mut best_sbr = 1;

    for osr in 4..=32 {
        let sbr = base_clock.checked_div(osr).ok_or(Error::Clock)?;
        let sbr = sbr.max(1).min(8191);
        let effective_baud = effective_clock.checked_div(osr * sbr).ok_or(Error::Clock)?;
        let err = effective_baud.max(baud) - effective_baud.min(baud);
        if err < error {
            best_osr = osr;
            best_sbr = sbr;
            error = err
        }
    }

    use core::convert::TryFrom;
    Ok(Timings {
        osr: u8::try_from(best_osr - 1).map_err(|_| Error::Clock)?,
        sbr: u16::try_from(best_sbr).map_err(|_| Error::Clock)?,
        both_edge: best_osr < 8,
    })
}
