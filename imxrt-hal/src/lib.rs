//! NXP iMXRT hardware abstraction layer
//!
//! An [`embedded-hal`](https://crates.io/crates/embedded-hal) implementation
//! targeting processors in NXP's IMXRT106x family.
//!
//! See the module-level documentation for more information and examples.

#![no_std]

pub use imxrt_ral as ral;

/// Pin muxing and configuration
///
/// See the `imxrt_iomuxc` crate documentation for more information on this module.
/// This module re-exports that crate, along with a chip-specific IOMUXC crate.
pub mod iomuxc {
    pub use imxrt_iomuxc::*;

    #[cfg(any(feature = "imxrt1062", feature = "imxrt1064"))]
    pub use imxrt_iomuxc::imxrt106x::*;

    /// Use this function to acquire the IOMUXC pads. It requires that you have an
    /// instance to the RAL's IOMUXC instance.
    pub(super) fn pads(_: crate::ral::iomuxc::Instance) -> Pads {
        unsafe { Pads::new() }
    }
}

pub mod adc;
pub mod ccm;
pub mod dma;
pub mod gpio;
pub mod gpt;
pub mod i2c;
pub mod pit;
pub mod pwm;
pub mod spi;
pub mod srtc;
pub mod trng;
pub mod uart;

pub mod dcdc {
    use imxrt_ral as ral;
    pub struct DCDC(pub(crate) ral::dcdc::Instance);
    impl DCDC {
        pub fn raw(&mut self) -> &ral::dcdc::Instance {
            &self.0
        }
    }
}

pub struct Peripherals {
    pub adc: adc::Unclocked,
    pub iomuxc: iomuxc::Pads,
    pub ccm: ccm::CCM,
    pub pit: pit::UnclockedPIT,
    pub dcdc: dcdc::DCDC,
    pub pwm1: pwm::Unclocked<iomuxc::consts::U1>,
    pub pwm2: pwm::Unclocked<iomuxc::consts::U2>,
    pub pwm3: pwm::Unclocked<iomuxc::consts::U3>,
    pub pwm4: pwm::Unclocked<iomuxc::consts::U4>,
    pub i2c: i2c::Unclocked,
    pub uart: uart::Unclocked,
    pub spi: spi::Unclocked,
    pub gpt1: gpt::Unclocked,
    pub gpt2: gpt::Unclocked,
    pub dma: dma::Unclocked,
    pub srtc: srtc::Unclocked,
    pub trng: trng::Unclocked,
}

impl Peripherals {
    /// Steal all of the HAL's peripherals
    ///
    /// # Safety
    ///
    /// The peripherals may be mutably aliased elsewhere in the code. Consider using
    /// [`take()`](struct.Peripherals.html#method.take) to safely acquire the HAL's
    /// peripherals.
    pub unsafe fn steal() -> Self {
        Peripherals {
            adc: adc::Unclocked {
                adc1: ral::adc::ADC1::steal(),
                adc2: ral::adc::ADC2::steal(),
            },
            iomuxc: iomuxc::pads(ral::iomuxc::IOMUXC::steal()),
            ccm: ccm::CCM::new(ral::ccm::CCM::steal(), ral::ccm_analog::CCM_ANALOG::steal()),
            pit: pit::UnclockedPIT::new(ral::pit::PIT::steal()),
            dcdc: dcdc::DCDC(ral::dcdc::DCDC::steal()),
            pwm1: pwm::Unclocked::new(ral::pwm::PWM1::steal()),
            pwm2: pwm::Unclocked::new(ral::pwm::PWM2::steal()),
            pwm3: pwm::Unclocked::new(ral::pwm::PWM3::steal()),
            pwm4: pwm::Unclocked::new(ral::pwm::PWM4::steal()),
            i2c: i2c::Unclocked {
                i2c1: ral::lpi2c::LPI2C1::steal(),
                i2c2: ral::lpi2c::LPI2C2::steal(),
                i2c3: ral::lpi2c::LPI2C3::steal(),
                i2c4: ral::lpi2c::LPI2C4::steal(),
            },
            uart: uart::Unclocked {
                uart1: ral::lpuart::LPUART1::steal(),
                uart2: ral::lpuart::LPUART2::steal(),
                uart3: ral::lpuart::LPUART3::steal(),
                uart4: ral::lpuart::LPUART4::steal(),
                uart5: ral::lpuart::LPUART5::steal(),
                uart6: ral::lpuart::LPUART6::steal(),
                uart7: ral::lpuart::LPUART7::steal(),
                uart8: ral::lpuart::LPUART8::steal(),
            },
            spi: spi::Unclocked {
                spi1: ral::lpspi::LPSPI1::steal(),
                spi2: ral::lpspi::LPSPI2::steal(),
                spi3: ral::lpspi::LPSPI3::steal(),
                spi4: ral::lpspi::LPSPI4::steal(),
            },
            gpt1: gpt::Unclocked::one(ral::gpt::GPT1::steal()),
            gpt2: gpt::Unclocked::two(ral::gpt::GPT2::steal()),
            dma: dma::Unclocked::new(ral::dma0::DMA0::steal(), ral::dmamux::DMAMUX::steal()),
            srtc: srtc::Unclocked::new(ral::snvs::SNVS::steal()),
            trng: trng::Unclocked::new(ral::trng::TRNG::steal()),
        }
    }

