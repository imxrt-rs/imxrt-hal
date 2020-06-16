#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! XTALOSC24M
//!
//! Used by: imxrt1011, imxrt1015

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Miscellaneous Register 0
pub mod MISC0 {

    /// Control bit to power-down the analog bandgap reference circuitry
    pub mod REFTOP_PWD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to disable the self-bias circuit in the analog bandgap
    pub mod REFTOP_SELFBIASOFF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Uses coarse bias currents for startup
            pub const REFTOP_SELFBIASOFF_0: u32 = 0b0;

            /// 0b1: Uses bandgap-based bias currents for best performance.
            pub const REFTOP_SELFBIASOFF_1: u32 = 0b1;
        }
    }

    /// Not related to oscillator.
    pub mod REFTOP_VBGADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Nominal VBG
            pub const REFTOP_VBGADJ_0: u32 = 0b000;

            /// 0b001: VBG+0.78%
            pub const REFTOP_VBGADJ_1: u32 = 0b001;

            /// 0b010: VBG+1.56%
            pub const REFTOP_VBGADJ_2: u32 = 0b010;

            /// 0b011: VBG+2.34%
            pub const REFTOP_VBGADJ_3: u32 = 0b011;

            /// 0b100: VBG-0.78%
            pub const REFTOP_VBGADJ_4: u32 = 0b100;

            /// 0b101: VBG-1.56%
            pub const REFTOP_VBGADJ_5: u32 = 0b101;

            /// 0b110: VBG-2.34%
            pub const REFTOP_VBGADJ_6: u32 = 0b110;

            /// 0b111: VBG-3.12%
            pub const REFTOP_VBGADJ_7: u32 = 0b111;
        }
    }

    /// Status bit that signals the analog bandgap voltage is up and stable
    pub mod REFTOP_VBGUP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configure the analog behavior in stop mode.Not related to oscillator.
    pub mod STOP_MODE_CONFIG {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;
            pub const STOP_MODE_CONFIG_0: u32 = 0b00;

            /// 0b01: Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;
            pub const STOP_MODE_CONFIG_1: u32 = 0b01;

            /// 0b10: XtalOsc=off, RCOsc=on, Old BG=on, New BG=off.
            pub const STOP_MODE_CONFIG_2: u32 = 0b10;

            /// 0b11: XtalOsc=off, RCOsc=on, Old BG=off, New BG=on.
            pub const STOP_MODE_CONFIG_3: u32 = 0b11;
        }
    }

    /// This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN.
    pub mod DISCON_HIGH_SNVS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Turn on the switch
            pub const DISCON_HIGH_SNVS_0: u32 = 0b0;

            /// 0b1: Turn off the switch
            pub const DISCON_HIGH_SNVS_1: u32 = 0b1;
        }
    }

    /// This field determines the bias current in the 24MHz oscillator
    pub mod OSC_I {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Nominal
            pub const NOMINAL: u32 = 0b00;

            /// 0b01: Decrease current by 12.5%
            pub const MINUS_12_5_PERCENT: u32 = 0b01;

            /// 0b10: Decrease current by 25.0%
            pub const MINUS_25_PERCENT: u32 = 0b10;

            /// 0b11: Decrease current by 37.5%
            pub const MINUS_37_5_PERCENT: u32 = 0b11;
        }
    }

    /// Status bit that signals that the output of the 24-MHz crystal oscillator is stable
    pub mod OSC_XTALOK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit enables the detector that signals when the 24MHz crystal oscillator is stable.
    pub mod OSC_XTALOK_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block
    pub mod CLKGATE_CTRL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allow the logic to automatically gate the clock when the XTAL is powered down.
            pub const ALLOW_AUTO_GATE: u32 = 0b0;

            /// 0b1: Prevent the logic from ever gating off the clock.
            pub const NO_AUTO_GATE: u32 = 0b1;
        }
    }

    /// This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block
    pub mod CLKGATE_DELAY {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0.5ms
            pub const CLKGATE_DELAY_0: u32 = 0b000;

            /// 0b001: 1.0ms
            pub const CLKGATE_DELAY_1: u32 = 0b001;

            /// 0b010: 2.0ms
            pub const CLKGATE_DELAY_2: u32 = 0b010;

            /// 0b011: 3.0ms
            pub const CLKGATE_DELAY_3: u32 = 0b011;

            /// 0b100: 4.0ms
            pub const CLKGATE_DELAY_4: u32 = 0b100;

            /// 0b101: 5.0ms
            pub const CLKGATE_DELAY_5: u32 = 0b101;

            /// 0b110: 6.0ms
            pub const CLKGATE_DELAY_6: u32 = 0b110;

            /// 0b111: 7.0ms
            pub const CLKGATE_DELAY_7: u32 = 0b111;
        }
    }

    /// This field indicates which chip source is being used for the rtc clock.
    pub mod RTC_XTAL_SOURCE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal ring oscillator
            pub const RTC_XTAL_SOURCE_0: u32 = 0b0;

            /// 0b1: RTC_XTAL
            pub const RTC_XTAL_SOURCE_1: u32 = 0b1;
        }
    }

    /// This field powers down the 24M crystal oscillator if set true.
    pub mod XTAL_24M_PWD {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Predivider for the source clock of the PLL's. Not related to oscillator.
    pub mod VID_PLL_PREDIV {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Divide by 1
            pub const VID_PLL_PREDIV_0: u32 = 0b0;

            /// 0b1: Divide by 2
            pub const VID_PLL_PREDIV_1: u32 = 0b1;
        }
    }
}

