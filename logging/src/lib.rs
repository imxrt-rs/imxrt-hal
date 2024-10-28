//! Logging extensions for i.MX RT processors.
//!
//! `imxrt-log` supports two logging frontends:
//!
//! - [`defmt`][defmt-docs] for efficient logging.
//! - [`log`][log-docs] for text-based logging.
//!
//! See the [`defmt`] and [`log`] modules for more information.
//!
//! `imxrt-log` builds upon the `imxrt-hal` hardware abstraction layer (HAL)
//! and provides two peripheral backends:
//!
//! - LPUART with DMA
//! - USB serial (CDC) device
//!
//! Mix and match these frontends and backends to integrate logging into your
//! i.MX RT processor. To understand the differences of each frontend, see
//! the package documentation. Read on to learn about building this package,
//! and to understand the differences in each backend.
//!
//! # Building
//!
//! Given its dependency on [`imxrt-hal`][hal-docs], this package has the same build
//! requirements as `imxrt-hal`. To learn how to build this package, consult the
//! HAL documentation. Essentially, if you can build the HAL, you can build this
//! package.
//!
//! This package uses [`critical-section`](https://crates.io/crates/critical-section)
//! to ensure safe concurrent access to the log producer. In order for this
//! to build, you must select a correct critical section implementation for your
//! system. See the `critical-section` documentation for more information.
//!
//! # Design
//!
//! Logging frontends place log frames in a circular buffer. Compile- and run-time
//! filters prevent log message formatting and copies. For more frontend
//! design details, see the documentation of each frontend module.
//!
//! Backends read from this circular buffer and asynchronously transfer data out
//! of memory. Backends may buffer data as part of their implementation.
//!
//! The circular buffer is the limiting resource. Once you initialize a logger
//! with a frontend-backend combination, you cannot initialize any other loggers.
//!
//! # Backend usage
//!
//! The LPUART and USB backends provide a consistent interface to drive logging.
//! After initializing a front and backend pair, you receive a [`Poller`] object.
//! In order to move log messages, you must occasionally call `poll()` on the poller
//! object. Each `poll()` call either does nothing, or drives the asynchronous
//! transfer of log messages from your peripheral.
//!
//! The API allows you to enable or disable interrupts that fire when a transfer
//! completes. Depending on the backend, the interrupt may periodically trigger.
//! If the interrupt periodically triggers, you can use the interrupt to occasionally
//! call `poll()`.
//!
//! The backends have some behavioral and performance differences. They're also
//! initialized differently. The next section describes these differences.
//!
//! ## LPUART with DMA
//!
//! The LPUART with DMA implementation transports log messages over LPUART using
//! DMA transfers. In summary,
//!
//! - Initialize your LPUART before initializing the logger.
//! - If you enable interrupts, define your interrupt handlers.
//! - Bring your own timer to call `poll()`.
//! - It uses less memory than USB.
//!
//! _Initialization_. The logging initialiation routine requires an LPUART
//! object from `imxrt-hal`. Configure your `Lpuart` object with baud rates,
//! parity bits, etc. before supplying it to the logging initialization routine.
//!
//! The initialization routine also requires a DMA channel. Any DMA channel will
//! do. The implementation fully configures the DMA channel, so there is no need for
//! you to configure the channel.
//!
//! _Interrupts_. If you enable interrupts (see [`Interrupts`]), the DMA channel
//! asserts its interrupt when each transfer completes. You must call `poll()`
//! to clear the interrupt. The implementation does not touch LPUART interrupts.
//!
//! _Timers_. The interrupts enabled by the LPUART backend cannot periodically
//! trigger. Therefore, you are responsible for periodically calling `poll()`.
//! Consider using a PIT or GPT timer from `imxrt-hal` to help with this, or
//! consider calling `poll()` in a software loop.
//!
//! _Buffer management_. The implementation performs DMA transfers directly out
//! of the log message buffer. This means that there is no intermediate buffer
//! for log messages. The implementation frees the log messages from the circular
//! buffer once the transfer completes.
//!
//! ## USBD
//!
//! The USB device implementation transports log messages over USB by presenting
//! a serial (USB CDC) class to a USB host. In summary,
//!
//! - Simply provide USB register blocks to the logger initialization routine.
//! - If you enable interrupts, define your interrupt handles.
//! - You might not need your own timer.
//! - It uses more memory than LPUART.
//!
//! _Initialization_. The logging initialization routine handles all peripheral
//! configuration. You simply provide the USB register block instances; `imxrt-hal`
//! can help with this.
//!
//! By default, the initialization routine configures a high-speed USB device with
//! a 512 byte bulk endpoint max packet size. You can change these settings with
//! build-time environment variables, discussed later.
//!
//! _Interrupts_. If you enable interrupts (see [`Interrupts`]), the USB device
//! controller asserts its interrupt when each transfer completes. It also enables
//! a USB-managed timer to periodically trigger the interrupt. You must call `poll()`
//! to clear these interrupt conditions.
//!
//! _Timers_. If you enable interrupts, the associated USB interrupt periodically
//! fires. You can use this to periodically call `poll()` without using any other
//! timer or software loop.
//!
//! The timer has a default interval. You can configure this interval through each
//! logger initialization routine.
//!
//! If you do not enable interrupts, you're responsible for periodically calling
//! `poll()`. See the LPUART _timers_ discussion for recommendations.
//!
//! _Buffer management_. The implementation copies data out of the circular buffer
//! and places it in an intermediate transfer buffer. Once this copy completes, the
//! implementation frees the log frames from the circular buffer, and starts the
//! USB transfer from this intermediate buffer. The requirement for the intermediate
//! buffer is a USB driver implementation detail that increases this backend's memory
//! requirements.
//!
//! # Examples
//!
//! It's easiest to use the USB backend because it has a built-in timer, and the
//! implementation handles all peripheral initialization. The example below shows
//! an interrupt-driven USB logger. It uses `imxrt-hal` APIs to prepare the logger.
//!
//! ```no_run
//! use imxrt_log::defmt; // <-- Change 'defmt' to 'log' to change the frontend.
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//!
//! use ral::interrupt;
//! #[cortex_m_rt::interrupt]
//! fn USB_OTG1() {
//!     static mut POLLER: Option<imxrt_log::Poller> = None;
//!     if let Some(poller) = POLLER.as_mut() {
//!         poller.poll();
//!     } else {
//!         let poller = initialize_logger().unwrap();
//!         *POLLER = Some(poller);
//!         // Since we enabled interrupts, this interrupt
//!         // handler will be called for USB traffic and timer
//!         // events. These are handled by poll().
//!     }
//! }
//!
//! /// Initialize a USB logger.
//! ///
//! /// Returns `None` if any USB peripheral instance is taken,
//! /// or if initialization fails.
//! fn initialize_logger() -> Option<imxrt_log::Poller> {
//!     let usb_instances = hal::usbd::Instances {
//!         usb: unsafe { ral::usb::USB1::instance() },
//!         usbnc: unsafe { ral::usbnc::USBNC1::instance() },
//!         usbphy: unsafe { ral::usbphy::USBPHY1::instance() },
//!     };
//!     // Initialize the logger, and ensure that it triggers interrupts.
//!     let poller = defmt::usbd(usb_instances, imxrt_log::Interrupts::Enabled).ok()?;
//!     Some(poller)
//! }
//!
//! // Elsewhere in your code, configure USB clocks. Then, pend the USB_OTG1()
//! // interrupt so that it fires and initializes the logger.
//! # || -> Option<()> {
//! let mut ccm = unsafe { ral::ccm::CCM::instance() };
//! let mut ccm_analog = unsafe { ral::ccm_analog::CCM_ANALOG::instance() };
//! hal::ccm::analog::pll3::restart(&mut ccm_analog);
//! hal::ccm::clock_gate::usb().set(&mut ccm, hal::ccm::clock_gate::ON);
//!
//! cortex_m::peripheral::NVIC::pend(interrupt::USB_OTG1);
//! // Safety: interrupt handler is self contained and safe to unmask.
//! unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
//! # Some(()) }().unwrap();
//!
//! // After the USB device enumerates and configures, you're ready for
//! // logging.
//! ::defmt::info!("Hello world!");
//! ```
//!
//! For an advanced example that uses RTIC, see the `rtic_logging` example
//! maintained in the `imxrt-hal` repository. This example lets you easily explore
//! all frontend-backend combinations, and it works on various i.MX RT development
//! boards.
//!
//! # Package configurations
//!
//! You can configure this package at compile time.
//!
//! - Binary configurations use feature flags.
//! - Variable configurations use environment variables set during compilation.
//!
//! The table below describes the package feature flags. Default features make it
//! easy for you to use all package features. To reduce dependencies, disable this
//! package's default features, then selectively enable frontends and backends.
//!
//! | Feature flag | Description                         | Enabled by default? |
//! | ------------ | ----------------------------------- | ------------------- |
//! | `defmt`      | Enable the `defmt` logging frontend |        Yes          |
//! | `log`        | Enable the `log` logging frontend   |        Yes          |
//! | `lpuart`     | Enable the LPUART backend           |        Yes          |
//! | `usbd`       | Enable the USB device backend       |        Yes          |
//!
//! This package isn't particularly interesting without a frontend-backend combination,
//! so this configuration is not supported. Any features not listed above are considered
//! an implementation detail and may change without notice.
//!
//! Environment variables provide additional configuration hooks. The table below
//! describes the supported configuration variables and their effects on the build.
//!
//! | Environment variable           | Description                                               | Default value | Accepted values           |
//! | ------------------------------ | --------------------------------------------------------- | ------------- | ------------------------- |
//! | `IMXRT_LOG_USB_BULK_MPS`       | Bulk endpoint max packet size, in bytes.                  |     512       | One of 8, 16, 32, 64, 512 |
//! | `IMXRT_LOG_USB_SPEED`          | Specify a high (USB2) or full (USB 1.1) speed USB device. |    HIGH       | Either `HIGH` or `FULL`   |
//! | `IMXRT_LOG_BUFFER_SIZE`        | Specify the log message buffer size, in bytes.            |    1024       | An integer power of two   |
//!
//! Note:
//!
//! - `IMXRT_LOG_USB_*` are always permitted. If `usbd` is disabled, then `IMXRT_LOG_USB_*`
//!    configurations do nothing.
//! - If `IMXRT_LOG_USB_SPEED=FULL`, then `IMXRT_LOG_USB_BULK_MPS` cannot be 512. On the other hand,
//!   if `IMXRT_LOG_USB_SPEED=HIGH`, then `IMXRT_LOG_USB_BULK_MPS` must be 512.
//! - Both `IMXRT_LOG_USB_BULK_MPS` and `IMXRT_LOG_BUFFER_SIZE` affect internally-managed buffer
//!   sizes. If space is tight, reduces these numbers to reclaim memory.
//!
//! # Limitations
//!
//! Although it uses `critical-section`, this logging package may not be designed for immediate
//! use in a multi-core system, like the i.MX RT 1160 and 1170 MCUs. Notably, there's no critical
//! section implementation for these processors that would ensure safe, shared access to the log
//! producer across the cores. Furthermore, its not yet clear how to build embedded Rust applications
//! for these systems.
//!
//! Despite these limitations, it may be possible to use this package on multi-core MCUs, but you need
//! to treat them as two single-core MCUs. Specifically, you would need to build two binaries -- one for
//! each core, each having separate memory regions for data -- and each core would need to use its own,
//! distinct peripheral for transport. Then, select a single-core `critical-section` implementation,
//! like the one provided by `cortex-m`.
//!
//! [defmt-docs]: https://defmt.ferrous-systems.com
//! [hal-docs]: https://docs.rs/imxrt-hal
//! [log-docs]: https://docs.rs/log/0.4/log/

