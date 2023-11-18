use eh1::{delay::DelayUs, spi::MODE_0};
use imxrt_dma::channel::Channel;

use super::{
    Arbiter, LpspiBus, LpspiData, LpspiDataInner, LpspiDevice, LpspiDma, LpspiDriver, LpspiError,
    LpspiInterruptHandler, Pins,
};
use crate::{
    iomuxc::{consts, lpspi},
    ral,
};

impl<const N: u8> LpspiBus<N> {
    /// The peripheral instance.
    pub const N: u8 = N;

    /// TODO
    pub fn new<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        mut pins: Pins<SDO, SDI, SCK>,
        data_storage: &'static mut Option<LpspiData<N>>,
        clk_frequency: u32,
    ) -> Self
    where
        SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
        SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
        SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    {
        let driver = LpspiDriver {};

        lpspi::prepare(&mut pins.sdo);
        lpspi::prepare(&mut pins.sdi);
        lpspi::prepare(&mut pins.sck);

        let data = LpspiData {
            bus: Arbiter::new(LpspiDataInner {
                driver,
                lpspi,
                clk_frequency,
                dma: LpspiDma::Disable,
                timer: None,
            }),
        };

        Self {
            data: data_storage.insert(data),
            // Sane defaults
            mode: MODE_0,
            baud_rate: 1_000_000,
        }
    }

    /// TODO
    pub fn set_dma(&mut self, dma: LpspiDma) -> Result<LpspiDma, LpspiError> {
        let mut bus = self.data.bus.try_access().ok_or(LpspiError::Busy)?;
        Ok(core::mem::replace(&mut bus.dma, dma))
    }

    /// TODO
    pub fn set_delay_source(&mut self, delay: &'static mut dyn DelayUs) -> Result<(), LpspiError> {
        let mut bus = self.data.bus.try_access().ok_or(LpspiError::Busy)?;
        bus.timer = Some(delay);
        Ok(())
    }

    /// TODO
    pub fn enable_interrupts(&mut self) -> Result<LpspiInterruptHandler, LpspiError> {
        todo!()
    }

    /// TODO
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }

    /// TODO
    pub fn device<CS>(&self, cs: crate::gpio::Output<CS>) -> LpspiDevice<N, CS> {
        LpspiDevice {
            data: self.data,
            cs,
        }
    }
}
