//! i.MX RT 1170 EVK board configuration, supporting CM7 applications.

use crate::{hal, iomuxc::imxrt1170 as iomuxc, ral, GPT1_DIVIDER, GPT2_DIVIDER, RUN_MODE};

#[cfg(target_arch = "arm")]
pub use imxrt1170evk_fcb as _;
#[cfg(target_arch = "arm")]
use panic_rtt_target as _;

use hal::ccm::clock_gate;
const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio(),
    clock_gate::dma(),
    clock_gate::pit::<1>(),
    clock_gate::gpt::<1>(),
    clock_gate::gpt::<2>(),
    clock_gate::usb(),
    clock_gate::lpuart::<{ CONSOLE_INSTANCE }>(),
];

pub(crate) unsafe fn configure() {
    let mut ccm = ral::ccm::CCM::instance();

    prepare_clock_tree(&mut ccm);
    CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(&mut ccm, clock_gate::ON));
}

fn prepare_clock_tree(ccm: &mut ral::ccm::CCM) {
    use crate::hal::ccm;
    ccm::clock_tree::configure_bus(RUN_MODE, ccm);
    ccm::clock_tree::configure_gpt::<1>(RUN_MODE, ccm);
    ccm::clock_tree::configure_gpt::<2>(RUN_MODE, ccm);
    ccm::clock_tree::configure_lpuart::<{ CONSOLE_INSTANCE }>(RUN_MODE, ccm);
}

pub const PIT_FREQUENCY: u32 = hal::ccm::clock_tree::bus_frequency(RUN_MODE);
pub const GPT1_FREQUENCY: u32 = hal::ccm::clock_tree::gpt_frequency::<1>(RUN_MODE) / GPT1_DIVIDER;
pub const GPT2_FREQUENCY: u32 = hal::ccm::clock_tree::gpt_frequency::<2>(RUN_MODE) / GPT2_DIVIDER;
pub const UART_CLK_FREQUENCY: u32 = hal::ccm::clock_tree::lpuart_frequency::<1>(RUN_MODE);
pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);

pub type Led = hal::gpio::Output<iomuxc::gpio_ad::GPIO_AD_04>;
pub type ConsolePins = hal::lpuart::Pins<
    iomuxc::gpio_ad::GPIO_AD_24, // TX, interfaced with debug chip
    iomuxc::gpio_ad::GPIO_AD_25, // RX, interfaced with debug chip
>;
const CONSOLE_INSTANCE: u8 = 1;
pub type Console = hal::lpuart::Lpuart<ConsolePins, { CONSOLE_INSTANCE }>;

pub struct Specifics {
    pub led: Led,
    pub console: Console,
}

impl Specifics {
    pub(crate) fn new(common: &mut crate::Common) -> Self {
        #[cfg(target_arch = "arm")]
        rtt_target::rtt_init_print!();

        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio9 = unsafe { ral::gpio::GPIO9::instance() };
        let mut gpio9 = hal::gpio::Port::new(gpio9);
        let led = gpio9.output(iomuxc.gpio_ad.p04);

        let console = unsafe { ral::lpuart::Instance::<{ CONSOLE_INSTANCE }>::instance() };
        let mut console = hal::lpuart::Lpuart::new(
            console,
            ConsolePins {
                tx: iomuxc.gpio_ad.p24,
                rx: iomuxc.gpio_ad.p25,
            },
        );
        console.disable(|console| {
            console.set_baud(&CONSOLE_BAUD);
            console.set_parity(None);
        });
        hal::usbphy::restart_pll(&mut common.usbphy1);

        Self { led, console }
    }
}

fn configure_pins(_: &mut super::Pads) {}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART1;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT1;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_USB1: Interrupt = Interrupt::USB_OTG1;
    pub const BOARD_SWTASK0: Interrupt = Interrupt::KPP;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_CONSOLE, syms::BOARD_CONSOLE),
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_PIT, syms::BOARD_PIT),
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}

pub use interrupt as Interrupt;
