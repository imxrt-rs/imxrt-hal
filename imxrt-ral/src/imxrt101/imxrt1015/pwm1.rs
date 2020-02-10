#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Counter Register
pub mod SM0CNT {

    /// Counter Register Bits
    pub mod CNT {
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

/// Initial Count Register
pub mod SM0INIT {

    /// Initial Count Register Bits
    pub mod INIT {
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

/// Control 2 Register
pub mod SM0CTRL2 {

    /// Clock Source Select
    pub mod CLK_SEL {
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

            /// 0b00: The IPBus clock is used as the clock for the local prescaler and counter.
            pub const CLK_SEL_0: u32 = 0b00;

            /// 0b01: EXT_CLK is used as the clock for the local prescaler and counter.
            pub const CLK_SEL_1: u32 = 0b01;

            /// 0b10: Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0.
            pub const CLK_SEL_2: u32 = 0b10;
        }
    }

    /// Reload Source Select
    pub mod RELOAD_SEL {
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

            /// 0b0: The local RELOAD signal is used to reload registers.
            pub const RELOAD_SEL_0: u32 = 0b0;

            /// 0b1: The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0.
            pub const RELOAD_SEL_1: u32 = 0b1;
        }
    }

    /// This read/write bit determines the source of the FORCE OUTPUT signal for this submodule.
    pub mod FORCE_SEL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The local force signal, CTRL2\[FORCE\], from this submodule is used to force updates.
            pub const FORCE_SEL_0: u32 = 0b000;

            /// 0b001: The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_1: u32 = 0b001;

            /// 0b010: The local reload signal from this submodule is used to force updates without regard to the state of LDOK.
            pub const FORCE_SEL_2: u32 = 0b010;

            /// 0b011: The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_3: u32 = 0b011;

            /// 0b100: The local sync signal from this submodule is used to force updates.
            pub const FORCE_SEL_4: u32 = 0b100;

            /// 0b101: The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0.
            pub const FORCE_SEL_5: u32 = 0b101;

            /// 0b110: The external force signal, EXT_FORCE, from outside the PWM module causes updates.
            pub const FORCE_SEL_6: u32 = 0b110;

            /// 0b111: The external sync signal, EXT_SYNC, from outside the PWM module causes updates.
            pub const FORCE_SEL_7: u32 = 0b111;
        }
    }

    /// Force Initialization
    pub mod FORCE {
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

    /// FRCEN
    pub mod FRCEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Initialization from a FORCE_OUT is disabled.
            pub const FRCEN_0: u32 = 0b0;

            /// 0b1: Initialization from a FORCE_OUT is enabled.
            pub const FRCEN_1: u32 = 0b1;
        }
    }

    /// Initialization Control Select
    pub mod INIT_SEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Local sync (PWM_X) causes initialization.
            pub const INIT_SEL_0: u32 = 0b00;

            /// 0b01: Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs.
            pub const INIT_SEL_1: u32 = 0b01;

            /// 0b10: Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0.
            pub const INIT_SEL_2: u32 = 0b10;

            /// 0b11: EXT_SYNC causes initialization.
            pub const INIT_SEL_3: u32 = 0b11;
        }
    }

    /// PWM_X Initial Value
    pub mod PWMX_INIT {
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

    /// PWM45 Initial Value
    pub mod PWM45_INIT {
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

    /// PWM23 Initial Value
    pub mod PWM23_INIT {
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

    /// Independent or Complementary Pair Operation
    pub mod INDEP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_A and PWM_B form a complementary PWM pair.
            pub const INDEP_0: u32 = 0b0;

            /// 0b1: PWM_A and PWM_B outputs are independent PWMs.
            pub const INDEP_1: u32 = 0b1;
        }
    }

    /// WAIT Enable
    pub mod WAITEN {
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

    /// Debug Enable
    pub mod DBGEN {
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
}

/// Control Register
pub mod SM0CTRL {

    /// Double Switching Enable
    pub mod DBLEN {
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

            /// 0b0: Double switching disabled.
            pub const DBLEN_0: u32 = 0b0;

            /// 0b1: Double switching enabled.
            pub const DBLEN_1: u32 = 0b1;
        }
    }

    /// PWMX Double Switching Enable
    pub mod DBLX {
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

            /// 0b0: PWMX double pulse disabled.
            pub const DBLX_0: u32 = 0b0;

            /// 0b1: PWMX double pulse enabled.
            pub const DBLX_1: u32 = 0b1;
        }
    }

    /// Load Mode Select
    pub mod LDMOD {
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

            /// 0b0: Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\[LDOK\] is set.
            pub const LDMOD_0: u32 = 0b0;

            /// 0b1: Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\[LDOK\] being set. In this case it is not necessary to set CTRL\[FULL\] or CTRL\[HALF\].
            pub const LDMOD_1: u32 = 0b1;
        }
    }

    /// Split the DBLPWM signal to PWMA and PWMB
    pub mod SPLIT {
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

            /// 0b0: DBLPWM is not split. PWMA and PWMB each have double pulses.
            pub const SPLIT_0: u32 = 0b0;

            /// 0b1: DBLPWM is split to PWMA and PWMB.
            pub const SPLIT_1: u32 = 0b1;
        }
    }

    /// Prescaler
    pub mod PRSC {
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

            /// 0b000: PWM clock frequency = fclk
            pub const PRSC_0: u32 = 0b000;

            /// 0b001: PWM clock frequency = fclk/2
            pub const PRSC_1: u32 = 0b001;

            /// 0b010: PWM clock frequency = fclk/4
            pub const PRSC_2: u32 = 0b010;

            /// 0b011: PWM clock frequency = fclk/8
            pub const PRSC_3: u32 = 0b011;

            /// 0b100: PWM clock frequency = fclk/16
            pub const PRSC_4: u32 = 0b100;

            /// 0b101: PWM clock frequency = fclk/32
            pub const PRSC_5: u32 = 0b101;

            /// 0b110: PWM clock frequency = fclk/64
            pub const PRSC_6: u32 = 0b110;

            /// 0b111: PWM clock frequency = fclk/128
            pub const PRSC_7: u32 = 0b111;
        }
    }

    /// Compare Mode
    pub mod COMPMODE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The VAL* registers and the PWM counter are compared using an "equal to" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period.
            pub const COMPMODE_0: u32 = 0b0;

            /// 0b1: The VAL* registers and the PWM counter are compared using an "equal to or greater than" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value.
            pub const COMPMODE_1: u32 = 0b1;
        }
    }

    /// Deadtime
    pub mod DT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Full Cycle Reload
    pub mod FULL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Full-cycle reloads disabled.
            pub const FULL_0: u32 = 0b0;

            /// 0b1: Full-cycle reloads enabled.
            pub const FULL_1: u32 = 0b1;
        }
    }

    /// Half Cycle Reload
    pub mod HALF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Half-cycle reloads disabled.
            pub const HALF_0: u32 = 0b0;

            /// 0b1: Half-cycle reloads enabled.
            pub const HALF_1: u32 = 0b1;
        }
    }

    /// Load Frequency
    pub mod LDFQ {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Every PWM opportunity
            pub const LDFQ_0: u32 = 0b0000;

            /// 0b0001: Every 2 PWM opportunities
            pub const LDFQ_1: u32 = 0b0001;

            /// 0b0010: Every 3 PWM opportunities
            pub const LDFQ_2: u32 = 0b0010;

            /// 0b0011: Every 4 PWM opportunities
            pub const LDFQ_3: u32 = 0b0011;

            /// 0b0100: Every 5 PWM opportunities
            pub const LDFQ_4: u32 = 0b0100;

            /// 0b0101: Every 6 PWM opportunities
            pub const LDFQ_5: u32 = 0b0101;

            /// 0b0110: Every 7 PWM opportunities
            pub const LDFQ_6: u32 = 0b0110;

            /// 0b0111: Every 8 PWM opportunities
            pub const LDFQ_7: u32 = 0b0111;

            /// 0b1000: Every 9 PWM opportunities
            pub const LDFQ_8: u32 = 0b1000;

            /// 0b1001: Every 10 PWM opportunities
            pub const LDFQ_9: u32 = 0b1001;

            /// 0b1010: Every 11 PWM opportunities
            pub const LDFQ_10: u32 = 0b1010;

            /// 0b1011: Every 12 PWM opportunities
            pub const LDFQ_11: u32 = 0b1011;

            /// 0b1100: Every 13 PWM opportunities
            pub const LDFQ_12: u32 = 0b1100;

            /// 0b1101: Every 14 PWM opportunities
            pub const LDFQ_13: u32 = 0b1101;

            /// 0b1110: Every 15 PWM opportunities
            pub const LDFQ_14: u32 = 0b1110;

            /// 0b1111: Every 16 PWM opportunities
            pub const LDFQ_15: u32 = 0b1111;
        }
    }
}

/// Value Register 0
pub mod SM0VAL0 {

    /// Value Register 0
    pub mod VAL0 {
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

/// Fractional Value Register 1
pub mod SM0FRACVAL1 {

    /// Fractional Value 1 Register
    pub mod FRACVAL1 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 1
pub mod SM0VAL1 {

    /// Value Register 1
    pub mod VAL1 {
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

/// Fractional Value Register 2
pub mod SM0FRACVAL2 {

    /// Fractional Value 2
    pub mod FRACVAL2 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 2
pub mod SM0VAL2 {

    /// Value Register 2
    pub mod VAL2 {
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

/// Fractional Value Register 3
pub mod SM0FRACVAL3 {

    /// Fractional Value 3
    pub mod FRACVAL3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 3
pub mod SM0VAL3 {

    /// Value Register 3
    pub mod VAL3 {
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

/// Fractional Value Register 4
pub mod SM0FRACVAL4 {

    /// Fractional Value 4
    pub mod FRACVAL4 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 4
pub mod SM0VAL4 {

    /// Value Register 4
    pub mod VAL4 {
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

/// Fractional Value Register 5
pub mod SM0FRACVAL5 {

    /// Fractional Value 5
    pub mod FRACVAL5 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Value Register 5
pub mod SM0VAL5 {

    /// Value Register 5
    pub mod VAL5 {
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

/// Fractional Control Register
pub mod SM0FRCTRL {

    /// Fractional Cycle PWM Period Enable
    pub mod FRAC1_EN {
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

            /// 0b0: Disable fractional cycle length for the PWM period.
            pub const FRAC1_EN_0: u32 = 0b0;

            /// 0b1: Enable fractional cycle length for the PWM period.
            pub const FRAC1_EN_1: u32 = 0b1;
        }
    }

    /// Fractional Cycle Placement Enable for PWM_A
    pub mod FRAC23_EN {
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

            /// 0b0: Disable fractional cycle placement for PWM_A.
            pub const FRAC23_EN_0: u32 = 0b0;

            /// 0b1: Enable fractional cycle placement for PWM_A.
            pub const FRAC23_EN_1: u32 = 0b1;
        }
    }

    /// Fractional Cycle Placement Enable for PWM_B
    pub mod FRAC45_EN {
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

            /// 0b0: Disable fractional cycle placement for PWM_B.
            pub const FRAC45_EN_0: u32 = 0b0;

            /// 0b1: Enable fractional cycle placement for PWM_B.
            pub const FRAC45_EN_1: u32 = 0b1;
        }
    }

    /// Fractional Delay Circuit Power Up
    pub mod FRAC_PU {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Turn off fractional delay logic.
            pub const FRAC_PU_0: u32 = 0b0;

            /// 0b1: Power up fractional delay logic.
            pub const FRAC_PU_1: u32 = 0b1;
        }
    }

    /// Test Status Bit
    pub mod TEST {
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
}

/// Output Control Register
pub mod SM0OCTRL {

    /// PWM_X Fault State
    pub mod PWMXFS {
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

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMXFS_0: u32 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMXFS_1: u32 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMXFS_2: u32 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMXFS_3: u32 = 0b11;
        }
    }

    /// PWM_B Fault State
    pub mod PWMBFS {
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

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMBFS_0: u32 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMBFS_1: u32 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMBFS_2: u32 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMBFS_3: u32 = 0b11;
        }
    }

    /// PWM_A Fault State
    pub mod PWMAFS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Output is forced to logic 0 state prior to consideration of output polarity control.
            pub const PWMAFS_0: u32 = 0b00;

            /// 0b01: Output is forced to logic 1 state prior to consideration of output polarity control.
            pub const PWMAFS_1: u32 = 0b01;

            /// 0b10: Output is tristated.
            pub const PWMAFS_2: u32 = 0b10;

            /// 0b11: Output is tristated.
            pub const PWMAFS_3: u32 = 0b11;
        }
    }

