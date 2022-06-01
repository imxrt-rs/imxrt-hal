//! A loopback device. Send characters, and you should see
//! the exact same characters sent back. The LED toggles for
//! every exchanged character.
//!
//! Baud: 115200bps.
//!
//! See the additional compile-time configurations for more
//! testing.

#![no_std]
#![no_main]

/// If true, enable the TX and RX FIFOs. If false,
/// simply use the receive / transmit data registers.
///
/// If the FIFOs are enabled, you must send 4 characters
/// to receive the echo of four characters / see the LED
/// toggle.
const CONFIG_FIFOS: bool = true;

#[rtic::app(device = imxrt_ral, peripherals = true)]
mod app {
    use hal::lpuart;
    use imxrt_hal as hal;

    #[local]
    struct Local {
        led: board::Led,
        console: board::Console,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Board {
            led, mut console, ..
        } = board::new(cx.device);
        console.disable(|console| {
            if crate::CONFIG_FIFOS {
                console.enable_fifo(lpuart::Watermark::rx(
                    core::num::NonZeroU32::new(3).unwrap(),
                ));
                console.enable_fifo(lpuart::Watermark::tx(3));
            } else {
                console.disable_fifo(lpuart::Direction::Tx);
                console.disable_fifo(lpuart::Direction::Rx);
            }
            // Interrupt when we receive a byte.
            console.set_interrupts(lpuart::Interrupts::RIE);
        });
        (Shared {}, Local { led, console }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }

    #[task(binds = BOARD_CONSOLE, local = [led, console])]
    fn console_interrupt(cx: console_interrupt::Context) {
        use lpuart::Status;
        let console = cx.local.console;
        let led = cx.local.led;

        let status = console.status();
        console.clear_status(Status::W1C);

        if status.contains(Status::RDRF) {
            loop {
                let data = console.read_data();
                if data.flags().contains(lpuart::ReadFlags::RXEMPT) {
                    break;
                }
                if console.status().contains(Status::TDRE) {
                    console.write_byte(data.into());
                }
            }
            led.toggle();
        }
    }
}
