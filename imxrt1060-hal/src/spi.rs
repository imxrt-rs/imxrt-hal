//! SPI support
//!
//! The module provides an implementation of the `embedded_hal::spi::FullDuplex` trait.
//! All blocking implementations are provided by the default implementations from
//! `embedded_hal`.
//!
//! # Chip selects (CS) for SPI peripherals
//!
//! The iMXRT SPI peripherals have one or more peripheral-controlled chip selects (CS). Using
//! the peripheral-controlled CS means that you do not need a GPIO to coordinate SPI operations.
//! The peripheral-controlled CS is disabled by default. Use the `enable_chip_select_N`, where
//! `N` is the CS number, to enable the peripheral-controlled CS. Your hardware must be wired to
//! accomodate this selection. If you do not want to use the peripheral-controlled CS, you may
//! select your own GPIO.
//!
//! # Example
//!
//! ```no_run
//! use imxrt1060_hal as hal;
//! use embedded_hal::blocking::spi::Transfer;
//! use hal::{ccm::{self, ClockGate}, iomuxc, spi::{SPI, Pins}};
//! use hal::ral::{
//!     ccm::CCM, iomuxc::IOMUXC, lpspi::LPSPI4,
//! };
//!
//! let pads = IOMUXC::take().map(iomuxc::new).unwrap();
//!
//! let mut ccm = CCM::take().map(ccm::CCM::from_ral).unwrap();
//!
//! let mut spi_clock = ccm.spi_clock.enable(&mut ccm.handle);
//! let spi_pins = Pins {
//!     sdo: pads.b0.p02,
//!     sdi: pads.b0.p01,
//!     sck: pads.b0.p03,
//!     pcs0: pads.b0.p00,
//! };
//! let mut spi4 = LPSPI4::take().unwrap();
//! spi_clock.set_clock_gate(&mut spi4, ClockGate::On);
//! let mut spi = SPI::new(
//!     spi4,
//!     spi_pins,
//!     &spi_clock,
//! );
//!
//! spi.set_clock_speed(hal::spi::ClockSpeed(1_000_000)).unwrap();
//!
//! let mut buffer: [u8; 3] = [1, 2, 3];
//! spi.transfer(&mut buffer).unwrap();
//! ```

use crate::iomuxc::{consts::Unsigned, spi};
use crate::ral;

/// SPI Clock speed, in Hz
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ClockSpeed(pub u32);

