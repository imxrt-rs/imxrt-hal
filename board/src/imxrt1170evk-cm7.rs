//! i.MX RT 1170 EVK board configuration, supporting CM7 applications.

use crate::{hal, iomuxc::imxrt1170 as iomuxc, ral};

#[cfg(target_arch = "arm")]
pub use imxrt1170evk_fcb as _;
#[cfg(target_arch = "arm")]
use panic_rtt_target as _;

mod imxrt11xx {}

pub(crate) unsafe fn configure() {
    // TODO these should all be clock gates...
    let ccm = ral::ccm::CCM::instance();

    // Enable LPCG for GPIOs.
    ral::write_reg!(ral::ccm, ccm, LPCG51_DIRECT, 1);

    // Enable LPCG for DMA.
    ral::write_reg!(ral::ccm, ccm, LPCG22_DIRECT, 1);
    ral::write_reg!(ral::ccm, ccm, LPCG23_DIRECT, 1);

    // Enable LPCG for PIT1 (PIT2 at 63).
    ral::write_reg!(ral::ccm, ccm, LPCG62_DIRECT, 1);

    // Enable LPCG for GPTs (only the first two).
    ral::write_reg!(ral::ccm, ccm, LPCG64_DIRECT, 1);
    ral::write_reg!(ral::ccm, ccm, LPCG65_DIRECT, 1);

    // Enable LPCG for USB1.
    // Enable LPCG for PITs.
    ral::write_reg!(ral::ccm, ccm, LPCG115_DIRECT, 1);
}

pub type Led = hal::gpio::Output<iomuxc::gpio_ad::GPIO_AD_04>;

pub struct Specifics {
    pub led: Led,
}

impl Specifics {
    pub(crate) fn new() -> Self {
        #[cfg(target_arch = "arm")]
        rtt_target::rtt_init_print!();

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio9 = unsafe { ral::gpio::GPIO9::instance() };
        let mut gpio9 = hal::gpio::Port::new(gpio9);
        let led = gpio9.output(iomuxc.gpio_ad.p04);

        Self { led }
    }
}

fn configure_pins(_: &mut super::Pads) {}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT1;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_USB1: Interrupt = Interrupt::USB_OTG1;
    pub const BOARD_SWTASK0: Interrupt = Interrupt::KPP;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_PIT, syms::BOARD_PIT),
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}
