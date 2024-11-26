//! i.MX RT 1170 EVK board configuration, supporting CM7 applications.

use crate::{hal, iomuxc::imxrt1170 as iomuxc, ral, GPT1_DIVIDER, GPT2_DIVIDER, RUN_MODE};

mod imxrt11xx {
    pub(super) mod clock_tree;
}

use imxrt11xx::clock_tree;

#[cfg(target_arch = "arm")]
use defmt_rtt as _;
#[cfg(target_arch = "arm")]
use imxrt1170evk_fcb as _;

use panic_probe as _;

#[defmt::panic_handler]
fn defmt_panic() -> ! {
    cortex_m::asm::udf();
}

/// You'll find log messages using the serial console, through the DAP.
pub(crate) const DEFAULT_LOGGING_BACKEND: crate::logging::Backend = crate::logging::Backend::Lpuart;

pub const ENET_MAC: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];

use hal::ccm::clock_gate;
const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::iomuxc(),
    clock_gate::iomuxc_lpsr(),
    clock_gate::gpio(),
    clock_gate::dma(),
    clock_gate::pit::<1>(),
    clock_gate::gpt::<1>(),
    clock_gate::gpt::<2>(),
    clock_gate::usb(),
    clock_gate::lpuart::<{ CONSOLE_INSTANCE }>(),
    clock_gate::lpspi::<SPI_INSTANCE>(),
    clock_gate::flexpwm::<{ PWM_INSTANCE }>(),
    clock_gate::lpi2c::<{ I2C_INSTANCE }>(),
    clock_gate::snvs(),
];

const ENET_CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::enet(),
    clock_gate::enet_1g(),
    clock_gate::enet_qos(),
];

pub(crate) unsafe fn configure() {
    let mut ccm = ral::ccm::CCM::instance();

    let gpc = unsafe { ral::gpc_cpu_mode_ctrl_::GPC_CPU_MODE_CTRL_0::instance() };
    let gpc = &*gpc;
    let pll = unsafe { ral::anadig_pll::ANADIG_PLL::instance() };
    let mut pmu = unsafe { ral::anadig_pmu::ANADIG_PMU::instance() };

    hal::pmu::enable_pll_reference_voltage(&mut pmu, true);
    hal::pmu::set_phy_ldo_setpoints(&mut pmu, hal::pmu::Setpoint::all());
    hal::pmu::set_phy_ldo_control(&mut pmu, hal::pmu::ControlMode::Gpc);
    hal::pmu::set_pll_reference_control(&mut pmu, hal::pmu::ControlMode::Gpc);

    for clock_source in {
        use hal::ccm::ClockSource::*;
        [Pll1, Pll1Clk, Pll1Div2, Pll1Div5]
    } {
        let oscpll = &ccm.OSCPLL[clock_source as usize];
        ral::write_reg!(ral::ccm::oscpll, oscpll, OSCPLL_DIRECT, 0);
        hal::ccm::set_setpoints(&mut ccm, clock_source, hal::ccm::Setpoint::SP1);
        hal::ccm::set_gpc_control_mode(&mut ccm, clock_source, hal::ccm::ControlMode::Gpc).unwrap();
    }

    ral::modify_reg!(ral::anadig_pll, pll, SYS_PLL1_CTRL,
        SYS_PLL1_CONTROL_MODE: 1,
        SYS_PLL1_DIV2_CONTROL_MODE: 1,
        SYS_PLL1_DIV5_CONTROL_MODE: 1,
    );

    prepare_clock_tree(&mut ccm);
    CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(&mut ccm, clock_gate::ON));

    ENET_CLOCK_GATES
        .iter()
        .for_each(|l| l.set(&mut ccm, clock_gate::OFF));
    clock_tree::enet_root_on(&mut ccm, false);

    clock_tree::configure_enet(RUN_MODE, &mut ccm);
    hal::gpc::request_setpoint_transition(gpc, 1).unwrap();

    clock_tree::enet_root_on(&mut ccm, true);
    ENET_CLOCK_GATES
        .iter()
        .for_each(|l| l.set(&mut ccm, clock_gate::ON));
}

