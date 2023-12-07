use eh1::spi::MODE_0;

use super::{
    transfer_actions::ActionSequence, Disabled, Lpspi, LpspiData, LpspiError,
    LpspiInterruptHandler, Pins, StatusWatcher,
};
use crate::{
    iomuxc::{consts, lpspi},
    lpspi::LpspiDma,
    ral,
};

mod eh1_impl;

const MAX_FRAME_SIZE_BITS: u32 = 1 << 12;

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

        let mut this = Self {
            source_clock_hz,
            dma: LpspiDma::Disabled,
            data: data_storage.insert(data),
            tx_fifo_size,
            rx_fifo_size,
            mode: MODE_0,
        };

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

    fn configure_dma(&mut self, dma_read: bool, dma_write: bool) {
        // Configure DMA
        ral::modify_reg!(ral::lpspi, self.lpspi(), DER,
            RDDE: if dma_read {RDDE_1} else {RDDE_0},
            TDDE: if dma_write {TDDE_1} else {TDDE_0}
        );
    }

    fn fifo_read_data_available(&mut self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), RSR, RXEMPTY == RXEMPTY_0)
    }

    fn fifo_write_space_available(&mut self) -> bool {
        ral::read_reg!(ral::lpspi, self.lpspi(), FSR, TXCOUNT < self.tx_fifo_size)
    }

    async fn wait_for_transfer_complete(&mut self) {
        self.data.lpspi.wait_transfer_complete().await;
    }

    // async fn transfer_single_word(
    //     &mut self,
    //     mut buffer: ,
    // ) -> Result<(), LpspiError> {
    //     assert!(buffer.max_len() < 4);
    //     assert!(buffer.max_len() >= 1);

    //     ral::write_reg!(ral::lpspi, self.lpspi(), TCR,
    //         RXMSK: RXMSK_0,
    //         TXMSK: TXMSK_0,
    //         PRESCALE: PRESCALE_7,
    //         FRAMESZ: buffer.max_len() as u32 * 8 - 1
    //     );

    //     let tx_buffer = buffer.tx_buffer();
    //     let tx_data = u32::from_le_bytes([
    //         tx_buffer.get(0).copied().unwrap_or_default(),
    //         tx_buffer.get(1).copied().unwrap_or_default(),
    //         tx_buffer.get(2).copied().unwrap_or_default(),
    //         tx_buffer.get(3).copied().unwrap_or_default(),
    //     ]);
    //     ral::write_reg!(ral::lpspi, self.lpspi(), TDR, tx_data);

    //     self.check_errors()?;
    //     self.wait_for_transfer_complete().await;

    //     if !self.fifo_read_data_available() {
    //         return Err(LpspiError::ReceiveFifo);
    //     }

    //     let rx_data = ral::read_reg!(ral::lpspi, self.lpspi(), RDR);
    //     let [r0, r1, r2, r3] = rx_data.to_le_bytes();
    //     let rx_buffer = buffer.rx_buffer();
    //     rx_buffer.get_mut(0).map(|x| *x = r0);
    //     rx_buffer.get_mut(1).map(|x| *x = r1);
    //     rx_buffer.get_mut(2).map(|x| *x = r2);
    //     rx_buffer.get_mut(3).map(|x| *x = r3);

    //     self.check_errors()
    // }

    /// Read + write into separate buffers
    async fn transfer(&mut self, mut buffers: ActionSequence<'_>) -> Result<(), LpspiError> {
        if self.busy() {
            return Err(LpspiError::Busy);
        }

        self.clear_fifos();
        self.check_errors()?;

        let read_task = async {};

        let write_task = async {};

        Ok(())
    }

    async fn flush(&mut self) -> Result<(), LpspiError> {
        self.data.lpspi.clear_transfer_complete();
        while ral::read_reg!(ral::lpspi, self.lpspi(), SR, MBF == MBF_1) {
            self.data.lpspi.wait_transfer_complete().await;
            self.data.lpspi.clear_transfer_complete();
        }
        self.check_errors()
    }
}
