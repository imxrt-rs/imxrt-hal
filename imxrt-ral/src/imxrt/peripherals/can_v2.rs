#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXCAN
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Module Configuration Register
pub mod MCR {

    /// This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes
    pub mod MAXMB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below
    pub mod IDAM {
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

            /// 0b00: Format A One full ID (standard or extended) per ID filter Table element.
            pub const IDAM_0: u32 = 0b00;

            /// 0b01: Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element.
            pub const IDAM_1: u32 = 0b01;

            /// 0b10: Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element.
            pub const IDAM_2: u32 = 0b10;

            /// 0b11: Format D All frames rejected.
            pub const IDAM_3: u32 = 0b11;
        }
    }

    /// This bit is supplied for backwards compatibility reasons
    pub mod AEN {
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

            /// 0b0: Abort disabled
            pub const AEN_0: u32 = 0b0;

            /// 0b1: Abort enabled
            pub const AEN_1: u32 = 0b1;
        }
    }

    /// This bit is provided for backwards compatibility reasons
    pub mod LPRIOEN {
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

            /// 0b0: Local Priority disabled
            pub const LPRIOEN_0: u32 = 0b0;

            /// 0b1: Local Priority enabled
            pub const LPRIOEN_1: u32 = 0b1;
        }
    }

    /// This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK
    pub mod IRMQ {
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

            /// 0b0: Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY.
            pub const IRMQ_0: u32 = 0b0;

            /// 0b1: Individual Rx masking and queue feature are enabled.
            pub const IRMQ_1: u32 = 0b1;
        }
    }

    /// This bit defines whether FlexCAN is allowed to receive frames transmitted by itself
    pub mod SRXDIS {
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

            /// 0b0: Self reception enabled
            pub const SRXDIS_0: u32 = 0b0;

            /// 0b1: Self reception disabled
            pub const SRXDIS_1: u32 = 0b1;
        }
    }

    /// This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up
    pub mod WAKSRC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus.
            pub const WAKSRC_0: u32 = 0b0;

            /// 0b1: FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus
            pub const WAKSRC_1: u32 = 0b1;
        }
    }

    /// This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode
    pub mod LPMACK {
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

            /// 0b0: FLEXCAN not in any of the low power modes
            pub const LPMACK_0: u32 = 0b0;

            /// 0b1: FLEXCAN is either in Disable Mode, or Stop mode
            pub const LPMACK_1: u32 = 0b1;
        }
    }

    /// When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register
    pub mod WRNEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters.
            pub const WRNEN_0: u32 = 0b0;

            /// 0b1: TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96.
            pub const WRNEN_1: u32 = 0b1;
        }
    }

    /// This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode
    pub mod SLFWAK {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: FLEXCAN Self Wake Up feature is disabled
            pub const SLFWAK_0: u32 = 0b0;

            /// 0b1: FLEXCAN Self Wake Up feature is enabled
            pub const SLFWAK_1: u32 = 0b1;
        }
    }

    /// This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode
    pub mod SUPV {
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

            /// 0b0: FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses
            pub const SUPV_0: u32 = 0b0;

            /// 0b1: FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location
            pub const SUPV_1: u32 = 0b1;
        }
    }

    /// This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped
    pub mod FRZACK {
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

            /// 0b0: FLEXCAN not in Freeze Mode, prescaler running
            pub const FRZACK_0: u32 = 0b0;

            /// 0b1: FLEXCAN in Freeze Mode, prescaler stopped
            pub const FRZACK_1: u32 = 0b1;
        }
    }

    /// When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers
    pub mod SOFTRST {
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

            /// 0b0: No reset request
            pub const SOFTRST_0: u32 = 0b0;

            /// 0b1: Reset the registers
            pub const SOFTRST_1: u32 = 0b1;
        }
    }

    /// This bit enables the Wake Up Interrupt generation.
    pub mod WAKMSK {
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

            /// 0b0: Wake Up Interrupt is disabled
            pub const WAKMSK_0: u32 = 0b0;

            /// 0b1: Wake Up Interrupt is enabled
            pub const WAKMSK_1: u32 = 0b1;
        }
    }

    /// This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode
    pub mod NOTRDY {
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

            /// 0b0: FLEXCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode
            pub const NOTRDY_0: u32 = 0b0;

            /// 0b1: FLEXCAN module is either in Disable Mode, Stop Mode or Freeze Mode
            pub const NOTRDY_1: u32 = 0b1;
        }
    }

    /// Assertion of this bit puts the FLEXCAN module into Freeze Mode
    pub mod HALT {
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

            /// 0b0: No Freeze Mode request.
            pub const HALT_0: u32 = 0b0;

            /// 0b1: Enters Freeze Mode if the FRZ bit is asserted.
            pub const HALT_1: u32 = 0b1;
        }
    }

    /// This bit controls whether the Rx FIFO feature is enabled or not
    pub mod RFEN {
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

            /// 0b0: FIFO not enabled
            pub const RFEN_0: u32 = 0b0;

            /// 0b1: FIFO enabled
            pub const RFEN_1: u32 = 0b1;
        }
    }

    /// The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level
    pub mod FRZ {
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

            /// 0b0: Not enabled to enter Freeze Mode
            pub const FRZ_0: u32 = 0b0;

            /// 0b1: Enabled to enter Freeze Mode
            pub const FRZ_1: u32 = 0b1;
        }
    }

    /// This bit controls whether FLEXCAN is enabled or not
    pub mod MDIS {
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

            /// 0b0: Enable the FLEXCAN module
            pub const MDIS_0: u32 = 0b0;

            /// 0b1: Disable the FLEXCAN module
            pub const MDIS_1: u32 = 0b1;
        }
    }
}

