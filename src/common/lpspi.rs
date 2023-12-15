//! TODO

pub use eh1::spi::Mode;
use imxrt_dma::channel::Channel;

use crate::ral;

mod bus;
mod disabled;
mod error;
mod read_part;
mod status_watcher;
mod transfer_actions;
mod write_part;

use status_watcher::StatusWatcher;

const MAX_FRAME_SIZE_BITS: u32 = 1 << 12;
const MAX_FRAME_SIZE_BYTES: u32 = MAX_FRAME_SIZE_BITS / 8;
const MAX_FRAME_SIZE_U32: u32 = MAX_FRAME_SIZE_BYTES / 4;

/// TODO
pub enum LpspiDma {
    /// Everything is CPU driven
    Disabled,
    /// Read and Write are DMA based,
    /// but Transfers are only partially
    /// DMA based
    Partial(Channel),
    /// Everything is DMA based
    Full(Channel, Channel),
}

/// Possible errors when interfacing the LPSPI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpspiError {
    /// An error occurred in the receive fifo.
    ReceiveFifo,
    /// An error occurred in the transmit fifo.
    TransmitFifo,
    /// Bus is busy at the start of a transfer.
    Busy,
}

/// TODO
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

/// Static shared data allocated by the user
pub struct LpspiData<const N: u8> {
    lpspi: StatusWatcher<N>,
}

struct LpspiReadPart<'a, const N: u8> {
    data: &'a LpspiData<N>,
    rx_fifo_size: u32,
}

struct LpspiWritePart<'a, const N: u8> {
    data: &'a LpspiData<N>,
    tx_fifo_size: u32,
    mode: Mode,
}

/// TODO
pub struct Lpspi<'a, const N: u8> {
    dma: LpspiDma,
    source_clock_hz: u32,
    data: &'a LpspiData<N>,
    read_part: LpspiReadPart<'a, N>,
    write_part: LpspiWritePart<'a, N>,
}

/// An LPSPI peripheral which is temporarily disabled.
pub struct Disabled<'a, 'b, const N: u8> {
    bus: &'a mut Lpspi<'b, N>,
    men: bool,
}

/// TODO
pub struct LpspiInterruptHandler<'a, const N: u8> {
    status_watcher: &'a StatusWatcher<N>,
}
impl<const N: u8> LpspiInterruptHandler<'_, N> {
    /// TODO
    pub fn on_interrupt(&mut self) {
        self.status_watcher.on_interrupt();
    }
}

/// A data word for LPSPI
pub trait LpspiWord: transfer_actions::BufferType {}
impl LpspiWord for u8 {}
impl LpspiWord for u16 {}
impl LpspiWord for u32 {}