    /// PWM_X Output Polarity
    pub mod POLX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_X output not inverted. A high level on the PWM_X pin represents the "on" or "active" state.
            pub const POLX_0: u32 = 0b0;

            /// 0b1: PWM_X output inverted. A low level on the PWM_X pin represents the "on" or "active" state.
            pub const POLX_1: u32 = 0b1;
        }
    }

    /// PWM_B Output Polarity
    pub mod POLB {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_B output not inverted. A high level on the PWM_B pin represents the "on" or "active" state.
            pub const POLB_0: u32 = 0b0;

            /// 0b1: PWM_B output inverted. A low level on the PWM_B pin represents the "on" or "active" state.
            pub const POLB_1: u32 = 0b1;
        }
    }

    /// PWM_A Output Polarity
    pub mod POLA {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PWM_A output not inverted. A high level on the PWM_A pin represents the "on" or "active" state.
            pub const POLA_0: u32 = 0b0;

            /// 0b1: PWM_A output inverted. A low level on the PWM_A pin represents the "on" or "active" state.
            pub const POLA_1: u32 = 0b1;
        }
    }

    /// PWM_X Input
    pub mod PWMX_IN {
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

    /// PWM_B Input
    pub mod PWMB_IN {
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

    /// PWM_A Input
    pub mod PWMA_IN {
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
}

/// Status Register
pub mod SM0STS {

    /// Compare Flags
    pub mod CMPF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: No compare event has occurred for a particular VALx value.
            pub const CMPF_0: u32 = 0b000000;

            /// 0b000001: A compare event has occurred for a particular VALx value.
            pub const CMPF_1: u32 = 0b000001;
        }
    }

    /// Capture Flag X0
    pub mod CFX0 {
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

    /// Capture Flag X1
    pub mod CFX1 {
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

    /// Capture Flag B0
    pub mod CFB0 {
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

    /// Capture Flag B1
    pub mod CFB1 {
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

    /// Capture Flag A0
    pub mod CFA0 {
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

    /// Capture Flag A1
    pub mod CFA1 {
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

    /// Reload Flag
    pub mod RF {
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

            /// 0b0: No new reload cycle since last STS\[RF\] clearing
            pub const RF_0: u32 = 0b0;

            /// 0b1: New reload cycle since last STS\[RF\] clearing
            pub const RF_1: u32 = 0b1;
        }
    }

    /// Reload Error Flag
    pub mod REF {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No reload error occurred.
            pub const REF_0: u32 = 0b0;

            /// 0b1: Reload signal occurred with non-coherent data and MCTRL\[LDOK\] = 0.
            pub const REF_1: u32 = 0b1;
        }
    }

    /// Registers Updated Flag
    pub mod RUF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No register update has occurred since last reload.
            pub const RUF_0: u32 = 0b0;

            /// 0b1: At least one of the double buffered registers has been updated since the last reload.
            pub const RUF_1: u32 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod SM0INTEN {

    /// Compare Interrupt Enables
    pub mod CMPIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: The corresponding STS\[CMPF\] bit will not cause an interrupt request.
            pub const CMPIE_0: u32 = 0b000000;

            /// 0b000001: The corresponding STS\[CMPF\] bit will cause an interrupt request.
            pub const CMPIE_1: u32 = 0b000001;
        }
    }

    /// Capture X 0 Interrupt Enable
    pub mod CX0IE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFX0\].
            pub const CX0IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFX0\].
            pub const CX0IE_1: u32 = 0b1;
        }
    }

    /// Capture X 1 Interrupt Enable
    pub mod CX1IE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFX1\].
            pub const CX1IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFX1\].
            pub const CX1IE_1: u32 = 0b1;
        }
    }

    /// Capture B 0 Interrupt Enable
    pub mod CB0IE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFB0\].
            pub const CB0IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFB0\].
            pub const CB0IE_1: u32 = 0b1;
        }
    }

    /// Capture B 1 Interrupt Enable
    pub mod CB1IE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFB1\].
            pub const CB1IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFB1\].
            pub const CB1IE_1: u32 = 0b1;
        }
    }

    /// Capture A 0 Interrupt Enable
    pub mod CA0IE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFA0\].
            pub const CA0IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFA0\].
            pub const CA0IE_1: u32 = 0b1;
        }
    }

    /// Capture A 1 Interrupt Enable
    pub mod CA1IE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt request disabled for STS\[CFA1\].
            pub const CA1IE_0: u32 = 0b0;

            /// 0b1: Interrupt request enabled for STS\[CFA1\].
            pub const CA1IE_1: u32 = 0b1;
        }
    }

    /// Reload Interrupt Enable
    pub mod RIE {
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

            /// 0b0: STS\[RF\] CPU interrupt requests disabled
            pub const RIE_0: u32 = 0b0;

            /// 0b1: STS\[RF\] CPU interrupt requests enabled
            pub const RIE_1: u32 = 0b1;
        }
    }

    /// Reload Error Interrupt Enable
    pub mod REIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: STS\[REF\] CPU interrupt requests disabled
            pub const REIE_0: u32 = 0b0;

            /// 0b1: STS\[REF\] CPU interrupt requests enabled
            pub const REIE_1: u32 = 0b1;
        }
    }
}

/// DMA Enable Register
pub mod SM0DMAEN {

    /// Capture X0 FIFO DMA Enable
    pub mod CX0DE {
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

    /// Capture X1 FIFO DMA Enable
    pub mod CX1DE {
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

    /// Capture B0 FIFO DMA Enable
    pub mod CB0DE {
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

    /// Capture B1 FIFO DMA Enable
    pub mod CB1DE {
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

    /// Capture A0 FIFO DMA Enable
    pub mod CA0DE {
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

    /// Capture A1 FIFO DMA Enable
    pub mod CA1DE {
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

    /// Capture DMA Enable Source Select
    pub mod CAPTDE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Read DMA requests disabled.
            pub const CAPTDE_0: u32 = 0b00;

            /// 0b01: Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\[CA1DE\], DMAEN\[CA0DE\], DMAEN\[CB1DE\], DMAEN\[CB0DE\], DMAEN\[CX1DE\], or DMAEN\[CX0DE\] to also be set in order to determine to which watermark(s) the DMA request is sensitive.
            pub const CAPTDE_1: u32 = 0b01;

            /// 0b10: A local sync (VAL1 matches counter) sets the read DMA request.
            pub const CAPTDE_2: u32 = 0b10;

            /// 0b11: A local reload (STS\[RF\] being set) sets the read DMA request.
            pub const CAPTDE_3: u32 = 0b11;
        }
    }

    /// FIFO Watermark AND Control
    pub mod FAND {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selected FIFO watermarks are OR'ed together.
            pub const FAND_0: u32 = 0b0;

            /// 0b1: Selected FIFO watermarks are AND'ed together.
            pub const FAND_1: u32 = 0b1;
        }
    }

    /// Value Registers DMA Enable
    pub mod VALDE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA write requests disabled
            pub const VALDE_0: u32 = 0b0;

            /// 0b1: DMA write requests for the VALx and FRACVALx registers enabled
            pub const VALDE_1: u32 = 0b1;
        }
    }
}

/// Output Trigger Control Register
pub mod SM0TCTRL {

    /// Output Trigger Enables
    pub mod OUT_TRIG_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: PWM_OUT_TRIGx will not set when the counter value matches the VALx value.
            pub const OUT_TRIG_EN_0: u32 = 0b000000;

            /// 0b000001: PWM_OUT_TRIGx will set when the counter value matches the VALx value.
            pub const OUT_TRIG_EN_1: u32 = 0b000001;
        }
    }

    /// Trigger frequency
    pub mod TRGFRQ {
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

            /// 0b0: Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\[LDFQ\] being non-zero.
            pub const TRGFRQ_0: u32 = 0b0;

            /// 0b1: Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\[LDFQ\] being non-zero.
            pub const TRGFRQ_1: u32 = 0b1;
        }
    }

    /// Output Trigger 1 Source Select
    pub mod PWBOT1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port.
            pub const PWBOT1_0: u32 = 0b0;

            /// 0b1: Route the PWMB output to the PWM_OUT_TRIG1 port.
            pub const PWBOT1_1: u32 = 0b1;
        }
    }

    /// Output Trigger 0 Source Select
    pub mod PWAOT0 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port.
            pub const PWAOT0_0: u32 = 0b0;

            /// 0b1: Route the PWMA output to the PWM_OUT_TRIG0 port.
            pub const PWAOT0_1: u32 = 0b1;
        }
    }
}

/// Fault Disable Mapping Register 0
pub mod SM0DISMAP0 {

    /// PWM_A Fault Disable Mask 0
    pub mod DIS0A {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_B Fault Disable Mask 0
    pub mod DIS0B {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_X Fault Disable Mask 0
    pub mod DIS0X {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Fault Disable Mapping Register 1
pub mod SM0DISMAP1 {

    /// PWM_A Fault Disable Mask 1
    pub mod DIS1A {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_B Fault Disable Mask 1
    pub mod DIS1B {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWM_X Fault Disable Mask 1
    pub mod DIS1X {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Deadtime Count Register 0
pub mod SM0DTCNT0 {

    /// DTCNT0
    pub mod DTCNT0 {
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

/// Deadtime Count Register 1
pub mod SM0DTCNT1 {

    /// DTCNT1
    pub mod DTCNT1 {
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

/// Capture Control A Register
pub mod SM0CAPTCTRLA {

    /// Arm A
    pub mod ARMA {
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

            /// 0b0: Input capture operation is disabled.
            pub const ARMA_0: u32 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLA\[EDGAx\] is enabled.
            pub const ARMA_1: u32 = 0b1;
        }
    }

    /// One Shot Mode A
    pub mod ONESHOTA {
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

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\[ARMA\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTA_0: u32 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\[ARMA\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLA\[ARMA\] is cleared. No further captures will be performed until CAPTCTRLA\[ARMA\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLA\[ARMA\] is then cleared.
            pub const ONESHOTA_1: u32 = 0b1;
        }
    }

    /// Edge A 0
    pub mod EDGA0 {
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

            /// 0b00: Disabled
            pub const EDGA0_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGA0_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGA0_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGA0_3: u32 = 0b11;
        }
    }

    /// Edge A 1
    pub mod EDGA1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGA1_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGA1_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGA1_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGA1_3: u32 = 0b11;
        }
    }

    /// Input Select A
    pub mod INP_SELA {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_A input signal selected as source.
            pub const INP_SELA_0: u32 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLA\[EDGA0\] and CAPTCTRLA\[EDGA1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRA\[EDGA0\] and/or CAPTCTRLA\[EDGA1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELA_1: u32 = 0b1;
        }
    }

    /// Edge Counter A Enable
    pub mod EDGCNTA_EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTA_EN_0: u32 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTA_EN_1: u32 = 0b1;
        }
    }

    /// Capture A FIFOs Water Mark
    pub mod CFAWM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A0 FIFO Word Count
    pub mod CA0CNT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture A1 FIFO Word Count
    pub mod CA1CNT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare A Register
pub mod SM0CAPTCOMPA {

    /// Edge Compare A
    pub mod EDGCMPA {
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

    /// Edge Counter A
    pub mod EDGCNTA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Control B Register
pub mod SM0CAPTCTRLB {

    /// Arm B
    pub mod ARMB {
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

            /// 0b0: Input capture operation is disabled.
            pub const ARMB_0: u32 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLB\[EDGBx\] is enabled.
            pub const ARMB_1: u32 = 0b1;
        }
    }

    /// One Shot Mode B
    pub mod ONESHOTB {
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

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\[ARMB\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTB_0: u32 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\[ARMB\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLB\[ARMB\] is cleared. No further captures will be performed until CAPTCTRLB\[ARMB\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLB\[ARMB\] is then cleared.
            pub const ONESHOTB_1: u32 = 0b1;
        }
    }

    /// Edge B 0
    pub mod EDGB0 {
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

            /// 0b00: Disabled
            pub const EDGB0_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGB0_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGB0_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGB0_3: u32 = 0b11;
        }
    }

    /// Edge B 1
    pub mod EDGB1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGB1_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGB1_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGB1_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGB1_3: u32 = 0b11;
        }
    }

    /// Input Select B
    pub mod INP_SELB {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_B input signal selected as source.
            pub const INP_SELB_0: u32 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLB\[EDGB0\] and CAPTCTRLB\[EDGB1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRB\[EDGB0\] and/or CAPTCTRLB\[EDGB1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELB_1: u32 = 0b1;
        }
    }

    /// Edge Counter B Enable
    pub mod EDGCNTB_EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTB_EN_0: u32 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTB_EN_1: u32 = 0b1;
        }
    }

