//! A thin board support package for `imxrt-hal` hardware examples.
//!
//! The top-level module exposes configurations and APIs that are common across
//! boards. For board specific information, like which LPUART is the console and
//! which pins are I2C, see the board-specific modules.

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;
use imxrt_rt as _;

mod ral_shim;

/// SOC run mode.
///
/// Each MCU specifies its own core clock speed
/// and power settings for these variants. They're
/// typically follow the recommendations in the
/// data sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[non_exhaustive]
pub enum RunMode {
    /// The fastest, highest-power mode.
    Overdrive,
}

pub use ral_shim::{BOARD_DMA_A_INDEX, BOARD_DMA_B_INDEX, NVIC_PRIO_BITS};

#[cfg(board = "imxrt1010evk")]
#[path = "imxrt1010evk.rs"]
mod board_impl;

#[cfg(board = "imxrt1060evk")]
#[path = "imxrt1060evk.rs"]
mod board_impl;

#[cfg(board = "teensy4")]
#[path = "teensy4.rs"]
mod board_impl;

#[cfg(board = "imxrt1170evk-cm7")]
#[path = "imxrt1170evk-cm7.rs"]
mod board_impl;

#[cfg(board = "imxrt1180evk-cm33")]
#[path = "imxrt1180evk-cm33.rs"]
mod board_impl;

#[cfg(feature = "lcd1602")]
pub use lcd_1602_i2c as lcd1602;

// rustdoc doesn't like when this is named 'board'
// since it happens to match the package name.
// So went with an '_impl' suffix.
pub use board_impl::*;

/// Components that are common to all boards.
///
/// This includes timers, DMA channels, and things
/// that don't necessarily depend on a pinout.
#[cfg(any(chip = "imxrt1010", chip = "imxrt1060", chip = "imxrt1170"))]
pub struct Common {
    /// PIT channels.
    pub pit: hal::pit::Channels,
    /// GPT1 timer.
    ///
    /// Use [`GPT1_FREQUENCY`] to understand its frequency.
    pub gpt1: hal::gpt::Gpt<1>,
    /// GPT2 timer.
    ///
    /// Use [`GPT2_FREQUENCY`] to understand its frequency.
    pub gpt2: hal::gpt::Gpt<2>,
    /// DMA channels.
    pub dma: [Option<hal::dma::channel::Channel>; hal::dma::CHANNEL_COUNT],
    /// Secure real-time counter.
    ///
    /// Examples may enable the SRTC.
    pub srtc: hal::snvs::srtc::Disabled,
    /// SNVS LP core registers.
    ///
    /// May be used with the SRTC.
    pub snvs_lp_core: hal::snvs::LpCore,
    /// USB1 core registers.
    pub usb1: Usb1,
    /// USB1 non-core registers.
    pub usbnc1: UsbNc1,
    /// USBPHY1 registers.
    pub usbphy1: UsbPhy1,
}

#[cfg(any(chip = "imxrt1010", chip = "imxrt1060", chip = "imxrt1170"))]
impl Common {
    /// Prepares common resources.
    fn new() -> Self {
        let pit: Pit = unsafe { Pit::instance() };
        // Stop timers in debug mode.
        ral::modify_reg!(ral::pit, pit, MCR, FRZ: FRZ_1);
        let pit = hal::pit::new(pit);

        let gpt1 = configure_gpt(unsafe { ral::gpt::GPT1::instance() }, GPT1_DIVIDER);
        let gpt2 = configure_gpt(unsafe { ral::gpt::GPT2::instance() }, GPT2_DIVIDER);

        let dma = hal::dma::channels(unsafe { ral::dma::DMA::instance() }, unsafe {
            ral::dmamux::DMAMUX::instance()
        });

        let hal::snvs::Snvs {
            low_power:
                hal::snvs::LowPower {
                    core: snvs_lp_core,
                    srtc,
                    ..
                },
            ..
        } = hal::snvs::new(unsafe { ral::snvs::SNVS::instance() });

        Self {
            pit,
            gpt1,
            gpt2,
            dma,
            srtc,
            snvs_lp_core,
            usb1: unsafe { Usb1::instance() },
            usbnc1: unsafe { UsbNc1::instance() },
            usbphy1: unsafe { UsbPhy1::instance() },
        }
    }
}

