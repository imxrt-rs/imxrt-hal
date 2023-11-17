pub use eh1::spi::Mode;
use imxrt_dma::channel::Channel;
use rtic_sync::arbiter::Arbiter;

use crate::ral;

mod builder;
mod driver;

struct LpspiBuilder<P, const N: u8, D, const INT: bool> {
    data: &'static mut Option<LpspiData<N>>,
    dma: D,
    pins: P,
    lpspi: ral::lpspi::Instance<N>,
}

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
    dma: Option<Channel>,
}

/// Static shared data allocated by the user
struct LpspiData<const N: u8> {
    inner: Arbiter<LpspiDataInner<N>>,
}

struct LpspiBus<const N: u8> {
    data: &'static LpspiData<N>,
    mode: Mode,
}

struct LpspiInterruptHandler<const N: u8> {
    data: &'static LpspiData<N>,
}

impl<const N: u8> LpspiBus<N> {
    pub fn device(cs: ()) -> LpspiDevice {
        todo!()
    }
}

struct LpspiDevice {}
