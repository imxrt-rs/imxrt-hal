//! Demonstrates an interrupt-driven SPI device.
//!
//! Connect SDI to SDO. The example uses the LPSPI interrupt to
//! schedule transfers, and to receive data. You can observe the
//! I/O with a scope / logic analyzer. The SPI CLK runs at 1MHz,
//! and the frame size is 64 bits.
//!
//! # Board compatibility
//!
//! Due to cortex-m-rtic #197 [1], this example is a non-traditional
//! RTIC application. The example conditionally compiles the
//! 'mod app' item in order to generalize platforms. The only thing
//! that changes is the "binds" hardware task attribute.
//!
//! [1]: https://github.com/rtic-rs/cortex-m-rtic/issues/197

#![no_std]
#![no_main]

use hal::lpspi::{Direction, Interrupts, MasterStatus, Transaction};
use imxrt_hal as hal;

#[cfg(feature = "imxrt1010evk")]
mod lpspi1 {
    #[rtic::app(device = imxrt_ral, peripherals = true)]
    mod app {

        #[local]
        struct Local {
            app: crate::App,
        }

        #[shared]
        struct Shared {}

        #[init]
        fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
            (
                Shared {},
                Local {
                    app: crate::App::init(cx.device),
                },
                init::Monotonics(),
            )
        }

        #[idle]
        fn idle(_: idle::Context) -> ! {
            loop {
                rtic::export::wfi();
            }
        }

        #[task(binds = LPSPI1, local = [app, word: u32 = 0xDEADBEEF])]
        fn spi_interrupt(cx: spi_interrupt::Context) {
            *cx.local.word = cx.local.app.spi_interrupt(*cx.local.word);
        }
    }
}

#[cfg(not(feature = "imxrt1010evk"))]
mod lpspi4 {
    #[rtic::app(device = imxrt_ral, peripherals = true)]
    mod app {

        #[local]
        struct Local {
            app: crate::App,
        }

        #[shared]
        struct Shared {}

        #[init]
        fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
            (
                Shared {},
                Local {
                    app: crate::App::init(cx.device),
                },
                init::Monotonics(),
            )
        }

        #[idle]
        fn idle(_: idle::Context) -> ! {
            loop {
                rtic::export::wfi();
            }
        }

        #[task(binds = LPSPI4, local = [app, word: u32 = 0xDEADBEEF])]
        fn spi_interrupt(cx: spi_interrupt::Context) {
            *cx.local.word = cx.local.app.spi_interrupt(*cx.local.word);
        }
    }
}

pub struct App {
    spi: board::Spi,
}

impl App {
    fn init(peripherals: imxrt_ral::Peripherals) -> Self {
        let board::Board { mut spi, .. } = board::new(peripherals);
        spi.disabled(|spi| {
            // Trigger when the TX FIFO is empty.
            spi.set_watermark(Direction::Tx, 0);
            // Wait to receive at least 2 u32s.
            spi.set_watermark(Direction::Rx, 1);
        });
        // Starts the I/O as soon as we're done initializing, since
        // the TX FIFO is empty.
        spi.set_interrupts(Interrupts::TDIE);
        Self { spi }
    }

    fn spi_interrupt(&mut self, word: u32) -> u32 {
        let status = self.spi.status();
        self.spi.clear_status(MasterStatus::TDF | MasterStatus::RDF);

        if status.intersects(MasterStatus::TDF) {
            // This write clears TDIE.
            self.spi.set_interrupts(Interrupts::RDIE);

            // Sending two u32s. Frame size is represented by bits.
            let transaction = Transaction::new(2 * 8 * core::mem::size_of::<u32>() as u16)
                .expect("Transaction frame size is within bounds");
            self.spi.enqueue_transaction(&transaction);

            self.spi.enqueue_data(word as u32);
            self.spi.enqueue_data(!word as u32);

            word.wrapping_add(1)
        } else if status.intersects(MasterStatus::RDF) {
            // This write clears RDIE.
            self.spi.set_interrupts(Interrupts::TDIE);

            assert!(self.spi.fifo_status().rxcount == 2);

            while let Some(_) = self.spi.read_data() {}
            word
        } else {
            word
        }
    }
}