/// Miscellaneous Register 0
pub mod MISC0_SET {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_CLR {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_TOG {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::VID_PLL_PREDIV;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// XTAL OSC (LP) Control Register
pub mod LOWPWR_CTRL {

    /// RC Osc. enable control.
    pub mod RC_OSC_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use XTAL OSC to source the 24MHz clock
            pub const RC_OSC_EN_0: u32 = 0b0;

            /// 0b1: Use RC OSC
            pub const RC_OSC_EN_1: u32 = 0b1;
        }
    }

    /// Select the source for the 24MHz clock.
    pub mod OSC_SEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: XTAL OSC
            pub const OSC_SEL_0: u32 = 0b0;

            /// 0b1: RC OSC
            pub const OSC_SEL_1: u32 = 0b1;
        }
    }

    /// Bandgap select. Not related to oscillator.
    pub mod LPBG_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal power bandgap
            pub const LPBG_SEL_0: u32 = 0b0;

            /// 0b1: Low power bandgap
            pub const LPBG_SEL_1: u32 = 0b1;
        }
    }

    /// Low power bandgap test bit. Not related to oscillator.
    pub mod LPBG_TEST {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low power reftop ibias disable. Not related to oscillator.
    pub mod REFTOP_IBIAS_OFF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// L1 power gate control. Used as software override. Not related to oscillator.
    pub mod L1_PWRGATE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// L2 power gate control. Used as software override. Not related to oscillator.
    pub mod L2_PWRGATE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CPU power gate control. Used as software override. Test purpose only Not related to oscillator.
    pub mod CPU_PWRGATE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Display logic power gate control. Used as software override. Not related to oscillator.
    pub mod DISPLAY_PWRGATE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// For debug purposes only
    pub mod RCOSC_CG_OVERRIDE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use
    pub mod XTALOSC_PWRUP_DELAY {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 0.25ms
            pub const XTALOSC_PWRUP_DELAY_0: u32 = 0b00;

            /// 0b01: 0.5ms
            pub const XTALOSC_PWRUP_DELAY_1: u32 = 0b01;

            /// 0b10: 1ms
            pub const XTALOSC_PWRUP_DELAY_2: u32 = 0b10;

            /// 0b11: 2ms
            pub const XTALOSC_PWRUP_DELAY_3: u32 = 0b11;
        }
    }

    /// Status of the 24MHz xtal oscillator.
    pub mod XTALOSC_PWRUP_STAT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not stable
            pub const XTALOSC_PWRUP_STAT_0: u32 = 0b0;

            /// 0b1: Stable and ready to use
            pub const XTALOSC_PWRUP_STAT_1: u32 = 0b1;
        }
    }

    /// Display power gate control. Used as software mask. Set to zero to force ungated.
    pub mod MIX_PWRGATE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPU power gate control. Used as software mask. Set to zero to force ungated.
    pub mod GPU_PWRGATE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// XTAL OSC (LP) Control Register
