use eh1::spi::MODE_0;

use super::{
    data_buffer::{LpspiDataBuffer, LpspiIndexChunks},
    dma::{FullDma, NoDma, PartialDma},
    Channel, Disabled, Lpspi, LpspiData, LpspiError, LpspiInterruptHandler, Pins, StatusWatcher,
};
use crate::{
    iomuxc::{consts, lpspi},
    ral,
};

mod eh1_impl;

const MAX_FRAME_SIZE_BIT: u32 = 1 << 12;

impl<'a, const N: u8> Lpspi<'a, N, NoDma> {
    /// Create a new LPSPI peripheral without DMA support.
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn new<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        pins: Pins<SDO, SDI, SCK>,
        data_storage: &'a mut Option<LpspiData<N>>,
        source_clock_hz: u32,
    ) -> Self
    where
        SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
        SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
        SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    {
        Self::create(lpspi, pins, data_storage, source_clock_hz, NoDma)
    }
}

impl<'a, const N: u8> Lpspi<'a, N, PartialDma> {
    /// Create a new LPSPI peripheral with partial DMA support.
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn new<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        pins: Pins<SDO, SDI, SCK>,
        data_storage: &'a mut Option<LpspiData<N>>,
        source_clock_hz: u32,
        dma: Channel,
    ) -> Self
    where
        SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
        SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
        SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    {
        Self::create(lpspi, pins, data_storage, source_clock_hz, PartialDma(dma))
    }
}

impl<'a, const N: u8> Lpspi<'a, N, FullDma> {
    /// Create a new LPSPI peripheral with full DMA support.
    ///
    /// This is required for `async` operation.
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    pub fn new<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        pins: Pins<SDO, SDI, SCK>,
        data_storage: &'a mut Option<LpspiData<N>>,
        source_clock_hz: u32,
        dma1: Channel,
        dma2: Channel,
    ) -> Self
    where
        SDO: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdo>,
        SDI: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sdi>,
        SCK: lpspi::Pin<Module = consts::Const<N>, Signal = lpspi::Sck>,
    {
        Self::create(
            lpspi,
            pins,
            data_storage,
            source_clock_hz,
            FullDma(dma1, dma2),
        )
    }
}

impl<'a, const N: u8, DMA> Lpspi<'a, N, DMA> {
    /// The peripheral instance.
    pub const N: u8 = N;

    /// Create a new LPSPI peripheral.
    ///
    /// `source_clock_hz` is the LPSPI peripheral clock speed. To specify the
    /// peripheral clock, see the [`ccm::lpspi_clk`](crate::ccm::lpspi_clk) documentation.
    fn create<SDO, SDI, SCK>(
        lpspi: ral::lpspi::Instance<N>,
        // TODO: Open question: How to make those pins optional? (For example, WS2812 driver only uses SDO pin)
        //       Or should we simply do a `new_without_pins` again?
        mut pins: Pins<SDO, SDI, SCK>,
        data_storage: &'a mut Option<LpspiData<N>>,
        source_clock_hz: u32,
        dma: DMA,
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
        };

        let mut this = Self {
            source_clock_hz,
            dma,
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

        // Configure watermarks.
        // This is more for good measure, we don't really use the watermarks.
        // ral::write_reg!(ral::lpspi, this.lpspi(), FCR,
        //     RXWATER: 0,
        //     TXWATER: u32::MAX
        // );

        // Enable
        ral::write_reg!(ral::lpspi, this.lpspi(), CR, MEN: MEN_1);

        this
    }

    /// Temporarily disable the LPSPI peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpspi::Disabled) driver lets you modify
    /// LPSPI settings that require a fully disabled peripheral.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N, DMA>) -> R) -> R {
        // Disable DMA and clear fifos
        ral::modify_reg!(ral::lpspi, self.lpspi(), DER, RDDE: RDDE_0, TDDE: TDDE_0);
        self.clear_fifos();

        let mut disabled = Disabled::new(self);
        func(&mut disabled)
    }

    /// Switches the SPI bus to interrupt based operation.
    ///
    /// This increases the efficiency of the `async` interface, as it avoids busy waiting.
    ///
    /// Note that it is the caller's responsibility to connect the interrupt source
    /// to the returned interrupt handler object.
    pub fn enable_interrupts(&mut self) -> LpspiInterruptHandler<'a, N> {
        LpspiInterruptHandler {
            status_watcher: &self.data.lpspi,
        }
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

    /// Returns whether or not the busy flag is set.
    fn busy(&self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), SR, MBF == MBF_1)
    }

    /// Returns errors, if any there are any.
    fn check_errors(&mut self) -> Result<(), LpspiError> {
        let (rx_error, tx_error) = ral::read_reg!(ral::lpspi, self.lpspi(), SR, REF, TEF);

        if tx_error != 0 || rx_error != 0 {
            ral::write_reg!(ral::lpspi, self.lpspi(), SR, REF: rx_error, TEF: tx_error);
            self.clear_fifos();

            if tx_error != 0 {
                Err(LpspiError::TransmitFifo)
            } else {
                Err(LpspiError::ReceiveFifo)
            }
        } else {
            Ok(())
        }
    }

    /// Prepares the device for a blocking transfer
    fn blocking_pre_transfer(&mut self) -> Result<(), LpspiError> {
        if self.busy() {
            return Err(LpspiError::Busy);
        }

        // Disable DMA
        ral::modify_reg!(ral::lpspi, self.lpspi(), DER, RDDE: RDDE_0, TDDE: TDDE_0);
        self.clear_fifos();

        self.data.lpspi.clear_transfer_complete();

        Ok(())
    }

    fn fifo_read_data_available(&mut self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), RSR, RXEMPTY == RXEMPTY_0)
    }

    fn fifo_write_space_available(&mut self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), FSR, TXCOUNT < self.tx_fifo_size)
    }

    /// Writes to the bus
    fn blocking_transfer<T>(
        &mut self,
        tx_buffer: &[T],
        rx_buffer: &mut [T],
    ) -> Result<(), LpspiError>
    where
        [T]: LpspiDataBuffer,
    {
        self.blocking_pre_transfer()?;

        let size = tx_buffer.bytecount().max(rx_buffer.bytecount());
        if size < 1 {
            return Err(LpspiError::NoData);
        }

        let mut rx_offset = 0;
        let mut do_receive = |this: &mut Self| -> Result<(), LpspiError> {
            while this.fifo_read_data_available() {
                this.check_errors()?;
                let rx_data = ral::read_reg!(ral::lpspi, this.lpspi(), RDR);
                rx_buffer.write(rx_offset, rx_data);
                rx_offset += 1;
            }
            Ok(())
        };

        for chunk in LpspiIndexChunks::new(size, MAX_FRAME_SIZE_BIT / 8) {
            self.check_errors()?;
            ral::write_reg!(ral::lpspi, self.lpspi(), TCR,
                RXMSK: RXMSK_0,
                TXMSK: TXMSK_1,
                FRAMESZ: chunk.bytecount() * 8
            );

            for tx_offset in chunk.offsets() {
                while !self.fifo_write_space_available() {
                    do_receive(self)?;
                    self.check_errors()?;
                }
                ral::write_reg!(ral::lpspi, self.lpspi(), TDR, tx_buffer.read(tx_offset));
            }
        }

        while !self.data.lpspi.poll_transfer_complete() {
            do_receive(self)?;
        }
        do_receive(self)?;

        self.check_errors()
    }
}