#[cfg(chip = "imxrt1180")]
#[non_exhaustive]
pub struct Common {}

#[cfg(chip = "imxrt1180")]
impl Common {
    fn new() -> Self {
        Self {}
    }
}
/// Board entrypoint.
///
/// Use this to configure the hardware and acquire peripherals.
///
/// # Panics
///
/// This should only be called once, at the top of your `main()` routine.
/// It panics if any hardware resource is already taken.
pub fn new() -> (Common, Specifics) {
    static ONCE: AtomicBool = AtomicBool::new(false);
    let done = ONCE.fetch_or(true, Ordering::SeqCst);
    assert!(!done, "You've already initialized the board.");

    // Safety: once flag ensures that this only happens once.
    unsafe {
        ral_shim::shim_vectors();
        configure();
        let mut common = Common::new();
        let specifics = Specifics::new(&mut common);
        (common, specifics)
    }
}

/// The board's run mode.
pub const RUN_MODE: RunMode = RunMode::Overdrive;

const GPT1_DIVIDER: u32 = 10;
const GPT2_DIVIDER: u32 = 100;
const GPT_SELECTION: hal::gpt::ClockSource = hal::gpt::ClockSource::HighFrequencyReferenceClock;

/// Target SPI baud rate (Hz).
pub const SPI_BAUD_RATE_FREQUENCY: u32 = 1_000_000;
/// The console baud rate: 115200bps.
pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);
/// Target I2C baud rate (Hz).
pub const I2C_BAUD_RATE: hal::lpi2c::Timing =
    hal::lpi2c::Timing::ideal(LPI2C_CLK_FREQUENCY, hal::lpi2c::ClockSpeed::KHz400);

#[cfg(chip = "imxrt1010")]
use iomuxc::imxrt1010::Pads;

#[cfg(chip = "imxrt1060")]
use iomuxc::imxrt1060::Pads;

#[cfg(chip = "imxrt1170")]
use iomuxc::imxrt1170::Pads;

#[cfg(chip = "imxrt1180")]
use iomuxc::imxrt1180::Pads;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

fn configure_gpt<const N: u8>(gpt: ral::gpt::Instance<N>, divider: u32) -> hal::gpt::Gpt<N>
where
    ral::gpt::Instance<N>: ral::Valid,
{
    let mut gpt = hal::gpt::Gpt::new(gpt);
    gpt.disable();
    gpt.set_wait_mode_enable(true);
    gpt.set_clock_source(GPT_SELECTION);
    gpt.set_divider(divider);
    gpt
}

#[cfg(chip = "imxrt1010")]
mod usb1 {
    use crate::ral;

    pub type Usb1 = ral::usb::USB;
    pub type UsbPhy1 = ral::usbphy::USBPHY;
    pub type UsbNc1 = ral::usbnc::USBNC;
}

#[cfg(not(chip = "imxrt1010"))]
mod usb1 {
    use crate::ral;

    pub type Usb1 = ral::usb::USB1;
    pub type UsbPhy1 = ral::usbphy::USBPHY1;
    pub type UsbNc1 = ral::usbnc::USBNC1;
}

use usb1::*;

#[cfg(any(chip = "imxrt1010", chip = "imxrt1060"))]
type Pit = crate::ral::pit::PIT;
#[cfg(chip = "imxrt1170")]
type Pit = crate::ral::pit::PIT1;

/// Board interrupts.
///
/// Associated to interrupt numbers in board modules.
#[allow(unused)]
mod board_interrupts {
    pub type Vector = unsafe extern "C" fn();
    extern "C" {
        pub fn BOARD_CONSOLE();
        pub fn BOARD_BUTTON();
        pub fn BOARD_SPI();
        pub fn BOARD_PWM();
        pub fn BOARD_DMA_A();
        pub fn BOARD_DMA_B();
        pub fn BOARD_PIT();
        pub fn BOARD_GPT1();
        pub fn BOARD_GPT2();
        pub fn BOARD_USB1();
        pub fn BOARD_SWTASK0();
    }
}

/// A simple blocking executor for async hardware examples.
pub mod blocking {
    use core::{future::Future, pin::Pin, task::Poll};

