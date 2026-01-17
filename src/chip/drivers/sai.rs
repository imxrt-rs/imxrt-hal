//! Synchronous Audio Interface.
//!
//! [`Sai`] provides a pair of synchronous audio word streams containing stereo data.
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
//!
//! ## DMA
//!
//! DMA transfers target the lowest-numbered enabled data line for each direction
//! (TX/RX). Multi-channel frames (e.g. stereo) on a single data line work
//! naturally, since frame words are interleaved through the same TDR/RDR by the
//! hardware FIFO. However, multiple data lines are not supported by DMA — only
//! the lowest-numbered enabled data line is used.
//!
//! ## Clock configuration
//!
//! Make sure to configure your clocks before using the audio interface. Note that there may be
//! additional clock settings in `IOMUXC_GPR`.

use crate::iomuxc::{consts, sai};
use crate::ral;

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
    /// Both tx/rx are setup as being independent of each other for frame sync
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

bitflags::bitflags! {
    /// Interrupt settings.
    ///
    /// A set bit indicates that the interrupt is enabled.
    pub struct Interrupts : u32 {
        /// Word Start Interrupt Enable.
        const WORD_START = 1 << 12;
        /// Sync Error Interrupt Enable.
        const SYNC_ERROR = 1 << 11;
        /// FIFO Error Interrupt Enable.
        const FIFO_ERROR = 1 << 10;
        /// FIFO Warning Interrupt Enable.
        const FIFO_WARNING = 1 << 9;
        /// FIFO Request Interrupt Enable.
        const FIFO_REQUEST = 1 << 8;
    }
}

bitflags::bitflags! {
    /// Status flags.
    pub struct Status : u32 {
        /// Word Start Flag.
        ///
        /// Indicates that the start of the configured word has been detected.
        const WORD_START = 1 << 20;
        /// Sync Error Flag.
        ///
        /// Indicates that an error in the externally-generated frame sync has been detected.
        const SYNC_ERROR = 1 << 19;
        /// FIFO Error Flag.
        ///
        /// Indicates that an enabled
        /// * receive FIFO has overflowed
        /// * transmit FIFO has underrun
        const FIFO_ERROR = 1 << 18;
        /// FIFO Warning Flag.
        ///
        /// Indicates that an enabled
        /// * receive FIFO is full
        /// * transmit FIFO is empty
        const FIFO_WARNING = 1 << 17;
        /// FIFO Request Flag.
        ///
        /// Indicates that the number of words in an enabled
        /// * receive channel FIFO is greater than the receive FIFO watermark
        /// * transmit channel FIFO is less than or equal to the transmit FIFO watermark
        const FIFO_REQUEST = 1 << 16;
    }
}

impl Status {
    const W1C: Self = Self::from_bits_truncate(
        Self::WORD_START.bits() | Self::SYNC_ERROR.bits() | Self::FIFO_ERROR.bits(),
    );
}

/// FIFO packing mode for audio words.
///
/// Packing allows multiple smaller audio words to be packed into a single
/// 32-bit FIFO entry.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Packing {
    /// No packing of audio words into a single 32-bit FIFO word.
    ///
    /// Each audio word occupies one 32-bit FIFO entry.
    #[default]
    None = 0b00,
    /// 8-bit audio words packed into a single 32-bit FIFO word.
    ///
    /// Four 8-bit audio words are packed per FIFO entry. Only valid when
    /// word size is 8 bits.
    Pack8bit = 0b10,
    /// 16-bit audio words packed into a single 32-bit FIFO word.
    ///
    /// Two 16-bit audio words are packed per FIFO entry. Only valid when
    /// word size is 16 bits.
    Pack16bit = 0b11,
}

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
    /// MCLK source
    pub mclk_source: MclkSource,
    /// TX fifo watermark
    pub tx_fifo_wm: u32,
    /// TX stop enable
    pub tx_stop_en: bool,
    /// TX debug enable
    pub tx_debug_en: bool,
    /// TX BCLK divider
    pub tx_bclk_div: u32,
    /// RX FIFO watermark
    pub rx_fifo_wm: u32,
    /// RX stop enable
    pub rx_stop_en: bool,
    /// RX debug enable
    pub rx_debug_en: bool,
    /// RX BCLK divider
    pub rx_bclk_div: u32,
    /// Byte order
    pub byte_order: ByteOrder,
    /// Mode
    pub mode: Mode,
    /// Sync width
    pub sync_width: SyncWidth,
    /// Sync early
    pub sync_early: bool,
    /// Sync polarity
    pub sync_polarity: ClockPolarity,
    /// Sync mode
    pub sync_mode: SyncMode,
    /// BCLK source swap
    pub bclk_src_swap: bool,
    /// BCLK input delay
    pub bclk_input_delay: bool,
    /// BCLK polarity
    pub bclk_polarity: ClockPolarity,
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

