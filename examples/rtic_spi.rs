//! Demonstrates an interrupt-driven SPI device.
//!
//! Connect SDI to SDO. The example uses the LPSPI interrupt to
//! schedule transfers, and to receive data. You can observe the
//! I/O with a scope / logic analyzer. The SPI CLK runs at 1MHz,
//! and the frame size is 64 bits.
//!
//! TODO: update description

#![no_std]
#![no_main]
// Required for RTIC 2 (for now)
#![feature(type_alias_impl_trait)]

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    use embedded_hal_bus::spi::ExclusiveDevice;
    use hal::lpspi::FullDma;
    use imxrt_hal as hal;

    use eh1::spi::Operation;
    use eh1::spi::SpiDevice;
    use rtic_monotonics::systick::*;

    use board::{SpiBusDma, SpiCsPin, SpiInterruptHandler};

    #[local]
    struct Local {
        spi_device: ExclusiveDevice<SpiBusDma, SpiCsPin, Systick>,
        spi_interrupt_handler: SpiInterruptHandler,
    }

    #[shared]
    struct Shared {}

    #[init(local = [
        spi_systick: Option<Systick> = None,
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let (
            board::Common { mut dma, .. },
            board::Specifics {
                spi: (spi_bus, spi_cs_pin),
                ..
            },
        ) = board::new();

        // Init monotonic
        let systick_token = rtic_monotonics::create_systick_token!();
        Systick::start(
            cx.core.SYST,
            600_000_000, /* TODO: fix */
            systick_token,
        );

        // Init DMA
        let mut chan_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        chan_a.set_disable_on_completion(true);

        let mut chan_b = dma[board::BOARD_DMA_B_INDEX].take().unwrap();
        chan_b.set_disable_on_completion(true);

        // Configure SPI
        let mut spi_bus = spi_bus.with_dma(FullDma(chan_a, chan_b));
        let spi_interrupt_handler = spi_bus.enable_interrupts();

        // Create SPI device
        let spi_device = ExclusiveDevice::new(spi_bus, spi_cs_pin, Systick);

        (
            Shared {},
            Local {
                spi_device,
                spi_interrupt_handler,
            },
        )
    }

    #[task(priority = 1, local = [spi_device])]
    async fn app(cx: app::Context) {
        let app::LocalResources { spi_device, .. } = cx.local;

        loop {
            Systick::delay(1000.millis()).await;

            // To demonstrate normal operation
            spi_device
                .transaction(&mut [
                    Operation::DelayUs(100),
                    Operation::Write(&[12345u16]),
                    Operation::DelayUs(10),
                    Operation::Write(&[420, 69, 42]),
                    Operation::Write(&[0xFFFF]),
                    Operation::DelayUs(50),
                ])
                .unwrap();

            // To demonstrate larger, DMA based transfers
            let mut buf = [0xf5u32; 512];
            spi_device.transfer_in_place(&mut buf).unwrap();
        }
    }

    #[task(priority = 2, binds = BOARD_SPI, local = [spi_interrupt_handler])]
    fn spi_interrupt(cx: spi_interrupt::Context) {
        cx.local.spi_interrupt_handler.on_interrupt();
    }
}
