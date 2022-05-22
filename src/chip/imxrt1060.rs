//! i.MX RT 1060 chip family features.
//!
//! Use this module to customize features for the
//! 1060 chips.

#[doc = include_str!("../ccm/README.md")]
#[path = "../ccm"]
pub mod ccm {
    mod common;
    pub use common::*;

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

    #[doc = include_str!("../ccm/analog/README.md")]
    pub mod analog {
        mod common;
        pub use common::*;

        pub mod pll1;
    }

    /// Re-exported by the common clock_gate module.
    mod root_clock_gates {
        use super::clock_gate;

        /// All clock gates downstream of the PERCLK root clock.
        pub const PERCLK_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
            clock_gate::pit(),
            clock_gate::gpt_bus::<1>(),
            clock_gate::gpt_bus::<2>(),
            clock_gate::gpt_serial::<1>(),
            clock_gate::gpt_serial::<2>(),
        ];

        /// All clock gates downstream of the UART root clock.
        pub const UART_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
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
        pub const LPSPI_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
            clock_gate::lpspi::<1>(),
            clock_gate::lpspi::<2>(),
            clock_gate::lpspi::<3>(),
            clock_gate::lpspi::<4>(),
        ];

        /// All clock gates downstream of the LPI2C root clock.
        pub const LPI2C_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
            clock_gate::lpi2c::<1>(),
            clock_gate::lpi2c::<2>(),
            clock_gate::lpi2c::<3>(),
            clock_gate::lpi2c::<4>(),
        ];

        /// All clock gates downstream of the IPG root clock.
        pub const IPG_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
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
        ];
    }

    #[doc = include_str!("../ccm/clock_tree/README.md")]
    pub mod clock_tree {
        mod common;
        pub use common::*;

        mod pll1_ahb;
        pub use pll1_ahb::ahb_frequency;
        use pll1_ahb::configure_ahb;
    }
}
