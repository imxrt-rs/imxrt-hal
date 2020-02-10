#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SNVS
//!
//! Used by: imxrt1011, imxrt1015

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SNVS_HP Lock Register
pub mod HPLR {

    /// Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR
    pub mod ZMK_WSL {
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

            /// 0b0: Write access is allowed
            pub const ZMK_WSL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const ZMK_WSL_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR
    pub mod ZMK_RSL {
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

            /// 0b0: Read access is allowed (only in software Programming mode)
            pub const ZMK_RSL_0: u32 = 0b0;

            /// 0b1: Read access is not allowed
            pub const ZMK_RSL_1: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits
    pub mod SRTC_SL {
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

            /// 0b0: Write access is allowed
            pub const SRTC_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const SRTC_SL_1: u32 = 0b1;
        }
    }

    /// LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)
    pub mod LPCALB_SL {
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

            /// 0b0: Write access is allowed
            pub const LPCALB_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const LPCALB_SL_1: u32 = 0b1;
        }
    }

    /// Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit
    pub mod MC_SL {
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

            /// 0b0: Write access (increment) is allowed
            pub const MC_SL_0: u32 = 0b0;

            /// 0b1: Write access (increment) is not allowed
            pub const MC_SL_1: u32 = 0b1;
        }
    }

    /// General Purpose Register Soft Lock When set, prevents any writes to the GPR
    pub mod GPR_SL {
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

            /// 0b0: Write access is allowed
            pub const GPR_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const GPR_SL_1: u32 = 0b1;
        }
    }

    /// LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR
    pub mod LPSVCR_SL {
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

            /// 0b0: Write access is allowed
            pub const LPSVCR_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const LPSVCR_SL_1: u32 = 0b1;
        }
    }

    /// LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR
    pub mod LPTDCR_SL {
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

            /// 0b0: Write access is allowed
            pub const LPTDCR_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const LPTDCR_SL_1: u32 = 0b1;
        }
    }

    /// Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR
    pub mod MKS_SL {
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

            /// 0b0: Write access is allowed
            pub const MKS_SL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const MKS_SL_1: u32 = 0b1;
        }
    }

    /// HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR
    pub mod HPSVCR_L {
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

            /// 0b0: Write access is allowed
            pub const HPSVCR_L_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const HPSVCR_L_1: u32 = 0b1;
        }
    }

    /// HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR
    pub mod HPSICR_L {
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

            /// 0b0: Write access is allowed
            pub const HPSICR_L_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const HPSICR_L_1: u32 = 0b1;
        }
    }

    /// High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR
    pub mod HAC_L {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write access is allowed
            pub const HAC_L_0: u32 = 0b0;

            /// 0b1: Write access is not allowed
            pub const HAC_L_1: u32 = 0b1;
        }
    }
}

/// SNVS_HP Command Register
pub mod HPCOMR {

    /// SSM State Transition Transition state of the system security monitor
    pub mod SSM_ST {
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

    /// SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state
    pub mod SSM_ST_DIS {
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

            /// 0b0: Secure to Trusted State transition is enabled
            pub const SSM_ST_DIS_0: u32 = 0b0;

            /// 0b1: Secure to Trusted State transition is disabled
            pub const SSM_ST_DIS_1: u32 = 0b1;
        }
    }

    /// SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state
    pub mod SSM_SFNS_DIS {
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

            /// 0b0: Soft Fail to Non-Secure State transition is enabled
            pub const SSM_SFNS_DIS_0: u32 = 0b0;

