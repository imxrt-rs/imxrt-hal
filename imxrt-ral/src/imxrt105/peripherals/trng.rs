#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TRNG
//!
//! Used by: imxrt1051, imxrt1052

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Miscellaneous Control Register
pub mod MCTL {

    /// Sample Mode
    pub mod SAMP_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: use Von Neumann data into both Entropy shifter and Statistical Checker
            pub const SAMP_MODE_0: u32 = 0b00;

            /// 0b01: use raw data into both Entropy shifter and Statistical Checker
            pub const SAMP_MODE_1: u32 = 0b01;

            /// 0b10: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker
            pub const SAMP_MODE_2: u32 = 0b10;

            /// 0b11: undefined/reserved.
            pub const SAMP_MODE_3: u32 = 0b11;
        }
    }

    /// Oscillator Divide
    pub mod OSC_DIV {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: use ring oscillator with no divide
            pub const OSC_DIV_0: u32 = 0b00;

            /// 0b01: use ring oscillator divided-by-2
            pub const OSC_DIV_1: u32 = 0b01;

            /// 0b10: use ring oscillator divided-by-4
            pub const OSC_DIV_2: u32 = 0b10;

            /// 0b11: use ring oscillator divided-by-8
            pub const OSC_DIV_3: u32 = 0b11;
        }
    }

    /// This bit is unused. Always reads zero.
    pub mod UNUSED4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit is unused. Always reads zero.
    pub mod UNUSED5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset Defaults
    pub mod RST_DEF {
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

    /// Force System Clock
    pub mod FOR_SCLK {
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

    /// Read only: Frequency Count Fail
    pub mod FCT_FAIL {
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

    /// Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT.
    pub mod FCT_VAL {
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

    /// Read only: Entropy Valid
    pub mod ENT_VAL {
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

    /// Read only: Test point inside ring oscillator.
    pub mod TST_OUT {
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

    /// Read: Error status
    pub mod ERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRNG_OK_TO_STOP
    pub mod TSTOP_OK {
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

    /// Long run count continues between entropy generations
    pub mod LRUN_CONT {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming Mode Select
    pub mod PRGM {
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
}

/// Statistical Check Miscellaneous Register
pub mod SCMISC {

    /// LONG RUN MAX LIMIT
    pub mod LRUN_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RETRY COUNT
    pub mod RTY_CT {
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
}

/// Poker Range Register
pub mod PKRRNG {

    /// Poker Range
    pub mod PKR_RNG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PKRMAX and PKRSQ
/// PKRMAX: Poker Maximum Limit Register
/// PKRSQ: Poker Square Calculation Result Register
pub mod PKR {

    /// Poker Maximum Limit.
    pub mod PKR_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker Square Calculation Result.
    pub mod PKR_SQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Seed Control Register
pub mod SDCTL {

    /// Sample Size
    pub mod SAMP_SIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Entropy Delay
    pub mod ENT_DLY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SBLIM and TOTSAM
/// SBLIM: Sparse Bit Limit Register
/// TOTSAM: Total Samples Register
pub mod SBLIM {

    /// Sparse Bit Limit
    pub mod SB_LIM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Total Samples
    pub mod TOT_SAM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (20 bits: 0xfffff << 0)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Frequency Count Minimum Limit Register
pub mod FRQMIN {

    /// Frequency Count Minimum Limit
    pub mod FRQ_MIN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FRQCNT and FRQMAX
/// FRQCNT: Frequency Count Register
/// FRQMAX: Frequency Count Maximum Limit Register
pub mod FRQ {

    /// Frequency Count
    pub mod FRQ_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frequency Counter Maximum Limit
    pub mod FRQ_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCMC and SCML
/// SCMC: Statistical Check Monobit Count Register
/// SCML: Statistical Check Monobit Limit Register
pub mod SCM {

    /// Monobit Count
    pub mod MONO_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Monobit Maximum Limit
    pub mod MONO_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Monobit Range
    pub mod MONO_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR1C and SCR1L
/// SCR1C: Statistical Check Run Length 1 Count Register
/// SCR1L: Statistical Check Run Length 1 Limit Register
pub mod SCR1 {

    /// Runs of Zero, Length 1 Count
    pub mod R1_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of One, Length 1 Count
    pub mod R1_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 1 Maximum Limit
    pub mod RUN1_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 1 Range
    pub mod RUN1_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR2C and SCR2L
/// SCR2C: Statistical Check Run Length 2 Count Register
/// SCR2L: Statistical Check Run Length 2 Limit Register
pub mod SCR2 {

    /// Runs of Zero, Length 2 Count
    pub mod R2_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of One, Length 2 Count
    pub mod R2_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 2 Maximum Limit
    pub mod RUN2_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 2 Range
    pub mod RUN2_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR3C and SCR3L
/// SCR3C: Statistical Check Run Length 3 Count Register
/// SCR3L: Statistical Check Run Length 3 Limit Register
pub mod SCR3 {

    /// Runs of Zeroes, Length 3 Count
    pub mod R3_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of Ones, Length 3 Count
    pub mod R3_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 3 Maximum Limit
    pub mod RUN3_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 3 Range
    pub mod RUN3_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (13 bits: 0x1fff << 16)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR4C and SCR4L
/// SCR4C: Statistical Check Run Length 4 Count Register
/// SCR4L: Statistical Check Run Length 4 Limit Register
pub mod SCR4 {

    /// Runs of Zero, Length 4 Count
    pub mod R4_0_CT {
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

    /// Runs of One, Length 4 Count
    pub mod R4_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 4 Maximum Limit
    pub mod RUN4_MAX {
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

    /// Run Length 4 Range
    pub mod RUN4_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR5C and SCR5L
/// SCR5C: Statistical Check Run Length 5 Count Register
/// SCR5L: Statistical Check Run Length 5 Limit Register
pub mod SCR5 {

    /// Runs of Zero, Length 5 Count
    pub mod R5_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of One, Length 5 Count
    pub mod R5_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 5 Maximum Limit
    pub mod RUN5_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 5 Range
    pub mod RUN5_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SCR6PC and SCR6PL
/// SCR6PC: Statistical Check Run Length 6+ Count Register
/// SCR6PL: Statistical Check Run Length 6+ Limit Register
pub mod SCR6P {

    /// Runs of Zero, Length 6+ Count
    pub mod R6P_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Runs of One, Length 6+ Count
    pub mod R6P_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 6+ Maximum Limit
    pub mod RUN6P_MAX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run Length 6+ Range
    pub mod RUN6P_RNG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status Register
pub mod STATUS {

    /// Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed.
    pub mod TF1BR0 {
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

    /// Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed.
    pub mod TF1BR1 {
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

    /// Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed.
    pub mod TF2BR0 {
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

    /// Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed.
    pub mod TF2BR1 {
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

    /// Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed.
    pub mod TF3BR0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed.
    pub mod TF3BR1 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed.
    pub mod TF4BR0 {
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

    /// Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed.
    pub mod TF4BR1 {
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

    /// Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed.
    pub mod TF5BR0 {
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

    /// Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed.
    pub mod TF5BR1 {
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

    /// Test Fail, 6 Plus Bit Run, Sampling 0s
    pub mod TF6PBR0 {
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

    /// Test Fail, 6 Plus Bit Run, Sampling 1s
    pub mod TF6PBR1 {
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

    /// Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed.
    pub mod TFSB {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test Fail, Long Run. If TFLR=1, the Long Run Test has failed.
    pub mod TFLR {
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

    /// Test Fail, Poker. If TFP=1, the Poker Test has failed.
    pub mod TFP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed.
    pub mod TFMB {
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

    /// RETRY COUNT
    pub mod RETRY_CT {
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
}

/// Entropy Read Register
pub mod ENT0 {

    /// Entropy Value
    pub mod ENT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Entropy Read Register
pub mod ENT1 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT2 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT3 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT4 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT5 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT6 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT7 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT8 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT9 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT10 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT11 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT12 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT13 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT14 {
    pub use super::ENT0::ENT;
}

/// Entropy Read Register
pub mod ENT15 {
    pub use super::ENT0::ENT;
}

/// Statistical Check Poker Count 1 and 0 Register
pub mod PKRCNT10 {

    /// Poker 0h Count
    pub mod PKR_0_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker 1h Count
    pub mod PKR_1_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count 3 and 2 Register
pub mod PKRCNT32 {

    /// Poker 2h Count
    pub mod PKR_2_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker 3h Count
    pub mod PKR_3_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count 5 and 4 Register
pub mod PKRCNT54 {

    /// Poker 4h Count
    pub mod PKR_4_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker 5h Count
    pub mod PKR_5_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count 7 and 6 Register
pub mod PKRCNT76 {

    /// Poker 6h Count
    pub mod PKR_6_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker 7h Count
    pub mod PKR_7_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count 9 and 8 Register
pub mod PKRCNT98 {

    /// Poker 8h Count
    pub mod PKR_8_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker 9h Count
    pub mod PKR_9_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count B and A Register
pub mod PKRCNTBA {

    /// Poker Ah Count
    pub mod PKR_A_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker Bh Count
    pub mod PKR_B_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count D and C Register
pub mod PKRCNTDC {

    /// Poker Ch Count
    pub mod PKR_C_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker Dh Count
    pub mod PKR_D_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Statistical Check Poker Count F and E Register
pub mod PKRCNTFE {

    /// Poker Eh Count
    pub mod PKR_E_CT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Poker Fh Count
    pub mod PKR_F_CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Security Configuration Register
pub mod SEC_CFG {

    /// This bit is unused. Ignore.
    pub mod UNUSED0 {
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

    /// If set, the TRNG registers cannot be programmed
    pub mod NO_PRGM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Programability of registers controlled only by the Miscellaneous Control Register's access mode bit.
            pub const NO_PRGM_0: u32 = 0b0;

            /// 0b1: Overides Miscellaneous Control Register access mode and prevents TRNG register programming.
            pub const NO_PRGM_1: u32 = 0b1;
        }
    }

    /// This bit is unused. Ignore.
    pub mod UNUSED2 {
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
}

/// Interrupt Control Register
pub mod INT_CTRL {

    /// Bit position that can be cleared if corresponding bit of INT_STATUS register has been asserted.
    pub mod HW_ERR {
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

            /// 0b0: Corresponding bit of INT_STATUS register cleared.
            pub const HW_ERR_0: u32 = 0b0;

            /// 0b1: Corresponding bit of INT_STATUS register active.
            pub const HW_ERR_1: u32 = 0b1;
        }
    }

    /// Same behavior as bit 0 of this register.
    pub mod ENT_VAL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as bit 0 of this register.
            pub const ENT_VAL_0: u32 = 0b0;

            /// 0b1: Same behavior as bit 0 of this register.
            pub const ENT_VAL_1: u32 = 0b1;
        }
    }

    /// Same behavior as bit 0 of this register.
    pub mod FRQ_CT_FAIL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as bit 0 of this register.
            pub const FRQ_CT_FAIL_0: u32 = 0b0;

            /// 0b1: Same behavior as bit 0 of this register.
            pub const FRQ_CT_FAIL_1: u32 = 0b1;
        }
    }
}

/// Mask Register
pub mod INT_MASK {

    /// Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted.
    pub mod HW_ERR {
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

            /// 0b0: Corresponding interrupt of INT_STATUS is masked.
            pub const HW_ERR_0: u32 = 0b0;

            /// 0b1: Corresponding bit of INT_STATUS is active.
            pub const HW_ERR_1: u32 = 0b1;
        }
    }

    /// Same behavior as bit 0 of this register.
    pub mod ENT_VAL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as bit 0 of this register.
            pub const ENT_VAL_0: u32 = 0b0;

            /// 0b1: Same behavior as bit 0 of this register.
            pub const ENT_VAL_1: u32 = 0b1;
        }
    }

    /// Same behavior as bit 0 of this register.
    pub mod FRQ_CT_FAIL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as bit 0 of this register.
            pub const FRQ_CT_FAIL_0: u32 = 0b0;

            /// 0b1: Same behavior as bit 0 of this register.
            pub const FRQ_CT_FAIL_1: u32 = 0b1;
        }
    }
}

/// Interrupt Status Register
pub mod INT_STATUS {

    /// Read: Error status
    pub mod HW_ERR {
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

            /// 0b0: no error
            pub const HW_ERR_0: u32 = 0b0;

            /// 0b1: error detected.
            pub const HW_ERR_1: u32 = 0b1;
        }
    }

    /// Read only: Entropy Valid
    pub mod ENT_VAL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Busy generation entropy. Any value read is invalid.
            pub const ENT_VAL_0: u32 = 0b0;

            /// 0b1: TRNG can be stopped and entropy is valid if read.
            pub const ENT_VAL_1: u32 = 0b1;
        }
    }

    /// Read only: Frequency Count Fail
    pub mod FRQ_CT_FAIL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No hardware nor self test frequency errors.
            pub const FRQ_CT_FAIL_0: u32 = 0b0;

            /// 0b1: The frequency counter has detected a failure.
            pub const FRQ_CT_FAIL_1: u32 = 0b1;
        }
    }
}

/// Version ID Register (MS)
pub mod VID1 {

    /// Shows the IP's Minor revision of the TRNG.
    pub mod MIN_REV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Minor revision number for TRNG.
            pub const MIN_REV_0: u32 = 0b00000000;
        }
    }

    /// Shows the IP's Major revision of the TRNG.
    pub mod MAJ_REV {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: Major revision number for TRNG.
            pub const MAJ_REV_1: u32 = 0b00000001;
        }
    }

    /// Shows the IP ID.
    pub mod IP_ID {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000110000: ID for TRNG.
            pub const IP_ID_48: u32 = 0b0000000000110000;
        }
    }
}

/// Version ID Register (LS)
pub mod VID2 {

    /// Shows the IP's Configuaration options for the TRNG.
    pub mod CONFIG_OPT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: TRNG_CONFIG_OPT for TRNG.
            pub const CONFIG_OPT_0: u32 = 0b00000000;
        }
    }

    /// Shows the IP's ECO revision of the TRNG.
    pub mod ECO_REV {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: TRNG_ECO_REV for TRNG.
            pub const ECO_REV_0: u32 = 0b00000000;
        }
    }

    /// Shows the integration options for the TRNG.
    pub mod INTG_OPT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: INTG_OPT for TRNG.
            pub const INTG_OPT_0: u32 = 0b00000000;
        }
    }

    /// Shows the compile options for the TRNG.
    pub mod ERA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: COMPILE_OPT for TRNG.
            pub const ERA_0: u32 = 0b00000000;
        }
    }
}
pub struct RegisterBlock {
    /// Miscellaneous Control Register
    pub MCTL: RWRegister<u32>,

    /// Statistical Check Miscellaneous Register
    pub SCMISC: RWRegister<u32>,

    /// Poker Range Register
    pub PKRRNG: RWRegister<u32>,

    /// PKRMAX and PKRSQ
    /// PKRMAX: Poker Maximum Limit Register
    /// PKRSQ: Poker Square Calculation Result Register
    pub PKR: RWRegister<u32>,

    /// Seed Control Register
    pub SDCTL: RWRegister<u32>,

    /// SBLIM and TOTSAM
    /// SBLIM: Sparse Bit Limit Register
    /// TOTSAM: Total Samples Register
    pub SBLIM: RWRegister<u32>,

    /// Frequency Count Minimum Limit Register
    pub FRQMIN: RWRegister<u32>,

    /// FRQCNT and FRQMAX
    /// FRQCNT: Frequency Count Register
    /// FRQMAX: Frequency Count Maximum Limit Register
    pub FRQ: RWRegister<u32>,

    /// SCMC and SCML
    /// SCMC: Statistical Check Monobit Count Register
    /// SCML: Statistical Check Monobit Limit Register
    pub SCM: RWRegister<u32>,

    /// SCR1C and SCR1L
    /// SCR1C: Statistical Check Run Length 1 Count Register
    /// SCR1L: Statistical Check Run Length 1 Limit Register
    pub SCR1: RWRegister<u32>,

    /// SCR2C and SCR2L
    /// SCR2C: Statistical Check Run Length 2 Count Register
    /// SCR2L: Statistical Check Run Length 2 Limit Register
    pub SCR2: RWRegister<u32>,

    /// SCR3C and SCR3L
    /// SCR3C: Statistical Check Run Length 3 Count Register
    /// SCR3L: Statistical Check Run Length 3 Limit Register
    pub SCR3: RWRegister<u32>,

    /// SCR4C and SCR4L
    /// SCR4C: Statistical Check Run Length 4 Count Register
    /// SCR4L: Statistical Check Run Length 4 Limit Register
    pub SCR4: RWRegister<u32>,

    /// SCR5C and SCR5L
    /// SCR5C: Statistical Check Run Length 5 Count Register
    /// SCR5L: Statistical Check Run Length 5 Limit Register
    pub SCR5: RWRegister<u32>,

    /// SCR6PC and SCR6PL
    /// SCR6PC: Statistical Check Run Length 6+ Count Register
    /// SCR6PL: Statistical Check Run Length 6+ Limit Register
    pub SCR6P: RWRegister<u32>,

    /// Status Register
    pub STATUS: RORegister<u32>,

    /// Entropy Read Register
    pub ENT0: RORegister<u32>,

    /// Entropy Read Register
    pub ENT1: RORegister<u32>,

    /// Entropy Read Register
    pub ENT2: RORegister<u32>,

    /// Entropy Read Register
    pub ENT3: RORegister<u32>,

    /// Entropy Read Register
    pub ENT4: RORegister<u32>,

    /// Entropy Read Register
    pub ENT5: RORegister<u32>,

    /// Entropy Read Register
    pub ENT6: RORegister<u32>,

    /// Entropy Read Register
    pub ENT7: RORegister<u32>,

    /// Entropy Read Register
    pub ENT8: RORegister<u32>,

    /// Entropy Read Register
    pub ENT9: RORegister<u32>,

    /// Entropy Read Register
    pub ENT10: RORegister<u32>,

    /// Entropy Read Register
    pub ENT11: RORegister<u32>,

    /// Entropy Read Register
    pub ENT12: RORegister<u32>,

    /// Entropy Read Register
    pub ENT13: RORegister<u32>,

    /// Entropy Read Register
    pub ENT14: RORegister<u32>,

    /// Entropy Read Register
    pub ENT15: RORegister<u32>,

    /// Statistical Check Poker Count 1 and 0 Register
    pub PKRCNT10: RORegister<u32>,

    /// Statistical Check Poker Count 3 and 2 Register
    pub PKRCNT32: RORegister<u32>,

    /// Statistical Check Poker Count 5 and 4 Register
    pub PKRCNT54: RORegister<u32>,

    /// Statistical Check Poker Count 7 and 6 Register
    pub PKRCNT76: RORegister<u32>,

    /// Statistical Check Poker Count 9 and 8 Register
    pub PKRCNT98: RORegister<u32>,

    /// Statistical Check Poker Count B and A Register
    pub PKRCNTBA: RORegister<u32>,

    /// Statistical Check Poker Count D and C Register
    pub PKRCNTDC: RORegister<u32>,

    /// Statistical Check Poker Count F and E Register
    pub PKRCNTFE: RORegister<u32>,

    /// Security Configuration Register
    pub SEC_CFG: RWRegister<u32>,

    /// Interrupt Control Register
    pub INT_CTRL: RWRegister<u32>,

    /// Mask Register
    pub INT_MASK: RWRegister<u32>,

    /// Interrupt Status Register
    pub INT_STATUS: RORegister<u32>,

    _reserved1: [u32; 16],

    /// Version ID Register (MS)
    pub VID1: RORegister<u32>,

    /// Version ID Register (LS)
    pub VID2: RORegister<u32>,
}
pub struct ResetValues {
    pub MCTL: u32,
    pub SCMISC: u32,
    pub PKRRNG: u32,
    pub PKR: u32,
    pub SDCTL: u32,
    pub SBLIM: u32,
    pub FRQMIN: u32,
    pub FRQ: u32,
    pub SCM: u32,
    pub SCR1: u32,
    pub SCR2: u32,
    pub SCR3: u32,
    pub SCR4: u32,
    pub SCR5: u32,
    pub SCR6P: u32,
    pub STATUS: u32,
    pub ENT0: u32,
    pub ENT1: u32,
    pub ENT2: u32,
    pub ENT3: u32,
    pub ENT4: u32,
    pub ENT5: u32,
    pub ENT6: u32,
    pub ENT7: u32,
    pub ENT8: u32,
    pub ENT9: u32,
    pub ENT10: u32,
    pub ENT11: u32,
    pub ENT12: u32,
    pub ENT13: u32,
    pub ENT14: u32,
    pub ENT15: u32,
    pub PKRCNT10: u32,
    pub PKRCNT32: u32,
    pub PKRCNT54: u32,
    pub PKRCNT76: u32,
    pub PKRCNT98: u32,
    pub PKRCNTBA: u32,
    pub PKRCNTDC: u32,
    pub PKRCNTFE: u32,
    pub SEC_CFG: u32,
    pub INT_CTRL: u32,
    pub INT_MASK: u32,
    pub INT_STATUS: u32,
    pub VID1: u32,
    pub VID2: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
