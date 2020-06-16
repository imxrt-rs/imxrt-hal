#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Quad Timer

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Timer Channel Compare Register 1
pub mod COMP10 {

    /// Comparison Value 1
    pub mod COMPARISON_1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Compare Register 1
pub mod COMP11 {
    pub use super::COMP10::COMPARISON_1;
}

/// Timer Channel Compare Register 1
pub mod COMP12 {
    pub use super::COMP10::COMPARISON_1;
}

/// Timer Channel Compare Register 1
pub mod COMP13 {
    pub use super::COMP10::COMPARISON_1;
}

/// Timer Channel Compare Register 2
pub mod COMP20 {

    /// Comparison Value 2
    pub mod COMPARISON_2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Compare Register 2
pub mod COMP21 {
    pub use super::COMP20::COMPARISON_2;
}

/// Timer Channel Compare Register 2
pub mod COMP22 {
    pub use super::COMP20::COMPARISON_2;
}

/// Timer Channel Compare Register 2
pub mod COMP23 {
    pub use super::COMP20::COMPARISON_2;
}

/// Timer Channel Capture Register
pub mod CAPT0 {

    /// Capture Value
    pub mod CAPTURE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Capture Register
pub mod CAPT1 {
    pub use super::CAPT0::CAPTURE;
}

/// Timer Channel Capture Register
pub mod CAPT2 {
    pub use super::CAPT0::CAPTURE;
}

/// Timer Channel Capture Register
pub mod CAPT3 {
    pub use super::CAPT0::CAPTURE;
}

/// Timer Channel Load Register
pub mod LOAD0 {

    /// Timer Load Register
    pub mod LOAD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Load Register
pub mod LOAD1 {
    pub use super::LOAD0::LOAD;
}

/// Timer Channel Load Register
pub mod LOAD2 {
    pub use super::LOAD0::LOAD;
}

/// Timer Channel Load Register
pub mod LOAD3 {
    pub use super::LOAD0::LOAD;
}

/// Timer Channel Hold Register
pub mod HOLD0 {

    /// This read/write register stores the counter's values of specific channels whenever any of the four counters within a module is read
    pub mod HOLD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Hold Register
pub mod HOLD1 {
    pub use super::HOLD0::HOLD;
}

/// Timer Channel Hold Register
pub mod HOLD2 {
    pub use super::HOLD0::HOLD;
}

/// Timer Channel Hold Register
pub mod HOLD3 {
    pub use super::HOLD0::HOLD;
}

/// Timer Channel Counter Register
pub mod CNTR0 {

    /// This read/write register is the counter for the corresponding channel in a timer module.
    pub mod COUNTER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Counter Register
pub mod CNTR1 {
    pub use super::CNTR0::COUNTER;
}

/// Timer Channel Counter Register
pub mod CNTR2 {
    pub use super::CNTR0::COUNTER;
}

/// Timer Channel Counter Register
pub mod CNTR3 {
    pub use super::CNTR0::COUNTER;
}

/// Timer Channel Control Register
pub mod CTRL0 {

    /// Output Mode
    pub mod OUTMODE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Asserted while counter is active
            pub const OUTMODE_0: u16 = 0b000;

            /// 0b001: Clear OFLAG output on successful compare
            pub const OUTMODE_1: u16 = 0b001;

            /// 0b010: Set OFLAG output on successful compare
            pub const OUTMODE_2: u16 = 0b010;

            /// 0b011: Toggle OFLAG output on successful compare
            pub const OUTMODE_3: u16 = 0b011;

            /// 0b100: Toggle OFLAG output using alternating compare registers
            pub const OUTMODE_4: u16 = 0b100;

            /// 0b101: Set on compare, cleared on secondary source input edge
            pub const OUTMODE_5: u16 = 0b101;

            /// 0b110: Set on compare, cleared on counter rollover
            pub const OUTMODE_6: u16 = 0b110;