fn prepare_clock_tree(ccm: &mut ral::ccm::CCM) {
    clock_tree::configure_bus(RUN_MODE, ccm);
    clock_tree::configure_gpt::<1>(RUN_MODE, ccm);
    clock_tree::configure_gpt::<2>(RUN_MODE, ccm);
    clock_tree::configure_lpuart::<{ CONSOLE_INSTANCE }>(RUN_MODE, ccm);
    clock_tree::configure_lpspi::<SPI_INSTANCE>(RUN_MODE, ccm);
    clock_tree::configure_lpi2c::<{ I2C_INSTANCE }>(RUN_MODE, ccm);
}

pub const PIT_FREQUENCY: u32 = clock_tree::bus_frequency(RUN_MODE);
pub const GPT1_FREQUENCY: u32 = clock_tree::gpt_frequency::<1>(RUN_MODE) / GPT1_DIVIDER;
pub const GPT2_FREQUENCY: u32 = clock_tree::gpt_frequency::<2>(RUN_MODE) / GPT2_DIVIDER;
pub const UART_CLK_FREQUENCY: u32 = clock_tree::lpuart_frequency::<1>(RUN_MODE);
pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);
pub const LPSPI_CLK_FREQUENCY: u32 = clock_tree::lpspi_frequency::<SPI_INSTANCE>(RUN_MODE);
pub const LPI2C_CLK_FREQUENCY: u32 = clock_tree::lpi2c_frequency::<I2C_INSTANCE>(RUN_MODE);
pub const PWM_PRESCALER: hal::flexpwm::Prescaler = hal::flexpwm::Prescaler::Prescaler8;
pub const PWM_FREQUENCY: u32 = clock_tree::bus_frequency(RUN_MODE) / PWM_PRESCALER.divider();
pub const ENET_FREQUENCY: u32 = clock_tree::ENET1_FREQUENCY;
pub const ENET_BUS_FREQUENCY: u32 = clock_tree::bus_frequency(RUN_MODE);

pub type Led = hal::gpio::Output<iomuxc::gpio_ad::GPIO_AD_04>;

/// SW7, the "CPU wakeup" button.
pub type Button = hal::gpio::Input<()>;

pub type ConsolePins = hal::lpuart::Pins<
    iomuxc::gpio_ad::GPIO_AD_24, // TX, interfaced with debug chip
    iomuxc::gpio_ad::GPIO_AD_25, // RX, interfaced with debug chip
>;
const CONSOLE_INSTANCE: u8 = 1;
pub type Console = hal::lpuart::Lpuart<ConsolePins, { CONSOLE_INSTANCE }>;

/// Test point 1002.
///
/// For evaluating clocks via `CCM_CLKO1`.
pub type Tp1002 = iomuxc::gpio_emc_b1::GPIO_EMC_B1_40;

/// Test point 1003.
///
/// For evaluating clocks via `CCM_CLKO2`.
pub type Tp1003 = iomuxc::gpio_emc_b1::GPIO_EMC_B1_41;

pub type SpiPins = hal::lpspi::Pins<
    iomuxc::gpio_ad::GPIO_AD_30, // SDO, J10_8
    iomuxc::gpio_ad::GPIO_AD_31, // SDI, J10_10
    iomuxc::gpio_ad::GPIO_AD_28, // SCK, J10_12
    iomuxc::gpio_ad::GPIO_AD_29, // PCS0, J10_6
>;
const SPI_INSTANCE: u8 = 1;

#[cfg(feature = "spi")]
pub type Spi = hal::lpspi::Lpspi<SpiPins, { SPI_INSTANCE }>;
#[cfg(not(feature = "spi"))]
pub type Spi = ();

pub type I2cPins = hal::lpi2c::Pins<
    iomuxc::gpio_lpsr::GPIO_LPSR_05, // SCL, J10_20
    iomuxc::gpio_lpsr::GPIO_LPSR_04, // SDA, J10_18
>;

const I2C_INSTANCE: u8 = 5;
pub type I2c = hal::lpi2c::Lpi2c<I2cPins, { I2C_INSTANCE }>;

const PWM_INSTANCE: u8 = 2;

#[cfg(not(feature = "spi"))]
pub mod pwm {
    use super::iomuxc;
    use super::PWM_INSTANCE;
    use crate::hal::flexpwm;

    pub type Peripheral = flexpwm::Pwm<{ PWM_INSTANCE }>;
    pub type Submodule = flexpwm::Submodule<{ PWM_INSTANCE }, 2>;
    pub type Outputs = (
        flexpwm::Output<iomuxc::gpio_ad::GPIO_AD_28>, // A, J9_8
        flexpwm::Output<iomuxc::gpio_ad::GPIO_AD_29>, // B, J9_12
    );
}