/// Control 1 Register
pub mod CTRL1 {

    /// This 3-bit field defines the length of the Propagation Segment in the bit time
    pub mod PROPSEG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit configures FLEXCAN to operate in Listen Only Mode
    pub mod LOM {
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

            /// 0b0: Listen Only Mode is deactivated
            pub const LOM_0: u32 = 0b0;

            /// 0b1: FLEXCAN module operates in Listen Only Mode
            pub const LOM_1: u32 = 0b1;
        }
    }

    /// This bit defines the ordering mechanism for Message Buffer transmission
    pub mod LBUF {
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

            /// 0b0: Buffer with highest priority is transmitted first
            pub const LBUF_0: u32 = 0b0;

            /// 0b1: Lowest number buffer is transmitted first
            pub const LBUF_1: u32 = 0b1;
        }
    }

    /// This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0
    pub mod TSYN {
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

            /// 0b0: Timer Sync feature disabled
            pub const TSYN_0: u32 = 0b0;

            /// 0b1: Timer Sync feature enabled
            pub const TSYN_1: u32 = 0b1;
        }
    }

    /// This bit defines how FLEXCAN recovers from Bus Off state
    pub mod BOFFREC {
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

            /// 0b0: Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B
            pub const BOFFREC_0: u32 = 0b0;

            /// 0b1: Automatic recovering from Bus Off state disabled
            pub const BOFFREC_1: u32 = 0b1;
        }
    }

    /// This bit defines the sampling mode of CAN bits at the FLEXCAN_RX
    pub mod SMP {
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

            /// 0b0: Just one sample is used to determine the bit value
            pub const SMP_0: u32 = 0b0;

            /// 0b1: Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used
            pub const SMP_1: u32 = 0b1;
        }
    }

    /// This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register
    pub mod RWRNMSK {
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

            /// 0b0: Rx Warning Interrupt disabled
            pub const RWRNMSK_0: u32 = 0b0;

            /// 0b1: Rx Warning Interrupt enabled
            pub const RWRNMSK_1: u32 = 0b1;
        }
    }

    /// This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register
    pub mod TWRNMSK {
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

            /// 0b0: Tx Warning Interrupt disabled
            pub const TWRNMSK_0: u32 = 0b0;

            /// 0b1: Tx Warning Interrupt enabled
            pub const TWRNMSK_1: u32 = 0b1;
        }
    }

    /// This bit configures FlexCAN to operate in Loop-Back Mode
    pub mod LPB {
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

            /// 0b0: Loop Back disabled
            pub const LPB_0: u32 = 0b0;

            /// 0b1: Loop Back enabled
            pub const LPB_1: u32 = 0b1;
        }
    }

    /// This bit provides a mask for the Error Interrupt.
    pub mod ERRMSK {
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

            /// 0b0: Error interrupt disabled
            pub const ERRMSK_0: u32 = 0b0;

            /// 0b1: Error interrupt enabled
            pub const ERRMSK_1: u32 = 0b1;
        }
    }

    /// This bit provides a mask for the Bus Off Interrupt.
    pub mod BOFFMSK {
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

            /// 0b0: Bus Off interrupt disabled
            pub const BOFFMSK_0: u32 = 0b0;

            /// 0b1: Bus Off interrupt enabled
            pub const BOFFMSK_1: u32 = 0b1;
        }
    }

    /// This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time
    pub mod PSEG2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time
    pub mod PSEG1 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period
    pub mod RJW {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency
    pub mod PRESDIV {
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

/// Free Running Timer Register
pub mod TIMER {

    /// TIMER
    pub mod TIMER {
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

/// Rx Mailboxes Global Mask Register
pub mod RXMGMASK {

    /// These bits mask the Mailbox filter bits as shown in the figure above
    pub mod MG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: the corresponding bit in the filter is "don't care"
            pub const MG_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding bit in the filter is checked against the one received
            pub const MG_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Rx Buffer 14 Mask Register
pub mod RX14MASK {

    /// These bits mask Mailbox 14 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )
    pub mod RX14M {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: the corresponding bit in the filter is "don't care"
            pub const RX14M_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding bit in the filter is checked
            pub const RX14M_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Rx Buffer 15 Mask Register
pub mod RX15MASK {

    /// These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )
    pub mod RX15M {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: the corresponding bit in the filter is "don't care"
            pub const RX15M_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding bit in the filter is checked
            pub const RX15M_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Error Counter Register
pub mod ECR {

    /// Tx_Err_Counter
    pub mod TX_ERR_COUNTER {
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

    /// Rx_Err_Counter
    pub mod RX_ERR_COUNTER {
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

/// Error and Status 1 Register
pub mod ESR1 {

    /// When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm
    pub mod WAKINT {
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

            /// 0b0: No such occurrence
            pub const WAKINT_0: u32 = 0b0;

            /// 0b1: Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode
            pub const WAKINT_1: u32 = 0b1;
        }
    }

    /// This bit indicates that at least one of the Error Bits (bits 15-10) is set
    pub mod ERRINT {
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

            /// 0b0: No such occurrence
            pub const ERRINT_0: u32 = 0b0;

            /// 0b1: Indicates setting of any Error Bit in the Error and Status Register
            pub const ERRINT_1: u32 = 0b1;
        }
    }

    /// This bit is set when FLEXCAN enters 'Bus Off' state
    pub mod BOFFINT {
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

            /// 0b0: No such occurrence
            pub const BOFFINT_0: u32 = 0b0;

            /// 0b1: FLEXCAN module entered 'Bus Off' state
            pub const BOFFINT_1: u32 = 0b1;
        }
    }

    /// This bit indicates if FlexCAN is receiving a message. Refer to .
    pub mod RX {
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

            /// 0b0: FLEXCAN is receiving a message
            pub const RX_0: u32 = 0b0;

            /// 0b1: FLEXCAN is transmitting a message
            pub const RX_1: u32 = 0b1;
        }
    }

    /// If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate "Error Passive"
    pub mod FLTCONF {
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

            /// 0b00: Error Active
            pub const FLTCONF_0: u32 = 0b00;

            /// 0b01: Error Passive
            pub const FLTCONF_1: u32 = 0b01;

            /// 0b00: Bus off
            pub const FLTCONF_2: u32 = 0b00;
        }
    }

    /// This bit indicates if FLEXCAN is transmitting a message.Refer to .
    pub mod TX {
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

            /// 0b0: FLEXCAN is receiving a message
            pub const TX_0: u32 = 0b0;

            /// 0b1: FLEXCAN is transmitting a message
            pub const TX_1: u32 = 0b1;
        }
    }

    /// This bit indicates when CAN bus is in IDLE state.Refer to .
    pub mod IDLE {
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

            /// 0b0: No such occurrence
            pub const IDLE_0: u32 = 0b0;

            /// 0b1: CAN bus is now IDLE
            pub const IDLE_1: u32 = 0b1;
        }
    }

    /// This bit indicates when repetitive errors are occurring during message reception.
    pub mod RXWRN {
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

            /// 0b0: No such occurrence
            pub const RXWRN_0: u32 = 0b0;

            /// 0b1: Rx_Err_Counter >= 96
            pub const RXWRN_1: u32 = 0b1;
        }
    }

    /// This bit indicates when repetitive errors are occurring during message transmission.
    pub mod TXWRN {
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

            /// 0b0: No such occurrence
            pub const TXWRN_0: u32 = 0b0;

            /// 0b1: TX_Err_Counter >= 96
            pub const TXWRN_1: u32 = 0b1;
        }
    }

    /// This bit indicates that a Stuffing Error has been detected.
    pub mod STFERR {
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

            /// 0b0: No such occurrence.
            pub const STFERR_0: u32 = 0b0;

            /// 0b1: A Stuffing Error occurred since last read of this register.
            pub const STFERR_1: u32 = 0b1;
        }
    }

    /// This bit indicates that a Form Error has been detected by the receiver node, i
    pub mod FRMERR {
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

            /// 0b0: No such occurrence
            pub const FRMERR_0: u32 = 0b0;

            /// 0b1: A Form Error occurred since last read of this register
            pub const FRMERR_1: u32 = 0b1;
        }
    }

    /// This bit indicates that a CRC Error has been detected by the receiver node, i
    pub mod CRCERR {
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

            /// 0b0: No such occurrence
            pub const CRCERR_0: u32 = 0b0;

            /// 0b1: A CRC error occurred since last read of this register.
            pub const CRCERR_1: u32 = 0b1;
        }
    }

    /// This bit indicates that an Acknowledge Error has been detected by the transmitter node, i
    pub mod ACKERR {
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

            /// 0b0: No such occurrence
            pub const ACKERR_0: u32 = 0b0;

            /// 0b1: An ACK error occurred since last read of this register
            pub const ACKERR_1: u32 = 0b1;
        }
    }

    /// This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message
    pub mod BIT0ERR {
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

            /// 0b0: No such occurrence
            pub const BIT0ERR_0: u32 = 0b0;

            /// 0b1: At least one bit sent as dominant is received as recessive
            pub const BIT0ERR_1: u32 = 0b1;
        }
    }

    /// This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message
    pub mod BIT1ERR {
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

            /// 0b0: No such occurrence
            pub const BIT1ERR_0: u32 = 0b0;

            /// 0b1: At least one bit sent as recessive is received as dominant
            pub const BIT1ERR_1: u32 = 0b1;
        }
    }

    /// If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96
    pub mod RWRNINT {
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

            /// 0b0: No such occurrence
            pub const RWRNINT_0: u32 = 0b0;

            /// 0b1: The Rx error counter transition from < 96 to >= 96
            pub const RWRNINT_1: u32 = 0b1;
        }
    }

    /// If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96
    pub mod TWRNINT {
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

            /// 0b0: No such occurrence
            pub const TWRNINT_0: u32 = 0b0;

            /// 0b1: The Tx error counter transition from < 96 to >= 96
            pub const TWRNINT_1: u32 = 0b1;
        }
    }

    /// This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process
    pub mod SYNCH {
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

            /// 0b0: FlexCAN is not synchronized to the CAN bus
            pub const SYNCH_0: u32 = 0b0;

            /// 0b1: FlexCAN is synchronized to the CAN bus
            pub const SYNCH_1: u32 = 0b1;
        }
    }
}

