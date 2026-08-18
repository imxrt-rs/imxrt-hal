//! Coral Dev Board Micro configuration, supporting CM7 applications.
//!
//! # Loader boot
//!
//! Flash holds coralmicro's `elf_loader`, which loads and calls the
//! application, so this board has no FCB and no `imxrt-rt`. It selects the
//! `loader-boot` feature instead. [`configure`] touches clock roots and gates
//! only, never the PLLs, since the loader has already configured those.
//!
//! # No debug probe
//!
//! There is no JTAG or SWD, so log messages go over USB CDC. A panic calls
//! [`reset_to_flash`] rather than spinning, since a spinning image still owns
//! USB, and getting the loader back then means holding the user button while
//! pressing reset. [`take_panic_record`] recovers where the panic happened.
//!
//! # Scope
//!
//! CM7 only. Nothing here configures the CM4 or the Edge TPU.
//!
//! # Pin muxing
//!
//! `imxrt-iomuxc` 0.3.2 reaches none of this board's LPUART6, LPSPI6, FlexPWM
//! or `IOMUXC_SNVS` pads, so [`configure_pins`] muxes them with raw register
//! writes and the drivers use `without_pins` constructors.
//!
//! # Pinout
//!
//! Both 12-pin headers ship unpopulated, and their pads are 1.8V. The values
//! come from coralmicro's `libs/base/gpio.cc`, `libs/base/spi.cc` and the
//! `pin_mux.c` in its vendored RT1176 SDK.
//!
//! | Function | Pad | Signal | Where |
//! |---|---|---|---|
//! | [`Led`] (green, "user") | `GPIO_SNVS_03` | `GPIO13_IO06` | on board |
//! | [`StatusLed`] (orange) | `GPIO_SNVS_02` | `GPIO13_IO05` | on board |
//! | [`Button`] ("user") | `GPIO_SNVS_00` | `GPIO13_IO03` | on board |
//! | [`Console`] TX | `GPIO_EMC_B1_40` | `LPUART6_TXD` | J9 pin 5 |
//! | [`Console`] RX | `GPIO_EMC_B1_41` | `LPUART6_RXD` | J9 pin 6 |
//! | [`pwm`] A | `GPIO_AD_00` | `FLEXPWM1_PWM0_A` | J9 pin 10 |
//! | [`pwm`] B | `GPIO_AD_01` | `FLEXPWM1_PWM0_B` | J9 pin 9 |
//! | `pwm3` A | `GPIO_EMC_B2_00` | `FLEXPWM3_PWM0_A` | J9 pin 7 |
//! | `pwm3` B | `GPIO_EMC_B2_01` | `FLEXPWM3_PWM0_B` | J9 pin 8 |
//! | [`Spi`] PCS0 | `GPIO_LPSR_09` | `LPSPI6_PCS0` | J10 pin 5 |
//! | [`Spi`] SCK | `GPIO_LPSR_10` | `LPSPI6_SCK` | J10 pin 6 |
//! | [`Spi`] SDO | `GPIO_LPSR_11` | `LPSPI6_SOUT` | J10 pin 7 |
//! | [`Spi`] SDI | `GPIO_LPSR_12` | `LPSPI6_SIN` | J10 pin 8 |
//! | [`I2c`] SDA | `GPIO_LPSR_04` | `LPI2C5_SDA` | PMIC, camera |
//! | [`I2c`] SCL | `GPIO_LPSR_05` | `LPI2C5_SCL` | PMIC, camera |
//!
//! J9 pins 7 and 8 are `UART6_CTS`/`UART6_RTS` in coralmicro's configuration.
//! FlexPWM3 takes them here, giving up console flow control for four PWM
//! channels.

use crate::{GPT1_DIVIDER, GPT2_DIVIDER, RUN_MODE, hal, iomuxc::imxrt1170 as iomuxc, ral};

mod imxrt11xx {
    pub(super) mod clock_tree;
}

use imxrt11xx::clock_tree;

