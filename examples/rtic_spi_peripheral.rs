//! A SPI device demonstration.
//!
//! Run this on one of your two development boards. Connect each board's
//! SPI peripherals, and establish a common ground. Run
//! rtic_spi_controller.rs on the other development board.
//!
//! To understand the protocol, see the rtic_spi_controller.rs documentation.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {

    use hal::lpspi::{BitOrder, Direction, Interrupts, Transaction};
    use imxrt_hal as hal;

    #[local]
    struct Local {
        spi: board::Spi,
    }

    #[shared]
    struct Shared {}

    type Elem = u8;
    const FRAME_SIZE: u16 = (core::mem::size_of::<Elem>() * 8) as u16;

    const BIT_ORDER: BitOrder = BitOrder::Msb;

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (_, board::Specifics { mut spi, .. }) = board::new();

        spi.disabled(|spi| spi.set_peripheral_enable(true));

        // Expect Elem bits per transaction.
        //
        // Don't transmit anything during this transaction!
        // Our protocol uses two separate transactions to
        // convey data.
        let mut transaction = Transaction::new(FRAME_SIZE).unwrap();
        transaction.transmit_data_mask = true;
        transaction.bit_order = BIT_ORDER;
        spi.enqueue_transaction(&transaction);

        // React once we have both operands in the FIFO.
        spi.set_watermark(Direction::Rx, 1);
        spi.set_interrupts(Interrupts::RECEIVE_DATA);

        (Shared {}, Local { spi })
    }

    #[task(binds=BOARD_SPI, local = [spi])]
    fn spi_interrupt(cx: spi_interrupt::Context) {
        let spi_interrupt::LocalResources { spi, .. } = cx.local;

        let status = spi.status();
        spi.clear_status(status);

        // There must be something, or we wouldn't have activated.
        let fst: Elem = spi.read_data().unwrap().try_into().unwrap();
        let snd: Elem = spi.read_data().unwrap().try_into().unwrap();

        // Prepare a new transaction that only sends data (ignores any received
        // data).
        let mut transaction = Transaction::new(FRAME_SIZE).unwrap();
        transaction.receive_data_mask = true;
        transaction.bit_order = BIT_ORDER;
        spi.enqueue_transaction(&transaction);

        // Send the result.
        let sum: u8 = fst.wrapping_add(snd);
        spi.enqueue_data(sum.into());

        // Prepare to receive the next elements from the controller.
        let mut transaction = Transaction::new(FRAME_SIZE).unwrap();
        transaction.transmit_data_mask = true;
        transaction.bit_order = BIT_ORDER;
        spi.enqueue_transaction(&transaction);
    }
}
