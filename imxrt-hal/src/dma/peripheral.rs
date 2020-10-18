//! Supporting traits for defining peripheral DMA
//! sources and destinations
//!
//! # How to add DMA capabilities to your peripheral
//!
//! 1. Make sure your device can support DMA transfers!
//! 2. If your device can either produce data for a DMA transfer, or
//!    accept data from a DMA transfer, implement the `private::Sealed`
//!    trait for your peripheral *in the `private` module*.
//! 3. If your device can produce data for a DMA transfer, implement the
//!    `Source` trait for your peripheral *in your peripheral's module*.
//! 4. If your device can accept data from a DMA transfer, implement the
//!    `Destination` trait for your peripheral *in your peripheral's module*.
//!
//! See the [Rust API guidelines on future-proofing](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
//! to learn about the 'Sealed' pattern. Use the UART peripheral as an example.

use super::{buffer, Channel, Circular, Element, Error, ReadHalf, Transfer, WriteHalf};
use core::sync::atomic::{compiler_fence, Ordering};
pub use imxrt_dma::{Destination, Source};

/// A DMA-capable peripheral
///
/// `Peripheral` wraps an object that can act as a source and / or destination
/// for a DMA transfer. It provides an interface for scheduling transfers, and
/// for knowing when transfers are complete.
///
/// Before constructing a `Peripheral`, you should configure the [`Channel`](struct.Channel.html)
/// as necessary. If you enable interrupts, you're responsible for registering the
/// interrupt, and for clearing the interrupt. The `Peripheral` has methods for clearing interrupts
/// due to transfer and receive DMA channels.
///
/// See the [module-level docs](index.html#example-full-duplex-spi-peripheral)
/// for an example of how to create and use a peripheral.
pub struct Peripheral<P, E, S, D = S> {
    /// Channel used for outgoing data (from software, to external device)
    tx_channel: Option<Channel>,
    /// Channel used for incoming data (from external device, to software)
    rx_channel: Option<Channel>,
    /// The peripheral that is either providing the data, or accepting the data,
    /// or both.
    peripheral: P,
    _element: core::marker::PhantomData<E>,
    /// The buffer that used to send data in a DMA transfer
    source_buffer: Option<S>,
    /// The buffer that's used to receive data in a DMA transfer
    destination_buffer: Option<D>,
}

impl<P, E, S, D> Peripheral<P, E, S, D> {
    fn new(peripheral: P) -> Self {
        Peripheral {
            peripheral,
            rx_channel: None,
            tx_channel: None,
            _element: core::marker::PhantomData,
            source_buffer: None,
            destination_buffer: None,
        }
    }
}