            /// 0b111: Enable gated clock output while counter is active
            pub const OUTMODE_7: u16 = 0b111;
        }
    }

    /// Co-Channel Initialization
    pub mod COINIT {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Co-channel counter/timers cannot force a re-initialization of this counter/timer
            pub const COINIT_0: u16 = 0b0;

            /// 0b1: Co-channel counter/timers may force a re-initialization of this counter/timer
            pub const COINIT_1: u16 = 0b1;
        }
    }

    /// Count Direction
    pub mod DIR {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Count up.
            pub const DIR_0: u16 = 0b0;

            /// 0b1: Count down.
            pub const DIR_1: u16 = 0b1;
        }
    }

    /// Count Length
    pub mod LENGTH {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Count until roll over at $FFFF and continue from $0000.
            pub const LENGTH_0: u16 = 0b0;

            /// 0b1: Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on.
            pub const LENGTH_1: u16 = 0b1;
        }
    }

    /// Count Once
    pub mod ONCE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Count repeatedly.
            pub const ONCE_0: u16 = 0b0;

            /// 0b1: Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops.
            pub const ONCE_1: u16 = 0b1;
        }
    }

    /// Secondary Count Source
    pub mod SCS {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Counter 0 input pin
            pub const SCS_0: u16 = 0b00;

            /// 0b01: Counter 1 input pin
            pub const SCS_1: u16 = 0b01;

            /// 0b10: Counter 2 input pin
            pub const SCS_2: u16 = 0b10;

            /// 0b11: Counter 3 input pin
            pub const SCS_3: u16 = 0b11;
        }
    }

    /// Primary Count Source
    pub mod PCS {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (4 bits: 0b1111 << 9)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Counter 0 input pin
            pub const PCS_0: u16 = 0b0000;

            /// 0b0001: Counter 1 input pin
            pub const PCS_1: u16 = 0b0001;

            /// 0b0010: Counter 2 input pin
            pub const PCS_2: u16 = 0b0010;

            /// 0b0011: Counter 3 input pin
            pub const PCS_3: u16 = 0b0011;

            /// 0b0100: Counter 0 output
            pub const PCS_4: u16 = 0b0100;

            /// 0b0101: Counter 1 output
            pub const PCS_5: u16 = 0b0101;

            /// 0b0110: Counter 2 output
            pub const PCS_6: u16 = 0b0110;

            /// 0b0111: Counter 3 output
            pub const PCS_7: u16 = 0b0111;

            /// 0b1000: IP bus clock divide by 1 prescaler
            pub const PCS_8: u16 = 0b1000;

            /// 0b1001: IP bus clock divide by 2 prescaler
            pub const PCS_9: u16 = 0b1001;

            /// 0b1010: IP bus clock divide by 4 prescaler
            pub const PCS_10: u16 = 0b1010;

            /// 0b1011: IP bus clock divide by 8 prescaler
            pub const PCS_11: u16 = 0b1011;

            /// 0b1100: IP bus clock divide by 16 prescaler
            pub const PCS_12: u16 = 0b1100;

            /// 0b1101: IP bus clock divide by 32 prescaler
            pub const PCS_13: u16 = 0b1101;

            /// 0b1110: IP bus clock divide by 64 prescaler
            pub const PCS_14: u16 = 0b1110;

            /// 0b1111: IP bus clock divide by 128 prescaler
            pub const PCS_15: u16 = 0b1111;
        }
    }

    /// Count Mode
    pub mod CM {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No operation
            pub const CM_0: u16 = 0b000;

            /// 0b001: Count rising edges of primary sourceRising edges are counted only when SCTRL\[IPS\] = 0. Falling edges are counted when SCTRL\[IPS\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\[IPS\].
            pub const CM_1: u16 = 0b001;

            /// 0b010: Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode.
            pub const CM_2: u16 = 0b010;

            /// 0b011: Count rising edges of primary source while secondary input high active
            pub const CM_3: u16 = 0b011;

            /// 0b100: Quadrature count mode, uses primary and secondary sources
            pub const CM_4: u16 = 0b100;

            /// 0b101: Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\[IPS\] = 0. Falling edges are counted when SCTRL\[IPS\] = 1.
            pub const CM_5: u16 = 0b101;

            /// 0b110: Edge of secondary source triggers primary count until compare
            pub const CM_6: u16 = 0b110;

            /// 0b111: Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs.
            pub const CM_7: u16 = 0b111;
        }
    }
}