/// Log messages come out over USB CDC; there's no debug probe on this board.
pub(crate) const DEFAULT_LOGGING_BACKEND: crate::logging::Backend = crate::logging::Backend::Usbd;

/// SNVS_LP Control Register (`SNVS_BASE + 0x38`).
const SNVS_LPCR: u32 = 0x40C9_0038;
/// `LPCR[TOP]`: turn off system power, which the SNVS domain then restores.
const SNVS_LPCR_TOP: u32 = 0x40;

/// Roughly 100ms at the clock the loader leaves the M7 on. A cycle delay, not a
/// timer, since a panic can reach here before [`crate::new`] runs.
const SNVS_SETTLE_CYCLES: u32 = 80_000_000;

/// Reset the board into whatever is programmed in flash. Never returns.
///
/// SNVS is what coralmicro's `ResetToFlash()` uses, but it only works if the
/// SRTC was initialised, which this package does not do, so `SYSRESETREQ` backs
/// it up. An image that fails to reset never gives USB back, and recovering
/// from that takes a button press on the board.
pub fn reset_to_flash() -> ! {
    cortex_m::interrupt::disable();
    // Safety: SNVS_LP is memory-mapped at a fixed address, interrupts are off,
    // and this function never returns, so no one observes the aliasing.
    unsafe {
        let lpcr = SNVS_LPCR as *mut u32;
        core::ptr::write_volatile(lpcr, core::ptr::read_volatile(lpcr) | SNVS_LPCR_TOP);
    }

    cortex_m::asm::delay(SNVS_SETTLE_CYCLES);

    cortex_m::peripheral::SCB::sys_reset()
}

/// `SNVS_LPGPR0`, the first of four general purpose words in the SNVS low power
/// domain (`SNVS_BASE + 0x100`). They survive [`reset_to_flash`].
const SNVS_LPGPR0: u32 = 0x40C9_0100;

/// Marks [`SNVS_LPGPR0`] as holding a panic record.
const PANIC_MAGIC: u32 = 0x434D_5041; // "CMPA"

/// Where the last panic happened, recovered from the reset it caused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PanicRecord {
    /// Source line of the panic.
    pub line: u32,
    /// FNV-1a hash of the source file path. Hash your candidates the same way.
    pub file_hash: u32,
    /// Stack pointer at the panic. Compare against `_stack_end` for overflow.
    pub stack_pointer: u32,
}

/// FNV-1a over the bytes of `text`.
const fn fnv1a(text: &str) -> u32 {
    let bytes = text.as_bytes();
    let mut hash: u32 = 0x811C_9DC5;
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u32;
        hash = hash.wrapping_mul(0x0100_0193);
        index += 1;
    }
    hash
}

/// Take the record left by the last panic, clearing it.
pub fn take_panic_record() -> Option<PanicRecord> {
    // Safety: SNVS_LP is memory mapped at a fixed address and these four words
    // are general purpose storage that nothing else in this package uses.
    unsafe {
        let gpr = SNVS_LPGPR0 as *mut u32;
        if core::ptr::read_volatile(gpr) != PANIC_MAGIC {
            return None;
        }
        let record = PanicRecord {
            line: core::ptr::read_volatile(gpr.add(1)),
            file_hash: core::ptr::read_volatile(gpr.add(2)),
            stack_pointer: core::ptr::read_volatile(gpr.add(3)),
        };
        core::ptr::write_volatile(gpr, 0);
        Some(record)
    }
}

