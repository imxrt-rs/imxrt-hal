#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PWM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

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

/// Counter Register
pub mod SMCNT0 {

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
pub mod SMINIT0 {

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
pub mod SMCTRL20 {

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
pub mod SMCTRL0 {

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
pub mod SMVAL00 {

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
pub mod SMFRACVAL10 {

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
pub mod SMVAL10 {

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
pub mod SMFRACVAL20 {

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
pub mod SMVAL20 {

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
pub mod SMFRACVAL30 {

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
pub mod SMVAL30 {

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
pub mod SMFRACVAL40 {

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
pub mod SMVAL40 {

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
pub mod SMFRACVAL50 {

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
pub mod SMVAL50 {

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
pub mod SMFRCTRL0 {

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
pub mod SMOCTRL0 {

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
pub mod SMSTS0 {

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
pub mod SMINTEN0 {

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
pub mod SMDMAEN0 {

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
pub mod SMTCTRL0 {

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
pub mod SMDISMAP00 {

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
pub mod SMDISMAP10 {

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
pub mod SMDTCNT00 {

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
pub mod SMDTCNT10 {

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
pub mod SMCAPTCTRLA0 {

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
pub mod SMCAPTCOMPA0 {

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
pub mod SMCAPTCTRLB0 {

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
pub mod SMCAPTCOMPB0 {

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
pub mod SMCAPTCTRLX0 {

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
pub mod SMCAPTCOMPX0 {

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
pub mod SMCVAL00 {

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
pub mod SMCVAL0CYC0 {

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
pub mod SMCVAL10 {

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
pub mod SMCVAL1CYC0 {

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
pub mod SMCVAL20 {

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
pub mod SMCVAL2CYC0 {

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
pub mod SMCVAL30 {

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
pub mod SMCVAL3CYC0 {

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
pub mod SMCVAL40 {

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
pub mod SMCVAL4CYC0 {

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
pub mod SMCVAL50 {

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
pub mod SMCVAL5CYC0 {

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
pub mod SMCNT1 {
    pub use super::SMCNT0::CNT;
}

/// Initial Count Register
pub mod SMINIT1 {
    pub use super::SMINIT0::INIT;
}

/// Control 2 Register
pub mod SMCTRL21 {
    pub use super::SMCTRL20::CLK_SEL;
    pub use super::SMCTRL20::DBGEN;
    pub use super::SMCTRL20::FORCE;
    pub use super::SMCTRL20::FORCE_SEL;
    pub use super::SMCTRL20::FRCEN;
    pub use super::SMCTRL20::INDEP;
    pub use super::SMCTRL20::INIT_SEL;
    pub use super::SMCTRL20::PWM23_INIT;
    pub use super::SMCTRL20::PWM45_INIT;
    pub use super::SMCTRL20::PWMX_INIT;
    pub use super::SMCTRL20::RELOAD_SEL;
    pub use super::SMCTRL20::WAITEN;
}

/// Control Register
pub mod SMCTRL1 {
    pub use super::SMCTRL0::COMPMODE;
    pub use super::SMCTRL0::DBLEN;
    pub use super::SMCTRL0::DBLX;
    pub use super::SMCTRL0::DT;
    pub use super::SMCTRL0::FULL;
    pub use super::SMCTRL0::HALF;
    pub use super::SMCTRL0::LDFQ;
    pub use super::SMCTRL0::LDMOD;
    pub use super::SMCTRL0::PRSC;
    pub use super::SMCTRL0::SPLIT;
}

/// Value Register 0
pub mod SMVAL01 {
    pub use super::SMVAL00::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL11 {
    pub use super::SMFRACVAL10::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL11 {
    pub use super::SMVAL10::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL21 {
    pub use super::SMFRACVAL20::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL21 {
    pub use super::SMVAL20::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL31 {
    pub use super::SMFRACVAL30::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL31 {
    pub use super::SMVAL30::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL41 {
    pub use super::SMFRACVAL40::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL41 {
    pub use super::SMVAL40::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL51 {
    pub use super::SMFRACVAL50::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL51 {
    pub use super::SMVAL50::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL1 {
    pub use super::SMFRCTRL0::FRAC1_EN;
    pub use super::SMFRCTRL0::FRAC23_EN;
    pub use super::SMFRCTRL0::FRAC45_EN;
    pub use super::SMFRCTRL0::FRAC_PU;
    pub use super::SMFRCTRL0::TEST;
}

/// Output Control Register
pub mod SMOCTRL1 {
    pub use super::SMOCTRL0::POLA;
    pub use super::SMOCTRL0::POLB;
    pub use super::SMOCTRL0::POLX;
    pub use super::SMOCTRL0::PWMAFS;
    pub use super::SMOCTRL0::PWMA_IN;
    pub use super::SMOCTRL0::PWMBFS;
    pub use super::SMOCTRL0::PWMB_IN;
    pub use super::SMOCTRL0::PWMXFS;
    pub use super::SMOCTRL0::PWMX_IN;
}

/// Status Register
pub mod SMSTS1 {
    pub use super::SMSTS0::CFA0;
    pub use super::SMSTS0::CFA1;
    pub use super::SMSTS0::CFB0;
    pub use super::SMSTS0::CFB1;
    pub use super::SMSTS0::CFX0;
    pub use super::SMSTS0::CFX1;
    pub use super::SMSTS0::CMPF;
    pub use super::SMSTS0::REF;
    pub use super::SMSTS0::RF;
    pub use super::SMSTS0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN1 {
    pub use super::SMINTEN0::CA0IE;
    pub use super::SMINTEN0::CA1IE;
    pub use super::SMINTEN0::CB0IE;
    pub use super::SMINTEN0::CB1IE;
    pub use super::SMINTEN0::CMPIE;
    pub use super::SMINTEN0::CX0IE;
    pub use super::SMINTEN0::CX1IE;
    pub use super::SMINTEN0::REIE;
    pub use super::SMINTEN0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN1 {
    pub use super::SMDMAEN0::CA0DE;
    pub use super::SMDMAEN0::CA1DE;
    pub use super::SMDMAEN0::CAPTDE;
    pub use super::SMDMAEN0::CB0DE;
    pub use super::SMDMAEN0::CB1DE;
    pub use super::SMDMAEN0::CX0DE;
    pub use super::SMDMAEN0::CX1DE;
    pub use super::SMDMAEN0::FAND;
    pub use super::SMDMAEN0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL1 {
    pub use super::SMTCTRL0::OUT_TRIG_EN;
    pub use super::SMTCTRL0::PWAOT0;
    pub use super::SMTCTRL0::PWBOT1;
    pub use super::SMTCTRL0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP01 {
    pub use super::SMDISMAP00::DIS0A;
    pub use super::SMDISMAP00::DIS0B;
    pub use super::SMDISMAP00::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP11 {
    pub use super::SMDISMAP10::DIS1A;
    pub use super::SMDISMAP10::DIS1B;
    pub use super::SMDISMAP10::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT01 {
    pub use super::SMDTCNT00::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT11 {
    pub use super::SMDTCNT10::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA1 {
    pub use super::SMCAPTCTRLA0::ARMA;
    pub use super::SMCAPTCTRLA0::CA0CNT;
    pub use super::SMCAPTCTRLA0::CA1CNT;
    pub use super::SMCAPTCTRLA0::CFAWM;
    pub use super::SMCAPTCTRLA0::EDGA0;
    pub use super::SMCAPTCTRLA0::EDGA1;
    pub use super::SMCAPTCTRLA0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA0::INP_SELA;
    pub use super::SMCAPTCTRLA0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA1 {
    pub use super::SMCAPTCOMPA0::EDGCMPA;
    pub use super::SMCAPTCOMPA0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB1 {
    pub use super::SMCAPTCTRLB0::ARMB;
    pub use super::SMCAPTCTRLB0::CB0CNT;
    pub use super::SMCAPTCTRLB0::CB1CNT;
    pub use super::SMCAPTCTRLB0::CFBWM;
    pub use super::SMCAPTCTRLB0::EDGB0;
    pub use super::SMCAPTCTRLB0::EDGB1;
    pub use super::SMCAPTCTRLB0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB0::INP_SELB;
    pub use super::SMCAPTCTRLB0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB1 {
    pub use super::SMCAPTCOMPB0::EDGCMPB;
    pub use super::SMCAPTCOMPB0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX1 {
    pub use super::SMCAPTCTRLX0::ARMX;
    pub use super::SMCAPTCTRLX0::CFXWM;
    pub use super::SMCAPTCTRLX0::CX0CNT;
    pub use super::SMCAPTCTRLX0::CX1CNT;
    pub use super::SMCAPTCTRLX0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX0::EDGX0;
    pub use super::SMCAPTCTRLX0::EDGX1;
    pub use super::SMCAPTCTRLX0::INP_SELX;
    pub use super::SMCAPTCTRLX0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX1 {
    pub use super::SMCAPTCOMPX0::EDGCMPX;
    pub use super::SMCAPTCOMPX0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL01 {
    pub use super::SMCVAL00::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC1 {
    pub use super::SMCVAL0CYC0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL11 {
    pub use super::SMCVAL10::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC1 {
    pub use super::SMCVAL1CYC0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL21 {
    pub use super::SMCVAL20::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC1 {
    pub use super::SMCVAL2CYC0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL31 {
    pub use super::SMCVAL30::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC1 {
    pub use super::SMCVAL3CYC0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL41 {
    pub use super::SMCVAL40::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC1 {
    pub use super::SMCVAL4CYC0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL51 {
    pub use super::SMCVAL50::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC1 {
    pub use super::SMCVAL5CYC0::CVAL5CYC;
}

/// Counter Register
pub mod SMCNT2 {
    pub use super::SMCNT0::CNT;
}

/// Initial Count Register
pub mod SMINIT2 {
    pub use super::SMINIT0::INIT;
}

/// Control 2 Register
pub mod SMCTRL22 {
    pub use super::SMCTRL20::CLK_SEL;
    pub use super::SMCTRL20::DBGEN;
    pub use super::SMCTRL20::FORCE;
    pub use super::SMCTRL20::FORCE_SEL;
    pub use super::SMCTRL20::FRCEN;
    pub use super::SMCTRL20::INDEP;
    pub use super::SMCTRL20::INIT_SEL;
    pub use super::SMCTRL20::PWM23_INIT;
    pub use super::SMCTRL20::PWM45_INIT;
    pub use super::SMCTRL20::PWMX_INIT;
    pub use super::SMCTRL20::RELOAD_SEL;
    pub use super::SMCTRL20::WAITEN;
}

/// Control Register
pub mod SMCTRL2 {
    pub use super::SMCTRL0::COMPMODE;
    pub use super::SMCTRL0::DBLEN;
    pub use super::SMCTRL0::DBLX;
    pub use super::SMCTRL0::DT;
    pub use super::SMCTRL0::FULL;
    pub use super::SMCTRL0::HALF;
    pub use super::SMCTRL0::LDFQ;
    pub use super::SMCTRL0::LDMOD;
    pub use super::SMCTRL0::PRSC;
    pub use super::SMCTRL0::SPLIT;
}

/// Value Register 0
pub mod SMVAL02 {
    pub use super::SMVAL00::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL12 {
    pub use super::SMFRACVAL10::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL12 {
    pub use super::SMVAL10::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL22 {
    pub use super::SMFRACVAL20::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL22 {
    pub use super::SMVAL20::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL32 {
    pub use super::SMFRACVAL30::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL32 {
    pub use super::SMVAL30::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL42 {
    pub use super::SMFRACVAL40::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL42 {
    pub use super::SMVAL40::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL52 {
    pub use super::SMFRACVAL50::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL52 {
    pub use super::SMVAL50::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL2 {
    pub use super::SMFRCTRL0::FRAC1_EN;
    pub use super::SMFRCTRL0::FRAC23_EN;
    pub use super::SMFRCTRL0::FRAC45_EN;
    pub use super::SMFRCTRL0::FRAC_PU;
    pub use super::SMFRCTRL0::TEST;
}

/// Output Control Register
pub mod SMOCTRL2 {
    pub use super::SMOCTRL0::POLA;
    pub use super::SMOCTRL0::POLB;
    pub use super::SMOCTRL0::POLX;
    pub use super::SMOCTRL0::PWMAFS;
    pub use super::SMOCTRL0::PWMA_IN;
    pub use super::SMOCTRL0::PWMBFS;
    pub use super::SMOCTRL0::PWMB_IN;
    pub use super::SMOCTRL0::PWMXFS;
    pub use super::SMOCTRL0::PWMX_IN;
}

/// Status Register
pub mod SMSTS2 {
    pub use super::SMSTS0::CFA0;
    pub use super::SMSTS0::CFA1;
    pub use super::SMSTS0::CFB0;
    pub use super::SMSTS0::CFB1;
    pub use super::SMSTS0::CFX0;
    pub use super::SMSTS0::CFX1;
    pub use super::SMSTS0::CMPF;
    pub use super::SMSTS0::REF;
    pub use super::SMSTS0::RF;
    pub use super::SMSTS0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN2 {
    pub use super::SMINTEN0::CA0IE;
    pub use super::SMINTEN0::CA1IE;
    pub use super::SMINTEN0::CB0IE;
    pub use super::SMINTEN0::CB1IE;
    pub use super::SMINTEN0::CMPIE;
    pub use super::SMINTEN0::CX0IE;
    pub use super::SMINTEN0::CX1IE;
    pub use super::SMINTEN0::REIE;
    pub use super::SMINTEN0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN2 {
    pub use super::SMDMAEN0::CA0DE;
    pub use super::SMDMAEN0::CA1DE;
    pub use super::SMDMAEN0::CAPTDE;
    pub use super::SMDMAEN0::CB0DE;
    pub use super::SMDMAEN0::CB1DE;
    pub use super::SMDMAEN0::CX0DE;
    pub use super::SMDMAEN0::CX1DE;
    pub use super::SMDMAEN0::FAND;
    pub use super::SMDMAEN0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL2 {
    pub use super::SMTCTRL0::OUT_TRIG_EN;
    pub use super::SMTCTRL0::PWAOT0;
    pub use super::SMTCTRL0::PWBOT1;
    pub use super::SMTCTRL0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP02 {
    pub use super::SMDISMAP00::DIS0A;
    pub use super::SMDISMAP00::DIS0B;
    pub use super::SMDISMAP00::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP12 {
    pub use super::SMDISMAP10::DIS1A;
    pub use super::SMDISMAP10::DIS1B;
    pub use super::SMDISMAP10::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT02 {
    pub use super::SMDTCNT00::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT12 {
    pub use super::SMDTCNT10::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA2 {
    pub use super::SMCAPTCTRLA0::ARMA;
    pub use super::SMCAPTCTRLA0::CA0CNT;
    pub use super::SMCAPTCTRLA0::CA1CNT;
    pub use super::SMCAPTCTRLA0::CFAWM;
    pub use super::SMCAPTCTRLA0::EDGA0;
    pub use super::SMCAPTCTRLA0::EDGA1;
    pub use super::SMCAPTCTRLA0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA0::INP_SELA;
    pub use super::SMCAPTCTRLA0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA2 {
    pub use super::SMCAPTCOMPA0::EDGCMPA;
    pub use super::SMCAPTCOMPA0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB2 {
    pub use super::SMCAPTCTRLB0::ARMB;
    pub use super::SMCAPTCTRLB0::CB0CNT;
    pub use super::SMCAPTCTRLB0::CB1CNT;
    pub use super::SMCAPTCTRLB0::CFBWM;
    pub use super::SMCAPTCTRLB0::EDGB0;
    pub use super::SMCAPTCTRLB0::EDGB1;
    pub use super::SMCAPTCTRLB0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB0::INP_SELB;
    pub use super::SMCAPTCTRLB0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB2 {
    pub use super::SMCAPTCOMPB0::EDGCMPB;
    pub use super::SMCAPTCOMPB0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX2 {
    pub use super::SMCAPTCTRLX0::ARMX;
    pub use super::SMCAPTCTRLX0::CFXWM;
    pub use super::SMCAPTCTRLX0::CX0CNT;
    pub use super::SMCAPTCTRLX0::CX1CNT;
    pub use super::SMCAPTCTRLX0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX0::EDGX0;
    pub use super::SMCAPTCTRLX0::EDGX1;
    pub use super::SMCAPTCTRLX0::INP_SELX;
    pub use super::SMCAPTCTRLX0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX2 {
    pub use super::SMCAPTCOMPX0::EDGCMPX;
    pub use super::SMCAPTCOMPX0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL02 {
    pub use super::SMCVAL00::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC2 {
    pub use super::SMCVAL0CYC0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL12 {
    pub use super::SMCVAL10::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC2 {
    pub use super::SMCVAL1CYC0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL22 {
    pub use super::SMCVAL20::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC2 {
    pub use super::SMCVAL2CYC0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL32 {
    pub use super::SMCVAL30::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC2 {
    pub use super::SMCVAL3CYC0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL42 {
    pub use super::SMCVAL40::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC2 {
    pub use super::SMCVAL4CYC0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL52 {
    pub use super::SMCVAL50::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC2 {
    pub use super::SMCVAL5CYC0::CVAL5CYC;
}

/// Counter Register
pub mod SMCNT3 {
    pub use super::SMCNT0::CNT;
}

/// Initial Count Register
pub mod SMINIT3 {
    pub use super::SMINIT0::INIT;
}

/// Control 2 Register
pub mod SMCTRL23 {
    pub use super::SMCTRL20::CLK_SEL;
    pub use super::SMCTRL20::DBGEN;
    pub use super::SMCTRL20::FORCE;
    pub use super::SMCTRL20::FORCE_SEL;
    pub use super::SMCTRL20::FRCEN;
    pub use super::SMCTRL20::INDEP;
    pub use super::SMCTRL20::INIT_SEL;
    pub use super::SMCTRL20::PWM23_INIT;
    pub use super::SMCTRL20::PWM45_INIT;
    pub use super::SMCTRL20::PWMX_INIT;
    pub use super::SMCTRL20::RELOAD_SEL;
    pub use super::SMCTRL20::WAITEN;
}

/// Control Register
pub mod SMCTRL3 {
    pub use super::SMCTRL0::COMPMODE;
    pub use super::SMCTRL0::DBLEN;
    pub use super::SMCTRL0::DBLX;
    pub use super::SMCTRL0::DT;
    pub use super::SMCTRL0::FULL;
    pub use super::SMCTRL0::HALF;
    pub use super::SMCTRL0::LDFQ;
    pub use super::SMCTRL0::LDMOD;
    pub use super::SMCTRL0::PRSC;
    pub use super::SMCTRL0::SPLIT;
}

/// Value Register 0
pub mod SMVAL03 {
    pub use super::SMVAL00::VAL0;
}

/// Fractional Value Register 1
pub mod SMFRACVAL13 {
    pub use super::SMFRACVAL10::FRACVAL1;
}

/// Value Register 1
pub mod SMVAL13 {
    pub use super::SMVAL10::VAL1;
}

/// Fractional Value Register 2
pub mod SMFRACVAL23 {
    pub use super::SMFRACVAL20::FRACVAL2;
}

/// Value Register 2
pub mod SMVAL23 {
    pub use super::SMVAL20::VAL2;
}

/// Fractional Value Register 3
pub mod SMFRACVAL33 {
    pub use super::SMFRACVAL30::FRACVAL3;
}

/// Value Register 3
pub mod SMVAL33 {
    pub use super::SMVAL30::VAL3;
}

/// Fractional Value Register 4
pub mod SMFRACVAL43 {
    pub use super::SMFRACVAL40::FRACVAL4;
}

/// Value Register 4
pub mod SMVAL43 {
    pub use super::SMVAL40::VAL4;
}

/// Fractional Value Register 5
pub mod SMFRACVAL53 {
    pub use super::SMFRACVAL50::FRACVAL5;
}

/// Value Register 5
pub mod SMVAL53 {
    pub use super::SMVAL50::VAL5;
}

/// Fractional Control Register
pub mod SMFRCTRL3 {
    pub use super::SMFRCTRL0::FRAC1_EN;
    pub use super::SMFRCTRL0::FRAC23_EN;
    pub use super::SMFRCTRL0::FRAC45_EN;
    pub use super::SMFRCTRL0::FRAC_PU;
    pub use super::SMFRCTRL0::TEST;
}

/// Output Control Register
pub mod SMOCTRL3 {
    pub use super::SMOCTRL0::POLA;
    pub use super::SMOCTRL0::POLB;
    pub use super::SMOCTRL0::POLX;
    pub use super::SMOCTRL0::PWMAFS;
    pub use super::SMOCTRL0::PWMA_IN;
    pub use super::SMOCTRL0::PWMBFS;
    pub use super::SMOCTRL0::PWMB_IN;
    pub use super::SMOCTRL0::PWMXFS;
    pub use super::SMOCTRL0::PWMX_IN;
}

/// Status Register
pub mod SMSTS3 {
    pub use super::SMSTS0::CFA0;
    pub use super::SMSTS0::CFA1;
    pub use super::SMSTS0::CFB0;
    pub use super::SMSTS0::CFB1;
    pub use super::SMSTS0::CFX0;
    pub use super::SMSTS0::CFX1;
    pub use super::SMSTS0::CMPF;
    pub use super::SMSTS0::REF;
    pub use super::SMSTS0::RF;
    pub use super::SMSTS0::RUF;
}

/// Interrupt Enable Register
pub mod SMINTEN3 {
    pub use super::SMINTEN0::CA0IE;
    pub use super::SMINTEN0::CA1IE;
    pub use super::SMINTEN0::CB0IE;
    pub use super::SMINTEN0::CB1IE;
    pub use super::SMINTEN0::CMPIE;
    pub use super::SMINTEN0::CX0IE;
    pub use super::SMINTEN0::CX1IE;
    pub use super::SMINTEN0::REIE;
    pub use super::SMINTEN0::RIE;
}

/// DMA Enable Register
pub mod SMDMAEN3 {
    pub use super::SMDMAEN0::CA0DE;
    pub use super::SMDMAEN0::CA1DE;
    pub use super::SMDMAEN0::CAPTDE;
    pub use super::SMDMAEN0::CB0DE;
    pub use super::SMDMAEN0::CB1DE;
    pub use super::SMDMAEN0::CX0DE;
    pub use super::SMDMAEN0::CX1DE;
    pub use super::SMDMAEN0::FAND;
    pub use super::SMDMAEN0::VALDE;
}

/// Output Trigger Control Register
pub mod SMTCTRL3 {
    pub use super::SMTCTRL0::OUT_TRIG_EN;
    pub use super::SMTCTRL0::PWAOT0;
    pub use super::SMTCTRL0::PWBOT1;
    pub use super::SMTCTRL0::TRGFRQ;
}

/// Fault Disable Mapping Register 0
pub mod SMDISMAP03 {
    pub use super::SMDISMAP00::DIS0A;
    pub use super::SMDISMAP00::DIS0B;
    pub use super::SMDISMAP00::DIS0X;
}

/// Fault Disable Mapping Register 1
pub mod SMDISMAP13 {
    pub use super::SMDISMAP10::DIS1A;
    pub use super::SMDISMAP10::DIS1B;
    pub use super::SMDISMAP10::DIS1X;
}

/// Deadtime Count Register 0
pub mod SMDTCNT03 {
    pub use super::SMDTCNT00::DTCNT0;
}

/// Deadtime Count Register 1
pub mod SMDTCNT13 {
    pub use super::SMDTCNT10::DTCNT1;
}

/// Capture Control A Register
pub mod SMCAPTCTRLA3 {
    pub use super::SMCAPTCTRLA0::ARMA;
    pub use super::SMCAPTCTRLA0::CA0CNT;
    pub use super::SMCAPTCTRLA0::CA1CNT;
    pub use super::SMCAPTCTRLA0::CFAWM;
    pub use super::SMCAPTCTRLA0::EDGA0;
    pub use super::SMCAPTCTRLA0::EDGA1;
    pub use super::SMCAPTCTRLA0::EDGCNTA_EN;
    pub use super::SMCAPTCTRLA0::INP_SELA;
    pub use super::SMCAPTCTRLA0::ONESHOTA;
}

/// Capture Compare A Register
pub mod SMCAPTCOMPA3 {
    pub use super::SMCAPTCOMPA0::EDGCMPA;
    pub use super::SMCAPTCOMPA0::EDGCNTA;
}

/// Capture Control B Register
pub mod SMCAPTCTRLB3 {
    pub use super::SMCAPTCTRLB0::ARMB;
    pub use super::SMCAPTCTRLB0::CB0CNT;
    pub use super::SMCAPTCTRLB0::CB1CNT;
    pub use super::SMCAPTCTRLB0::CFBWM;
    pub use super::SMCAPTCTRLB0::EDGB0;
    pub use super::SMCAPTCTRLB0::EDGB1;
    pub use super::SMCAPTCTRLB0::EDGCNTB_EN;
    pub use super::SMCAPTCTRLB0::INP_SELB;
    pub use super::SMCAPTCTRLB0::ONESHOTB;
}

/// Capture Compare B Register
pub mod SMCAPTCOMPB3 {
    pub use super::SMCAPTCOMPB0::EDGCMPB;
    pub use super::SMCAPTCOMPB0::EDGCNTB;
}

/// Capture Control X Register
pub mod SMCAPTCTRLX3 {
    pub use super::SMCAPTCTRLX0::ARMX;
    pub use super::SMCAPTCTRLX0::CFXWM;
    pub use super::SMCAPTCTRLX0::CX0CNT;
    pub use super::SMCAPTCTRLX0::CX1CNT;
    pub use super::SMCAPTCTRLX0::EDGCNTX_EN;
    pub use super::SMCAPTCTRLX0::EDGX0;
    pub use super::SMCAPTCTRLX0::EDGX1;
    pub use super::SMCAPTCTRLX0::INP_SELX;
    pub use super::SMCAPTCTRLX0::ONESHOTX;
}

/// Capture Compare X Register
pub mod SMCAPTCOMPX3 {
    pub use super::SMCAPTCOMPX0::EDGCMPX;
    pub use super::SMCAPTCOMPX0::EDGCNTX;
}

/// Capture Value 0 Register
pub mod SMCVAL03 {
    pub use super::SMCVAL00::CAPTVAL0;
}

/// Capture Value 0 Cycle Register
pub mod SMCVAL0CYC3 {
    pub use super::SMCVAL0CYC0::CVAL0CYC;
}

/// Capture Value 1 Register
pub mod SMCVAL13 {
    pub use super::SMCVAL10::CAPTVAL1;
}

/// Capture Value 1 Cycle Register
pub mod SMCVAL1CYC3 {
    pub use super::SMCVAL1CYC0::CVAL1CYC;
}

/// Capture Value 2 Register
pub mod SMCVAL23 {
    pub use super::SMCVAL20::CAPTVAL2;
}

/// Capture Value 2 Cycle Register
pub mod SMCVAL2CYC3 {
    pub use super::SMCVAL2CYC0::CVAL2CYC;
}

/// Capture Value 3 Register
pub mod SMCVAL33 {
    pub use super::SMCVAL30::CAPTVAL3;
}

/// Capture Value 3 Cycle Register
pub mod SMCVAL3CYC3 {
    pub use super::SMCVAL3CYC0::CVAL3CYC;
}

/// Capture Value 4 Register
pub mod SMCVAL43 {
    pub use super::SMCVAL40::CAPTVAL4;
}

/// Capture Value 4 Cycle Register
pub mod SMCVAL4CYC3 {
    pub use super::SMCVAL4CYC0::CVAL4CYC;
}

/// Capture Value 5 Register
pub mod SMCVAL53 {
    pub use super::SMCVAL50::CAPTVAL5;
}

/// Capture Value 5 Cycle Register
pub mod SMCVAL5CYC3 {
    pub use super::SMCVAL5CYC0::CVAL5CYC;
}
pub struct RegisterBlock {
    /// Counter Register
    pub SMCNT0: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT0: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL20: RWRegister<u16>,

    /// Control Register
    pub SMCTRL0: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// Value Register 0
    pub SMVAL00: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL10: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL10: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL20: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL20: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL30: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL30: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL40: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL40: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL50: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL50: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL0: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL0: RWRegister<u16>,

    /// Status Register
    pub SMSTS0: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN0: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN0: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL0: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP00: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP10: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT00: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT10: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA0: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA0: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB0: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB0: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX0: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX0: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL00: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC0: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL10: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC0: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL20: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC0: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL30: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC0: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL40: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC0: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL50: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC0: RORegister<u16>,

    _reserved2: [u32; 2],

    /// Counter Register
    pub SMCNT1: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT1: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL21: RWRegister<u16>,

    /// Control Register
    pub SMCTRL1: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// Value Register 0
    pub SMVAL01: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL11: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL11: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL21: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL21: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL31: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL31: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL41: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL41: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL51: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL51: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL1: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL1: RWRegister<u16>,

    /// Status Register
    pub SMSTS1: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN1: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN1: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL1: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP01: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP11: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT01: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT11: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA1: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA1: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB1: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB1: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX1: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX1: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL01: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC1: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL11: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC1: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL21: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC1: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL31: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC1: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL41: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC1: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL51: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC1: RORegister<u16>,

    _reserved4: [u32; 2],

    /// Counter Register
    pub SMCNT2: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT2: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL22: RWRegister<u16>,

    /// Control Register
    pub SMCTRL2: RWRegister<u16>,

    _reserved5: [u16; 1],

    /// Value Register 0
    pub SMVAL02: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL12: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL12: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL22: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL22: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL32: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL32: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL42: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL42: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL52: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL52: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL2: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL2: RWRegister<u16>,

    /// Status Register
    pub SMSTS2: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN2: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN2: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL2: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP02: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP12: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT02: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT12: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA2: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA2: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB2: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB2: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX2: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX2: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL02: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC2: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL12: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC2: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL22: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC2: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL32: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC2: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL42: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC2: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL52: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC2: RORegister<u16>,

    _reserved6: [u32; 2],

    /// Counter Register
    pub SMCNT3: RORegister<u16>,

    /// Initial Count Register
    pub SMINIT3: RWRegister<u16>,

    /// Control 2 Register
    pub SMCTRL23: RWRegister<u16>,

    /// Control Register
    pub SMCTRL3: RWRegister<u16>,

    _reserved7: [u16; 1],

    /// Value Register 0
    pub SMVAL03: RWRegister<u16>,

    /// Fractional Value Register 1
    pub SMFRACVAL13: RWRegister<u16>,

    /// Value Register 1
    pub SMVAL13: RWRegister<u16>,

    /// Fractional Value Register 2
    pub SMFRACVAL23: RWRegister<u16>,

    /// Value Register 2
    pub SMVAL23: RWRegister<u16>,

    /// Fractional Value Register 3
    pub SMFRACVAL33: RWRegister<u16>,

    /// Value Register 3
    pub SMVAL33: RWRegister<u16>,

    /// Fractional Value Register 4
    pub SMFRACVAL43: RWRegister<u16>,

    /// Value Register 4
    pub SMVAL43: RWRegister<u16>,

    /// Fractional Value Register 5
    pub SMFRACVAL53: RWRegister<u16>,

    /// Value Register 5
    pub SMVAL53: RWRegister<u16>,

    /// Fractional Control Register
    pub SMFRCTRL3: RWRegister<u16>,

    /// Output Control Register
    pub SMOCTRL3: RWRegister<u16>,

    /// Status Register
    pub SMSTS3: RWRegister<u16>,

    /// Interrupt Enable Register
    pub SMINTEN3: RWRegister<u16>,

    /// DMA Enable Register
    pub SMDMAEN3: RWRegister<u16>,

    /// Output Trigger Control Register
    pub SMTCTRL3: RWRegister<u16>,

    /// Fault Disable Mapping Register 0
    pub SMDISMAP03: RWRegister<u16>,

    /// Fault Disable Mapping Register 1
    pub SMDISMAP13: RWRegister<u16>,

    /// Deadtime Count Register 0
    pub SMDTCNT03: RWRegister<u16>,

    /// Deadtime Count Register 1
    pub SMDTCNT13: RWRegister<u16>,

    /// Capture Control A Register
    pub SMCAPTCTRLA3: RWRegister<u16>,

    /// Capture Compare A Register
    pub SMCAPTCOMPA3: RWRegister<u16>,

    /// Capture Control B Register
    pub SMCAPTCTRLB3: RWRegister<u16>,

    /// Capture Compare B Register
    pub SMCAPTCOMPB3: RWRegister<u16>,

    /// Capture Control X Register
    pub SMCAPTCTRLX3: RWRegister<u16>,

    /// Capture Compare X Register
    pub SMCAPTCOMPX3: RWRegister<u16>,

    /// Capture Value 0 Register
    pub SMCVAL03: RORegister<u16>,

    /// Capture Value 0 Cycle Register
    pub SMCVAL0CYC3: RORegister<u16>,

    /// Capture Value 1 Register
    pub SMCVAL13: RORegister<u16>,

    /// Capture Value 1 Cycle Register
    pub SMCVAL1CYC3: RORegister<u16>,

    /// Capture Value 2 Register
    pub SMCVAL23: RORegister<u16>,

    /// Capture Value 2 Cycle Register
    pub SMCVAL2CYC3: RORegister<u16>,

    /// Capture Value 3 Register
    pub SMCVAL33: RORegister<u16>,

    /// Capture Value 3 Cycle Register
    pub SMCVAL3CYC3: RORegister<u16>,

    /// Capture Value 4 Register
    pub SMCVAL43: RORegister<u16>,

    /// Capture Value 4 Cycle Register
    pub SMCVAL4CYC3: RORegister<u16>,

    /// Capture Value 5 Register
    pub SMCVAL53: RORegister<u16>,

    /// Capture Value 5 Cycle Register
    pub SMCVAL5CYC3: RORegister<u16>,

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
    pub SMCNT0: u16,
    pub SMINIT0: u16,
    pub SMCTRL20: u16,
    pub SMCTRL0: u16,
    pub SMVAL00: u16,
    pub SMFRACVAL10: u16,
    pub SMVAL10: u16,
    pub SMFRACVAL20: u16,
    pub SMVAL20: u16,
    pub SMFRACVAL30: u16,
    pub SMVAL30: u16,
    pub SMFRACVAL40: u16,
    pub SMVAL40: u16,
    pub SMFRACVAL50: u16,
    pub SMVAL50: u16,
    pub SMFRCTRL0: u16,
    pub SMOCTRL0: u16,
    pub SMSTS0: u16,
    pub SMINTEN0: u16,
    pub SMDMAEN0: u16,
    pub SMTCTRL0: u16,
    pub SMDISMAP00: u16,
    pub SMDISMAP10: u16,
    pub SMDTCNT00: u16,
    pub SMDTCNT10: u16,
    pub SMCAPTCTRLA0: u16,
    pub SMCAPTCOMPA0: u16,
    pub SMCAPTCTRLB0: u16,
    pub SMCAPTCOMPB0: u16,
    pub SMCAPTCTRLX0: u16,
    pub SMCAPTCOMPX0: u16,
    pub SMCVAL00: u16,
    pub SMCVAL0CYC0: u16,
    pub SMCVAL10: u16,
    pub SMCVAL1CYC0: u16,
    pub SMCVAL20: u16,
    pub SMCVAL2CYC0: u16,
    pub SMCVAL30: u16,
    pub SMCVAL3CYC0: u16,
    pub SMCVAL40: u16,
    pub SMCVAL4CYC0: u16,
    pub SMCVAL50: u16,
    pub SMCVAL5CYC0: u16,
    pub SMCNT1: u16,
    pub SMINIT1: u16,
    pub SMCTRL21: u16,
    pub SMCTRL1: u16,
    pub SMVAL01: u16,
    pub SMFRACVAL11: u16,
    pub SMVAL11: u16,
    pub SMFRACVAL21: u16,
    pub SMVAL21: u16,
    pub SMFRACVAL31: u16,
    pub SMVAL31: u16,
    pub SMFRACVAL41: u16,
    pub SMVAL41: u16,
    pub SMFRACVAL51: u16,
    pub SMVAL51: u16,
    pub SMFRCTRL1: u16,
    pub SMOCTRL1: u16,
    pub SMSTS1: u16,
    pub SMINTEN1: u16,
    pub SMDMAEN1: u16,
    pub SMTCTRL1: u16,
    pub SMDISMAP01: u16,
    pub SMDISMAP11: u16,
    pub SMDTCNT01: u16,
    pub SMDTCNT11: u16,
    pub SMCAPTCTRLA1: u16,
    pub SMCAPTCOMPA1: u16,
    pub SMCAPTCTRLB1: u16,
    pub SMCAPTCOMPB1: u16,
    pub SMCAPTCTRLX1: u16,
    pub SMCAPTCOMPX1: u16,
    pub SMCVAL01: u16,
    pub SMCVAL0CYC1: u16,
    pub SMCVAL11: u16,
    pub SMCVAL1CYC1: u16,
    pub SMCVAL21: u16,
    pub SMCVAL2CYC1: u16,
    pub SMCVAL31: u16,
    pub SMCVAL3CYC1: u16,
    pub SMCVAL41: u16,
    pub SMCVAL4CYC1: u16,
    pub SMCVAL51: u16,
    pub SMCVAL5CYC1: u16,
    pub SMCNT2: u16,
    pub SMINIT2: u16,
    pub SMCTRL22: u16,
    pub SMCTRL2: u16,
    pub SMVAL02: u16,
    pub SMFRACVAL12: u16,
    pub SMVAL12: u16,
    pub SMFRACVAL22: u16,
    pub SMVAL22: u16,
    pub SMFRACVAL32: u16,
    pub SMVAL32: u16,
    pub SMFRACVAL42: u16,
    pub SMVAL42: u16,
    pub SMFRACVAL52: u16,
    pub SMVAL52: u16,
    pub SMFRCTRL2: u16,
    pub SMOCTRL2: u16,
    pub SMSTS2: u16,
    pub SMINTEN2: u16,
    pub SMDMAEN2: u16,
    pub SMTCTRL2: u16,
    pub SMDISMAP02: u16,
    pub SMDISMAP12: u16,
    pub SMDTCNT02: u16,
    pub SMDTCNT12: u16,
    pub SMCAPTCTRLA2: u16,
    pub SMCAPTCOMPA2: u16,
    pub SMCAPTCTRLB2: u16,
    pub SMCAPTCOMPB2: u16,
    pub SMCAPTCTRLX2: u16,
    pub SMCAPTCOMPX2: u16,
    pub SMCVAL02: u16,
    pub SMCVAL0CYC2: u16,
    pub SMCVAL12: u16,
    pub SMCVAL1CYC2: u16,
    pub SMCVAL22: u16,
    pub SMCVAL2CYC2: u16,
    pub SMCVAL32: u16,
    pub SMCVAL3CYC2: u16,
    pub SMCVAL42: u16,
    pub SMCVAL4CYC2: u16,
    pub SMCVAL52: u16,
    pub SMCVAL5CYC2: u16,
    pub SMCNT3: u16,
    pub SMINIT3: u16,
    pub SMCTRL23: u16,
    pub SMCTRL3: u16,
    pub SMVAL03: u16,
    pub SMFRACVAL13: u16,
    pub SMVAL13: u16,
    pub SMFRACVAL23: u16,
    pub SMVAL23: u16,
    pub SMFRACVAL33: u16,
    pub SMVAL33: u16,
    pub SMFRACVAL43: u16,
    pub SMVAL43: u16,
    pub SMFRACVAL53: u16,
    pub SMVAL53: u16,
    pub SMFRCTRL3: u16,
    pub SMOCTRL3: u16,
    pub SMSTS3: u16,
    pub SMINTEN3: u16,
    pub SMDMAEN3: u16,
    pub SMTCTRL3: u16,
    pub SMDISMAP03: u16,
    pub SMDISMAP13: u16,
    pub SMDTCNT03: u16,
    pub SMDTCNT13: u16,
    pub SMCAPTCTRLA3: u16,
    pub SMCAPTCOMPA3: u16,
    pub SMCAPTCTRLB3: u16,
    pub SMCAPTCOMPB3: u16,
    pub SMCAPTCTRLX3: u16,
    pub SMCAPTCOMPX3: u16,
    pub SMCVAL03: u16,
    pub SMCVAL0CYC3: u16,
    pub SMCVAL13: u16,
    pub SMCVAL1CYC3: u16,
    pub SMCVAL23: u16,
    pub SMCVAL2CYC3: u16,
    pub SMCVAL33: u16,
    pub SMCVAL3CYC3: u16,
    pub SMCVAL43: u16,
    pub SMCVAL4CYC3: u16,
    pub SMCVAL53: u16,
    pub SMCVAL5CYC3: u16,
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