/// Timer Channel Control Register
pub mod CTRL1 {
    pub use super::CTRL0::CM;
    pub use super::CTRL0::COINIT;
    pub use super::CTRL0::DIR;
    pub use super::CTRL0::LENGTH;
    pub use super::CTRL0::ONCE;
    pub use super::CTRL0::OUTMODE;
    pub use super::CTRL0::PCS;
    pub use super::CTRL0::SCS;
}

/// Timer Channel Control Register
pub mod CTRL2 {
    pub use super::CTRL0::CM;
    pub use super::CTRL0::COINIT;
    pub use super::CTRL0::DIR;
    pub use super::CTRL0::LENGTH;
    pub use super::CTRL0::ONCE;
    pub use super::CTRL0::OUTMODE;
    pub use super::CTRL0::PCS;
    pub use super::CTRL0::SCS;
}

/// Timer Channel Control Register
pub mod CTRL3 {
    pub use super::CTRL0::CM;
    pub use super::CTRL0::COINIT;
    pub use super::CTRL0::DIR;
    pub use super::CTRL0::LENGTH;
    pub use super::CTRL0::ONCE;
    pub use super::CTRL0::OUTMODE;
    pub use super::CTRL0::PCS;
    pub use super::CTRL0::SCS;
}

/// Timer Channel Status and Control Register
pub mod SCTRL0 {

    /// Output Enable
    pub mod OEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The external pin is configured as an input.
            pub const OEN_0: u16 = 0b0;

