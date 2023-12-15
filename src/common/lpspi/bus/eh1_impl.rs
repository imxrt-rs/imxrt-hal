use cassette::Cassette;

use crate::lpspi::transfer_actions::{
    create_actions_read_write, create_actions_read_write_in_place,
};

use super::{Lpspi, LpspiError};

impl<const N: u8> eh1::spi::ErrorType for Lpspi<'_, N> {
    type Error = LpspiError;
}

impl<const N: u8, T> eh1::spi::SpiBus<T> for Lpspi<'_, N>
where
    T: crate::lpspi::LpspiWord,
{
    fn read(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(unsafe {
            self.transfer(create_actions_read_write(words, &[]))
        },))
        .block_on()
    }

    fn write(&mut self, words: &[T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(unsafe {
            self.transfer(create_actions_read_write(&mut [], words))
        }))
        .block_on()
    }

    fn transfer(&mut self, read: &mut [T], write: &[T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(unsafe {
            self.transfer(create_actions_read_write(read, write))
        }))
        .block_on()
    }

    fn transfer_in_place(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(unsafe {
            self.transfer(create_actions_read_write_in_place(words))
        }))
        .block_on()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Cassette::new(core::pin::pin!(self.flush())).block_on()
    }
}

// Async only makes sense for DMA; DMA only supports u32.
#[cfg(feature = "async")]
impl<const N: u8, T> eh1_async::spi::SpiBus<T> for Lpspi<'_, N>
where
    T: crate::lpspi::LpspiWord,
{
    async fn read(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        unsafe { self.transfer(create_actions_read_write(words, &[])).await }
    }

    async fn write(&mut self, words: &[T]) -> Result<(), Self::Error> {
        unsafe {
            self.transfer(create_actions_read_write(&mut [], words))
                .await
        }
    }

    async fn transfer(&mut self, read: &mut [T], write: &[T]) -> Result<(), Self::Error> {
        unsafe { self.transfer(create_actions_read_write(read, write)).await }
    }

    async fn transfer_in_place(&mut self, words: &mut [T]) -> Result<(), Self::Error> {
        unsafe {
            self.transfer(create_actions_read_write_in_place(words))
                .await
        }
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush().await
    }
}
