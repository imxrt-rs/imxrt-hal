//! Demonstrates an interrupt-driven SPI device.
//!
//! Connect SDI to SDO. The example uses the LPSPI interrupt to
//! schedule transfers, and to receive data. You can observe the
//! I/O with a scope / logic analyzer. The SPI CLK runs at 1MHz,
//! and the frame size is 64 bits.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {

    use hal::lpspi::{Direction, Interrupts, Status, Transaction};
    use imxrt_hal as hal;

    #[local]
    struct Local {
        spi: board::Spi,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (_, board::Specifics { mut spi, .. }) = board::new();
        // Trigger when the TX FIFO is empty.
        spi.set_watermark(Direction::Tx, 0);
        // Wait to receive at least 2 u32s.
        spi.set_watermark(Direction::Rx, 1);
        // Starts the I/O as soon as we're done initializing, since
        // the TX FIFO is empty.
        spi.set_interrupts(Interrupts::TRANSMIT_DATA);
        (Shared {}, Local { spi })
    }

    #[task(binds = BOARD_SPI, local = [spi])]
    fn spi_interrupt(cx: spi_interrupt::Context) {
        let spi_interrupt::LocalResources { spi, .. } = cx.local;

        let status = spi.status();
        spi.clear_status(Status::TRANSMIT_DATA | Status::RECEIVE_DATA);

        if status.intersects(Status::TRANSMIT_DATA) {
            // This write clears TRANSMIT_DATA.
            spi.set_interrupts(Interrupts::RECEIVE_DATA);

            // Sending two u32s. Frame size is represented by bits.
            let transaction = Transaction::new(2 * 8 * core::mem::size_of::<u32>() as u16)
                .expect("Transaction frame size is within bounds");
            spi.enqueue_transaction(&transaction);

            spi.enqueue_data(0xDEADBEEF);
            spi.enqueue_data(!0xDEADBEEF);
        } else if status.intersects(Status::RECEIVE_DATA) {
            // This write clears RECEIVE_DATA.
            spi.set_interrupts(Interrupts::TRANSMIT_DATA);

            assert!(spi.fifo_status().rxcount == 2);

            while spi.read_data().is_some() {}
        }
    }
}
