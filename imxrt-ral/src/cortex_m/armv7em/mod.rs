//! stm32ral module for armv7em

pub use super::instances::cpb;
pub use super::instances::cpuid;
pub use super::instances::dcb;
pub use super::instances::dwt;
pub use super::instances::fpb;
pub use super::instances::itm;
pub use super::instances::syst;
pub use super::instances::tpiu;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CPB: cpb::Instance,
    pub CPUID: cpuid::Instance,
    pub DCB: dcb::Instance,
    pub DWT: dwt::Instance,
    pub FPB: fpb::Instance,
    pub ITM: itm::Instance,
    pub SYST: syst::Instance,
    pub TPIU: tpiu::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            CPB: cpb::CPB::steal(),
            CPUID: cpuid::CPUID::steal(),
            DCB: dcb::DCB::steal(),
            DWT: dwt::DWT::steal(),
            FPB: fpb::FPB::steal(),
            ITM: itm::ITM::steal(),
            SYST: syst::SYST::steal(),
            TPIU: tpiu::TPIU::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
