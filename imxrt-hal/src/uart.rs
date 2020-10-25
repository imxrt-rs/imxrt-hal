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
//! use imxrt_hal;
//! use embedded_hal::serial::{Read, Write};
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! let uarts = peripherals.uart.clock(
//!     &mut peripherals.ccm.handle,
//!     imxrt_hal::ccm::uart::ClockSelect::OSC,
//!     imxrt_hal::ccm::uart::PrescalarSelect::DIVIDE_1,
//! );
//!
//! let mut uart = uarts
//!     .uart2
//!     .init(
//!         peripherals.iomuxc.ad_b1.p02,
//!         peripherals.iomuxc.ad_b1.p03,
//!         115_200,
//!     )
//!     .unwrap();
//!
//! uart.set_tx_fifo(core::num::NonZeroU8::new(3));
//! uart.set_rx_fifo(true);
//! uart.set_parity(Some(imxrt_hal::uart::Parity::Even));
//! uart.set_rx_inversion(true);
//! uart.set_tx_inversion(false);
//!
//! uart.write(0xDE).unwrap();
//! let byte = uart.read().unwrap();
//!
//! // Split the peripheral into transfer and receive halves
//! let (tx, rx) = uart.split();
//! ```

use crate::ccm;
use crate::iomuxc::consts::{Unsigned, U1, U2, U3, U4, U5, U6, U7, U8};
use crate::iomuxc::uart;
use crate::ral;
use core::marker::PhantomData;

/// An uninitialized UART peripheral
///
/// Call `init()` to initialize the peripheral
pub struct Uninit<M: Unsigned> {
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
    reg: ral::lpuart::Instance,
}

impl<M: Unsigned> Uninit<M> {
    fn new(effective_clock: ccm::Frequency, reg: ral::lpuart::Instance) -> Self {
        Uninit {
            effective_clock,
            _module: PhantomData,
            reg,
        }
    }
}

/// All available UARTs
///
/// All UARTs are uninitialized. Call `init()` to take and initialize the
/// peripheral.
pub struct UARTs {
    pub uart1: Uninit<U1>,
    pub uart2: Uninit<U2>,
    pub uart3: Uninit<U3>,
    pub uart4: Uninit<U4>,
    pub uart5: Uninit<U5>,
    pub uart6: Uninit<U6>,
    pub uart7: Uninit<U7>,
    pub uart8: Uninit<U8>,
}

/// Unclocked UART peripherals
///
/// The `Unclocked` UART represents all UART peripherals
/// that do not have an activated clock. In order to obtain
/// any UART peripheral, the `Unclocked` type must be clocked.
#[allow(dead_code)] // Remove once all UARTs peripherals are implemented
pub struct Unclocked {
    pub(crate) uart1: ral::lpuart::Instance,
    pub(crate) uart2: ral::lpuart::Instance,
    pub(crate) uart3: ral::lpuart::Instance,
    pub(crate) uart4: ral::lpuart::Instance,
    pub(crate) uart5: ral::lpuart::Instance,
    pub(crate) uart6: ral::lpuart::Instance,
    pub(crate) uart7: ral::lpuart::Instance,
    pub(crate) uart8: ral::lpuart::Instance,
}
impl Unclocked {
    /// Enable all clocks for the UART peripherals. Returns a collection
    /// of UART peripherals.
    pub fn clock(
        self,
        ccm: &mut ccm::Handle,
        clock_select: ccm::uart::ClockSelect,
        prescalar: ccm::uart::PrescalarSelect,
    ) -> UARTs {
        let (ccm, _) = ccm.raw();

        //
        // See table 13-4 for clock gating registers
        //

        // -----------------------------------------
        // Disable clocks before modifying selection
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR5,
            CG12: 0,    // UART1
            CG13: 0     // UART7
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR0,
            CG14: 0,    // UART2
            CG6: 0      // UART3
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR1,
            CG12: 0     // UART4
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR3,
            CG1: 0,     // UART5
            CG3: 0      // UART6
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR6,
            CG7: 0      // UART8
        );
        // -----------------------------------------

        // -------------------------
        // Select clocks & prescalar
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CSCDR1,
            UART_CLK_SEL: (clock_select as u32),
            UART_CLK_PODF: (prescalar as u32)
        );
        // -------------------------

        // -------------
        // Enable clocks
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR5,
            CG12: 0b11,    // UART1
            CG13: 0b11     // UART7
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR0,
            CG14: 0b11,    // UART2
            CG6: 0b11      // UART3
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR1,
            CG12: 0b11     // UART4
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR3,
            CG1: 0b11,     // UART5
            CG3: 0b11      // UART6
        );
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR6,
            CG7: 0b11      // UART8
        );

        // -------------

        let effective_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(prescalar);
        UARTs {
            uart1: Uninit::new(effective_clock, self.uart1),
            uart2: Uninit::new(effective_clock, self.uart2),
            uart3: Uninit::new(effective_clock, self.uart3),
            uart4: Uninit::new(effective_clock, self.uart4),
            uart5: Uninit::new(effective_clock, self.uart5),
            uart6: Uninit::new(effective_clock, self.uart6),
            uart7: Uninit::new(effective_clock, self.uart7),
            uart8: Uninit::new(effective_clock, self.uart8),
        }
    }
}