type AnyInstance = crate::AnyInstance<ral::sai::RegisterBlock>;

/// A SAI peripheral instance.
pub struct Sai {
    sai: AnyInstance,
    tx_chan_mask: u32,
    rx_chan_mask: u32,
}

impl Sai {
    /// Creates SAI instance with single channel RX and TX.
    pub fn new<const N: u8, Chan, Mclk, TxSync, TxBclk, TxData, RxSync, RxBclk, RxData>(
        sai: ral::sai::Instance<N>,
        mut mclk_pin: Mclk,
        mut tx_pins: Pins<TxSync, TxBclk, TxData>,
        mut rx_pins: Pins<RxSync, RxBclk, RxData>,
    ) -> Self
    where
        Mclk: sai::Pin<consts::Const<N>, Signal = sai::Mclk>,
        TxSync: sai::Pin<consts::Const<N>, Signal = sai::TxSync>,
        TxBclk: sai::Pin<consts::Const<N>, Signal = sai::TxBclk>,
        TxData: sai::Pin<consts::Const<N>>,
        RxSync: sai::Pin<consts::Const<N>, Signal = sai::RxSync>,
        RxBclk: sai::Pin<consts::Const<N>, Signal = sai::RxBclk>,
        RxData: sai::Pin<consts::Const<N>>,
        Chan: consts::Unsigned,
        <TxData as sai::Pin<consts::Const<N>>>::Signal: sai::TxDataSignal<Index = Chan>,
        <RxData as sai::Pin<consts::Const<N>>>::Signal: sai::RxDataSignal<Index = Chan>,
    {
        reset(&sai);

        sai::prepare(&mut mclk_pin);
        sai::prepare(&mut tx_pins.sync);
        sai::prepare(&mut tx_pins.bclk);
        sai::prepare(&mut tx_pins.data);
        sai::prepare(&mut rx_pins.sync);
        sai::prepare(&mut rx_pins.bclk);
        sai::prepare(&mut rx_pins.data);

        Self {
            sai: crate::into_any(sai),
            tx_chan_mask: 1 << Chan::to_usize(),
            rx_chan_mask: 1 << Chan::to_usize(),
        }
    }
}

/// A SAI transmit half.
pub struct Tx {
    pub(crate) sai: AnyInstance,
    word_size: u8,
    frame_size: usize,
    channel: usize,
}

impl Tx {
    /// Returns the word size in bits.
    pub fn word_size(&self) -> u8 {
        self.word_size
    }

    /// Returns the frame size (number of words per frame).
    pub fn frame_size(&self) -> usize {
        self.frame_size
    }

    /// Enable/Disable transmission
    pub fn set_enable(&mut self, en: bool) {
        let mut tcsr = ral::read_reg!(ral::sai, self.sai, TCSR) & !Status::W1C.bits();
        if en {
            tcsr |= ral::sai::TCSR::TE::mask
        } else {
            tcsr &= !ral::sai::TCSR::TE::mask
        }
        ral::write_reg!(ral::sai, self.sai, TCSR, tcsr);
        self.clear_status(Status::W1C);
    }