/// Interrupt Masks 2 Register
pub mod IMASK2 {

    /// Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt
    pub mod BUFHM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: The corresponding buffer Interrupt is disabled
            pub const BUFHM_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding buffer Interrupt is enabled
            pub const BUFHM_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Interrupt Masks 1 Register
pub mod IMASK1 {

    /// Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt
    pub mod BUFLM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: The corresponding buffer Interrupt is disabled
            pub const BUFLM_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding buffer Interrupt is enabled
            pub const BUFLM_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Interrupt Flags 2 Register
pub mod IFLAG2 {

    /// Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt.
    pub mod BUFHI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: No such occurrence
            pub const BUFHI_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding buffer has successfully completed transmission or reception
            pub const BUFHI_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Interrupt Flags 1 Register
pub mod IFLAG1 {

    /// If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4
    pub mod BUF4TO0I {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: No such occurrence
            pub const BUF4TO0I_0: u32 = 0b00000;

            /// 0b00001: Corresponding MB completed transmission/reception
            pub const BUF4TO0I_1: u32 = 0b00001;
        }
    }

    /// If the Rx FIFO is not enabled, this bit flags the interrupt for MB5
    pub mod BUF5I {
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

            /// 0b0: No such occurrence
            pub const BUF5I_0: u32 = 0b0;

