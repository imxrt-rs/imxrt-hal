use core::num::NonZeroUsize;

use eh1::spi::{Phase, Polarity, MODE_0};
use futures::FutureExt;

use super::{
    transfer_actions::{ActionSequence, ByteOrder},
    Disabled, Lpspi, LpspiData, LpspiError, LpspiInterruptHandler, Pins, StatusWatcher,
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
        ral::write_reg!(ral::lpspi, this.lpspi(), FCR,
            RXWATER: this.rx_fifo_size/2 - 1, // Notify when we have at least rx_fifo_size/2 data available
            TXWATER: this.tx_fifo_size/2      // Nofify when we have at least tx_fifo_size/2 space available
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

    /// Returns errors, if any there are any.
    fn check_errors(&mut self) -> Result<(), LpspiError> {
        self.data.lpspi.check_for_errors()
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

    async fn wait_for_read_watermark(&mut self) {
        self.data.lpspi.wait_for_rx_watermark().await.unwrap();
    }

    async fn wait_for_write_watermark(&mut self) {
        self.data.lpspi.wait_for_tx_watermark().await.unwrap();
    }

    fn start_frame(
        &mut self,
        reverse_bytes: bool,
        is_first_frame: bool,
        is_last_frame: bool,
        enable_read: bool,
        enable_write: bool,
        frame_size_bytes: NonZeroUsize,
    ) {
        let num_bits = frame_size_bytes.get() as u32 * 8;
        assert!(num_bits <= MAX_FRAME_SIZE_BITS);

        ral::write_reg!(ral::lpspi, self.lpspi(), TCR,
            CPOL: if self.mode.polarity == Polarity::IdleHigh {CPOL_1} else {CPOL_0},
            CPHA: if self.mode.phase == Phase::CaptureOnSecondTransition {CPHA_1} else {CPHA_0},
            PRESCALE: PRESCALE_0,
            PCS: PCS_0,
            LSBF: LSBF_0,
            BYSW: if reverse_bytes {BYSW_0} else {BYSW_1},
            CONT: if is_last_frame {CONT_0} else {CONT_1},
            CONTC: if is_first_frame {CONTC_0} else {CONTC_1},
            RXMSK: if enable_read {RXMSK_0} else {RXMSK_1},
            TXMSK: if enable_write {TXMSK_0} else {TXMSK_1},
            WIDTH: WIDTH_0,
            FRAMESZ: num_bits - 1
        );
    }

    async unsafe fn write_single_word(
        &mut self,
        write_data: Option<*const u8>,
        byteorder: ByteOrder,
        read: bool,
        len: NonZeroUsize,
        is_first_frame: bool,
        is_last_frame: bool,
    ) {
        assert!(len.get() < 4);

        let reverse_bytes = match byteorder {
            ByteOrder::Normal => false,
            ByteOrder::WordReversed => true,
            ByteOrder::HalfWordReversed => true,
        };

        // This should make sure that at least two words are free to be written
        self.wait_for_write_watermark().await;

        self.start_frame(
            false,
            is_first_frame,
            is_last_frame,
            read,
            write_data.is_some(),
            len,
        );

        if let Some(data) = write_data {
            let mut tx_buffer = [0u8; 4];
            let active_buffer = &mut tx_buffer[(4 - len.get())..];
            if reverse_bytes {
                active_buffer
                    .iter_mut()
                    .rev()
                    .enumerate()
                    .for_each(|(pos, val)| *val = data.add(pos).read());
            } else {
                active_buffer
                    .iter_mut()
                    .enumerate()
                    .for_each(|(pos, val)| *val = data.add(pos).read());
            }

            ral::write_reg!(ral::lpspi, self.lpspi(), TDR, u32::from_le_bytes(tx_buffer));
        }
    }

    async unsafe fn write_u32_stream(
        &mut self,
        write_data: Option<*const u8>,
        byteorder: ByteOrder,
        read: bool,
        len: NonZeroUsize,
        is_first_frame: bool,
        is_last_frame: bool,
        // TODO: dma
    ) {
        assert!(len.get() % 4 == 0);
        let len = NonZeroUsize::new(len.get() / 4).unwrap();
        let write_data: Option<*const u32> = write_data.map(|p| p.cast());

        todo!();
    }

    /// Perform a sequence of transfer actions
    async fn transfer_unchecked(&mut self, sequence: ActionSequence<'_>) -> Result<(), LpspiError> {
        self.flush_unchecked().await?;

        self.clear_fifos();

        let read_task = async { assert!(!sequence.contains_read_actions()) };
        let write_task = async {
            unsafe {
                let has_phase_1 = sequence.phase1.is_some();
                let has_phase_2 = sequence.phase2.is_some();

                if let Some(phase1) = &sequence.phase1 {
                    for action in phase1.get_write_actions() {
                        if action.len.get() < 4 {
                            self.write_single_word(
                                action.buf,
                                sequence.byteorder,
                                action.read,
                                action.len,
                                action.is_first,
                                action.is_last && !has_phase_2,
                            )
                            .await
                        } else {
                            self.write_u32_stream(
                                action.buf,
                                sequence.byteorder,
                                action.read,
                                action.len,
                                action.is_first,
                                action.is_last && !has_phase_2,
                            )
                            .await;
                        }
                    }
                }
                if let Some(phase2) = &sequence.phase2 {
                    for action in phase2.get_write_actions() {
                        if action.len.get() < 4 {
                            self.write_single_word(
                                action.buf,
                                sequence.byteorder,
                                action.read,
                                action.len,
                                action.is_first && !has_phase_1,
                                action.is_last,
                            )
                            .await
                        } else {
                            self.write_u32_stream(
                                action.buf,
                                sequence.byteorder,
                                action.read,
                                action.len,
                                action.is_first && !has_phase_1,
                                action.is_last,
                            )
                            .await
                        }
                    }
                }
            }
        };

        futures::join!(read_task, write_task);

        self.check_errors()
    }

    /// Perform a sequence of transfer actions while continuously checking for errors.
    async fn transfer(&mut self, sequence: ActionSequence<'_>) -> Result<(), LpspiError> {
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
            self.driver.clear_fifos();
            self.driver.data.lpspi.clear_errors();
        }
    }
}