impl Default for ClockSpeed {
    fn default() -> Self {
        ClockSpeed(8_000_000)
    }
}

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// # Safety
    ///
    /// The function touches SPI registers that should only be touched
    /// while the SPI master is disabled.
    unsafe fn set(self, source_clock: u32, reg: &ral::lpspi::RegisterBlock) {
        log::debug!(
            "SPI baud rate = {:?}, source clock = {:?}",
            self,
            source_clock
        );

        let mut div = source_clock / self.0;

        if source_clock / div > self.0 {
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
}

/// An SPI master
///
/// By default, the SPI master runs at 8Mhz, Use `set_clock_speed` to vary
/// the SPI bus speed.
pub struct SPI<M, P> {
    reg: ral::lpspi::Instance<M>,
    /// LPSPI effective input clock frequency
    source_clock: u32,
    pins: P,
}

/// Indicates an error when computing the parameters that control
/// the clock speed.
#[derive(Debug)]
pub struct ClockSpeedError(());

/// Indicates an error when computing the parameters that control
/// the mode.
#[derive(Debug)]
pub struct ModeError(());

/// Indicates an error when computing the parameters that control
/// the pin low timeout
#[derive(Debug)]
pub struct PinLowTimeoutError(());

/// Indicates an error when computing the parameters that control
/// the bus idle timeout
#[derive(Debug)]
pub struct BusIdleTimeoutError(());

const RETRIES: usize = 100_000;

/// Pins for a SPI device
///
/// Consider using type aliases to simplify your [`SPI`] usage:
///
/// ```no_run
/// use imxrt1060_hal as hal;
/// use hal::iomuxc::b0::*;
///
/// // SPI pins used in my application
/// type SPIPins = hal::spi::Pins<
///     B0_02,
///     B0_01,
///     B0_03,
///     B0_00,
/// >;
///
/// // Helper type for your SPI peripheral
/// type SPI<M> = hal::spi::SPI<M, SPIPins>;
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

impl<M, SDO, SDI, SCK, PCS0> SPI<M, Pins<SDO, SDI, SCK, PCS0>>
where
    M: Unsigned,
    SDO: spi::Pin<Module = M, Signal = spi::SDO>,
    SDI: spi::Pin<Module = M, Signal = spi::SDI>,
    SCK: spi::Pin<Module = M, Signal = spi::SCK>,
    PCS0: spi::Pin<Module = M, Signal = spi::PCS0>,
{
    /// Create a new SPI driver from the RAL SPI instance, a set of SPI pins, and
    /// an initialized SPI clock
    pub fn new(
        reg: ral::lpspi::Instance<M>,
        mut pins: Pins<SDO, SDI, SCK, PCS0>,
        clock: &crate::ccm::SPIClock,
    ) -> Self {
        spi::prepare(&mut pins.sdo);
        spi::prepare(&mut pins.sdi);
        spi::prepare(&mut pins.sck);
        spi::prepare(&mut pins.pcs0);

        let mut spi = SPI {
            reg,
            source_clock: clock.frequency(),
            pins,
        };
        ral::write_reg!(ral::lpspi, spi.reg, CR, RST: RST_1);
        ral::write_reg!(ral::lpspi, spi.reg, CR, RST: RST_0);
        spi.set_clock_speed(ClockSpeed::default()).unwrap();
        ral::write_reg!(
            ral::lpspi,
            spi.reg,
            CFGR1,
            MASTER: MASTER_1,
            SAMPLE: SAMPLE_1
        );
        spi.set_mode(embedded_hal::spi::MODE_0).unwrap();
        ral::write_reg!(ral::lpspi, spi.reg, FCR, RXWATER: 0xF, TXWATER: 0xF);
        ral::write_reg!(ral::lpspi, spi.reg, CR, MEN: MEN_1);
        spi
    }
}

impl<M, P> SPI<M, P>
where
    M: Unsigned,
{
    const DMA_DESTINATION_REQUEST_SIGNAL: u32 = DMA_TX_REQUEST_LOOKUP[M::USIZE - 1];
    const DMA_SOURCE_REQUEST_SIGNAL: u32 = DMA_RX_REQUEST_LOOKUP[M::USIZE - 1];

    /// Release the SPI peripheral components
    pub fn release(self) -> (ral::lpspi::Instance<M>, P) {
        (self.reg, self.pins)
    }

    fn with_master_disabled<F: FnMut() -> R, R>(&self, mut act: F) -> R {
        let men = ral::read_reg!(ral::lpspi, self.reg, CR, MEN == MEN_1);
        ral::modify_reg!(ral::lpspi, self.reg, CR, MEN: MEN_0);
        let res = act();
        ral::modify_reg!(ral::lpspi, self.reg, CR, MEN: (men as u32));
        res
    }

    /// Enables the peripheral-controlled chip select 0 (PCS0)
    ///
    /// Using the peripheral-controlled chip select is typically more efficient,
    /// and it means that software doesn't need to cooridnate its control.
    pub fn enable_chip_select_0<PCS>(&mut self, mut pcs: PCS)
    where
        PCS: spi::Pin<Module = M, Signal = spi::PCS0>,
    {
        crate::iomuxc::spi::prepare(&mut pcs);
    }

    /// Set the SPI mode for the peripheral
    pub fn set_mode(&mut self, mode: embedded_hal::spi::Mode) -> Result<(), ModeError> {
        ral::modify_reg!(
            ral::lpspi,
            self.reg,
            TCR,
            CPOL: ((mode.polarity == embedded_hal::spi::Polarity::IdleHigh) as u32),
            CPHA: ((mode.phase == embedded_hal::spi::Phase::CaptureOnSecondTransition) as u32)
        );
        Ok(())
    }

    /// Set the SPI master clock speed
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) -> Result<(), ClockSpeedError> {
        self.with_master_disabled(|| unsafe {
            // Safety: master is disabled
            clock_speed.set(self.source_clock, &self.reg);
            Ok(())
        })
    }

    #[inline(always)]
    fn wait<F>(&mut self, mut on: F) -> Result<(), Error>
    where
        F: FnMut(u32) -> bool,
    {
        for _ in 0..RETRIES {
            if on(self.check_errors()?) {
                return Ok(());
            }
        }
        Err(Error::WaitTimeout)
    }

    /// Clears all master status flags that are required to be
    /// low before acting as an SPI master.
    ///
    /// All flags are W1C.
    fn clear_status(&mut self) {
        ral::write_reg!(
            ral::lpspi,
            self.reg,
            SR,
            WCF: WCF_1,
            FCF: FCF_1,
            TCF: TCF_1,
            TEF: TEF_1,
            REF: REF_1,
            DMF: DMF_1
        );
    }

    /// Clear any existing data in the SPI receive or transfer FIFOs
    // TODO: for now I believe this is required to be public for the cases where an user wishes
    // to clear the FIFO.  It would be a bit cleaner if we had SPI transaction methods
    pub fn clear_fifo(&mut self) {
        ral::modify_reg!(ral::lpspi, self.reg, CR, RRF: RRF_1, RTF: RTF_1);
    }

    /// Check master status flags for erroneous conditions
    #[inline(always)]
    fn check_errors(&mut self) -> Result<u32, Error> {
        use ral::lpspi::SR::*;
        let status = ral::read_reg!(ral::lpspi, self.reg, SR);
        if status & TEF::mask != 0 {
            Err(Error::Transmit)
        } else if status & REF::mask != 0 {
            Err(Error::Receive)
        } else if status & DMF::mask != 0 {
            Err(Error::DataMismatch) // TODO: is this an error?
        } else {
            Ok(status)
        }
    }

    /// # Safety
    ///
    /// Interior mutability must be atomic
    #[inline(always)]
    unsafe fn set_frame_size<Word>(&self) {
        ral::modify_reg!(ral::lpspi, self.reg, TCR, FRAMESZ: ((core::mem::size_of::<Word>() * 8 - 1) as u32));
    }

    #[inline(always)]
    fn send<Word: Into<u32> + Copy>(&mut self, word: Word) -> nb::Result<(), Error> {
        use ral::lpspi::SR::*;

        let sr = self.check_errors()?;
        self.clear_status();
        // Safety: user provided mutable reference to SPI, so they are ensuring that
        // we can safely change this.
        unsafe { self.set_frame_size::<Word>() };

        if (sr & MBF::mask != 0) || (sr & TDF::mask == 0) {
            return Err(nb::Error::WouldBlock);
        }

        ral::write_reg!(ral::lpspi, self.reg, TDR, DATA: word.into());
        self.wait(|msr| msr & TDF::mask != 0)?;
        Ok(())
    }

    #[inline(always)]
    fn read(&mut self) -> nb::Result<u32, Error> {
        use ral::lpspi::SR::*;
        let sr = self.check_errors()?;
        if sr & MBF::mask != 0 {
            return Err(nb::Error::WouldBlock);
        }

        if ral::read_reg!(ral::lpspi, self.reg, RSR, RXEMPTY == RXEMPTY_0) {
            let word = ral::read_reg!(ral::lpspi, self.reg, RDR, DATA);
            Ok(word)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Perform common actions for enabling a DMA source
    ///
    /// # Safety
    ///
    /// Interior mutability must be atomic
    #[inline(always)]
    unsafe fn enable_dma_source<W>(&self) {
        self.set_frame_size::<W>();
        ral::modify_reg!(ral::lpspi, self.reg, FCR, RXWATER: 0); // No watermarks; affects DMA signaling
        ral::modify_reg!(ral::lpspi, self.reg, DER, RDDE: 1);
    }

    /// Perform common actions for disabling a DMA source
    ///
    /// # Safety
    ///
    /// Performs writes behind an immutable receiver. Interior mutability must be atomic.
    #[inline(always)]
    unsafe fn disable_dma_source(&self) {
        while ral::read_reg!(ral::lpspi, self.reg, DER, RDDE == 1) {
            ral::modify_reg!(ral::lpspi, self.reg, DER, RDDE: 0);
        }
    }

    /// # Safety
    ///
    /// Performs writes behind an immutable receiver. Interior mutability must be atomic.
    #[inline(always)]
    unsafe fn enable_dma_destination<W>(&self) {
        self.set_frame_size::<W>();
        ral::modify_reg!(ral::lpspi, self.reg, FCR, TXWATER: 0); // No watermarks; affects DMA signaling
        ral::modify_reg!(ral::lpspi, self.reg, DER, TDDE: 1);
    }

    /// # Safety
    ///
    /// Performs writes behind an immutable receiver. Interior mutability must be atomic.
    #[inline(always)]
    unsafe fn disable_dma_destination(&self) {
        while ral::read_reg!(ral::lpspi, self.reg, DER, TDDE == 1) {
            ral::modify_reg!(ral::lpspi, self.reg, DER, TDDE: 0);
        }
    }
}

/// An error that occured during a SPI operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A generic transmit error
    Transmit,
    /// A generic receive error
    Receive,
    /// Data mismatch error
    DataMismatch,
    /// Busy-wait on an internal flag was too long
    WaitTimeout,
}

impl<M, P> embedded_hal::spi::FullDuplex<u8> for SPI<M, P>
where
    M: Unsigned,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Self::read(self).map(|w| w as u8)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        Self::send::<u8>(self, word)
    }
}