/// Panicking resets the board into the flash-resident loader.
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let (line, file_hash) = match info.location() {
        Some(location) => (location.line(), fnv1a(location.file())),
        None => (0, 0),
    };
    let stack_pointer: u32;
    // Safety: reading SP clobbers nothing.
    unsafe { core::arch::asm!("mov {}, sp", out(reg) stack_pointer, options(nomem, nostack)) };

    // Safety: as in `take_panic_record`. Interrupts are about to be disabled by
    // `reset_to_flash` and this never returns, so nobody observes a half-written
    // record.
    unsafe {
        let gpr = SNVS_LPGPR0 as *mut u32;
        core::ptr::write_volatile(gpr.add(1), line);
        core::ptr::write_volatile(gpr.add(2), file_hash);
        core::ptr::write_volatile(gpr.add(3), stack_pointer);
        core::ptr::write_volatile(gpr, PANIC_MAGIC);
    }

    reset_to_flash()
}

#[defmt::panic_handler]
fn defmt_panic() -> ! {
    reset_to_flash()
}

use hal::ccm::clock_gate;
const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio(),
    clock_gate::dma(),
    clock_gate::pit::<1>(),
    clock_gate::gpt::<1>(),
    clock_gate::gpt::<2>(),
    clock_gate::usb(),
    clock_gate::lpuart::<{ CONSOLE_INSTANCE }>(),
    clock_gate::lpspi::<SPI_INSTANCE>(),
    clock_gate::flexpwm::<{ PWM_INSTANCE }>(),
    clock_gate::flexpwm::<{ PWM3_INSTANCE }>(),
    clock_gate::lpi2c::<{ I2C_INSTANCE }>(),
    clock_gate::snvs(),
];

pub(crate) unsafe fn configure() {
    let mut ccm = unsafe { ral::ccm::CCM::instance() };

    prepare_clock_tree(&mut ccm);
    CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(&mut ccm, clock_gate::ON));
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
pub const UART_CLK_FREQUENCY: u32 = clock_tree::lpuart_frequency::<{ CONSOLE_INSTANCE }>(RUN_MODE);
pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);
pub const LPSPI_CLK_FREQUENCY: u32 = clock_tree::lpspi_frequency::<SPI_INSTANCE>(RUN_MODE);
pub const LPI2C_CLK_FREQUENCY: u32 = clock_tree::lpi2c_frequency::<I2C_INSTANCE>(RUN_MODE);
pub const PWM_PRESCALER: hal::flexpwm::Prescaler = hal::flexpwm::Prescaler::Prescaler8;
pub const PWM_FREQUENCY: u32 = clock_tree::bus_frequency(RUN_MODE) / PWM_PRESCALER.divider();

/// The green "user" LED. GPIO13 is always on, so it needs no clock gate.
pub type Led = hal::gpio::Output;

/// The orange "status" LED.
pub type StatusLed = hal::gpio::Output;

/// The "user" button. Pulled up, brought to GND on press.
pub type Button = hal::gpio::Input;

/// The UART console, on J9 pins 5 and 6 at 1.8V. coralmicro drives it at the
/// same baud and documents a USB-to-TTL cable for it.
pub type Console = hal::lpuart::Lpuart;
const CONSOLE_INSTANCE: u8 = 6;

/// SPI peripheral. Present whether or not the `"spi"` feature is on, since
/// these pins collide with nothing else this package configures.
pub type Spi = hal::lpspi::Lpspi;
const SPI_INSTANCE: u8 = 6;

/// The PMIC's bus, shared with the camera. Not one of the two buses coralmicro
/// brings out to the headers.
pub type I2c = hal::lpi2c::Lpi2c;
pub type I2cPins = hal::lpi2c::Pins<
    iomuxc::gpio_lpsr::GPIO_LPSR_05, // SCL
    iomuxc::gpio_lpsr::GPIO_LPSR_04, // SDA
>;
const I2C_INSTANCE: u8 = 5;

/// The error an [`I2c`] transaction fails with.
pub type I2cError = <I2c as eh1::i2c::ErrorType>::Error;

/// The PMIC's I2C address. It's a Dialog DA9053 derivative, and everything here
/// comes from coralmicro's `libs/pmic/pmic.cc`.
pub const PMIC_I2C_ADDRESS: u8 = 0x58;

/// `PAGE_CON`, which lives at offset zero of every page.
const PMIC_PAGE_CON: u8 = 0x00;

