//! Synchronous Audio Interface.
//!
//! [`Sai`] provides a pair of synchronous audio word streams containaing stereo data.
//!
//! This driver also exposes the peripheral's lower-level, hardware-dependent audio stream
//! configuration and FIFO pair.
//!
//! Each SAI instance has at minimum a tx and rx data line. Each data line supports up to 32 audio
//! words per frame. Audio words are 8 to 32 bits. Frames can be used to send multichannel audio
//! data over a single serial stream such as stereo audio.
//!
//! Each data line comes with its own 32x32 FIFO allowing for a full frame to be sent and/or received
//! without software interaction.
//!
//! The configuration of the SAI is encoded in configuration structure that can be used with a singular
//! configure method.

use crate::iomuxc::{consts, sai};
use crate::ral;
use core::marker::PhantomData;

/// Audio word byte order
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum ByteOrder {
    /// Least significant byte first
    #[default]
    LSB = 0,
    /// Most significant byte first
    MSB = 1,
}

/// Mode of operation for the SAI peripheral
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum Mode {
    /// Master mode where all clocks are generated from the SAI
    #[default]
    Master = 0,
    /// Slave mode where all audio clocks are expecting to come from external sources
    Slave = 1,
    /// Bitclock Master, FrameSync Slave
    BclkMasterFrameSyncSlave = 2,
    /// Bitclock Slave, FrameSync Master
    BclkSlaveFrameSyncMaster = 3,
}

/// Clock Polarity Options for Bclk/Mclk
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum ClockPolarity {
    /// Transmitted clock implies active high
    #[default]
    ActiveHigh = 0,
    /// Transmitted clock implies active low
    ActiveLow = 1,
}

#[allow(non_upper_case_globals)]
impl ClockPolarity {
    /// Received clock implies sample on rising edge
    pub const SampleOnRising: ClockPolarity = ClockPolarity::ActiveLow;
    /// Received clock implies sample on falling edge
    pub const SampleOnFalling: ClockPolarity = ClockPolarity::ActiveHigh;
}

/// Mclk source option
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum MclkSource {
    /// Mclk sourced from system clock
    #[default]
    Sysclk = 0,
    /// Select 1, part dependent
    Select1 = 1,
    /// Select 2, part dependent
    Select2 = 2,
    /// Select 3, part dependent
    Select3 = 3,
}

/// Frame sync mode between rx/tx
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum SyncMode {
    /// Both tx/rx are setup as being independent of eachother for frame sync
    #[default]
    Async = 0,
    /// Tx synchronously follows Rx frame sync
    TxFollowRx = 1,
    /// Rx synchronously follows Tx frame sync
    RxFollowTx = 2,
}

/// Frame Sync Width
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum SyncWidth {
    /// Frame sync width is the size of the word
    #[default]
    WordSize,
}

/// Source for Bclk, check part datasheet for details
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum BclkSource {
    /// Bus clock is the source of Bclk
    #[default]
    Bus = 0,
    /// Option 1, part dependent
    Opt1 = 1,
    /// Option 2, part dependent
    Opt2 = 2,
    /// Option 3, part dependent
    Opt3 = 3,
}

mod private {
    pub trait Sealed {}
}

/// Packing is useful to have function variants over so is provided as a trait
pub trait Packing<const WORD_SIZE: u8>: private::Sealed {
    /// FPACK register field value to set the appropriate byte packing of the FIFO
    const FPACK: u32;
}

/// Indicates No packing of audio words into a single 32bit FIFO word
pub struct PackingNone;

/// Indicates 8bit audio words packed into a single 32bit FIFO word
pub struct Packing8bit;

/// Indicates 16bit audio words packed into a single 32bit FIFO word
pub struct Packing16bit;

impl<const WORD_SIZE: u8> Packing<WORD_SIZE> for PackingNone {
    const FPACK: u32 = 0b00;
}

impl private::Sealed for PackingNone {}

impl Packing<8> for Packing8bit {
    const FPACK: u32 = 0b01;
}

impl private::Sealed for Packing8bit {}

impl Packing<16> for Packing16bit {
    const FPACK: u32 = 0b10;
}

impl private::Sealed for Packing16bit {}

#[allow(non_upper_case_globals)]
impl BclkSource {
    /// Mclk Divider as Bclk source
    pub const MclkDiv: BclkSource = BclkSource::Opt1;
}

fn reset_tx(regs: &ral::sai::RegisterBlock) {
    ral::write_reg!(ral::sai, regs, TCSR, SR: 1, FR: 1);
    ral::modify_reg!(ral::sai, regs, TCSR, SR: 0);
}