pub mod LOWPWR_CTRL_SET {
    pub use super::LOWPWR_CTRL::CPU_PWRGATE;
    pub use super::LOWPWR_CTRL::DISPLAY_PWRGATE;
    pub use super::LOWPWR_CTRL::GPU_PWRGATE;
    pub use super::LOWPWR_CTRL::L1_PWRGATE;
    pub use super::LOWPWR_CTRL::L2_PWRGATE;
    pub use super::LOWPWR_CTRL::LPBG_SEL;
    pub use super::LOWPWR_CTRL::LPBG_TEST;
    pub use super::LOWPWR_CTRL::MIX_PWRGATE;
    pub use super::LOWPWR_CTRL::OSC_SEL;
    pub use super::LOWPWR_CTRL::RCOSC_CG_OVERRIDE;
    pub use super::LOWPWR_CTRL::RC_OSC_EN;
    pub use super::LOWPWR_CTRL::REFTOP_IBIAS_OFF;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_DELAY;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_STAT;
}

/// XTAL OSC (LP) Control Register
pub mod LOWPWR_CTRL_CLR {
    pub use super::LOWPWR_CTRL::CPU_PWRGATE;
    pub use super::LOWPWR_CTRL::DISPLAY_PWRGATE;
    pub use super::LOWPWR_CTRL::GPU_PWRGATE;
    pub use super::LOWPWR_CTRL::L1_PWRGATE;
    pub use super::LOWPWR_CTRL::L2_PWRGATE;
    pub use super::LOWPWR_CTRL::LPBG_SEL;
    pub use super::LOWPWR_CTRL::LPBG_TEST;
    pub use super::LOWPWR_CTRL::MIX_PWRGATE;
    pub use super::LOWPWR_CTRL::OSC_SEL;
    pub use super::LOWPWR_CTRL::RCOSC_CG_OVERRIDE;
    pub use super::LOWPWR_CTRL::RC_OSC_EN;
    pub use super::LOWPWR_CTRL::REFTOP_IBIAS_OFF;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_DELAY;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_STAT;
}

/// XTAL OSC (LP) Control Register
pub mod LOWPWR_CTRL_TOG {
    pub use super::LOWPWR_CTRL::CPU_PWRGATE;
    pub use super::LOWPWR_CTRL::DISPLAY_PWRGATE;
    pub use super::LOWPWR_CTRL::GPU_PWRGATE;
    pub use super::LOWPWR_CTRL::L1_PWRGATE;
    pub use super::LOWPWR_CTRL::L2_PWRGATE;
    pub use super::LOWPWR_CTRL::LPBG_SEL;
    pub use super::LOWPWR_CTRL::LPBG_TEST;
    pub use super::LOWPWR_CTRL::MIX_PWRGATE;
    pub use super::LOWPWR_CTRL::OSC_SEL;
    pub use super::LOWPWR_CTRL::RCOSC_CG_OVERRIDE;
    pub use super::LOWPWR_CTRL::RC_OSC_EN;
    pub use super::LOWPWR_CTRL::REFTOP_IBIAS_OFF;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_DELAY;
    pub use super::LOWPWR_CTRL::XTALOSC_PWRUP_STAT;
}

/// XTAL OSC Configuration 0 Register
pub mod OSC_CONFIG0 {