trait ModuleExtension {
    unsafe fn steal() -> ral::lpuart::Instance;
}

impl<U> ModuleExtension for U
where
    U: Unsigned,
{
    unsafe fn steal() -> ral::lpuart::Instance {
        match U::USIZE {
            1 => ral::lpuart::LPUART1::steal(),
            2 => ral::lpuart::LPUART2::steal(),
            3 => ral::lpuart::LPUART3::steal(),
            4 => ral::lpuart::LPUART4::steal(),
            5 => ral::lpuart::LPUART5::steal(),
            6 => ral::lpuart::LPUART6::steal(),
            7 => ral::lpuart::LPUART7::steal(),
            8 => ral::lpuart::LPUART8::steal(),
            _ => unreachable!("there are only eight UART peripherals"),
        }
    }
}

impl<M> Uninit<M>
where
    M: Unsigned,
{
    /// Initializes a UART on the `tx` and `rx` pins. Specify the initial
    /// baud rate of the bus with `baud`. Returns the configured UART
    /// peripheral, or an error that indicates we could not configure the
    /// baud rate.
    pub fn init<TX, RX>(
        self,
        mut tx: TX,
        mut rx: RX,
        baud: u32,
    ) -> Result<UART<M>, ccm::uart::TimingsError>
    where
        TX: uart::Pin<Direction = uart::TX, Module = M>,
        RX: uart::Pin<Direction = uart::RX, Module = M>,
    {
        crate::iomuxc::uart::prepare(&mut tx);
        crate::iomuxc::uart::prepare(&mut rx);
        UART::start(self.reg, self.effective_clock, baud)
    }
}

/// An initialized UART peripheral
///
/// Call `read()` or `write()` to transmit bytes.
pub struct UART<M: Unsigned> {
    reg: ral::lpuart::Instance,
    effective_clock: ccm::Frequency,
    _module: PhantomData<M>,
}

/// A UART transfer half
///
/// `Tx` is capable of writing data, and nothing else. To configure
/// a transfer half, configure the [`UART`](struct.UART.html) peripheral
/// before calling [`split()`](struct.UART.html#method.split).
pub struct Tx<M: Unsigned>(UART<M>);

/// A UART receive half
///
/// `Rx` is capable of receiving data, and nothing else. To configure
/// a receive half, configure the [`UART`](struct.UART.html) peripheral
/// before calling [`split()`](struct.UART.html#method.split).
pub struct Rx<M: Unsigned>(UART<M>);

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

