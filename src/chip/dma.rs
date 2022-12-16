//! Chip-specific DMA APIs.

use crate::ral;

use crate::common::dma::channel::Channel;

/// The total number of DMA channels.
///
/// This is 16 the minumum number of DMA channels available for all
/// i.MX RT processors. However, if you've enabled a chip family feature
/// and that chip family has more than 16 DMA channels, this value may
/// increase.
pub const CHANNEL_COUNT: usize = crate::chip::config::DMA_CHANNEL_COUNT;

/// Allocate all DMA channels.
///
/// The number of channels depends on [`CHANNEL_COUNT`], which may change
/// depending on feature selection.
///
/// When `channels` returns, each element is guaranteed to hold `Some` channel.
/// You may then `take()` the channel, leaving `None` in its place.
pub fn channels(_: ral::dma::DMA, _: ral::dmamux::DMAMUX) -> [Option<Channel>; CHANNEL_COUNT] {
    const NO_CHANNEL: Option<Channel> = None;
    let mut channels: [Option<Channel>; CHANNEL_COUNT] = [NO_CHANNEL; CHANNEL_COUNT];

    for (idx, channel) in channels.iter_mut().enumerate() {
        // Safety: own the DMA instances, so we're OK to fabricate the channels.
        // It would be unsafe for the user to subsequently access the DMA instances.
        let mut chan = unsafe { Channel::new(idx) };
        chan.reset();
        *channel = Some(chan);
    }
    channels
}

//
// Peripheral implementations.
//
// These depend on DMA MUX peripheral mappings, which are chip (family) specific.
//
use crate::dma::peripheral;

#[cfg(family = "imxrt10xx")]
mod mappings {
    pub(super) const LPUART_DMA_RX_MAPPING: [u32; 8] = [3, 67, 5, 69, 7, 71, 9, 73];
    pub(super) const LPUART_DMA_TX_MAPPING: [u32; 8] = [2, 66, 4, 68, 6, 70, 8, 72];

    pub(super) const LPSPI_DMA_RX_MAPPING: [u32; 4] = [13, 77, 15, 79];
    pub(super) const LPSPI_DMA_TX_MAPPING: [u32; 4] = [14, 78, 16, 80];

    pub(super) const ADC_DMA_RX_MAPPING: [u32; 2] = [32, 88];
}
#[cfg(family = "imxrt11xx")]
mod mappings {
    pub(super) const LPUART_DMA_RX_MAPPING: [u32; 12] =
        [9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31];
    pub(super) const LPUART_DMA_TX_MAPPING: [u32; 12] =
        [8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];

    pub(super) const LPSPI_DMA_RX_MAPPING: [u32; 6] = [36, 38, 40, 42, 44, 46];
    pub(super) const LPSPI_DMA_TX_MAPPING: [u32; 6] = [37, 39, 41, 43, 45, 47];
}
use mappings::*;

// LPUART
use crate::lpuart;

unsafe impl<P, const N: u8> peripheral::Destination<u8> for lpuart::Lpuart<P, N> {
    fn destination_signal(&self) -> u32 {
        LPUART_DMA_TX_MAPPING[N as usize - 1]
    }
    fn destination_address(&self) -> *const u8 {
        self.data().cast()
    }
    fn enable_destination(&mut self) {
        self.enable_dma_transmit();
    }
    fn disable_destination(&mut self) {
        self.disable_dma_transmit();
    }
}

unsafe impl<P, const N: u8> peripheral::Source<u8> for lpuart::Lpuart<P, N> {
    fn source_signal(&self) -> u32 {
        LPUART_DMA_RX_MAPPING[N as usize - 1]
    }
    fn source_address(&self) -> *const u8 {
        self.data().cast()
    }
    fn enable_source(&mut self) {
        self.enable_dma_receive();
    }
    fn disable_source(&mut self) {
        self.disable_dma_receive();
    }
}

impl<P, const N: u8> lpuart::Lpuart<P, N> {
    /// Use a DMA channel to write data to the UART peripheral
    ///
    /// Completes when all data in `buffer` has been written to the UART
    /// peripheral.
    pub fn dma_write<'a>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a [u8],
    ) -> peripheral::Tx<'a, Self, u8> {
        peripheral::transfer(channel, buffer, self)
    }

    /// Use a DMA channel to read data from the UART peripheral
    ///
    /// Completes when `buffer` is filled.
    pub fn dma_read<'a>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a mut [u8],
    ) -> peripheral::Rx<'a, Self, u8> {
        peripheral::receive(channel, self, buffer)
    }
}

