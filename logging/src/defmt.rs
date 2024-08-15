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
    use critical_section::RestoreState;

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

    /// The restore state for the critical section.
    static mut RESTORE_STATE: RestoreState = RestoreState::invalid();

    pub fn init(producer: Producer) {
        // Safety: module inspection shows that this function is only called once,
        // so we're not changing the pointed-to value while another accessor is
        // looking at this memory.
        unsafe { PRODUCER.write(producer) };
        INIT.store(true, Ordering::Release);
    }

    /// Try to acquire the logger, skipping everything if the logger isn't initialized.
    pub fn acquire<R>(func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R) -> Option<R> {
        INIT.load(Ordering::Acquire).then(|| {
            // Safety: we're relying on the defmt::Logger user to meet some of these
            // critical section guarantees. We're also relying on the firmware developer
            // to either
            //
            // - build separate binaries for multi-core systems
            // - select a proper critical_section implementation for those multi-core systems
            //   that needed to share the logging queue.
            unsafe { RESTORE_STATE = critical_section::acquire() };

            // Safety:
            // - the memory is initialized because INIT is true.
            // - we have exclusive access due to acquire.
            func(unsafe { PRODUCER.assume_init_mut() }, encoder())
        })
    }

    /// Try to release the logger, skipping everything if the loger isn't initialized.
    pub fn release<R>(func: impl FnOnce(&mut Producer, &mut defmt::Encoder) -> R) -> Option<R> {
        INIT.load(Ordering::Acquire).then(|| {
            // Safety:
            // - the memory is initialized.
            // - the defmt user can only call this after calling
            //   acquire, so this is the only mutable access.
            let result = func(unsafe { PRODUCER.assume_init_mut() }, encoder());

            // Safety: we're relying on the defmt::Logger user to meet some critical
            // section guarantees. See the related safety comment on acquire.
            unsafe { critical_section::release(RESTORE_STATE) };
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

    /// Access the defmt log frame encoder.
    fn encoder() -> &'static mut defmt::Encoder {
        static mut ENCODER: defmt::Encoder = defmt::Encoder::new();
        // Safety: memory is statically initialized. Inspection of this
        // module shows that this is always accessed within the acquire-
        // release critical section, so there's only one access to this
        // mutable data.
        unsafe { &mut *core::ptr::addr_of_mut!(ENCODER) }
    }
}

#[defmt::global_logger]
struct Logger;

// Safety: we rely on a critical section acquire in order to meet the
// defmt::Logger safety contract.
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
        frontend::init(producer);
        // Safety: "called once" requirement met by the buffer split, above.
        // Only the first successful split flows into this call.
        unsafe { crate::lpuart::init(lpuart, dma_channel, consumer, interrupts) };
        Ok(crate::Poller::new(crate::lpuart::VTABLE))
    })
}