            /// 0b1: MB5 completed transmission/reception or frames available in the FIFO
            pub const BUF5I_1: u32 = 0b1;
        }
    }

    /// If the Rx FIFO is not enabled, this bit flags the interrupt for MB6
    pub mod BUF6I {
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

            /// 0b0: No such occurrence
            pub const BUF6I_0: u32 = 0b0;

            /// 0b1: MB6 completed transmission/reception or FIFO almost full
            pub const BUF6I_1: u32 = 0b1;
        }
    }

    /// If the Rx FIFO is not enabled, this bit flags the interrupt for MB7
    pub mod BUF7I {
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

            /// 0b0: No such occurrence
            pub const BUF7I_0: u32 = 0b0;

            /// 0b1: MB7 completed transmission/reception or FIFO overflow
            pub const BUF7I_1: u32 = 0b1;
        }
    }

    /// Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt.
    pub mod BUF31TO8I {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000000000000000000000: No such occurrence
            pub const BUF31TO8I_0: u32 = 0b000000000000000000000000;

            /// 0b000000000000000000000001: The corresponding MB has successfully completed transmission or reception
            pub const BUF31TO8I_1: u32 = 0b000000000000000000000001;
        }
    }
}

/// Control 2 Register
pub mod CTRL2 {

    /// This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process
    pub mod EACEN {
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

