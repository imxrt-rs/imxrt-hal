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

/// The DMA driver.
///
/// This DMA driver is configured for your chip. You could use it to allocate
/// channels; however, it's safer to use [`channels()`] to acquire your DMA
/// channels.
///
/// This driver provides access to the wakers that are provided to DMA futures.
/// If you're implementing an async runtime, you should use this object to wake
/// DMA channel wakers on interrupt.
// Safety: pointers come from RAL, and are correct for the selected chip.
// DMA channel count is also valid for the chip selection.
pub static DMA: crate::common::dma::Dma<{ CHANNEL_COUNT }> = unsafe {
    crate::common::dma::Dma::new(
        crate::ral::dma::DMA.cast(),
        crate::ral::dmamux::DMAMUX.cast(),
    )
};

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
        // Safety: we own the DMA instances, so we're OK to fabricate the channels.
        // It would be unsafe for the user to subsequently access the DMA instances.
        let mut chan = unsafe { DMA.channel(idx) };
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

    pub(super) const ADC_DMA_RX_MAPPING: [u32; 2] = [24, 88];
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

// Safety: a LPUART can support writes from a DMA engine into its data register.
// The peripheral is static, so it's always a valid target for memory writes.
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

// Safety: a LPUART can support reads performed by a DMA engine from its data
// register. The peripheral is static and always valid for reading.
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
    ) -> peripheral::Write<'a, Self, u8> {
        peripheral::write(channel, buffer, self)
    }

    /// Use a DMA channel to read data from the UART peripheral
    ///
    /// Completes when `buffer` is filled.
    pub fn dma_read<'a>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a mut [u8],
    ) -> peripheral::Read<'a, Self, u8> {
        peripheral::read(channel, self, buffer)
    }
}

// LPSPI
use crate::lpspi;

// Safety: a LPSPI can provide data for a DMA transfer. Its receive data register
// points to static memory.
unsafe impl<P, const N: u8> peripheral::Source<u32> for lpspi::Lpspi<P, N> {
    fn source_signal(&self) -> u32 {
        LPSPI_DMA_RX_MAPPING[N as usize - 1]
    }
    fn source_address(&self) -> *const u32 {
        self.rdr().cast()
    }
    fn enable_source(&mut self) {
        self.enable_dma_receive()
    }
    fn disable_source(&mut self) {
        self.disable_dma_receive();
    }
}

// Safety: a LPSPI can receive data for a DMA transfer. Its transmit data register
// points to static memory.
unsafe impl<P, const N: u8> peripheral::Destination<u32> for lpspi::Lpspi<P, N> {
    fn destination_signal(&self) -> u32 {
        LPSPI_DMA_TX_MAPPING[N as usize - 1]
    }
    fn destination_address(&self) -> *const u32 {
        self.tdr().cast()
    }
    fn enable_destination(&mut self) {
        self.enable_dma_transmit();
    }
    fn disable_destination(&mut self) {
        self.disable_dma_transmit();
    }
}

// Safety: a LPSPI can perform bi-directional I/O from a single buffer. Reads from
// the buffer are always performed before writes.
unsafe impl<P, const N: u8> peripheral::Bidirectional<u32> for lpspi::Lpspi<P, N> {}

impl<P, const N: u8> lpspi::Lpspi<P, N> {
    /// Use a DMA channel to write data to the LPSPI peripheral.
    ///
    /// The future completes when all data in `buffer` has been written to the
    /// peripheral. This call may block until space is available in the
    /// command queue. An error indicates that there was an issue preparing the
    /// transaction, or there was an issue while waiting for space in the command
    /// queue.
    pub fn dma_write<'a>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a [u32],
    ) -> Result<peripheral::Write<'a, Self, u32>, lpspi::LpspiError> {
        let mut transaction = self.bus_transaction(buffer)?;
        transaction.receive_data_mask = true;

        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::write(channel, buffer, self))
    }

    /// Use a DMA channel to read data from the LPSPI peripheral.
    ///
    /// The future completes when `buffer` is filled. This call may block until
    /// space is available in the command queue. An error indicates that there was
    /// an issue preparing the transaction, or there was an issue waiting for space
    /// in the command queue.
    pub fn dma_read<'a>(
        &'a mut self,
        channel: &'a mut Channel,
        buffer: &'a mut [u32],
    ) -> Result<peripheral::Read<'a, Self, u32>, lpspi::LpspiError> {
        let mut transaction = self.bus_transaction(buffer)?;
        transaction.transmit_data_mask = true;

        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::read(channel, self, buffer))
    }

    /// Use a DMA channel to simultaneously read and write from a buffer
    /// and the LPSPI peripheral.
    ///
    /// The future completes when `buffer` is filled and after sending `buffer` elements.
    /// This call may block until space is available in the command queue. An error
    /// indicates that there was an issue preparing the transaction, or there was an
    /// issue waiting for space in the command queue.
    pub fn dma_full_duplex<'a>(
        &'a mut self,
        rx: &'a mut Channel,
        tx: &'a mut Channel,
        buffer: &'a mut [u32],
    ) -> Result<peripheral::FullDuplex<'a, Self, u32>, lpspi::LpspiError> {
        let transaction = self.bus_transaction(buffer)?;

        self.wait_for_transmit_fifo_space()?;
        self.enqueue_transaction(&transaction);
        Ok(peripheral::full_duplex(rx, tx, self, buffer))
    }
}

// ADC
#[cfg(family = "imxrt10xx")]
use crate::adc;

#[cfg(family = "imxrt10xx")]
// Safety: an ADC source adapter points to a static register that's always valid
// for reads.
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