impl<P, E, S, D> Peripheral<P, E, S, D>
where
    P: Source<E>,
    E: Element,
    D: buffer::Destination<E>,
{
    /// Wraps a peripheral that can act as the source of a DMA transfer
    pub fn new_receive(source: P, channel: Channel) -> Self {
        let mut peripheral = Peripheral::new(source);
        peripheral.init_receive(channel);
        peripheral
    }

    fn init_receive(&mut self, mut channel: Channel) {
        channel.set_trigger_from_hardware(Some(P::SOURCE_REQUEST_SIGNAL));
        // Safety: Source trait is only implemented on peripherals within
        // this crate. We may study those implementations to show that the
        // pointers point to valid memory.
        unsafe {
            channel.set_source_transfer(&Transfer::hardware(self.peripheral.source()));
        }
        channel.set_disable_on_completion(true);
        self.rx_channel = Some(channel);
    }

    /// Start a DMA transfer that transfers data from the peripheral into the supplied buffer
    ///
    /// A complete transfer is signaled by `is_receive_complete()`, and possibly an interrupt.
    pub fn start_receive(&mut self, mut buffer: D) -> Result<(), (D, Error)> {
        let rx_channel = self.rx_channel.as_mut().unwrap();
        if rx_channel.is_enabled() {
            return Err((buffer, Error::ScheduledTransfer));
        }

        let dst = buffer.destination();

        unsafe {
            rx_channel.set_destination_transfer(&dst);
        }
        rx_channel.set_minor_loop_elements::<E>(1);
        rx_channel.set_transfer_iterations(buffer.destination_len() as u16);

        buffer.prepare_destination();

        compiler_fence(Ordering::Release);
        unsafe {
            rx_channel.enable();
        }
        if rx_channel.is_error() {
            let es = rx_channel.error_status();
            rx_channel.clear_error();
            Err((buffer, Error::Setup(es)))
        } else {
            self.destination_buffer = Some(buffer);
            Ok(())
        }
    }

    /// Returns `true` if the receive is complete
    pub fn is_receive_complete(&self) -> bool {
        self.rx_channel.as_ref().unwrap().is_complete()
    }

    /// Clears the flag that indicates the DMA transfer is complete, and
    /// disable the peripheral.
    ///
    /// Users are **required** to call this to disable the source. Otherwise,
    /// the source may continue to generate DMA requests.
    pub fn receive_complete(&mut self) -> Option<D> {
        self.rx_channel.as_mut().unwrap().clear_complete();
        self.peripheral.disable_source();
        self.destination_buffer.take().map(|mut buffer| {
            buffer.complete_destination();
            buffer
        })
    }

    /// Indicates if the receive channel has generated an interrupt
    pub fn is_receive_interrupt(&self) -> bool {
        self.rx_channel.as_ref().unwrap().is_interrupt()
    }

    /// Clears the interrupt flag on the receive channel
    ///
    /// Users are **required** to clear the interrupt flag, or the hardware
    /// may continue to generate interrupts for the channel. This must be called
    /// for completion interrupts and half-transfer interrupts.
    pub fn receive_clear_interrupt(&mut self) {
        self.rx_channel.as_mut().unwrap().clear_interrupt()
    }

    /// Cancel a receive transfer
    pub fn receive_cancel(&mut self) -> Option<D> {
        self.peripheral.disable_source();
        let rx_channel = self.rx_channel.as_mut().unwrap();
        while rx_channel.is_hardware_signaling() {
            core::sync::atomic::spin_loop_hint();
        }
        rx_channel.disable();
        compiler_fence(Ordering::Acquire);
        self.destination_buffer.take()
    }

    /// Release the peripheral and the channel
    ///
    /// Users should ensure that any started transfer has completed. If the
    /// `Peripheral` was constructed with [`new_bidirectional()`](struct.Peripheral.html#method.new_bidirectional),
    /// callers should use [`bidirectional_release()`](struct.Peripheral.html#method.bidirectional_release);
    /// otherwise, the transfer channel will be dropped when this method returns.
    pub fn receive_release(mut self) -> (P, Channel) {
        (self.peripheral, self.rx_channel.take().unwrap())
    }

    /// Indicates if the DMA controller is actively moving data for this DMA request
    ///
    /// It may not be moving data if
    ///
    /// - the transfer is complete, or there is no transfer
    /// - the transfer is preempted
    pub fn is_receive_active(&self) -> bool {
        self.rx_channel.as_ref().unwrap().is_active()
    }
}

impl<P, E, S> Peripheral<P, E, S, Circular<E>>
where
    P: Source<E>,
    E: Element,
{
    /// Returns the read half of the circular buffer that's being
    /// used as a DMA transfer destination
    ///
    /// Returns `None` if there's no destination buffer, which may mean
    /// that there's no active transfer.
    pub fn read_half(&mut self) -> Option<ReadHalf<E>> {
        self.destination_buffer.as_mut().map(ReadHalf::new)
    }
}