    /// Return the interrupt flags.
    ///
    /// The interrupt flags indicate the reasons that this peripheral may generate an interrupt.
    pub fn interrupts(&self) -> Interrupts {
        let tcsr = ral::read_reg!(ral::sai, self.sai, TCSR);
        Interrupts::from_bits_truncate(tcsr)
    }

    /// Set the interrupt flags for this SAI transmitter.
    pub fn set_interrupts(&mut self, interrupts: Interrupts) {
        ral::modify_reg!(ral::sai, self.sai, TCSR, |tcsr| {
            let tcsr = tcsr & !Interrupts::all().bits();
            tcsr | interrupts.bits()
        })
    }

    /// Get the status register of the transmitter, this can be used in conjunction with
    /// status field masks to determine the state of the SAI peripheral.
    pub fn status(&mut self) -> Status {
        let tcsr = ral::read_reg!(ral::sai, self.sai, TCSR);
        Status::from_bits_truncate(tcsr)
    }

    /// Clear status error flags
    pub fn clear_status(&mut self, flags: Status) {
        let flags = flags & Status::W1C;
        ral::modify_reg!(ral::sai, self.sai, TCSR, |tcsr| { tcsr | flags.bits() });
    }

    /// Get a dump of the Tx configuration registers
    pub fn reg_dump(&mut self) -> [u32; 6] {
        [
            ral::read_reg!(ral::sai, self.sai, TCR1),
            ral::read_reg!(ral::sai, self.sai, TCR2),
            ral::read_reg!(ral::sai, self.sai, TCR3),
            ral::read_reg!(ral::sai, self.sai, TCR4),
            ral::read_reg!(ral::sai, self.sai, TCR5),
            ral::read_reg!(ral::sai, self.sai, TCSR),
        ]
    }

    /// Get the FIFO write and read position
    ///
    /// ```no_run
    /// use imxrt_ral::sai::SAI1;
    /// use imxrt_hal::sai::{Packing, Sai, SaiConfig};
    /// let sai = Sai::without_pins(unsafe { SAI1::instance() }, 0, 0);
    /// let (Some(mut sai_tx), None) = sai.split(16, 2, Packing::None, &SaiConfig::i2s(8)) else { panic!() };
    ///
    /// let (write_pos, read_pos) = sai_tx.fifo_position(0);
    /// ```
    pub fn fifo_position(&mut self, chan: usize) -> (u32, u32) {
        ral::read_reg!(ral::sai, self.sai, TFR[chan], WFP, RFP)
    }

    /// Returns the data channel index used by this transmitter.
    pub fn channel(&self) -> usize {
        self.channel
    }

    /// Produces a pointer to the transmit data register for the given channel.
    ///
    /// Use this pointer when coordinating a DMA transfer.
    pub fn tdr(&self, chan: usize) -> *const u32 {
        core::ptr::addr_of!(self.sai.TDR[chan]).cast()
    }

    /// Enable DMA request on FIFO warning (FWDE bit in TCSR).
    ///
    /// When enabled, the transmit FIFO generates a DMA request whenever
    /// the number of words in the FIFO falls to or below the watermark.
    pub fn enable_dma_transmit(&mut self) {
        ral::modify_reg!(ral::sai, self.sai, TCSR, |tcsr| {
            (tcsr & !Status::W1C.bits()) | ral::sai::TCSR::FWDE::mask
        });
    }

    /// Disable DMA request on FIFO warning (clear FWDE bit in TCSR).
    pub fn disable_dma_transmit(&mut self) {
        ral::modify_reg!(ral::sai, self.sai, TCSR, |tcsr| {
            (tcsr & !Status::W1C.bits()) & !ral::sai::TCSR::FWDE::mask
        });
    }

    /// Write a single audio frame of 32-bit samples to a channel's FIFO.
    ///
    /// This writes samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn write_frame_u32(&mut self, chan: usize, frame: &[u32]) {
        for &sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample);
        }
    }

    /// Write a single audio frame of 16-bit samples to a channel's FIFO.
    ///
    /// This writes samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn write_frame_u16(&mut self, chan: usize, frame: &[u16]) {
        for &sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample as u32);
        }
    }

    /// Write a single audio frame of 8-bit samples to a channel's FIFO.
    ///
    /// This writes samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn write_frame_u8(&mut self, chan: usize, frame: &[u8]) {
        for &sample in frame {
            ral::write_reg!(ral::sai, self.sai, TDR[chan], sample as u32);
        }
    }
}

