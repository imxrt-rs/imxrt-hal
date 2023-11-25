use crate::lpspi::word_types::LpspiDataBuffer;

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
        todo!()
    }

    fn write(&mut self, words: &[T]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer(&mut self, read: &mut [T], write: &[T]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer_in_place(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.block_until_finished()
    }
}

// Async only makes sense for DMA; DMA only supports u32.
#[cfg(feature = "async")]
impl<const N: u8> eh1_async::spi::SpiBus<u32> for Lpspi<'_, N, FullDma> {
    async fn read(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer(&mut self, read: &mut [u32], write: &[u32]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer_in_place(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
