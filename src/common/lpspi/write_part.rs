use core::num::NonZeroUsize;

use eh1::spi::{Phase, Polarity};

use super::{
    ral,
    transfer_actions::{ByteOrder, ChunkIter, WriteAction},
    LpspiWritePart, MAX_FRAME_SIZE_BITS, MAX_FRAME_SIZE_U32,
};

impl<const N: u8> LpspiWritePart<'_, N> {
    fn fifo_write_space_available(&self) -> bool {
        ral::read_reg!(
            ral::lpspi,
            self.data.lpspi.instance(),
            FSR,
            TXCOUNT < self.tx_fifo_size
        )
    }

    async fn wait_for_write_watermark(&self) {
        self.data.lpspi.wait_for_tx_watermark().await.unwrap();
    }

    async fn wait_for_write_space_available(&self) {
        if !self.fifo_write_space_available() {
            self.wait_for_write_watermark().await;
        }
    }

    async fn tx_fifo_enqueue_data(&self, val: u32) {
        self.wait_for_write_space_available().await;
        ral::write_reg!(ral::lpspi, self.data.lpspi.instance(), TDR, val);
    }

    async fn start_frame(
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

        self.wait_for_write_space_available().await;
        ral::write_reg!(ral::lpspi, self.data.lpspi.instance(), TCR,
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

    pub async unsafe fn perform_write_actions(
        &mut self,
        actions: impl Iterator<Item = WriteAction>,
        has_previous: bool,
        has_next: bool,
        byteorder: ByteOrder,
    ) {
        for action in actions {
            if action.len.get() < 4 {
                self.write_single_word(
                    action.buf,
                    byteorder,
                    action.read,
                    action.len,
                    action.is_first && !has_previous,
                    action.is_last && !has_next,
                )
                .await
            } else {
                self.write_u32_stream(
                    action.buf,
                    byteorder,
                    action.read,
                    action.len,
                    action.is_first && !has_previous,
                    action.is_last && !has_next,
                )
                .await;
            }
        }
    }

    pub async unsafe fn write_single_word(
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
        self.start_frame(
            false,
            is_first_frame,
            is_last_frame,
            read,
            write_data.is_some(),
            len,
        )
        .await;

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

            self.tx_fifo_enqueue_data(u32::from_le_bytes(tx_buffer))
                .await;
        }
    }

    pub async unsafe fn write_u32_stream(
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

        for chunk in ChunkIter::new(len, MAX_FRAME_SIZE_U32 as usize) {
            self.start_frame(
                byteorder.requires_flip(),
                is_first_frame && chunk.first,
                is_last_frame && chunk.last,
                read,
                write_data.is_some(),
                chunk.size.saturating_mul(NonZeroUsize::new_unchecked(4)),
            )
            .await;

            if let Some(write_data) = write_data {
                let write_data = write_data.add(chunk.position);

                let is_aligned = write_data.align_offset(core::mem::align_of::<u32>()) == 0;
                let requires_reorder = byteorder.requires_reorder();

                match (is_aligned, requires_reorder) {
                    (true, true) => {
                        for i in 0..chunk.size.get() {
                            let val = write_data.add(i).read();
                            let val = byteorder.reorder(val);
                            self.tx_fifo_enqueue_data(val).await;
                        }
                    }
                    (true, false) => {
                        // This is the case that supports DMA.
                        // TODO: add DMA.
                        for i in 0..chunk.size.get() {
                            let val = write_data.add(i).read();
                            self.tx_fifo_enqueue_data(val).await;
                        }
                        log::info!("Would support DMA.");
                    }
                    (false, true) => {
                        for i in 0..chunk.size.get() {
                            let val = write_data.add(i).read_unaligned();
                            let val = byteorder.reorder(val);
                            self.tx_fifo_enqueue_data(val).await;
                        }
                    }
                    (false, false) => {
                        for i in 0..chunk.size.get() {
                            let val = write_data.add(i).read_unaligned();
                            self.tx_fifo_enqueue_data(val).await;
                        }
                    }
                }
            }
        }
    }
}
