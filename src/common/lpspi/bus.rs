use eh1::spi::MODE_0;
use futures::FutureExt;

use super::{
    transfer_actions::ActionSequence, Disabled, Lpspi, LpspiData, LpspiDma, LpspiError,
    LpspiInterruptHandler, LpspiReadPart, LpspiWritePart, Pins, StatusWatcher,
};
use crate::{
    iomuxc::{consts, lpspi},
    lpspi::transfer_actions::TransferDirection,
    ral,
};

mod eh1_impl;

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
        let (tx_fifo_size_exp, rx_fifo_size_exp) =
            ral::read_reg!(ral::lpspi, lpspi, PARAM, TXFIFO, RXFIFO);
        let tx_fifo_size = 1 << tx_fifo_size_exp;
        let rx_fifo_size = 1 << rx_fifo_size_exp;

        let data = LpspiData {
            lpspi: StatusWatcher::new(lpspi),
        };

        let data = data_storage.insert(data);

        let mut this = Self {
            source_clock_hz,
            dma: LpspiDma::Disabled,
            data: data,
            read_part: LpspiReadPart { data, rx_fifo_size },
            write_part: LpspiWritePart {
                data,
                tx_fifo_size,
                mode: MODE_0,
            },
        };

        log::info!("Fifo sizes: {tx_fifo_size}(TX) {rx_fifo_size}(RX)");

        // Reset and disable
        ral::modify_reg!(ral::lpspi, this.lpspi(), CR, MEN: MEN_0, RST: RST_1);
        while ral::read_reg!(ral::lpspi, this.lpspi(), CR, MEN == MEN_1) {}
        ral::modify_reg!(ral::lpspi, this.lpspi(), CR, RST: RST_0, RTF: RTF_1, RRF: RRF_1);

        // Configure master mode
        ral::write_reg!(
            ral::lpspi,
            this.lpspi(),
            CFGR1,
            MASTER: MASTER_1,
            SAMPLE: SAMPLE_1
        );

        // Set sane default parameters
        this.disabled(|bus| {
            bus.set_clock_hz(1_000_000);
        });

        // Configure pins
        lpspi::prepare(&mut pins.sdo);
        lpspi::prepare(&mut pins.sdi);
        lpspi::prepare(&mut pins.sck);

        // Configure watermarks
        ral::write_reg!(ral::lpspi, this.lpspi(), FCR,
            RXWATER: 0,             // Notify when we have any data available
            TXWATER: tx_fifo_size/2 // Nofify when we have at least tx_fifo_size/2 space available
        );

        // Enable
        ral::write_reg!(ral::lpspi, this.lpspi(), CR, MEN: MEN_1);

        this
    }

    /// Temporarily disable the LPSPI peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpspi::Disabled) driver lets you modify
    /// LPSPI settings that require a fully disabled peripheral.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
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
        self.data.lpspi.enable_interrupts();
        LpspiInterruptHandler {
            status_watcher: &self.data.lpspi,
        }
    }

    /// Provides the SPI bus with one or two DMA channels.
    ///
    /// This drastically increases the efficiency of reads/writes.
    ///
    /// For simultaneous read/write, two DMA channels are required.
    pub fn set_dma(&mut self, dma: LpspiDma) -> LpspiDma {
        core::mem::replace(&mut self.dma, dma)
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

    /// Stops the current transfer without changing its current configuration.
    fn reset(&mut self) {
        // Restore old registers
        cortex_m::interrupt::free(|_| {
            // Backup
            let ier = ral::read_reg!(ral::lpspi, self.lpspi(), IER);
            let der = ral::read_reg!(ral::lpspi, self.lpspi(), DER);
            let cfgr0 = ral::read_reg!(ral::lpspi, self.lpspi(), CFGR0);
            let cfgr1 = ral::read_reg!(ral::lpspi, self.lpspi(), CFGR1);
            let dmr0 = ral::read_reg!(ral::lpspi, self.lpspi(), DMR0);
            let dmr1 = ral::read_reg!(ral::lpspi, self.lpspi(), DMR1);
            let ccr = ral::read_reg!(ral::lpspi, self.lpspi(), CCR);
            let fcr = ral::read_reg!(ral::lpspi, self.lpspi(), FCR);

            // Reset and disable
            ral::modify_reg!(ral::lpspi, self.lpspi(), CR, MEN: MEN_0, RST: RST_1);
            while ral::read_reg!(ral::lpspi, self.lpspi(), CR, MEN == MEN_1) {}
            ral::modify_reg!(ral::lpspi, self.lpspi(), CR, RST: RST_0, RTF: RTF_1, RRF: RRF_1);

            // Reset fifos
            ral::modify_reg!(ral::lpspi, self.lpspi(), CR, RTF: RTF_1, RRF: RRF_1);

            // Resore settings
            ral::write_reg!(ral::lpspi, self.lpspi(), IER, ier);
            ral::write_reg!(ral::lpspi, self.lpspi(), DER, der);
            ral::write_reg!(ral::lpspi, self.lpspi(), CFGR0, cfgr0);
            ral::write_reg!(ral::lpspi, self.lpspi(), CFGR1, cfgr1);
            ral::write_reg!(ral::lpspi, self.lpspi(), DMR0, dmr0);
            ral::write_reg!(ral::lpspi, self.lpspi(), DMR1, dmr1);
            ral::write_reg!(ral::lpspi, self.lpspi(), CCR, ccr);
            ral::write_reg!(ral::lpspi, self.lpspi(), FCR, fcr);

            // Enable
            ral::write_reg!(ral::lpspi, self.lpspi(), CR, MEN: MEN_1);
        });
    }

    /// Returns errors, if any there are any.
    fn check_errors(&self) -> Result<(), LpspiError> {
        self.data.lpspi.check_for_errors()
    }

    fn configure_dma(&self, dma_read: bool, dma_write: bool) {
        // Configure DMA
        ral::modify_reg!(ral::lpspi, self.lpspi(), DER,
            RDDE: if dma_read {RDDE_1} else {RDDE_0},
            TDDE: if dma_write {TDDE_1} else {TDDE_0}
        );
    }

    /// Perform a sequence of transfer actions
    async unsafe fn transfer_unchecked(
        &mut self,
        sequence: ActionSequence<'_>,
    ) -> Result<(), LpspiError> {
        self.flush_unchecked().await?;

        self.clear_fifos();

        let byteorder = sequence.byteorder;

        let read_part = &mut self.read_part;
        let write_part = &mut self.write_part;

        let (mut write_dma, mut read_dma) = match &mut self.dma {
            LpspiDma::Disabled => (None, None),
            LpspiDma::Partial(dma) => match sequence.recommended_dma_direction() {
                TransferDirection::Read => (None, Some(dma)),
                TransferDirection::Write => (Some(dma), None),
            },
            LpspiDma::Full(dma1, dma2) => (Some(dma1), Some(dma2)),
        };

        let read_task = async {
            if let Some(phase1) = &sequence.phase1 {
                let actions = phase1.get_read_actions();
                read_part.perform_read_actions(actions, byteorder).await;
            }
            if let Some(phase2) = &sequence.phase2 {
                if let Some(actions) = phase2.get_read_actions() {
                    read_part.perform_read_actions(actions, byteorder).await;
                }
            }
        };
        let write_task = async {
            let has_phase_1 = sequence.phase1.is_some();
            let has_phase_2 = sequence.phase2.is_some();

            if let Some(phase1) = &sequence.phase1 {
                let actions = phase1.get_write_actions();
                write_part
                    .perform_write_actions(
                        actions,
                        false,
                        has_phase_2,
                        byteorder,
                        write_dma.as_deref_mut(),
                    )
                    .await;
            }
            if let Some(phase2) = &sequence.phase2 {
                let actions = phase2.get_write_actions();
                write_part
                    .perform_write_actions(
                        actions,
                        has_phase_1,
                        false,
                        byteorder,
                        write_dma.as_deref_mut(),
                    )
                    .await;
            }
        };

        futures::join!(read_task, write_task);

        self.check_errors()
    }

    /// Perform a sequence of transfer actions while continuously checking for errors.
    async unsafe fn transfer(&mut self, sequence: ActionSequence<'_>) -> Result<(), LpspiError> {
        let mut cleanup_on_error = CleanupOnError::new(self);
        let this = cleanup_on_error.driver();

        let data = this.data;

        let result: Result<(), LpspiError> = futures::select_biased! {
            res = data.lpspi.watch_for_errors().fuse() => res,
            res = this.transfer_unchecked(sequence).fuse() => res,
        };

        cleanup_on_error.finish(result)
    }

    /// Returns whether or not the busy flag is set.
    fn busy(&self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), SR, MBF == MBF_1)
    }

    /// Waits for the device to become idle.
    async fn flush_unchecked(&mut self) -> Result<(), LpspiError> {
        self.data.lpspi.clear_transfer_complete();
        while self.busy() {
            self.check_errors()?;
            self.data.lpspi.wait_transfer_complete().await;
            self.data.lpspi.clear_transfer_complete();
        }
        self.check_errors()
    }

    /// Waits for the device to become idle while continuously checking for errors.
    async fn flush(&mut self) -> Result<(), LpspiError> {
        let mut cleanup_on_error = CleanupOnError::new(self);
        let this = cleanup_on_error.driver();

        let data = this.data;

        let result = futures::select_biased! {
            res = data.lpspi.watch_for_errors().fuse() => res,
            res = this.flush_unchecked().fuse() => res,
        };

        cleanup_on_error.finish(result)
    }
}

struct CleanupOnError<'a, 'b, const N: u8> {
    defused: bool,
    driver: &'a mut Lpspi<'b, N>,
}

impl<'a, 'b, const N: u8> CleanupOnError<'a, 'b, N> {
    pub fn new(driver: &'a mut Lpspi<'b, N>) -> Self {
        Self {
            defused: false,
            driver,
        }
    }
    pub fn finish(mut self, result: Result<(), LpspiError>) -> Result<(), LpspiError> {
        let result = result.and_then(|()| self.driver.check_errors());
        if result.is_ok() {
            self.defused = true;
        }
        result
    }
    pub fn driver(&mut self) -> &mut Lpspi<'b, N> {
        &mut self.driver
    }
}

impl<'a, 'b, const N: u8> Drop for CleanupOnError<'a, 'b, N> {
    fn drop(&mut self) {
        if !self.defused {
            log::warn!("An LPSPI error happened! Cleaning up ...");
            self.driver.reset();
            self.driver.data.lpspi.clear_errors();
        }
    }
}