            /// 0b1: Soft Fail to Non-Secure State transition is disabled
            pub const SSM_SFNS_DIS_1: u32 = 0b1;
        }
    }

    /// LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set
    pub mod LP_SWR {
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

            /// 0b0: No Action
            pub const LP_SWR_0: u32 = 0b0;

            /// 0b1: Reset LP section
            pub const LP_SWR_1: u32 = 0b1;
        }
    }

    /// LP Software Reset Disable When set, disables the LP software reset
    pub mod LP_SWR_DIS {
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

            /// 0b0: LP software reset is enabled
            pub const LP_SWR_DIS_0: u32 = 0b0;

            /// 0b1: LP software reset is disabled
            pub const LP_SWR_DIS_1: u32 = 0b1;
        }
    }

    /// Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation
    pub mod SW_SV {
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

    /// Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation
    pub mod SW_FSV {
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

    /// LP Software Security Violation When set, SNVS_LP treats this bit as a security violation
    pub mod SW_LPSV {
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

    /// Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism
    pub mod PROG_ZMK {
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

            /// 0b0: No Action
            pub const PROG_ZMK_0: u32 = 0b0;

            /// 0b1: Activate hardware key programming mechanism
            pub const PROG_ZMK_1: u32 = 0b1;
        }
    }

    /// Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default
    pub mod MKS_EN {
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

            /// 0b0: OTP master key is selected as an SNVS master key
            pub const MKS_EN_0: u32 = 0b0;

            /// 0b1: SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR
            pub const MKS_EN_1: u32 = 0b1;
        }
    }

    /// High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state
    pub mod HAC_EN {
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

            /// 0b0: High Assurance Counter is disabled
            pub const HAC_EN_0: u32 = 0b0;

            /// 0b1: High Assurance Counter is enabled
            pub const HAC_EN_1: u32 = 0b1;
        }
    }

    /// High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register
    pub mod HAC_LOAD {
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

            /// 0b0: No Action
            pub const HAC_LOAD_0: u32 = 0b0;

            /// 0b1: Load the HAC
            pub const HAC_LOAD_1: u32 = 0b1;
        }
    }

    /// High Assurance Counter Clear When set, it clears the High Assurance Counter Register
    pub mod HAC_CLEAR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Action
            pub const HAC_CLEAR_0: u32 = 0b0;

            /// 0b1: Clear the HAC
            pub const HAC_CLEAR_1: u32 = 0b1;
        }
    }

    /// High Assurance Counter Stop This bit can be set only when SSM is in soft fail state
    pub mod HAC_STOP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only
    pub mod NPSWA_EN {
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

/// SNVS_HP Control Register
pub mod HPCR {

    /// HP Real Time Counter Enable
    pub mod RTC_EN {
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

            /// 0b0: RTC is disabled
            pub const RTC_EN_0: u32 = 0b0;

            /// 0b1: RTC is enabled
            pub const RTC_EN_1: u32 = 0b1;
        }
    }

    /// HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter
    pub mod HPTA_EN {
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

            /// 0b0: HP Time Alarm Interrupt is disabled
            pub const HPTA_EN_0: u32 = 0b0;

            /// 0b1: HP Time Alarm Interrupt is enabled
            pub const HPTA_EN_1: u32 = 0b1;
        }
    }

    /// Disable periodic interrupt in the functional interrupt
    pub mod DIS_PI {
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

            /// 0b0: Periodic interrupt will trigger a functional interrupt
            pub const DIS_PI_0: u32 = 0b0;

            /// 0b1: Disable periodic interrupt in the function interrupt
            pub const DIS_PI_1: u32 = 0b1;
        }
    }

    /// HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled
    pub mod PI_EN {
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

            /// 0b0: HP Periodic Interrupt is disabled
            pub const PI_EN_0: u32 = 0b0;

            /// 0b1: HP Periodic Interrupt is enabled
            pub const PI_EN_1: u32 = 0b1;
        }
    }

    /// Periodic Interrupt Frequency Defines frequency of the periodic interrupt
    pub mod PI_FREQ {
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

            /// 0b0000: - bit 0 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_0: u32 = 0b0000;

            /// 0b0001: - bit 1 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_1: u32 = 0b0001;

            /// 0b0010: - bit 2 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_2: u32 = 0b0010;

            /// 0b0011: - bit 3 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_3: u32 = 0b0011;

            /// 0b0100: - bit 4 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_4: u32 = 0b0100;

            /// 0b0101: - bit 5 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_5: u32 = 0b0101;

            /// 0b0110: - bit 6 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_6: u32 = 0b0110;

            /// 0b0111: - bit 7 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_7: u32 = 0b0111;

            /// 0b1000: - bit 8 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_8: u32 = 0b1000;

            /// 0b1001: - bit 9 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_9: u32 = 0b1001;

            /// 0b1010: - bit 10 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_10: u32 = 0b1010;

            /// 0b1011: - bit 11 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_11: u32 = 0b1011;

            /// 0b1100: - bit 12 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_12: u32 = 0b1100;

            /// 0b1101: - bit 13 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_13: u32 = 0b1101;

            /// 0b1110: - bit 14 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_14: u32 = 0b1110;

            /// 0b1111: - bit 15 of the HPRTCLR is selected as a source of the periodic interrupt
            pub const PI_FREQ_15: u32 = 0b1111;
        }
    }

    /// HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled.
    pub mod HPCALB_EN {
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

            /// 0b0: HP Timer calibration disabled
            pub const HPCALB_EN_0: u32 = 0b0;

            /// 0b1: HP Timer calibration enabled
            pub const HPCALB_EN_1: u32 = 0b1;
        }
    }

    /// HP Calibration Value Defines signed calibration value for the HP Real Time Counter
    pub mod HPCALB_VAL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: +0 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_0: u32 = 0b00000;

            /// 0b00001: +1 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_1: u32 = 0b00001;

            /// 0b00010: +2 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_2: u32 = 0b00010;

            /// 0b01111: +15 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_15: u32 = 0b01111;

            /// 0b10000: -16 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_16: u32 = 0b10000;

            /// 0b10001: -15 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_17: u32 = 0b10001;

            /// 0b11110: -2 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_30: u32 = 0b11110;

            /// 0b11111: -1 counts per each 32768 ticks of the counter
            pub const HPCALB_VAL_31: u32 = 0b11111;
        }
    }

    /// HP Time Synchronize
    pub mod HP_TS {
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

            /// 0b0: No Action
            pub const HP_TS_0: u32 = 0b0;

            /// 0b1: Synchronize the HP Time Counter to the LP Time Counter
            pub const HP_TS_1: u32 = 0b1;
        }
    }

    /// Button Configuration
    pub mod BTN_CONFIG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Button interrupt mask
    pub mod BTN_MASK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_HP Security Interrupt Control Register
pub mod HPSICR {

    /// Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation
    pub mod SV0_EN {
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