fn reset_rx(regs: &ral::sai::RegisterBlock) {
    ral::write_reg!(ral::sai, regs, RCSR, SR: 1, FR: 1);
    ral::modify_reg!(ral::sai, regs, RCSR, SR: 0);
}

fn reset(regs: &ral::sai::RegisterBlock) {
    reset_tx(regs);
    reset_rx(regs);
}

/// A set of Pins for Tx or Rx
///
/// NOTE: The Data type *could* be more than a single Pin.
pub struct Pins<Sync, Bclk, Data> {
    /// Frame sync pin
    pub sync: Sync,
    /// Bit clock pin
    pub bclk: Bclk,
    /// Data pin(s)
    pub data: Data,
}

/// Configuration for SAI peripheral
#[derive(Default)]
pub struct SaiConfig {
    mclk_source: MclkSource,
    tx_fifo_wm: u32,
    tx_stop_en: bool,
    tx_debug_en: bool,
    tx_bclk_div: u32,
    rx_fifo_wm: u32,
    rx_stop_en: bool,
    rx_debug_en: bool,
    rx_bclk_div: u32,
    byte_order: ByteOrder,
    mode: Mode,
    sync_width: SyncWidth,
    sync_early: bool,
    sync_polarity: ClockPolarity,
    sync_mode: SyncMode,
    bclk_src_swap: bool,
    bclk_input_delay: bool,
    bclk_polarity: ClockPolarity,
}

const MIN_BCLK_DIV: u32 = 2;
const MAX_BCLK_DIV: u32 = 512;

/// Compute a bclk divider setting given a desired numerical divider
pub fn bclk_div(bclk_div: u32) -> u32 {
    bclk_div.clamp(MIN_BCLK_DIV, MAX_BCLK_DIV).div_ceil(2) - 1
}

impl SaiConfig {
    /// Initialize a constant SaiConfig to be used with an i2s signaling scheme
    pub const fn i2s(bclk_div: u32) -> Self {
        // Get the bitclock divider for a given integer division. Notably non-even numbers
        // are rounded up as the bclk divider is *always* an even number from 2 to 512
        Self {
            mclk_source: MclkSource::Sysclk,
            tx_fifo_wm: 16,
            tx_stop_en: false,
            tx_debug_en: false,
            tx_bclk_div: bclk_div,
            rx_fifo_wm: 16,
            rx_stop_en: false,
            rx_debug_en: false,
            rx_bclk_div: bclk_div,
            byte_order: ByteOrder::MSB,
            mode: Mode::Master,
            sync_early: true,
            sync_width: SyncWidth::WordSize,
            sync_polarity: ClockPolarity::ActiveLow,
            sync_mode: SyncMode::Async,
            bclk_src_swap: false,
            bclk_input_delay: false,
            bclk_polarity: ClockPolarity::SampleOnRising,
        }
    }
}

/// A SAI peripheral instance
pub struct Sai<const N: u8, MclkPin, TxPins, RxPins> {
    pub(super) sai: ral::sai::Instance<N>,
    _mclk_pin: MclkPin,
    tx_pins: Option<TxPins>,
    rx_pins: Option<RxPins>,
    tx_chan_mask: u32,
    rx_chan_mask: u32,
}

/// A SAI transmit half
pub struct Tx<
    const N: u8,
    const WORD_SIZE: u8,
    const FRAME_SIZE: usize,
    PACKING: Packing<WORD_SIZE>,
> {
    sai: ral::sai::Instance<N>,
    _packing: PhantomData<PACKING>,
}

/// Needed for modifying the status register where some bit updates imply clearing status flags
/// which is undesirable
const SAI_STATUS_MASK: u32 = 0xFFE3FFFF;

/// Constant mask covering all interupt mask bits
const SAI_INT_MASK: u32 = ral::sai::TCSR::WSIE::mask
    | ral::sai::TCSR::SEIE::mask
    | ral::sai::TCSR::FEIE::mask
    | ral::sai::TCSR::FWIE::mask
    | ral::sai::TCSR::FRIE::mask;