/// A SAI receive half.
pub struct Rx {
    pub(crate) sai: AnyInstance,
    word_size: u8,
    frame_size: usize,
    channel: usize,
}

impl Rx {
    /// Returns the word size in bits.
    pub fn word_size(&self) -> u8 {
        self.word_size
    }

    /// Returns the frame size (number of words per frame).
    pub fn frame_size(&self) -> usize {
        self.frame_size
    }

    /// Enable/Disable reception.
    pub fn set_enable(&mut self, en: bool) {
        let mut rcsr = ral::read_reg!(ral::sai, self.sai, RCSR) & !Status::W1C.bits();
        if en {
            rcsr |= ral::sai::RCSR::RE::mask
        } else {
            rcsr &= !ral::sai::RCSR::RE::mask
        }
        ral::write_reg!(ral::sai, self.sai, RCSR, rcsr);
        self.clear_status(Status::W1C);
    }

    /// Return the interrupt flags.
    ///
    /// The interrupt flags indicate the reasons that this peripheral may generate an interrupt.
    pub fn interrupts(&self) -> Interrupts {
        let rcsr = ral::read_reg!(ral::sai, self.sai, RCSR);
        Interrupts::from_bits_truncate(rcsr)
    }

    /// Set the interrupt flags for this SAI receiver.
    pub fn set_interrupts(&mut self, interrupts: Interrupts) {
        ral::modify_reg!(ral::sai, self.sai, RCSR, |rcsr| {
            let rcsr = rcsr & !Interrupts::all().bits();
            rcsr | interrupts.bits()
        })
    }

    /// Get the status register of the receiver, this can be used in conjunction with
    /// status field masks to determine the state of the SAI peripheral.
    pub fn status(&mut self) -> Status {
        let rcsr = ral::read_reg!(ral::sai, self.sai, RCSR);
        Status::from_bits_truncate(rcsr)
    }

    /// Clear status error flags
    pub fn clear_status(&mut self, flags: Status) {
        let flags = flags & Status::W1C;
        ral::modify_reg!(ral::sai, self.sai, RCSR, |rcsr| { rcsr | flags.bits() });
    }

    /// Get a dump of the Rx configuration registers
    pub fn reg_dump(&mut self) -> [u32; 6] {
        [
            ral::read_reg!(ral::sai, self.sai, RCR1),
            ral::read_reg!(ral::sai, self.sai, RCR2),
            ral::read_reg!(ral::sai, self.sai, RCR3),
            ral::read_reg!(ral::sai, self.sai, RCR4),
            ral::read_reg!(ral::sai, self.sai, RCR5),
            ral::read_reg!(ral::sai, self.sai, RCSR),
        ]
    }

    /// Get the FIFO write and read position
    ///
    /// ```no_run
    /// use imxrt_ral::sai::SAI1;
    /// use imxrt_hal::sai::{Packing, Sai, SaiConfig};
    /// let sai = Sai::without_pins(unsafe { SAI1::instance() }, 0, 0);
    /// let (None, Some(mut sai_rx)) = sai.split(16, 2, Packing::None, &SaiConfig::i2s(8)) else { panic!() };
    ///
    /// let (write_pos, read_pos) = sai_rx.fifo_position(0);
    /// ```
    pub fn fifo_position(&mut self, chan: usize) -> (u32, u32) {
        ral::read_reg!(ral::sai, self.sai, RFR[chan], WFP, RFP)
    }

    /// Returns the data channel index used by this receiver.
    pub fn channel(&self) -> usize {
        self.channel
    }