            /// 0b0: Security Violation 0 Interrupt is Disabled
            pub const SV0_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 0 Interrupt is Enabled
            pub const SV0_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation
    pub mod SV1_EN {
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

            /// 0b0: Security Violation 1 Interrupt is Disabled
            pub const SV1_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 1 Interrupt is Enabled
            pub const SV1_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation
    pub mod SV2_EN {
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

            /// 0b0: Security Violation 2 Interrupt is Disabled
            pub const SV2_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 2 Interrupt is Enabled
            pub const SV2_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation
    pub mod SV3_EN {
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

            /// 0b0: Security Violation 3 Interrupt is Disabled
            pub const SV3_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 3 Interrupt is Enabled
            pub const SV3_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation
    pub mod SV4_EN {
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

            /// 0b0: Security Violation 4 Interrupt is Disabled
            pub const SV4_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 4 Interrupt is Enabled
            pub const SV4_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation
    pub mod SV5_EN {
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

            /// 0b0: Security Violation 5 Interrupt is Disabled
            pub const SV5_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 5 Interrupt is Enabled
            pub const SV5_EN_1: u32 = 0b1;
        }
    }

    /// LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section
    pub mod LPSVI_EN {
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

            /// 0b0: LP Security Violation Interrupt is Disabled
            pub const LPSVI_EN_0: u32 = 0b0;

            /// 0b1: LP Security Violation Interrupt is Enabled
            pub const LPSVI_EN_1: u32 = 0b1;
        }
    }
}

/// SNVS_HP Security Violation Control Register
pub mod HPSVCR {

    /// Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input
    pub mod SV0_CFG {
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

            /// 0b0: Security Violation 0 is a non-fatal violation
            pub const SV0_CFG_0: u32 = 0b0;

            /// 0b1: Security Violation 0 is a fatal violation
            pub const SV0_CFG_1: u32 = 0b1;
        }
    }

    /// Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input
    pub mod SV1_CFG {
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

            /// 0b0: Security Violation 1 is a non-fatal violation
            pub const SV1_CFG_0: u32 = 0b0;

            /// 0b1: Security Violation 1 is a fatal violation
            pub const SV1_CFG_1: u32 = 0b1;
        }
    }

    /// Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input
    pub mod SV2_CFG {
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

            /// 0b0: Security Violation 2 is a non-fatal violation
            pub const SV2_CFG_0: u32 = 0b0;

            /// 0b1: Security Violation 2 is a fatal violation
            pub const SV2_CFG_1: u32 = 0b1;
        }
    }

    /// Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input
    pub mod SV3_CFG {
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

            /// 0b0: Security Violation 3 is a non-fatal violation
            pub const SV3_CFG_0: u32 = 0b0;

            /// 0b1: Security Violation 3 is a fatal violation
            pub const SV3_CFG_1: u32 = 0b1;
        }
    }

    /// Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input
    pub mod SV4_CFG {
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

            /// 0b0: Security Violation 4 is a non-fatal violation
            pub const SV4_CFG_0: u32 = 0b0;

            /// 0b1: Security Violation 4 is a fatal violation
            pub const SV4_CFG_1: u32 = 0b1;
        }
    }

    /// Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input
    pub mod SV5_CFG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Security Violation 5 is disabled
            pub const SV5_CFG_0: u32 = 0b00;

            /// 0b01: Security Violation 5 is a non-fatal violation
            pub const SV5_CFG_1: u32 = 0b01;

            /// 0b00: Security Violation 5 is a fatal violation
            pub const SV5_CFG_2: u32 = 0b00;
        }
    }

    /// LP Security Violation Configuration This field configures the LP security violation source.
    pub mod LPSV_CFG {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: LP security violation is disabled
            pub const LPSV_CFG_0: u32 = 0b00;

            /// 0b01: LP security violation is a non-fatal violation
            pub const LPSV_CFG_1: u32 = 0b01;

            /// 0b00: LP security violation is a fatal violation
            pub const LPSV_CFG_2: u32 = 0b00;
        }
    }
}

/// SNVS_HP Status Register
pub mod HPSR {

    /// HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared.
    pub mod HPTA {
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

            /// 0b0: No time alarm interrupt occurred.
            pub const HPTA_0: u32 = 0b0;

