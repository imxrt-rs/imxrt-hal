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

trait LpspiDma {
    fn get_one(&mut self) -> Option<&mut Channel>;
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)>;
}

/// Everything is CPU driven
struct NoDma;

/// Read and Write are DMA based,
/// but Transfers are only partially
/// DMA based
///
struct PartialDma(Channel);

/// Everything is DMA based.
///
/// This is a requirement for the async interface.
struct FullDma(Channel, Channel);

impl LpspiDma for NoDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        None
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        None
    }
}
impl LpspiDma for PartialDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        Some(&mut self.0)
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        None
    }
}
impl LpspiDma for FullDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        Some(&mut self.0)
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        Some((&mut self.0, &mut self.1))
    }
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
