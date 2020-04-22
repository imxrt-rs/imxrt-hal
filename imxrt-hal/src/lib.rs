//! NXP iMXRT hardware abstraction layer
//!
//! An [`embedded-hal`](https://crates.io/crates/embedded-hal) implementation
//! targeting processors in NXP's IMXRT106x family.
//!
//! The HAL is a WIP. More documentation will become available once more capabilities
//! are exposed.
//!
//! In some cases, the HAL simply re-exports peripherals from the peripheral access
//! crates (PAC). If they are not re-exported, all PAC components are available
//! in the `pac` module.
//!
//! To see examples of the HAL, check out the `teensy4-bsp` and the `teensy4-examples` crates.
//! We will skip documentation example and tests, since we cannot yet test them as part
//! of the `cargo test` workflow...

#![no_std]

pub use imxrt_ral as ral;

pub mod ccm;
pub mod dma;
pub mod gpio;
pub mod gpt;
pub mod i2c;
pub mod iomuxc;
pub mod pit;
pub mod pwm;
pub mod spi;
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
    pub iomuxc: iomuxc::IOMUXC,
    pub ccm: ccm::CCM,
    pub pit: pit::UnclockedPIT,
    pub dcdc: dcdc::DCDC,
    pub pwm1: pwm::Unclocked<pwm::module::_1>,
    pub pwm2: pwm::Unclocked<pwm::module::_2>,
    pub pwm3: pwm::Unclocked<pwm::module::_3>,
    pub pwm4: pwm::Unclocked<pwm::module::_4>,
    pub i2c: i2c::Unclocked,
    pub uart: uart::Unclocked,
    pub spi: spi::Unclocked,
    pub gpt1: gpt::Unclocked,
    pub gpt2: gpt::Unclocked,
    pub dma: [dma::Channel; 32],
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let p = Peripherals {
            iomuxc: iomuxc::IOMUXC::new(ral::iomuxc::IOMUXC::take()?),
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
            dma: [
                dma::Channel::new(0),
                dma::Channel::new(1),
                dma::Channel::new(2),
                dma::Channel::new(3),
                dma::Channel::new(4),
                dma::Channel::new(5),
                dma::Channel::new(6),
                dma::Channel::new(7),
                dma::Channel::new(8),
                dma::Channel::new(9),
                dma::Channel::new(10),
                dma::Channel::new(11),
                dma::Channel::new(12),
                dma::Channel::new(13),
                dma::Channel::new(14),
                dma::Channel::new(15),
                dma::Channel::new(16),
                dma::Channel::new(17),
                dma::Channel::new(18),
                dma::Channel::new(19),
                dma::Channel::new(20),
                dma::Channel::new(21),
                dma::Channel::new(22),
                dma::Channel::new(23),
                dma::Channel::new(24),
                dma::Channel::new(25),
                dma::Channel::new(26),
                dma::Channel::new(27),
                dma::Channel::new(28),
                dma::Channel::new(29),
                dma::Channel::new(30),
                dma::Channel::new(31),
            ],
        };
        Some(p)
    }
}