/// `PAGE_CON[REVERT]`: go back to page zero after the next access. A page
/// selection is good for one transaction only.
const PMIC_PAGE_REVERT: u8 = 0x80;

/// The PMIC's `DEVICE_ID` register, readable whether or not any rail is up.
pub const PMIC_DEVICE_ID: u16 = 0x181;

/// A power rail that the PMIC gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum PmicRail {
    /// `LDO4`: the microphone's 1.8V supply.
    Microphone1V8,
}

impl PmicRail {
    /// The rail's `LDOn_CONT` register. Bit 0 is the enable.
    const fn control_register(self) -> u16 {
        match self {
            Self::Microphone1V8 => 0x029, // LDO4_CONT
        }
    }
}

/// Point the PMIC at the page that `register` lives in.
///
/// Registers are eight bits plus a page, written as the 12-bit number coralmicro
/// uses: bits 8:7 are the page, and the offset keeps all eight of its bits, so
/// `0x181` is offset `0x81` of page 3.
fn pmic_set_page(i2c: &mut I2c, register: u16) -> Result<(), I2cError> {
    use eh1::i2c::I2c as _;
    let page = ((register >> 7) & 0x3) as u8;
    i2c.write(PMIC_I2C_ADDRESS, &[PMIC_PAGE_CON, PMIC_PAGE_REVERT | page])
}

/// Read one PMIC register.
pub fn pmic_read(i2c: &mut I2c, register: u16) -> Result<u8, I2cError> {
    use eh1::i2c::I2c as _;
    pmic_set_page(i2c, register)?;
    let mut value = [0u8; 1];
    i2c.write_read(PMIC_I2C_ADDRESS, &[register as u8], &mut value)?;
    Ok(value[0])
}

/// Write one PMIC register.
pub fn pmic_write(i2c: &mut I2c, register: u16, value: u8) -> Result<(), I2cError> {
    use eh1::i2c::I2c as _;
    pmic_set_page(i2c, register)?;
    i2c.write(PMIC_I2C_ADDRESS, &[register as u8, value])
}

/// Turn one PMIC rail on or off.
pub fn set_pmic_rail(i2c: &mut I2c, rail: PmicRail, enable: bool) -> Result<(), I2cError> {
    let register = rail.control_register();
    let control = pmic_read(i2c, register)?;
    let control = if enable { control | 1 } else { control & !1 };
    pmic_write(i2c, register, control)
}

/// Returns `true` if the rail's enable bit is set.
pub fn is_pmic_rail_enabled(i2c: &mut I2c, rail: PmicRail) -> Result<bool, I2cError> {
    Ok(pmic_read(i2c, rail.control_register())? & 1 != 0)
}

const PWM_INSTANCE: u8 = 1;
const PWM3_INSTANCE: u8 = 3;

/// PWM components.
///
/// There are no pin types here, unlike the other 1170 boards, because
/// `configure_pins` muxes these pads by hand.
pub mod pwm {
    use crate::hal::flexpwm;

    pub use flexpwm::Pwm;

    pub(super) const N: u8 = super::PWM_INSTANCE;
    pub(super) const N3: u8 = super::PWM3_INSTANCE;
    pub const SM: flexpwm::SM = flexpwm::SM::SM0;

    pub use flexpwm::Channel::*;
}

/// Opaque structure for managing GPIO ports.
///
/// Exposes methods to configure your board's GPIOs.
pub struct GpioPorts {
    gpio13: hal::gpio::Port,
}

impl GpioPorts {
    /// Returns the GPIO port for the button and both LEDs.
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port {
        &mut self.gpio13
    }
}

/// Coral Dev Board Micro specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub status_led: StatusLed,
    pub button: Button,
    pub ports: GpioPorts,
    pub console: Console,
    pub spi: Spi,
    pub pwm: pwm::Pwm,
    /// A second FlexPWM instance, with the same submodule and channels.
    pub pwm3: pwm::Pwm,
    pub i2c: I2c,
}

