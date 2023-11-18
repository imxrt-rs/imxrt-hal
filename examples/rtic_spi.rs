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

    use imxrt_hal as hal;

    use hal::lpspi::LpspiInterruptHandler;

    use rtic_monotonics::systick::*;

    #[local]
    struct Local {
        spi_device: board::SpiDevice,
        spi_interrupt_handler: LpspiInterruptHandler,
    }

    #[shared]
    struct Shared {}

    #[init(local = [
        spi_systick: Option<Systick> = None,
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let (
            _,
            board::Specifics {
                spi: (mut spi_bus, spi_cs_pin),
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

        // Configure SPI
        let spi_systick = cx.local.spi_systick.insert(Systick);
        spi_bus.set_delay_source(spi_systick).unwrap();
        let spi_interrupt_handler = spi_bus.enable_interrupts().unwrap();

        // Create SPI device
        let spi_device = spi_bus.device(spi_cs_pin);

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
            //spi_device.transfer(TODO);
        }
    }

    #[task(binds = BOARD_SPI, local = [spi_interrupt_handler])]
    fn spi_interrupt(cx: spi_interrupt::Context) {
        cx.local.spi_interrupt_handler.on_interrupt();
    }
}
