//! i.MX RT 1010 chip family features.
//!
//! Use this module to customize features for the
//! 1010 chips.

pub use imxrt_iomuxc::imxrt1010 as pads;

#[path = "ccm"]
pub(crate) mod ccm {
    // There's no ARM divider.

    #[path = "pre_periph_clk_pll6.rs"]
    pub mod pre_periph_clk;

    mod periph_clk2_sel;
    // There's no divider at the PERIPH_CLK2 output.

    /// Peripheral clock 2.
    pub mod periph_clk2 {
        pub use super::periph_clk2_sel::*;
    }

    pub(crate) mod analog {
        #[path = "pll6_500mhz.rs"]
        pub mod pll6;
    }

    /// Re-exported by the common clock_gate module.
    pub(crate) mod clock_gate {
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
        ];

        /// All clock gates downstream of the LPSPI root clock.
        pub const LPSPI_CLOCK_GATES: &[clock_gate::Locator] =
            &[clock_gate::lpspi::<1>(), clock_gate::lpspi::<2>()];

        /// All clock gates downstream of the LPI2C root clock.
        pub const LPI2C_CLOCK_GATES: &[clock_gate::Locator] =
            &[clock_gate::lpi2c::<1>(), clock_gate::lpi2c::<2>()];

        /// All SAI clock gates.
        pub const SAI_CLOCK_GATES: &[clock_gate::Locator] =
            &[clock_gate::sai::<1>(), clock_gate::sai::<3>()];

        /// All clock gates downstream of the IPG root clock.
        pub const IPG_CLOCK_GATES: &[clock_gate::Locator] = &[
            clock_gate::adc::<{ crate::ral::SOLE_INSTANCE }>(),
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
            clock_gate::snvs_lp(),
            clock_gate::snvs_hp(),
            clock_gate::usb(),
        ];
    }

    pub(crate) mod clko {
        /// CLKO1 output clock selections.
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Clko1Selection {
            /// PLL3 divided by 2.
            Pll3SwClkDiv2 = 0b0000,
            /// PLL2 divided by 2.
            Pll2Div2 = 0b0001,
            /// ENET PLL divided by 2.
            EnetPllDiv2 = 0b0010,
            /// Core clock (AHB) root.
            AhbClk = 0b1011,
            /// IPG clock root.
            IpgClk = 0b1100,
            /// PERCLK clock root.
            Perclk = 0b1101,
            /// PLL4 main clock root.
            Pll4MainClk = 0b1111,
        }

        /// CLKO2 output clock selections.
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Clko2Selection {
            /// LPI2C clock root.
            Lpi2cClk = 0b00110,
            /// Oscillator clock root.
            OscClk = 0b01110,
            /// LPSPI clock root.
            LpspiClk = 0b10000,
            /// SAI1 clock root.
            Sai1Clk = 0b10010,
            /// SAI3 clock root.
            Sai3Clk = 0b10100,
            /// Trace clock root.
            TraceClk = 0b10110,
            /// FlexSPI clock root.
            FlexspiClk = 0b11011,
            /// UART clock root.
            UartClk = 0b11100,
            /// SPDIF0 clock root.
            Spdif0Clk = 0b11101,
        }

        impl Clko2Selection {
            /// Trace clock root.
            ///
            /// Prefer [`TraceClk`](Self::TraceClk), which is correctly spelled.
            #[deprecated(
                since = "0.5.1",
                note = "Use the correctly-spelled 'TraceClk' variant."
            )]
            #[allow(non_upper_case_globals)]
            pub const TracClk: Clko2Selection = Clko2Selection::TraceClk;
        }
    }

    ccm_flexio!(
        flexio1_clk, "FLEXIO1",
        divider: (CS1CDR, FLEXIO1_CLK_PODF),
        predivider: (CS1CDR, FLEXIO1_CLK_PRED),
        selection: (CSCMR2, FLEXIO1_CLK_SEL),
    );
}
pub(crate) const DMA_CHANNEL_COUNT: usize = 16;
