//! # IOMUX Controller
//!
//! The IOMUXC controller exposes all processor pads as unique GPIO structs.
//! Each GPIO may be transitioned into various alternatives, or modes, that
//! support different use-cases. All GPIOs are defined in [the `gpio` module](gpio/index.html).
//!
//! [The `pin_config` module](pin_config/index.html) lets users specify pin configurations, like
//! pull-up resistors and pin speeds. See the module-level docs for more information.
//!
//! Finally, the peripheral-specific modules define type tags for pins, and traits that denote
//! peripheral-compatible pins. Each peripheral-specific module, like `spi` and `uart`, defines
//! a `Pin` trait. The list of trait implementors describe which pad and alternative is needed
//! to support that peripheral. For example, an implementor of the `uart::Pin` trait is
//! `GPIO_AD_B1_02<Alt2>`: 
//!
//! ```text
//! impl uart::Pin for GPIO_AD_B1_02<Alt2>
//!    type Direction = TX
//!    type Module = _2
//! ```
//!
//! The listing indicates that, in the second alternative, `GPIO_AD_B1_02` is a UART TX pin for
//! UART2. The HAL's UART peripheral will design to that trait, accepting GPIOs that can satisfy
//! the trait bounds:
//!
//! ```no_run
//! use imxrt_hal;
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! let uarts = peripherals.uart.clock(
//!     // ...
//! #    &mut peripherals.ccm.handle,
//! #    imxrt_hal::ccm::uart::ClockSelect::OSC,
//! #    imxrt_hal::ccm::uart::PrescalarSelect::DIVIDE_1,
//! );
//!
//! // Use the UART2-compatible pins to create a UART peripheral
//! let mut uart = uarts
//!     .uart2
//!     .init(
//!         peripherals.iomuxc.gpio_ad_b1_02.alt2(),
//!         peripherals.iomuxc.gpio_ad_b1_03.alt2(),
//!         115_200,
//!     )
//!     .unwrap();
//! ```

#![allow(non_camel_case_types)]


#[macro_use]
mod macros;

pub mod gpio;
pub mod i2c;
pub mod pin_config;
pub mod pwm;
pub mod spi;
pub mod uart;

// IOMUXC section of docs originally state that there are up to 8
// alternative modes. However, some have up to 10 (like GPIO_AD_B1_00)

/// Pad alternative 0 (type tag)
pub struct Alt0;
/// Pad alternative 1 (type tag)
pub struct Alt1;
/// Pad alternative 2 (type tag)
pub struct Alt2;
/// Pad alternative 3 (type tag)
pub struct Alt3;
/// Pad alternative 4 (type tag)
pub struct Alt4;
/// Pad alternative 5 (type tag)
pub struct Alt5;
/// Pad alternative 6 (type tag)
pub struct Alt6;
/// Pad alternative 7 (type tag)
pub struct Alt7;
/// Pad alternative 8 (type tag)
pub struct Alt8;
/// Pad alternative 9 (type tag)
pub struct Alt9;

pub struct IOMUXC {
    //
    // GPIO_B0
    //
    pub gpio_b0_00: gpio::GPIO_B0_00<Alt5>,
    pub gpio_b0_01: gpio::GPIO_B0_01<Alt5>,
    pub gpio_b0_02: gpio::GPIO_B0_02<Alt5>,
    pub gpio_b0_03: gpio::GPIO_B0_03<Alt5>,
    pub gpio_b0_10: gpio::GPIO_B0_10<Alt5>,
    pub gpio_b0_11: gpio::GPIO_B0_11<Alt5>,

    //
    // GPIO_B1
    //
    pub gpio_b1_00: gpio::GPIO_B1_00<Alt5>,
    pub gpio_b1_01: gpio::GPIO_B1_01<Alt5>,

    //
    // GPIO_AD_B0
    //
    pub gpio_ad_b0_00: gpio::GPIO_AD_B0_00<Alt5>,
    pub gpio_ad_b0_01: gpio::GPIO_AD_B0_01<Alt5>,
    pub gpio_ad_b0_02: gpio::GPIO_AD_B0_02<Alt5>,
    pub gpio_ad_b0_03: gpio::GPIO_AD_B0_03<Alt5>,

    // Note that these pads default to Alt0 rather than Alt5
    // as described in the reference manual in section 11.7
    pub gpio_ad_b0_04: gpio::GPIO_AD_B0_04<Alt0>,
    pub gpio_ad_b0_05: gpio::GPIO_AD_B0_05<Alt0>,
    pub gpio_ad_b0_06: gpio::GPIO_AD_B0_06<Alt0>,
    pub gpio_ad_b0_07: gpio::GPIO_AD_B0_07<Alt0>,
    pub gpio_ad_b0_08: gpio::GPIO_AD_B0_08<Alt0>,
    pub gpio_ad_b0_09: gpio::GPIO_AD_B0_09<Alt0>,
    pub gpio_ad_b0_10: gpio::GPIO_AD_B0_10<Alt0>,
    pub gpio_ad_b0_11: gpio::GPIO_AD_B0_11<Alt0>,

    pub gpio_ad_b0_12: gpio::GPIO_AD_B0_12<Alt5>,
    pub gpio_ad_b0_13: gpio::GPIO_AD_B0_13<Alt5>,
    pub gpio_ad_b0_14: gpio::GPIO_AD_B0_14<Alt5>,
    pub gpio_ad_b0_15: gpio::GPIO_AD_B0_15<Alt5>,

    //
    // GPIO_AD_B1
    //
    pub gpio_ad_b1_00: gpio::GPIO_AD_B1_00<Alt5>,
    pub gpio_ad_b1_01: gpio::GPIO_AD_B1_01<Alt5>,
    pub gpio_ad_b1_02: gpio::GPIO_AD_B1_02<Alt5>,
    pub gpio_ad_b1_03: gpio::GPIO_AD_B1_03<Alt5>,
    pub gpio_ad_b1_04: gpio::GPIO_AD_B1_04<Alt5>,
    pub gpio_ad_b1_05: gpio::GPIO_AD_B1_05<Alt5>,
    pub gpio_ad_b1_06: gpio::GPIO_AD_B1_06<Alt5>,
    pub gpio_ad_b1_07: gpio::GPIO_AD_B1_07<Alt5>,
    pub gpio_ad_b1_08: gpio::GPIO_AD_B1_08<Alt5>,
    pub gpio_ad_b1_09: gpio::GPIO_AD_B1_09<Alt5>,
    pub gpio_ad_b1_10: gpio::GPIO_AD_B1_10<Alt5>,
    pub gpio_ad_b1_11: gpio::GPIO_AD_B1_11<Alt5>,
    pub gpio_ad_b1_12: gpio::GPIO_AD_B1_12<Alt5>,
    pub gpio_ad_b1_13: gpio::GPIO_AD_B1_13<Alt5>,
    pub gpio_ad_b1_14: gpio::GPIO_AD_B1_14<Alt5>,
    pub gpio_ad_b1_15: gpio::GPIO_AD_B1_15<Alt5>,

    //
    // GPIO_SD_B0
    //
    pub gpio_sd_b0_00: gpio::GPIO_SD_B0_00<Alt5>,
    pub gpio_sd_b0_01: gpio::GPIO_SD_B0_01<Alt5>,
    pub gpio_sd_b0_02: gpio::GPIO_SD_B0_02<Alt5>,
    pub gpio_sd_b0_03: gpio::GPIO_SD_B0_03<Alt5>,

    //
    // GPIO_EMC
    //
    pub gpio_emc_04: gpio::GPIO_EMC_04<Alt5>,
    pub gpio_emc_05: gpio::GPIO_EMC_05<Alt5>,
    pub gpio_emc_06: gpio::GPIO_EMC_06<Alt5>,
    pub gpio_emc_07: gpio::GPIO_EMC_07<Alt5>,
    pub gpio_emc_08: gpio::GPIO_EMC_08<Alt5>,
    pub gpio_emc_31: gpio::GPIO_EMC_31<Alt5>,
    pub gpio_emc_32: gpio::GPIO_EMC_32<Alt5>,

    // GPRs
    pub gpr: GPR,
}

impl IOMUXC {
    pub(crate) fn new(_: crate::ral::iomuxc::Instance) -> Self {
        // Intentionally dropping the IOMUXC instance. Once it's dropped,
        // it's not un-taken. So, the user will believe that we own it,
        // and we don't need to carry it around. The user may still access it
        // unsafely, but that's on them.
        //
        // (Instances never implement Drop)
        Self {
            //
            // GPIO_B0
            //
            gpio_b0_00: gpio::GPIO_B0_00::new(),
            gpio_b0_01: gpio::GPIO_B0_01::new(),
            gpio_b0_02: gpio::GPIO_B0_02::new(),
            gpio_b0_03: gpio::GPIO_B0_03::new(),
            gpio_b0_10: gpio::GPIO_B0_10::new(),
            gpio_b0_11: gpio::GPIO_B0_11::new(),

            //
            // GPIO_B1
            //
            gpio_b1_00: gpio::GPIO_B1_00::new(),
            gpio_b1_01: gpio::GPIO_B1_01::new(),

            //
            // GPIO_AD_B0
            //
            gpio_ad_b0_00: gpio::GPIO_AD_B0_00::new(),
            gpio_ad_b0_01: gpio::GPIO_AD_B0_01::new(),
            gpio_ad_b0_02: gpio::GPIO_AD_B0_02::new(),
            gpio_ad_b0_03: gpio::GPIO_AD_B0_03::new(),
            gpio_ad_b0_04: gpio::GPIO_AD_B0_04::new(),
            gpio_ad_b0_05: gpio::GPIO_AD_B0_05::new(),
            gpio_ad_b0_06: gpio::GPIO_AD_B0_06::new(),
            gpio_ad_b0_07: gpio::GPIO_AD_B0_07::new(),
            gpio_ad_b0_08: gpio::GPIO_AD_B0_08::new(),
            gpio_ad_b0_09: gpio::GPIO_AD_B0_09::new(),
            gpio_ad_b0_10: gpio::GPIO_AD_B0_10::new(),
            gpio_ad_b0_11: gpio::GPIO_AD_B0_11::new(),
            gpio_ad_b0_12: gpio::GPIO_AD_B0_12::new(),
            gpio_ad_b0_13: gpio::GPIO_AD_B0_13::new(),
            gpio_ad_b0_14: gpio::GPIO_AD_B0_14::new(),
            gpio_ad_b0_15: gpio::GPIO_AD_B0_15::new(),

            //
            // GPIO_AD_B1
            //
            gpio_ad_b1_00: gpio::GPIO_AD_B1_00::new(),
            gpio_ad_b1_01: gpio::GPIO_AD_B1_01::new(),
            gpio_ad_b1_02: gpio::GPIO_AD_B1_02::new(),
            gpio_ad_b1_03: gpio::GPIO_AD_B1_03::new(),
            gpio_ad_b1_04: gpio::GPIO_AD_B1_04::new(),
            gpio_ad_b1_05: gpio::GPIO_AD_B1_05::new(),
            gpio_ad_b1_06: gpio::GPIO_AD_B1_06::new(),
            gpio_ad_b1_07: gpio::GPIO_AD_B1_07::new(),
            gpio_ad_b1_08: gpio::GPIO_AD_B1_08::new(),
            gpio_ad_b1_09: gpio::GPIO_AD_B1_09::new(),
            gpio_ad_b1_10: gpio::GPIO_AD_B1_10::new(),
            gpio_ad_b1_11: gpio::GPIO_AD_B1_11::new(),
            gpio_ad_b1_12: gpio::GPIO_AD_B1_12::new(),
            gpio_ad_b1_13: gpio::GPIO_AD_B1_13::new(),
            gpio_ad_b1_14: gpio::GPIO_AD_B1_14::new(),
            gpio_ad_b1_15: gpio::GPIO_AD_B1_15::new(),

            //
            // GPIO_SD_B0
            //
            gpio_sd_b0_00: gpio::GPIO_SD_B0_00::new(),
            gpio_sd_b0_01: gpio::GPIO_SD_B0_01::new(),
            gpio_sd_b0_02: gpio::GPIO_SD_B0_02::new(),
            gpio_sd_b0_03: gpio::GPIO_SD_B0_03::new(),

            //
            // GPIO_EMC
            //
            gpio_emc_04: gpio::GPIO_EMC_04::new(),
            gpio_emc_05: gpio::GPIO_EMC_05::new(),
            gpio_emc_06: gpio::GPIO_EMC_06::new(),
            gpio_emc_07: gpio::GPIO_EMC_07::new(),
            gpio_emc_08: gpio::GPIO_EMC_08::new(),
            gpio_emc_31: gpio::GPIO_EMC_31::new(),
            gpio_emc_32: gpio::GPIO_EMC_32::new(),

            // GPRs
            gpr: GPR(()),
        }
    }
}

pub struct GPR(());

impl GPR {
    pub(crate) unsafe fn gpr26(&mut self) -> &crate::ral::RWRegister<u32> {
        &(*crate::ral::iomuxc_gpr::IOMUXC_GPR).GPR26
    }
    pub(crate) unsafe fn gpr27(&mut self) -> &crate::ral::RWRegister<u32> {
        &(*crate::ral::iomuxc_gpr::IOMUXC_GPR).GPR27
    }
}
