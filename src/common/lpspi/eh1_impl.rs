use super::{Lpspi, LpspiError};

impl<const N: u8> eh1::spi::ErrorType for Lpspi<'_, N> {
    type Error = LpspiError;
}

impl<const N: u8> eh1::spi::SpiBus<u8> for Lpspi<'_, N> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<const N: u8> eh1::spi::SpiBus<u16> for Lpspi<'_, N> {
    fn read(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer(&mut self, read: &mut [u16], write: &[u16]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer_in_place(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<const N: u8> eh1::spi::SpiBus<u32> for Lpspi<'_, N> {
    fn read(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer(&mut self, read: &mut [u32], write: &[u32]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transfer_in_place(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "async")]
impl<const N: u8> eh1_async::spi::SpiBus<u8> for Lpspi<'_, N> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "async")]
impl<const N: u8> eh1_async::spi::SpiBus<u16> for Lpspi<'_, N> {
    async fn read(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer(&mut self, read: &mut [u16], write: &[u16]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn transfer_in_place(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        todo!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "async")]
impl<const N: u8> eh1_async::spi::SpiBus<u32> for Lpspi<'_, N> {
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
