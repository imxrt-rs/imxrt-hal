pub use drivers::{ccm_118x as ccm, rgpio};

pub(crate) mod iomuxc {
    pub use super::config::pads;
    use crate::ral;

    /// Transform the `imxrt-ral` IOMUXC instances into pad objects.
    pub fn into_pads(_: ral::iomuxc::IOMUXC, _: ral::iomuxc_aon::IOMUXC_AON) -> pads::Pads {
        // Safety: acquiring pads has the same safety implications
        // as acquiring the IOMUXC instances. The user has already
        // assumed the unsafety.
        unsafe { pads::Pads::new() }
    }
}

pub mod dma {
    #[doc(hidden)]
    pub struct __PretendUsed(());
}

mod drivers {
    pub mod ccm_118x;
    pub mod rgpio;
}

mod config {
    pub use imxrt_iomuxc::imxrt1180 as pads;
}
