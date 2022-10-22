//! Tests for the LPUART async DMA interface.
//!
//! Wait for a serial character, then send the same
//! character back 32 times. The LED toggles after
//! every round trip.

#![no_std]
#![no_main]

#[imxrt_rt::entry]
fn main() -> ! {
    let (
        board::Common { mut dma, .. },
        board::Specifics {
            led, mut console, ..
        },
    ) = board::new();

    let mut channel = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    channel.set_disable_on_completion(true);

    let task = async {
        loop {
            led.toggle();

            let mut buffer = [0u8; 1];
            console.dma_read(&mut channel, &mut buffer).await.unwrap();

            let buffer = [buffer[0]; 32];
            console.dma_write(&mut channel, &buffer).await.unwrap();
        }
    };
    pin_utils::pin_mut!(task);
    board::blocking::run(task);
    unreachable!();
}
