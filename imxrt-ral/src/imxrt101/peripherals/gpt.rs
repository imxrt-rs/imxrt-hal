#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPT
//!
//! Used by: imxrt1011, imxrt1015

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPT Control Register
pub mod CR {

    /// GPT Enable
    pub mod EN {
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

            /// 0b0: GPT is disabled.
            pub const EN_0: u32 = 0b0;

            /// 0b1: GPT is enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// GPT Enable mode
    pub mod ENMOD {
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

            /// 0b0: GPT counter will retain its value when it is disabled.
            pub const ENMOD_0: u32 = 0b0;

            /// 0b1: GPT counter value is reset to 0 when it is disabled.
            pub const ENMOD_1: u32 = 0b1;
        }
    }

    /// GPT debug mode enable
    pub mod DBGEN {
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

            /// 0b0: GPT is disabled in debug mode.
            pub const DBGEN_0: u32 = 0b0;

            /// 0b1: GPT is enabled in debug mode.
            pub const DBGEN_1: u32 = 0b1;
        }
    }

    /// GPT Wait Mode enable
    pub mod WAITEN {
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

            /// 0b0: GPT is disabled in wait mode.
            pub const WAITEN_0: u32 = 0b0;

            /// 0b1: GPT is enabled in wait mode.
            pub const WAITEN_1: u32 = 0b1;
        }
    }

    /// GPT Doze Mode Enable
    pub mod DOZEEN {
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

            /// 0b0: GPT is disabled in doze mode.
            pub const DOZEEN_0: u32 = 0b0;

            /// 0b1: GPT is enabled in doze mode.
            pub const DOZEEN_1: u32 = 0b1;
        }
    }

    /// GPT Stop Mode enable
    pub mod STOPEN {
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

            /// 0b0: GPT is disabled in Stop mode.
            pub const STOPEN_0: u32 = 0b0;

            /// 0b1: GPT is enabled in Stop mode.
            pub const STOPEN_1: u32 = 0b1;
        }
    }

    /// Clock Source select
    pub mod CLKSRC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No clock
            pub const CLKSRC_0: u32 = 0b000;

            /// 0b001: Peripheral Clock (ipg_clk)
            pub const CLKSRC_1: u32 = 0b001;

            /// 0b010: High Frequency Reference Clock (ipg_clk_highfreq)
            pub const CLKSRC_2: u32 = 0b010;

            /// 0b011: External Clock
            pub const CLKSRC_3: u32 = 0b011;

            /// 0b100: Low Frequency Reference Clock (ipg_clk_32k)
            pub const CLKSRC_4: u32 = 0b100;

            /// 0b101: Crystal oscillator as Reference Clock (ipg_clk_24M)
            pub const CLKSRC_5: u32 = 0b101;
        }
    }

    /// Free-Run or Restart mode
    pub mod FRR {
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

            /// 0b0: Restart mode
            pub const FRR_0: u32 = 0b0;

            /// 0b1: Free-Run mode
            pub const FRR_1: u32 = 0b1;
        }
    }

    /// Enable 24 MHz clock input from crystal
    pub mod EN_24M {
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

            /// 0b0: 24M clock disabled
            pub const EN_24M_0: u32 = 0b0;

            /// 0b1: 24M clock enabled
            pub const EN_24M_1: u32 = 0b1;
        }
    }

    /// Software reset
    pub mod SWR {
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

            /// 0b0: GPT is not in reset state
            pub const SWR_0: u32 = 0b0;

            /// 0b1: GPT is in reset state
            pub const SWR_1: u32 = 0b1;
        }
    }

    /// See IM2
    pub mod IM1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event
    pub mod IM2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: capture disabled
            pub const IM2_0: u32 = 0b00;

            /// 0b01: capture on rising edge only
            pub const IM2_1: u32 = 0b01;

            /// 0b10: capture on falling edge only
            pub const IM2_2: u32 = 0b10;

            /// 0b11: capture on both edges
            pub const IM2_3: u32 = 0b11;
        }
    }

    /// See OM3
    pub mod OM1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// See OM3
    pub mod OM2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (3 bits: 0b111 << 23)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode
    pub mod OM3 {
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

            /// 0b000: Output disconnected. No response on pin.
            pub const OM3_0: u32 = 0b000;

            /// 0b001: Toggle output pin
            pub const OM3_1: u32 = 0b001;

            /// 0b010: Clear output pin
            pub const OM3_2: u32 = 0b010;

            /// 0b011: Set output pin
            pub const OM3_3: u32 = 0b011;

            /// 0b000: Generate an active low pulse (that is one input clock wide) on the output pin.
            pub const OM3_4: u32 = 0b000;
        }
    }

    /// See F03
    pub mod FO1 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// See F03
    pub mod FO2 {
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

    /// FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)
    pub mod FO3 {
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

            /// 0b0: Writing a 0 has no effect.
            pub const FO3_0: u32 = 0b0;

            /// 0b1: Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set.
            pub const FO3_1: u32 = 0b1;
        }
    }
}