impl<M> UART<M>
where
    M: Unsigned,
{
    fn start(
        reg: ral::lpuart::Instance,
        effective_clock: ccm::Frequency,
        baud: u32,
    ) -> Result<Self, ccm::uart::TimingsError> {
        let mut uart = UART {
            reg,
            effective_clock,
            _module: PhantomData,
        };
        uart.set_baud(baud)?;
        ral::modify_reg!(ral::lpuart, uart.reg, CTRL, TE: TE_1, RE: RE_1);
        Ok(uart)
    }

    /// Split the UART peripheral into its transfer and receive half
    ///
    /// Ensure your UART peripheral is configured before calling
    /// `split()`.
    pub fn split(self) -> (Tx<M>, Rx<M>) {
        let rx_half = UART {
            reg: unsafe { M::steal() },
            effective_clock: self.effective_clock,
            _module: self._module,
        };
        (Tx(self), Rx(rx_half))
    }

    /// Re-combine the transfer and receive halves to create a full UART peripheral
    ///
    /// `join()` will let you re-configure a UART peripheral if theres a need to change
    /// settings.
    pub fn join(tx: Tx<M>, _rx: Rx<M>) -> Self {
        UART {
            reg: tx.0.reg,
            effective_clock: tx.0.effective_clock,
            _module: tx.0._module,
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

    /// Set the baud rate for the UART bus. Returns a `TimingsError` if there was
    /// an error computing the values that describe the baud rate.
    ///
    /// Calling this method temporarily disables the peripheral, flusing all data
    /// from *both* TX and RX FIFOs.
    pub fn set_baud(&mut self, baud: u32) -> Result<(), ccm::uart::TimingsError> {
        let timings = ccm::uart::timings(self.effective_clock, baud)?;
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

    /// Clear the UART status flags
    ///
    /// # Safety
    ///
    /// Performs writes behind an immutable receiver. Caller must ensure
    /// that the operation is atomic.
    unsafe fn clear_status(&self) {
        ral::modify_reg!(
            ral::lpuart,
            self.reg,
            STAT,
            IDLE: IDLE_1,
            OR: OR_1,
            NF: NF_1,
            FE: FE_1,
            PF: PF_1
        );
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

impl<M> serial::Write<u8> for UART<M>
where
    M: Unsigned,
{
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.flush()?;
        ral::write_reg!(ral::lpuart, self.reg, DATA, word as u32);
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if ral::read_reg!(ral::lpuart, self.reg, STAT, TDRE == TDRE_0) {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl<M> serial::Write<u8> for Tx<M>
where
    M: Unsigned,
{
    type Error = core::convert::Infallible;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.0.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.0.flush()
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

impl<M> serial::Read<u8> for UART<M>
where
    M: Unsigned,
{
    type Error = ReadError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        use ral::lpuart::DATA::*;
        let data = ral::read_reg!(ral::lpuart, self.reg, DATA);
        if data & RXEMPT::mask != 0 {
            Err(nb::Error::WouldBlock)
        } else {
            let mut flags = ReadErrorFlags::empty();
            flags.set(
                ReadErrorFlags::OVERRUN,
                ral::read_reg!(ral::lpuart, self.reg, STAT, OR == OR_1),
            );
            flags.set(ReadErrorFlags::PARITY, data & PARITYE::mask != 0);
            flags.set(ReadErrorFlags::FRAME_ERROR, data & FRETSC::mask != 0);
            flags.set(ReadErrorFlags::NOISY, data & NOISY::mask != 0);

            let raw = (data & 0xFF) as u8;
            // Safety: called with mutable receiver; caller is ensuring that this
            // entire read() operation occurs atomically.
            unsafe {
                self.clear_status();
            }

            if flags.is_empty() {
                Ok(raw)
            } else {
                Err(nb::Error::Other(ReadError { flags, raw }))
            }
        }
    }
}

impl<M> serial::Read<u8> for Rx<M>
where
    M: Unsigned,
{
    type Error = ReadError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.0.read()
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

unsafe impl<M> dma::peripheral::Source<u8> for UART<M>
where
    M: Unsigned,
{
    const SOURCE_REQUEST_SIGNAL: u32 = DMA_RX_REQUEST_LOOKUP[M::USIZE - 1];
    fn source(&self) -> *const u8 {
        &self.reg.DATA as *const _ as *const u8
    }
    fn enable_source(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: mutability is atomic
            self.clear_status();
            ral::modify_reg!(ral::lpuart, self.reg, BAUD, RDMAE: 1);
        });
    }
    fn disable_source(&self) {
        cortex_m::interrupt::free(|_| {
            while ral::read_reg!(ral::lpuart, self.reg, BAUD, RDMAE == 1) {
                ral::modify_reg!(ral::lpuart, self.reg, BAUD, RDMAE: 0);
            }
        });
    }
}

unsafe impl<M> dma::peripheral::Source<u8> for Rx<M>
where
    M: Unsigned,
{
    const SOURCE_REQUEST_SIGNAL: u32 = DMA_RX_REQUEST_LOOKUP[M::USIZE - 1];
    fn source(&self) -> *const u8 {
        self.0.source()
    }
    fn enable_source(&self) {
        self.0.enable_source()
    }
    fn disable_source(&self) {
        self.0.disable_source()
    }
}

unsafe impl<M> dma::peripheral::Destination<u8> for UART<M>
where
    M: Unsigned,
{
    const DESTINATION_REQUEST_SIGNAL: u32 = DMA_TX_REQUEST_LOOKUP[M::USIZE - 1];
    fn destination(&self) -> *const u8 {
        &self.reg.DATA as *const _ as *const u8
    }
    fn enable_destination(&self) {
        cortex_m::interrupt::free(|_| {
            ral::modify_reg!(ral::lpuart, self.reg, BAUD, TDMAE: 1);
        });
    }
    fn disable_destination(&self) {
        cortex_m::interrupt::free(|_| {
            while ral::read_reg!(ral::lpuart, self.reg, BAUD, TDMAE == 1) {
                ral::modify_reg!(ral::lpuart, self.reg, BAUD, TDMAE: 0);
            }
        });
    }
}

unsafe impl<M> dma::peripheral::Destination<u8> for Tx<M>
where
    M: Unsigned,
{
    const DESTINATION_REQUEST_SIGNAL: u32 = DMA_TX_REQUEST_LOOKUP[M::USIZE - 1];
    fn destination(&self) -> *const u8 {
        self.0.destination()
    }
    fn enable_destination(&self) {
        self.0.enable_destination()
    }
    fn disable_destination(&self) {
        self.0.disable_destination()
    }
}

use embedded_hal::blocking::serial::write::Default as BlockingWrite;

impl<M> BlockingWrite<u8> for UART<M> where M: Unsigned {}
impl<M> BlockingWrite<u8> for Tx<M> where M: Unsigned {}
