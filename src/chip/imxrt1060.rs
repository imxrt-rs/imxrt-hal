//! i.MX RT 1060 chip family features.
//!
//! Use this module to customize features for the
//! 1060 chips.

#[path = "ccm"]
pub mod root_clock_gen {
    pub mod arm_divider;

    #[path = "pre_periph_clk_pll1.rs"]
    pub mod pre_periph_clk;

    mod periph_clk2_podf;
    mod periph_clk2_sel;

    /// Peripheral clock 2.
    pub mod periph_clk2 {
        pub use super::periph_clk2_podf::*;
        pub use super::periph_clk2_sel::*;
    }
}

#[path = "ccm/analog"]
pub mod plls {
    pub mod pll1;
}

#[path = "ccm/clock_tree"]
pub mod clock_tree {
    mod pll1_ahb;
    pub use pll1_ahb::{ahb_frequency, configure_ahb_ipg};
}

/// Re-exported by the common clock_gate module.
pub mod root_clock_gates {
    use crate::chip::ccm::clock_gate;

    /// All clock gates downstream of the PERCLK root clock.
    pub const PERCLK_CLOCK_GATES: &[clock_gate::Locator] = &[
        clock_gate::pit(),
        clock_gate::gpt_bus::<1>(),
        clock_gate::gpt_bus::<2>(),
        clock_gate::gpt_serial::<1>(),
        clock_gate::gpt_serial::<2>(),
    ];

    /// All clock gates downstream of the UART root clock.
    pub const UART_CLOCK_GATES: &[clock_gate::Locator] = &[
        clock_gate::lpuart::<1>(),
        clock_gate::lpuart::<2>(),
        clock_gate::lpuart::<3>(),
        clock_gate::lpuart::<4>(),
        clock_gate::lpuart::<5>(),
        clock_gate::lpuart::<6>(),
        clock_gate::lpuart::<7>(),
        clock_gate::lpuart::<8>(),
    ];

    /// All clock gates downstream of the LPSPI root clock.
    pub const LPSPI_CLOCK_GATES: &[clock_gate::Locator] = &[
        clock_gate::lpspi::<1>(),
        clock_gate::lpspi::<2>(),
        clock_gate::lpspi::<3>(),
        clock_gate::lpspi::<4>(),
    ];

    /// All clock gates downstream of the LPI2C root clock.
    pub const LPI2C_CLOCK_GATES: &[clock_gate::Locator] = &[
        clock_gate::lpi2c::<1>(),
        clock_gate::lpi2c::<2>(),
        clock_gate::lpi2c::<3>(),
        clock_gate::lpi2c::<4>(),
    ];

    /// All clock gates downstream of the IPG root clock.
    pub const IPG_CLOCK_GATES: &[clock_gate::Locator] = &[
        // TODO ADC.
        clock_gate::dma(),
        clock_gate::flexpwm::<1>(),
        clock_gate::flexpwm::<2>(),
        clock_gate::flexpwm::<3>(),
        clock_gate::flexpwm::<4>(),
        // GPIOs assume that we're not "fast," since the fast
        // GPIOs run directly off the ARM / AHB clock. This
        // is safe for now, since we don't currently support fast
        // GPIOs.
        clock_gate::gpio::<1>(),
        clock_gate::gpio::<2>(),
        clock_gate::gpio::<3>(),
        clock_gate::gpio::<4>(),
        clock_gate::gpio::<5>(),
        clock_gate::trng(),
        clock_gate::snvs(),
        clock_gate::usb(),
    ];
}

// TODO
pub mod clko {
    // #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]

    pub enum Clko1Selection {}

    // #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]

    pub enum Clko2Selection {}
}

pub(crate) const DMA_CHANNEL_COUNT: usize = 32;