            /// 0b1: A time alarm interrupt occurred.
            pub const HPTA_1: u32 = 0b1;
        }
    }

    /// Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared.
    pub mod PI {
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

            /// 0b0: No periodic interrupt occurred.
            pub const PI_0: u32 = 0b0;

            /// 0b1: A periodic interrupt occurred.
            pub const PI_1: u32 = 0b1;
        }
    }

    /// Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS
    pub mod LPDIS {
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

    /// Button Value of the BTN input
    pub mod BTN {
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

    /// Button Interrupt Signal ipi_snvs_btn_int_b was asserted.
    pub mod BI {
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

    /// System Security Monitor State This field contains the encoded state of the SSM's state machine
    pub mod SSM_STATE {
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

            /// 0b0000: Init
            pub const SSM_STATE_0: u32 = 0b0000;

            /// 0b0001: Hard Fail
            pub const SSM_STATE_1: u32 = 0b0001;

            /// 0b0011: Soft Fail
            pub const SSM_STATE_3: u32 = 0b0011;

            /// 0b1000: Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)
            pub const SSM_STATE_8: u32 = 0b1000;

            /// 0b1001: Check
            pub const SSM_STATE_9: u32 = 0b1001;

            /// 0b1011: Non-Secure
            pub const SSM_STATE_11: u32 = 0b1011;

            /// 0b1101: Trusted
            pub const SSM_STATE_13: u32 = 0b1101;

            /// 0b1111: Secure
            pub const SSM_STATE_15: u32 = 0b1111;
        }
    }

    /// One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location
    pub mod OTPMK_SYNDROME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// One Time Programmable Master Key is Equal to Zero
    pub mod OTPMK_ZERO {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The OTPMK is not zero.
            pub const OTPMK_ZERO_0: u32 = 0b0;

            /// 0b1: The OTPMK is zero.
            pub const OTPMK_ZERO_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key is Equal to Zero
    pub mod ZMK_ZERO {
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

            /// 0b0: The ZMK is not zero.
            pub const ZMK_ZERO_0: u32 = 0b0;

            /// 0b1: The ZMK is zero.
            pub const ZMK_ZERO_1: u32 = 0b1;
        }
    }

    /// Security Configuration This field reflects the settings of the sys_secure_boot input and the three security configuration inputs to SNVS
    pub mod SECURITY_CONFIG {
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

            /// 0b0000: FAB Configuration
            pub const FAB_CONFIG: u32 = 0b0000;

            /// 0b0001: OPEN Configuration
            pub const OPEN_CONFIG: u32 = 0b0001;

            /// 0b1001: CLOSED Configuration
            pub const CLOSED_CONFIG: u32 = 0b1001;
        }
    }
}

/// SNVS_HP Security Violation Status Register
pub mod HPSVSR {

    /// Security Violation 0 security violation was detected.
    pub mod SV0 {
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

            /// 0b0: No Security Violation 0 security violation was detected.
            pub const SV0_0: u32 = 0b0;

