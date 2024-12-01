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
                use eh1::spi::SpiDevice;
                use hal::lpspi::BitOrder::{self, *};

                const BIT_ORDERS: [BitOrder; 2] = [Msb, Lsb];

                const U32_WORDS: [u32; 2] = [0xDEADBEEFu32, 0xAD1CAC1D];
                for bit_order in BIT_ORDERS {
                    spi.bus_mut().set_bit_order(bit_order);
                    spi.write(&U32_WORDS).unwrap();
                }

                const U8_WORDS: [u8; 7] = [0xDEu8, 0xAD, 0xBE, 0xEF, 0x12, 0x34, 0x56];
                for bit_order in BIT_ORDERS {
                    spi.bus_mut().set_bit_order(bit_order);
                    spi.write(&U8_WORDS).unwrap();
                }

                const U16_WORDS: [u16; 3] = [0xDEADu16, 0xBEEF, 0x1234];
                for bit_order in BIT_ORDERS {
                    spi.bus_mut().set_bit_order(bit_order);
                    spi.write(&U16_WORDS).unwrap();
                }

                delay();
            }

            // Change me to explore bit order behavors in the
            // remaining write / loopback transfer tests.
            spi.bus_mut().set_bit_order(hal::lpspi::BitOrder::Msb);

            // Make sure concatenated elements look correct on the wire.
            // Make sure we can read those elements.
            {
                use eh1::spi::SpiDevice;
                use hal::lpspi::BitOrder;

                macro_rules! transfer_test {
                    ($arr:expr, $bit_order:expr) => {
                        let bit_order_name = match $bit_order {
                            BitOrder::Msb => "MSB",
                            BitOrder::Lsb => "LSB",
                        };

                        spi.bus_mut().set_bit_order($bit_order);
                        let mut buffer = $arr;
                        spi.transfer_in_place(&mut buffer).unwrap();
                        defmt::assert_eq!(buffer, $arr, "Bit Order {}", bit_order_name);
                    };
                }

                transfer_test!([1u8, 2, 3], BitOrder::Msb);
                transfer_test!([1u8, 2, 3], BitOrder::Lsb);

                transfer_test!([1u8, 2, 3, 4], BitOrder::Msb);
                transfer_test!([1u8, 2, 3, 4], BitOrder::Lsb);

                transfer_test!([1u8, 2, 3, 4, 5], BitOrder::Msb);
                transfer_test!([1u8, 2, 3, 4, 5], BitOrder::Lsb);

                transfer_test!([1u8, 2, 3, 4, 5, 6], BitOrder::Msb);
                transfer_test!([1u8, 2, 3, 4, 5, 6], BitOrder::Lsb);

                transfer_test!([1u8, 2, 3, 4, 5, 6, 7], BitOrder::Msb);
                transfer_test!([1u8, 2, 3, 4, 5, 6, 7], BitOrder::Lsb);

                transfer_test!([0x0102u16, 0x0304, 0x0506], BitOrder::Msb);
                transfer_test!([0x0102u16, 0x0304, 0x0506], BitOrder::Lsb);

                transfer_test!([0x0102u16, 0x0304, 0x0506, 0x0708], BitOrder::Msb);
                transfer_test!([0x0102u16, 0x0304, 0x0506, 0x0708], BitOrder::Lsb);

                transfer_test!([0x0102u16, 0x0304, 0x0506, 0x0708, 0x090A], BitOrder::Msb);
                transfer_test!([0x0102u16, 0x0304, 0x0506, 0x0708, 0x090A], BitOrder::Lsb);

                transfer_test!([0x01020304u32, 0x05060708, 0x090A0B0C], BitOrder::Msb);
                transfer_test!([0x01020304u32, 0x05060708, 0x090A0B0C], BitOrder::Lsb);

                spi.bus_mut().set_bit_order(BitOrder::Msb);
                delay();
            }

            {
                use eh1::spi::SpiDevice;

                // Change me to test different Elem sizes, buffer sizes,
                // bit patterns.
                type Elem = u8;
                const SENTINEL: Elem = 0x0F;
                const BUFFER: [Elem; 13] = [SENTINEL; 13];

                // Simple loopback transfer. Easy to find with your
                // scope.
                let mut buffer = BUFFER;
                spi.transfer_in_place(&mut buffer).unwrap();
                defmt::assert_eq!(buffer, BUFFER);

                delay();

                // Adjacent loopback transfer. Look for the big
                // burst of data on your scope.
                let mut buffer = BUFFER;
                let mut error = false;
                for idx in 0u32..16 {
                    buffer.fill(SENTINEL.rotate_right(idx));
                    let expected = buffer;
                    spi.transfer_in_place(&mut buffer).unwrap();
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

            {
                use eh1::spi::{
                    Operation::{Read, TransferInPlace, Write},
                    SpiDevice,
                };

                let mut read = [0u8; 7];
                let mut xfer = [0u8; 10];
                for idx in 0..xfer.len() {
                    xfer[idx] = idx as u8;
                }

                spi.transaction(&mut [
                    TransferInPlace(&mut xfer),
                    Read(&mut read),
                    Write(&[0xA5; 13][..]),
                ])
                .unwrap();

                assert_eq!(read, [0xff; 7]);
                assert_eq!(xfer, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

                delay();
            }
        }
    }
}
