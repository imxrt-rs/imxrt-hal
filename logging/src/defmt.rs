//! defmt logging frontend.
//!
//! The module provides a [`defmt`](https://defmt.ferrous-systems.com/introduction.html)
//! implementation that transfers data using any supported backend.
//!
//! The implementation only supports one logging channel. The acquire-release sequence
//! on that channel uses a critical section to ensure mutual exclusion.

use bbqueue as bbq;
type Producer = bbq::Producer<'static, { crate::BUFFER_SIZE }>;

#[cfg(feature = "lpuart")]
use imxrt_hal::{dma::channel::Channel, lpuart::Lpuart};

/// The defmt frontend provides safe access to the producer
/// and encoder.
mod frontend {
    use super::Producer;
    use core::{
        cell::{Cell, RefCell},
        mem::MaybeUninit,
        sync::atomic::{AtomicBool, Ordering},
    };
    use critical_section::RestoreState;

    pub(super) struct Frontend {
        init: AtomicBool,
        producer: RefCell<MaybeUninit<Producer>>,
        restore_state: Cell<RestoreState>,
        encoder: RefCell<defmt::Encoder>,
    }

    // Safety: The frontend uses internal critical sections to prevent
    // concurrent access. The frontend uses an initializatino flag to
    // track uninitialized memory.
    unsafe impl Sync for Frontend {}

    impl Frontend {
        const fn new() -> Self {
            Self {
                init: AtomicBool::new(false),
                producer: RefCell::new(MaybeUninit::uninit()),
                restore_state: Cell::new(RestoreState::invalid()),
                encoder: RefCell::new(defmt::Encoder::new()),
            }
        }

        pub(super) fn init(&self, producer: Producer) {
            // Safety: module inspection shows that this function is only called once,
            // so we're not changing the pointed-to value while another accessor is
            // looking at this memory.
            *self.producer.borrow_mut() = MaybeUninit::new(producer);
            self.init.store(true, Ordering::Release);
        }

        /// # Safety
        ///
        /// Caller must check that init is true, and that we're within a critical
        /// section.
        unsafe fn with_producer_encoder<R>(
            &self,
            func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R,
        ) -> R {
            // Safety:
            // - the memory is initialized because init is true.
            // - we have exclusive access due to acquire.
            unsafe {
                let mut producer = self.producer.borrow_mut();
                let producer = producer.assume_init_mut();

                let mut encoder = self.encoder.borrow_mut();
                func(producer, &mut encoder)
            }
        }

        /// Try to acquire the logger, skipping everything if the logger isn't initialized.
        pub fn acquire<R>(
            &self,
            func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R,
        ) -> Option<R> {
            self.init.load(Ordering::Acquire).then(|| {
                // Safety: we're relying on the defmt::Logger user to meet some of these
                // critical section guarantees. We're also relying on the firmware developer
                // to either
                //
                // - build separate binaries for multi-core systems
                // - select a proper critical_section implementation for those multi-core systems
                //   that needed to share the logging queue.
                unsafe { self.restore_state.set(critical_section::acquire()) };

                // Safety: Acquired critical section, and init is true.
                unsafe { self.with_producer_encoder(func) }
            })
        }

        /// Try to release the logger, skipping everything if the loger isn't initialized.
        pub fn release<R>(
            &self,
            func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R,
        ) -> Option<R> {
            self.init.load(Ordering::Acquire).then(|| {
                // Safety:
                // - init is true.
                // - if we're releasing the critical section, we must
                //   already have acquired the critical section.
                let result = unsafe { self.with_producer_encoder(func) };

                // Safety: we're relying on the defmt::Logger user to meet some critical
                // section guarantees. See the related safety comment on acquire.
                unsafe {
                    critical_section::release(self.restore_state.replace(RestoreState::invalid()))
                };

                result
            })
        }