#![no_std]
#![warn(
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::undocumented_unsafe_blocks,
    clippy::missing_safety_doc
)]

#[cfg(feature = "defmt")]
pub mod defmt;
#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "lpuart")]
mod lpuart;

#[cfg(feature = "usbd")]
mod usbd;
#[cfg(feature = "usbd")]
pub use usbd::{UsbdConfig, UsbdConfigBuilder};

/// Interrupt configuration.
///
/// If interrupts are enabled, you're responsible for registering the ISR
/// associated with the peripheral. See the crate-level documentation to
/// understand how this affects each logging backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupts {
    /// Peripheral interrupts are disabled.
    Disabled,
    /// Peripheral interrupts are enabled.
    Enabled,
}

fn try_write_producer<const N: usize>(
    buffer: &[u8],
    producer: &mut bbqueue::Producer<'_, N>,
) -> Result<(), bbqueue::Error> {
    fn write_grant<'a, const N: usize>(
        bytes: &'a [u8],
        prod: &mut bbqueue::Producer<'_, N>,
    ) -> Result<&'a [u8], bbqueue::Error> {
        let mut grant = prod.grant_max_remaining(bytes.len())?;
        let grant_len = grant.len();
        grant.copy_from_slice(&bytes[..grant_len]);
        grant.commit(grant_len);
        Ok(&bytes[grant_len..])
    }

    // Either (1) write all of s into the buffer, (2) fill up the back of the buffer,
    // or (3) fill up as much as you can until you hit old data.
    let buffer = write_grant::<N>(buffer, producer)?;

    // Non-empty for (2) and (3).
    if !buffer.is_empty() {
        // This could either fail, or the grant could be smaller than the (remaining)
        // string. In the latter case, we drop data.
        write_grant::<N>(buffer, producer)?;
    }

    Ok(())
}