            /// 0b1: The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS.
            pub const OEN_1: u16 = 0b1;
        }
    }

    /// Output Polarity Select
    pub mod OPS {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: True polarity.
            pub const OPS_0: u16 = 0b0;

            /// 0b1: Inverted polarity.
            pub const OPS_1: u16 = 0b1;
        }
    }

    /// Force OFLAG Output
    pub mod FORCE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Forced OFLAG Value
    pub mod VAL {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable External OFLAG Force
    pub mod EEOF {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Master Mode
    pub mod MSTR {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Capture Mode
    pub mod CAPTURE_MODE {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Capture function is disabled
            pub const CAPTURE_MODE_0: u16 = 0b00;

            /// 0b01: Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input
            pub const CAPTURE_MODE_1: u16 = 0b01;

            /// 0b10: Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input
            pub const CAPTURE_MODE_2: u16 = 0b10;

            /// 0b11: Load capture register on both edges of input
            pub const CAPTURE_MODE_3: u16 = 0b11;
        }
    }

    /// External Input Signal
    pub mod INPUT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Polarity Select
    pub mod IPS {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Edge Flag Interrupt Enable
    pub mod IEFIE {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Edge Flag
    pub mod IEF {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Overflow Flag Interrupt Enable
    pub mod TOFIE {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Overflow Flag
    pub mod TOF {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Compare Flag Interrupt Enable
    pub mod TCFIE {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Compare Flag
    pub mod TCF {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Status and Control Register
pub mod SCTRL1 {
    pub use super::SCTRL0::CAPTURE_MODE;
    pub use super::SCTRL0::EEOF;
    pub use super::SCTRL0::FORCE;
    pub use super::SCTRL0::IEF;
    pub use super::SCTRL0::IEFIE;
    pub use super::SCTRL0::INPUT;
    pub use super::SCTRL0::IPS;
    pub use super::SCTRL0::MSTR;
    pub use super::SCTRL0::OEN;
    pub use super::SCTRL0::OPS;
    pub use super::SCTRL0::TCF;
    pub use super::SCTRL0::TCFIE;
    pub use super::SCTRL0::TOF;
    pub use super::SCTRL0::TOFIE;
    pub use super::SCTRL0::VAL;
}

/// Timer Channel Status and Control Register
pub mod SCTRL2 {
    pub use super::SCTRL0::CAPTURE_MODE;
    pub use super::SCTRL0::EEOF;
    pub use super::SCTRL0::FORCE;
    pub use super::SCTRL0::IEF;
    pub use super::SCTRL0::IEFIE;
    pub use super::SCTRL0::INPUT;
    pub use super::SCTRL0::IPS;
    pub use super::SCTRL0::MSTR;
    pub use super::SCTRL0::OEN;
    pub use super::SCTRL0::OPS;
    pub use super::SCTRL0::TCF;
    pub use super::SCTRL0::TCFIE;
    pub use super::SCTRL0::TOF;
    pub use super::SCTRL0::TOFIE;
    pub use super::SCTRL0::VAL;
}

/// Timer Channel Status and Control Register
pub mod SCTRL3 {
    pub use super::SCTRL0::CAPTURE_MODE;
    pub use super::SCTRL0::EEOF;
    pub use super::SCTRL0::FORCE;
    pub use super::SCTRL0::IEF;
    pub use super::SCTRL0::IEFIE;
    pub use super::SCTRL0::INPUT;
    pub use super::SCTRL0::IPS;
    pub use super::SCTRL0::MSTR;
    pub use super::SCTRL0::OEN;
    pub use super::SCTRL0::OPS;
    pub use super::SCTRL0::TCF;
    pub use super::SCTRL0::TCFIE;
    pub use super::SCTRL0::TOF;
    pub use super::SCTRL0::TOFIE;
    pub use super::SCTRL0::VAL;
}

/// Timer Channel Comparator Load Register 1
pub mod CMPLD10 {

    /// This read/write register is the comparator 1 preload value for the COMP1 register for the corresponding channel in a timer module
    pub mod COMPARATOR_LOAD_1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Comparator Load Register 1
pub mod CMPLD11 {
    pub use super::CMPLD10::COMPARATOR_LOAD_1;
}

/// Timer Channel Comparator Load Register 1
pub mod CMPLD12 {
    pub use super::CMPLD10::COMPARATOR_LOAD_1;
}

/// Timer Channel Comparator Load Register 1
pub mod CMPLD13 {
    pub use super::CMPLD10::COMPARATOR_LOAD_1;
}

/// Timer Channel Comparator Load Register 2
pub mod CMPLD20 {

    /// This read/write register is the comparator 2 preload value for the COMP2 register for the corresponding channel in a timer module
    pub mod COMPARATOR_LOAD_2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Comparator Load Register 2
pub mod CMPLD21 {
    pub use super::CMPLD20::COMPARATOR_LOAD_2;
}

/// Timer Channel Comparator Load Register 2
pub mod CMPLD22 {
    pub use super::CMPLD20::COMPARATOR_LOAD_2;
}

/// Timer Channel Comparator Load Register 2
pub mod CMPLD23 {
    pub use super::CMPLD20::COMPARATOR_LOAD_2;
}

/// Timer Channel Comparator Status and Control Register
pub mod CSCTRL0 {

    /// Compare Load Control 1
    pub mod CL1 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Never preload
            pub const CL1_0: u16 = 0b00;

            /// 0b01: Load upon successful compare with the value in COMP1
            pub const CL1_1: u16 = 0b01;

            /// 0b10: Load upon successful compare with the value in COMP2
            pub const CL1_2: u16 = 0b10;
        }
    }

    /// Compare Load Control 2
    pub mod CL2 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Never preload
            pub const CL2_0: u16 = 0b00;

            /// 0b01: Load upon successful compare with the value in COMP1
            pub const CL2_1: u16 = 0b01;

            /// 0b10: Load upon successful compare with the value in COMP2
            pub const CL2_2: u16 = 0b10;
        }
    }

    /// Timer Compare 1 Interrupt Flag
    pub mod TCF1 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Compare 2 Interrupt Flag
    pub mod TCF2 {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Compare 1 Interrupt Enable
    pub mod TCF1EN {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer Compare 2 Interrupt Enable
    pub mod TCF2EN {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counting Direction Indicator
    pub mod UP {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The last count was in the DOWN direction.
            pub const UP_0: u16 = 0b0;

            /// 0b1: The last count was in the UP direction.
            pub const UP_1: u16 = 0b1;
        }
    }

    /// Triggered Count Initialization Control
    pub mod TCI {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stop counter upon receiving a second trigger event while still counting from the first trigger event.
            pub const TCI_0: u16 = 0b0;

            /// 0b1: Reload the counter upon receiving a second trigger event while still counting from the first trigger event.
            pub const TCI_1: u16 = 0b1;
        }
    }

    /// Reload on Capture
    pub mod ROC {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not reload the counter on a capture event.
            pub const ROC_0: u16 = 0b0;

            /// 0b1: Reload the counter on a capture event.
            pub const ROC_1: u16 = 0b1;
        }
    }

    /// Alternative Load Enable
    pub mod ALT_LOAD {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counter can be re-initialized only with the LOAD register.
            pub const ALT_LOAD_0: u16 = 0b0;

            /// 0b1: Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction.
            pub const ALT_LOAD_1: u16 = 0b1;
        }
    }

    /// Fault Enable
    pub mod FAULT {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fault function disabled.
            pub const FAULT_0: u16 = 0b0;

            /// 0b1: Fault function enabled.
            pub const FAULT_1: u16 = 0b1;
        }
    }

    /// Debug Actions Enable
    pub mod DBG_EN {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Continue with normal operation during debug mode. (default)
            pub const DBG_EN_0: u16 = 0b00;

            /// 0b01: Halt TMR counter during debug mode.
            pub const DBG_EN_1: u16 = 0b01;

            /// 0b10: Force TMR output to logic 0 (prior to consideration of SCTRL\[OPS\]).
            pub const DBG_EN_2: u16 = 0b10;

            /// 0b11: Both halt counter and force output to 0 during debug mode.
            pub const DBG_EN_3: u16 = 0b11;
        }
    }
}

/// Timer Channel Comparator Status and Control Register
pub mod CSCTRL1 {
    pub use super::CSCTRL0::ALT_LOAD;
    pub use super::CSCTRL0::CL1;
    pub use super::CSCTRL0::CL2;
    pub use super::CSCTRL0::DBG_EN;
    pub use super::CSCTRL0::FAULT;
    pub use super::CSCTRL0::ROC;
    pub use super::CSCTRL0::TCF1;
    pub use super::CSCTRL0::TCF1EN;
    pub use super::CSCTRL0::TCF2;
    pub use super::CSCTRL0::TCF2EN;
    pub use super::CSCTRL0::TCI;
    pub use super::CSCTRL0::UP;
}

/// Timer Channel Comparator Status and Control Register
pub mod CSCTRL2 {
    pub use super::CSCTRL0::ALT_LOAD;
    pub use super::CSCTRL0::CL1;
    pub use super::CSCTRL0::CL2;
    pub use super::CSCTRL0::DBG_EN;
    pub use super::CSCTRL0::FAULT;
    pub use super::CSCTRL0::ROC;
    pub use super::CSCTRL0::TCF1;
    pub use super::CSCTRL0::TCF1EN;
    pub use super::CSCTRL0::TCF2;
    pub use super::CSCTRL0::TCF2EN;
    pub use super::CSCTRL0::TCI;
    pub use super::CSCTRL0::UP;
}

/// Timer Channel Comparator Status and Control Register
pub mod CSCTRL3 {
    pub use super::CSCTRL0::ALT_LOAD;
    pub use super::CSCTRL0::CL1;
    pub use super::CSCTRL0::CL2;
    pub use super::CSCTRL0::DBG_EN;
    pub use super::CSCTRL0::FAULT;
    pub use super::CSCTRL0::ROC;
    pub use super::CSCTRL0::TCF1;
    pub use super::CSCTRL0::TCF1EN;
    pub use super::CSCTRL0::TCF2;
    pub use super::CSCTRL0::TCF2EN;
    pub use super::CSCTRL0::TCI;
    pub use super::CSCTRL0::UP;
}

/// Timer Channel Input Filter Register
pub mod FILT0 {

    /// Input Filter Sample Period
    pub mod FILT_PER {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input Filter Sample Count
    pub mod FILT_CNT {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u16 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel Input Filter Register
pub mod FILT1 {
    pub use super::FILT0::FILT_CNT;
    pub use super::FILT0::FILT_PER;
}

/// Timer Channel Input Filter Register
pub mod FILT2 {
    pub use super::FILT0::FILT_CNT;
    pub use super::FILT0::FILT_PER;
}

/// Timer Channel Input Filter Register
pub mod FILT3 {
    pub use super::FILT0::FILT_CNT;
    pub use super::FILT0::FILT_PER;
}

/// Timer Channel DMA Enable Register
pub mod DMA0 {

    /// Input Edge Flag DMA Enable
    pub mod IEFDE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator Preload Register 1 DMA Enable
    pub mod CMPLD1DE {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Comparator Preload Register 2 DMA Enable
    pub mod CMPLD2DE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Timer Channel DMA Enable Register
pub mod DMA1 {
    pub use super::DMA0::CMPLD1DE;
    pub use super::DMA0::CMPLD2DE;
    pub use super::DMA0::IEFDE;
}

/// Timer Channel DMA Enable Register
pub mod DMA2 {
    pub use super::DMA0::CMPLD1DE;
    pub use super::DMA0::CMPLD2DE;
    pub use super::DMA0::IEFDE;
}

/// Timer Channel DMA Enable Register
pub mod DMA3 {
    pub use super::DMA0::CMPLD1DE;
    pub use super::DMA0::CMPLD2DE;
    pub use super::DMA0::IEFDE;
}

/// Timer Channel Enable Register
pub mod ENBL {

    /// Timer Channel Enable
    pub mod ENBL {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Timer channel is disabled.
            pub const ENBL_0: u16 = 0b0000;

            /// 0b0001: Timer channel is enabled. (default)
            pub const ENBL_1: u16 = 0b0001;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Timer Channel Compare Register 1
    pub COMP10: RWRegister<u16>,

    /// Timer Channel Compare Register 2
    pub COMP20: RWRegister<u16>,

    /// Timer Channel Capture Register
    pub CAPT0: RWRegister<u16>,

    /// Timer Channel Load Register
    pub LOAD0: RWRegister<u16>,

    /// Timer Channel Hold Register
    pub HOLD0: RWRegister<u16>,

    /// Timer Channel Counter Register
    pub CNTR0: RWRegister<u16>,

    /// Timer Channel Control Register
    pub CTRL0: RWRegister<u16>,

    /// Timer Channel Status and Control Register
    pub SCTRL0: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 1
    pub CMPLD10: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 2
    pub CMPLD20: RWRegister<u16>,

    /// Timer Channel Comparator Status and Control Register
    pub CSCTRL0: RWRegister<u16>,

    /// Timer Channel Input Filter Register
    pub FILT0: RWRegister<u16>,

    /// Timer Channel DMA Enable Register
    pub DMA0: RWRegister<u16>,

    _reserved1: [u32; 1],

    /// Timer Channel Enable Register
    pub ENBL: RWRegister<u16>,

    /// Timer Channel Compare Register 1
    pub COMP11: RWRegister<u16>,

    /// Timer Channel Compare Register 2
    pub COMP21: RWRegister<u16>,

    /// Timer Channel Capture Register
    pub CAPT1: RWRegister<u16>,

    /// Timer Channel Load Register
    pub LOAD1: RWRegister<u16>,

    /// Timer Channel Hold Register
    pub HOLD1: RWRegister<u16>,

    /// Timer Channel Counter Register
    pub CNTR1: RWRegister<u16>,

    /// Timer Channel Control Register
    pub CTRL1: RWRegister<u16>,

    /// Timer Channel Status and Control Register
    pub SCTRL1: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 1
    pub CMPLD11: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 2
    pub CMPLD21: RWRegister<u16>,

    /// Timer Channel Comparator Status and Control Register
    pub CSCTRL1: RWRegister<u16>,

    /// Timer Channel Input Filter Register
    pub FILT1: RWRegister<u16>,

    /// Timer Channel DMA Enable Register
    pub DMA1: RWRegister<u16>,

    _reserved2: [u32; 1],
    _reserved3: [u16; 1],

    /// Timer Channel Compare Register 1
    pub COMP12: RWRegister<u16>,

    /// Timer Channel Compare Register 2
    pub COMP22: RWRegister<u16>,

    /// Timer Channel Capture Register
    pub CAPT2: RWRegister<u16>,

    /// Timer Channel Load Register
    pub LOAD2: RWRegister<u16>,

    /// Timer Channel Hold Register
    pub HOLD2: RWRegister<u16>,

    /// Timer Channel Counter Register
    pub CNTR2: RWRegister<u16>,

    /// Timer Channel Control Register
    pub CTRL2: RWRegister<u16>,

    /// Timer Channel Status and Control Register
    pub SCTRL2: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 1
    pub CMPLD12: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 2
    pub CMPLD22: RWRegister<u16>,

    /// Timer Channel Comparator Status and Control Register
    pub CSCTRL2: RWRegister<u16>,

    /// Timer Channel Input Filter Register
    pub FILT2: RWRegister<u16>,

    /// Timer Channel DMA Enable Register
    pub DMA2: RWRegister<u16>,

    _reserved4: [u32; 1],
    _reserved5: [u16; 1],

    /// Timer Channel Compare Register 1
    pub COMP13: RWRegister<u16>,

    /// Timer Channel Compare Register 2
    pub COMP23: RWRegister<u16>,

    /// Timer Channel Capture Register
    pub CAPT3: RWRegister<u16>,

    /// Timer Channel Load Register
    pub LOAD3: RWRegister<u16>,

    /// Timer Channel Hold Register
    pub HOLD3: RWRegister<u16>,

    /// Timer Channel Counter Register
    pub CNTR3: RWRegister<u16>,

    /// Timer Channel Control Register
    pub CTRL3: RWRegister<u16>,

    /// Timer Channel Status and Control Register
    pub SCTRL3: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 1
    pub CMPLD13: RWRegister<u16>,

    /// Timer Channel Comparator Load Register 2
    pub CMPLD23: RWRegister<u16>,

    /// Timer Channel Comparator Status and Control Register
    pub CSCTRL3: RWRegister<u16>,

    /// Timer Channel Input Filter Register
    pub FILT3: RWRegister<u16>,

    /// Timer Channel DMA Enable Register
    pub DMA3: RWRegister<u16>,
}
pub struct ResetValues {
    pub COMP10: u16,
    pub COMP20: u16,
    pub CAPT0: u16,
    pub LOAD0: u16,
    pub HOLD0: u16,
    pub CNTR0: u16,
    pub CTRL0: u16,
    pub SCTRL0: u16,
    pub CMPLD10: u16,
    pub CMPLD20: u16,
    pub CSCTRL0: u16,
    pub FILT0: u16,
    pub DMA0: u16,
    pub ENBL: u16,
    pub COMP11: u16,
    pub COMP21: u16,
    pub CAPT1: u16,
    pub LOAD1: u16,
    pub HOLD1: u16,
    pub CNTR1: u16,
    pub CTRL1: u16,
    pub SCTRL1: u16,
    pub CMPLD11: u16,
    pub CMPLD21: u16,
    pub CSCTRL1: u16,
    pub FILT1: u16,
    pub DMA1: u16,
    pub COMP12: u16,
    pub COMP22: u16,
    pub CAPT2: u16,
    pub LOAD2: u16,
    pub HOLD2: u16,
    pub CNTR2: u16,
    pub CTRL2: u16,
    pub SCTRL2: u16,
    pub CMPLD12: u16,
    pub CMPLD22: u16,
    pub CSCTRL2: u16,
    pub FILT2: u16,
    pub DMA2: u16,
    pub COMP13: u16,
    pub COMP23: u16,
    pub CAPT3: u16,
    pub LOAD3: u16,
    pub HOLD3: u16,
    pub CNTR3: u16,
    pub CTRL3: u16,
    pub SCTRL3: u16,
    pub CMPLD13: u16,
    pub CMPLD23: u16,
    pub CSCTRL3: u16,
    pub FILT3: u16,
    pub DMA3: u16,
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

/// Access functions for the TMR1 peripheral instance
pub mod TMR1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401dc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TMR1
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TMR1_TAKEN: bool = false;

    /// Safe access to TMR1
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TMR1_TAKEN {
                None
            } else {
                TMR1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TMR1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TMR1_TAKEN && inst.addr == INSTANCE.addr {
                TMR1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TMR1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TMR1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TMR1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR1: *const RegisterBlock = 0x401dc000 as *const _;

/// Access functions for the TMR2 peripheral instance
pub mod TMR2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401e0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TMR2
    pub const reset: ResetValues = ResetValues {
        COMP10: 0x00000000,
        COMP11: 0x00000000,
        COMP12: 0x00000000,
        COMP13: 0x00000000,
        COMP20: 0x00000000,
        COMP21: 0x00000000,
        COMP22: 0x00000000,
        COMP23: 0x00000000,
        CAPT0: 0x00000000,
        CAPT1: 0x00000000,
        CAPT2: 0x00000000,
        CAPT3: 0x00000000,
        LOAD0: 0x00000000,
        LOAD1: 0x00000000,
        LOAD2: 0x00000000,
        LOAD3: 0x00000000,
        HOLD0: 0x00000000,
        HOLD1: 0x00000000,
        HOLD2: 0x00000000,
        HOLD3: 0x00000000,
        CNTR0: 0x00000000,
        CNTR1: 0x00000000,
        CNTR2: 0x00000000,
        CNTR3: 0x00000000,
        CTRL0: 0x00000000,
        CTRL1: 0x00000000,
        CTRL2: 0x00000000,
        CTRL3: 0x00000000,
        SCTRL0: 0x00000000,
        SCTRL1: 0x00000000,
        SCTRL2: 0x00000000,
        SCTRL3: 0x00000000,
        CMPLD10: 0x00000000,
        CMPLD11: 0x00000000,
        CMPLD12: 0x00000000,
        CMPLD13: 0x00000000,
        CMPLD20: 0x00000000,
        CMPLD21: 0x00000000,
        CMPLD22: 0x00000000,
        CMPLD23: 0x00000000,
        CSCTRL0: 0x00000000,
        CSCTRL1: 0x00000000,
        CSCTRL2: 0x00000000,
        CSCTRL3: 0x00000000,
        FILT0: 0x00000000,
        FILT1: 0x00000000,
        FILT2: 0x00000000,
        FILT3: 0x00000000,
        DMA0: 0x00000000,
        DMA1: 0x00000000,
        DMA2: 0x00000000,
        DMA3: 0x00000000,
        ENBL: 0x0000000F,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TMR2_TAKEN: bool = false;

    /// Safe access to TMR2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TMR2_TAKEN {
                None
            } else {
                TMR2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TMR2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TMR2_TAKEN && inst.addr == INSTANCE.addr {
                TMR2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TMR2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TMR2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TMR2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TMR2: *const RegisterBlock = 0x401e0000 as *const _;
