//! Logging front-end.
//!
//! Bridges the `log` singleton with a `bbqueue` queue.
//! Asynchronous peripherals can then read the queue to
//! transport data.

use core::cell::RefCell;

use super::Filters;

use bbqueue as bbq;
use critical_section::Mutex;
use static_cell::StaticCell;

struct Logger<'a, const N: usize> {
    producer: Mutex<RefCell<bbq::Producer<'a, N>>>,
    filters: Filters,
}

impl log::Log for Logger<'_, { crate::BUFFER_SIZE }> {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        metadata.level() <= ::log::max_level() // The log level is appropriate
            && self.filters.is_enabled(metadata) // The target is in the filter list
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut writer = Writer(self);
            use core::fmt::Write;
            let _ = write!(
                writer,
                "[{} {}]: {}\r\n",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        // Not yet supported.
    }
}

struct Writer<'a, 'b, const N: usize>(&'b Logger<'a, N>);

impl<const N: usize> core::fmt::Write for Writer<'_, '_, N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        critical_section::with(|cs| {
            let prod = self.0.producer.borrow(cs);
            let mut prod = prod.borrow_mut();
            crate::try_write_producer(s.as_bytes(), &mut prod).map_err(|_| core::fmt::Error)
        })
    }
}

/// Initialize the logging frontend.
///
/// # Panics
///
/// Panics if called more than once.
pub(crate) fn init(
    producer: bbq::Producer<'static, { crate::BUFFER_SIZE }>,
    config: &super::LoggingConfig,
) -> Result<(), crate::AlreadySetError<()>> {
    static LOGGER: StaticCell<Logger<'static, { crate::BUFFER_SIZE }>> = StaticCell::new();
    let logger = LOGGER.init(Logger {
        producer: Mutex::new(RefCell::new(producer)),
        filters: super::Filters(config.filters),
    });
    ::log::set_logger(logger)
        .map(|_| ::log::set_max_level(config.max_level))
        .map_err(|_| crate::AlreadySetError::new(()))
}
