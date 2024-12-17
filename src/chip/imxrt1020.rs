//! i.MX RT 1020 chip family features.
//!
//! Use this module to customize features for the
//! 1020 chips.

pub use drivers::{
    adc, ccm_10xx as ccm, dcdc, dma, gpio, iomuxc_10xx as iomuxc, pit, snvs, tempmon, timer, trng,
};

#[macro_use]
mod drivers {
    pub mod adc;
    pub mod dcdc;
    pub mod dma;
    pub mod gpio;
    pub mod pit;
    pub mod snvs;
    pub mod tempmon;
    pub mod timer;
    pub mod trng;

    #[macro_use]
    pub mod ccm_10xx;
    pub mod iomuxc_10xx;
}

#[path = "drivers"]
pub(crate) mod config {
    pub use imxrt_iomuxc::imxrt1020 as pads;

    #[path = "ccm_10xx"]
    pub(crate) mod ccm {
        pub mod arm_divider;

        #[path = "pre_periph_clk_pll6.rs"]
        pub mod pre_periph_clk;

        mod periph_clk2_podf;
        mod periph_clk2_sel;

        /// Peripheral clock 2.
        pub mod periph_clk2 {
            pub use super::periph_clk2_podf::*;
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
                clock_gate::adc::<1>(),
                clock_gate::adc::<2>(),
                clock_gate::dma(),
                clock_gate::flexpwm::<1>(),
                clock_gate::flexpwm::<2>(),
                // GPIOs assume that we're not "fast," since the fast
                // GPIOs run directly off the ARM / AHB clock. This
                // is safe for now, since we don't currently support fast
                // GPIOs.
                clock_gate::gpio::<1>(),
                clock_gate::gpio::<2>(),
                clock_gate::gpio::<3>(),
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
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Clko1Selection {
                /// PLL3 divided by 2.
                Pll3SwClkDiv2 = 0b0000,
                /// PLL2 divided by 2.
                Pll2Div2 = 0b0001,
                /// ENET PLL divided by 2.
                EnetPllDiv2 = 0b0010,
                /// SEMC clock root.
                SemcClk = 0b0101,
                /// AHB clock root.
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
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Clko2Selection {
                /// USDHC1 clock root.
                Usdhc1Clk = 0b00011,
                /// LPI2C clock root.
                Lpi2cClk = 0b00110,
                /// Oscillator clock root.
                OscClk = 0b01110,
                /// LPSPI clock root.
                LpspiClk = 0b10000,
                /// USDHC2 clock root.
                Usdhc2Clk = 0b10001,
                /// SAI1 clock root.
                Sai1Clk = 0b10010,
                /// SAI2 clock root.
                Sai2Clk = 0b10011,
                /// SAI3 clock root.
                Sai3Clk = 0b10100,
                /// Trace clock root.
                TraceClk = 0b10110,
                /// CAN clock root.
                CanClk = 0b10111,
                /// FlexSPI clock root.
                FlexspiClk = 0b11011,
                /// UART clock root.
                UartClk = 0b11100,
                /// SPDIF0 clock root.
                Spdif0Clk = 0b11101,
            }
        }

        ccm_flexio!(
            flexio1_clk, "FLEXIO1",
            divider: (CS1CDR, FLEXIO1_CLK_PODF),
            predivider: (CS1CDR, FLEXIO1_CLK_PRED),
            selection: (CSCMR2, FLEXIO1_CLK_SEL),
        );
    }

    pub(crate) const DMA_CHANNEL_COUNT: usize = 32;
}