impl<P, E, S, D> Peripheral<P, E, S, D>
where
    P: Destination<E>,
    E: Element,
    S: buffer::Source<E>,
{
    /// Wraps a peripheral that can act as the destination of a DMA transfer
    pub fn new_transfer(destination: P, channel: Channel) -> Self {
        let mut peripheral = Peripheral::new(destination);
        peripheral.init_transfer(channel);
        peripheral
    }

    fn init_transfer(&mut self, mut channel: Channel) {
        channel.set_trigger_from_hardware(Some(P::DESTINATION_REQUEST_SIGNAL));
        // Safety: Destination trait is only implemented on peripherals within
        // this crate. We may study those implementations to show that the pointers
        // point to valid memory.
        unsafe {
            channel.set_destination_transfer(&Transfer::hardware(self.peripheral.destination()));
        }
        channel.set_disable_on_completion(true);
        self.tx_channel = Some(channel);
    }

    /// Start a DMA transfer that transfers data from the supplied buffer to the peripheral
    ///
    /// A complete transfer is signaled by `is_transfer_complete()`, and possibly an interrupt.
    pub fn start_transfer(&mut self, mut buffer: S) -> Result<(), (S, Error)> {
        let tx_channel = self.tx_channel.as_mut().unwrap();
        if tx_channel.is_enabled() {
            return Err((buffer, Error::ScheduledTransfer));
        }

        let src = buffer.source();

        unsafe {
            tx_channel.set_source_transfer(&src);
        }
        tx_channel.set_minor_loop_elements::<E>(1);
        tx_channel.set_transfer_iterations(buffer.source_len() as u16);

        buffer.prepare_source();

        compiler_fence(Ordering::Release);
        unsafe {
            tx_channel.enable();
        }
        if tx_channel.is_error() {
            let es = tx_channel.error_status();
            tx_channel.clear_error();
            Err((buffer, Error::Setup(es)))
        } else {
            self.source_buffer = Some(buffer);
            Ok(())
        }
    }

    /// Returns `true` if the transfer is complete
    pub fn is_transfer_complete(&self) -> bool {
        self.tx_channel.as_ref().unwrap().is_complete()
    }

    /// Clears the flag that indicates the DMA transfer is complete, and
    /// disable the peripheral.
    ///
    /// Users are **required** to call this to disable the source. Otherwise,
    /// the source may continue to generate DMA requests.
    pub fn transfer_complete(&mut self) -> Option<S> {
        self.tx_channel.as_mut().unwrap().clear_complete();
        self.peripheral.disable_destination();
        self.source_buffer.take().map(|mut buffer| {
            buffer.complete_source();
            buffer
        })
    }

    /// Indicates if the transfer channel has generated an interrupt
    pub fn is_transfer_interrupt(&self) -> bool {
        self.tx_channel.as_ref().unwrap().is_interrupt()
    }

    /// Clears the interrupt flag on the transfer channel
    ///
    /// Users are **required** to clear the interrupt flag, or the hardware
    /// may continue to generate interrupts for the channel. This must be called
    /// for completion interrupts and half-transfer interrupts.
    pub fn transfer_clear_interrupt(&mut self) {
        self.tx_channel.as_mut().unwrap().clear_interrupt()
    }

    /// Cancel a transfer that sends data to the peripheral
    pub fn transfer_cancel(&mut self) -> Option<S> {
        self.peripheral.disable_destination();
        let tx_channel = self.tx_channel.as_mut().unwrap();
        while tx_channel.is_hardware_signaling() {
            core::sync::atomic::spin_loop_hint();
        }
        tx_channel.disable();
        compiler_fence(Ordering::Acquire);
        self.source_buffer.take()
    }

    /// Release the peripheral and the channel
    ///
    /// Users should ensure that any started transfer has completed. If the
    /// `Peripheral` was constructed with [`new_bidirectional()`](struct.Peripheral.html#method.new_bidirectional),
    /// callers should use [`bidirectional_release()`](struct.Peripheral.html#method.bidirectional_release);
    /// otherwise, the receiver channel will be dropped when this method returns.
    pub fn transfer_release(mut self) -> (P, Channel) {
        (self.peripheral, self.tx_channel.take().unwrap())
    }

    /// Indicates if the DMA controller is actively moving data for this DMA request
    ///
    /// It may not be moving data if
    ///
    /// - the transfer is complete, or there is no transfer
    /// - the transfer is preempted
    pub fn is_transfer_active(&self) -> bool {
        self.rx_channel.as_ref().unwrap().is_active()
    }
}