/// An error indicating the logger is already set.
///
/// This could happen because
///
/// - you've already initialized a logger provided by this package
/// - you're using the `log` package, and something else has registered
///   the dynamic logger
pub struct AlreadySetError<R> {
    /// Holds the peripherals and other state provided to the
    /// initialization routine.
    pub resources: R,
}

impl<R> AlreadySetError<R> {
    fn new(resources: R) -> Self {
        Self { resources }
    }
}

impl<R> core::fmt::Debug for AlreadySetError<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("logger already set")
    }
}

include!(concat!(env!("OUT_DIR"), "/config.rs"));
use config::BUFFER_SIZE;

static BUFFER: bbqueue::BBBuffer<BUFFER_SIZE> = bbqueue::BBBuffer::new();
type Consumer = bbqueue::Consumer<'static, { crate::BUFFER_SIZE }>;

/// The poller drives the logging process.
///
/// You're expected to periodically call [`poll()`](Self::poll) to asynchronously
/// move log messages from memory to your peripheral. `poll()` never
/// blocks on I/O or data.
///
/// `Poller` logically owns static, mutable state that's allocated behind
/// this package's API. The specific state depends on the selected backend.
/// Since it manages static, mutable state, there can only be one instance
/// of a `Poller` in any program.
pub struct Poller {
    inner: Inner,
}

