//! stm32ral module for imxrt1011

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub mod adc_etc;
pub mod aipstz;
pub mod aoi;
pub mod dcdc;
pub mod flexspi;
pub mod otfad;
pub mod pit;
pub mod xbara;
pub use super::instances::iomuxc_snvs_gpr;
pub mod flexram;
pub mod iomuxc_gpr;
pub mod iomuxc_snvs;
pub use super::instances::ewm;
pub mod wdog;
pub use super::instances::rtwdog;
pub mod adc1;
pub mod trng;
pub use super::instances::ccm_analog;
pub use super::instances::snvs;
pub mod pmu;
pub use super::instances::tempmon;
pub mod usb_analog;
pub use super::instances::csu;
pub use super::instances::usbphy;
pub use super::instances::xtalosc24m;
pub mod dcp;
pub mod dma0;
pub mod dmamux;
pub mod gpc;
pub mod usb1;
pub mod usbnc;
pub use super::instances::pgc;
pub mod ccm;
pub mod src;
pub use super::instances::romc;
pub mod lpi2c;
pub mod lpspi;
pub mod lpuart;
pub use super::instances::flexio1;
pub mod gpio;
pub mod pwm1;
pub mod sai;
pub mod spdif;
pub use super::instances::gpt;
pub mod iomuxc;
pub mod ocotp;
pub use super::instances::kpp;
pub use super::instances::systemcontrol;
pub mod nvic;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AIPSTZ1: aipstz::Instance,
    pub AIPSTZ2: aipstz::Instance,
    pub DCDC: dcdc::Instance,
    pub PIT: pit::Instance,
    pub ADC_ETC: adc_etc::Instance,
    pub AOI: aoi::Instance,
    pub XBARA: xbara::Instance,
    pub FLEXSPI: flexspi::Instance,
    pub OTFAD: otfad::Instance,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance,
    pub IOMUXC_GPR: iomuxc_gpr::Instance,
    pub FLEXRAM: flexram::Instance,
    pub EWM: ewm::Instance,
    pub WDOG1: wdog::Instance,
    pub WDOG2: wdog::Instance,
    pub RTWDOG: rtwdog::Instance,
    pub ADC1: adc1::Instance,
    pub TRNG: trng::Instance,
    pub SNVS: snvs::Instance,
    pub CCM_ANALOG: ccm_analog::Instance,
    pub PMU: pmu::Instance,
    pub TEMPMON: tempmon::Instance,
    pub USB_ANALOG: usb_analog::Instance,
    pub XTALOSC24M: xtalosc24m::Instance,
    pub USBPHY: usbphy::Instance,
    pub CSU: csu::Instance,
    pub USB1: usb1::Instance,
    pub USBNC: usbnc::Instance,
    pub DMA0: dma0::Instance,
    pub DMAMUX: dmamux::Instance,
    pub DCP: dcp::Instance,
    pub GPC: gpc::Instance,
    pub PGC: pgc::Instance,
    pub SRC: src::Instance,
    pub CCM: ccm::Instance,
    pub ROMC: romc::Instance,
    pub LPUART1: lpuart::Instance,
    pub LPUART2: lpuart::Instance,
    pub LPUART3: lpuart::Instance,
    pub LPUART4: lpuart::Instance,
    pub LPSPI1: lpspi::Instance,
    pub LPSPI2: lpspi::Instance,
    pub LPI2C1: lpi2c::Instance,
    pub LPI2C2: lpi2c::Instance,
    pub FLEXIO1: flexio1::Instance,
    pub GPIO1: gpio::Instance,
    pub GPIO5: gpio::Instance,
    pub GPIO2: gpio::Instance,
    pub PWM1: pwm1::Instance,
    pub SPDIF: spdif::Instance,
    pub SAI1: sai::Instance,
    pub SAI3: sai::Instance,
    pub GPT1: gpt::Instance,
    pub GPT2: gpt::Instance,
    pub OCOTP: ocotp::Instance,
    pub IOMUXC: iomuxc::Instance,
    pub KPP: kpp::Instance,
    pub SystemControl: systemcontrol::Instance,
    pub NVIC: nvic::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            AIPSTZ1: aipstz::AIPSTZ1::steal(),
            AIPSTZ2: aipstz::AIPSTZ2::steal(),
            DCDC: dcdc::DCDC::steal(),
            PIT: pit::PIT::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            AOI: aoi::AOI::steal(),
            XBARA: xbara::XBARA::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            OTFAD: otfad::OTFAD::steal(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::steal(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::steal(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::steal(),
            FLEXRAM: flexram::FLEXRAM::steal(),
            EWM: ewm::EWM::steal(),
            WDOG1: wdog::WDOG1::steal(),
            WDOG2: wdog::WDOG2::steal(),
            RTWDOG: rtwdog::RTWDOG::steal(),
            ADC1: adc1::ADC1::steal(),
            TRNG: trng::TRNG::steal(),
            SNVS: snvs::SNVS::steal(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::steal(),
            PMU: pmu::PMU::steal(),
            TEMPMON: tempmon::TEMPMON::steal(),
            USB_ANALOG: usb_analog::USB_ANALOG::steal(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::steal(),
            USBPHY: usbphy::USBPHY::steal(),
            CSU: csu::CSU::steal(),
            USB1: usb1::USB1::steal(),
            USBNC: usbnc::USBNC::steal(),
            DMA0: dma0::DMA0::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            DCP: dcp::DCP::steal(),
            GPC: gpc::GPC::steal(),
            PGC: pgc::PGC::steal(),
            SRC: src::SRC::steal(),
            CCM: ccm::CCM::steal(),
            ROMC: romc::ROMC::steal(),
            LPUART1: lpuart::LPUART1::steal(),
            LPUART2: lpuart::LPUART2::steal(),
            LPUART3: lpuart::LPUART3::steal(),
            LPUART4: lpuart::LPUART4::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
            FLEXIO1: flexio1::FLEXIO1::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            PWM1: pwm1::PWM1::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai::SAI1::steal(),
            SAI3: sai::SAI3::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
            SystemControl: systemcontrol::SystemControl::steal(),
            NVIC: nvic::NVIC::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
