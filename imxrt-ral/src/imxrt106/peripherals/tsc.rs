#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Touch Screen Controller
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// no description available
pub mod BASIC_SETTING {

    /// Auto Measure
    pub mod AUTO_MEASURE {
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

            /// 0b0: Disable Auto Measure
            pub const AUTO_MEASURE_0: u32 = 0b0;

            /// 0b1: Auto Measure
            pub const AUTO_MEASURE_1: u32 = 0b1;
        }
    }

    /// 4/5 Wire detection
    pub mod _4_5_WIRE {
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

            /// 0b0: 4-Wire Detection Mode
            pub const _4_5_WIRE_0: u32 = 0b0;

            /// 0b1: 5-Wire Detection Mode
            pub const _4_5_WIRE_1: u32 = 0b1;
        }
    }

    /// Measure Delay Time
    pub mod MEASURE_DELAY_TIME {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// no description available
pub mod PRE_CHARGE_TIME {

    /// Before detection, the top screen needs some time before being pulled up to a high voltage.
    pub mod PRE_CHARGE_TIME {
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

/// Flow Control
pub mod FLOW_CONTROL {

    /// Soft Reset
    pub mod SW_RST {
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

    /// Start Measure
    pub mod START_MEASURE {
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

            /// 0b0: Do not start measure for now
            pub const START_MEASURE_0: u32 = 0b0;

            /// 0b1: Start measure the X/Y coordinate value
            pub const START_MEASURE_1: u32 = 0b1;
        }
    }

    /// Drop Measure
    pub mod DROP_MEASURE {
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

            /// 0b0: Do not drop measure for now
            pub const DROP_MEASURE_0: u32 = 0b0;

            /// 0b1: Drop the measure and controller return to idle status
            pub const DROP_MEASURE_1: u32 = 0b1;
        }
    }

    /// Start Sense
    pub mod START_SENSE {
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

            /// 0b0: Stay at idle status
            pub const START_SENSE_0: u32 = 0b0;

            /// 0b1: Start sense detection and (if auto_measure set to 1) measure after detect a touch
            pub const START_SENSE_1: u32 = 0b1;
        }
    }

    /// This bit is for SW disable registers
    pub mod DISABLE {
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

            /// 0b0: Leave HW state machine control
            pub const DISABLE_0: u32 = 0b0;

            /// 0b1: SW set to idle status
            pub const DISABLE_1: u32 = 0b1;
        }
    }
}

/// Measure Value
pub mod MEASEURE_VALUE {

    /// Y Value
    pub mod Y_VALUE {
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

    /// X Value
    pub mod X_VALUE {
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

/// Interrupt Enable
pub mod INT_EN {

    /// Measure Interrupt Enable
    pub mod MEASURE_INT_EN {
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

            /// 0b0: Disable measure interrupt
            pub const MEASURE_INT_EN_0: u32 = 0b0;

            /// 0b1: Enable measure interrupt
            pub const MEASURE_INT_EN_1: u32 = 0b1;
        }
    }

    /// Detect Interrupt Enable
    pub mod DETECT_INT_EN {
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

            /// 0b0: Disable detect interrupt
            pub const DETECT_INT_EN_0: u32 = 0b0;

            /// 0b1: Enable detect interrupt
            pub const DETECT_INT_EN_1: u32 = 0b1;
        }
    }

    /// Idle Software Interrupt Enable
    pub mod IDLE_SW_INT_EN {
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

            /// 0b0: Disable idle software interrupt
            pub const IDLE_SW_INT_EN_0: u32 = 0b0;

            /// 0b1: Enable idle software interrupt
            pub const IDLE_SW_INT_EN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Signal Enable
pub mod INT_SIG_EN {

    /// Measure Signal Enable
    pub mod MEASURE_SIG_EN {
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

    /// Detect Signal Enable
    pub mod DETECT_SIG_EN {
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

            /// 0b0: Disable detect signal
            pub const DETECT_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enable detect signal
            pub const DETECT_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Valid Signal Enable
    pub mod VALID_SIG_EN {
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

            /// 0b0: Disable valid signal
            pub const VALID_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enable valid signal
            pub const VALID_SIG_EN_1: u32 = 0b1;
        }
    }

    /// Idle Software Signal Enable
    pub mod IDLE_SW_SIG_EN {
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

            /// 0b0: Disable idle software signal
            pub const IDLE_SW_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enable idle software signal
            pub const IDLE_SW_SIG_EN_1: u32 = 0b1;
        }
    }
}

/// Intterrupt Status
pub mod INT_STATUS {

    /// Measure Signal
    pub mod MEASURE {
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

            /// 0b0: Does not exist a measure signal
            pub const MEASURE_0: u32 = 0b0;

            /// 0b1: Exist a measure signal
            pub const MEASURE_1: u32 = 0b1;
        }
    }

    /// Detect Signal
    pub mod DETECT {
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

            /// 0b0: Does not exist a detect signal
            pub const DETECT_0: u32 = 0b0;

            /// 0b1: Exist detect signal
            pub const DETECT_1: u32 = 0b1;
        }
    }