            /// 0b0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits.
            pub const EACEN_0: u32 = 0b0;

            /// 0b1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply.
            pub const EACEN_1: u32 = 0b1;
        }
    }

    /// If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame
    pub mod RRS {
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

            /// 0b0: Remote Response Frame is generated
            pub const RRS_0: u32 = 0b0;

            /// 0b1: Remote Request Frame is stored
            pub const RRS_1: u32 = 0b1;
        }
    }

    /// If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO
    pub mod MRP {
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

            /// 0b0: Matching starts from Rx FIFO and continues on Mailboxes
            pub const MRP_0: u32 = 0b0;

            /// 0b1: Matching starts from Mailboxes and continues on Rx FIFO
            pub const MRP_1: u32 = 0b1;
        }
    }

    /// This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus
    pub mod TASD {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This 4-bit field defines the number of Rx FIFO filters according to
    pub mod RFFN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable unrestricted write access to FlexCAN memory in Freeze mode
    pub mod WRMFRZ {
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

            /// 0b0: Keep the write access restricted in some regions of FlexCAN memory
            pub const WRMFRZ_0: u32 = 0b0;

            /// 0b1: Enable unrestricted write access to FlexCAN memory
            pub const WRMFRZ_1: u32 = 0b1;
        }
    }
}

/// Error and Status 2 Register
pub mod ESR2 {

    /// If ESR2\[VPS\] is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)
    pub mod IMB {
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

            /// 0b0: If ESR2\[VPS\] is asserted, the ESR2\[LPTM\] is not an inactive Mailbox.
            pub const IMB_0: u32 = 0b0;

            /// 0b1: If ESR2\[VPS\] is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one.
            pub const IMB_1: u32 = 0b1;
        }
    }

    /// This bit indicates whether IMB and LPTM contents are currently valid or not
    pub mod VPS {
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

            /// 0b0: Contents of IMB and LPTM are invalid
            pub const VPS_0: u32 = 0b0;

            /// 0b1: Contents of IMB and LPTM are valid
            pub const VPS_1: u32 = 0b1;
        }
    }

    /// If ESR2\[VPS\] is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)
    pub mod LPTM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CRC Register
pub mod CRCR {

    /// This field indicates the CRC value of the last message transmitted
    pub mod TXCRC {
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

