//! stm32ral module for imxrt1021

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 4;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::aipstz;
pub use super::instances::cmp;
pub use super::instances::dcdc_1015_1021 as dcdc;
pub use super::instances::iomuxc_snvs;
pub use super::instances::iomuxc_snvs_gpr;
pub use super::instances::pit_1015_1021 as pit;
pub mod iomuxc_gpr;
pub use super::instances::adc;
pub use super::instances::ewm;
pub use super::instances::flexram;
pub use super::instances::rtwdog;
pub use super::instances::snvs_1011_1015_1021 as snvs;
pub use super::instances::trng_1015_1021 as trng;
pub use super::instances::wdog;
pub mod ccm_analog;
pub mod pmu;
pub use super::instances::csu;
pub use super::instances::dma0_1015_1021 as dma0;
pub use super::instances::dmamux;
pub use super::instances::gpc_1021_1051_1052 as gpc;
pub use super::instances::pgc;
pub use super::instances::tempmon;
pub use super::instances::usb_analog_1015_1021 as usb_analog;
pub use super::instances::usbphy_1011_1015_1021 as usbphy;
pub use super::instances::xtalosc24m;
pub mod ccm;
pub mod src;
pub use super::instances::romc;
pub mod lpuart;
pub use super::instances::can_1021_1051_1052 as can;
pub use super::instances::flexio1;
pub use super::instances::gpio_1015_1021 as gpio;
pub mod tmr;
pub use super::instances::gpt;
pub use super::instances::ocotp_1021_1051_1052 as ocotp;
pub mod iomuxc;
pub use super::instances::enet_1021_1051_1052 as enet;
pub use super::instances::flexspi_1015_1021_1051_1052 as flexspi;
pub use super::instances::kpp;
pub use super::instances::usdhc;
pub mod usb1;
pub use super::instances::usbnc_1015_1021 as usbnc;
pub mod semc;
pub use super::instances::dcp;
pub use super::instances::sai;
pub use super::instances::spdif_1021_1051_1052 as spdif;
pub mod adc_etc;
pub mod lpspi;
pub use super::instances::aoi_1015_1021 as aoi;
pub use super::instances::xbara;
pub use super::instances::xbarb_1015_1021 as xbarb;
pub mod enc;
pub mod pwm;
pub use super::instances::bee;
pub use super::instances::lpi2c;
pub use super::instances::systemcontrol;
pub mod nvic;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub AIPSTZ1: aipstz::Instance,
    pub AIPSTZ2: aipstz::Instance,
    pub AIPSTZ3: aipstz::Instance,
    pub AIPSTZ4: aipstz::Instance,
    pub DCDC: dcdc::Instance,
    pub PIT: pit::Instance,
    pub CMP1: cmp::Instance,
    pub CMP2: cmp::Instance,
    pub CMP3: cmp::Instance,
    pub CMP4: cmp::Instance,
    pub IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::Instance,
    pub IOMUXC_SNVS: iomuxc_snvs::Instance,
    pub IOMUXC_GPR: iomuxc_gpr::Instance,
    pub FLEXRAM: flexram::Instance,
    pub EWM: ewm::Instance,
    pub WDOG1: wdog::Instance,
    pub WDOG2: wdog::Instance,
    pub RTWDOG: rtwdog::Instance,
    pub ADC1: adc::Instance,
    pub ADC2: adc::Instance,
    pub TRNG: trng::Instance,
    pub SNVS: snvs::Instance,
    pub CCM_ANALOG: ccm_analog::Instance,
    pub PMU: pmu::Instance,
    pub TEMPMON: tempmon::Instance,
    pub USB_ANALOG: usb_analog::Instance,
    pub XTALOSC24M: xtalosc24m::Instance,
    pub USBPHY: usbphy::Instance,
    pub CSU: csu::Instance,
    pub DMA0: dma0::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPC: gpc::Instance,
    pub PGC: pgc::Instance,
    pub SRC: src::Instance,
    pub CCM: ccm::Instance,
    pub ROMC: romc::Instance,
    pub LPUART1: lpuart::Instance,
    pub LPUART2: lpuart::Instance,
    pub LPUART3: lpuart::Instance,
    pub LPUART4: lpuart::Instance,
    pub LPUART5: lpuart::Instance,
    pub LPUART6: lpuart::Instance,
    pub LPUART7: lpuart::Instance,
    pub LPUART8: lpuart::Instance,
    pub FLEXIO1: flexio1::Instance,
    pub GPIO1: gpio::Instance,
    pub GPIO5: gpio::Instance,
    pub GPIO2: gpio::Instance,
    pub GPIO3: gpio::Instance,
    pub CAN1: can::Instance,
    pub CAN2: can::Instance,
    pub TMR1: tmr::Instance,
    pub TMR2: tmr::Instance,
    pub GPT1: gpt::Instance,
    pub GPT2: gpt::Instance,
    pub OCOTP: ocotp::Instance,
    pub IOMUXC: iomuxc::Instance,
    pub KPP: kpp::Instance,
    pub FLEXSPI: flexspi::Instance,
    pub USDHC1: usdhc::Instance,
    pub USDHC2: usdhc::Instance,
    pub ENET: enet::Instance,
    pub USB1: usb1::Instance,
    pub USBNC: usbnc::Instance,
    pub SEMC: semc::Instance,
    pub DCP: dcp::Instance,
    pub SPDIF: spdif::Instance,
    pub SAI1: sai::Instance,
    pub SAI2: sai::Instance,
    pub SAI3: sai::Instance,
    pub LPSPI1: lpspi::Instance,
    pub LPSPI2: lpspi::Instance,
    pub LPSPI3: lpspi::Instance,
    pub LPSPI4: lpspi::Instance,
    pub ADC_ETC: adc_etc::Instance,
    pub AOI: aoi::Instance,
    pub XBARA: xbara::Instance,
    pub XBARB: xbarb::Instance,
    pub ENC1: enc::Instance,
    pub ENC2: enc::Instance,
    pub PWM1: pwm::Instance,
    pub PWM2: pwm::Instance,
    pub BEE: bee::Instance,
    pub LPI2C1: lpi2c::Instance,
    pub LPI2C2: lpi2c::Instance,
    pub LPI2C3: lpi2c::Instance,
    pub LPI2C4: lpi2c::Instance,
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
            AIPSTZ3: aipstz::AIPSTZ3::steal(),
            AIPSTZ4: aipstz::AIPSTZ4::steal(),
            DCDC: dcdc::DCDC::steal(),
            PIT: pit::PIT::steal(),
            CMP1: cmp::CMP1::steal(),
            CMP2: cmp::CMP2::steal(),
            CMP3: cmp::CMP3::steal(),
            CMP4: cmp::CMP4::steal(),
            IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IOMUXC_SNVS_GPR::steal(),
            IOMUXC_SNVS: iomuxc_snvs::IOMUXC_SNVS::steal(),
            IOMUXC_GPR: iomuxc_gpr::IOMUXC_GPR::steal(),
            FLEXRAM: flexram::FLEXRAM::steal(),
            EWM: ewm::EWM::steal(),
            WDOG1: wdog::WDOG1::steal(),
            WDOG2: wdog::WDOG2::steal(),
            RTWDOG: rtwdog::RTWDOG::steal(),
            ADC1: adc::ADC1::steal(),
            ADC2: adc::ADC2::steal(),
            TRNG: trng::TRNG::steal(),
            SNVS: snvs::SNVS::steal(),
            CCM_ANALOG: ccm_analog::CCM_ANALOG::steal(),
            PMU: pmu::PMU::steal(),
            TEMPMON: tempmon::TEMPMON::steal(),
            USB_ANALOG: usb_analog::USB_ANALOG::steal(),
            XTALOSC24M: xtalosc24m::XTALOSC24M::steal(),
            USBPHY: usbphy::USBPHY::steal(),
            CSU: csu::CSU::steal(),
            DMA0: dma0::DMA0::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPC: gpc::GPC::steal(),
            PGC: pgc::PGC::steal(),
            SRC: src::SRC::steal(),
            CCM: ccm::CCM::steal(),
            ROMC: romc::ROMC::steal(),
            LPUART1: lpuart::LPUART1::steal(),
            LPUART2: lpuart::LPUART2::steal(),
            LPUART3: lpuart::LPUART3::steal(),
            LPUART4: lpuart::LPUART4::steal(),
            LPUART5: lpuart::LPUART5::steal(),
            LPUART6: lpuart::LPUART6::steal(),
            LPUART7: lpuart::LPUART7::steal(),
            LPUART8: lpuart::LPUART8::steal(),
            FLEXIO1: flexio1::FLEXIO1::steal(),
            GPIO1: gpio::GPIO1::steal(),
            GPIO5: gpio::GPIO5::steal(),
            GPIO2: gpio::GPIO2::steal(),
            GPIO3: gpio::GPIO3::steal(),
            CAN1: can::CAN1::steal(),
            CAN2: can::CAN2::steal(),
            TMR1: tmr::TMR1::steal(),
            TMR2: tmr::TMR2::steal(),
            GPT1: gpt::GPT1::steal(),
            GPT2: gpt::GPT2::steal(),
            OCOTP: ocotp::OCOTP::steal(),
            IOMUXC: iomuxc::IOMUXC::steal(),
            KPP: kpp::KPP::steal(),
            FLEXSPI: flexspi::FLEXSPI::steal(),
            USDHC1: usdhc::USDHC1::steal(),
            USDHC2: usdhc::USDHC2::steal(),
            ENET: enet::ENET::steal(),
            USB1: usb1::USB1::steal(),
            USBNC: usbnc::USBNC::steal(),
            SEMC: semc::SEMC::steal(),
            DCP: dcp::DCP::steal(),
            SPDIF: spdif::SPDIF::steal(),
            SAI1: sai::SAI1::steal(),
            SAI2: sai::SAI2::steal(),
            SAI3: sai::SAI3::steal(),
            LPSPI1: lpspi::LPSPI1::steal(),
            LPSPI2: lpspi::LPSPI2::steal(),
            LPSPI3: lpspi::LPSPI3::steal(),
            LPSPI4: lpspi::LPSPI4::steal(),
            ADC_ETC: adc_etc::ADC_ETC::steal(),
            AOI: aoi::AOI::steal(),
            XBARA: xbara::XBARA::steal(),
            XBARB: xbarb::XBARB::steal(),
            ENC1: enc::ENC1::steal(),
            ENC2: enc::ENC2::steal(),
            PWM1: pwm::PWM1::steal(),
            PWM2: pwm::PWM2::steal(),
            BEE: bee::BEE::steal(),
            LPI2C1: lpi2c::LPI2C1::steal(),
            LPI2C2: lpi2c::LPI2C2::steal(),
            LPI2C3: lpi2c::LPI2C3::steal(),
            LPI2C4: lpi2c::LPI2C4::steal(),
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