    /// Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset.
    pub mod START {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the tuning logic to calculate new RC tuning values
    pub mod ENABLE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bypasses any calculated RC tuning value and uses the programmed register value.
    pub mod BYPASS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Invert the stepping of the calculated RC tuning value.
    pub mod INVERT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RC osc. tuning values.
    pub mod RC_OSC_PROG {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Positive hysteresis value
    pub mod HYST_PLUS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Negative hysteresis value
    pub mod HYST_MINUS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The current tuning value in use.
    pub mod RC_OSC_PROG_CUR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// XTAL OSC Configuration 0 Register
pub mod OSC_CONFIG0_SET {
    pub use super::OSC_CONFIG0::BYPASS;
    pub use super::OSC_CONFIG0::ENABLE;
    pub use super::OSC_CONFIG0::HYST_MINUS;
    pub use super::OSC_CONFIG0::HYST_PLUS;
    pub use super::OSC_CONFIG0::INVERT;
    pub use super::OSC_CONFIG0::RC_OSC_PROG;
    pub use super::OSC_CONFIG0::RC_OSC_PROG_CUR;
    pub use super::OSC_CONFIG0::START;
}

/// XTAL OSC Configuration 0 Register
pub mod OSC_CONFIG0_CLR {
    pub use super::OSC_CONFIG0::BYPASS;
    pub use super::OSC_CONFIG0::ENABLE;
    pub use super::OSC_CONFIG0::HYST_MINUS;
    pub use super::OSC_CONFIG0::HYST_PLUS;
    pub use super::OSC_CONFIG0::INVERT;
    pub use super::OSC_CONFIG0::RC_OSC_PROG;
    pub use super::OSC_CONFIG0::RC_OSC_PROG_CUR;
    pub use super::OSC_CONFIG0::START;
}

/// XTAL OSC Configuration 0 Register
pub mod OSC_CONFIG0_TOG {
    pub use super::OSC_CONFIG0::BYPASS;
    pub use super::OSC_CONFIG0::ENABLE;
    pub use super::OSC_CONFIG0::HYST_MINUS;
    pub use super::OSC_CONFIG0::HYST_PLUS;
    pub use super::OSC_CONFIG0::INVERT;
    pub use super::OSC_CONFIG0::RC_OSC_PROG;
    pub use super::OSC_CONFIG0::RC_OSC_PROG_CUR;
    pub use super::OSC_CONFIG0::START;
}

/// XTAL OSC Configuration 1 Register
pub mod OSC_CONFIG1 {

    /// The target count used to tune the RC OSC frequency
    pub mod COUNT_RC_TRG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The current tuning value in use.
    pub mod COUNT_RC_CUR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// XTAL OSC Configuration 1 Register
pub mod OSC_CONFIG1_SET {
    pub use super::OSC_CONFIG1::COUNT_RC_CUR;
    pub use super::OSC_CONFIG1::COUNT_RC_TRG;
}

/// XTAL OSC Configuration 1 Register
pub mod OSC_CONFIG1_CLR {
    pub use super::OSC_CONFIG1::COUNT_RC_CUR;
    pub use super::OSC_CONFIG1::COUNT_RC_TRG;
}

/// XTAL OSC Configuration 1 Register
pub mod OSC_CONFIG1_TOG {
    pub use super::OSC_CONFIG1::COUNT_RC_CUR;
    pub use super::OSC_CONFIG1::COUNT_RC_TRG;
}

/// XTAL OSC Configuration 2 Register
pub mod OSC_CONFIG2 {

