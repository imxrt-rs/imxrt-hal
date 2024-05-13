//! Demonstrates a SPI device with blocking I/O.
//!
//! Connect SDI to SDO. The example uses the LPSPI interrupt to
//! schedule transfers, and to receive data. You can observe the
//! I/O with a scope / logic analyzer. The SPI CLK runs at 1MHz.
//!
//! Keep an eye on the defmt log to see if tests fail.

#![no_std]
#![no_main]

#[rtic::app(device = board, peripherals = false)]
mod app {

    use imxrt_hal as hal;

    const PIT_DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

    #[local]
    struct Local {
        spi: board::Spi,
        pit: hal::pit::Pit<2>,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (
            board::Common {
                pit: (_, _, pit, _),
                ..
            },
            board::Specifics { spi, .. },
        ) = board::new();
        (Shared {}, Local { spi, pit })
    }

    #[idle(local = [spi, pit])]
    fn idle(cx: idle::Context) -> ! {
        let idle::LocalResources { spi, pit, .. } = cx.local;
        pit.set_load_timer_value(PIT_DELAY_MS);

        let mut delay = move || {
            pit.enable();
            while !pit.is_elapsed() {}
            pit.clear_elapsed();
            pit.disable();
        };

        loop {
            for _ in 0..3 {
                delay();
            }

            // For studying the effects of bit order and word size.
            //
            // If you have a logic analyzer that can change its word
            // size and bit order, use this sequence to evaluate how
            // the driver packs your transfer elements.
            {
                use eh02::blocking::spi::Write;
                use hal::lpspi::BitOrder::{self, *};

                const BIT_ORDERS: [BitOrder; 2] = [Msb, Lsb];

                const U32_WORDS: [u32; 2] = [0xDEADBEEFu32, 0xAD1CAC1D];
                for bit_order in BIT_ORDERS {
                    spi.set_bit_order(bit_order);
                    spi.write(&U32_WORDS).unwrap();
                }

                const U8_WORDS: [u8; 7] = [0xDEu8, 0xAD, 0xBE, 0xEF, 0xA5, 0x00, 0x1D];
                for bit_order in BIT_ORDERS {
                    spi.set_bit_order(bit_order);
                    spi.write(&U8_WORDS).unwrap();
                }

                const U16_WORDS: [u16; 3] = [0xDEADu16, 0xBEEF, 0xA5A5];
                for bit_order in BIT_ORDERS {
                    spi.set_bit_order(bit_order);
                    spi.write(&U16_WORDS).unwrap();
                }

                delay();
            }

            // Change me to explore bit order behavors in the
            // remaining write / loopback transfer tests.
            spi.set_bit_order(hal::lpspi::BitOrder::Msb);

            // Make sure concatenated elements look correct on the wire.
            {
                use eh02::blocking::spi::Write;

                spi.write(&[1u8, 2, 3]).unwrap();
                spi.write(&[1u8, 2, 3, 4]).unwrap();
                spi.write(&[1u8, 2, 3, 4, 5]).unwrap();
                spi.write(&[1u8, 2, 3, 4, 5, 6]).unwrap();
                spi.write(&[1u8, 2, 3, 4, 5, 6, 7]).unwrap();

                spi.write(&[0x0102u16, 0x0304, 0x0506]).unwrap();
                spi.write(&[0x0102u16, 0x0304, 0x0506, 0x0708]).unwrap();
                spi.write(&[0x0102u16, 0x0304, 0x0506, 0x0708, 0x090A])
                    .unwrap();

                spi.write(&[0x01020304u32, 0x05060708, 0x090A0B0C]).unwrap();

                delay();
            }

            {
                use eh02::blocking::spi::{Transfer, Write};

                // Change me to test different Elem sizes, buffer sizes,
                // bit patterns.
                type Elem = u8;
                const SENTINEL: Elem = 0x0F;
                const BUFFER: [Elem; 13] = [SENTINEL; 13];

                // Simple loopback transfer. Easy to find with your
                // scope.
                let mut buffer = BUFFER;
                spi.transfer(&mut buffer).unwrap();
                if buffer != BUFFER {
                    defmt::error!("Simple transfer buffer mismatch!");
                }

                delay();

                // Adjacent loopback transfer. Look for the big
                // burst of data on your scope.
                let mut buffer = BUFFER;
                let mut error = false;
                for idx in 0u32..16 {
                    buffer.fill(SENTINEL.rotate_right(idx));
                    let expected = buffer;
                    spi.transfer(&mut buffer).unwrap();
                    error |= buffer != expected;
                }
                if error {
                    defmt::error!("At least one of the bursted transfers didn't match!");
                }

                delay();

                // Simple write.
                let buffer = BUFFER;
                spi.write(&buffer).unwrap();

                delay();

                // Pipelined writes. Look for the burst of data
                // on your scope. Internally, the writes will flush,
                // so the delay between transfers should be about
                // the same as they are for the transfers.
                let mut buffer = BUFFER;
                for idx in 0..16 {
                    buffer.fill(SENTINEL.rotate_right(idx));
                    spi.write(&buffer).unwrap();
                }

                delay();
            }
        }
    }
}
