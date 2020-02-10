//! stm32ral module for armv6m

pub mod cpuid;
pub use super::instances::dcb;
pub use super::instances::dwt;
pub use super::instances::syst;

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub CPUID: cpuid::Instance,
    pub DCB: dcb::Instance,
    pub DWT: dwt::Instance,
    pub SYST: syst::Instance,
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtfm", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            CPUID: cpuid::CPUID::steal(),
            DCB: dcb::DCB::steal(),
            DWT: dwt::DWT::steal(),
            SYST: syst::SYST::steal(),
        }
    }
}

#[cfg(all(feature = "rtfm", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