impl<const N: u8, const WORD_SIZE: u8, const FRAME_SIZE: usize, PACKING: Packing<WORD_SIZE>>
    Tx<N, WORD_SIZE, FRAME_SIZE, PACKING>
{
    /// Enable/Disable transmission
    pub fn set_enable(&mut self, en: bool) {
        let mut tcsr = ral::read_reg!(ral::sai, self.sai, TCSR) & SAI_STATUS_MASK;
        if en {
            tcsr |= ral::sai::TCSR::TE::mask
        } else {
            tcsr &= !ral::sai::TCSR::TE::mask
        }
        ral::write_reg!(ral::sai, self.sai, TCSR, tcsr);
        self.clear_status();
    }

    /// Mask/Unmask interrupts, takes the full set of mask values to update
    /// and modifies them wholesale clearing and setting interrupt bits.
    ///
    /// ```no_run
    /// use imxrt_ral::sai::{SAI1, TCSR};
    /// use imxrt_hal::sai::{Tx, PackingNone, Sai, SaiConfig};
    /// let sai = Sai::without_pins(unsafe { SAI1::instance() }, 0, 0);
    /// let (Some(mut sai_tx), None) = sai.split::<16, 2, PackingNone>(&SaiConfig::i2s(8)) else { panic!() };
    ///
    /// sai_tx.set_int_mask(TCSR::FEIE::mask | TCSR::FWIE::mask | TCSR::FRIE::mask);
    /// ```
    pub fn set_int_mask(&mut self, mask: u32) {
        let mut tcsr = ral::read_reg!(ral::sai, self.sai, TCSR) & !SAI_INT_MASK;
        tcsr |= mask & SAI_INT_MASK;
        ral::write_reg!(ral::sai, self.sai, TCSR, tcsr);
    }

    /// Get the status register of the transmitter, this can be used in conjuction with
    /// status field masks to determine the state of the SAI peripheral.
    pub fn status(&mut self) -> u32 {
        ral::read_reg!(ral::sai, self.sai, TCSR)
    }

    /// Clear status error flags
    pub fn clear_status(&mut self) {
        let mut tcsr = ral::read_reg!(ral::sai, self.sai, TCSR) & SAI_STATUS_MASK;
        tcsr |= ral::sai::TCSR::FEF::mask | ral::sai::TCSR::WSF::mask | ral::sai::TCSR::SEF::mask;
        ral::write_reg!(ral::sai, self.sai, TCSR, tcsr);
    }

    /// Get a dump of the Tx configuration registers
    pub fn reg_dump(&mut self) -> [u32; 5] {
        [
            ral::read_reg!(ral::sai, self.sai, TCR1),
            ral::read_reg!(ral::sai, self.sai, TCR2),
            ral::read_reg!(ral::sai, self.sai, TCR3),
            ral::read_reg!(ral::sai, self.sai, TCR4),
            ral::read_reg!(ral::sai, self.sai, TCR5),
        ]
    }

    /// Get the FIFO write and read position
    ///
    /// ```no_run
    /// use imxrt_ral::sai::{SAI1, TCSR};
    /// use imxrt_hal::sai::{Tx, PackingNone, Sai, SaiConfig};
    /// let sai = Sai::without_pins(unsafe { SAI1::instance() }, 0, 0);
    /// let (Some(mut sai_tx), None) = sai.split::<16, 2, PackingNone>(&SaiConfig::i2s(8)) else { panic!() };
    ///
    /// let (write_pos, read_pos) = sai_tx.fifo_position(0);
    /// ```
    pub fn fifo_position(&mut self, chan: usize) -> (u32, u32) {
        ral::read_reg!(ral::sai, self.sai, TFR[chan], WFP, RFP)
    }
}

impl<const N: u8, const FRAME_SIZE: usize> Tx<N, 32, FRAME_SIZE, PackingNone> {
    /// Write without checks or blocking a single audio frame to channels FIFO
    pub fn write_frame(&mut self, chan: usize, frame: [u32; FRAME_SIZE]) {
        for sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample);
        }
    }
}

impl<const N: u8, const FRAME_SIZE: usize> Tx<N, 16, FRAME_SIZE, PackingNone> {
    /// Write without checks or blocking a single audio frame to channels FIFO
    pub fn write_frame(&mut self, chan: usize, frame: [u16; FRAME_SIZE]) {
        for sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample as u32);
        }
    }
}

impl<const N: u8, const FRAME_SIZE: usize> Tx<N, 8, FRAME_SIZE, PackingNone> {
    /// Write without checks or blocking a single audio frame to channels FIFO
    pub fn write_frame(&mut self, chan: usize, frame: [u8; FRAME_SIZE]) {
        for sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample as u32);
        }
    }
}

/// A SAI receive half
pub struct Rx<
    const N: u8,
    const WORD_SIZE: u8,
    const FRAME_SIZE: usize,
    PACKING: Packing<WORD_SIZE>,
> {
    _sai: ral::sai::Instance<N>,
    _packing: PhantomData<PACKING>,
}

