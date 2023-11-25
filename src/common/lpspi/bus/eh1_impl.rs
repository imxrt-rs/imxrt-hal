use cassette::Cassette;

use crate::lpspi::data_buffer::{LpspiDataBuffer, TransferBuffer};

use super::{FullDma, Lpspi, LpspiError};

impl<const N: u8, DMA> eh1::spi::ErrorType for Lpspi<'_, N, DMA> {
    type Error = LpspiError;
}

impl<const N: u8, DMA, T> eh1::spi::SpiBus<T> for Lpspi<'_, N, DMA>
where
    T: 'static + Copy,
    [T]: LpspiDataBuffer,
{
    fn read(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(
            self.transfer(TransferBuffer::Dual(words, &[])),
        ))
        .block_on()
    }

    fn write(&mut self, words: &[T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(
            self.transfer(TransferBuffer::Dual(&mut [], words))
        ))
        .block_on()
    }

    fn transfer(&mut self, read: &mut [T], write: &[T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(
            self.transfer(TransferBuffer::Dual(read, write))
        ))
        .block_on()
    }

    fn transfer_in_place(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(self.transfer(TransferBuffer::Single(words)))).block_on()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // Nothing to do, all other calls only return after the device is finished
        Ok(())
    }
}

// Async only makes sense for DMA; DMA only supports u32.
#[cfg(feature = "async")]
impl<const N: u8> eh1_async::spi::SpiBus<u32> for Lpspi<'_, N, FullDma> {
    async fn read(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        self.transfer(TransferBuffer::Dual(words, &[])).await
    }

    async fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        self.transfer(TransferBuffer::Dual(&mut [], words)).await
    }

    async fn transfer(&mut self, read: &mut [u32], write: &[u32]) -> Result<(), Self::Error> {
        self.transfer(TransferBuffer::Dual(read, write)).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        self.transfer(TransferBuffer::Single(words)).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        // Nothing to do, all other calls only return after the device is finished
        Ok(())
    }
}