impl Specifics {
    pub(crate) fn new(common: &mut crate::Common) -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        // The LEDs and the button are IOMUXC_SNVS pads on GPIO13, which
        // imxrt-iomuxc doesn't model. Same situation, and the same workaround,
        // as the 1170 EVK's wakeup button.
        let iomuxc_snvs = unsafe { ral::iomuxc_snvs::IOMUXC_SNVS::instance() };
        // ALT5 selects GPIO13 on all three pads.
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_MUX_CTL_PAD_GPIO_SNVS_00_DIG, MUX_MODE: 5);
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_MUX_CTL_PAD_GPIO_SNVS_02_DIG, MUX_MODE: 5);
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_MUX_CTL_PAD_GPIO_SNVS_03_DIG, MUX_MODE: 5);
        // Pull up, normal drive: 0x0C, which is what coralmicro writes to all
        // three (libs/base/gpio.cc). The button is brought to GND on press.
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_PAD_CTL_PAD_GPIO_SNVS_00_DIG, PUS: 1, PUE: 1, DSE: 0);
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_PAD_CTL_PAD_GPIO_SNVS_02_DIG, PUS: 1, PUE: 1, DSE: 0);
        ral::write_reg!(ral::iomuxc_snvs, iomuxc_snvs, SW_PAD_CTL_PAD_GPIO_SNVS_03_DIG, PUS: 1, PUE: 1, DSE: 0);

        let mut gpio13 = hal::gpio::Port::new(unsafe { ral::gpio::GPIO13::instance() });
        let led = hal::gpio::Output::without_pin(&mut gpio13, 6);
        let status_led = hal::gpio::Output::without_pin(&mut gpio13, 5);
        let button = hal::gpio::Input::without_pin(&mut gpio13, 3);

        let console = unsafe { ral::lpuart::Instance::<{ CONSOLE_INSTANCE }>::instance() };
        let mut console = hal::lpuart::Lpuart::without_pins(console);
        console.disable(|console| {
            console.set_baud(&CONSOLE_BAUD);
            console.set_parity(None);
        });
        hal::usbphy::restart_pll(&mut common.usbphy1);

        let spi = {
            let lpspi6 = unsafe { ral::lpspi::LPSPI6::instance() };
            let mut spi = Spi::without_pins(lpspi6);
            spi.disabled(|spi| {
                spi.set_clock_hz(LPSPI_CLK_FREQUENCY, super::SPI_BAUD_RATE_FREQUENCY);
            });
            spi
        };

        let pwm = pwm::Pwm::new::<{ pwm::N }>(unsafe { ral::pwm::PWM1::instance() });
        let pwm3 = pwm::Pwm::new::<{ pwm::N3 }>(unsafe { ral::pwm::PWM3::instance() });

        let i2c = {
            let lpi2c5 = unsafe { ral::lpi2c::LPI2C5::instance() };
            I2c::with_pins(
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
            status_led,
            button,
            ports: GpioPorts { gpio13 },
            console,
            spi,
            pwm,
            pwm3,
            i2c,
        }
    }
}