// LPSPI
use crate::lpspi;

unsafe impl<E, P, const N: u8> peripheral::Source<E> for lpspi::Lpspi<P, N>
where
    E: lpspi::DmaElement,
{
    fn source_signal(&self) -> u32 {
        LPSPI_DMA_RX_MAPPING[N as usize - 1]
    }
    fn source_address(&self) -> *const E {
        self.rdr().cast()
    }
    fn enable_source(&mut self) {
        self.enable_dma_receive()
    }
    fn disable_source(&mut self) {
        self.disable_dma_receive();
    }
}

unsafe impl<E, P, const N: u8> peripheral::Destination<E> for lpspi::Lpspi<P, N>
where
    E: lpspi::DmaElement,
{
    fn destination_signal(&self) -> u32 {
        LPSPI_DMA_TX_MAPPING[N as usize - 1]
    }
    fn destination_address(&self) -> *const E {
        self.tdr().cast()
    }
    fn enable_destination(&mut self) {
        self.enable_dma_transmit();
    }
    fn disable_destination(&mut self) {
        self.disable_dma_transmit();
    }
}

unsafe impl<E, P, const N: u8> peripheral::Bidirectional<E> for lpspi::Lpspi<P, N> where
    E: lpspi::DmaElement
{
}

impl<P, const N: u8> lpspi::Lpspi<P, N> {
    /// Use a DMA channel to write data to the LPSPI peripheral.
    ///
    /// The future completes when all data in `buffer` has been written to the
    /// peripheral. This call may block until space is available in the
    /// command queue. An error indicates that there was an issue preparing the
    /// transaction, or there was an issue while waiting for space in the command
    /// queue.
    pub fn dma_write<'a, E: lpspi::DmaElement>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a [E],
    ) -> Result<peripheral::Tx<'a, Self, E>, lpspi::LpspiError> {
        let mut transaction = self.prepare_transaction(buffer)?;
        transaction.disable_receive();
        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::transfer(channel, buffer, self))
    }

    /// Use a DMA channel to read data from the LPSPI peripheral.
    ///
    /// The future completes when `buffer` is filled. This call may block until
    /// space is available in the command queue. An error indicates that there was
    /// an issue preparing the transaction, or there was an issue waiting for space
    /// in the command queue.
    pub fn dma_read<'a, E: lpspi::DmaElement>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a mut [E],
    ) -> Result<peripheral::Rx<'a, Self, E>, lpspi::LpspiError> {
        let mut transaction = self.prepare_transaction(buffer)?;
        transaction.disable_transmit();
        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::receive(channel, self, buffer))
    }

    /// Use a DMA channel to simultaneously read and write from a buffer
    /// and the LPSPI peripheral.
    ///
    /// The future completes when `buffer` is filled and after sending `buffer` elements.
    /// This call may block until space is available in the command queue. An error
    /// indicates that there was an issue preparing the transaction, or there was an
    /// issue waiting for space in the command queue.
    pub fn dma_full_duplex<'a, E: lpspi::DmaElement>(
        &'a mut self,
        rx: &'a mut Channel,
        tx: &'a mut Channel,
        buffer: &'a mut [E],
    ) -> Result<peripheral::FullDuplex<'a, Self, E>, lpspi::LpspiError> {
        let transaction = self.prepare_transaction(buffer)?;
        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::full_duplex(rx, tx, self, buffer))
    }
}

// ADC
#[cfg(family = "imxrt10xx")]
use crate::adc;

#[cfg(family = "imxrt10xx")]
unsafe impl<P, const N: u8> peripheral::Source<u16> for adc::DmaSource<P, N> {
    fn source_signal(&self) -> u32 {
        ADC_DMA_RX_MAPPING[if N == ral::SOLE_INSTANCE {
            N as usize
        } else {
            N as usize - 1
        }]
    }
    fn source_address(&self) -> *const u16 {
        self.r0().cast()
    }
    fn enable_source(&mut self) {
        self.enable_dma();
    }
    fn disable_source(&mut self) {
        self.disable_dma();
    }
}