// Safety: it's OK to move this across execution contexts.
// Poller is !Send, so the same object cannot be safely accessed
// across these execution contexts.
unsafe impl Send for Poller {}

impl Poller {
    fn new<B: Into<Inner>>(backend: B) -> Self {
        Poller {
            inner: backend.into(),
        }
    }

    /// Drive the logging process.
    ///
    /// If log messages are available, and if there is no active transfer,
    /// `poll()` initiates a new transfer. It also manages the state of the
    /// backend peripheral. There's no guarantee on how many bytes are sent
    /// in each transfer.
    #[inline]
    pub fn poll(&mut self) {
        self.inner.poll();
    }
}

enum Inner {
    Lpuart(&'static mut lpuart::Backend),
    Usbd(&'static mut usbd::Backend),
}

impl From<&'static mut lpuart::Backend> for Inner {
    fn from(backend: &'static mut lpuart::Backend) -> Self {
        Inner::Lpuart(backend)
    }
}

impl From<&'static mut usbd::Backend> for Inner {
    fn from(backend: &'static mut usbd::Backend) -> Self {
        Inner::Usbd(backend)
    }
}

impl Inner {
    fn poll(&mut self) {
        match self {
            Self::Lpuart(backend) => backend.poll(),
            Self::Usbd(backend) => backend.poll(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::try_write_producer;
    use bbqueue::BBBuffer;

    #[test]
    fn write_producer_simple() {
        let bb = BBBuffer::<4>::new();
        let (mut prod, mut cons) = bb.try_split().unwrap();
        try_write_producer(&[1, 2, 3], &mut prod).unwrap();
        assert_eq!(cons.read().unwrap().buf(), &[1, 2, 3]);
    }

    #[test]
    fn write_producer_lost_data() {
        let bb = BBBuffer::<5>::new();
        let (mut prod, mut cons) = bb.try_split().unwrap();
        prod.grant_exact(2).unwrap().commit(2);
        cons.read().unwrap().release(1);
        assert!(try_write_producer(&[1, 2, 3, 4], &mut prod).is_err());
        assert_eq!(cons.read().unwrap().buf(), &[0, 1, 2, 3]);
    }

    #[test]
    fn write_producer_wrap_around() {
        let bb = BBBuffer::<5>::new();
        let (mut prod, mut cons) = bb.try_split().unwrap();
        prod.grant_exact(3).unwrap().commit(3);
        cons.read().unwrap().release(2);
        try_write_producer(&[1, 2, 3, 4], &mut prod).unwrap();
        let grant = cons.split_read().unwrap();
        let (bck, fnt) = grant.bufs();
        assert_eq!(bck, &[0, 1, 2]);
        // Looks like BBBuffer uses an extra element to differentiate start / end points,
        // so we lost data without any error. That's OK.
        assert_eq!(fnt, &[3]);
    }

    #[test]
    fn default_configs() {
        assert_eq!(crate::config::USB_BULK_MPS, 512);
        assert_eq!(crate::config::USB_SPEED, imxrt_usbd::Speed::High);
        assert_eq!(crate::config::BUFFER_SIZE, 1024);
    }
}