    /// This field indicates the number of the Mailbox corresponding to the value in TXCRC field.
    pub mod MBCRC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Rx FIFO Global Mask Register
pub mod RXFGMASK {

    /// These bits mask the ID Filter Table elements bits in a perfect alignment
    pub mod FGM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: The corresponding bit in the filter is "don't care"
            pub const FGM_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding bit in the filter is checked
            pub const FGM_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Rx FIFO Information Register
pub mod RXFIR {

    /// This 9-bit field indicates which Identifier Acceptance Filter (see Rx FIFO Structure) was hit by the received message that is in the output of the Rx FIFO
    pub mod IDHIT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Debug 1 register
pub mod DBG1 {

    /// CAN Finite State Machine
    pub mod CFSM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CAN Bit Number
    pub mod CBN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Debug 2 register
pub mod DBG2 {

    /// Rx Matching Pointer
    pub mod RMP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Matching Process in Progress
    pub mod MPP {
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

            /// 0b0: No matching process ongoing.
            pub const MPP_0: u32 = 0b0;

            /// 0b1: Matching process is in progress.
            pub const MPP_1: u32 = 0b1;
        }
    }

    /// Tx Arbitration Pointer
    pub mod TAP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arbitration Process in Progress
    pub mod APP {
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

            /// 0b0: No matching process ongoing.
            pub const APP_0: u32 = 0b0;

            /// 0b1: Matching process is in progress.
            pub const APP_1: u32 = 0b1;
        }
    }
}

/// Rx Individual Mask Registers
pub mod RXIMR0 {

    /// These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways
    pub mod MI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000000000000000000000: the corresponding bit in the filter is "don't care"
            pub const MI_0: u32 = 0b00000000000000000000000000000000;

            /// 0b00000000000000000000000000000001: The corresponding bit in the filter is checked
            pub const MI_1: u32 = 0b00000000000000000000000000000001;
        }
    }
}

/// Rx Individual Mask Registers
pub mod RXIMR1 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR2 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR3 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR4 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR5 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR6 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR7 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR8 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR9 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR10 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR11 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR12 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR13 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR14 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR15 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR16 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR17 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR18 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR19 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR20 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR21 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR22 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR23 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR24 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR25 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR26 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR27 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR28 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR29 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR30 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR31 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR32 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR33 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR34 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR35 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR36 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR37 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR38 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR39 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR40 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR41 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR42 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR43 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR44 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR45 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR46 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR47 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR48 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR49 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR50 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR51 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR52 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR53 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR54 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR55 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR56 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR57 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR58 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR59 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR60 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR61 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR62 {
    pub use super::RXIMR0::MI;
}

/// Rx Individual Mask Registers
pub mod RXIMR63 {
    pub use super::RXIMR0::MI;
}

/// Glitch Filter Width Registers
pub mod GFWR {

    /// It determines the Glitch Filter Width
    pub mod GFWR {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// Module Configuration Register
    pub MCR: RWRegister<u32>,

    /// Control 1 Register
    pub CTRL1: RWRegister<u32>,

    /// Free Running Timer Register
    pub TIMER: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Rx Mailboxes Global Mask Register
    pub RXMGMASK: RWRegister<u32>,

    /// Rx Buffer 14 Mask Register
    pub RX14MASK: RWRegister<u32>,

    /// Rx Buffer 15 Mask Register
    pub RX15MASK: RWRegister<u32>,

    /// Error Counter Register
    pub ECR: RWRegister<u32>,

    /// Error and Status 1 Register
    pub ESR1: RWRegister<u32>,

    /// Interrupt Masks 2 Register
    pub IMASK2: RWRegister<u32>,

    /// Interrupt Masks 1 Register
    pub IMASK1: RWRegister<u32>,

    /// Interrupt Flags 2 Register
    pub IFLAG2: RWRegister<u32>,

    /// Interrupt Flags 1 Register
    pub IFLAG1: RWRegister<u32>,

    /// Control 2 Register
    pub CTRL2: RWRegister<u32>,

    /// Error and Status 2 Register
    pub ESR2: RORegister<u32>,

    _reserved2: [u32; 2],

    /// CRC Register
    pub CRCR: RORegister<u32>,

