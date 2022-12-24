//! Demonstrates full-duplex, async SPI I/O.
//!
//! Connect SDI to SDO to enable loopback transfers. If there's no loopback,
//! the example panics. Use this to interrupt an otherwise working example.

#![no_std]
#![no_main]

#[imxrt_rt::entry]
fn main() -> ! {
    let (board::Common { mut dma, .. }, board::Specifics { mut spi, .. }) = board::new();

    let mut chan_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
    chan_a.set_disable_on_completion(true);

    let mut chan_b = dma[board::BOARD_DMA_B_INDEX].take().unwrap();
    chan_b.set_disable_on_completion(true);

    let task = async {
        let mut elem: u32 = 0;
        loop {
            let expected = [elem; 8];
            let mut actual = [elem; 8];
            spi.dma_full_duplex(&mut chan_a, &mut chan_b, &mut actual)
                .unwrap()
                .await
                .unwrap();
            assert_eq!(actual, expected);
            elem = elem.wrapping_add(1);
        }
    };
    pin_utils::pin_mut!(task);
    board::blocking::run(task);
    unreachable!();
}