    /// Capture B FIFOs Water Mark
    pub mod CFBWM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B0 FIFO Word Count
    pub mod CB0CNT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture B1 FIFO Word Count
    pub mod CB1CNT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare B Register
pub mod SM0CAPTCOMPB {

    /// Edge Compare B
    pub mod EDGCMPB {
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

    /// Edge Counter B
    pub mod EDGCNTB {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Control X Register
pub mod SM0CAPTCTRLX {

    /// Arm X
    pub mod ARMX {
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

            /// 0b0: Input capture operation is disabled.
            pub const ARMX_0: u32 = 0b0;

            /// 0b1: Input capture operation as specified by CAPTCTRLX\[EDGXx\] is enabled.
            pub const ARMX_1: u32 = 0b1;
        }
    }

    /// One Shot Mode Aux
    pub mod ONESHOTX {
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

            /// 0b0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit.
            pub const ONESHOTX_0: u32 = 0b0;

            /// 0b1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and the ARMX bit is cleared. No further captures will be performed until the ARMX bit is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and the ARMX bit is then cleared.
            pub const ONESHOTX_1: u32 = 0b1;
        }
    }

    /// Edge X 0
    pub mod EDGX0 {
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

            /// 0b00: Disabled
            pub const EDGX0_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGX0_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGX0_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGX0_3: u32 = 0b11;
        }
    }

    /// Edge X 1
    pub mod EDGX1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Disabled
            pub const EDGX1_0: u32 = 0b00;

            /// 0b01: Capture falling edges
            pub const EDGX1_1: u32 = 0b01;

            /// 0b10: Capture rising edges
            pub const EDGX1_2: u32 = 0b10;

            /// 0b11: Capture any edge
            pub const EDGX1_3: u32 = 0b11;
        }
    }

    /// Input Select X
    pub mod INP_SELX {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Raw PWM_X input signal selected as source.
            pub const INP_SELX_0: u32 = 0b0;

            /// 0b1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLX\[EDGX0\] and CAPTCTRLX\[EDGX1\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRX\[EDGX0\] and/or CAPTCTRLX\[EDGX1\] fields in order to enable one or both of the capture registers.
            pub const INP_SELX_1: u32 = 0b1;
        }
    }

    /// Edge Counter X Enable
    pub mod EDGCNTX_EN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Edge counter disabled and held in reset
            pub const EDGCNTX_EN_0: u32 = 0b0;

            /// 0b1: Edge counter enabled
            pub const EDGCNTX_EN_1: u32 = 0b1;
        }
    }

    /// Capture X FIFOs Water Mark
    pub mod CFXWM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture X0 FIFO Word Count
    pub mod CX0CNT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture X1 FIFO Word Count
    pub mod CX1CNT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Compare X Register
pub mod SM0CAPTCOMPX {

    /// Edge Compare X
    pub mod EDGCMPX {
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

    /// Edge Counter X
    pub mod EDGCNTX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 0 Register
pub mod SM0CVAL0 {

    /// CAPTVAL0
    pub mod CAPTVAL0 {
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

/// Capture Value 0 Cycle Register
pub mod SM0CVAL0CYC {

    /// CVAL0CYC
    pub mod CVAL0CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 1 Register
pub mod SM0CVAL1 {

    /// CAPTVAL1
    pub mod CAPTVAL1 {
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

/// Capture Value 1 Cycle Register
pub mod SM0CVAL1CYC {

    /// CVAL1CYC
    pub mod CVAL1CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 2 Register
pub mod SM0CVAL2 {

    /// CAPTVAL2
    pub mod CAPTVAL2 {
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

/// Capture Value 2 Cycle Register
pub mod SM0CVAL2CYC {

    /// CVAL2CYC
    pub mod CVAL2CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 3 Register
pub mod SM0CVAL3 {

    /// CAPTVAL3
    pub mod CAPTVAL3 {
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

/// Capture Value 3 Cycle Register
pub mod SM0CVAL3CYC {

    /// CVAL3CYC
    pub mod CVAL3CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 4 Register
pub mod SM0CVAL4 {

    /// CAPTVAL4
    pub mod CAPTVAL4 {
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

/// Capture Value 4 Cycle Register
pub mod SM0CVAL4CYC {

    /// CVAL4CYC
    pub mod CVAL4CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Capture Value 5 Register
pub mod SM0CVAL5 {

    /// CAPTVAL5
    pub mod CAPTVAL5 {
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

/// Capture Value 5 Cycle Register
pub mod SM0CVAL5CYC {

