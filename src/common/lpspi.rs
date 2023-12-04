//! TODO

pub use eh1::spi::Mode;
use imxrt_dma::channel::Channel;

use crate::ral;

mod bus;
mod data_buffer;
mod disabled;
mod error;
mod status_watcher;

use status_watcher::StatusWatcher;

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

/// TODO
pub struct Lpspi<'a, const N: u8> {
    dma: LpspiDma,
    source_clock_hz: u32,
    data: &'a LpspiData<N>,
    tx_fifo_size: u32,
    rx_fifo_size: u32,
    mode: Mode,
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