    /// The target count used to tune the 1MHz clock frequency
    pub mod COUNT_1M_TRG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the 1MHz clock output. 0 - disabled; 1 - enabled.
    pub mod ENABLE_1M {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mux the corrected or uncorrected 1MHz clock to the output
    pub mod MUX_1M {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Flag indicates that the count_1m count wasn't reached within 1 32kHz period
    pub mod CLK_1M_ERR_FL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// XTAL OSC Configuration 2 Register
pub mod OSC_CONFIG2_SET {
    pub use super::OSC_CONFIG2::CLK_1M_ERR_FL;
    pub use super::OSC_CONFIG2::COUNT_1M_TRG;
    pub use super::OSC_CONFIG2::ENABLE_1M;
    pub use super::OSC_CONFIG2::MUX_1M;
}

/// XTAL OSC Configuration 2 Register
pub mod OSC_CONFIG2_CLR {
    pub use super::OSC_CONFIG2::CLK_1M_ERR_FL;
    pub use super::OSC_CONFIG2::COUNT_1M_TRG;
    pub use super::OSC_CONFIG2::ENABLE_1M;
    pub use super::OSC_CONFIG2::MUX_1M;
}

/// XTAL OSC Configuration 2 Register
pub mod OSC_CONFIG2_TOG {
    pub use super::OSC_CONFIG2::CLK_1M_ERR_FL;
    pub use super::OSC_CONFIG2::COUNT_1M_TRG;
    pub use super::OSC_CONFIG2::ENABLE_1M;
    pub use super::OSC_CONFIG2::MUX_1M;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 84],

    /// Miscellaneous Register 0
    pub MISC0: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_SET: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_CLR: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_TOG: RWRegister<u32>,

    _reserved2: [u32; 68],

    /// XTAL OSC (LP) Control Register
    pub LOWPWR_CTRL: RWRegister<u32>,

    /// XTAL OSC (LP) Control Register
    pub LOWPWR_CTRL_SET: RWRegister<u32>,

    /// XTAL OSC (LP) Control Register
    pub LOWPWR_CTRL_CLR: RWRegister<u32>,

    /// XTAL OSC (LP) Control Register
    pub LOWPWR_CTRL_TOG: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// XTAL OSC Configuration 0 Register
    pub OSC_CONFIG0: RWRegister<u32>,

    /// XTAL OSC Configuration 0 Register
    pub OSC_CONFIG0_SET: RWRegister<u32>,

    /// XTAL OSC Configuration 0 Register
    pub OSC_CONFIG0_CLR: RWRegister<u32>,

    /// XTAL OSC Configuration 0 Register
    pub OSC_CONFIG0_TOG: RWRegister<u32>,

    /// XTAL OSC Configuration 1 Register
    pub OSC_CONFIG1: RWRegister<u32>,

    /// XTAL OSC Configuration 1 Register
    pub OSC_CONFIG1_SET: RWRegister<u32>,

    /// XTAL OSC Configuration 1 Register
    pub OSC_CONFIG1_CLR: RWRegister<u32>,

    /// XTAL OSC Configuration 1 Register
    pub OSC_CONFIG1_TOG: RWRegister<u32>,

    /// XTAL OSC Configuration 2 Register
    pub OSC_CONFIG2: RWRegister<u32>,

    /// XTAL OSC Configuration 2 Register
    pub OSC_CONFIG2_SET: RWRegister<u32>,

    /// XTAL OSC Configuration 2 Register
    pub OSC_CONFIG2_CLR: RWRegister<u32>,

    /// XTAL OSC Configuration 2 Register
    pub OSC_CONFIG2_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub MISC0: u32,
    pub MISC0_SET: u32,
    pub MISC0_CLR: u32,
    pub MISC0_TOG: u32,
    pub LOWPWR_CTRL: u32,
    pub LOWPWR_CTRL_SET: u32,
    pub LOWPWR_CTRL_CLR: u32,
    pub LOWPWR_CTRL_TOG: u32,
    pub OSC_CONFIG0: u32,
    pub OSC_CONFIG0_SET: u32,
    pub OSC_CONFIG0_CLR: u32,
    pub OSC_CONFIG0_TOG: u32,
    pub OSC_CONFIG1: u32,
    pub OSC_CONFIG1_SET: u32,
    pub OSC_CONFIG1_CLR: u32,
    pub OSC_CONFIG1_TOG: u32,
    pub OSC_CONFIG2: u32,
    pub OSC_CONFIG2_SET: u32,
    pub OSC_CONFIG2_CLR: u32,
    pub OSC_CONFIG2_TOG: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}
