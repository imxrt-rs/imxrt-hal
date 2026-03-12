//! Read fuse values.
//!
//! This example uses `defmt` to convey the fuse value over RTT.
//! Although the example may build on your board, it may not work
//! properly.
//!
//! If you set `FUSE_VALUE_CAUTION_PERMANENT` to a non-zero value,
//! that value will be written to the given `FUSE_ADDRESS`. This
//! is permament, so be careful where you're writing!

#![no_main]
#![no_std]

use hal::ocotp::FuseAddress;
use imxrt_hal as hal;

const FUSE_ADDRESS: FuseAddress = FuseAddress::new(0x1300).unwrap();

/// If set to a non-zero value, the example attempts to write this
/// value to the fuse address above!
///
/// (It's possible to write zero to a ECC-redundant fuse and lock it.
/// We assume you don't want to do that!)
const FUSE_VALUE_CAUTION_PERMANENT: u32 = 0;

const DELAY_MS: u32 = board::PIT_FREQUENCY / 1_000 * 250;

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common {
            mut pit, mut ocotp, ..
        },
        board::Specifics { led, .. },
    ) = board::new();

    pit.0.set_load_timer_value(DELAY_MS);
    pit.0.enable();

    let mut write_delays = 50;
    let mut fuse_written = false;
    loop {
        while !pit.0.is_elapsed() {}
        pit.0.clear_elapsed();
        led.toggle();

        defmt::println!("Reading fuse {}...", FUSE_ADDRESS);
        match ocotp.blocking_fuse_read(FUSE_ADDRESS) {
            Ok(fuse_value) => {
                defmt::println!("Fuse value: {=u32:#010X}", fuse_value);
            }
            Err(err) => {
                defmt::error!("Failed to read fuse!: {}", err);
            }
        }

        if FUSE_VALUE_CAUTION_PERMANENT != 0 && !fuse_written {
            if write_delays == 0 {
                defmt::println!("=== WRITING FUSE ===");
                fuse_written = true;
                match ocotp.blocking_fuse_write(FUSE_ADDRESS, FUSE_VALUE_CAUTION_PERMANENT) {
                    Ok(()) => {
                        defmt::println!("Fuse write finished!");
                    }
                    Err(err) => {
                        defmt::error!("Failed to write fuse! {}", err);
                    }
                }
            } else {
                defmt::println!("!! Writing fuse when {=u32} reaches zero", write_delays);
                write_delays -= 1;
            }
        }
    }
}