    /// Valid Signal
    pub mod VALID {
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

            /// 0b0: There is no touch detected after measurement, indicates that the measured value is not valid
            pub const VALID_0: u32 = 0b0;

            /// 0b1: There is touch detection after measurement, indicates that the measure is valid
            pub const VALID_1: u32 = 0b1;
        }
    }

    /// Idle Software
    pub mod IDLE_SW {
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

            /// 0b0: Haven't return to idle status
            pub const IDLE_SW_0: u32 = 0b0;

            /// 0b1: Already return to idle status
            pub const IDLE_SW_1: u32 = 0b1;
        }
    }
}

/// no description available
pub mod DEBUG_MODE {

    /// ADC Conversion Value
    pub mod ADC_CONV_VALUE {
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

    /// ADC COCO Signal
    pub mod ADC_COCO {
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

    /// Hardware Trigger Select Signal
    pub mod EXT_HWTS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger
    pub mod TRIGGER {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No hardware trigger signal
            pub const TRIGGER_0: u32 = 0b0;

            /// 0b1: Hardware trigger signal, the signal must last at least 1 ips clock period
            pub const TRIGGER_1: u32 = 0b1;
        }
    }

    /// ADC Coco Clear
    pub mod ADC_COCO_CLEAR {
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

            /// 0b0: No ADC COCO clear
            pub const ADC_COCO_CLEAR_0: u32 = 0b0;

            /// 0b1: Set ADC COCO clear
            pub const ADC_COCO_CLEAR_1: u32 = 0b1;
        }
    }

    /// ADC COCO Clear Disable
    pub mod ADC_COCO_CLEAR_DISABLE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allow TSC hardware generates ADC COCO clear
            pub const ADC_COCO_CLEAR_DISABLE_0: u32 = 0b0;

            /// 0b1: Prevent TSC from generate ADC COCO clear signal
            pub const ADC_COCO_CLEAR_DISABLE_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DEBUG_EN {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable debug mode
            pub const DEBUG_EN_0: u32 = 0b0;

            /// 0b1: Disable debug mode
            pub const DEBUG_EN_1: u32 = 0b1;
        }
    }
}

/// no description available
pub mod DEBUG_MODE2 {

    /// XPUL Wire Pull Down Switch
    pub mod XPUL_PULL_DOWN {
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

