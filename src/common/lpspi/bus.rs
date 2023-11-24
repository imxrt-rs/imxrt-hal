use cortex_m::interrupt::Mutex;
use eh1::spi::{Mode, MODE_0};

use super::{
    Disabled, Lpspi, LpspiData, LpspiDataInner, LpspiDma, LpspiError, LpspiInterruptHandler, Pins,
    StatusWatcher,
};
use crate::{
    iomuxc::{consts, lpspi},
    ral,
};

impl<'a, const N: u8> Lpspi<'a, N> {
    /// The peripheral instance.
    pub const N: u8 = N;

    /// Create a new LPSPI peripheral.
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn new<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        // TODO: Open question: How to make those pins optional? (For example, WS2812 driver only uses SDO pin)
        //       Or should we simply do a `new_without_pins` again?
        mut pins: Pins<SDO, SDI, SCK>,
        data_storage: &'a mut Option<LpspiData<N>>,
        source_clock_hz: u32,
    ) -> Self
    where
        SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
        SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
        SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    {
        let (rx_fifo_size_exp, tx_fifo_size_exp) =
            ral::read_reg!(ral::lpspi, lpspi, PARAM, RXFIFO, TXFIFO);
        let rx_fifo_size = 1 << rx_fifo_size_exp;
        let tx_fifo_size = 1 << tx_fifo_size_exp;

        let data = LpspiData {
            lpspi: StatusWatcher::new(lpspi),
            shared: Mutex::new(LpspiDataInner {}),
        };

        let mut this = Self {
            source_clock_hz,
            dma: LpspiDma::Disable,
            data: data_storage.insert(data),
            rx_fifo_size,
            tx_fifo_size,
        };

        // Reset, enable master mode
        ral::write_reg!(ral::lpspi, this.lpspi(), CR, RST: RST_1);
        ral::write_reg!(ral::lpspi, this.lpspi(), CR, RST: RST_0);
        ral::write_reg!(
            ral::lpspi,
            this.data.lpspi.instance(),
            CFGR1,
            MASTER: MASTER_1,
            SAMPLE: SAMPLE_1
        );

        // Set sane default parameters
        this.disabled(|bus| {
            bus.set_clock_hz(1_000_000);
            bus.set_mode(MODE_0)
        });

        // Configure pins
        lpspi::prepare(&mut pins.sdo);
        lpspi::prepare(&mut pins.sdi);
        lpspi::prepare(&mut pins.sck);

        // TODO think about this
        ral::write_reg!(ral::lpspi, this.lpspi(), FCR,
            RXWATER: this.rx_fifo_size / 2 - 1, // always divisible by two
            TXWATER: this.tx_fifo_size / 2 - 1
        );

        // Enable
        ral::write_reg!(ral::lpspi, this.lpspi(), CR, MEN: MEN_1);

        this
    }

    /// Temporarily disable the LPSPI peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpspi::Disabled) driver lets you modify
    /// LPSPI settings that require a fully disabled peripheral. This will clear the transmit
    /// and receive FIFOs.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        self.clear_fifos();
        let mut disabled = Disabled::new(self);
        func(&mut disabled)
    }

    /// Provides the SPI bus with one or two DMA channels.
    ///
    /// This drastically increases the efficiency of u32 based reads/writes.
    ///
    /// For simultaneous read/write, two DMA channels are required.
    pub fn set_dma(&mut self, dma: LpspiDma) -> LpspiDma {
        core::mem::replace(&mut self.dma, dma)
    }

    /// Switches the SPI bus to interrupt based operation.
    ///
    /// This increases efficiency drastically, as it avoids busy waiting.
    ///
    /// Note that it is the caller's responsibility to connect the interrupt source
    /// to the returned interrupt handler object.
    pub fn enable_interrupts(&mut self) -> Result<LpspiInterruptHandler, LpspiError> {
        todo!()
    }

    /// TODO
    pub fn set_spi_clock_hz(&mut self, _clk_hz: u32) {
        todo!()
    }

    /// Set the SPI mode for the peripheral
    pub fn set_mode(&mut self, mode: Mode) {
        todo!()
    }

    // ////////////////// PRIVATE DRIVER STUFF ///////////////////////

    /// Get LPSPI Register Instance
    #[inline]
    fn lpspi(&self) -> &ral::lpspi::Instance<N> {
        self.data.lpspi.instance()
    }

    /// Clear both FIFOs.
    fn clear_fifos(&mut self) {
        ral::modify_reg!(ral::lpspi, self.lpspi(), CR, RTF: RTF_1, RRF: RRF_1);
    }
}