    /// CVAL5CYC
    pub mod CVAL5CYC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Counter Register
pub mod SM1CNT {
    pub use super::SM0CNT::CNT;
}

/// Initial Count Register
pub mod SM1INIT {
    pub use super::SM0INIT::INIT;
}

/// Control 2 Register
pub mod SM1CTRL2 {
    pub use super::SM0CTRL2::CLK_SEL;
    pub use super::SM0CTRL2::DBGEN;
    pub use super::SM0CTRL2::FORCE;
    pub use super::SM0CTRL2::FORCE_SEL;
    pub use super::SM0CTRL2::FRCEN;
    pub use super::SM0CTRL2::INDEP;
    pub use super::SM0CTRL2::INIT_SEL;
    pub use super::SM0CTRL2::PWM23_INIT;
    pub use super::SM0CTRL2::PWM45_INIT;
    pub use super::SM0CTRL2::PWMX_INIT;
    pub use super::SM0CTRL2::RELOAD_SEL;
    pub use super::SM0CTRL2::WAITEN;
}

/// Control Register
pub mod SM1CTRL {
    pub use super::SM0CTRL::COMPMODE;
    pub use super::SM0CTRL::DBLEN;
    pub use super::SM0CTRL::DBLX;
    pub use super::SM0CTRL::DT;
    pub use super::SM0CTRL::FULL;
    pub use super::SM0CTRL::HALF;
    pub use super::SM0CTRL::LDFQ;
    pub use super::SM0CTRL::LDMOD;
    pub use super::SM0CTRL::PRSC;
    pub use super::SM0CTRL::SPLIT;
}

/// Value Register 0
pub mod SM1VAL0 {
    pub use super::SM0VAL0::VAL0;
}

/// Fractional Value Register 1
pub mod SM1FRACVAL1 {
    pub use super::SM0FRACVAL1::FRACVAL1;
}

/// Value Register 1
pub mod SM1VAL1 {
    pub use super::SM0VAL1::VAL1;
}

/// Fractional Value Register 2
pub mod SM1FRACVAL2 {
    pub use super::SM0FRACVAL2::FRACVAL2;
}

/// Value Register 2
pub mod SM1VAL2 {
    pub use super::SM0VAL2::VAL2;
}

/// Fractional Value Register 3
pub mod SM1FRACVAL3 {
    pub use super::SM0FRACVAL3::FRACVAL3;
}

/// Value Register 3
pub mod SM1VAL3 {
    pub use super::SM0VAL3::VAL3;
}

/// Fractional Value Register 4
pub mod SM1FRACVAL4 {
    pub use super::SM0FRACVAL4::FRACVAL4;
}

/// Value Register 4
pub mod SM1VAL4 {
    pub use super::SM0VAL4::VAL4;
}

/// Fractional Value Register 5
pub mod SM1FRACVAL5 {
    pub use super::SM0FRACVAL5::FRACVAL5;
}

/// Value Register 5
pub mod SM1VAL5 {
    pub use super::SM0VAL5::VAL5;
}

/// Fractional Control Register
pub mod SM1FRCTRL {
    pub use super::SM0FRCTRL::FRAC1_EN;
    pub use super::SM0FRCTRL::FRAC23_EN;
    pub use super::SM0FRCTRL::FRAC45_EN;
    pub use super::SM0FRCTRL::FRAC_PU;
    pub use super::SM0FRCTRL::TEST;
}

/// Output Control Register
pub mod SM1OCTRL {
    pub use super::SM0OCTRL::POLA;
    pub use super::SM0OCTRL::POLB;
    pub use super::SM0OCTRL::POLX;
    pub use super::SM0OCTRL::PWMAFS;
    pub use super::SM0OCTRL::PWMA_IN;
    pub use super::SM0OCTRL::PWMBFS;
    pub use super::SM0OCTRL::PWMB_IN;
    pub use super::SM0OCTRL::PWMXFS;
    pub use super::SM0OCTRL::PWMX_IN;
}

/// Status Register
pub mod SM1STS {
    pub use super::SM0STS::CFA0;
    pub use super::SM0STS::CFA1;
    pub use super::SM0STS::CFB0;
    pub use super::SM0STS::CFB1;
    pub use super::SM0STS::CFX0;
    pub use super::SM0STS::CFX1;
    pub use super::SM0STS::CMPF;
    pub use super::SM0STS::REF;
    pub use super::SM0STS::RF;
    pub use super::SM0STS::RUF;
}

/// Interrupt Enable Register
pub mod SM1INTEN {
    pub use super::SM0INTEN::CA0IE;
    pub use super::SM0INTEN::CA1IE;
    pub use super::SM0INTEN::CB0IE;
    pub use super::SM0INTEN::CB1IE;
    pub use super::SM0INTEN::CMPIE;
    pub use super::SM0INTEN::CX0IE;
    pub use super::SM0INTEN::CX1IE;
    pub use super::SM0INTEN::REIE;
    pub use super::SM0INTEN::RIE;
}

/// DMA Enable Register
pub mod SM1DMAEN {
    pub use super::SM0DMAEN::CA0DE;
    pub use super::SM0DMAEN::CA1DE;
    pub use super::SM0DMAEN::CAPTDE;
    pub use super::SM0DMAEN::CB0DE;
    pub use super::SM0DMAEN::CB1DE;
    pub use super::SM0DMAEN::CX0DE;
    pub use super::SM0DMAEN::CX1DE;
    pub use super::SM0DMAEN::FAND;
    pub use super::SM0DMAEN::VALDE;
}

/// Output Trigger Control Register
pub mod SM1TCTRL {
    pub use super::SM0TCTRL::OUT_TRIG_EN;
    pub use super::SM0TCTRL::PWAOT0;
    pub use super::SM0TCTRL::PWBOT1;
    pub use super::SM0TCTRL::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SM1DISMAP0 {
    pub use super::SM0DISMAP0::DIS0A;
    pub use super::SM0DISMAP0::DIS0B;
    pub use super::SM0DISMAP0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SM1DISMAP1 {
    pub use super::SM0DISMAP1::DIS1A;
    pub use super::SM0DISMAP1::DIS1B;
    pub use super::SM0DISMAP1::DIS1X;
}

/// Deadtime Count Register 0
pub mod SM1DTCNT0 {
    pub use super::SM0DTCNT0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SM1DTCNT1 {
    pub use super::SM0DTCNT1::DTCNT1;
}

/// Capture Control A Register
pub mod SM1CAPTCTRLA {
    pub use super::SM0CAPTCTRLA::ARMA;
    pub use super::SM0CAPTCTRLA::CA0CNT;
    pub use super::SM0CAPTCTRLA::CA1CNT;
    pub use super::SM0CAPTCTRLA::CFAWM;
    pub use super::SM0CAPTCTRLA::EDGA0;
    pub use super::SM0CAPTCTRLA::EDGA1;
    pub use super::SM0CAPTCTRLA::EDGCNTA_EN;
    pub use super::SM0CAPTCTRLA::INP_SELA;
    pub use super::SM0CAPTCTRLA::ONESHOTA;
}

/// Capture Compare A Register
pub mod SM1CAPTCOMPA {
    pub use super::SM0CAPTCOMPA::EDGCMPA;
    pub use super::SM0CAPTCOMPA::EDGCNTA;
}

/// Capture Control B Register
pub mod SM1CAPTCTRLB {
    pub use super::SM0CAPTCTRLB::ARMB;
    pub use super::SM0CAPTCTRLB::CB0CNT;
    pub use super::SM0CAPTCTRLB::CB1CNT;
    pub use super::SM0CAPTCTRLB::CFBWM;
    pub use super::SM0CAPTCTRLB::EDGB0;
    pub use super::SM0CAPTCTRLB::EDGB1;
    pub use super::SM0CAPTCTRLB::EDGCNTB_EN;
    pub use super::SM0CAPTCTRLB::INP_SELB;
    pub use super::SM0CAPTCTRLB::ONESHOTB;
}

/// Capture Compare B Register
pub mod SM1CAPTCOMPB {
    pub use super::SM0CAPTCOMPB::EDGCMPB;
    pub use super::SM0CAPTCOMPB::EDGCNTB;
}

/// Capture Control X Register
pub mod SM1CAPTCTRLX {
    pub use super::SM0CAPTCTRLX::ARMX;
    pub use super::SM0CAPTCTRLX::CFXWM;
    pub use super::SM0CAPTCTRLX::CX0CNT;
    pub use super::SM0CAPTCTRLX::CX1CNT;
    pub use super::SM0CAPTCTRLX::EDGCNTX_EN;
    pub use super::SM0CAPTCTRLX::EDGX0;
    pub use super::SM0CAPTCTRLX::EDGX1;
    pub use super::SM0CAPTCTRLX::INP_SELX;
    pub use super::SM0CAPTCTRLX::ONESHOTX;
}

/// Capture Compare X Register
pub mod SM1CAPTCOMPX {
    pub use super::SM0CAPTCOMPX::EDGCMPX;
    pub use super::SM0CAPTCOMPX::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SM1CVAL0 {
    pub use super::SM0CVAL0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SM1CVAL0CYC {
    pub use super::SM0CVAL0CYC::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SM1CVAL1 {
    pub use super::SM0CVAL1::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SM1CVAL1CYC {
    pub use super::SM0CVAL1CYC::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SM1CVAL2 {
    pub use super::SM0CVAL2::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SM1CVAL2CYC {
    pub use super::SM0CVAL2CYC::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SM1CVAL3 {
    pub use super::SM0CVAL3::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SM1CVAL3CYC {
    pub use super::SM0CVAL3CYC::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SM1CVAL4 {
    pub use super::SM0CVAL4::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SM1CVAL4CYC {
    pub use super::SM0CVAL4CYC::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SM1CVAL5 {
    pub use super::SM0CVAL5::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SM1CVAL5CYC {
    pub use super::SM0CVAL5CYC::CVAL5CYC;
}

/// Counter Register
pub mod SM2CNT {
    pub use super::SM0CNT::CNT;
}

/// Initial Count Register
pub mod SM2INIT {
    pub use super::SM0INIT::INIT;
}

/// Control 2 Register
pub mod SM2CTRL2 {
    pub use super::SM0CTRL2::CLK_SEL;
    pub use super::SM0CTRL2::DBGEN;
    pub use super::SM0CTRL2::FORCE;
    pub use super::SM0CTRL2::FORCE_SEL;
    pub use super::SM0CTRL2::FRCEN;
    pub use super::SM0CTRL2::INDEP;
    pub use super::SM0CTRL2::INIT_SEL;
    pub use super::SM0CTRL2::PWM23_INIT;
    pub use super::SM0CTRL2::PWM45_INIT;
    pub use super::SM0CTRL2::PWMX_INIT;
    pub use super::SM0CTRL2::RELOAD_SEL;
    pub use super::SM0CTRL2::WAITEN;
}

/// Control Register
pub mod SM2CTRL {
    pub use super::SM0CTRL::COMPMODE;
    pub use super::SM0CTRL::DBLEN;
    pub use super::SM0CTRL::DBLX;
    pub use super::SM0CTRL::DT;
    pub use super::SM0CTRL::FULL;
    pub use super::SM0CTRL::HALF;
    pub use super::SM0CTRL::LDFQ;
    pub use super::SM0CTRL::LDMOD;
    pub use super::SM0CTRL::PRSC;
    pub use super::SM0CTRL::SPLIT;
}

/// Value Register 0
pub mod SM2VAL0 {
    pub use super::SM0VAL0::VAL0;
}

/// Fractional Value Register 1
pub mod SM2FRACVAL1 {
    pub use super::SM0FRACVAL1::FRACVAL1;
}

/// Value Register 1
pub mod SM2VAL1 {
    pub use super::SM0VAL1::VAL1;
}

/// Fractional Value Register 2
pub mod SM2FRACVAL2 {
    pub use super::SM0FRACVAL2::FRACVAL2;
}

/// Value Register 2
pub mod SM2VAL2 {
    pub use super::SM0VAL2::VAL2;
}

/// Fractional Value Register 3
pub mod SM2FRACVAL3 {
    pub use super::SM0FRACVAL3::FRACVAL3;
}

/// Value Register 3
pub mod SM2VAL3 {
    pub use super::SM0VAL3::VAL3;
}

/// Fractional Value Register 4
pub mod SM2FRACVAL4 {
    pub use super::SM0FRACVAL4::FRACVAL4;
}

/// Value Register 4
pub mod SM2VAL4 {
    pub use super::SM0VAL4::VAL4;
}

/// Fractional Value Register 5
pub mod SM2FRACVAL5 {
    pub use super::SM0FRACVAL5::FRACVAL5;
}

/// Value Register 5
pub mod SM2VAL5 {
    pub use super::SM0VAL5::VAL5;
}

/// Fractional Control Register
pub mod SM2FRCTRL {
    pub use super::SM0FRCTRL::FRAC1_EN;
    pub use super::SM0FRCTRL::FRAC23_EN;
    pub use super::SM0FRCTRL::FRAC45_EN;
    pub use super::SM0FRCTRL::FRAC_PU;
    pub use super::SM0FRCTRL::TEST;
}

/// Output Control Register
pub mod SM2OCTRL {
    pub use super::SM0OCTRL::POLA;
    pub use super::SM0OCTRL::POLB;
    pub use super::SM0OCTRL::POLX;
    pub use super::SM0OCTRL::PWMAFS;
    pub use super::SM0OCTRL::PWMA_IN;
    pub use super::SM0OCTRL::PWMBFS;
    pub use super::SM0OCTRL::PWMB_IN;
    pub use super::SM0OCTRL::PWMXFS;
    pub use super::SM0OCTRL::PWMX_IN;
}

/// Status Register
pub mod SM2STS {
    pub use super::SM0STS::CFA0;
    pub use super::SM0STS::CFA1;
    pub use super::SM0STS::CFB0;
    pub use super::SM0STS::CFB1;
    pub use super::SM0STS::CFX0;
    pub use super::SM0STS::CFX1;
    pub use super::SM0STS::CMPF;
    pub use super::SM0STS::REF;
    pub use super::SM0STS::RF;
    pub use super::SM0STS::RUF;
}

/// Interrupt Enable Register
pub mod SM2INTEN {
    pub use super::SM0INTEN::CA0IE;
    pub use super::SM0INTEN::CA1IE;
    pub use super::SM0INTEN::CB0IE;
    pub use super::SM0INTEN::CB1IE;
    pub use super::SM0INTEN::CMPIE;
    pub use super::SM0INTEN::CX0IE;
    pub use super::SM0INTEN::CX1IE;
    pub use super::SM0INTEN::REIE;
    pub use super::SM0INTEN::RIE;
}

/// DMA Enable Register
pub mod SM2DMAEN {
    pub use super::SM0DMAEN::CA0DE;
    pub use super::SM0DMAEN::CA1DE;
    pub use super::SM0DMAEN::CAPTDE;
    pub use super::SM0DMAEN::CB0DE;
    pub use super::SM0DMAEN::CB1DE;
    pub use super::SM0DMAEN::CX0DE;
    pub use super::SM0DMAEN::CX1DE;
    pub use super::SM0DMAEN::FAND;
    pub use super::SM0DMAEN::VALDE;
}

/// Output Trigger Control Register
pub mod SM2TCTRL {
    pub use super::SM0TCTRL::OUT_TRIG_EN;
    pub use super::SM0TCTRL::PWAOT0;
    pub use super::SM0TCTRL::PWBOT1;
    pub use super::SM0TCTRL::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SM2DISMAP0 {
    pub use super::SM0DISMAP0::DIS0A;
    pub use super::SM0DISMAP0::DIS0B;
    pub use super::SM0DISMAP0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SM2DISMAP1 {
    pub use super::SM0DISMAP1::DIS1A;
    pub use super::SM0DISMAP1::DIS1B;
    pub use super::SM0DISMAP1::DIS1X;
}

/// Deadtime Count Register 0
pub mod SM2DTCNT0 {
    pub use super::SM0DTCNT0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SM2DTCNT1 {
    pub use super::SM0DTCNT1::DTCNT1;
}

/// Capture Control A Register
pub mod SM2CAPTCTRLA {
    pub use super::SM0CAPTCTRLA::ARMA;
    pub use super::SM0CAPTCTRLA::CA0CNT;
    pub use super::SM0CAPTCTRLA::CA1CNT;
    pub use super::SM0CAPTCTRLA::CFAWM;
    pub use super::SM0CAPTCTRLA::EDGA0;
    pub use super::SM0CAPTCTRLA::EDGA1;
    pub use super::SM0CAPTCTRLA::EDGCNTA_EN;
    pub use super::SM0CAPTCTRLA::INP_SELA;
    pub use super::SM0CAPTCTRLA::ONESHOTA;
}

/// Capture Compare A Register
pub mod SM2CAPTCOMPA {
    pub use super::SM0CAPTCOMPA::EDGCMPA;
    pub use super::SM0CAPTCOMPA::EDGCNTA;
}

/// Capture Control B Register
pub mod SM2CAPTCTRLB {
    pub use super::SM0CAPTCTRLB::ARMB;
    pub use super::SM0CAPTCTRLB::CB0CNT;
    pub use super::SM0CAPTCTRLB::CB1CNT;
    pub use super::SM0CAPTCTRLB::CFBWM;
    pub use super::SM0CAPTCTRLB::EDGB0;
    pub use super::SM0CAPTCTRLB::EDGB1;
    pub use super::SM0CAPTCTRLB::EDGCNTB_EN;
    pub use super::SM0CAPTCTRLB::INP_SELB;
    pub use super::SM0CAPTCTRLB::ONESHOTB;
}

/// Capture Compare B Register
pub mod SM2CAPTCOMPB {
    pub use super::SM0CAPTCOMPB::EDGCMPB;
    pub use super::SM0CAPTCOMPB::EDGCNTB;
}

/// Capture Control X Register
pub mod SM2CAPTCTRLX {
    pub use super::SM0CAPTCTRLX::ARMX;
    pub use super::SM0CAPTCTRLX::CFXWM;
    pub use super::SM0CAPTCTRLX::CX0CNT;
    pub use super::SM0CAPTCTRLX::CX1CNT;
    pub use super::SM0CAPTCTRLX::EDGCNTX_EN;
    pub use super::SM0CAPTCTRLX::EDGX0;
    pub use super::SM0CAPTCTRLX::EDGX1;
    pub use super::SM0CAPTCTRLX::INP_SELX;
    pub use super::SM0CAPTCTRLX::ONESHOTX;
}

/// Capture Compare X Register
pub mod SM2CAPTCOMPX {
    pub use super::SM0CAPTCOMPX::EDGCMPX;
    pub use super::SM0CAPTCOMPX::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SM2CVAL0 {
    pub use super::SM0CVAL0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SM2CVAL0CYC {
    pub use super::SM0CVAL0CYC::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SM2CVAL1 {
    pub use super::SM0CVAL1::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SM2CVAL1CYC {
    pub use super::SM0CVAL1CYC::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SM2CVAL2 {
    pub use super::SM0CVAL2::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SM2CVAL2CYC {
    pub use super::SM0CVAL2CYC::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SM2CVAL3 {
    pub use super::SM0CVAL3::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SM2CVAL3CYC {
    pub use super::SM0CVAL3CYC::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SM2CVAL4 {
    pub use super::SM0CVAL4::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SM2CVAL4CYC {
    pub use super::SM0CVAL4CYC::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SM2CVAL5 {
    pub use super::SM0CVAL5::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SM2CVAL5CYC {
    pub use super::SM0CVAL5CYC::CVAL5CYC;
}

/// Counter Register
pub mod SM3CNT {
    pub use super::SM0CNT::CNT;
}

/// Initial Count Register
pub mod SM3INIT {
    pub use super::SM0INIT::INIT;
}

/// Control 2 Register
pub mod SM3CTRL2 {
    pub use super::SM0CTRL2::CLK_SEL;
    pub use super::SM0CTRL2::DBGEN;
    pub use super::SM0CTRL2::FORCE;
    pub use super::SM0CTRL2::FORCE_SEL;
    pub use super::SM0CTRL2::FRCEN;
    pub use super::SM0CTRL2::INDEP;
    pub use super::SM0CTRL2::INIT_SEL;
    pub use super::SM0CTRL2::PWM23_INIT;
    pub use super::SM0CTRL2::PWM45_INIT;
    pub use super::SM0CTRL2::PWMX_INIT;
    pub use super::SM0CTRL2::RELOAD_SEL;
    pub use super::SM0CTRL2::WAITEN;
}

/// Control Register
pub mod SM3CTRL {
    pub use super::SM0CTRL::COMPMODE;
    pub use super::SM0CTRL::DBLEN;
    pub use super::SM0CTRL::DBLX;
    pub use super::SM0CTRL::DT;
    pub use super::SM0CTRL::FULL;
    pub use super::SM0CTRL::HALF;
    pub use super::SM0CTRL::LDFQ;
    pub use super::SM0CTRL::LDMOD;
    pub use super::SM0CTRL::PRSC;
    pub use super::SM0CTRL::SPLIT;
}

/// Value Register 0
pub mod SM3VAL0 {
    pub use super::SM0VAL0::VAL0;
}

/// Fractional Value Register 1
pub mod SM3FRACVAL1 {
    pub use super::SM0FRACVAL1::FRACVAL1;
}

/// Value Register 1
pub mod SM3VAL1 {
    pub use super::SM0VAL1::VAL1;
}

/// Fractional Value Register 2
pub mod SM3FRACVAL2 {
    pub use super::SM0FRACVAL2::FRACVAL2;
}

/// Value Register 2
pub mod SM3VAL2 {
    pub use super::SM0VAL2::VAL2;
}

/// Fractional Value Register 3
pub mod SM3FRACVAL3 {
    pub use super::SM0FRACVAL3::FRACVAL3;
}

/// Value Register 3
pub mod SM3VAL3 {
    pub use super::SM0VAL3::VAL3;
}

/// Fractional Value Register 4
pub mod SM3FRACVAL4 {
    pub use super::SM0FRACVAL4::FRACVAL4;
}

/// Value Register 4
pub mod SM3VAL4 {
    pub use super::SM0VAL4::VAL4;
}

/// Fractional Value Register 5
pub mod SM3FRACVAL5 {
    pub use super::SM0FRACVAL5::FRACVAL5;
}

/// Value Register 5
pub mod SM3VAL5 {
    pub use super::SM0VAL5::VAL5;
}

/// Fractional Control Register
pub mod SM3FRCTRL {
    pub use super::SM0FRCTRL::FRAC1_EN;
    pub use super::SM0FRCTRL::FRAC23_EN;
    pub use super::SM0FRCTRL::FRAC45_EN;
    pub use super::SM0FRCTRL::FRAC_PU;
    pub use super::SM0FRCTRL::TEST;
}

/// Output Control Register
pub mod SM3OCTRL {
    pub use super::SM0OCTRL::POLA;
    pub use super::SM0OCTRL::POLB;
    pub use super::SM0OCTRL::POLX;
    pub use super::SM0OCTRL::PWMAFS;
    pub use super::SM0OCTRL::PWMA_IN;
    pub use super::SM0OCTRL::PWMBFS;
    pub use super::SM0OCTRL::PWMB_IN;
    pub use super::SM0OCTRL::PWMXFS;
    pub use super::SM0OCTRL::PWMX_IN;
}

/// Status Register
pub mod SM3STS {
    pub use super::SM0STS::CFA0;
    pub use super::SM0STS::CFA1;
    pub use super::SM0STS::CFB0;
    pub use super::SM0STS::CFB1;
    pub use super::SM0STS::CFX0;
    pub use super::SM0STS::CFX1;
    pub use super::SM0STS::CMPF;
    pub use super::SM0STS::REF;
    pub use super::SM0STS::RF;
    pub use super::SM0STS::RUF;
}

/// Interrupt Enable Register
pub mod SM3INTEN {
    pub use super::SM0INTEN::CA0IE;
    pub use super::SM0INTEN::CA1IE;
    pub use super::SM0INTEN::CB0IE;
    pub use super::SM0INTEN::CB1IE;
    pub use super::SM0INTEN::CMPIE;
    pub use super::SM0INTEN::CX0IE;
    pub use super::SM0INTEN::CX1IE;
    pub use super::SM0INTEN::REIE;
    pub use super::SM0INTEN::RIE;
}

/// DMA Enable Register
pub mod SM3DMAEN {
    pub use super::SM0DMAEN::CA0DE;
    pub use super::SM0DMAEN::CA1DE;
    pub use super::SM0DMAEN::CAPTDE;
    pub use super::SM0DMAEN::CB0DE;
    pub use super::SM0DMAEN::CB1DE;
    pub use super::SM0DMAEN::CX0DE;
    pub use super::SM0DMAEN::CX1DE;
    pub use super::SM0DMAEN::FAND;
    pub use super::SM0DMAEN::VALDE;
}

/// Output Trigger Control Register
pub mod SM3TCTRL {
    pub use super::SM0TCTRL::OUT_TRIG_EN;
    pub use super::SM0TCTRL::PWAOT0;
    pub use super::SM0TCTRL::PWBOT1;
    pub use super::SM0TCTRL::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SM3DISMAP0 {
    pub use super::SM0DISMAP0::DIS0A;
    pub use super::SM0DISMAP0::DIS0B;
    pub use super::SM0DISMAP0::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SM3DISMAP1 {
    pub use super::SM0DISMAP1::DIS1A;
    pub use super::SM0DISMAP1::DIS1B;
    pub use super::SM0DISMAP1::DIS1X;
}

/// Deadtime Count Register 0
pub mod SM3DTCNT0 {
    pub use super::SM0DTCNT0::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SM3DTCNT1 {
    pub use super::SM0DTCNT1::DTCNT1;
}

/// Capture Control A Register
pub mod SM3CAPTCTRLA {
    pub use super::SM0CAPTCTRLA::ARMA;
    pub use super::SM0CAPTCTRLA::CA0CNT;
    pub use super::SM0CAPTCTRLA::CA1CNT;
    pub use super::SM0CAPTCTRLA::CFAWM;
    pub use super::SM0CAPTCTRLA::EDGA0;
    pub use super::SM0CAPTCTRLA::EDGA1;
    pub use super::SM0CAPTCTRLA::EDGCNTA_EN;
    pub use super::SM0CAPTCTRLA::INP_SELA;
    pub use super::SM0CAPTCTRLA::ONESHOTA;
}

/// Capture Compare A Register
pub mod SM3CAPTCOMPA {
    pub use super::SM0CAPTCOMPA::EDGCMPA;
    pub use super::SM0CAPTCOMPA::EDGCNTA;
}

/// Capture Control B Register
pub mod SM3CAPTCTRLB {
    pub use super::SM0CAPTCTRLB::ARMB;
    pub use super::SM0CAPTCTRLB::CB0CNT;
    pub use super::SM0CAPTCTRLB::CB1CNT;
    pub use super::SM0CAPTCTRLB::CFBWM;
    pub use super::SM0CAPTCTRLB::EDGB0;
    pub use super::SM0CAPTCTRLB::EDGB1;
    pub use super::SM0CAPTCTRLB::EDGCNTB_EN;
    pub use super::SM0CAPTCTRLB::INP_SELB;
    pub use super::SM0CAPTCTRLB::ONESHOTB;
}

/// Capture Compare B Register
pub mod SM3CAPTCOMPB {
    pub use super::SM0CAPTCOMPB::EDGCMPB;
    pub use super::SM0CAPTCOMPB::EDGCNTB;
}

/// Capture Control X Register
pub mod SM3CAPTCTRLX {
    pub use super::SM0CAPTCTRLX::ARMX;
    pub use super::SM0CAPTCTRLX::CFXWM;
    pub use super::SM0CAPTCTRLX::CX0CNT;
    pub use super::SM0CAPTCTRLX::CX1CNT;
    pub use super::SM0CAPTCTRLX::EDGCNTX_EN;
    pub use super::SM0CAPTCTRLX::EDGX0;
    pub use super::SM0CAPTCTRLX::EDGX1;
    pub use super::SM0CAPTCTRLX::INP_SELX;
    pub use super::SM0CAPTCTRLX::ONESHOTX;
}

/// Capture Compare X Register
pub mod SM3CAPTCOMPX {
    pub use super::SM0CAPTCOMPX::EDGCMPX;
    pub use super::SM0CAPTCOMPX::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SM3CVAL0 {
    pub use super::SM0CVAL0::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SM3CVAL0CYC {
    pub use super::SM0CVAL0CYC::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SM3CVAL1 {
    pub use super::SM0CVAL1::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SM3CVAL1CYC {
    pub use super::SM0CVAL1CYC::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SM3CVAL2 {
    pub use super::SM0CVAL2::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SM3CVAL2CYC {
    pub use super::SM0CVAL2CYC::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SM3CVAL3 {
    pub use super::SM0CVAL3::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SM3CVAL3CYC {
    pub use super::SM0CVAL3CYC::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SM3CVAL4 {
    pub use super::SM0CVAL4::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SM3CVAL4CYC {
    pub use super::SM0CVAL4CYC::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SM3CVAL5 {
    pub use super::SM0CVAL5::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SM3CVAL5CYC {
    pub use super::SM0CVAL5CYC::CVAL5CYC;
}

/// Output Enable Register
pub mod OUTEN {

    /// PWM_X Output Enables
    pub mod PWMX_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_X output disabled.
            pub const PWMX_EN_0: u32 = 0b0000;

            /// 0b0001: PWM_X output enabled.
            pub const PWMX_EN_1: u32 = 0b0001;
        }
    }

    /// PWM_B Output Enables
    pub mod PWMB_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_B output disabled.
            pub const PWMB_EN_0: u32 = 0b0000;

            /// 0b0001: PWM_B output enabled.
            pub const PWMB_EN_1: u32 = 0b0001;
        }
    }

    /// PWM_A Output Enables
    pub mod PWMA_EN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_A output disabled.
            pub const PWMA_EN_0: u32 = 0b0000;

            /// 0b0001: PWM_A output enabled.
            pub const PWMA_EN_1: u32 = 0b0001;
        }
    }
}

/// Mask Register
pub mod MASK {

    /// PWM_X Masks
    pub mod MASKX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_X output normal.
            pub const MASKX_0: u32 = 0b0000;

            /// 0b0001: PWM_X output masked.
            pub const MASKX_1: u32 = 0b0001;
        }
    }

    /// PWM_B Masks
    pub mod MASKB {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_B output normal.
            pub const MASKB_0: u32 = 0b0000;

            /// 0b0001: PWM_B output masked.
            pub const MASKB_1: u32 = 0b0001;
        }
    }

    /// PWM_A Masks
    pub mod MASKA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM_A output normal.
            pub const MASKA_0: u32 = 0b0000;

            /// 0b0001: PWM_A output masked.
            pub const MASKA_1: u32 = 0b0001;
        }
    }

    /// Update Mask Bits Immediately
    pub mod UPDATE_MASK {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule.
            pub const UPDATE_MASK_0: u32 = 0b0000;

            /// 0b0001: Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit.
            pub const UPDATE_MASK_1: u32 = 0b0001;
        }
    }
}

/// Software Controlled Output Register
pub mod SWCOUT {

    /// Submodule 0 Software Controlled Output 45
    pub mod SM0OUT45 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45.
            pub const SM0OUT45_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45.
            pub const SM0OUT45_1: u32 = 0b1;
        }
    }

    /// Submodule 0 Software Controlled Output 23
    pub mod SM0OUT23 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23.
            pub const SM0OUT23_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23.
            pub const SM0OUT23_1: u32 = 0b1;
        }
    }

    /// Submodule 1 Software Controlled Output 45
    pub mod SM1OUT45 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45.
            pub const SM1OUT45_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45.
            pub const SM1OUT45_1: u32 = 0b1;
        }
    }

    /// Submodule 1 Software Controlled Output 23
    pub mod SM1OUT23 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23.
            pub const SM1OUT23_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23.
            pub const SM1OUT23_1: u32 = 0b1;
        }
    }

    /// Submodule 2 Software Controlled Output 45
    pub mod SM2OUT45 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45.
            pub const SM2OUT45_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45.
            pub const SM2OUT45_1: u32 = 0b1;
        }
    }

    /// Submodule 2 Software Controlled Output 23
    pub mod SM2OUT23 {
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

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23.
            pub const SM2OUT23_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23.
            pub const SM2OUT23_1: u32 = 0b1;
        }
    }

    /// Submodule 3 Software Controlled Output 45
    pub mod SM3OUT45 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45.
            pub const SM3OUT45_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45.
            pub const SM3OUT45_1: u32 = 0b1;
        }
    }

    /// Submodule 3 Software Controlled Output 23
    pub mod SM3OUT23 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23.
            pub const SM3OUT23_0: u32 = 0b0;

            /// 0b1: A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23.
            pub const SM3OUT23_1: u32 = 0b1;
        }
    }
}

/// PWM Source Select Register
pub mod DTSRCSEL {

    /// Submodule 0 PWM45 Control Select
    pub mod SM0SEL45 {
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

            /// 0b00: Generated SM0PWM45 signal is used by the deadtime logic.
            pub const SM0SEL45_0: u32 = 0b00;

            /// 0b01: Inverted generated SM0PWM45 signal is used by the deadtime logic.
            pub const SM0SEL45_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM0OUT45\] is used by the deadtime logic.
            pub const SM0SEL45_2: u32 = 0b10;

            /// 0b11: PWM0_EXTB signal is used by the deadtime logic.
            pub const SM0SEL45_3: u32 = 0b11;
        }
    }

    /// Submodule 0 PWM23 Control Select
    pub mod SM0SEL23 {
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

            /// 0b00: Generated SM0PWM23 signal is used by the deadtime logic.
            pub const SM0SEL23_0: u32 = 0b00;

            /// 0b01: Inverted generated SM0PWM23 signal is used by the deadtime logic.
            pub const SM0SEL23_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM0OUT23\] is used by the deadtime logic.
            pub const SM0SEL23_2: u32 = 0b10;

            /// 0b11: PWM0_EXTA signal is used by the deadtime logic.
            pub const SM0SEL23_3: u32 = 0b11;
        }
    }

    /// Submodule 1 PWM45 Control Select
    pub mod SM1SEL45 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM1PWM45 signal is used by the deadtime logic.
            pub const SM1SEL45_0: u32 = 0b00;

            /// 0b01: Inverted generated SM1PWM45 signal is used by the deadtime logic.
            pub const SM1SEL45_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM1OUT45\] is used by the deadtime logic.
            pub const SM1SEL45_2: u32 = 0b10;

            /// 0b11: PWM1_EXTB signal is used by the deadtime logic.
            pub const SM1SEL45_3: u32 = 0b11;
        }
    }

    /// Submodule 1 PWM23 Control Select
    pub mod SM1SEL23 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM1PWM23 signal is used by the deadtime logic.
            pub const SM1SEL23_0: u32 = 0b00;

            /// 0b01: Inverted generated SM1PWM23 signal is used by the deadtime logic.
            pub const SM1SEL23_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM1OUT23\] is used by the deadtime logic.
            pub const SM1SEL23_2: u32 = 0b10;

            /// 0b11: PWM1_EXTA signal is used by the deadtime logic.
            pub const SM1SEL23_3: u32 = 0b11;
        }
    }

    /// Submodule 2 PWM45 Control Select
    pub mod SM2SEL45 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM2PWM45 signal is used by the deadtime logic.
            pub const SM2SEL45_0: u32 = 0b00;

            /// 0b01: Inverted generated SM2PWM45 signal is used by the deadtime logic.
            pub const SM2SEL45_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM2OUT45\] is used by the deadtime logic.
            pub const SM2SEL45_2: u32 = 0b10;

            /// 0b11: PWM2_EXTB signal is used by the deadtime logic.
            pub const SM2SEL45_3: u32 = 0b11;
        }
    }

    /// Submodule 2 PWM23 Control Select
    pub mod SM2SEL23 {
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

            /// 0b00: Generated SM2PWM23 signal is used by the deadtime logic.
            pub const SM2SEL23_0: u32 = 0b00;

            /// 0b01: Inverted generated SM2PWM23 signal is used by the deadtime logic.
            pub const SM2SEL23_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM2OUT23\] is used by the deadtime logic.
            pub const SM2SEL23_2: u32 = 0b10;

            /// 0b11: PWM2_EXTA signal is used by the deadtime logic.
            pub const SM2SEL23_3: u32 = 0b11;
        }
    }

    /// Submodule 3 PWM45 Control Select
    pub mod SM3SEL45 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Generated SM3PWM45 signal is used by the deadtime logic.
            pub const SM3SEL45_0: u32 = 0b00;

            /// 0b01: Inverted generated SM3PWM45 signal is used by the deadtime logic.
            pub const SM3SEL45_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM3OUT45\] is used by the deadtime logic.
            pub const SM3SEL45_2: u32 = 0b10;

            /// 0b11: PWM3_EXTB signal is used by the deadtime logic.
            pub const SM3SEL45_3: u32 = 0b11;
        }
    }