            /// 0b0: Close the switch
            pub const XPUL_PULL_DOWN_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XPUL_PULL_DOWN_1: u32 = 0b1;
        }
    }

    /// XPUL Wire Pull Up Switch
    pub mod XPUL_PULL_UP {
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

            /// 0b0: Close the switch
            pub const XPUL_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XPUL_PULL_UP_1: u32 = 0b1;
        }
    }

    /// XPUL Wire 200K Pull Up Switch
    pub mod XPUL_200K_PULL_UP {
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

            /// 0b0: Close the switch
            pub const XPUL_200K_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XPUL_200K_PULL_UP_1: u32 = 0b1;
        }
    }

    /// XNUR Wire Pull Down Switch
    pub mod XNUR_PULL_DOWN {
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

            /// 0b0: Close the switch
            pub const XNUR_PULL_DOWN_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XNUR_PULL_DOWN_1: u32 = 0b1;
        }
    }

    /// XNUR Wire Pull Up Switch
    pub mod XNUR_PULL_UP {
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

            /// 0b0: Close the switch
            pub const XNUR_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XNUR_PULL_UP_1: u32 = 0b1;
        }
    }

    /// XNUR Wire 200K Pull Up Switch
    pub mod XNUR_200K_PULL_UP {
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

            /// 0b0: Close the switch
            pub const XNUR_200K_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const XNUR_200K_PULL_UP_1: u32 = 0b1;
        }
    }

    /// YPLL Wire Pull Down Switch
    pub mod YPLL_PULL_DOWN {
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

            /// 0b0: Close the switch
            pub const YPLL_PULL_DOWN_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const YPLL_PULL_DOWN_1: u32 = 0b1;
        }
    }

    /// YPLL Wire Pull Up Switch
    pub mod YPLL_PULL_UP {
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

            /// 0b0: Close the switch
            pub const YPLL_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open the switch
            pub const YPLL_PULL_UP_1: u32 = 0b1;
        }
    }

    /// YPLL Wire 200K Pull Up Switch
    pub mod YPLL_200K_PULL_UP {
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

            /// 0b0: Close the switch
            pub const YPLL_200K_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const YPLL_200K_PULL_UP_1: u32 = 0b1;
        }
    }

    /// YNLR Wire Pull Down Switch
    pub mod YNLR_PULL_DOWN {
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

            /// 0b0: Close the switch
            pub const YNLR_PULL_DOWN_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const YNLR_PULL_DOWN_1: u32 = 0b1;
        }
    }

    /// YNLR Wire Pull Up Switch
    pub mod YNLR_PULL_UP {
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

            /// 0b0: Close the switch
            pub const YNLR_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const YNLR_PULL_UP_1: u32 = 0b1;
        }
    }

    /// YNLR Wire 200K Pull Up Switch
    pub mod YNLR_200K_PULL_UP {
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

            /// 0b0: Close the switch
            pub const YNLR_200K_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const YNLR_200K_PULL_UP_1: u32 = 0b1;
        }
    }

    /// Wiper Wire Pull Down Switch
    pub mod WIPER_PULL_DOWN {
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

            /// 0b0: Close the switch
            pub const WIPER_PULL_DOWN_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const WIPER_PULL_DOWN_1: u32 = 0b1;
        }
    }

    /// Wiper Wire Pull Up Switch
    pub mod WIPER_PULL_UP {
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

            /// 0b0: Close the switch
            pub const WIPER_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const WIPER_PULL_UP_1: u32 = 0b1;
        }
    }

    /// Wiper Wire 200K Pull Up Switch
    pub mod WIPER_200K_PULL_UP {
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

            /// 0b0: Close the switch
            pub const WIPER_200K_PULL_UP_0: u32 = 0b0;

            /// 0b1: Open up the switch
            pub const WIPER_200K_PULL_UP_1: u32 = 0b1;
        }
    }

    /// Detect Four Wire
    pub mod DETECT_FOUR_WIRE {
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

            /// 0b0: No detect signal
            pub const DETECT_FOUR_WIRE_0: u32 = 0b0;

            /// 0b1: Yes, there is a detect on the touch screen.
            pub const DETECT_FOUR_WIRE_1: u32 = 0b1;
        }
    }

    /// Detect Five Wire
    pub mod DETECT_FIVE_WIRE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No detect signal
            pub const DETECT_FIVE_WIRE_0: u32 = 0b0;

            /// 0b1: Yes, there is a detect on the touch screen.
            pub const DETECT_FIVE_WIRE_1: u32 = 0b1;
        }
    }

    /// State Machine
    pub mod STATE_MACHINE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Idle
            pub const STATE_MACHINE_0: u32 = 0b000;

            /// 0b001: Pre-charge
            pub const STATE_MACHINE_1: u32 = 0b001;

            /// 0b010: Detect
            pub const STATE_MACHINE_2: u32 = 0b010;

            /// 0b011: X-measure
            pub const STATE_MACHINE_3: u32 = 0b011;

            /// 0b100: Y-measure
            pub const STATE_MACHINE_4: u32 = 0b100;

            /// 0b101: Pre-charge
            pub const STATE_MACHINE_5: u32 = 0b101;

            /// 0b110: Detect
            pub const STATE_MACHINE_6: u32 = 0b110;
        }
    }

    /// Intermediate State
    pub mod INTERMEDIATE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Not in intermedia
            pub const INTERMEDIATE_0: u32 = 0b0;

            /// 0b1: Intermedia
            pub const INTERMEDIATE_1: u32 = 0b1;
        }
    }

    /// Detect Enable Four Wire
    pub mod DETECT_ENABLE_FOUR_WIRE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not read four wire detect value, read default value from analogue
            pub const DETECT_ENABLE_FOUR_WIRE_0: u32 = 0b0;

            /// 0b1: Read four wire detect status from analogue
            pub const DETECT_ENABLE_FOUR_WIRE_1: u32 = 0b1;
        }
    }

    /// Detect Enable Five Wire
    pub mod DETECT_ENABLE_FIVE_WIRE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Do not read five wire detect value, read default value from analogue
            pub const DETECT_ENABLE_FIVE_WIRE_0: u32 = 0b0;

            /// 0b1: Read five wire detect status from analogue
            pub const DETECT_ENABLE_FIVE_WIRE_1: u32 = 0b1;
        }
    }

    /// This field indicates glitch threshold
    pub mod DE_GLITCH {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles
            pub const DE_GLITCH_0: u32 = 0b00;

            /// 0b01: Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles
            pub const DE_GLITCH_1: u32 = 0b01;

            /// 0b10: Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles
            pub const DE_GLITCH_2: u32 = 0b10;

            /// 0b11: Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles
            pub const DE_GLITCH_3: u32 = 0b11;
        }
    }
}
pub struct RegisterBlock {
    /// no description available
    pub BASIC_SETTING: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// no description available
    pub PRE_CHARGE_TIME: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// Flow Control
    pub FLOW_CONTROL: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Measure Value
    pub MEASEURE_VALUE: RORegister<u32>,

    _reserved4: [u32; 3],

    /// Interrupt Enable
    pub INT_EN: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Interrupt Signal Enable
    pub INT_SIG_EN: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Intterrupt Status
    pub INT_STATUS: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// no description available
    pub DEBUG_MODE: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// no description available
    pub DEBUG_MODE2: RWRegister<u32>,
}
pub struct ResetValues {
    pub BASIC_SETTING: u32,
    pub PRE_CHARGE_TIME: u32,
    pub FLOW_CONTROL: u32,
    pub MEASEURE_VALUE: u32,
    pub INT_EN: u32,
    pub INT_SIG_EN: u32,
    pub INT_STATUS: u32,
    pub DEBUG_MODE: u32,
    pub DEBUG_MODE2: u32,
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
