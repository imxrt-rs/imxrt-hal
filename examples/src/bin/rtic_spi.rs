//! Demonstrates an interrupt-driven SPI device.
//!
//! Connect SDI to SDO. The example uses the LPSPI interrupt to
//! schedule transfers, and to receive data. You can observe the
//! I/O with a scope / logic analyzer. The SPI CLK runs at 1MHz,
//! and the frame size is 64 bits.

#![no_std]
#![no_main]

#[rtic::app(device = imxrt_ral, peripherals = true)]
mod app {

    use hal::lpspi::{Direction, Interrupts, MasterStatus, Transaction};
    use imxrt_hal as hal;

    #[local]
    struct Local {
        spi: board::Spi,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Board { mut spi, .. } = board::new(cx.device);
        spi.disabled(|spi| {
            // Trigger when the TX FIFO is empty.
            spi.set_watermark(Direction::Tx, 0);
            // Wait to receive at least 2 u32s.
            spi.set_watermark(Direction::Rx, 1);
        });
        // Starts the I/O as soon as we're done initializing, since
        // the TX FIFO is empty.
        spi.set_interrupts(Interrupts::TDIE);
        (Shared {}, Local { spi }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }

    #[task(binds = BOARD_SPI, local = [spi])]
    fn spi_interrupt(cx: spi_interrupt::Context) {
        let spi_interrupt::LocalResources { spi } = cx.local;

        let status = spi.status();
        spi.clear_status(MasterStatus::TDF | MasterStatus::RDF);

        if status.intersects(MasterStatus::TDF) {
            // This write clears TDIE.
            spi.set_interrupts(Interrupts::RDIE);

            // Sending two u32s. Frame size is represented by bits.
            let transaction = Transaction::new(2 * 8 * core::mem::size_of::<u32>() as u16)
                .expect("Transaction frame size is within bounds");
            spi.enqueue_transaction(&transaction);

            spi.enqueue_data(0xDEADBEEF);
            spi.enqueue_data(!0xDEADBEEF);
        } else if status.intersects(MasterStatus::RDF) {
            // This write clears RDIE.
            spi.set_interrupts(Interrupts::TDIE);

            assert!(spi.fifo_status().rxcount == 2);

            while let Some(_) = spi.read_data() {}
        }
    }
}
