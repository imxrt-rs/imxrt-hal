//! i.MX RT 1180 EVK, supporting the Cortex-M33.

use imxrt_iomuxc::imxrt1180::gpio_ad::*;
use imxrt_ral as ral;

#[cfg(target_arch = "arm")]
use defmt_rtt as _;
#[cfg(target_arch = "arm")]
use imxrt1180evk_fcb as _;

use panic_probe as _;

pub unsafe fn configure() {}

/// TODO: I'm making this up. Don't make it up.
pub const UART_CLK_FREQUENCY: u32 = 24_000_000;
/// TODO: I'm making this up. Don't make it up.
pub const LPI2C_CLK_FREQUENCY: u32 = 24_000_000;

/// USER_LED1 on the board.
///
/// Managed through GPIO4_27.
pub type Led = imxrt_hal::rgpio::Output<GPIO_AD_27>;

#[non_exhaustive]
pub struct Specifics {
    pub led: Led,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let iomuxc_aon = unsafe { ral::iomuxc_aon::IOMUXC_AON::instance() };
        let pads = imxrt_hal::iomuxc::into_pads(iomuxc, iomuxc_aon);

        let gpio4 = unsafe { ral::rgpio::RGPIO4::instance() };
        let mut gpio4 = imxrt_hal::rgpio::Port::new(gpio4);
        let led = gpio4.output(pads.gpio_ad.p27);

        Specifics { led }
    }
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[];
}