impl<const N: u8, Chan, Mclk, TxSync, TxBclk, TxData> Sai<N, Mclk, Pins<TxSync, TxBclk, TxData>, ()>
where
    Mclk: sai::Pin<consts::Const<N>, Signal = sai::Mclk>,
    TxSync: sai::Pin<consts::Const<N>, Signal = sai::TxSync>,
    TxBclk: sai::Pin<consts::Const<N>, Signal = sai::TxBclk>,
    TxData: sai::Pin<consts::Const<N>>,
    Chan: consts::Unsigned,
    <TxData as sai::Pin<consts::Const<N>>>::Signal: sai::TxDataSignal<Index = Chan>,
{
    /// The peripheral instance.
    pub const N: u8 = N;

    /// Create a Sai instance given a set of transmit pins
    pub fn from_tx(
        sai: ral::sai::Instance<N>,
        mut mclk_pin: Mclk,
        mut tx_pins: Pins<TxSync, TxBclk, TxData>,
    ) -> Self {
        reset(&sai);

        sai::prepare(&mut mclk_pin);
        sai::prepare(&mut tx_pins.sync);
        sai::prepare(&mut tx_pins.bclk);
        sai::prepare(&mut tx_pins.data);

        Sai {
            sai,
            _mclk_pin: mclk_pin,
            tx_pins: Some(tx_pins),
            rx_pins: None,
            tx_chan_mask: 1 << Chan::to_usize(),
            rx_chan_mask: 0,
        }
    }
}

impl<const N: u8, Chan, Mclk, RxSync, RxBclk, RxData> Sai<N, Mclk, (), Pins<RxSync, RxBclk, RxData>>
where
    Mclk: sai::Pin<consts::Const<N>, Signal = sai::Mclk>,
    RxSync: sai::Pin<consts::Const<N>, Signal = sai::RxSync>,
    RxBclk: sai::Pin<consts::Const<N>, Signal = sai::RxBclk>,
    RxData: sai::Pin<consts::Const<N>>,
    Chan: consts::Unsigned,
    <RxData as sai::Pin<consts::Const<N>>>::Signal: sai::RxDataSignal<Index = Chan>,
{
    /// Create a Sai instance given a set of receive pins
    pub fn from_rx(
        sai: ral::sai::Instance<N>,
        mut mclk_pin: Mclk,
        mut rx_pins: Pins<RxSync, RxBclk, RxData>,
    ) -> Self {
        reset(&sai);

        sai::prepare(&mut mclk_pin);
        sai::prepare(&mut rx_pins.sync);
        sai::prepare(&mut rx_pins.bclk);
        sai::prepare(&mut rx_pins.data);

        Sai {
            sai,
            _mclk_pin: mclk_pin,
            tx_pins: None,
            rx_pins: Some(rx_pins),
            tx_chan_mask: 0,
            rx_chan_mask: 1 << Chan::to_usize(),
        }
    }
}

impl<const N: u8> Sai<N, (), (), ()> {
    /// Create a new SAI driver from the RAL SAI instance.
    ///
    /// You're responsible for configuring pins, and for making sure
    /// the pin configuration doesn't change while this driver is in use.
    /// Setting the channel mask is *also* your responsibility
    pub fn without_pins(sai: ral::sai::Instance<N>, tx_chan_mask: u32, rx_chan_mask: u32) -> Self {
        Sai {
            sai,
            _mclk_pin: (),
            tx_pins: None,
            rx_pins: None,
            tx_chan_mask,
            rx_chan_mask,
        }
    }
}