#[cfg(feature = "spi")]
pub mod pwm {
    pub type Peripheral = ();
    pub type Submodule = ();
    pub type Outputs = ();
}

/// The board's PWM components.
pub struct Pwm {
    /// Core PWM peripheral.
    pub module: pwm::Peripheral,
    /// PWM submodule control registers.
    pub submodule: pwm::Submodule,
    /// The output pairs (tupler of A, B outputs).
    pub outputs: pwm::Outputs,
}

pub struct GpioPorts {
    gpio13: hal::gpio::Port<13>,
}

impl GpioPorts {
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port<13> {
        &mut self.gpio13
    }
}

pub struct Specifics {
    pub led: Led,
    pub button: Button,
    pub ports: GpioPorts,
    pub console: Console,
    pub tp1002: Tp1002,
    pub tp1003: Tp1003,
    pub spi: Spi,
    pub pwm: Pwm,
    pub i2c: I2c,
    pub enet: imxrt_ral::enet::ENET,
}

impl Specifics {
    pub(crate) fn new(common: &mut crate::Common) -> Self {
        // Manually configuring IOMUXC_SNVS pads, since there's no
        // equivalent API in imxrt-iomuxc.
        let iomuxc_snvs = unsafe { ral::iomuxc_snvs::IOMUXC_SNVS::instance() };
        // ALT5 => GPIO13[00]
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_MUX_CTL_PAD_WAKEUP_DIG, MUX_MODE: 5);
        // Pull up the pin to be brought to GND on switch press. No need for a high drive.
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_PAD_CTL_PAD_WAKEUP_DIG, PUS: 1, PUE: 1, DSE: 0);

        let gpio13 = unsafe { ral::gpio::GPIO13::instance() };
        let mut gpio13 = hal::gpio::Port::new(gpio13);
        let button = hal::gpio::Input::without_pin(&mut gpio13, 0);

        let iomuxc_gpr = unsafe { ral::iomuxc_gpr::IOMUXC_GPR::instance() };
        // ENET_REF_CLK driven by ENET1_CLK_ROOT.
        ral::modify_reg!(ral::iomuxc_gpr, iomuxc_gpr, GPR4, ENET_REF_CLK_DIR: 1);
        // Handle ERR050396. Depending on the runtime config, we might place buffers into TCM.
        ral::modify_reg!(ral::iomuxc_gpr, iomuxc_gpr, GPR28, CACHE_ENET1G: 0, CACHE_ENET: 0);

        let mut iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc_lpsr = unsafe { ral::iomuxc_lpsr::IOMUXC_LPSR::instance() };
        configure_eth_pins(&mut iomuxc, &mut iomuxc_lpsr);
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio9 = unsafe { ral::gpio::GPIO9::instance() };
        let gpio12 = unsafe { ral::gpio::GPIO12::instance() };

        const ENET_INT: u32 = 1 << 11;
        const ENET_RST: u32 = 1 << 12;
        ral::modify_reg!(ral::gpio, gpio9, GDIR, |gdir| gdir | ENET_INT);
        ral::modify_reg!(ral::gpio, gpio12, GDIR, |gdir| gdir | ENET_RST);

        ral::write_reg!(ral::gpio, gpio9, DR_CLEAR, ENET_INT);
        ral::write_reg!(ral::gpio, gpio12, DR_CLEAR, ENET_RST);
        common.block_ms(500);

        ral::write_reg!(ral::gpio, gpio9, DR_SET, ENET_INT);
        common.block_ms(500);
        ral::write_reg!(ral::gpio, gpio12, DR_SET, ENET_RST);
        common.block_ms(500);

        let mut gpio9 = hal::gpio::Port::new(gpio9);
        let led = gpio9.output(iomuxc.gpio_ad.p04);

        // let ccm_enet_25mh_ref = gpio9.output(iomuxc.gpio_ad.p14);
        // ccm_enet_25mh_ref.set();
        hal::iomuxc::alternate(&mut iomuxc.gpio_ad.p14, 9);

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

        #[cfg(feature = "spi")]
        let spi = {
            let lpspi1 = unsafe { ral::lpspi::LPSPI1::instance() };
            let pins = SpiPins {
                sdo: iomuxc.gpio_ad.p30,
                sdi: iomuxc.gpio_ad.p31,
                sck: iomuxc.gpio_ad.p28,
                pcs0: iomuxc.gpio_ad.p29,
            };
            let mut spi = Spi::new(lpspi1, pins);
            spi.disabled(|spi| {
                spi.set_clock_hz(LPSPI_CLK_FREQUENCY, super::SPI_BAUD_RATE_FREQUENCY);
            });
            spi
        };
        #[cfg(not(feature = "spi"))]
        #[allow(clippy::let_unit_value)]
        let spi = ();

        #[cfg(not(feature = "spi"))]
        let pwm = {
            let flexpwm = unsafe { ral::pwm::PWM2::instance() };
            let (pwm, (_, _, sm, _)) = hal::flexpwm::new(flexpwm);

            let out_a = hal::flexpwm::Output::new_a(iomuxc.gpio_ad.p28);
            let out_b = hal::flexpwm::Output::new_b(iomuxc.gpio_ad.p29);

            super::Pwm {
                module: pwm,
                submodule: sm,
                outputs: (out_a, out_b),
            }
        };
        #[cfg(feature = "spi")]
        let pwm = Pwm {
            module: (),
            submodule: (),
            outputs: (),
        };
        let i2c = {
            let lpi2c5 = unsafe { ral::lpi2c::LPI2C5::instance() };
            I2c::new(
                lpi2c5,
                I2cPins {
                    scl: iomuxc.gpio_lpsr.p05,
                    sda: iomuxc.gpio_lpsr.p04,
                },
                &super::I2C_BAUD_RATE,
            )
        };

        Self {
            led,
            console,
            button,
            ports: GpioPorts { gpio13 },
            tp1002: iomuxc.gpio_emc_b1.p40,
            tp1003: iomuxc.gpio_emc_b1.p41,
            spi,
            pwm,
            i2c,
            enet: unsafe { imxrt_ral::enet::ENET::instance() },
        }
    }
}