    /// Rx FIFO Global Mask Register
    pub RXFGMASK: RWRegister<u32>,

    /// Rx FIFO Information Register
    pub RXFIR: RORegister<u32>,

    _reserved3: [u32; 2],

    /// Debug 1 register
    pub DBG1: RORegister<u32>,

    /// Debug 2 register
    pub DBG2: RORegister<u32>,

    _reserved4: [u32; 520],

    /// Rx Individual Mask Registers
    pub RXIMR0: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR1: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR2: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR3: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR4: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR5: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR6: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR7: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR8: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR9: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR10: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR11: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR12: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR13: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR14: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR15: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR16: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR17: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR18: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR19: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR20: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR21: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR22: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR23: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR24: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR25: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR26: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR27: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR28: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR29: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR30: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR31: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR32: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR33: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR34: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR35: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR36: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR37: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR38: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR39: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR40: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR41: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR42: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR43: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR44: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR45: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR46: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR47: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR48: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR49: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR50: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR51: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR52: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR53: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR54: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR55: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR56: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR57: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR58: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR59: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR60: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR61: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR62: RWRegister<u32>,

    /// Rx Individual Mask Registers
    pub RXIMR63: RWRegister<u32>,

    _reserved5: [u32; 24],

    /// Glitch Filter Width Registers
    pub GFWR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub CTRL1: u32,
    pub TIMER: u32,
    pub RXMGMASK: u32,
    pub RX14MASK: u32,
    pub RX15MASK: u32,
    pub ECR: u32,
    pub ESR1: u32,
    pub IMASK2: u32,
    pub IMASK1: u32,
    pub IFLAG2: u32,
    pub IFLAG1: u32,
    pub CTRL2: u32,
    pub ESR2: u32,
    pub CRCR: u32,
    pub RXFGMASK: u32,
    pub RXFIR: u32,
    pub DBG1: u32,
    pub DBG2: u32,
    pub RXIMR0: u32,
    pub RXIMR1: u32,
    pub RXIMR2: u32,
    pub RXIMR3: u32,
    pub RXIMR4: u32,
    pub RXIMR5: u32,
    pub RXIMR6: u32,
    pub RXIMR7: u32,
    pub RXIMR8: u32,
    pub RXIMR9: u32,
    pub RXIMR10: u32,
    pub RXIMR11: u32,
    pub RXIMR12: u32,
    pub RXIMR13: u32,
    pub RXIMR14: u32,
    pub RXIMR15: u32,
    pub RXIMR16: u32,
    pub RXIMR17: u32,
    pub RXIMR18: u32,
    pub RXIMR19: u32,
    pub RXIMR20: u32,
    pub RXIMR21: u32,
    pub RXIMR22: u32,
    pub RXIMR23: u32,
    pub RXIMR24: u32,
    pub RXIMR25: u32,
    pub RXIMR26: u32,
    pub RXIMR27: u32,
    pub RXIMR28: u32,
    pub RXIMR29: u32,
    pub RXIMR30: u32,
    pub RXIMR31: u32,
    pub RXIMR32: u32,
    pub RXIMR33: u32,
    pub RXIMR34: u32,
    pub RXIMR35: u32,
    pub RXIMR36: u32,
    pub RXIMR37: u32,
    pub RXIMR38: u32,
    pub RXIMR39: u32,
    pub RXIMR40: u32,
    pub RXIMR41: u32,
    pub RXIMR42: u32,
    pub RXIMR43: u32,
    pub RXIMR44: u32,
    pub RXIMR45: u32,
    pub RXIMR46: u32,
    pub RXIMR47: u32,
    pub RXIMR48: u32,
    pub RXIMR49: u32,
    pub RXIMR50: u32,
    pub RXIMR51: u32,
    pub RXIMR52: u32,
    pub RXIMR53: u32,
    pub RXIMR54: u32,
    pub RXIMR55: u32,
    pub RXIMR56: u32,
    pub RXIMR57: u32,
    pub RXIMR58: u32,
    pub RXIMR59: u32,
    pub RXIMR60: u32,
    pub RXIMR61: u32,
    pub RXIMR62: u32,
    pub RXIMR63: u32,
    pub GFWR: u32,
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
