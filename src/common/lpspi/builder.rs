use eh1::spi::MODE_0;
use imxrt_dma::channel::Channel;

use super::{
    Arbiter, LpspiBuilder, LpspiBus, LpspiData, LpspiDataInner, LpspiDriver, LpspiInterruptHandler,
    Pins,
};
use crate::ral;

impl<SDO, SDI, SCK, const N: u8> LpspiBuilder<Pins<SDO, SDI, SCK>, N, (), false> {
    pub fn new(
        lpspi: ral::lpspi::Instance<N>,
        pins: Pins<SDO, SDI, SCK>,
        data: &'static mut Option<LpspiData<N>>,
    ) -> Self {
        Self {
            data,
            pins,
            lpspi,
            dma: (),
        }
    }
}

impl<P, const N: u8, const INT: bool> LpspiBuilder<P, N, (), INT> {
    pub fn with_dma(self, dma: Channel) -> LpspiBuilder<P, N, Channel, INT> {
        let Self {
            pins,
            lpspi,
            data,
            dma: (),
        } = self;
        LpspiBuilder {
            dma,
            data,
            pins,
            lpspi,
        }
    }
}

impl<P, const N: u8, D> LpspiBuilder<P, N, D, false> {
    pub fn with_interrupts(self) -> LpspiBuilder<P, N, D, true> {
        let Self {
            pins,
            lpspi,
            data,
            dma,
        } = self;
        LpspiBuilder {
            dma,
            data,
            pins,
            lpspi,
        }
    }
}

fn build_bus<SDO, SDI, SCK, const N: u8>(
    pins: Pins<SDO, SDI, SCK>,
    data_storage: &'static mut Option<LpspiData<N>>,
    dma: Option<Channel>,
) -> LpspiBus<N> {
    let driver = LpspiDriver {};

    let data = LpspiData {
        inner: Arbiter::new(LpspiDataInner { driver, dma }),
    };

    LpspiBus {
        data: data_storage.insert(data),
        mode: MODE_0,
    }
}

impl<SDO, SDI, SCK, const N: u8> LpspiBuilder<Pins<SDO, SDI, SCK>, N, Channel, false> {
    pub fn build(self) -> LpspiBus<N> {
        build_bus(self.pins, self.data, Some(self.dma))
    }
}
impl<SDO, SDI, SCK, const N: u8> LpspiBuilder<Pins<SDO, SDI, SCK>, N, (), false> {
    pub fn build(self) -> LpspiBus<N> {
        build_bus(self.pins, self.data, None)
    }
}

impl<SDO, SDI, SCK, const N: u8> LpspiBuilder<Pins<SDO, SDI, SCK>, N, Channel, true> {
    pub fn build(self) -> (LpspiBus<N>, LpspiInterruptHandler<N>) {
        let bus = build_bus(self.pins, self.data, Some(self.dma));
        let interrupt_handler = LpspiInterruptHandler { data: bus.data };
        (bus, interrupt_handler)
    }
}
impl<SDO, SDI, SCK, const N: u8> LpspiBuilder<Pins<SDO, SDI, SCK>, N, (), true> {
    pub fn build(self) -> (LpspiBus<N>, LpspiInterruptHandler<N>) {
        let bus = build_bus(self.pins, self.data, None);
        let interrupt_handler = LpspiInterruptHandler { data: bus.data };
        (bus, interrupt_handler)
    }
}