impl<const N: u8, Mclk, TxPins, RxPins> Sai<N, Mclk, TxPins, RxPins> {
    /// Split the Tx/Rx pair from a SAI, with word, frame, and packing options as type parameters
    pub fn split<const WORD_SIZE: u8, const FRAME_SIZE: usize, PACKING: Packing<WORD_SIZE>>(
        self,
        cfg: &SaiConfig,
    ) -> (
        Option<Tx<N, WORD_SIZE, FRAME_SIZE, PACKING>>,
        Option<Rx<N, WORD_SIZE, FRAME_SIZE, PACKING>>,
    ) {
        // Set the mclk pin to be an output
        unsafe {
            ral::write_reg!(ral::iomuxc_gpr, ral::iomuxc_gpr::IOMUXC_GPR::instance(), GPR1, SAI1_MCLK_DIR: 1);
        }

        let tx = self.tx_pins.map(|_| Tx {
            sai: unsafe { ral::sai::Instance::<N>::new(&*self.sai) },
            _packing: PhantomData::<PACKING>,
        });

        let rx = self.rx_pins.map(|_| Rx {
            _sai: unsafe { ral::sai::Instance::<N>::new(&*self.sai) },
            _packing: PhantomData::<PACKING>,
        });

        let frame_sync_dir = match cfg.mode {
            Mode::Master => 1,
            Mode::BclkSlaveFrameSyncMaster => 1,
            _ => 0,
        };

        let bclk_dir = match cfg.mode {
            Mode::Master => 1,
            Mode::BclkMasterFrameSyncSlave => 1,
            _ => 0,
        };

        let (tx_sync_mode, rx_sync_mode) = match cfg.sync_mode {
            SyncMode::Async => (0b00, 0b00),
            SyncMode::TxFollowRx => (0b01, 0b00),
            SyncMode::RxFollowTx => (0b00, 0b01),
        };

        let sync_width = match cfg.sync_width {
            SyncWidth::WordSize => WORD_SIZE as u32,
        };

        if tx.is_some() {
            ral::write_reg!(ral::sai, self.sai, TCR1, TFW: cfg.tx_fifo_wm);
            if cfg.mode == Mode::Master || cfg.mode == Mode::BclkMasterFrameSyncSlave {
                ral::write_reg!(ral::sai, self.sai, TCR2, SYNC: tx_sync_mode,
                    BCS: cfg.bclk_src_swap as u32, BCI: cfg.bclk_input_delay as u32,
                    MSEL: 0x11_u32, BCP: cfg.bclk_polarity as u32, BCD: bclk_dir,
                    DIV: cfg.tx_bclk_div);
            } else {
                ral::modify_reg!(ral::sai, self.sai, TCR2, BCP: cfg.bclk_polarity as u32);
            }
            ral::modify_reg!(ral::sai, self.sai, TCR3, TCE: self.tx_chan_mask, WDFL: 0_u32);
            ral::write_reg!(ral::sai, self.sai, TCR4, FRSZ: ((FRAME_SIZE - 1) as u32),
                FPACK: 0_u32, SYWD: (sync_width - 1), MF: cfg.byte_order as u32,
                FSE: cfg.sync_early as u32, FSP: cfg.sync_polarity as u32, FSD: frame_sync_dir);
            ral::write_reg!(ral::sai, self.sai, TCR5, W0W: ((WORD_SIZE - 1) as u32), WNW: ((WORD_SIZE - 1) as u32), FBT: (WORD_SIZE - 1) as u32);
            ral::write_reg!(ral::sai, self.sai, TCSR, TE: 0, STOPE: cfg.tx_stop_en as u32,
                DBGE: cfg.tx_debug_en as u32, BCE: 1, WSF: 1, SEF: 1, FEF: 1, FWF: 0, FRF: 0,
                WSIE: 0, SEIE: 0, FEIE: 0, FWIE: 0, FWDE: 0, FRDE: 0);
        }

        if rx.is_some() {
            ral::write_reg!(ral::sai, self.sai, RCR1, RFW: cfg.rx_fifo_wm);
            if cfg.mode == Mode::Master || cfg.mode == Mode::BclkMasterFrameSyncSlave {
                ral::write_reg!(ral::sai, self.sai, RCR2, SYNC: rx_sync_mode,
                    BCS: cfg.bclk_src_swap as u32, BCI: cfg.bclk_input_delay as u32,
                    MSEL: cfg.mclk_source as u32, BCP: cfg.bclk_polarity as u32, BCD: bclk_dir,
                    DIV: cfg.rx_bclk_div);
            } else {
                ral::modify_reg!(ral::sai, self.sai, RCR2, BCP: cfg.bclk_polarity as u32);
            }
            ral::modify_reg!(ral::sai, self.sai, RCR3, RCE: self.rx_chan_mask);
            ral::write_reg!(ral::sai, self.sai, RCR4, FRSZ: ((FRAME_SIZE - 1) as u32),
                FPACK: PACKING::FPACK, SYWD: (sync_width - 1), MF: cfg.byte_order as u32,
                FSE: cfg.sync_early as u32, FSP: cfg.sync_polarity as u32, FSD: frame_sync_dir);
            ral::write_reg!(ral::sai, self.sai, RCR5, W0W: ((WORD_SIZE - 1) as u32), WNW: ((WORD_SIZE - 1) as u32));
            ral::write_reg!(ral::sai, self.sai, RCSR, RE: 0, STOPE: cfg.rx_stop_en as u32,
                DBGE: cfg.rx_debug_en as u32, BCE: 1, WSF: 1, SEF: 1, FEF: 1, FWF: 0, FRF: 0,
                WSIE: 0, SEIE: 0, FEIE: 0, FWIE: 0, FWDE: 0, FRDE: 0);
        }

        (tx, rx)
    }
}
