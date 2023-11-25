use imxrt_dma::channel::Channel;

pub trait LpspiDma {
    fn get_one(&mut self) -> Option<&mut Channel>;
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)>;
}

/// Everything is CPU driven
pub struct NoDma;

/// Read and Write are DMA based,
/// but Transfers are only partially
/// DMA based
///
pub struct PartialDma(pub Channel);

/// Everything is DMA based.
///
/// This is a requirement for the async interface.
pub struct FullDma(pub Channel, pub Channel);

impl LpspiDma for NoDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        None
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        None
    }
}
impl LpspiDma for PartialDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        Some(&mut self.0)
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        None
    }
}
impl LpspiDma for FullDma {
    fn get_one(&mut self) -> Option<&mut Channel> {
        Some(&mut self.0)
    }
    fn get_two(&mut self) -> Option<(&mut Channel, &mut Channel)> {
        Some((&mut self.0, &mut self.1))
    }
}