fn configure_pins(iomuxc: &mut super::Pads) {
    // Set the pin muxing for the two test points.
    let clko1: &mut Tp1002 = &mut iomuxc.gpio_emc_b1.p40;
    let clko2: &mut Tp1003 = &mut iomuxc.gpio_emc_b1.p41;
    crate::iomuxc::ccm::prepare(clko1);
    crate::iomuxc::ccm::prepare(clko2);

    // Can't use imxrt-iomuxc configuration APIs for this chip.
    // See the -iomuxc issue tracker for more information.
    //
    // Safety: We have exclusive ownership of the (higher-level)
    // IOMUXC instance.
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };

    // SPI: High drive strength, slow slew, no pulls, not open drain.
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_30, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_31, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_28, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_29, DSE: DSE_1_HIGH_DRIVER);
}

fn configure_eth_pins(
    iomuxc: &mut ral::iomuxc::IOMUXC,
    iomuxc_lpsr: &mut ral::iomuxc_lpsr::IOMUXC_LPSR,
) {
    //
    // Muxing
    //
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_32, MUX_MODE: ALT3_ENET_MDC, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_33, MUX_MODE: ALT3_ENET_MDIO, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_02, MUX_MODE: ALT1_ENET_TX_DATA0, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_03, MUX_MODE: ALT1_ENET_TX_DATA1, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_04, MUX_MODE: ALT1_ENET_TX_EN, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_05, MUX_MODE: ALT2_ENET_REF_CLK, SION: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_06, MUX_MODE: ALT1_ENET_RX_DATA0, SION: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_07, MUX_MODE: ALT1_ENET_RX_DATA1, SION: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_08, MUX_MODE: ALT1_ENET_RX_EN, SION: 0);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_DISP_B2_09, MUX_MODE: ALT1_ENET_RX_ER, SION: 0);

    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_12, MUX_MODE: ALT10_GPIO9_IO11, SION: 0);
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_MUX_CTL_PAD_GPIO_LPSR_12, MUX_MODE: ALT10_GPIO12_IO12);

    //
    // Pad configurations
    //
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_02, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_0_PULL_DISABLE__HIGHZ, ODE: ODE_0_DISABLED);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_03, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_0_PULL_DISABLE__HIGHZ, ODE: ODE_0_DISABLED);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_04, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_0_PULL_DISABLE__HIGHZ, ODE: ODE_0_DISABLED);

    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_05, SRE: SRE_1_FAST_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_0_PULL_DISABLE__HIGHZ, ODE: ODE_0_DISABLED);

    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_06, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL_ENABLE, PUS: PUS_0_WEAK_PULL_DOWN, ODE: ODE_0_DISABLED);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_07, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL_ENABLE, PUS: PUS_0_WEAK_PULL_DOWN, ODE: ODE_0_DISABLED);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_08, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL_ENABLE, PUS: PUS_0_WEAK_PULL_DOWN, ODE: ODE_0_DISABLED);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_DISP_B2_09, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL_ENABLE, PUS: PUS_0_WEAK_PULL_DOWN, ODE: ODE_0_DISABLED);

    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_12, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL, PUS: PUS_0_WEAK_PULL_DOWN, ODE: ODE_0_OPEN_DRAIN_DISABLED);
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_12, SRE: SRE_0_SLOW_SLEW_RATE, DSE: DSE_1_HIGH_DRIVER, PUE: PUE_1_PULL, PUS: PUS_1_WEAK_PULL_UP, ODE_LPSR: ODE_LPSR_0_DISABLED);

    //
    // Input daisies
    //
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_MAC0_MDIO_SELECT_INPUT, DAISY: SELECT_GPIO_AD_33_ALT3);
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_IPG_CLK_RMII_SELECT_INPUT, DAISY: SELECT_GPIO_DISP_B2_05_ALT2);
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_MAC0_RXDATA_SELECT_INPUT_0, DAISY: SELECT_GPIO_DISP_B2_06_ALT1);
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_MAC0_RXDATA_SELECT_INPUT_1, DAISY: SELECT_GPIO_DISP_B2_07_ALT1);
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_MAC0_RXEN_SELECT_INPUT, DAISY: SELECT_GPIO_DISP_B2_08_ALT1);
    ral::write_reg!(ral::iomuxc, iomuxc, ENET_MAC0_RXERR_SELECT_INPUT, DAISY: SELECT_GPIO_DISP_B2_09_ALT1);
}