    /// Produces a pointer to the receive data register for the given channel.
    ///
    /// Use this pointer when coordinating a DMA transfer.
    pub fn rdr(&self, chan: usize) -> *const u32 {
        core::ptr::addr_of!(self.sai.RDR[chan]).cast()
    }

    /// Enable DMA request on FIFO request (FRDE bit in RCSR).
    ///
    /// When enabled, the receive FIFO generates a DMA request whenever
    /// the number of words in the FIFO reaches the watermark.
    pub fn enable_dma_receive(&mut self) {
        ral::modify_reg!(ral::sai, self.sai, RCSR, |rcsr| {
            (rcsr & !Status::W1C.bits()) | ral::sai::RCSR::FRDE::mask
        });
    }

    /// Disable DMA request on FIFO request (clear FRDE bit in RCSR).
    pub fn disable_dma_receive(&mut self) {
        ral::modify_reg!(ral::sai, self.sai, RCSR, |rcsr| {
            (rcsr & !Status::W1C.bits()) & !ral::sai::RCSR::FRDE::mask
        });
    }

    /// Read a single audio frame of 32-bit samples from a channel's FIFO.
    ///
    /// This reads samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn read_frame_u32(&mut self, chan: usize, frame: &mut [u32]) {
        for sample in frame {
            *sample = ral::read_reg!(ral::sai, self.sai, RDR[chan]);
        }
    }

    /// Read a single audio frame of 16-bit samples from a channel's FIFO.
    ///
    /// This reads samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn read_frame_u16(&mut self, chan: usize, frame: &mut [u16]) {
        for sample in frame {
            *sample = ral::read_reg!(ral::sai, self.sai, RDR[chan]) as u16;
        }
    }

    /// Read a single audio frame of 8-bit samples from a channel's FIFO.
    ///
    /// This reads samples without checks or blocking.
    ///
    /// Your slice is expected to be sized based on the configured frame size.
    /// For example, if your frame size is 2, then your slice should be two
    /// elements large.
    pub fn read_frame_u8(&mut self, chan: usize, frame: &mut [u8]) {
        for sample in frame {
            *sample = ral::read_reg!(ral::sai, self.sai, RDR[chan]) as u8;
        }
    }
}

impl Sai {
    /// Create a Sai instance given a set of transmit pins.
    pub fn from_tx<const N: u8, Chan, Mclk, TxSync, TxBclk, TxData>(
        sai: ral::sai::Instance<N>,
        mut mclk_pin: Mclk,
        mut tx_pins: Pins<TxSync, TxBclk, TxData>,
    ) -> Self
    where
        Mclk: sai::Pin<consts::Const<N>, Signal = sai::Mclk>,
        TxSync: sai::Pin<consts::Const<N>, Signal = sai::TxSync>,
        TxBclk: sai::Pin<consts::Const<N>, Signal = sai::TxBclk>,
        TxData: sai::Pin<consts::Const<N>>,
        Chan: consts::Unsigned,
        <TxData as sai::Pin<consts::Const<N>>>::Signal: sai::TxDataSignal<Index = Chan>,
    {
        reset(&sai);

        sai::prepare(&mut mclk_pin);
        sai::prepare(&mut tx_pins.sync);
        sai::prepare(&mut tx_pins.bclk);
        sai::prepare(&mut tx_pins.data);

        Sai {
            sai: crate::into_any(sai),
            tx_chan_mask: 1 << Chan::to_usize(),
            rx_chan_mask: 0,
        }
    }

    /// Create a Sai instance given a set of receive pins.
    pub fn from_rx<const N: u8, Chan, Mclk, RxSync, RxBclk, RxData>(
        sai: ral::sai::Instance<N>,
        mut mclk_pin: Mclk,
        mut rx_pins: Pins<RxSync, RxBclk, RxData>,
    ) -> Self
    where
        Mclk: sai::Pin<consts::Const<N>, Signal = sai::Mclk>,
        RxSync: sai::Pin<consts::Const<N>, Signal = sai::RxSync>,
        RxBclk: sai::Pin<consts::Const<N>, Signal = sai::RxBclk>,
        RxData: sai::Pin<consts::Const<N>>,
        Chan: consts::Unsigned,
        <RxData as sai::Pin<consts::Const<N>>>::Signal: sai::RxDataSignal<Index = Chan>,
    {
        reset(&sai);

        sai::prepare(&mut mclk_pin);
        sai::prepare(&mut rx_pins.sync);
        sai::prepare(&mut rx_pins.bclk);
        sai::prepare(&mut rx_pins.data);

        Sai {
            sai: crate::into_any(sai),
            tx_chan_mask: 0,
            rx_chan_mask: 1 << Chan::to_usize(),
        }
    }