            /// 0b1: Security Violation 0 security violation was detected.
            pub const SV0_1: u32 = 0b1;
        }
    }

    /// Security Violation 1 security violation was detected.
    pub mod SV1 {
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

            /// 0b0: No Security Violation 1 security violation was detected.
            pub const SV1_0: u32 = 0b0;

            /// 0b1: Security Violation 1 security violation was detected.
            pub const SV1_1: u32 = 0b1;
        }
    }

    /// Security Violation 2 security violation was detected.
    pub mod SV2 {
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

            /// 0b0: No Security Violation 2 security violation was detected.
            pub const SV2_0: u32 = 0b0;

            /// 0b1: Security Violation 2 security violation was detected.
            pub const SV2_1: u32 = 0b1;
        }
    }

    /// Security Violation 3 security violation was detected.
    pub mod SV3 {
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

            /// 0b0: No Security Violation 3 security violation was detected.
            pub const SV3_0: u32 = 0b0;

            /// 0b1: Security Violation 3 security violation was detected.
            pub const SV3_1: u32 = 0b1;
        }
    }

    /// Security Violation 4 security violation was detected.
    pub mod SV4 {
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

            /// 0b0: No Security Violation 4 security violation was detected.
            pub const SV4_0: u32 = 0b0;

            /// 0b1: Security Violation 4 security violation was detected.
            pub const SV4_1: u32 = 0b1;
        }
    }

    /// Security Violation 5 security violation was detected.
    pub mod SV5 {
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

            /// 0b0: No Security Violation 5 security violation was detected.
            pub const SV5_0: u32 = 0b0;

            /// 0b1: Security Violation 5 security violation was detected.
            pub const SV5_1: u32 = 0b1;
        }
    }

    /// Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register
    pub mod SW_SV {
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

    /// Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register
    pub mod SW_FSV {
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

    /// LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register
    pub mod SW_LPSV {
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

    /// Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register
    pub mod ZMK_SYNDROME {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data
    pub mod ZMK_ECC_FAIL {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ZMK ECC Failure was not detected.
            pub const ZMK_ECC_FAIL_0: u32 = 0b0;

            /// 0b1: ZMK ECC Failure was detected.
            pub const ZMK_ECC_FAIL_1: u32 = 0b1;
        }
    }

    /// LP Security Violation A security volation was detected in the SNVS low power section.
    pub mod LP_SEC_VIO {
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

/// SNVS_HP High Assurance Counter IV Register
pub mod HPHACIVR {

    /// High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter
    pub mod HAC_COUNTER_IV {
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

/// SNVS_HP High Assurance Counter Register
pub mod HPHACR {

    /// High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock
    pub mod HAC_COUNTER {
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

/// SNVS_HP Real Time Counter MSB Register
pub mod HPRTCMR {

    /// HP Real Time Counter The most-significant 15 bits of the RTC
    pub mod RTC {
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
}

/// SNVS_HP Real Time Counter LSB Register
pub mod HPRTCLR {

    /// HP Real Time Counter least-significant 32 bits
    pub mod RTC {
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

/// SNVS_HP Time Alarm MSB Register
pub mod HPTAMR {

    /// HP Time Alarm, most-significant 15 bits
    pub mod HPTA_MS {
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
}

/// SNVS_HP Time Alarm LSB Register
pub mod HPTALR {

    /// HP Time Alarm, 32 least-significant bits
    pub mod HPTA_LS {
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

/// SNVS_LP Lock Register
pub mod LPLR {

    /// Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR
    pub mod ZMK_WHL {
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

            /// 0b0: Write access is allowed.
            pub const ZMK_WHL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const ZMK_WHL_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR
    pub mod ZMK_RHL {
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

            /// 0b0: Read access is allowed (only in software programming mode).
            pub const ZMK_RHL_0: u32 = 0b0;

            /// 0b1: Read access is not allowed.
            pub const ZMK_RHL_1: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits
    pub mod SRTC_HL {
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

            /// 0b0: Write access is allowed.
            pub const SRTC_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const SRTC_HL_1: u32 = 0b1;
        }
    }

    /// LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)
    pub mod LPCALB_HL {
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

            /// 0b0: Write access is allowed.
            pub const LPCALB_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const LPCALB_HL_1: u32 = 0b1;
        }
    }

    /// Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit
    pub mod MC_HL {
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

            /// 0b0: Write access (increment) is allowed.
            pub const MC_HL_0: u32 = 0b0;

            /// 0b1: Write access (increment) is not allowed.
            pub const MC_HL_1: u32 = 0b1;
        }
    }

    /// General Purpose Register Hard Lock When set, prevents any writes to the GPR
    pub mod GPR_HL {
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

            /// 0b0: Write access is allowed.
            pub const GPR_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const GPR_HL_1: u32 = 0b1;
        }
    }

    /// LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR
    pub mod LPSVCR_HL {
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

            /// 0b0: Write access is allowed.
            pub const LPSVCR_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const LPSVCR_HL_1: u32 = 0b1;
        }
    }

    /// LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR
    pub mod LPTDCR_HL {
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

            /// 0b0: Write access is allowed.
            pub const LPTDCR_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const LPTDCR_HL_1: u32 = 0b1;
        }
    }

    /// Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register
    pub mod MKS_HL {
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

            /// 0b0: Write access is allowed.
            pub const MKS_HL_0: u32 = 0b0;

            /// 0b1: Write access is not allowed.
            pub const MKS_HL_1: u32 = 0b1;
        }
    }
}

/// SNVS_LP Control Register
pub mod LPCR {

    /// Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational
    pub mod SRTC_ENV {
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

            /// 0b0: SRTC is disabled or invalid.
            pub const SRTC_ENV_0: u32 = 0b0;

            /// 0b1: SRTC is enabled and valid.
            pub const SRTC_ENV_1: u32 = 0b1;
        }
    }

    /// LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter
    pub mod LPTA_EN {
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

            /// 0b0: LP time alarm interrupt is disabled.
            pub const LPTA_EN_0: u32 = 0b0;

            /// 0b1: LP time alarm interrupt is enabled.
            pub const LPTA_EN_1: u32 = 0b1;
        }
    }

    /// Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)
    pub mod MC_ENV {
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

            /// 0b0: MC is disabled or invalid.
            pub const MC_ENV_0: u32 = 0b0;

            /// 0b1: MC is enabled and valid.
            pub const MC_ENV_1: u32 = 0b1;
        }
    }

    /// LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )
    pub mod LPWUI_EN {
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

    /// If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)
    pub mod SRTC_INV_EN {
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

            /// 0b0: SRTC stays valid in the case of security violation.
            pub const SRTC_INV_EN_0: u32 = 0b0;

            /// 0b1: SRTC is invalidated in the case of security violation.
            pub const SRTC_INV_EN_1: u32 = 0b1;
        }
    }

    /// Dumb PMIC Enabled When set, software can control the system power
    pub mod DP_EN {
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

            /// 0b0: Smart PMIC enabled.
            pub const DP_EN_0: u32 = 0b0;

            /// 0b1: Dumb PMIC enabled.
            pub const DP_EN_1: u32 = 0b1;
        }
    }

    /// Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power
    pub mod TOP {
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

            /// 0b0: Leave system power on.
            pub const TOP_0: u32 = 0b0;

            /// 0b1: Turn off system power.
            pub const TOP_1: u32 = 0b1;
        }
    }

    /// Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted
    pub mod PWR_GLITCH_EN {
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

    /// LP Calibration Enable When set, enables the SRTC calibration mechanism
    pub mod LPCALB_EN {
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

            /// 0b0: SRTC Time calibration is disabled.
            pub const LPCALB_EN_0: u32 = 0b0;

            /// 0b1: SRTC Time calibration is enabled.
            pub const LPCALB_EN_1: u32 = 0b1;
        }
    }

    /// LP Calibration Value Defines signed calibration value for SRTC
    pub mod LPCALB_VAL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: +0 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_0: u32 = 0b00000;

            /// 0b00001: +1 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_1: u32 = 0b00001;

            /// 0b00010: +2 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_2: u32 = 0b00010;

            /// 0b01111: +15 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_15: u32 = 0b01111;

            /// 0b10000: -16 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_16: u32 = 0b10000;

            /// 0b10001: -15 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_17: u32 = 0b10001;

            /// 0b11110: -2 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_30: u32 = 0b11110;

            /// 0b11111: -1 counts per each 32768 ticks of the counter clock
            pub const LPCALB_VAL_31: u32 = 0b11111;
        }
    }

    /// This field configures the button press time out values for the PMIC Logic
    pub mod BTN_PRESS_TIME {
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

    /// This field configures the amount of debounce time for the BTN input signal
    pub mod DEBOUNCE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power
    pub mod ON_TIME {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en
    pub mod PK_EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override
    pub mod PK_OVERRIDE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Registers Zeroization Disable
    pub mod GPR_Z_DIS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Master Key Control Register
pub mod LPMKCR {

    /// Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR
    pub mod MASTER_KEY_SEL {
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

            /// 0b00: Select one time programmable master key.
            pub const MASTER_KEY_SEL_0: u32 = 0b00;

            /// 0b10: Select zeroizable master key when MKS_EN bit is set .
            pub const MASTER_KEY_SEL_2: u32 = 0b10;

            /// 0b11: Select combined master key when MKS_EN bit is set .
            pub const MASTER_KEY_SEL_3: u32 = 0b11;
        }
    }

    /// Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it
    pub mod ZMK_HWP {
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

            /// 0b0: ZMK is in the software programming mode.
            pub const ZMK_HWP_0: u32 = 0b0;

            /// 0b1: ZMK is in the hardware programming mode.
            pub const ZMK_HWP_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules
    pub mod ZMK_VAL {
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

            /// 0b0: ZMK is not valid.
            pub const ZMK_VAL_0: u32 = 0b0;

            /// 0b1: ZMK is valid.
            pub const ZMK_VAL_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register
    pub mod ZMK_ECC_EN {
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

            /// 0b0: ZMK ECC check is disabled.
            pub const ZMK_ECC_EN_0: u32 = 0b0;

            /// 0b1: ZMK ECC check is enabled.
            pub const ZMK_ECC_EN_1: u32 = 0b1;
        }
    }

    /// Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register
    pub mod ZMK_ECC_VALUE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (9 bits: 0x1ff << 7)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SNVS_LP Security Violation Control Register
pub mod LPSVCR {

    /// Security Violation 0 Enable This bit enables Security Violation 0 Input
    pub mod SV0_EN {
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

            /// 0b0: Security Violation 0 is disabled in the LP domain.
            pub const SV0_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 0 is enabled in the LP domain.
            pub const SV0_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 1 Enable This bit enables Security Violation 1 Input
    pub mod SV1_EN {
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

            /// 0b0: Security Violation 1 is disabled in the LP domain.
            pub const SV1_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 1 is enabled in the LP domain.
            pub const SV1_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 2 Enable This bit enables Security Violation 2 Input
    pub mod SV2_EN {
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

            /// 0b0: Security Violation 2 is disabled in the LP domain.
            pub const SV2_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 2 is enabled in the LP domain.
            pub const SV2_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 3 Enable This bit enables Security Violation 3 Input
    pub mod SV3_EN {
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

            /// 0b0: Security Violation 3 is disabled in the LP domain.
            pub const SV3_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 3 is enabled in the LP domain.
            pub const SV3_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 4 Enable This bit enables Security Violation 4 Input
    pub mod SV4_EN {
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

            /// 0b0: Security Violation 4 is disabled in the LP domain.
            pub const SV4_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 4 is enabled in the LP domain.
            pub const SV4_EN_1: u32 = 0b1;
        }
    }

    /// Security Violation 5 Enable This bit enables Security Violation 5 Input
    pub mod SV5_EN {
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

            /// 0b0: Security Violation 5 is disabled in the LP domain.
            pub const SV5_EN_0: u32 = 0b0;

            /// 0b1: Security Violation 5 is enabled in the LP domain.
            pub const SV5_EN_1: u32 = 0b1;
        }
    }
}

/// SNVS_LP Tamper Detectors Configuration Register
pub mod LPTDCR {

    /// SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation.
    pub mod SRTCR_EN {
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

            /// 0b0: SRTC rollover is disabled.
            pub const SRTCR_EN_0: u32 = 0b0;

            /// 0b1: SRTC rollover is enabled.
            pub const SRTCR_EN_1: u32 = 0b1;
        }
    }

    /// MC Rollover Enable When set, an MC Rollover event generates an LP security violation.
    pub mod MCR_EN {
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

            /// 0b0: MC rollover is disabled.
            pub const MCR_EN_0: u32 = 0b0;

            /// 0b1: MC rollover is enabled.
            pub const MCR_EN_1: u32 = 0b1;
        }
    }

    /// External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation
    pub mod ET1_EN {
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

            /// 0b0: External tamper 1 is disabled.
            pub const ET1_EN_0: u32 = 0b0;

            /// 0b1: External tamper 1 is enabled.
            pub const ET1_EN_1: u32 = 0b1;
        }
    }

    /// External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1.
    pub mod ET1P {
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

            /// 0b0: External tamper 1 is active low.
            pub const ET1P_0: u32 = 0b0;

            /// 0b1: External tamper 1 is active high.
            pub const ET1P_1: u32 = 0b1;
        }
    }

    /// System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)
    pub mod PFD_OBSERV {
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

    /// Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS
    pub mod POR_OBSERV {
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

    /// Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted
    pub mod OSCB {
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

            /// 0b0: Normal SRTC clock oscillator not bypassed.
            pub const OSCB_0: u32 = 0b0;

            /// 0b1: Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source.
            pub const OSCB_1: u32 = 0b1;
        }
    }
}

/// SNVS_LP Status Register
pub mod LPSR {

    /// LP Time Alarm
    pub mod LPTA {
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

            /// 0b0: No time alarm interrupt occurred.
            pub const LPTA_0: u32 = 0b0;

            /// 0b1: A time alarm interrupt occurred.
            pub const LPTA_1: u32 = 0b1;
        }
    }

    /// Secure Real Time Counter Rollover
    pub mod SRTCR {
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

            /// 0b0: SRTC has not reached its maximum value.
            pub const SRTCR_0: u32 = 0b0;

            /// 0b1: SRTC has reached its maximum value.
            pub const SRTCR_1: u32 = 0b1;
        }
    }

    /// Monotonic Counter Rollover
    pub mod MCR {
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

            /// 0b0: MC has not reached its maximum value.
            pub const MCR_0: u32 = 0b0;

            /// 0b1: MC has reached its maximum value.
            pub const MCR_1: u32 = 0b1;
        }
    }

    /// Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected.
    pub mod PGD {
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

    /// External Tampering 1 Detected
    pub mod ET1D {
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

            /// 0b0: External tampering 1 not detected.
            pub const ET1D_0: u32 = 0b0;

            /// 0b1: External tampering 1 detected.
            pub const ET1D_1: u32 = 0b1;
        }
    }

    /// External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports
    pub mod ESVD {
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

            /// 0b0: No external security violation.
            pub const ESVD_0: u32 = 0b0;

            /// 0b1: External security violation is detected.
            pub const ESVD_1: u32 = 0b1;
        }
    }

    /// Emergency Off This bit is set when a power off is requested.
    pub mod EO {
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

            /// 0b0: Emergency off was not detected.
            pub const EO_0: u32 = 0b0;

            /// 0b1: Emergency off was detected.
            pub const EO_1: u32 = 0b1;
        }
    }

    /// Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time
    pub mod SPO {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Set Power Off was not detected.
            pub const SPO_0: u32 = 0b0;

            /// 0b1: Set Power Off was detected.
            pub const SPO_1: u32 = 0b1;
        }
    }

    /// Scan Exit Detected
    pub mod SED {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Scan exit was not detected.
            pub const SED_0: u32 = 0b0;

            /// 0b1: Scan exit was detected.
            pub const SED_1: u32 = 0b1;
        }
    }

    /// LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state
    pub mod LPNS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LP section was not programmed in the non-secure state.
            pub const LPNS_0: u32 = 0b0;

            /// 0b1: LP section was programmed in the non-secure state.
            pub const LPNS_1: u32 = 0b1;
        }
    }

    /// LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state
    pub mod LPS {
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

            /// 0b0: LP section was not programmed in secure or trusted state.
            pub const LPS_0: u32 = 0b0;

            /// 0b1: LP section was programmed in secure or trusted state.
            pub const LPS_1: u32 = 0b1;
        }
    }
}

