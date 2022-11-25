//! i.MX RT 1170 chip family features.

pub(crate) const DMA_CHANNEL_COUNT: usize = 32;

#[path = "ccm"]
pub(crate) mod ccm {
    pub(crate) mod clko {
        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Clko1Selection {
            /// 48MHz RC oscillator, divided by 2.
            OscRc48MDiv2,
            /// 24MHz oscillator.
            Osc24M,
            /// 400MHz RC oscillator.
            OscRc400M,
            /// 16MHz RC oscillator.
            OscRc16M,
            /// PFD2 of PLL2.
            SysPll2Pfd2,
            /// PLL2.
            SysPll2CLK,
            /// PFD1 of PLL3.
            SysPll3Pfd1,
            /// PLL1 divided by 5.
            SysPll1Div5,
        }

        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Clko2Selection {
            /// 48MHz RC oscillator, divided by 2.
            OscRc48MDiv2,
            /// 24MHz oscillator.
            Osc24M,
            /// 400MHz RC oscillator.
            OscRc400M,
            /// 16MHz RC oscillator.
            OscRc16M,
            /// PFD3 of PLL2.
            SysPll2Pfd3,
            /// 48MHz RC oscillator (without any divider).
            OscRc48M,
            /// PFD1 of PLL3.
            SysPll3Pfd1,
            /// Audio PLL.
            AudioPllClk,
        }
    }
}