        pub fn write<R>(
            &self,
            func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R,
        ) -> Option<R> {
            self.init.load(Ordering::Acquire).then(|| {
                // Safety:
                // - the memory is initialized because INIT is true.
                // - the defmt user can only call this after calling
                //   acquire, so there's only one mutable access.
                unsafe { self.with_producer_encoder(func) }
            })
        }
    }

    pub(super) static FRONTEND: Frontend = Frontend::new();
}

#[defmt::global_logger]
struct Logger;

// Safety: we rely on a critical section acquire in order to meet the
// defmt::Logger safety contract.
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        frontend::FRONTEND.acquire(|producer, encoder| {
            encoder.start_frame(|buffer| {
                let _ = crate::try_write_producer(buffer, producer);
            });
        });
    }

    // In release and write, we enver check if there was a corresponding
    // acquire call. This means that, if the demft front-end was initialized
    // after an acquire but before a release / write (somehow?), then the
    // producer may receive incomplete frames (those not prefixed with any
    // start_frame encodings). The safety contracts for release and write
    // make this assumption safe.

    unsafe fn release() {
        frontend::FRONTEND.release(|producer, encoder| {
            encoder.end_frame(|buffer| {
                let _ = crate::try_write_producer(buffer, producer);
            });
        });
    }

    unsafe fn write(bytes: &[u8]) {
        frontend::FRONTEND.write(|producer, encoder| {
            encoder.write(bytes, |buffer| {
                let _ = crate::try_write_producer(buffer, producer);
            });
        });
    }

    unsafe fn flush() {
        // Nothing to do today.
    }
}

/// Initialize a USB logger with the `defmt` frontend and custom configurations.
///
/// See the crate-level documentation to understand how the USB device backend works.
#[cfg(feature = "usbd")]
pub fn usb_with_config<P: imxrt_usbd::Peripherals>(
    peripherals: P,
    interrupts: super::Interrupts,
    backend_config: &crate::UsbdConfig,
) -> Result<crate::Poller, crate::AlreadySetError<P>> {
    let (producer, consumer) = match crate::BUFFER.try_split() {
        Ok((prod, cons)) => (prod, cons),
        Err(_) => return Err(crate::AlreadySetError::new(peripherals)),
    };

    critical_section::with(|_| {
        frontend::FRONTEND.init(producer);
        let backend = crate::usbd::init(peripherals, interrupts, consumer, backend_config);
        Ok(crate::Poller::new(backend))
    })
}

/// Initialize a USB logger with the `defmt` frontend.
///
/// This function uses default configurations for the backend.
/// See the crate-level documentation to understand how the USB device backend works.
#[cfg(feature = "usbd")]
pub fn usbd<P: imxrt_usbd::Peripherals>(
    peripherals: P,
    interrupts: super::Interrupts,
) -> Result<crate::Poller, crate::AlreadySetError<P>> {
    usb_with_config(
        peripherals,
        interrupts,
        &crate::UsbdConfigBuilder::new().build(),
    )
}

/// Initialize a LPUART & DMA logger with the `defmt` frontend.
///
/// See the crate-level documentation to understand how the LPUART backend works.
#[cfg(feature = "lpuart")]
pub fn lpuart<P, const LPUART: u8>(
    lpuart: Lpuart<P, LPUART>,
    dma_channel: Channel,
    interrupts: crate::Interrupts,
) -> Result<crate::Poller, crate::AlreadySetError<(Lpuart<P, LPUART>, Channel)>> {
    let (producer, consumer) = match crate::BUFFER.try_split() {
        Ok((prod, cons)) => (prod, cons),
        Err(_) => return Err(crate::AlreadySetError::new((lpuart, dma_channel))),
    };

    critical_section::with(|_| {
        frontend::FRONTEND.init(producer);
        let backend = crate::lpuart::init(lpuart, dma_channel, consumer, interrupts);
        Ok(crate::Poller::new(backend))
    })
}