impl<M, P> embedded_hal::blocking::spi::write::Default<u8> for SPI<M, P> where M: Unsigned {}
impl<M, P> embedded_hal::blocking::spi::transfer::Default<u8> for SPI<M, P> where M: Unsigned {}
impl<M, P> embedded_hal::blocking::spi::write_iter::Default<u8> for SPI<M, P> where M: Unsigned {}

impl<M, P> embedded_hal::spi::FullDuplex<u16> for SPI<M, P>
where
    M: Unsigned,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u16, Self::Error> {
        Self::read(self).map(|w| w as u16)
    }

    fn send(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        Self::send::<u16>(self, word)
    }
}

impl<M, P> embedded_hal::blocking::spi::write::Default<u16> for SPI<M, P> where M: Unsigned {}
impl<M, P> embedded_hal::blocking::spi::transfer::Default<u16> for SPI<M, P> where M: Unsigned {}
impl<M, P> embedded_hal::blocking::spi::write_iter::Default<u16> for SPI<M, P> where M: Unsigned {}

//
// DMA peripheral support
//

use crate::dma;

/// SPI RX DMA Request signal
///
/// See table 4-3 of the iMXRT1060 Reference Manual (Rev 2)
const DMA_RX_REQUEST_LOOKUP: [u32; 4] = [13, 77, 15, 79];