    /// Submodule 3 PWM23 Control Select
    pub mod SM3SEL23 {
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

            /// 0b00: Generated SM3PWM23 signal is used by the deadtime logic.
            pub const SM3SEL23_0: u32 = 0b00;

            /// 0b01: Inverted generated SM3PWM23 signal is used by the deadtime logic.
            pub const SM3SEL23_1: u32 = 0b01;

            /// 0b10: SWCOUT\[SM3OUT23\] is used by the deadtime logic.
            pub const SM3SEL23_2: u32 = 0b10;

            /// 0b11: PWM3_EXTA signal is used by the deadtime logic.
            pub const SM3SEL23_3: u32 = 0b11;
        }
    }
}

/// Master Control Register
pub mod MCTRL {

    /// Load Okay
    pub mod LDOK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Do not load new values.
            pub const LDOK_0: u32 = 0b0000;

            /// 0b0001: Load prescaler, modulus, and PWM values of the corresponding submodule.
            pub const LDOK_1: u32 = 0b0001;
        }
    }

    /// Clear Load Okay
    pub mod CLDOK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Run
    pub mod RUN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM generator is disabled in the corresponding submodule.
            pub const RUN_0: u32 = 0b0000;

            /// 0b0001: PWM generator is enabled in the corresponding submodule.
            pub const RUN_1: u32 = 0b0001;
        }
    }