/// SNVS_LP Secure Real Time Counter MSB Register
pub mod LPSRTCMR {

    /// LP Secure Real Time Counter The most-significant 15 bits of the SRTC
    pub mod SRTC {
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
}

/// SNVS_LP Secure Real Time Counter LSB Register
pub mod LPSRTCLR {

    /// LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set
    pub mod SRTC {
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

/// SNVS_LP Time Alarm Register
pub mod LPTAR {

    /// LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)
    pub mod LPTA {
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

/// SNVS_LP Secure Monotonic Counter MSB Register
pub mod LPSMCMR {

    /// Monotonic Counter most-significant 16 Bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR register is detected
    pub mod MON_COUNTER {
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

    /// Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses
    pub mod MC_ERA_BITS {
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

/// SNVS_LP Secure Monotonic Counter LSB Register
pub mod LPSMCLR {

    /// Monotonic Counter bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR Register is detected
    pub mod MON_COUNTER {
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

/// SNVS_LP Power Glitch Detector Register
pub mod LPPGDR {

    /// Power Glitch Detector Value
    pub mod PGD {
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

/// SNVS_LP General Purpose Register 0 (legacy alias)
pub mod LPGPR0_legacy_alias {

    /// General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed.
    pub mod GPR {
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

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR0 {

    /// Zeroizable Master Key Each of these registers contains 32 bits of the 256-bit ZMK value
    pub mod ZMK {
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

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR1 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR2 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR3 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR4 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR5 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR6 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP Zeroizable Master Key Register
pub mod LPZMKR7 {
    pub use super::LPZMKR0::ZMK;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias0 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias1 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias2 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR_alias3 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR0 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR1 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR2 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_LP General Purpose Registers 0 .. 3
pub mod LPGPR3 {
    pub use super::LPGPR0_legacy_alias::GPR;
}

/// SNVS_HP Version ID Register 1
pub mod HPVIDR1 {

    /// SNVS block minor version number
    pub mod MINOR_REV {
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

    /// SNVS block major version number
    pub mod MAJOR_REV {
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

    /// SNVS block ID
    pub mod IP_ID {
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

/// SNVS_HP Version ID Register 2
pub mod HPVIDR2 {

    /// SNVS Configuration Options
    pub mod CONFIG_OPT {
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

    /// SNVS ECO Revision
    pub mod ECO_REV {
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

    /// SNVS Integration Options
    pub mod INTG_OPT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5
    pub mod IP_ERA {
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
pub struct RegisterBlock {
    /// SNVS_HP Lock Register
    pub HPLR: RWRegister<u32>,

    /// SNVS_HP Command Register
    pub HPCOMR: RWRegister<u32>,

    /// SNVS_HP Control Register
    pub HPCR: RWRegister<u32>,

    /// SNVS_HP Security Interrupt Control Register
    pub HPSICR: RWRegister<u32>,

    /// SNVS_HP Security Violation Control Register
    pub HPSVCR: RWRegister<u32>,

    /// SNVS_HP Status Register
    pub HPSR: RWRegister<u32>,

    /// SNVS_HP Security Violation Status Register
    pub HPSVSR: RWRegister<u32>,

    /// SNVS_HP High Assurance Counter IV Register
    pub HPHACIVR: RWRegister<u32>,

    /// SNVS_HP High Assurance Counter Register
    pub HPHACR: RORegister<u32>,

    /// SNVS_HP Real Time Counter MSB Register
    pub HPRTCMR: RWRegister<u32>,

    /// SNVS_HP Real Time Counter LSB Register
    pub HPRTCLR: RWRegister<u32>,

    /// SNVS_HP Time Alarm MSB Register
    pub HPTAMR: RWRegister<u32>,

    /// SNVS_HP Time Alarm LSB Register
    pub HPTALR: RWRegister<u32>,

    /// SNVS_LP Lock Register
    pub LPLR: RWRegister<u32>,

    /// SNVS_LP Control Register
    pub LPCR: RWRegister<u32>,

    /// SNVS_LP Master Key Control Register
    pub LPMKCR: RWRegister<u32>,

    /// SNVS_LP Security Violation Control Register
    pub LPSVCR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// SNVS_LP Tamper Detectors Configuration Register
    pub LPTDCR: RWRegister<u32>,

    /// SNVS_LP Status Register
    pub LPSR: RWRegister<u32>,

    /// SNVS_LP Secure Real Time Counter MSB Register
    pub LPSRTCMR: RWRegister<u32>,

    /// SNVS_LP Secure Real Time Counter LSB Register
    pub LPSRTCLR: RWRegister<u32>,

    /// SNVS_LP Time Alarm Register
    pub LPTAR: RWRegister<u32>,

    /// SNVS_LP Secure Monotonic Counter MSB Register
    pub LPSMCMR: RORegister<u32>,

    /// SNVS_LP Secure Monotonic Counter LSB Register
    pub LPSMCLR: RORegister<u32>,

    /// SNVS_LP Power Glitch Detector Register
    pub LPPGDR: RWRegister<u32>,

    /// SNVS_LP General Purpose Register 0 (legacy alias)
    pub LPGPR0_legacy_alias: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR0: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR1: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR2: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR3: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR4: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR5: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR6: RWRegister<u32>,

    /// SNVS_LP Zeroizable Master Key Register
    pub LPZMKR7: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias0: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias1: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias2: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR_alias3: RWRegister<u32>,

    _reserved3: [u32; 24],

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR0: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR1: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR2: RWRegister<u32>,

    /// SNVS_LP General Purpose Registers 0 .. 3
    pub LPGPR3: RWRegister<u32>,

    _reserved4: [u32; 698],

    /// SNVS_HP Version ID Register 1
    pub HPVIDR1: RORegister<u32>,

    /// SNVS_HP Version ID Register 2
    pub HPVIDR2: RORegister<u32>,
}
pub struct ResetValues {
    pub HPLR: u32,
    pub HPCOMR: u32,
    pub HPCR: u32,
    pub HPSICR: u32,
    pub HPSVCR: u32,
    pub HPSR: u32,
    pub HPSVSR: u32,
    pub HPHACIVR: u32,
    pub HPHACR: u32,
    pub HPRTCMR: u32,
    pub HPRTCLR: u32,
    pub HPTAMR: u32,
    pub HPTALR: u32,
    pub LPLR: u32,
    pub LPCR: u32,
    pub LPMKCR: u32,
    pub LPSVCR: u32,
    pub LPTDCR: u32,
    pub LPSR: u32,
    pub LPSRTCMR: u32,
    pub LPSRTCLR: u32,
    pub LPTAR: u32,
    pub LPSMCMR: u32,
    pub LPSMCLR: u32,
    pub LPPGDR: u32,
    pub LPGPR0_legacy_alias: u32,
    pub LPZMKR0: u32,
    pub LPZMKR1: u32,
    pub LPZMKR2: u32,
    pub LPZMKR3: u32,
    pub LPZMKR4: u32,
    pub LPZMKR5: u32,
    pub LPZMKR6: u32,
    pub LPZMKR7: u32,
    pub LPGPR_alias0: u32,
    pub LPGPR_alias1: u32,
    pub LPGPR_alias2: u32,
    pub LPGPR_alias3: u32,
    pub LPGPR0: u32,
    pub LPGPR1: u32,
    pub LPGPR2: u32,
    pub LPGPR3: u32,
    pub HPVIDR1: u32,
    pub HPVIDR2: u32,
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