/// SPI TX DMA Request signal
///
/// See table 4-3 of the iMXRT1060 Reference Manual (Rev 2)
const DMA_TX_REQUEST_LOOKUP: [u32; 4] = [14, 78, 16, 80];

unsafe impl<M, P> dma::peripheral::Source<u8> for SPI<M, P>
where
    M: Unsigned,
{
    fn source_signal(&self) -> u32 {
        Self::DMA_SOURCE_REQUEST_SIGNAL
    }
    fn source(&self) -> *const u8 {
        &self.reg.RDR as *const _ as *const u8
    }
    fn enable_source(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.enable_dma_source::<u8>();
        });
    }
    fn disable_source(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.disable_dma_source();
        });
    }
}

unsafe impl<M, P> dma::peripheral::Destination<u8> for SPI<M, P>
where
    M: Unsigned,
{
    fn destination_signal(&self) -> u32 {
        Self::DMA_DESTINATION_REQUEST_SIGNAL
    }
    fn destination(&self) -> *const u8 {
        &self.reg.TDR as *const _ as *const u8
    }
    fn enable_destination(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.enable_dma_destination::<u8>();
        });
    }
    fn disable_destination(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.disable_dma_destination();
        });
    }
}

unsafe impl<M, P> dma::peripheral::Source<u16> for SPI<M, P>
where
    M: Unsigned,
{
    fn source_signal(&self) -> u32 {
        Self::DMA_SOURCE_REQUEST_SIGNAL
    }
    fn source(&self) -> *const u16 {
        &self.reg.RDR as *const _ as *const u16
    }
    fn enable_source(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.enable_dma_source::<u16>();
        });
    }
    fn disable_source(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.disable_dma_source();
        });
    }
}

unsafe impl<M, P> dma::peripheral::Destination<u16> for SPI<M, P>
where
    M: Unsigned,
{
    fn destination_signal(&self) -> u32 {
        Self::DMA_DESTINATION_REQUEST_SIGNAL
    }
    fn destination(&self) -> *const u16 {
        &self.reg.TDR as *const _ as *const u16
    }
    fn enable_destination(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.enable_dma_destination::<u16>();
        });
    }
    fn disable_destination(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            // Safety: atomic operation
            self.disable_dma_destination();
        });
    }
}
