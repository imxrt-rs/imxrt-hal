//! Text-based logging frontend.
//!
//! The module provides a [`log`](https://crates.io/crates/log) implementation that
//! transfers data using any supported backend.
//!
//! Strings are formatted and serialized to a buffer. Compile and runtime filters prevent formatting
//! and serialization into the buffer. When it's time to copy the data into the circular buffer, the
//! implementation takes a short critical section.
//!
//! See [`LoggingConfig`] to learn more about the runtime filters.
//! See the `log` package documentation to learn about static filters.

mod filters;
mod frontend;

pub use filters::Filter;
use filters::Filters;

use crate::{Poller, BUFFER};

#[cfg(feature = "lpuart")]
use imxrt_hal::{dma::channel::Channel, lpuart::Lpuart};

/// Logging configuration
///
/// Use this to specify certain configurations of the logging
/// system. By default, the max log level is the log level set at
/// compile time. See the [compile time filters](https://docs.rs/log/0.4/log/#compile-time-filters)
/// section for more information. The frontend also enables logging for all targets.
/// Set the `filters` collection to specify log targets of interest.
///
/// If the default configuration is good for you, use `Default::default()`.
///
/// ```
/// use imxrt_log::log::{Filter, LoggingConfig};
///
/// const I2C_LOGGING: Filter = ("i2c", None);
/// const SPI_LOGGING: Filter = ("spi", Some(log::LevelFilter::Warn));
/// const MOTOR_LOGGING: Filter = ("motor", Some(log::LevelFilter::Trace));
///
/// let config = LoggingConfig {
///     max_level: log::LevelFilter::Debug,
///     filters: &[
///         I2C_LOGGING,
///         SPI_LOGGING,
///         MOTOR_LOGGING,
///     ]
/// };
/// ```
pub struct LoggingConfig {
    /// The max log level for *all* logging
    ///
    /// This is the static max level. You may
    /// override this to bypass the statically-assigned
    /// max level
    pub max_level: ::log::LevelFilter,
    /// A list of filtered targets to log.
    ///
    /// If set to an empty slice (default), the logger performs no
    /// filtering. Otherwise, the frontend filters the specified targets by
    /// the accompanying log level. See [`Filter`](type.Filter.html) for
    /// more information.
    pub filters: &'static [Filter],
}

impl LoggingConfig {
    /// Create a default logging config.
    ///
    /// Unlike `default()`, this works in `const` contexts.
    pub const fn new() -> Self {
        LoggingConfig {
            max_level: ::log::STATIC_MAX_LEVEL,
            filters: &[],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> LoggingConfig {
        Self::new()
    }
}

/// Initialize a USB logger with the `log` frontend and custom configurations.
///
/// See the crate-level documentation to understand how the USB device backend works.
#[cfg(feature = "usbd")]
pub fn usbd_with_config<P: imxrt_usbd::Peripherals>(
    peripherals: P,
    interrupts: super::Interrupts,
    frontend_config: &LoggingConfig,
    backend_config: &crate::UsbdConfig,
) -> Result<Poller, crate::AlreadySetError<P>> {
    let (producer, consumer) = match BUFFER.try_split() {
        Ok((prod, cons)) => (prod, cons),
        Err(_) => return Err(crate::AlreadySetError::new(peripherals)),
    };

    critical_section::with(|_| {
        if frontend::init(producer, frontend_config).is_err() {
            return Err(crate::AlreadySetError::new(peripherals));
        }
        let backend = crate::usbd::init(peripherals, interrupts, consumer, backend_config);
        Ok(Poller::new(backend))
    })
}

/// Initialize a USB logger with the `log` frontend.
///
/// This function uses default configurations for the frontend and backend.
/// See the crate-level documentation to understand how the USB device backend works.
#[cfg(feature = "usbd")]
pub fn usbd<P: imxrt_usbd::Peripherals>(
    peripherals: P,
    interrupts: super::Interrupts,
) -> Result<Poller, crate::AlreadySetError<P>> {
    usbd_with_config(
        peripherals,
        interrupts,
        &LoggingConfig::default(),
        &crate::UsbdConfigBuilder::new().build(),
    )
}

/// Initialize a LPUART & DMA logger with the `log` frontend and custom configurations.
///
/// See the crate-level documentation to understand how the LPUART backend works.
#[cfg(feature = "lpuart")]
pub fn lpuart_with_config<P, const LPUART: u8>(
    lpuart: Lpuart<P, LPUART>,
    dma_channel: Channel,
    interrupts: crate::Interrupts,
    frontend_config: &LoggingConfig,
) -> Result<Poller, crate::AlreadySetError<(Lpuart<P, LPUART>, Channel)>> {
    let (producer, consumer) = match BUFFER.try_split() {
        Ok((prod, cons)) => (prod, cons),
        Err(_) => return Err(crate::AlreadySetError::new((lpuart, dma_channel))),
    };

    critical_section::with(|_| {
        if frontend::init(producer, frontend_config).is_err() {
            return Err(crate::AlreadySetError::new((lpuart, dma_channel)));
        }
        let backend = crate::lpuart::init(lpuart, dma_channel, consumer, interrupts);
        Ok(Poller::new(backend))
    })
}

/// Initialize a LPUART & DMA logger with the `log` frontend.
///
/// This function uses default configurations for the frontend.
/// See the crate-level documentation to understand how the LPUART backend works.
#[cfg(feature = "lpuart")]
pub fn lpuart<P, const LPUART: u8>(
    lpuart: Lpuart<P, LPUART>,
    dma_channel: Channel,
    interrupts: crate::Interrupts,
) -> Result<Poller, crate::AlreadySetError<(Lpuart<P, LPUART>, Channel)>> {
    lpuart_with_config(lpuart, dma_channel, interrupts, &LoggingConfig::default())
}