    /// Current Polarity
    pub mod IPOL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM23 is used to generate complementary PWM pair in the corresponding submodule.
            pub const IPOL_0: u32 = 0b0000;

            /// 0b0001: PWM45 is used to generate complementary PWM pair in the corresponding submodule.
            pub const IPOL_1: u32 = 0b0001;
        }
    }
}

/// Master Control 2 Register
pub mod MCTRL2 {

    /// Monitor PLL State
    pub mod MONPLL {
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

            /// 0b00: Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software.
            pub const MONPLL_0: u32 = 0b00;

            /// 0b01: Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems.
            pub const MONPLL_1: u32 = 0b01;

            /// 0b10: Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset.
            pub const MONPLL_2: u32 = 0b10;

            /// 0b11: Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset.
            pub const MONPLL_3: u32 = 0b11;
        }
    }
}

/// Fault Control Register
pub mod FCTRL0 {

    /// Fault Interrupt Enables
    pub mod FIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: FAULTx CPU interrupt requests disabled.
            pub const FIE_0: u32 = 0b0000;

            /// 0b0001: FAULTx CPU interrupt requests enabled.
            pub const FIE_1: u32 = 0b0001;
        }
    }

    /// Fault Safety Mode
    pub mod FSAFE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\] without regard to the state of FSTS\[FFPINx\]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn).
            pub const FSAFE_0: u32 = 0b0000;

            /// 0b0001: Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear and FSTS\[FFPINx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\].
            pub const FSAFE_1: u32 = 0b0001;
        }
    }

    /// Automatic Fault Clearing
    pub mod FAUTO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\[FFLAGx\] is clear at the start of a half cycle or full cycle depending the state of FSTS\[FFULL\]. This is further controlled by FCTRL\[FSAFE\].
            pub const FAUTO_0: u32 = 0b0000;

            /// 0b0001: Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\[FFPINx\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\[FFULL\] without regard to the state of FSTS\[FFLAGx\].
            pub const FAUTO_1: u32 = 0b0001;
        }
    }

    /// Fault Level
    pub mod FLVL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: A logic 0 on the fault input indicates a fault condition.
            pub const FLVL_0: u32 = 0b0000;

            /// 0b0001: A logic 1 on the fault input indicates a fault condition.
            pub const FLVL_1: u32 = 0b0001;
        }
    }
}

/// Fault Status Register
pub mod FSTS0 {

    /// Fault Flags
    pub mod FFLAG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No fault on the FAULTx pin.
            pub const FFLAG_0: u32 = 0b0000;

            /// 0b0001: Fault on the FAULTx pin.
            pub const FFLAG_1: u32 = 0b0001;
        }
    }

    /// Full Cycle
    pub mod FFULL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM outputs are not re-enabled at the start of a full cycle
            pub const FFULL_0: u32 = 0b0000;

            /// 0b0001: PWM outputs are re-enabled at the start of a full cycle
            pub const FFULL_1: u32 = 0b0001;
        }
    }

    /// Filtered Fault Pins
    pub mod FFPIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Half Cycle Fault Recovery
    pub mod FHALF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: PWM outputs are not re-enabled at the start of a half cycle.
            pub const FHALF_0: u32 = 0b0000;

            /// 0b0001: PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0).
            pub const FHALF_1: u32 = 0b0001;
        }
    }
}

/// Fault Filter Register
pub mod FFILT0 {

    /// Fault Filter Period
    pub mod FILT_PER {
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

    /// Fault Filter Count
    pub mod FILT_CNT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fault Glitch Stretch Enable
    pub mod GSTR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fault input glitch stretching is disabled.
            pub const GSTR_0: u32 = 0b0;

            /// 0b1: Input fault signals will be stretched to at least 2 IPBus clock cycles.
            pub const GSTR_1: u32 = 0b1;
        }
    }
}

/// Fault Test Register
pub mod FTST0 {

    /// Fault Test
    pub mod FTEST {
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

            /// 0b0: No fault
            pub const FTEST_0: u32 = 0b0;

            /// 0b1: Cause a simulated fault
            pub const FTEST_1: u32 = 0b1;
        }
    }
}

/// Fault Control 2 Register
pub mod FCTRL20 {

    /// No Combinational Path From Fault Input To PWM Output
    pub mod NOCOMB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs.
            pub const NOCOMB_0: u32 = 0b0000;

            /// 0b0001: The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs.
            pub const NOCOMB_1: u32 = 0b0001;
        }
    }
}
pub struct RegisterBlock {
    /// Counter Register
    pub SM0CNT: RORegister<u16>,

    /// Initial Count Register
    pub SM0INIT: RWRegister<u16>,

    /// Control 2 Register
    pub SM0CTRL2: RWRegister<u16>,

    /// Control Register
    pub SM0CTRL: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// Value Register 0
    pub SM0VAL0: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SM0FRACVAL1: RWRegister<u16>,

    /// Value Register 1
    pub SM0VAL1: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SM0FRACVAL2: RWRegister<u16>,

    /// Value Register 2
    pub SM0VAL2: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SM0FRACVAL3: RWRegister<u16>,

    /// Value Register 3
    pub SM0VAL3: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SM0FRACVAL4: RWRegister<u16>,

    /// Value Register 4
    pub SM0VAL4: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SM0FRACVAL5: RWRegister<u16>,

    /// Value Register 5
    pub SM0VAL5: RWRegister<u16>,

    /// Fractional Control Register
    pub SM0FRCTRL: RWRegister<u16>,

    /// Output Control Register
    pub SM0OCTRL: RWRegister<u16>,

    /// Status Register
    pub SM0STS: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SM0INTEN: RWRegister<u16>,

    /// DMA Enable Register
    pub SM0DMAEN: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SM0TCTRL: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SM0DISMAP0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SM0DISMAP1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SM0DTCNT0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SM0DTCNT1: RWRegister<u16>,

    /// Capture Control A Register
    pub SM0CAPTCTRLA: RWRegister<u16>,

    /// Capture Compare A Register
    pub SM0CAPTCOMPA: RWRegister<u16>,

    /// Capture Control B Register
    pub SM0CAPTCTRLB: RWRegister<u16>,

    /// Capture Compare B Register
    pub SM0CAPTCOMPB: RWRegister<u16>,

    /// Capture Control X Register
    pub SM0CAPTCTRLX: RWRegister<u16>,

    /// Capture Compare X Register
    pub SM0CAPTCOMPX: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SM0CVAL0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SM0CVAL0CYC: RORegister<u16>,

    /// Capture Value 1 Register
    pub SM0CVAL1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SM0CVAL1CYC: RORegister<u16>,

    /// Capture Value 2 Register
    pub SM0CVAL2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SM0CVAL2CYC: RORegister<u16>,

    /// Capture Value 3 Register
    pub SM0CVAL3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SM0CVAL3CYC: RORegister<u16>,

    /// Capture Value 4 Register
    pub SM0CVAL4: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SM0CVAL4CYC: RORegister<u16>,

    /// Capture Value 5 Register
    pub SM0CVAL5: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SM0CVAL5CYC: RORegister<u16>,

    _reserved2: [u32; 2],

    /// Counter Register
    pub SM1CNT: RORegister<u16>,

    /// Initial Count Register
    pub SM1INIT: RWRegister<u16>,

    /// Control 2 Register
    pub SM1CTRL2: RWRegister<u16>,