/// GPT Prescaler Register
pub mod PR {

    /// Prescaler bits
    pub mod PRESCALER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000000: Divide by 1
            pub const PRESCALER_0: u32 = 0b000000000000;

            /// 0b000000000001: Divide by 2
            pub const PRESCALER_1: u32 = 0b000000000001;

            /// 0b111111111111: Divide by 4096
            pub const PRESCALER_4095: u32 = 0b111111111111;
        }
    }

    /// Prescaler bits
    pub mod PRESCALER24M {
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

            /// 0b0000: Divide by 1
            pub const PRESCALER24M_0: u32 = 0b0000;

            /// 0b0001: Divide by 2
            pub const PRESCALER24M_1: u32 = 0b0001;

            /// 0b1111: Divide by 16
            pub const PRESCALER24M_15: u32 = 0b1111;
        }
    }
}

/// GPT Status Register
pub mod SR {

    /// See OF3
    pub mod OF1 {
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

    /// See OF3
    pub mod OF2 {
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

    /// OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n
    pub mod OF3 {
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

            /// 0b0: Compare event has not occurred.
            pub const OF3_0: u32 = 0b0;

            /// 0b1: Compare event has occurred.
            pub const OF3_1: u32 = 0b1;
        }
    }

    /// See IF2
    pub mod IF1 {
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

    /// IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n
    pub mod IF2 {
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

            /// 0b0: Capture event has not occurred.
            pub const IF2_0: u32 = 0b0;

            /// 0b1: Capture event has occurred.
            pub const IF2_1: u32 = 0b1;
        }
    }

    /// Rollover Flag
    pub mod ROV {
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

            /// 0b0: Rollover has not occurred.
            pub const ROV_0: u32 = 0b0;

            /// 0b1: Rollover has occurred.
            pub const ROV_1: u32 = 0b1;
        }
    }
}

/// GPT Interrupt Register
pub mod IR {

    /// See OF3IE
    pub mod OF1IE {
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

    /// See OF3IE
    pub mod OF2IE {
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

    /// OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt
    pub mod OF3IE {
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

            /// 0b0: Output Compare Channel n interrupt is disabled.
            pub const OF3IE_0: u32 = 0b0;

            /// 0b1: Output Compare Channel n interrupt is enabled.
            pub const OF3IE_1: u32 = 0b1;
        }
    }

    /// See IF2IE
    pub mod IF1IE {
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

    /// IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable
    pub mod IF2IE {
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

            /// 0b0: IF2IE Input Capture n Interrupt Enable is disabled.
            pub const IF2IE_0: u32 = 0b0;

            /// 0b1: IF2IE Input Capture n Interrupt Enable is enabled.
            pub const IF2IE_1: u32 = 0b1;
        }
    }

    /// Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt.
    pub mod ROVIE {
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

            /// 0b0: Rollover interrupt is disabled.
            pub const ROVIE_0: u32 = 0b0;

            /// 0b1: Rollover interrupt enabled.
            pub const ROVIE_1: u32 = 0b1;
        }
    }
}

/// GPT Output Compare Register 1
pub mod OCR1 {

    /// Compare Value
    pub mod COMP {
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

/// GPT Output Compare Register 2
pub mod OCR2 {
    pub use super::OCR1::COMP;
}

/// GPT Output Compare Register 3
pub mod OCR3 {
    pub use super::OCR1::COMP;
}

/// GPT Input Capture Register 1
pub mod ICR1 {

    /// Capture Value
    pub mod CAPT {
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

/// GPT Input Capture Register 2
pub mod ICR2 {
    pub use super::ICR1::CAPT;
}

/// GPT Counter Register
pub mod CNT {

    /// Counter Value. The COUNT bits show the current count value of the GPT counter.
    pub mod COUNT {
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
pub struct RegisterBlock {
    /// GPT Control Register
    pub CR: RWRegister<u32>,

    /// GPT Prescaler Register
    pub PR: RWRegister<u32>,

    /// GPT Status Register
    pub SR: RWRegister<u32>,

    /// GPT Interrupt Register
    pub IR: RWRegister<u32>,

    /// GPT Output Compare Register 1
    pub OCR1: RWRegister<u32>,

    /// GPT Output Compare Register 2
    pub OCR2: RWRegister<u32>,

    /// GPT Output Compare Register 3
    pub OCR3: RWRegister<u32>,

    /// GPT Input Capture Register 1
    pub ICR1: RORegister<u32>,

    /// GPT Input Capture Register 2
    pub ICR2: RORegister<u32>,

    /// GPT Counter Register
    pub CNT: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub PR: u32,
    pub SR: u32,
    pub IR: u32,
    pub OCR1: u32,
    pub OCR2: u32,
    pub OCR3: u32,
    pub ICR1: u32,
    pub ICR2: u32,
    pub CNT: u32,
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