    /// Take the HAL's peripherals
    ///
    /// If the peripherals were already taken, `take()` returns `None`. Consider calling `take()`
    /// near the start of your program.
    pub fn take() -> Option<Self> {
        let p = Peripherals {
            adc: adc::Unclocked {
                adc1: ral::adc::ADC1::take()?,
                adc2: ral::adc::ADC2::take()?,
            },
            iomuxc: iomuxc::pads(ral::iomuxc::IOMUXC::take()?),
            ccm: ccm::CCM::new(ral::ccm::CCM::take()?, ral::ccm_analog::CCM_ANALOG::take()?),
            pit: pit::UnclockedPIT::new(ral::pit::PIT::take()?),
            dcdc: dcdc::DCDC(ral::dcdc::DCDC::take()?),
            pwm1: pwm::Unclocked::new(ral::pwm::PWM1::take()?),
            pwm2: pwm::Unclocked::new(ral::pwm::PWM2::take()?),
            pwm3: pwm::Unclocked::new(ral::pwm::PWM3::take()?),
            pwm4: pwm::Unclocked::new(ral::pwm::PWM4::take()?),
            i2c: i2c::Unclocked {
                i2c1: ral::lpi2c::LPI2C1::take()?,
                i2c2: ral::lpi2c::LPI2C2::take()?,
                i2c3: ral::lpi2c::LPI2C3::take()?,
                i2c4: ral::lpi2c::LPI2C4::take()?,
            },
            uart: uart::Unclocked {
                uart1: ral::lpuart::LPUART1::take()?,
                uart2: ral::lpuart::LPUART2::take()?,
                uart3: ral::lpuart::LPUART3::take()?,
                uart4: ral::lpuart::LPUART4::take()?,
                uart5: ral::lpuart::LPUART5::take()?,
                uart6: ral::lpuart::LPUART6::take()?,
                uart7: ral::lpuart::LPUART7::take()?,
                uart8: ral::lpuart::LPUART8::take()?,
            },
            spi: spi::Unclocked {
                spi1: ral::lpspi::LPSPI1::take()?,
                spi2: ral::lpspi::LPSPI2::take()?,
                spi3: ral::lpspi::LPSPI3::take()?,
                spi4: ral::lpspi::LPSPI4::take()?,
            },
            gpt1: gpt::Unclocked::one(ral::gpt::GPT1::take()?),
            gpt2: gpt::Unclocked::two(ral::gpt::GPT2::take()?),
            dma: dma::Unclocked::new(ral::dma0::DMA0::take()?, ral::dmamux::DMAMUX::take()?),
            srtc: srtc::Unclocked::new(ral::snvs::SNVS::take()?),
            trng: trng::Unclocked::new(ral::trng::TRNG::take()?),
        };
        Some(p)
    }
}
