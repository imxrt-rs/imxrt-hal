//! TODO

pub use eh1::spi::Mode;

use imxrt_dma::channel::Channel;

use crate::ral;

mod bus;
mod disabled;
mod dma;
mod error;
mod status_watcher;
mod word_types;

use status_watcher::StatusWatcher;

/// Possible errors when interfacing the LPSPI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpspiError {
    /// An error occurred in the receive fifo.
    ReceiveFifo,
    /// An error occurred in the transmit fifo.
    TransmitFifo,
    /// Bus is busy at the start of a transfer.
    Busy,
    /// Caller provided no data.
    NoData,
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
pub struct Lpspi<'a, const N: u8, DMA> {
    dma: DMA,
    source_clock_hz: u32,
    data: &'a LpspiData<N>,
    rx_fifo_size: u32,
    tx_fifo_size: u32,
}

/// An LPSPI peripheral which is temporarily disabled.
pub struct Disabled<'a, 'b, const N: u8, DMA> {
    bus: &'a mut Lpspi<'b, N, DMA>,
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