    /// Control Register
    pub SM1CTRL: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// Value Register 0
    pub SM1VAL0: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SM1FRACVAL1: RWRegister<u16>,

    /// Value Register 1
    pub SM1VAL1: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SM1FRACVAL2: RWRegister<u16>,

    /// Value Register 2
    pub SM1VAL2: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SM1FRACVAL3: RWRegister<u16>,

    /// Value Register 3
    pub SM1VAL3: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SM1FRACVAL4: RWRegister<u16>,

    /// Value Register 4
    pub SM1VAL4: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SM1FRACVAL5: RWRegister<u16>,

    /// Value Register 5
    pub SM1VAL5: RWRegister<u16>,

    /// Fractional Control Register
    pub SM1FRCTRL: RWRegister<u16>,

    /// Output Control Register
    pub SM1OCTRL: RWRegister<u16>,

    /// Status Register
    pub SM1STS: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SM1INTEN: RWRegister<u16>,

    /// DMA Enable Register
    pub SM1DMAEN: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SM1TCTRL: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SM1DISMAP0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SM1DISMAP1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SM1DTCNT0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SM1DTCNT1: RWRegister<u16>,

    /// Capture Control A Register
    pub SM1CAPTCTRLA: RWRegister<u16>,

    /// Capture Compare A Register
    pub SM1CAPTCOMPA: RWRegister<u16>,

    /// Capture Control B Register
    pub SM1CAPTCTRLB: RWRegister<u16>,

    /// Capture Compare B Register
    pub SM1CAPTCOMPB: RWRegister<u16>,

    /// Capture Control X Register
    pub SM1CAPTCTRLX: RWRegister<u16>,

    /// Capture Compare X Register
    pub SM1CAPTCOMPX: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SM1CVAL0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SM1CVAL0CYC: RORegister<u16>,

    /// Capture Value 1 Register
    pub SM1CVAL1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SM1CVAL1CYC: RORegister<u16>,

    /// Capture Value 2 Register
    pub SM1CVAL2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SM1CVAL2CYC: RORegister<u16>,

    /// Capture Value 3 Register
    pub SM1CVAL3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SM1CVAL3CYC: RORegister<u16>,

    /// Capture Value 4 Register
    pub SM1CVAL4: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SM1CVAL4CYC: RORegister<u16>,

    /// Capture Value 5 Register
    pub SM1CVAL5: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SM1CVAL5CYC: RORegister<u16>,

    _reserved4: [u32; 2],

    /// Counter Register
    pub SM2CNT: RORegister<u16>,

    /// Initial Count Register
    pub SM2INIT: RWRegister<u16>,

    /// Control 2 Register
    pub SM2CTRL2: RWRegister<u16>,

    /// Control Register
    pub SM2CTRL: RWRegister<u16>,

    _reserved5: [u16; 1],

    /// Value Register 0
    pub SM2VAL0: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SM2FRACVAL1: RWRegister<u16>,

    /// Value Register 1
    pub SM2VAL1: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SM2FRACVAL2: RWRegister<u16>,

    /// Value Register 2
    pub SM2VAL2: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SM2FRACVAL3: RWRegister<u16>,

    /// Value Register 3
    pub SM2VAL3: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SM2FRACVAL4: RWRegister<u16>,

    /// Value Register 4
    pub SM2VAL4: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SM2FRACVAL5: RWRegister<u16>,

    /// Value Register 5
    pub SM2VAL5: RWRegister<u16>,

    /// Fractional Control Register
    pub SM2FRCTRL: RWRegister<u16>,

    /// Output Control Register
    pub SM2OCTRL: RWRegister<u16>,

    /// Status Register
    pub SM2STS: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SM2INTEN: RWRegister<u16>,

    /// DMA Enable Register
    pub SM2DMAEN: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SM2TCTRL: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SM2DISMAP0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SM2DISMAP1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SM2DTCNT0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SM2DTCNT1: RWRegister<u16>,

    /// Capture Control A Register
    pub SM2CAPTCTRLA: RWRegister<u16>,

    /// Capture Compare A Register
    pub SM2CAPTCOMPA: RWRegister<u16>,

    /// Capture Control B Register
    pub SM2CAPTCTRLB: RWRegister<u16>,

    /// Capture Compare B Register
    pub SM2CAPTCOMPB: RWRegister<u16>,

    /// Capture Control X Register
    pub SM2CAPTCTRLX: RWRegister<u16>,

    /// Capture Compare X Register
    pub SM2CAPTCOMPX: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SM2CVAL0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SM2CVAL0CYC: RORegister<u16>,

    /// Capture Value 1 Register
    pub SM2CVAL1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SM2CVAL1CYC: RORegister<u16>,

    /// Capture Value 2 Register
    pub SM2CVAL2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SM2CVAL2CYC: RORegister<u16>,

    /// Capture Value 3 Register
    pub SM2CVAL3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SM2CVAL3CYC: RORegister<u16>,

    /// Capture Value 4 Register
    pub SM2CVAL4: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SM2CVAL4CYC: RORegister<u16>,

    /// Capture Value 5 Register
    pub SM2CVAL5: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SM2CVAL5CYC: RORegister<u16>,

    _reserved6: [u32; 2],

    /// Counter Register
    pub SM3CNT: RORegister<u16>,

    /// Initial Count Register
    pub SM3INIT: RWRegister<u16>,

    /// Control 2 Register
    pub SM3CTRL2: RWRegister<u16>,

    /// Control Register
    pub SM3CTRL: RWRegister<u16>,

    _reserved7: [u16; 1],

    /// Value Register 0
    pub SM3VAL0: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SM3FRACVAL1: RWRegister<u16>,

    /// Value Register 1
    pub SM3VAL1: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SM3FRACVAL2: RWRegister<u16>,

    /// Value Register 2
    pub SM3VAL2: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SM3FRACVAL3: RWRegister<u16>,

    /// Value Register 3
    pub SM3VAL3: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SM3FRACVAL4: RWRegister<u16>,

    /// Value Register 4
    pub SM3VAL4: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SM3FRACVAL5: RWRegister<u16>,

    /// Value Register 5
    pub SM3VAL5: RWRegister<u16>,

    /// Fractional Control Register
    pub SM3FRCTRL: RWRegister<u16>,

    /// Output Control Register
    pub SM3OCTRL: RWRegister<u16>,

    /// Status Register
    pub SM3STS: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SM3INTEN: RWRegister<u16>,

    /// DMA Enable Register
    pub SM3DMAEN: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SM3TCTRL: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SM3DISMAP0: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SM3DISMAP1: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SM3DTCNT0: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SM3DTCNT1: RWRegister<u16>,

    /// Capture Control A Register
    pub SM3CAPTCTRLA: RWRegister<u16>,

    /// Capture Compare A Register
    pub SM3CAPTCOMPA: RWRegister<u16>,

    /// Capture Control B Register
    pub SM3CAPTCTRLB: RWRegister<u16>,

    /// Capture Compare B Register
    pub SM3CAPTCOMPB: RWRegister<u16>,

    /// Capture Control X Register
    pub SM3CAPTCTRLX: RWRegister<u16>,

    /// Capture Compare X Register
    pub SM3CAPTCOMPX: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SM3CVAL0: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SM3CVAL0CYC: RORegister<u16>,

    /// Capture Value 1 Register
    pub SM3CVAL1: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SM3CVAL1CYC: RORegister<u16>,

    /// Capture Value 2 Register
    pub SM3CVAL2: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SM3CVAL2CYC: RORegister<u16>,

    /// Capture Value 3 Register
    pub SM3CVAL3: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SM3CVAL3CYC: RORegister<u16>,

    /// Capture Value 4 Register
    pub SM3CVAL4: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SM3CVAL4CYC: RORegister<u16>,

    /// Capture Value 5 Register
    pub SM3CVAL5: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SM3CVAL5CYC: RORegister<u16>,

    _reserved8: [u32; 2],

    /// Output Enable Register
    pub OUTEN: RWRegister<u16>,

    /// Mask Register
    pub MASK: RWRegister<u16>,

    /// Software Controlled Output Register
    pub SWCOUT: RWRegister<u16>,

    /// PWM Source Select Register
    pub DTSRCSEL: RWRegister<u16>,

    /// Master Control Register
    pub MCTRL: RWRegister<u16>,

    /// Master Control 2 Register
    pub MCTRL2: RWRegister<u16>,

    /// Fault Control Register
    pub FCTRL0: RWRegister<u16>,

    /// Fault Status Register
    pub FSTS0: RWRegister<u16>,

    /// Fault Filter Register
    pub FFILT0: RWRegister<u16>,

    /// Fault Test Register
    pub FTST0: RWRegister<u16>,

