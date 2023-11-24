//! TODO

pub use eh1::spi::Mode;

use imxrt_dma::channel::Channel;

use crate::ral;
use cortex_m::interrupt::Mutex;

mod bus;
mod disabled;
mod eh1_impl;
mod error;
mod status_watcher;

use status_watcher::StatusWatcher;

/// TODO
pub enum LpspiDma {
    /// Everything is CPU driven
    Disable,
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
    // /// The transaction frame size is incorrect.
    // ///
    // /// The frame size, in bits, must be between 8 bits and
    // /// 4095 bits.
    // FrameSize,
    // /// FIFO error in the given direction.
    // Fifo(Direction),
    /// Bus is busy at the start of a transfer.
    Busy,
    // /// Caller provided no data.
    // NoData,
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

struct LpspiDataInner<const N: u8> {
    // TODO: interrupt stuff
}

/// Static shared data allocated by the user
pub struct LpspiData<const N: u8> {
    shared: Mutex<LpspiDataInner<N>>,
    lpspi: status_watcher::StatusWatcher<N>,
}

/// TODO
pub struct Lpspi<'a, const N: u8> {
    dma: LpspiDma,
    source_clock_hz: u32,
    data: &'a LpspiData<N>,
    rx_fifo_size: u32,
    tx_fifo_size: u32,
}

/// An LPSPI peripheral which is temporarily disabled.
pub struct Disabled<'a, 'b, const N: u8> {
    bus: &'a mut Lpspi<'b, N>,
    men: bool,
}

/// TODO
pub struct LpspiInterruptHandler {}
impl LpspiInterruptHandler {
    /// TODO
    pub fn on_interrupt(&mut self) {
        todo!()
    }
}
