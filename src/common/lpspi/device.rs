use super::{LpspiDevice, LpspiError};

impl<const N: u8, CS> LpspiDevice<N, CS> {
    /// The peripheral instance.
    pub const N: u8 = N;
}

impl<const N: u8, CS> eh1::spi::ErrorType for LpspiDevice<N, CS> {
    type Error = LpspiError;
}

impl<const N: u8, CS> eh1::spi::SpiDevice<u8> for LpspiDevice<N, CS> {
    fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<const N: u8, CS> eh1::spi::SpiDevice<u16> for LpspiDevice<N, CS> {
    fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u16>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<const N: u8, CS> eh1::spi::SpiDevice<u32> for LpspiDevice<N, CS> {
    fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u32>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

// TODO: should we support u64 and u128?

#[cfg(feature = "async")]
impl<const N: u8, CS> eh1_async::spi::SpiDevice<u8> for LpspiDevice<N, CS> {
    async fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "async")]
impl<const N: u8, CS> eh1_async::spi::SpiDevice<u16> for LpspiDevice<N, CS> {
    async fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u16>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(feature = "async")]
impl<const N: u8, CS> eh1_async::spi::SpiDevice<u32> for LpspiDevice<N, CS> {
    async fn transaction(
        &mut self,
        operations: &mut [eh1::spi::Operation<'_, u32>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