fn init_enet_phy_ksz8081rnb<
    M: hal::enet::MiimRead<Error = hal::enet::MiiError>
        + hal::enet::MiimWrite<Error = hal::enet::MiiError>,
>(
    mdio: &mut M,
) -> Result<(), &'static str> {
    //
    // TODO(mciantyre) Upstream this, somewhere, and add missing parts.
    //
    const PHY_ID1_REG: u8 = 0x02;
    const PHY_BASICCONTROL_REG: u8 = 0x00;
    const PHY_BASICSTATUS_REG: u8 = 0x01;
    const PHY_AUTONEG_ADVERTISE_REG: u8 = 0x04;

    const KSZ8081_PHY_ADDR: u8 = 0x02; // Arbitrary.
    const KSZ8081_PHY_CONTROL2_REG: u8 = 0x1F;
    const KSZ8081_PHY_CTL2_REFCLK_SELECT_MASK: u16 = 0x0080;

    bitflags::bitflags! {
        struct BasicControl : u16 {
            const RESET = 0x8000;
            const AUTONEG = 0x1000;
            const RESTART_AUTONEG = 0x0200;
        }
    }

    bitflags::bitflags! {
        struct BasicStatus : u16 {
            const LINKSTATUS = 0x0004;
            const AUTONEG_COMP = 0x0020;
        }
    }

    bitflags::bitflags! {
        struct AutoNegotiation : u16 {
            const BASET4_100_ABILITY = 0x0200;
            const BASETX_100_FULL_DUPLEX = 0x0100;
            const BASETX_100_HALF_DUPLEX = 0x0080;
            const BASETX_10_FULL_DUPLEX = 0x0040;
            const BASETX_10_HALF_DUPLEX = 0x0020;
            const IEEE802_3_SELECTOR = 0x0001;
        }
    }

    let mut retries = 100000usize;
    while retries > 0 {
        const KSZ8081_ID: u16 = 0x22;
        let address = mdio.read(KSZ8081_PHY_ADDR, PHY_ID1_REG).unwrap();
        if address == KSZ8081_ID {
            break;
        }
        retries -= 1;
    }
    if retries == 0 {
        return Err("Could not detect KSZ8081");
    }

    mdio.write(
        KSZ8081_PHY_ADDR,
        PHY_BASICCONTROL_REG,
        BasicControl::RESET.bits(),
    )
    .unwrap();

    // Use RMII50M (KSZ8081-specific configuration).
    let ctrl2 = mdio
        .read(KSZ8081_PHY_ADDR, KSZ8081_PHY_CONTROL2_REG)
        .unwrap();
    mdio.write(
        KSZ8081_PHY_ADDR,
        KSZ8081_PHY_CONTROL2_REG,
        ctrl2 | KSZ8081_PHY_CTL2_REFCLK_SELECT_MASK,
    )
    .unwrap();

    // Enable auto-negotiation.
    mdio.write(
        KSZ8081_PHY_ADDR,
        PHY_AUTONEG_ADVERTISE_REG,
        (AutoNegotiation::BASETX_100_FULL_DUPLEX | AutoNegotiation::IEEE802_3_SELECTOR).bits(),
    )
    .unwrap();
    mdio.write(
        KSZ8081_PHY_ADDR,
        PHY_BASICCONTROL_REG,
        (BasicControl::AUTONEG | BasicControl::RESTART_AUTONEG).bits(),
    )
    .unwrap();

    // Wait for auto-negotiation, and for the link to be available.
    //
    // This blocks while there's no link.
    while !BasicStatus::from_bits_truncate(
        mdio.read(KSZ8081_PHY_ADDR, PHY_BASICSTATUS_REG).unwrap(),
    )
    .contains(BasicStatus::AUTONEG_COMP | BasicStatus::LINKSTATUS)
    {}

    Ok(())
}

