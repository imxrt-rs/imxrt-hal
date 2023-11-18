//! TODO

use eh1::delay::DelayUs;
pub use eh1::spi::Mode;

use imxrt_dma::channel::Channel;
use rtic_sync::arbiter::Arbiter;

use crate::{gpio, ral};

mod bus;
mod device;

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

/// The internal driver implementation
struct LpspiDriver<const N: u8> {}

struct LpspiDataInner<const N: u8> {
    driver: LpspiDriver<N>,
    dma: LpspiDma,
    clk_frequency: u32,
    timer: Option<&'static mut dyn DelayUs>,
    lpspi: ral::lpspi::Instance<N>,
}

/// Static shared data allocated by the user
pub struct LpspiData<const N: u8> {
    bus: Arbiter<LpspiDataInner<N>>,
    // TODO: interrupt register struct
}

/// TODO
pub struct LpspiBus<const N: u8> {
    data: &'static LpspiData<N>,
    mode: Mode,
    baud_rate: u32,
}

/// TODO
pub struct LpspiDevice<const N: u8, CS> {
    data: &'static LpspiData<N>,
    cs: gpio::Output<CS>,
}

/// TODO
pub struct LpspiInterruptHandler {}
impl LpspiInterruptHandler {
    /// TODO
    pub fn on_interrupt(&mut self) {
        todo!()
    }
}