    /// Poll a future with a dummy waker.
    ///
    /// Use `poll_no_wake` when you want to drive a future to completion, but you
    /// don't care about the future waking an executor. It may be used to initiate
    /// a DMA transfer that will later be awaited with [`block`].
    ///
    /// Do not use `poll_no_wake` if you want an executor to be woken when the DMA
    /// transfer completes.
    fn poll_no_wake<F>(future: Pin<&mut F>) -> Poll<F::Output>
    where
        F: Future,
    {
        use core::task::{Context, RawWaker, RawWakerVTable, Waker};
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});

        const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
        // Safety: raw waker meets documented requirements.
        let waker = unsafe { Waker::from_raw(RAW_WAKER) };
        let mut context = Context::from_waker(&waker);
        future.poll(&mut context)
    }

    /// Block until the future returns a result.
    ///
    /// `block` invokes `poll_no_wake()` in a loop until the future
    /// returns a result. Consider using `block` after starting a transfer
    /// with `poll_no_wake`, and after doing other work.
    pub fn run<F>(mut future: Pin<&mut F>) -> F::Output
    where
        F: Future,
    {
        loop {
            match poll_no_wake(future.as_mut()) {
                Poll::Ready(result) => return result,
                Poll::Pending => {}
            }
        }
    }
}

/// Configurations for the logger.
///
/// If your board is ready to support the logging infrastructure,
/// add the 'imxrt-log' feature to your board's list of enabled
/// features. Then, simply define the default backend in your module.
#[cfg(feature = "imxrt-log")]
pub mod logging {
    use crate::hal::{dma::channel::Channel, lpuart::Lpuart, usbd::Instances};
    pub use imxrt_log::Poller;
    pub const BACKEND: Backend = crate::board_impl::DEFAULT_LOGGING_BACKEND;

    /// Select the logging front-end.
    #[derive(Debug, defmt::Format, PartialEq, Eq)]
    pub enum Frontend {
        /// Use the `log` crate.
        Log,
        #[cfg(feature = "logging-defmt")]
        /// Use `defmt`.
        Defmt,
    }

    /// Select the logging back-end.
    #[derive(Debug, defmt::Format, PartialEq, Eq)]
    pub enum Backend {
        /// Use a USB peripheral.
        Usbd,
        /// Use LPUART and DMA.
        Lpuart,
    }

    /// Initialize the logger.
    pub fn init<P, const LPUART: u8, const USBD: u8>(
        frontend: Frontend,
        backend: Backend,
        lpuart: Lpuart<P, LPUART>,
        dma: Channel,
        usbd: Instances<USBD>,
    ) -> imxrt_log::Poller {
        // Always enable interrupts. If you don't want them to activate, don't unmask them.
        match (frontend, backend) {
            // Logging frontends...
            (Frontend::Log, Backend::Lpuart) => {
                imxrt_log::log::lpuart(lpuart, dma, imxrt_log::Interrupts::Enabled).unwrap()
            }
            (Frontend::Log, Backend::Usbd) => {
                imxrt_log::log::usbd(usbd, imxrt_log::Interrupts::Enabled).unwrap()
            }
            // Defmt frontends...
            #[cfg(feature = "logging-defmt")]
            (Frontend::Defmt, Backend::Lpuart) => {
                imxrt_log::defmt::lpuart(lpuart, dma, imxrt_log::Interrupts::Enabled).unwrap()
            }
            #[cfg(feature = "logging-defmt")]
            (Frontend::Defmt, Backend::Usbd) => {
                imxrt_log::defmt::usbd(usbd, imxrt_log::Interrupts::Enabled).unwrap()
            }
        }
    }

    /// Initialize the LPUART logger.
    ///
    /// Useful when you're debugging USB devices, and you want to get
    /// log messages out of the device some other way.
    ///
    /// This always enables interrupts. If you don't want interrupts to active,
    /// then don't unmask them.
    pub fn lpuart<P, const LPUART: u8>(
        frontend: Frontend,
        lpuart: Lpuart<P, LPUART>,
        dma_channel: Channel,
    ) -> imxrt_log::Poller {
        match frontend {
            Frontend::Log => {
                imxrt_log::log::lpuart(lpuart, dma_channel, imxrt_log::Interrupts::Enabled).unwrap()
            }
            #[cfg(feature = "logging-defmt")]
            Frontend::Defmt => {
                imxrt_log::defmt::lpuart(lpuart, dma_channel, imxrt_log::Interrupts::Enabled)
                    .unwrap()
            }
        }
    }
}
