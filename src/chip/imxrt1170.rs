//! i.MX RT 1170 chip family features.

pub use drivers::{ccm_11xx as ccm, dma, gpio, pit, snvs, timer, usbphy};

pub(crate) mod iomuxc {
    pub use super::config::pads;
    use crate::ral;

    /// Transform the `imxrt-ral` IOMUXC instances into pad objects.
    pub fn into_pads(_: ral::iomuxc::IOMUXC, _: ral::iomuxc_lpsr::IOMUXC_LPSR) -> pads::Pads {
        // Safety: acquiring pads has the same safety implications
        // as acquiring the IOMUXC instances. The user has already
        // assumed the unsafety.
        unsafe { pads::Pads::new() }
    }
}

mod drivers {
    pub mod dma;
    pub mod gpio;
    pub mod pit;
    pub mod snvs;
    pub mod timer;
    pub mod usbphy;

    pub mod ccm_11xx;
}

#[path = "drivers"]
pub(crate) mod config {
    pub(crate) const DMA_CHANNEL_COUNT: usize = 32;

    pub use imxrt_iomuxc::imxrt1170 as pads;

    #[path = "ccm_11xx"]
    pub(crate) mod ccm {
        pub(crate) mod clko {
            /// CLKO1 output clock selections.
            #[repr(u32)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

            /// CLKO2 output clock selections.
            #[repr(u32)]
            #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
}