/// Configure board pins.
///
/// Mux modes, daisy values and pad control words are coralmicro's, from the
/// `IOMUXC_*` five-tuples in the RT1176 SDK's `fsl_iomuxc.h`.
fn configure_pins(_: &mut super::Pads) {
    // Safety: we have exclusive ownership of the (higher-level) IOMUXC and
    // IOMUXC_LPSR instances. The `Pads` argument is taken by reference purely to
    // prove that.
    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let iomuxc_lpsr = unsafe { ral::iomuxc_lpsr::IOMUXC_LPSR::instance() };

    // Console: LPUART6 on J9 pins 5 and 6, ALT3, no daisy. The EMC pads encode
    // drive and pull differently to the AD pads: PDRV, then a two-bit PULL where
    // 0b11 is "no pull". 0x0E all told, as coralmicro writes.
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_EMC_B1_40, MUX_MODE: 3);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_EMC_B1_41, MUX_MODE: 3);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_EMC_B1_40, PDRV: 1, PULL: 0b11);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_EMC_B1_41, PDRV: 1, PULL: 0b11);

    // SPI: LPSPI6 on J10 pins 5 through 8, ALT4 throughout, no daisy needed.
    // High drive strength, slow slew, no pulls, not open drain.
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_MUX_CTL_PAD_GPIO_LPSR_09, MUX_MODE: 4); // PCS0
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_MUX_CTL_PAD_GPIO_LPSR_10, MUX_MODE: 4); // SCK
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_MUX_CTL_PAD_GPIO_LPSR_11, MUX_MODE: 4); // SOUT
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_MUX_CTL_PAD_GPIO_LPSR_12, MUX_MODE: 4); // SIN
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_09, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_10, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_11, DSE: DSE_1_HIGH_DRIVER);
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_12, DSE: DSE_1_HIGH_DRIVER);

    // I2C open drain. `imxrt-iomuxc`'s `lpi2c::prepare()` deliberately does
    // *not* set the open drain bit when only the 1170 feature is enabled; see
    // imxrt-rs/imxrt-iomuxc#28. It doesn't touch any other PAD_CTL bit either,
    // so setting ODE here survives the `Lpi2c::with_pins` call that follows.
    // This bus has external pull-ups and more than one master, so driving it
    // push-pull would have them fighting.
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_04, ODE_LPSR: 1); // SDA
    ral::write_reg!(ral::iomuxc_lpsr, iomuxc_lpsr, SW_PAD_CTL_PAD_GPIO_LPSR_05, ODE_LPSR: 1); // SCL

    // PWM. The ALT differs per pad: 4 on the AD pads, 0xB on the EMC_B2 pads.
    // The input-select (daisy) registers must be written too, since FlexPWM's A
    // and B inputs are shared between several pads.
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_00, MUX_MODE: 4); // FLEXPWM1_PWM0_A
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_AD_01, MUX_MODE: 4); // FLEXPWM1_PWM0_B
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_EMC_B2_00, MUX_MODE: 0xB); // FLEXPWM3_PWM0_A
    ral::write_reg!(ral::iomuxc, iomuxc, SW_MUX_CTL_PAD_GPIO_EMC_B2_01, MUX_MODE: 0xB); // FLEXPWM3_PWM0_B
    ral::write_reg!(ral::iomuxc, iomuxc, FLEXPWM1_PWMA_SELECT_INPUT_0, DAISY: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, FLEXPWM1_PWMB_SELECT_INPUT_0, DAISY: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, FLEXPWM3_PWMA_SELECT_INPUT_0, DAISY: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, FLEXPWM3_PWMB_SELECT_INPUT_0, DAISY: 1);
    // High drive strength, no pull on the EMC pads; the AD pads keep coralmicro's
    // pull-up. Both come to 0x0E, through different field layouts.
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_00, DSE: DSE_1_HIGH_DRIVER, PUE: 1, PUS: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_AD_01, DSE: DSE_1_HIGH_DRIVER, PUE: 1, PUS: 1);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_EMC_B2_00, PDRV: 1, PULL: 0b11);
    ral::write_reg!(ral::iomuxc, iomuxc, SW_PAD_CTL_PAD_GPIO_EMC_B2_01, PDRV: 1, PULL: 0b11);
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART6;
    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO13_COMBINED_0_31;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA7_DMA23;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA11_DMA27;
    pub const BOARD_PIT: Interrupt = Interrupt::PIT1;
    pub const BOARD_GPT1: Interrupt = Interrupt::GPT1;
    pub const BOARD_GPT2: Interrupt = Interrupt::GPT2;
    pub const BOARD_SPI: Interrupt = Interrupt::LPSPI6;
    pub const BOARD_PWM: Interrupt = Interrupt::PWM1_0;
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