    /// Fault Control 2 Register
    pub FCTRL20: RWRegister<u16>,
}
pub struct ResetValues {
    pub SM0CNT: u16,
    pub SM0INIT: u16,
    pub SM0CTRL2: u16,
    pub SM0CTRL: u16,
    pub SM0VAL0: u16,
    pub SM0FRACVAL1: u16,
    pub SM0VAL1: u16,
    pub SM0FRACVAL2: u16,
    pub SM0VAL2: u16,
    pub SM0FRACVAL3: u16,
    pub SM0VAL3: u16,
    pub SM0FRACVAL4: u16,
    pub SM0VAL4: u16,
    pub SM0FRACVAL5: u16,
    pub SM0VAL5: u16,
    pub SM0FRCTRL: u16,
    pub SM0OCTRL: u16,
    pub SM0STS: u16,
    pub SM0INTEN: u16,
    pub SM0DMAEN: u16,
    pub SM0TCTRL: u16,
    pub SM0DISMAP0: u16,
    pub SM0DISMAP1: u16,
    pub SM0DTCNT0: u16,
    pub SM0DTCNT1: u16,
    pub SM0CAPTCTRLA: u16,
    pub SM0CAPTCOMPA: u16,
    pub SM0CAPTCTRLB: u16,
    pub SM0CAPTCOMPB: u16,
    pub SM0CAPTCTRLX: u16,
    pub SM0CAPTCOMPX: u16,
    pub SM0CVAL0: u16,
    pub SM0CVAL0CYC: u16,
    pub SM0CVAL1: u16,
    pub SM0CVAL1CYC: u16,
    pub SM0CVAL2: u16,
    pub SM0CVAL2CYC: u16,
    pub SM0CVAL3: u16,
    pub SM0CVAL3CYC: u16,
    pub SM0CVAL4: u16,
    pub SM0CVAL4CYC: u16,
    pub SM0CVAL5: u16,
    pub SM0CVAL5CYC: u16,
    pub SM1CNT: u16,
    pub SM1INIT: u16,
    pub SM1CTRL2: u16,
    pub SM1CTRL: u16,
    pub SM1VAL0: u16,
    pub SM1FRACVAL1: u16,
    pub SM1VAL1: u16,
    pub SM1FRACVAL2: u16,
    pub SM1VAL2: u16,
    pub SM1FRACVAL3: u16,
    pub SM1VAL3: u16,
    pub SM1FRACVAL4: u16,
    pub SM1VAL4: u16,
    pub SM1FRACVAL5: u16,
    pub SM1VAL5: u16,
    pub SM1FRCTRL: u16,
    pub SM1OCTRL: u16,
    pub SM1STS: u16,
    pub SM1INTEN: u16,
    pub SM1DMAEN: u16,
    pub SM1TCTRL: u16,
    pub SM1DISMAP0: u16,
    pub SM1DISMAP1: u16,
    pub SM1DTCNT0: u16,
    pub SM1DTCNT1: u16,
    pub SM1CAPTCTRLA: u16,
    pub SM1CAPTCOMPA: u16,
    pub SM1CAPTCTRLB: u16,
    pub SM1CAPTCOMPB: u16,
    pub SM1CAPTCTRLX: u16,
    pub SM1CAPTCOMPX: u16,
    pub SM1CVAL0: u16,
    pub SM1CVAL0CYC: u16,
    pub SM1CVAL1: u16,
    pub SM1CVAL1CYC: u16,
    pub SM1CVAL2: u16,
    pub SM1CVAL2CYC: u16,
    pub SM1CVAL3: u16,
    pub SM1CVAL3CYC: u16,
    pub SM1CVAL4: u16,
    pub SM1CVAL4CYC: u16,
    pub SM1CVAL5: u16,
    pub SM1CVAL5CYC: u16,
    pub SM2CNT: u16,
    pub SM2INIT: u16,
    pub SM2CTRL2: u16,
    pub SM2CTRL: u16,
    pub SM2VAL0: u16,
    pub SM2FRACVAL1: u16,
    pub SM2VAL1: u16,
    pub SM2FRACVAL2: u16,
    pub SM2VAL2: u16,
    pub SM2FRACVAL3: u16,
    pub SM2VAL3: u16,
    pub SM2FRACVAL4: u16,
    pub SM2VAL4: u16,
    pub SM2FRACVAL5: u16,
    pub SM2VAL5: u16,
    pub SM2FRCTRL: u16,
    pub SM2OCTRL: u16,
    pub SM2STS: u16,
    pub SM2INTEN: u16,
    pub SM2DMAEN: u16,
    pub SM2TCTRL: u16,
    pub SM2DISMAP0: u16,
    pub SM2DISMAP1: u16,
    pub SM2DTCNT0: u16,
    pub SM2DTCNT1: u16,
    pub SM2CAPTCTRLA: u16,
    pub SM2CAPTCOMPA: u16,
    pub SM2CAPTCTRLB: u16,
    pub SM2CAPTCOMPB: u16,
    pub SM2CAPTCTRLX: u16,
    pub SM2CAPTCOMPX: u16,
    pub SM2CVAL0: u16,
    pub SM2CVAL0CYC: u16,
    pub SM2CVAL1: u16,
    pub SM2CVAL1CYC: u16,
    pub SM2CVAL2: u16,
    pub SM2CVAL2CYC: u16,
    pub SM2CVAL3: u16,
    pub SM2CVAL3CYC: u16,
    pub SM2CVAL4: u16,
    pub SM2CVAL4CYC: u16,
    pub SM2CVAL5: u16,
    pub SM2CVAL5CYC: u16,
    pub SM3CNT: u16,
    pub SM3INIT: u16,
    pub SM3CTRL2: u16,
    pub SM3CTRL: u16,
    pub SM3VAL0: u16,
    pub SM3FRACVAL1: u16,
    pub SM3VAL1: u16,
    pub SM3FRACVAL2: u16,
    pub SM3VAL2: u16,
    pub SM3FRACVAL3: u16,
    pub SM3VAL3: u16,
    pub SM3FRACVAL4: u16,
    pub SM3VAL4: u16,
    pub SM3FRACVAL5: u16,
    pub SM3VAL5: u16,
    pub SM3FRCTRL: u16,
    pub SM3OCTRL: u16,
    pub SM3STS: u16,
    pub SM3INTEN: u16,
    pub SM3DMAEN: u16,
    pub SM3TCTRL: u16,
    pub SM3DISMAP0: u16,
    pub SM3DISMAP1: u16,
    pub SM3DTCNT0: u16,
    pub SM3DTCNT1: u16,
    pub SM3CAPTCTRLA: u16,
    pub SM3CAPTCOMPA: u16,
    pub SM3CAPTCTRLB: u16,
    pub SM3CAPTCOMPB: u16,
    pub SM3CAPTCTRLX: u16,
    pub SM3CAPTCOMPX: u16,
    pub SM3CVAL0: u16,
    pub SM3CVAL0CYC: u16,
    pub SM3CVAL1: u16,
    pub SM3CVAL1CYC: u16,
    pub SM3CVAL2: u16,
    pub SM3CVAL2CYC: u16,
    pub SM3CVAL3: u16,
    pub SM3CVAL3CYC: u16,
    pub SM3CVAL4: u16,
    pub SM3CVAL4CYC: u16,
    pub SM3CVAL5: u16,
    pub SM3CVAL5CYC: u16,
    pub OUTEN: u16,
    pub MASK: u16,
    pub SWCOUT: u16,
    pub DTSRCSEL: u16,
    pub MCTRL: u16,
    pub MCTRL2: u16,
    pub FCTRL0: u16,
    pub FSTS0: u16,
    pub FFILT0: u16,
    pub FTST0: u16,
    pub FCTRL20: u16,
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

/// Access functions for the PWM1 peripheral instance
pub mod PWM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403dc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWM1
    pub const reset: ResetValues = ResetValues {
        SM0CNT: 0x00000000,
        SM0INIT: 0x00000000,
        SM0CTRL2: 0x00000000,
        SM0CTRL: 0x00000400,
        SM0VAL0: 0x00000000,
        SM0FRACVAL1: 0x00000000,
        SM0VAL1: 0x00000000,
        SM0FRACVAL2: 0x00000000,
        SM0VAL2: 0x00000000,
        SM0FRACVAL3: 0x00000000,
        SM0VAL3: 0x00000000,
        SM0FRACVAL4: 0x00000000,
        SM0VAL4: 0x00000000,
        SM0FRACVAL5: 0x00000000,
        SM0VAL5: 0x00000000,
        SM0FRCTRL: 0x00000000,
        SM0OCTRL: 0x00000000,
        SM0STS: 0x00000000,
        SM0INTEN: 0x00000000,
        SM0DMAEN: 0x00000000,
        SM0TCTRL: 0x00000000,
        SM0DISMAP0: 0x0000FFFF,
        SM0DISMAP1: 0x0000FFFF,
        SM0DTCNT0: 0x000007FF,
        SM0DTCNT1: 0x000007FF,
        SM0CAPTCTRLA: 0x00000000,
        SM0CAPTCOMPA: 0x00000000,
        SM0CAPTCTRLB: 0x00000000,
        SM0CAPTCOMPB: 0x00000000,
        SM0CAPTCTRLX: 0x00000000,
        SM0CAPTCOMPX: 0x00000000,
        SM0CVAL0: 0x00000000,
        SM0CVAL0CYC: 0x00000000,
        SM0CVAL1: 0x00000000,
        SM0CVAL1CYC: 0x00000000,
        SM0CVAL2: 0x00000000,
        SM0CVAL2CYC: 0x00000000,
        SM0CVAL3: 0x00000000,
        SM0CVAL3CYC: 0x00000000,
        SM0CVAL4: 0x00000000,
        SM0CVAL4CYC: 0x00000000,
        SM0CVAL5: 0x00000000,
        SM0CVAL5CYC: 0x00000000,
        SM1CNT: 0x00000000,
        SM1INIT: 0x00000000,
        SM1CTRL2: 0x00000000,
        SM1CTRL: 0x00000400,
        SM1VAL0: 0x00000000,
        SM1FRACVAL1: 0x00000000,
        SM1VAL1: 0x00000000,
        SM1FRACVAL2: 0x00000000,
        SM1VAL2: 0x00000000,
        SM1FRACVAL3: 0x00000000,
        SM1VAL3: 0x00000000,
        SM1FRACVAL4: 0x00000000,
        SM1VAL4: 0x00000000,
        SM1FRACVAL5: 0x00000000,
        SM1VAL5: 0x00000000,
        SM1FRCTRL: 0x00000000,
        SM1OCTRL: 0x00000000,
        SM1STS: 0x00000000,
        SM1INTEN: 0x00000000,
        SM1DMAEN: 0x00000000,
        SM1TCTRL: 0x00000000,
        SM1DISMAP0: 0x0000FFFF,
        SM1DISMAP1: 0x0000FFFF,
        SM1DTCNT0: 0x000007FF,
        SM1DTCNT1: 0x000007FF,
        SM1CAPTCTRLA: 0x00000000,
        SM1CAPTCOMPA: 0x00000000,
        SM1CAPTCTRLB: 0x00000000,
        SM1CAPTCOMPB: 0x00000000,
        SM1CAPTCTRLX: 0x00000000,
        SM1CAPTCOMPX: 0x00000000,
        SM1CVAL0: 0x00000000,
        SM1CVAL0CYC: 0x00000000,
        SM1CVAL1: 0x00000000,
        SM1CVAL1CYC: 0x00000000,
        SM1CVAL2: 0x00000000,
        SM1CVAL2CYC: 0x00000000,
        SM1CVAL3: 0x00000000,
        SM1CVAL3CYC: 0x00000000,
        SM1CVAL4: 0x00000000,
        SM1CVAL4CYC: 0x00000000,
        SM1CVAL5: 0x00000000,
        SM1CVAL5CYC: 0x00000000,
        SM2CNT: 0x00000000,
        SM2INIT: 0x00000000,
        SM2CTRL2: 0x00000000,
        SM2CTRL: 0x00000400,
        SM2VAL0: 0x00000000,
        SM2FRACVAL1: 0x00000000,
        SM2VAL1: 0x00000000,
        SM2FRACVAL2: 0x00000000,
        SM2VAL2: 0x00000000,
        SM2FRACVAL3: 0x00000000,
        SM2VAL3: 0x00000000,
        SM2FRACVAL4: 0x00000000,
        SM2VAL4: 0x00000000,
        SM2FRACVAL5: 0x00000000,
        SM2VAL5: 0x00000000,
        SM2FRCTRL: 0x00000000,
        SM2OCTRL: 0x00000000,
        SM2STS: 0x00000000,
        SM2INTEN: 0x00000000,
        SM2DMAEN: 0x00000000,
        SM2TCTRL: 0x00000000,
        SM2DISMAP0: 0x0000FFFF,
        SM2DISMAP1: 0x0000FFFF,
        SM2DTCNT0: 0x000007FF,
        SM2DTCNT1: 0x000007FF,
        SM2CAPTCTRLA: 0x00000000,
        SM2CAPTCOMPA: 0x00000000,
        SM2CAPTCTRLB: 0x00000000,
        SM2CAPTCOMPB: 0x00000000,
        SM2CAPTCTRLX: 0x00000000,
        SM2CAPTCOMPX: 0x00000000,
        SM2CVAL0: 0x00000000,
        SM2CVAL0CYC: 0x00000000,
        SM2CVAL1: 0x00000000,
        SM2CVAL1CYC: 0x00000000,
        SM2CVAL2: 0x00000000,
        SM2CVAL2CYC: 0x00000000,
        SM2CVAL3: 0x00000000,
        SM2CVAL3CYC: 0x00000000,
        SM2CVAL4: 0x00000000,
        SM2CVAL4CYC: 0x00000000,
        SM2CVAL5: 0x00000000,
        SM2CVAL5CYC: 0x00000000,
        SM3CNT: 0x00000000,
        SM3INIT: 0x00000000,
        SM3CTRL2: 0x00000000,
        SM3CTRL: 0x00000400,
        SM3VAL0: 0x00000000,
        SM3FRACVAL1: 0x00000000,
        SM3VAL1: 0x00000000,
        SM3FRACVAL2: 0x00000000,
        SM3VAL2: 0x00000000,
        SM3FRACVAL3: 0x00000000,
        SM3VAL3: 0x00000000,
        SM3FRACVAL4: 0x00000000,
        SM3VAL4: 0x00000000,
        SM3FRACVAL5: 0x00000000,
        SM3VAL5: 0x00000000,
        SM3FRCTRL: 0x00000000,
        SM3OCTRL: 0x00000000,
        SM3STS: 0x00000000,
        SM3INTEN: 0x00000000,
        SM3DMAEN: 0x00000000,
        SM3TCTRL: 0x00000000,
        SM3DISMAP0: 0x0000FFFF,
        SM3DISMAP1: 0x0000FFFF,
        SM3DTCNT0: 0x000007FF,
        SM3DTCNT1: 0x000007FF,
        SM3CAPTCTRLA: 0x00000000,
        SM3CAPTCOMPA: 0x00000000,
        SM3CAPTCTRLB: 0x00000000,
        SM3CAPTCOMPB: 0x00000000,
        SM3CAPTCTRLX: 0x00000000,
        SM3CAPTCOMPX: 0x00000000,
        SM3CVAL0: 0x00000000,
        SM3CVAL0CYC: 0x00000000,
        SM3CVAL1: 0x00000000,
        SM3CVAL1CYC: 0x00000000,
        SM3CVAL2: 0x00000000,
        SM3CVAL2CYC: 0x00000000,
        SM3CVAL3: 0x00000000,
        SM3CVAL3CYC: 0x00000000,
        SM3CVAL4: 0x00000000,
        SM3CVAL4CYC: 0x00000000,
        SM3CVAL5: 0x00000000,
        SM3CVAL5CYC: 0x00000000,
        OUTEN: 0x00000000,
        MASK: 0x00000000,
        SWCOUT: 0x00000000,
        DTSRCSEL: 0x00000000,
        MCTRL: 0x00000000,
        MCTRL2: 0x00000000,
        FCTRL0: 0x00000000,
        FSTS0: 0x00000000,
        FFILT0: 0x00000000,
        FTST0: 0x00000000,
        FCTRL20: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWM1_TAKEN: bool = false;

    /// Safe access to PWM1
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
            if PWM1_TAKEN {
                None
            } else {
                PWM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWM1_TAKEN && inst.addr == INSTANCE.addr {
                PWM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWM1: *const RegisterBlock = 0x403dc000 as *const _;