    /// Create a new SAI driver from the RAL SAI instance.
    ///
    /// You're responsible for configuring pins, and for making sure
    /// the pin configuration doesn't change while this driver is in use.
    /// Setting the channel mask is *also* your responsibility.
    pub fn without_pins<const N: u8>(
        sai: ral::sai::Instance<N>,
        tx_chan_mask: u32,
        rx_chan_mask: u32,
    ) -> Self {
        Sai {
            sai: crate::into_any(sai),
            tx_chan_mask,
            rx_chan_mask,
        }
    }

    /// Split the Tx/Rx pair from a SAI.
    ///
    /// # Arguments
    ///
    /// * `word_size` - Audio word size in bits (8-32)
    /// * `frame_size` - Number of words per frame
    /// * `packing` - FIFO packing mode
    /// * `cfg` - SAI configuration
    pub fn split(
        self,
        word_size: u8,
        frame_size: usize,
        packing: Packing,
        cfg: &SaiConfig,
    ) -> (Option<Tx>, Option<Rx>) {
        let tx_channel = self.tx_chan_mask.trailing_zeros() as usize;
        let rx_channel = self.rx_chan_mask.trailing_zeros() as usize;

        let tx = (self.tx_chan_mask != 0).then(|| Tx {
            // SAFETY: We're creating an alias to the same register block.
            // Tx and Rx operate on different parts of the register block.
            sai: unsafe { AnyInstance::new(&*self.sai) },
            word_size,
            frame_size,
            channel: tx_channel,
        });
        let rx = (self.rx_chan_mask != 0).then(|| Rx {
            // SAFETY: We're creating an alias to the same register block.
            // Tx and Rx operate on different parts of the register block.
            sai: unsafe { AnyInstance::new(&*self.sai) },
            word_size,
            frame_size,
            channel: rx_channel,
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
            SyncWidth::WordSize => word_size as u32,
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
            ral::write_reg!(ral::sai, self.sai, TCR4, FRSZ: ((frame_size - 1) as u32),
                FPACK: packing as u32, SYWD: (sync_width - 1), MF: cfg.byte_order as u32,
                FSE: cfg.sync_early as u32, FSP: cfg.sync_polarity as u32, FSD: frame_sync_dir);
            ral::write_reg!(ral::sai, self.sai, TCR5, W0W: ((word_size - 1) as u32), WNW: ((word_size - 1) as u32), FBT: (word_size - 1) as u32);
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
            ral::write_reg!(ral::sai, self.sai, RCR4, FRSZ: ((frame_size - 1) as u32),
                FPACK: packing as u32, SYWD: (sync_width - 1), MF: cfg.byte_order as u32,
                FSE: cfg.sync_early as u32, FSP: cfg.sync_polarity as u32, FSD: frame_sync_dir);
            ral::write_reg!(ral::sai, self.sai, RCR5, W0W: ((word_size - 1) as u32), WNW: ((word_size - 1) as u32), FBT: (word_size - 1) as u32);
            ral::write_reg!(ral::sai, self.sai, RCSR, RE: 0, STOPE: cfg.rx_stop_en as u32,
                DBGE: cfg.rx_debug_en as u32, BCE: 1, WSF: 1, SEF: 1, FEF: 1, FWF: 0, FRF: 0,
                WSIE: 0, SEIE: 0, FEIE: 0, FWIE: 0, FWDE: 0, FRDE: 0);
        }

        (tx, rx)
    }
}