fn init_enet_phy_rtl8211f<
    M: hal::enet::MiimRead<Error = hal::enet::MiiError>
        + hal::enet::MiimWrite<Error = hal::enet::MiiError>,
>(
    _: &mut M,
) -> Result<(), &'static str> {
    // Intentionally left blank.
    Ok(())
}

/// Initialize the KSZ8081 10/100-Mbit PHY.
pub fn init_enet_phy<
    M: hal::enet::MiimRead<Error = hal::enet::MiiError>
        + hal::enet::MiimWrite<Error = hal::enet::MiiError>,
>(
    mdio: &mut M,
) -> Result<(), &'static str> {
    init_enet_phy_ksz8081rnb(mdio)?;
    init_enet_phy_rtl8211f(mdio)?;
    Ok(())
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART1;
    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO13_COMBINED_0_31;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA11_DMA27;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT1;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI1;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM2_2;
    pub const BOARD_USB1: Interrupt = Interrupt::USB_OTG1;
    pub const BOARD_SWTASK0: Interrupt = Interrupt::KPP;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_CONSOLE, syms::BOARD_CONSOLE),
        (BOARD_BUTTON, syms::BOARD_BUTTON),
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_DMA_B, syms::BOARD_DMA_B),
        (BOARD_PIT, syms::BOARD_PIT),
        (BOARD_GPT1, syms::BOARD_GPT1),
        (BOARD_GPT2, syms::BOARD_GPT2),
        (BOARD_SPI, syms::BOARD_SPI),
        (BOARD_PWM, syms::BOARD_PWM),
        (BOARD_USB1, syms::BOARD_USB1),
        (BOARD_SWTASK0, syms::BOARD_SWTASK0),
    ];
}

pub use interrupt as Interrupt;

/// Helpers for the clock_out example.
pub mod clock_out {
    use crate::hal::ccm::output_source::{clko1::Selection as Clko1, clko2::Selection as Clko2};

    pub const CLKO1_SELECTIONS: [Clko1; 8] = [
        Clko1::OscRc48MDiv2,
        Clko1::Osc24M,
        Clko1::OscRc400M,
        Clko1::OscRc16M,
        Clko1::SysPll2Pfd2,
        Clko1::SysPll2CLK,
        Clko1::SysPll3Pfd1,
        Clko1::SysPll1Div5,
    ];
    pub const CLKO2_SELECTIONS: [Clko2; 8] = [
        Clko2::OscRc48MDiv2,
        Clko2::Osc24M,
        Clko2::OscRc400M,
        Clko2::OscRc16M,
        Clko2::SysPll2Pfd3,
        Clko2::OscRc400M,
        Clko2::SysPll3Pfd1,
        Clko2::AudioPllClk,
    ];

    pub const MAX_DIVIDER_VALUE: u32 = 256;
}
