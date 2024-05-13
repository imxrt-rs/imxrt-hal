//! A SPI controller for testing SPI devices.
//!
//! Run this on one of your two development boards. Connect each board's
//! SPI peripherals, and establish a common ground. Run rtic_spi_peripheral.rs
//! on the other development board.
//!
//! The controller sends two operands to the device. The device is expected
//! to add those two operands (with wrapping). The controller expects this
//! response from the device. The controllers transacts I/O as fast as
//! possible. It periodically logs errors.
//!
//! You can monitor this device's defmt output to track the number of
//! protocol errors.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {

    use core::sync::atomic::{AtomicU32, Ordering};

    use hal::lpspi::BitOrder;
    use imxrt_hal as hal;

    const BIT_ORDER: BitOrder = BitOrder::Msb;

    #[local]
    struct Local {
        spi: board::Spi,
        pit: hal::pit::Pit<2>,
    }

    #[shared]
    struct Shared {
        errors: AtomicU32,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (
            board::Common {
                pit: (_, _, mut pit, _),
                ..
            },
            board::Specifics { mut spi, .. },
        ) = board::new();

        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(board::PIT_FREQUENCY);
        pit.enable();

        spi.set_bit_order(BIT_ORDER);
        (
            Shared {
                errors: AtomicU32::new(0),
            },
            Local { spi, pit },
        )
    }

    #[idle(shared = [&errors], local = [spi])]
    fn idle(cx: idle::Context) -> ! {
        use eh02::blocking::spi::*;
        let idle::SharedResources { errors, .. } = cx.shared;
        let idle::LocalResources { spi, .. } = cx.local;

        for fst in (0u8..!0).cycle() {
            let snd = fst.wrapping_mul(7).wrapping_sub(13);
            spi.write(&[fst]).unwrap();
            spi.write(&[snd]).unwrap();

            let mut sum = [0xFFu8; 1];
            spi.transfer(&mut sum).unwrap();

            errors.fetch_add(
                u32::from(sum[0] != fst.wrapping_add(snd)),
                Ordering::Relaxed,
            );
        }

        unreachable!();
    }

    #[task(binds = BOARD_PIT, shared = [&errors], local = [pit, count: usize = 0])]
    fn report_errors(cx: report_errors::Context) {
        let report_errors::LocalResources { pit, count, .. } = cx.local;

        while pit.is_elapsed() {
            pit.clear_elapsed();
        }

        *count = count.wrapping_add(1);
        defmt::warn!(
            "({=usize}) Total errors: {=u32}",
            *count,
            cx.shared.errors.load(Ordering::Relaxed)
        );
    }
}