impl<P, E, D> Peripheral<P, E, Circular<E>, D>
where
    P: Destination<E>,
    E: Element,
{
    /// Returns the write half of the circular buffer that's being
    /// used as a DMA transfer source.
    ///
    /// Returns `None` if there's no source buffer, which may mean
    /// that there's no active transfer.
    ///
    /// You may use the [`WriteHalf`](struct.WriteHalf.html) to prepare
    /// data for another DMA transfer.
    pub fn write_half(&mut self) -> Option<WriteHalf<E>> {
        self.source_buffer.as_mut().map(WriteHalf::new)
    }
}

impl<P, E, S, D> Peripheral<P, E, S, D>
where
    P: Source<E> + Destination<E>,
    E: Element,
    S: buffer::Source<E>,
    D: buffer::Destination<E>,
{
    /// Wraps a peripheral that can act as both the source and destination of a DMA transfer
    pub fn new_bidirectional(peripheral: P, tx: Channel, rx: Channel) -> Self {
        let mut peripheral = Peripheral::new(peripheral);
        peripheral.init_receive(rx);
        peripheral.init_transfer(tx);
        peripheral
    }

    /// Release the peripheral and both channels (transfer channel, release channel)
    ///
    /// Users should ensure that any active transfers are complete before releasing the
    /// peripheral.
    pub fn bidirectional_release(mut self) -> (P, (Channel, Channel)) {
        (
            self.peripheral,
            (
                self.tx_channel.take().unwrap(),
                self.rx_channel.take().unwrap(),
            ),
        )
    }
}

/// Helper functions for constructing `Peripheral`s
pub mod helpers {
    use super::{buffer, Channel, Destination, Peripheral, Source};

    /// Create a peripheral that can suppy `u8` data for DMA transfers
    pub fn receive_u8<P, B>(source: P, channel: Channel) -> Peripheral<P, u8, B>
    where
        P: Source<u8>,
        B: buffer::Destination<u8>,
    {
        Peripheral::new_receive(source, channel)
    }

    /// Create a peripheral that can supply `u16` data for DMA transfers
    pub fn receive_u16<P, B>(source: P, channel: Channel) -> Peripheral<P, u16, B>
    where
        P: Source<u16>,
        B: buffer::Destination<u16>,
    {
        Peripheral::new_receive(source, channel)
    }

    /// Create a peripheral that can accept `u8` data from DMA transfers
    pub fn transfer_u8<P, B>(destination: P, channel: Channel) -> Peripheral<P, u8, B>
    where
        P: Destination<u8>,
        B: buffer::Source<u8>,
    {
        Peripheral::new_transfer(destination, channel)
    }

    /// Create a peripheral that can accept `u16` data from DMA transfers
    pub fn transfer_u16<P, B>(destination: P, channel: Channel) -> Peripheral<P, u16, B>
    where
        P: Destination<u16>,
        B: buffer::Source<u16>,
    {
        Peripheral::new_transfer(destination, channel)
    }

    /// Create a peripheral that can accept `u8` data from DMA transfers, and can
    /// source `u8` data for DMA transfers
    pub fn bidirectional_u8<P, S, D>(
        peripheral: P,
        tx: Channel,
        rx: Channel,
    ) -> Peripheral<P, u8, S, D>
    where
        P: Source<u8> + Destination<u8>,
        S: buffer::Source<u8>,
        D: buffer::Destination<u8>,
    {
        Peripheral::new_bidirectional(peripheral, tx, rx)
    }

    /// Create a peripheral that can accept `u16` data from DMA transfers, and can
    /// source `u16` data for DMA transfers
    pub fn bidirectional_u16<P, S, D>(
        peripheral: P,
        tx: Channel,
        rx: Channel,
    ) -> Peripheral<P, u16, S, D>
    where
        P: Source<u16> + Destination<u16>,
        S: buffer::Source<u16>,
        D: buffer::Destination<u16>,
    {
        Peripheral::new_bidirectional(peripheral, tx, rx)
    }
}
