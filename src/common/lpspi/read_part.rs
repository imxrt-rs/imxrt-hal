use core::num::NonZeroUsize;

use super::{
    ral,
    transfer_actions::{ByteOrder, ReadAction},
    LpspiReadPart,
};

impl<const N: u8> LpspiReadPart<'_, N> {
    fn fifo_read_data_available(&mut self) -> bool {
        ral::read_reg!(
            ral::lpspi,
            self.data.lpspi.instance(),
            RSR,
            RXEMPTY == RXEMPTY_0
        )
    }

    async fn wait_for_read_watermark(&mut self, watermark: u32) {
        self.data
            .lpspi
            .wait_for_rx_watermark(watermark)
            .await
            .unwrap();
    }

    async fn wait_for_read_data_available(&mut self, at_most: usize) {
        if !self.fifo_read_data_available() {
            let mut watermark = self.rx_fifo_size / 2;

            // If there are only a couple of bytes left in the current
            // transmission, then waiting for rx_fifo_size/2 bytes
            // might not wake us, causing a deadlock.
            // Therefore dynamically reduce the watermark if required.
            if let Ok(at_most) = u32::try_from(at_most) {
                watermark = watermark.min(at_most);
            }
            self.wait_for_read_watermark(watermark).await;
        }
    }

    pub async unsafe fn perform_read_actions(
        &mut self,
        actions: impl Iterator<Item = ReadAction>,
        byteorder: ByteOrder,
    ) {
        for action in actions {
            if action.len.get() < 4 {
                self.read_single_word(action.buf, byteorder, action.len)
                    .await
            } else {
                todo!();
                // self.read_u32_stream(action.buf, byteorder, action.len)
                //     .await;
            }
        }
    }

    async fn rx_fifo_read_data(&mut self, num_leftover: usize) -> u32 {
        self.wait_for_read_data_available(num_leftover).await;
        ral::read_reg!(ral::lpspi, self.data.lpspi.instance(), RDR)
    }

    pub async unsafe fn read_single_word(
        &mut self,
        data: *mut u8,
        byteorder: ByteOrder,
        len: NonZeroUsize,
    ) {
        assert!(len.get() < 4);

        let reverse_bytes = match byteorder {
            ByteOrder::Normal => false,
            ByteOrder::WordReversed => true,
            ByteOrder::HalfWordReversed => true,
        };

        log::info!("Receiving ...");
        let value = self.rx_fifo_read_data(1).await;
        log::info!("Read: 0x{:02x}", value);
        let rx_buffer = value.to_le_bytes();

        let active_buffer = &rx_buffer[(4 - len.get())..];
        if reverse_bytes {
            active_buffer
                .iter()
                .rev()
                .enumerate()
                .for_each(|(pos, val)| data.add(pos).write(*val));
        } else {
            active_buffer
                .iter()
                .enumerate()
                .for_each(|(pos, val)| data.add(pos).write(*val));
        }
    }
}
