use super::LpspiDevice;

impl<const N: u8, CS> LpspiDevice<N, CS> {
    /// The peripheral instance.
    pub const N: u8 = N;
}
