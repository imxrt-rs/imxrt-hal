//! i.MX RT 1010 chip family features.
//!
//! Use this module to customize features for the
//! 1010 chips.

#[doc = include_str!("../ccm/README.md")]
#[path = "../ccm"]
pub mod ccm {
    mod common;
    pub use common::*;

    // There's no ARM divider.

    #[path = "pre_periph_clk_pll6.rs"]
    pub mod pre_periph_clk;

    mod periph_clk2_sel;
    // There's no divider at the PERIPH_CLK2 output.

    /// Peripheral clock 2.
    pub mod periph_clk2 {
        pub use super::periph_clk2_sel::*;
    }

    #[doc = include_str!("../ccm/analog/README.md")]
    pub mod analog {
        mod common;
        pub use common::*;

        #[path = "pll6_500mhz.rs"]
        pub mod pll6;
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
        ];

        /// All clock gates downstream of the LPSPI root clock.
        pub const LPSPI_CLOCK_GATES: &[&'static clock_gate::Locator] =
            &[clock_gate::lpspi::<1>(), clock_gate::lpspi::<2>()];

        /// All clock gates downstream of the LPI2C root clock.
        pub const LPI2C_CLOCK_GATES: &[&'static clock_gate::Locator] =
            &[clock_gate::lpi2c::<1>(), clock_gate::lpi2c::<2>()];

        /// All clock gates downstream of the IPG root clock.
        pub const IPG_CLOCK_GATES: &[&'static clock_gate::Locator] = &[
            // TODO ADC.
            clock_gate::dma(),
            clock_gate::flexpwm::<{ crate::ral::SOLE_INSTANCE }>(),
            // GPIOs assume that we're not "fast," since the fast
            // GPIOs run directly off the ARM / AHB clock. This
            // is safe for now, since we don't currently support fast
            // GPIOs.
            clock_gate::gpio::<1>(),
            clock_gate::gpio::<2>(),
            clock_gate::gpio::<5>(),
            clock_gate::trng(),
            clock_gate::snvs(),
        ];
    }

    #[doc = include_str!("../ccm/clock_tree/README.md")]
    pub mod clock_tree {
        mod common;
        pub use common::*;

        mod pll6_ahb;
        pub use pll6_ahb::ahb_frequency;
        use pll6_ahb::configure_ahb;
    }
}

#[path = "../dma.rs"]
pub mod dma;

pub(crate) const DMA_CHANNEL_COUNT: usize = 16;
