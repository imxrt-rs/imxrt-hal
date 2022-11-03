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
        mem::MaybeUninit,
        sync::atomic::{AtomicBool, Ordering},
    };

    /// The defmt global_logger is a static singleton. This differs
    /// from the log singleton, which is dynamically set at runtime.
    ///
    /// Since the singleton is static, it could be invoked before
    /// PRODUCER has been initialized. This INIT flag guards against
    /// that case.
    static INIT: AtomicBool = AtomicBool::new(false);
    /// The producer holds encoded defmt frames.
    ///
    /// Assume initialized when INIT is true.
    static mut PRODUCER: MaybeUninit<Producer> = MaybeUninit::uninit();

    pub fn init(producer: Producer) {
        // Safety: module inspection shows that this function is only called once,
        // so we're not changing the pointed-to value while another accessor is
        // looking at this memory.
        unsafe { PRODUCER.write(producer) };
        INIT.store(true, Ordering::Release);
    }

    pub fn acquire<R>(func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R) -> Option<R> {
        INIT.load(Ordering::Acquire).then(|| {
            critical_section::acquire();
            // Safety:
            // - the memory is initialized because INIT is true.
            // - we have exclusive access due to acquire.
            func(unsafe { PRODUCER.assume_init_mut() }, encoder())
        })
    }

    pub fn release<R>(func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R) -> Option<R> {
        INIT.load(Ordering::Acquire).then(|| {
            // Safety:
            // - the memory is initialized.
            // - the defmt user can only call this after calling
            //   acquire, so this is the only mutable access.
            let result = func(unsafe { PRODUCER.assume_init_mut() }, encoder());
            critical_section::release();
            result
        })
    }

    pub fn write<R>(func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R) -> Option<R> {
        INIT.load(Ordering::Acquire).then(|| {
            // Safety:
            // - the memory is initialized because INIT is true.
            // - the defmt user can only call this after calling
            //   acquire, so there's only one mutable access.
            func(unsafe { PRODUCER.assume_init_mut() }, encoder())
        })
    }

    /// Manages critical section.
    ///
    /// Not necessarily sufficient for any multi-core i.MX RT chip variant. Might
    /// work if the program is guaranteed to only run on one core (so no shared
    /// memory across the cores). Implementation modeled after cortex_m::interrupt::free.
    ///
    /// The critical section is coupled to the producer. A previous implementation
    /// tied together this critical_section and producer module inside the logger,
    /// like
    ///
    /// ```text
    /// impl defmt::Logger for Logger {
    ///     fn acquire() {
    ///         producer::try(|p| {
    ///             critical_section::acquire();
    ///             // Use p...
    ///         })
    ///     }
    ///
    ///     // ...
    /// }
    /// ```
    ///
    /// If two callers call acquire() and enter the producer::try closure, we have two live
    /// mutable references. I think _producing_ those two mutable references is a
    /// no-no, even if higher-level parts of the system prevent that from happening.
    /// So, this code is structured specifically to prevent that.
    mod critical_section {
        use core::sync::atomic::{AtomicBool, Ordering};
        static HAD_INTERRUPTS: AtomicBool = AtomicBool::new(false);

        pub fn acquire() {
            let had_interrupts = cortex_m::register::primask::read().is_active();
            HAD_INTERRUPTS.store(had_interrupts, Ordering::Release);
            cortex_m::interrupt::disable();
        }

        pub fn release() {
            if HAD_INTERRUPTS.swap(false, Ordering::Acquire) {
                // Safety: intended to re-enable interrupts at the release
                // of the defmt critical section.
                unsafe {
                    cortex_m::interrupt::enable();
                }
            }
        }
    }

    /// Access the defmt log frame encoder.
    fn encoder() -> &'static mut defmt::Encoder {
        static mut ENCODER: defmt::Encoder = defmt::Encoder::new();
        // Safety: memory is statically initialized. Inspection of this
        // module shows that this is always accessed within the acquire-
        // release critical section, so there's only one access to this
        // mutable data.
        unsafe { &mut ENCODER }
    }
}

#[defmt::global_logger]
struct Logger;

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        frontend::acquire(|producer, encoder| {
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
        frontend::release(|producer, encoder| {
            encoder.end_frame(|buffer| {
                let _ = crate::try_write_producer(buffer, producer);
            });
        });
    }

    unsafe fn write(bytes: &[u8]) {
        frontend::write(|producer, encoder| {
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
pub fn usb_with_config<const N: u8>(
    peripherals: imxrt_usbd::Instances<N>,
    interrupts: super::Interrupts,
    backend_config: &crate::UsbdConfig,
) -> Result<crate::Poller, crate::AlreadySetError<imxrt_usbd::Instances<N>>> {
    let (producer, consumer) = match crate::BUFFER.try_split() {
        Ok((prod, cons)) => (prod, cons),
        Err(_) => return Err(crate::AlreadySetError::new(peripherals)),
    };

    cortex_m::interrupt::free(|_| {
        frontend::init(producer);
        // Safety: BUFFER.try_split() guarantees that this is only called once.
        unsafe { crate::usbd::init(peripherals, interrupts, consumer, backend_config) };
        Ok(crate::Poller::new(crate::usbd::VTABLE))
    })
}

/// Initialize a USB logger with the `defmt` frontend.
///
/// This function uses default configurations for the backend.
/// See the crate-level documentation to understand how the USB device backend works.
#[cfg(feature = "usbd")]
pub fn usbd<const N: u8>(
    peripherals: imxrt_usbd::Instances<N>,
    interrupts: super::Interrupts,
) -> Result<crate::Poller, crate::AlreadySetError<imxrt_usbd::Instances<N>>> {
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

    cortex_m::interrupt::free(|_| {
        frontend::init(producer);
        unsafe { crate::lpuart::init(lpuart, dma_channel, consumer, interrupts) };
        Ok(crate::Poller::new(crate::lpuart::VTABLE))
    })
}
