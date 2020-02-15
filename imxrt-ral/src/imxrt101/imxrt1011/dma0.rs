#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control Register
pub mod CR {

    /// Enable Debug
    pub mod EDBG {
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

            /// 0b0: When in debug mode, the DMA continues to operate.
            pub const EDBG_0: u32 = 0b0;

            /// 0b1: When in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete. Channel execution resumes when the system exits debug mode or the EDBG bit is cleared.
            pub const EDBG_1: u32 = 0b1;
        }
    }

    /// Enable Round Robin Channel Arbitration
    pub mod ERCA {
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

            /// 0b0: Fixed priority arbitration is used for channel selection .
            pub const ERCA_0: u32 = 0b0;

            /// 0b1: Round robin arbitration is used for channel selection .
            pub const ERCA_1: u32 = 0b1;
        }
    }

    /// Halt On Error
    pub mod HOE {
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

            /// 0b0: Normal operation
            pub const HOE_0: u32 = 0b0;

            /// 0b1: Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared.
            pub const HOE_1: u32 = 0b1;
        }
    }

    /// Halt DMA Operations
    pub mod HALT {
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

            /// 0b0: Normal operation
            pub const HALT_0: u32 = 0b0;

            /// 0b1: Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared.
            pub const HALT_1: u32 = 0b1;
        }
    }

    /// Continuous Link Mode
    pub mod CLM {
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

            /// 0b0: A minor loop channel link made to itself goes through channel arbitration before being activated again.
            pub const CLM_0: u32 = 0b0;

            /// 0b1: A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop.
            pub const CLM_1: u32 = 0b1;
        }
    }

    /// Enable Minor Loop Mapping
    pub mod EMLM {
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

            /// 0b0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field.
            pub const EMLM_0: u32 = 0b0;

            /// 0b1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled.
            pub const EMLM_1: u32 = 0b1;
        }
    }

    /// Error Cancel Transfer
    pub mod ECX {
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

            /// 0b0: Normal operation
            pub const ECX_0: u32 = 0b0;

            /// 0b1: Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt.
            pub const ECX_1: u32 = 0b1;
        }
    }

    /// Cancel Transfer
    pub mod CX {
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

            /// 0b0: Normal operation
            pub const CX_0: u32 = 0b0;

            /// 0b1: Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed.
            pub const CX_1: u32 = 0b1;
        }
    }

    /// DMA Active Status
    pub mod ACTIVE {
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

            /// 0b0: eDMA is idle.
            pub const ACTIVE_0: u32 = 0b0;

            /// 0b1: eDMA is executing a channel.
            pub const ACTIVE_1: u32 = 0b1;
        }
    }
}

/// Error Status Register
pub mod ES {

    /// Destination Bus Error
    pub mod DBE {
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

            /// 0b0: No destination bus error
            pub const DBE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a bus error on a destination write
            pub const DBE_1: u32 = 0b1;
        }
    }

    /// Source Bus Error
    pub mod SBE {
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

            /// 0b0: No source bus error
            pub const SBE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a bus error on a source read
            pub const SBE_1: u32 = 0b1;
        }
    }

    /// Scatter/Gather Configuration Error
    pub mod SGE {
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

            /// 0b0: No scatter/gather configuration error
            pub const SGE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\[ESG\] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary.
            pub const SGE_1: u32 = 0b1;
        }
    }

    /// NBYTES/CITER Configuration Error
    pub mod NCE {
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

            /// 0b0: No NBYTES/CITER configuration error
            pub const NCE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\[SSIZE\] and TCDn_ATTR\[DSIZE\], or TCDn_CITER\[CITER\] is equal to zero, or TCDn_CITER\[ELINK\] is not equal to TCDn_BITER\[ELINK\]
            pub const NCE_1: u32 = 0b1;
        }
    }

    /// Destination Offset Error
    pub mod DOE {
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

            /// 0b0: No destination offset configuration error
            pub const DOE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const DOE_1: u32 = 0b1;
        }
    }

    /// Destination Address Error
    pub mod DAE {
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

            /// 0b0: No destination address configuration error
            pub const DAE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\[DSIZE\].
            pub const DAE_1: u32 = 0b1;
        }
    }

    /// Source Offset Error
    pub mod SOE {
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

            /// 0b0: No source offset configuration error
            pub const SOE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const SOE_1: u32 = 0b1;
        }
    }

    /// Source Address Error
    pub mod SAE {
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

            /// 0b0: No source address configuration error.
            pub const SAE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\[SSIZE\].
            pub const SAE_1: u32 = 0b1;
        }
    }

    /// Error Channel Number or Canceled Channel Number
    pub mod ERRCHN {
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

    /// Channel Priority Error
    pub mod CPE {
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

            /// 0b0: No channel priority error
            pub const CPE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error in the channel priorities . Channel priorities are not unique.
            pub const CPE_1: u32 = 0b1;
        }
    }

    /// Transfer Canceled
    pub mod ECX {
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

            /// 0b0: No canceled transfers
            pub const ECX_0: u32 = 0b0;

            /// 0b1: The last recorded entry was a canceled transfer by the error cancel transfer input
            pub const ECX_1: u32 = 0b1;
        }
    }

    /// VLD
    pub mod VLD {
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

            /// 0b0: No ERR bits are set.
            pub const VLD_0: u32 = 0b0;

            /// 0b1: At least one ERR bit is set indicating a valid error exists that has not been cleared.
            pub const VLD_1: u32 = 0b1;
        }
    }
}

/// Enable Request Register
pub mod ERQ {

    /// Enable DMA Request 0
    pub mod ERQ0 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ0_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ0_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 1
    pub mod ERQ1 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ1_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ1_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 2
    pub mod ERQ2 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ2_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ2_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 3
    pub mod ERQ3 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ3_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ3_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 4
    pub mod ERQ4 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ4_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ4_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 5
    pub mod ERQ5 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ5_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ5_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 6
    pub mod ERQ6 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ6_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ6_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 7
    pub mod ERQ7 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ7_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ7_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 8
    pub mod ERQ8 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ8_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ8_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 9
    pub mod ERQ9 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ9_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ9_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 10
    pub mod ERQ10 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ10_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ10_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 11
    pub mod ERQ11 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ11_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ11_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 12
    pub mod ERQ12 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ12_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ12_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 13
    pub mod ERQ13 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ13_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ13_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 14
    pub mod ERQ14 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ14_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ14_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 15
    pub mod ERQ15 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ15_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ15_1: u32 = 0b1;
        }
    }
}

/// Enable Error Interrupt Register
pub mod EEI {

    /// Enable Error Interrupt 0
    pub mod EEI0 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI0_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI0_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 1
    pub mod EEI1 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI1_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI1_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 2
    pub mod EEI2 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI2_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI2_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 3
    pub mod EEI3 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI3_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI3_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 4
    pub mod EEI4 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI4_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI4_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 5
    pub mod EEI5 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI5_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI5_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 6
    pub mod EEI6 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI6_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI6_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 7
    pub mod EEI7 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI7_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI7_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 8
    pub mod EEI8 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI8_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI8_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 9
    pub mod EEI9 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI9_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI9_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 10
    pub mod EEI10 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI10_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI10_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 11
    pub mod EEI11 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI11_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI11_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 12
    pub mod EEI12 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI12_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI12_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 13
    pub mod EEI13 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI13_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI13_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 14
    pub mod EEI14 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI14_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI14_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 15
    pub mod EEI15 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI15_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI15_1: u32 = 0b1;
        }
    }
}

/// Clear Enable Error Interrupt Register
pub mod CEEI {

    /// Clear Enable Error Interrupt
    pub mod CEEI {
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

    /// Clear All Enable Error Interrupts
    pub mod CAEE {
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

            /// 0b0: Clear only the EEI bit specified in the CEEI field
            pub const CAEE_0: u32 = 0b0;

            /// 0b1: Clear all bits in EEI
            pub const CAEE_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Set Enable Error Interrupt Register
pub mod SEEI {

    /// Set Enable Error Interrupt
    pub mod SEEI {
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

    /// Sets All Enable Error Interrupts
    pub mod SAEE {
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

            /// 0b0: Set only the EEI bit specified in the SEEI field.
            pub const SAEE_0: u32 = 0b0;

            /// 0b1: Sets all bits in EEI
            pub const SAEE_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Clear Enable Request Register
pub mod CERQ {

    /// Clear Enable Request
    pub mod CERQ {
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

    /// Clear All Enable Requests
    pub mod CAER {
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

            /// 0b0: Clear only the ERQ bit specified in the CERQ field
            pub const CAER_0: u32 = 0b0;

            /// 0b1: Clear all bits in ERQ
            pub const CAER_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Set Enable Request Register
pub mod SERQ {

    /// Set Enable Request
    pub mod SERQ {
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

    /// Set All Enable Requests
    pub mod SAER {
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

            /// 0b0: Set only the ERQ bit specified in the SERQ field
            pub const SAER_0: u32 = 0b0;

            /// 0b1: Set all bits in ERQ
            pub const SAER_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Clear DONE Status Bit Register
pub mod CDNE {

    /// Clear DONE Bit
    pub mod CDNE {
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

    /// Clears All DONE Bits
    pub mod CADN {
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

            /// 0b0: Clears only the TCDn_CSR\[DONE\] bit specified in the CDNE field
            pub const CADN_0: u32 = 0b0;

            /// 0b1: Clears all bits in TCDn_CSR\[DONE\]
            pub const CADN_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Set START Bit Register
pub mod SSRT {

    /// Set START Bit
    pub mod SSRT {
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

    /// Set All START Bits (activates all channels)
    pub mod SAST {
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

            /// 0b0: Set only the TCDn_CSR\[START\] bit specified in the SSRT field
            pub const SAST_0: u32 = 0b0;

            /// 0b1: Set all bits in TCDn_CSR\[START\]
            pub const SAST_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Clear Error Register
pub mod CERR {

    /// Clear Error Indicator
    pub mod CERR {
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

    /// Clear All Error Indicators
    pub mod CAEI {
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

            /// 0b0: Clear only the ERR bit specified in the CERR field
            pub const CAEI_0: u32 = 0b0;

            /// 0b1: Clear all bits in ERR
            pub const CAEI_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Clear Interrupt Request Register
pub mod CINT {

    /// Clear Interrupt Request
    pub mod CINT {
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

    /// Clear All Interrupt Requests
    pub mod CAIR {
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

            /// 0b0: Clear only the INT bit specified in the CINT field
            pub const CAIR_0: u32 = 0b0;

            /// 0b1: Clear all bits in INT
            pub const CAIR_1: u32 = 0b1;
        }
    }

    /// No Op enable
    pub mod NOP {
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

            /// 0b0: Normal operation
            pub const NOP_0: u32 = 0b0;

            /// 0b1: No operation, ignore the other bits in this register
            pub const NOP_1: u32 = 0b1;
        }
    }
}

/// Interrupt Request Register
pub mod INT {

    /// Interrupt Request 0
    pub mod INT0 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT0_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT0_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 1
    pub mod INT1 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT1_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT1_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 2
    pub mod INT2 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT2_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT2_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 3
    pub mod INT3 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT3_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT3_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 4
    pub mod INT4 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT4_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT4_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 5
    pub mod INT5 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT5_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT5_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 6
    pub mod INT6 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT6_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT6_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 7
    pub mod INT7 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT7_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT7_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 8
    pub mod INT8 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT8_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT8_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 9
    pub mod INT9 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT9_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT9_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 10
    pub mod INT10 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT10_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT10_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 11
    pub mod INT11 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT11_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT11_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 12
    pub mod INT12 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT12_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT12_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 13
    pub mod INT13 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT13_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT13_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 14
    pub mod INT14 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT14_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT14_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 15
    pub mod INT15 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT15_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT15_1: u32 = 0b1;
        }
    }
}

/// Error Register
pub mod ERR {

    /// Error In Channel 0
    pub mod ERR0 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR0_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR0_1: u32 = 0b1;
        }
    }

    /// Error In Channel 1
    pub mod ERR1 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR1_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR1_1: u32 = 0b1;
        }
    }

    /// Error In Channel 2
    pub mod ERR2 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR2_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR2_1: u32 = 0b1;
        }
    }

    /// Error In Channel 3
    pub mod ERR3 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR3_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR3_1: u32 = 0b1;
        }
    }

    /// Error In Channel 4
    pub mod ERR4 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR4_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR4_1: u32 = 0b1;
        }
    }

    /// Error In Channel 5
    pub mod ERR5 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR5_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR5_1: u32 = 0b1;
        }
    }

    /// Error In Channel 6
    pub mod ERR6 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR6_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR6_1: u32 = 0b1;
        }
    }

    /// Error In Channel 7
    pub mod ERR7 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR7_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR7_1: u32 = 0b1;
        }
    }

    /// Error In Channel 8
    pub mod ERR8 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR8_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR8_1: u32 = 0b1;
        }
    }

    /// Error In Channel 9
    pub mod ERR9 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR9_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR9_1: u32 = 0b1;
        }
    }

    /// Error In Channel 10
    pub mod ERR10 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR10_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR10_1: u32 = 0b1;
        }
    }

    /// Error In Channel 11
    pub mod ERR11 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR11_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR11_1: u32 = 0b1;
        }
    }

    /// Error In Channel 12
    pub mod ERR12 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR12_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR12_1: u32 = 0b1;
        }
    }

    /// Error In Channel 13
    pub mod ERR13 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR13_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR13_1: u32 = 0b1;
        }
    }

    /// Error In Channel 14
    pub mod ERR14 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR14_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR14_1: u32 = 0b1;
        }
    }

    /// Error In Channel 15
    pub mod ERR15 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR15_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR15_1: u32 = 0b1;
        }
    }
}

/// Hardware Request Status Register
pub mod HRS {

    /// Hardware Request Status Channel 0
    pub mod HRS0 {
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

            /// 0b0: A hardware service request for channel 0 is not present
            pub const HRS0_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 0 is present
            pub const HRS0_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 1
    pub mod HRS1 {
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

            /// 0b0: A hardware service request for channel 1 is not present
            pub const HRS1_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 1 is present
            pub const HRS1_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 2
    pub mod HRS2 {
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

            /// 0b0: A hardware service request for channel 2 is not present
            pub const HRS2_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 2 is present
            pub const HRS2_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 3
    pub mod HRS3 {
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

            /// 0b0: A hardware service request for channel 3 is not present
            pub const HRS3_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 3 is present
            pub const HRS3_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 4
    pub mod HRS4 {
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

            /// 0b0: A hardware service request for channel 4 is not present
            pub const HRS4_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 4 is present
            pub const HRS4_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 5
    pub mod HRS5 {
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

            /// 0b0: A hardware service request for channel 5 is not present
            pub const HRS5_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 5 is present
            pub const HRS5_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 6
    pub mod HRS6 {
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

            /// 0b0: A hardware service request for channel 6 is not present
            pub const HRS6_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 6 is present
            pub const HRS6_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 7
    pub mod HRS7 {
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

            /// 0b0: A hardware service request for channel 7 is not present
            pub const HRS7_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 7 is present
            pub const HRS7_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 8
    pub mod HRS8 {
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

            /// 0b0: A hardware service request for channel 8 is not present
            pub const HRS8_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 8 is present
            pub const HRS8_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 9
    pub mod HRS9 {
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

            /// 0b0: A hardware service request for channel 9 is not present
            pub const HRS9_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 9 is present
            pub const HRS9_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 10
    pub mod HRS10 {
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

            /// 0b0: A hardware service request for channel 10 is not present
            pub const HRS10_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 10 is present
            pub const HRS10_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 11
    pub mod HRS11 {
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

            /// 0b0: A hardware service request for channel 11 is not present
            pub const HRS11_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 11 is present
            pub const HRS11_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 12
    pub mod HRS12 {
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

            /// 0b0: A hardware service request for channel 12 is not present
            pub const HRS12_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 12 is present
            pub const HRS12_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 13
    pub mod HRS13 {
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

            /// 0b0: A hardware service request for channel 13 is not present
            pub const HRS13_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 13 is present
            pub const HRS13_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 14
    pub mod HRS14 {
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

            /// 0b0: A hardware service request for channel 14 is not present
            pub const HRS14_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 14 is present
            pub const HRS14_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 15
    pub mod HRS15 {
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

            /// 0b0: A hardware service request for channel 15 is not present
            pub const HRS15_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 15 is present
            pub const HRS15_1: u32 = 0b1;
        }
    }
}

/// Enable Asynchronous Request in Stop Register
pub mod EARS {

    /// Enable asynchronous DMA request in stop mode for channel 0.
    pub mod EDREQ_0 {
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

            /// 0b0: Disable asynchronous DMA request for channel 0.
            pub const EDREQ_0_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 0.
            pub const EDREQ_0_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 1.
    pub mod EDREQ_1 {
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

            /// 0b0: Disable asynchronous DMA request for channel 1
            pub const EDREQ_1_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 1.
            pub const EDREQ_1_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 2.
    pub mod EDREQ_2 {
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

            /// 0b0: Disable asynchronous DMA request for channel 2.
            pub const EDREQ_2_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 2.
            pub const EDREQ_2_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 3.
    pub mod EDREQ_3 {
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

            /// 0b0: Disable asynchronous DMA request for channel 3.
            pub const EDREQ_3_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 3.
            pub const EDREQ_3_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 4
    pub mod EDREQ_4 {
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

            /// 0b0: Disable asynchronous DMA request for channel 4.
            pub const EDREQ_4_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 4.
            pub const EDREQ_4_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 5
    pub mod EDREQ_5 {
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

            /// 0b0: Disable asynchronous DMA request for channel 5.
            pub const EDREQ_5_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 5.
            pub const EDREQ_5_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 6
    pub mod EDREQ_6 {
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

            /// 0b0: Disable asynchronous DMA request for channel 6.
            pub const EDREQ_6_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 6.
            pub const EDREQ_6_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 7
    pub mod EDREQ_7 {
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

            /// 0b0: Disable asynchronous DMA request for channel 7.
            pub const EDREQ_7_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 7.
            pub const EDREQ_7_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 8
    pub mod EDREQ_8 {
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

            /// 0b0: Disable asynchronous DMA request for channel 8.
            pub const EDREQ_8_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 8.
            pub const EDREQ_8_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 9
    pub mod EDREQ_9 {
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

            /// 0b0: Disable asynchronous DMA request for channel 9.
            pub const EDREQ_9_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 9.
            pub const EDREQ_9_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 10
    pub mod EDREQ_10 {
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

            /// 0b0: Disable asynchronous DMA request for channel 10.
            pub const EDREQ_10_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 10.
            pub const EDREQ_10_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 11
    pub mod EDREQ_11 {
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

            /// 0b0: Disable asynchronous DMA request for channel 11.
            pub const EDREQ_11_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 11.
            pub const EDREQ_11_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 12
    pub mod EDREQ_12 {
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

            /// 0b0: Disable asynchronous DMA request for channel 12.
            pub const EDREQ_12_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 12.
            pub const EDREQ_12_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 13
    pub mod EDREQ_13 {
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

            /// 0b0: Disable asynchronous DMA request for channel 13.
            pub const EDREQ_13_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 13.
            pub const EDREQ_13_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 14
    pub mod EDREQ_14 {
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

            /// 0b0: Disable asynchronous DMA request for channel 14.
            pub const EDREQ_14_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 14.
            pub const EDREQ_14_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 15
    pub mod EDREQ_15 {
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

            /// 0b0: Disable asynchronous DMA request for channel 15.
            pub const EDREQ_15_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 15.
            pub const EDREQ_15_1: u32 = 0b1;
        }
    }
}

/// Channel Priority Register
pub mod DCHPRI3 {

    /// Channel n Arbitration Priority
    pub mod CHPRI {
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

    /// Disable Preempt Ability. This field resets to 0.
    pub mod DPA {
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

            /// 0b0: Channel n can suspend a lower priority channel.
            pub const DPA_0: u32 = 0b0;

            /// 0b1: Channel n cannot suspend any channel, regardless of channel priority.
            pub const DPA_1: u32 = 0b1;
        }
    }

    /// Enable Channel Preemption. This field resets to 0.
    pub mod ECP {
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

            /// 0b0: Channel n cannot be suspended by a higher priority channel's service request.
            pub const ECP_0: u32 = 0b0;

            /// 0b1: Channel n can be temporarily suspended by the service request of a higher priority channel.
            pub const ECP_1: u32 = 0b1;
        }
    }
}

/// Channel Priority Register
pub mod DCHPRI2 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI1 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI0 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI7 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI6 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI5 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI4 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI11 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI10 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI9 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI8 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI15 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI14 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI13 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// Channel Priority Register
pub mod DCHPRI12 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
}

/// TCD Source Address
pub mod TCD_SADDR0 {

    /// Source Address
    pub mod SADDR {
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

/// TCD Signed Source Address Offset
pub mod TCD_SOFF0 {

    /// Source address signed offset
    pub mod SOFF {
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

/// TCD Transfer Attributes
pub mod TCD_ATTR0 {

    /// Destination data transfer size
    pub mod DSIZE {
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

    /// Destination Address Modulo
    pub mod DMOD {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Source data transfer size
    pub mod SSIZE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 8-bit
            pub const SSIZE_0: u32 = 0b000;

            /// 0b001: 16-bit
            pub const SSIZE_1: u32 = 0b001;

            /// 0b010: 32-bit
            pub const SSIZE_2: u32 = 0b010;

            /// 0b011: 64-bit
            pub const SSIZE_3: u32 = 0b011;

            /// 0b101: 32-byte burst (4 beats of 64 bits)
            pub const SSIZE_5: u32 = 0b101;
        }
    }

    /// Source Address Modulo
    pub mod SMOD {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Source address modulo feature is disabled
            pub const SMOD_0: u32 = 0b00000;

            /// 0b00001: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_1: u32 = 0b00001;

            /// 0b00010: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_2: u32 = 0b00010;

            /// 0b00011: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_3: u32 = 0b00011;

            /// 0b00100: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_4: u32 = 0b00100;

            /// 0b00101: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_5: u32 = 0b00101;

            /// 0b00110: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_6: u32 = 0b00110;

            /// 0b00111: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_7: u32 = 0b00111;

            /// 0b01000: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_8: u32 = 0b01000;

            /// 0b01001: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range.
            pub const SMOD_9: u32 = 0b01001;
        }
    }
}

/// TCD_NBYTES_ML and TCD_NBYTES_MLOFFYES0
/// TCD_NBYTES_ML: TCD_NBYTES_MLNO0 and TCD_NBYTES_MLOFFNO0
/// TCD_NBYTES_MLNO0: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO0: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES0: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_ML {

    /// Minor Byte Transfer Count
    pub mod NBYTES {
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

    /// Destination Minor Loop Offset enable
    pub mod DMLOE {
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

            /// 0b0: The minor loop offset is not applied to the DADDR
            pub const DMLOE_0: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the DADDR
            pub const DMLOE_1: u32 = 0b1;
        }
    }

    /// Source Minor Loop Offset Enable
    pub mod SMLOE {
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

            /// 0b0: The minor loop offset is not applied to the SADDR
            pub const SMLOE_0: u32 = 0b0;

            /// 0b1: The minor loop offset is applied to the SADDR
            pub const SMLOE_1: u32 = 0b1;
        }
    }

    /// If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes.
    pub mod MLOFF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (20 bits: 0xfffff << 10)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST0 {

    /// Last Source Address Adjustment
    pub mod SLAST {
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

/// TCD Destination Address
pub mod TCD_DADDR0 {

    /// Destination Address
    pub mod DADDR {
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

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF0 {

    /// Destination Address Signed Offset
    pub mod DOFF {
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

/// TCD_CITER_ELINKNO0 and TCD_CITER_ELINKYES0
/// TCD_CITER_ELINKNO0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINK {

    /// Current Major Iteration Count
    pub mod CITER {
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

    /// Enable channel-to-channel linking on minor-loop complete
    pub mod ELINK {
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

            /// 0b0: The channel-to-channel linking is disabled
            pub const ELINK_0: u32 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled
            pub const ELINK_1: u32 = 0b1;
        }
    }

    /// Minor Loop Link Channel Number
    pub mod LINKCH {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (4 bits: 0b1111 << 9)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA0 {

    /// DLASTSGA
    pub mod DLASTSGA {
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

/// TCD Control and Status
pub mod TCD_CSR0 {

    /// Channel Start
    pub mod START {
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

            /// 0b0: The channel is not explicitly started.
            pub const START_0: u32 = 0b0;

            /// 0b1: The channel is explicitly started via a software initiated service request.
            pub const START_1: u32 = 0b1;
        }
    }

    /// Enable an interrupt when major iteration count completes.
    pub mod INTMAJOR {
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

            /// 0b0: The end-of-major loop interrupt is disabled.
            pub const INTMAJOR_0: u32 = 0b0;

            /// 0b1: The end-of-major loop interrupt is enabled.
            pub const INTMAJOR_1: u32 = 0b1;
        }
    }

    /// Enable an interrupt when major counter is half complete.
    pub mod INTHALF {
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

            /// 0b0: The half-point interrupt is disabled.
            pub const INTHALF_0: u32 = 0b0;

            /// 0b1: The half-point interrupt is enabled.
            pub const INTHALF_1: u32 = 0b1;
        }
    }

    /// Disable Request
    pub mod DREQ {
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

            /// 0b0: The channel's ERQ bit is not affected.
            pub const DREQ_0: u32 = 0b0;

            /// 0b1: The channel's ERQ bit is cleared when the major loop is complete.
            pub const DREQ_1: u32 = 0b1;
        }
    }

    /// Enable Scatter/Gather Processing
    pub mod ESG {
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

            /// 0b0: The current channel's TCD is normal format.
            pub const ESG_0: u32 = 0b0;

            /// 0b1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution.
            pub const ESG_1: u32 = 0b1;
        }
    }

    /// Enable channel-to-channel linking on major loop complete
    pub mod MAJORELINK {
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

            /// 0b0: The channel-to-channel linking is disabled.
            pub const MAJORELINK_0: u32 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled.
            pub const MAJORELINK_1: u32 = 0b1;
        }
    }

    /// Channel Active
    pub mod ACTIVE {
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

    /// Channel Done
    pub mod DONE {
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

    /// Major Loop Link Channel Number
    pub mod MAJORLINKCH {
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

    /// Bandwidth Control
    pub mod BWC {
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

            /// 0b00: No eDMA engine stalls.
            pub const BWC_0: u32 = 0b00;

            /// 0b10: eDMA engine stalls for 4 cycles after each R/W.
            pub const BWC_2: u32 = 0b10;

            /// 0b11: eDMA engine stalls for 8 cycles after each R/W.
            pub const BWC_3: u32 = 0b11;
        }
    }
}

/// TCD_BITER_ELINKNO0 and TCD_BITER_ELINKYES0
/// TCD_BITER_ELINKNO0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINK {

    /// Starting Major Iteration Count
    pub mod BITER {
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

    /// Enables channel-to-channel linking on minor loop complete
    pub mod ELINK {
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

            /// 0b0: The channel-to-channel linking is disabled
            pub const ELINK_0: u32 = 0b0;

            /// 0b1: The channel-to-channel linking is enabled
            pub const ELINK_1: u32 = 0b1;
        }
    }

    /// Link Channel Number
    pub mod LINKCH {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (4 bits: 0b1111 << 9)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TCD Source Address
pub mod TCD_SADDR1 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF1 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR1 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO1 and TCD_NBYTES_MLOFFYES1
/// TCD_NBYTES_MLNO1: TCD_NBYTES_MLNO1 and TCD_NBYTES_MLOFFNO1
/// TCD_NBYTES_MLNO1: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO1: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES1: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO1 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST1 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR1 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF1 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO1 and TCD_CITER_ELINKYES1
/// TCD_CITER_ELINKNO1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO1 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA1 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR1 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO1 and TCD_BITER_ELINKYES1
/// TCD_BITER_ELINKNO1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO1 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR2 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF2 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR2 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO2 and TCD_NBYTES_MLOFFYES2
/// TCD_NBYTES_MLNO2: TCD_NBYTES_MLNO2 and TCD_NBYTES_MLOFFNO2
/// TCD_NBYTES_MLNO2: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO2: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES2: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO2 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST2 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR2 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF2 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO2 and TCD_CITER_ELINKYES2
/// TCD_CITER_ELINKNO2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO2 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA2 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR2 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO2 and TCD_BITER_ELINKYES2
/// TCD_BITER_ELINKNO2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO2 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR3 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF3 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR3 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO3 and TCD_NBYTES_MLOFFYES3
/// TCD_NBYTES_MLNO3: TCD_NBYTES_MLNO3 and TCD_NBYTES_MLOFFNO3
/// TCD_NBYTES_MLNO3: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO3: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES3: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO3 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST3 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR3 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF3 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO3 and TCD_CITER_ELINKYES3
/// TCD_CITER_ELINKNO3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO3 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA3 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR3 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO3 and TCD_BITER_ELINKYES3
/// TCD_BITER_ELINKNO3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO3 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR4 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF4 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR4 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO4 and TCD_NBYTES_MLOFFYES4
/// TCD_NBYTES_MLNO4: TCD_NBYTES_MLNO4 and TCD_NBYTES_MLOFFNO4
/// TCD_NBYTES_MLNO4: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO4: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES4: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO4 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST4 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR4 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF4 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO4 and TCD_CITER_ELINKYES4
/// TCD_CITER_ELINKNO4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO4 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA4 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR4 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO4 and TCD_BITER_ELINKYES4
/// TCD_BITER_ELINKNO4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO4 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR5 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF5 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR5 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO5 and TCD_NBYTES_MLOFFYES5
/// TCD_NBYTES_MLNO5: TCD_NBYTES_MLNO5 and TCD_NBYTES_MLOFFNO5
/// TCD_NBYTES_MLNO5: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO5: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES5: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO5 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST5 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR5 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF5 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO5 and TCD_CITER_ELINKYES5
/// TCD_CITER_ELINKNO5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO5 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA5 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR5 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO5 and TCD_BITER_ELINKYES5
/// TCD_BITER_ELINKNO5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO5 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR6 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF6 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR6 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO6 and TCD_NBYTES_MLOFFYES6
/// TCD_NBYTES_MLNO6: TCD_NBYTES_MLNO6 and TCD_NBYTES_MLOFFNO6
/// TCD_NBYTES_MLNO6: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO6: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES6: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO6 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST6 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR6 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF6 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO6 and TCD_CITER_ELINKYES6
/// TCD_CITER_ELINKNO6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO6 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA6 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR6 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO6 and TCD_BITER_ELINKYES6
/// TCD_BITER_ELINKNO6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO6 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR7 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF7 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR7 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO7 and TCD_NBYTES_MLOFFYES7
/// TCD_NBYTES_MLNO7: TCD_NBYTES_MLNO7 and TCD_NBYTES_MLOFFNO7
/// TCD_NBYTES_MLNO7: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO7: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES7: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO7 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST7 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR7 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF7 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO7 and TCD_CITER_ELINKYES7
/// TCD_CITER_ELINKNO7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO7 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA7 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR7 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO7 and TCD_BITER_ELINKYES7
/// TCD_BITER_ELINKNO7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO7 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR8 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF8 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR8 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO8 and TCD_NBYTES_MLOFFYES8
/// TCD_NBYTES_MLNO8: TCD_NBYTES_MLNO8 and TCD_NBYTES_MLOFFNO8
/// TCD_NBYTES_MLNO8: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO8: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES8: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO8 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST8 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR8 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF8 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO8 and TCD_CITER_ELINKYES8
/// TCD_CITER_ELINKNO8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO8 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA8 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR8 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO8 and TCD_BITER_ELINKYES8
/// TCD_BITER_ELINKNO8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO8 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR9 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF9 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR9 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO9 and TCD_NBYTES_MLOFFYES9
/// TCD_NBYTES_MLNO9: TCD_NBYTES_MLNO9 and TCD_NBYTES_MLOFFNO9
/// TCD_NBYTES_MLNO9: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO9: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES9: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO9 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST9 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR9 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF9 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO9 and TCD_CITER_ELINKYES9
/// TCD_CITER_ELINKNO9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO9 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA9 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR9 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO9 and TCD_BITER_ELINKYES9
/// TCD_BITER_ELINKNO9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO9 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR10 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF10 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR10 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO10 and TCD_NBYTES_MLOFFYES10
/// TCD_NBYTES_MLNO10: TCD_NBYTES_MLNO10 and TCD_NBYTES_MLOFFNO10
/// TCD_NBYTES_MLNO10: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO10: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES10: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO10 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST10 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR10 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF10 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO10 and TCD_CITER_ELINKYES10
/// TCD_CITER_ELINKNO10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO10 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA10 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR10 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO10 and TCD_BITER_ELINKYES10
/// TCD_BITER_ELINKNO10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO10 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR11 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF11 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR11 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO11 and TCD_NBYTES_MLOFFYES11
/// TCD_NBYTES_MLNO11: TCD_NBYTES_MLNO11 and TCD_NBYTES_MLOFFNO11
/// TCD_NBYTES_MLNO11: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO11: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES11: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO11 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST11 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR11 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF11 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO11 and TCD_CITER_ELINKYES11
/// TCD_CITER_ELINKNO11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO11 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA11 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR11 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO11 and TCD_BITER_ELINKYES11
/// TCD_BITER_ELINKNO11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO11 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR12 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF12 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR12 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO12 and TCD_NBYTES_MLOFFYES12
/// TCD_NBYTES_MLNO12: TCD_NBYTES_MLNO12 and TCD_NBYTES_MLOFFNO12
/// TCD_NBYTES_MLNO12: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO12: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES12: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO12 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST12 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR12 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF12 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO12 and TCD_CITER_ELINKYES12
/// TCD_CITER_ELINKNO12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO12 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA12 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR12 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO12 and TCD_BITER_ELINKYES12
/// TCD_BITER_ELINKNO12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO12 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR13 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF13 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR13 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO13 and TCD_NBYTES_MLOFFYES13
/// TCD_NBYTES_MLNO13: TCD_NBYTES_MLNO13 and TCD_NBYTES_MLOFFNO13
/// TCD_NBYTES_MLNO13: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO13: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES13: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO13 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST13 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR13 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF13 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO13 and TCD_CITER_ELINKYES13
/// TCD_CITER_ELINKNO13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO13 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA13 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR13 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO13 and TCD_BITER_ELINKYES13
/// TCD_BITER_ELINKNO13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO13 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR14 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF14 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR14 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO14 and TCD_NBYTES_MLOFFYES14
/// TCD_NBYTES_MLNO14: TCD_NBYTES_MLNO14 and TCD_NBYTES_MLOFFNO14
/// TCD_NBYTES_MLNO14: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO14: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES14: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO14 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST14 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR14 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF14 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO14 and TCD_CITER_ELINKYES14
/// TCD_CITER_ELINKNO14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO14 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA14 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR14 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO14 and TCD_BITER_ELINKYES14
/// TCD_BITER_ELINKNO14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO14 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR15 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF15 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR15 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO15 and TCD_NBYTES_MLOFFYES15
/// TCD_NBYTES_MLNO15: TCD_NBYTES_MLNO15 and TCD_NBYTES_MLOFFNO15
/// TCD_NBYTES_MLNO15: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO15: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES15: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO15 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST15 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR15 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF15 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO15 and TCD_CITER_ELINKYES15
/// TCD_CITER_ELINKNO15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO15 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA15 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR15 {
    pub use super::TCD_CSR0::ACTIVE;
    pub use super::TCD_CSR0::BWC;
    pub use super::TCD_CSR0::DONE;
    pub use super::TCD_CSR0::DREQ;
    pub use super::TCD_CSR0::ESG;
    pub use super::TCD_CSR0::INTHALF;
    pub use super::TCD_CSR0::INTMAJOR;
    pub use super::TCD_CSR0::MAJORELINK;
    pub use super::TCD_CSR0::MAJORLINKCH;
    pub use super::TCD_CSR0::START;
}

/// TCD_BITER_ELINKNO15 and TCD_BITER_ELINKYES15
/// TCD_BITER_ELINKNO15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO15 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}
pub struct RegisterBlock {
    /// Control Register
    pub CR: RWRegister<u32>,

    /// Error Status Register
    pub ES: RORegister<u32>,

    _reserved1: [u32; 1],

    /// Enable Request Register
    pub ERQ: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// Enable Error Interrupt Register
    pub EEI: RWRegister<u32>,

    /// Clear Enable Error Interrupt Register
    pub CEEI: RWRegister<u8>,

    /// Set Enable Error Interrupt Register
    pub SEEI: RWRegister<u8>,

    /// Clear Enable Request Register
    pub CERQ: RWRegister<u8>,

    /// Set Enable Request Register
    pub SERQ: RWRegister<u8>,

    /// Clear DONE Status Bit Register
    pub CDNE: RWRegister<u8>,

    /// Set START Bit Register
    pub SSRT: RWRegister<u8>,

    /// Clear Error Register
    pub CERR: RWRegister<u8>,

    /// Clear Interrupt Request Register
    pub CINT: RWRegister<u8>,

    _reserved3: [u32; 1],

    /// Interrupt Request Register
    pub INT: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Error Register
    pub ERR: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Hardware Request Status Register
    pub HRS: RORegister<u32>,

    _reserved6: [u32; 3],

    /// Enable Asynchronous Request in Stop Register
    pub EARS: RWRegister<u32>,

    _reserved7: [u32; 46],

    /// Channel Priority Register
    pub DCHPRI3: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI2: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI1: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI0: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI7: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI6: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI5: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI4: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI11: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI10: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI9: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI8: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI15: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI14: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI13: RWRegister<u8>,

    /// Channel Priority Register
    pub DCHPRI12: RWRegister<u8>,

    _reserved8: [u32; 956],

    /// TCD Source Address
    pub TCD_SADDR0: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF0: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR0: RWRegister<u16>,

    /// TCD_NBYTES_ML and TCD_NBYTES_MLOFFYES0
    /// TCD_NBYTES_ML: TCD_NBYTES_MLNO0 and TCD_NBYTES_MLOFFNO0
    /// TCD_NBYTES_MLNO0: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO0: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES0: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST0: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR0: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF0: RWRegister<u16>,

    /// TCD_CITER_ELINKNO0 and TCD_CITER_ELINKYES0
    /// TCD_CITER_ELINKNO0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES0: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA0: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR0: RWRegister<u16>,

    /// TCD_BITER_ELINKNO0 and TCD_BITER_ELINKYES0
    /// TCD_BITER_ELINKNO0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES0: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR1: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF1: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR1: RWRegister<u16>,

    /// TCD_NBYTES_MLNO1 and TCD_NBYTES_MLOFFYES1
    /// TCD_NBYTES_MLNO1: TCD_NBYTES_MLNO1 and TCD_NBYTES_MLOFFNO1
    /// TCD_NBYTES_MLNO1: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO1: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES1: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO1: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST1: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR1: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF1: RWRegister<u16>,

    /// TCD_CITER_ELINKNO1 and TCD_CITER_ELINKYES1
    /// TCD_CITER_ELINKNO1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES1: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO1: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA1: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR1: RWRegister<u16>,

    /// TCD_BITER_ELINKNO1 and TCD_BITER_ELINKYES1
    /// TCD_BITER_ELINKNO1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES1: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO1: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR2: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF2: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR2: RWRegister<u16>,

    /// TCD_NBYTES_MLNO2 and TCD_NBYTES_MLOFFYES2
    /// TCD_NBYTES_MLNO2: TCD_NBYTES_MLNO2 and TCD_NBYTES_MLOFFNO2
    /// TCD_NBYTES_MLNO2: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO2: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES2: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO2: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST2: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR2: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF2: RWRegister<u16>,

    /// TCD_CITER_ELINKNO2 and TCD_CITER_ELINKYES2
    /// TCD_CITER_ELINKNO2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES2: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO2: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA2: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR2: RWRegister<u16>,

    /// TCD_BITER_ELINKNO2 and TCD_BITER_ELINKYES2
    /// TCD_BITER_ELINKNO2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES2: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO2: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR3: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF3: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR3: RWRegister<u16>,

    /// TCD_NBYTES_MLNO3 and TCD_NBYTES_MLOFFYES3
    /// TCD_NBYTES_MLNO3: TCD_NBYTES_MLNO3 and TCD_NBYTES_MLOFFNO3
    /// TCD_NBYTES_MLNO3: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO3: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES3: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO3: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST3: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR3: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF3: RWRegister<u16>,

    /// TCD_CITER_ELINKNO3 and TCD_CITER_ELINKYES3
    /// TCD_CITER_ELINKNO3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES3: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO3: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA3: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR3: RWRegister<u16>,

    /// TCD_BITER_ELINKNO3 and TCD_BITER_ELINKYES3
    /// TCD_BITER_ELINKNO3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES3: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO3: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR4: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF4: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR4: RWRegister<u16>,

    /// TCD_NBYTES_MLNO4 and TCD_NBYTES_MLOFFYES4
    /// TCD_NBYTES_MLNO4: TCD_NBYTES_MLNO4 and TCD_NBYTES_MLOFFNO4
    /// TCD_NBYTES_MLNO4: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO4: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES4: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO4: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST4: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR4: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF4: RWRegister<u16>,

    /// TCD_CITER_ELINKNO4 and TCD_CITER_ELINKYES4
    /// TCD_CITER_ELINKNO4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES4: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO4: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA4: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR4: RWRegister<u16>,

    /// TCD_BITER_ELINKNO4 and TCD_BITER_ELINKYES4
    /// TCD_BITER_ELINKNO4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES4: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO4: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR5: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF5: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR5: RWRegister<u16>,

    /// TCD_NBYTES_MLNO5 and TCD_NBYTES_MLOFFYES5
    /// TCD_NBYTES_MLNO5: TCD_NBYTES_MLNO5 and TCD_NBYTES_MLOFFNO5
    /// TCD_NBYTES_MLNO5: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO5: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES5: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO5: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST5: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR5: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF5: RWRegister<u16>,

    /// TCD_CITER_ELINKNO5 and TCD_CITER_ELINKYES5
    /// TCD_CITER_ELINKNO5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES5: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO5: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA5: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR5: RWRegister<u16>,

    /// TCD_BITER_ELINKNO5 and TCD_BITER_ELINKYES5
    /// TCD_BITER_ELINKNO5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES5: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO5: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR6: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF6: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR6: RWRegister<u16>,

    /// TCD_NBYTES_MLNO6 and TCD_NBYTES_MLOFFYES6
    /// TCD_NBYTES_MLNO6: TCD_NBYTES_MLNO6 and TCD_NBYTES_MLOFFNO6
    /// TCD_NBYTES_MLNO6: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO6: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES6: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO6: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST6: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR6: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF6: RWRegister<u16>,

    /// TCD_CITER_ELINKNO6 and TCD_CITER_ELINKYES6
    /// TCD_CITER_ELINKNO6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES6: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO6: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA6: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR6: RWRegister<u16>,

    /// TCD_BITER_ELINKNO6 and TCD_BITER_ELINKYES6
    /// TCD_BITER_ELINKNO6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES6: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO6: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR7: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF7: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR7: RWRegister<u16>,

    /// TCD_NBYTES_MLNO7 and TCD_NBYTES_MLOFFYES7
    /// TCD_NBYTES_MLNO7: TCD_NBYTES_MLNO7 and TCD_NBYTES_MLOFFNO7
    /// TCD_NBYTES_MLNO7: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO7: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES7: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO7: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST7: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR7: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF7: RWRegister<u16>,

    /// TCD_CITER_ELINKNO7 and TCD_CITER_ELINKYES7
    /// TCD_CITER_ELINKNO7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES7: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO7: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA7: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR7: RWRegister<u16>,

    /// TCD_BITER_ELINKNO7 and TCD_BITER_ELINKYES7
    /// TCD_BITER_ELINKNO7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES7: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO7: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR8: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF8: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR8: RWRegister<u16>,

    /// TCD_NBYTES_MLNO8 and TCD_NBYTES_MLOFFYES8
    /// TCD_NBYTES_MLNO8: TCD_NBYTES_MLNO8 and TCD_NBYTES_MLOFFNO8
    /// TCD_NBYTES_MLNO8: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO8: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES8: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO8: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST8: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR8: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF8: RWRegister<u16>,

    /// TCD_CITER_ELINKNO8 and TCD_CITER_ELINKYES8
    /// TCD_CITER_ELINKNO8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES8: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO8: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA8: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR8: RWRegister<u16>,

    /// TCD_BITER_ELINKNO8 and TCD_BITER_ELINKYES8
    /// TCD_BITER_ELINKNO8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES8: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO8: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR9: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF9: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR9: RWRegister<u16>,

    /// TCD_NBYTES_MLNO9 and TCD_NBYTES_MLOFFYES9
    /// TCD_NBYTES_MLNO9: TCD_NBYTES_MLNO9 and TCD_NBYTES_MLOFFNO9
    /// TCD_NBYTES_MLNO9: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO9: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES9: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO9: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST9: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR9: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF9: RWRegister<u16>,

    /// TCD_CITER_ELINKNO9 and TCD_CITER_ELINKYES9
    /// TCD_CITER_ELINKNO9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES9: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO9: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA9: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR9: RWRegister<u16>,

    /// TCD_BITER_ELINKNO9 and TCD_BITER_ELINKYES9
    /// TCD_BITER_ELINKNO9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES9: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO9: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR10: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF10: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR10: RWRegister<u16>,

    /// TCD_NBYTES_MLNO10 and TCD_NBYTES_MLOFFYES10
    /// TCD_NBYTES_MLNO10: TCD_NBYTES_MLNO10 and TCD_NBYTES_MLOFFNO10
    /// TCD_NBYTES_MLNO10: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO10: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES10: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO10: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST10: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR10: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF10: RWRegister<u16>,

    /// TCD_CITER_ELINKNO10 and TCD_CITER_ELINKYES10
    /// TCD_CITER_ELINKNO10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES10: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO10: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA10: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR10: RWRegister<u16>,

    /// TCD_BITER_ELINKNO10 and TCD_BITER_ELINKYES10
    /// TCD_BITER_ELINKNO10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES10: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO10: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR11: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF11: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR11: RWRegister<u16>,

    /// TCD_NBYTES_MLNO11 and TCD_NBYTES_MLOFFYES11
    /// TCD_NBYTES_MLNO11: TCD_NBYTES_MLNO11 and TCD_NBYTES_MLOFFNO11
    /// TCD_NBYTES_MLNO11: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO11: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES11: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO11: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST11: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR11: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF11: RWRegister<u16>,

    /// TCD_CITER_ELINKNO11 and TCD_CITER_ELINKYES11
    /// TCD_CITER_ELINKNO11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES11: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO11: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA11: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR11: RWRegister<u16>,

    /// TCD_BITER_ELINKNO11 and TCD_BITER_ELINKYES11
    /// TCD_BITER_ELINKNO11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES11: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO11: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR12: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF12: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR12: RWRegister<u16>,

    /// TCD_NBYTES_MLNO12 and TCD_NBYTES_MLOFFYES12
    /// TCD_NBYTES_MLNO12: TCD_NBYTES_MLNO12 and TCD_NBYTES_MLOFFNO12
    /// TCD_NBYTES_MLNO12: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO12: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES12: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO12: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST12: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR12: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF12: RWRegister<u16>,

    /// TCD_CITER_ELINKNO12 and TCD_CITER_ELINKYES12
    /// TCD_CITER_ELINKNO12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES12: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO12: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA12: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR12: RWRegister<u16>,

    /// TCD_BITER_ELINKNO12 and TCD_BITER_ELINKYES12
    /// TCD_BITER_ELINKNO12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES12: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO12: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR13: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF13: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR13: RWRegister<u16>,

    /// TCD_NBYTES_MLNO13 and TCD_NBYTES_MLOFFYES13
    /// TCD_NBYTES_MLNO13: TCD_NBYTES_MLNO13 and TCD_NBYTES_MLOFFNO13
    /// TCD_NBYTES_MLNO13: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO13: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES13: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO13: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST13: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR13: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF13: RWRegister<u16>,

    /// TCD_CITER_ELINKNO13 and TCD_CITER_ELINKYES13
    /// TCD_CITER_ELINKNO13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES13: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO13: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA13: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR13: RWRegister<u16>,

    /// TCD_BITER_ELINKNO13 and TCD_BITER_ELINKYES13
    /// TCD_BITER_ELINKNO13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES13: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO13: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR14: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF14: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR14: RWRegister<u16>,

    /// TCD_NBYTES_MLNO14 and TCD_NBYTES_MLOFFYES14
    /// TCD_NBYTES_MLNO14: TCD_NBYTES_MLNO14 and TCD_NBYTES_MLOFFNO14
    /// TCD_NBYTES_MLNO14: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO14: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES14: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO14: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST14: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR14: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF14: RWRegister<u16>,

    /// TCD_CITER_ELINKNO14 and TCD_CITER_ELINKYES14
    /// TCD_CITER_ELINKNO14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES14: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO14: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA14: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR14: RWRegister<u16>,

    /// TCD_BITER_ELINKNO14 and TCD_BITER_ELINKYES14
    /// TCD_BITER_ELINKNO14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES14: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO14: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR15: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF15: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR15: RWRegister<u16>,

    /// TCD_NBYTES_MLNO15 and TCD_NBYTES_MLOFFYES15
    /// TCD_NBYTES_MLNO15: TCD_NBYTES_MLNO15 and TCD_NBYTES_MLOFFNO15
    /// TCD_NBYTES_MLNO15: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO15: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES15: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO15: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST15: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR15: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF15: RWRegister<u16>,

    /// TCD_CITER_ELINKNO15 and TCD_CITER_ELINKYES15
    /// TCD_CITER_ELINKNO15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES15: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO15: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA15: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR15: RWRegister<u16>,

    /// TCD_BITER_ELINKNO15 and TCD_BITER_ELINKYES15
    /// TCD_BITER_ELINKNO15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES15: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO15: RWRegister<u16>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ES: u32,
    pub ERQ: u32,
    pub EEI: u32,
    pub CEEI: u8,
    pub SEEI: u8,
    pub CERQ: u8,
    pub SERQ: u8,
    pub CDNE: u8,
    pub SSRT: u8,
    pub CERR: u8,
    pub CINT: u8,
    pub INT: u32,
    pub ERR: u32,
    pub HRS: u32,
    pub EARS: u32,
    pub DCHPRI3: u8,
    pub DCHPRI2: u8,
    pub DCHPRI1: u8,
    pub DCHPRI0: u8,
    pub DCHPRI7: u8,
    pub DCHPRI6: u8,
    pub DCHPRI5: u8,
    pub DCHPRI4: u8,
    pub DCHPRI11: u8,
    pub DCHPRI10: u8,
    pub DCHPRI9: u8,
    pub DCHPRI8: u8,
    pub DCHPRI15: u8,
    pub DCHPRI14: u8,
    pub DCHPRI13: u8,
    pub DCHPRI12: u8,
    pub TCD_SADDR0: u32,
    pub TCD_SOFF0: u16,
    pub TCD_ATTR0: u16,
    pub TCD_NBYTES_ML: u32,
    pub TCD_SLAST0: u32,
    pub TCD_DADDR0: u32,
    pub TCD_DOFF0: u16,
    pub TCD_CITER_ELINK: u16,
    pub TCD_DLASTSGA0: u32,
    pub TCD_CSR0: u16,
    pub TCD_BITER_ELINK: u16,
    pub TCD_SADDR1: u32,
    pub TCD_SOFF1: u16,
    pub TCD_ATTR1: u16,
    pub TCD_NBYTES_MLNO1: u32,
    pub TCD_SLAST1: u32,
    pub TCD_DADDR1: u32,
    pub TCD_DOFF1: u16,
    pub TCD_CITER_ELINKNO1: u16,
    pub TCD_DLASTSGA1: u32,
    pub TCD_CSR1: u16,
    pub TCD_BITER_ELINKNO1: u16,
    pub TCD_SADDR2: u32,
    pub TCD_SOFF2: u16,
    pub TCD_ATTR2: u16,
    pub TCD_NBYTES_MLNO2: u32,
    pub TCD_SLAST2: u32,
    pub TCD_DADDR2: u32,
    pub TCD_DOFF2: u16,
    pub TCD_CITER_ELINKNO2: u16,
    pub TCD_DLASTSGA2: u32,
    pub TCD_CSR2: u16,
    pub TCD_BITER_ELINKNO2: u16,
    pub TCD_SADDR3: u32,
    pub TCD_SOFF3: u16,
    pub TCD_ATTR3: u16,
    pub TCD_NBYTES_MLNO3: u32,
    pub TCD_SLAST3: u32,
    pub TCD_DADDR3: u32,
    pub TCD_DOFF3: u16,
    pub TCD_CITER_ELINKNO3: u16,
    pub TCD_DLASTSGA3: u32,
    pub TCD_CSR3: u16,
    pub TCD_BITER_ELINKNO3: u16,
    pub TCD_SADDR4: u32,
    pub TCD_SOFF4: u16,
    pub TCD_ATTR4: u16,
    pub TCD_NBYTES_MLNO4: u32,
    pub TCD_SLAST4: u32,
    pub TCD_DADDR4: u32,
    pub TCD_DOFF4: u16,
    pub TCD_CITER_ELINKNO4: u16,
    pub TCD_DLASTSGA4: u32,
    pub TCD_CSR4: u16,
    pub TCD_BITER_ELINKNO4: u16,
    pub TCD_SADDR5: u32,
    pub TCD_SOFF5: u16,
    pub TCD_ATTR5: u16,
    pub TCD_NBYTES_MLNO5: u32,
    pub TCD_SLAST5: u32,
    pub TCD_DADDR5: u32,
    pub TCD_DOFF5: u16,
    pub TCD_CITER_ELINKNO5: u16,
    pub TCD_DLASTSGA5: u32,
    pub TCD_CSR5: u16,
    pub TCD_BITER_ELINKNO5: u16,
    pub TCD_SADDR6: u32,
    pub TCD_SOFF6: u16,
    pub TCD_ATTR6: u16,
    pub TCD_NBYTES_MLNO6: u32,
    pub TCD_SLAST6: u32,
    pub TCD_DADDR6: u32,
    pub TCD_DOFF6: u16,
    pub TCD_CITER_ELINKNO6: u16,
    pub TCD_DLASTSGA6: u32,
    pub TCD_CSR6: u16,
    pub TCD_BITER_ELINKNO6: u16,
    pub TCD_SADDR7: u32,
    pub TCD_SOFF7: u16,
    pub TCD_ATTR7: u16,
    pub TCD_NBYTES_MLNO7: u32,
    pub TCD_SLAST7: u32,
    pub TCD_DADDR7: u32,
    pub TCD_DOFF7: u16,
    pub TCD_CITER_ELINKNO7: u16,
    pub TCD_DLASTSGA7: u32,
    pub TCD_CSR7: u16,
    pub TCD_BITER_ELINKNO7: u16,
    pub TCD_SADDR8: u32,
    pub TCD_SOFF8: u16,
    pub TCD_ATTR8: u16,
    pub TCD_NBYTES_MLNO8: u32,
    pub TCD_SLAST8: u32,
    pub TCD_DADDR8: u32,
    pub TCD_DOFF8: u16,
    pub TCD_CITER_ELINKNO8: u16,
    pub TCD_DLASTSGA8: u32,
    pub TCD_CSR8: u16,
    pub TCD_BITER_ELINKNO8: u16,
    pub TCD_SADDR9: u32,
    pub TCD_SOFF9: u16,
    pub TCD_ATTR9: u16,
    pub TCD_NBYTES_MLNO9: u32,
    pub TCD_SLAST9: u32,
    pub TCD_DADDR9: u32,
    pub TCD_DOFF9: u16,
    pub TCD_CITER_ELINKNO9: u16,
    pub TCD_DLASTSGA9: u32,
    pub TCD_CSR9: u16,
    pub TCD_BITER_ELINKNO9: u16,
    pub TCD_SADDR10: u32,
    pub TCD_SOFF10: u16,
    pub TCD_ATTR10: u16,
    pub TCD_NBYTES_MLNO10: u32,
    pub TCD_SLAST10: u32,
    pub TCD_DADDR10: u32,
    pub TCD_DOFF10: u16,
    pub TCD_CITER_ELINKNO10: u16,
    pub TCD_DLASTSGA10: u32,
    pub TCD_CSR10: u16,
    pub TCD_BITER_ELINKNO10: u16,
    pub TCD_SADDR11: u32,
    pub TCD_SOFF11: u16,
    pub TCD_ATTR11: u16,
    pub TCD_NBYTES_MLNO11: u32,
    pub TCD_SLAST11: u32,
    pub TCD_DADDR11: u32,
    pub TCD_DOFF11: u16,
    pub TCD_CITER_ELINKNO11: u16,
    pub TCD_DLASTSGA11: u32,
    pub TCD_CSR11: u16,
    pub TCD_BITER_ELINKNO11: u16,
    pub TCD_SADDR12: u32,
    pub TCD_SOFF12: u16,
    pub TCD_ATTR12: u16,
    pub TCD_NBYTES_MLNO12: u32,
    pub TCD_SLAST12: u32,
    pub TCD_DADDR12: u32,
    pub TCD_DOFF12: u16,
    pub TCD_CITER_ELINKNO12: u16,
    pub TCD_DLASTSGA12: u32,
    pub TCD_CSR12: u16,
    pub TCD_BITER_ELINKNO12: u16,
    pub TCD_SADDR13: u32,
    pub TCD_SOFF13: u16,
    pub TCD_ATTR13: u16,
    pub TCD_NBYTES_MLNO13: u32,
    pub TCD_SLAST13: u32,
    pub TCD_DADDR13: u32,
    pub TCD_DOFF13: u16,
    pub TCD_CITER_ELINKNO13: u16,
    pub TCD_DLASTSGA13: u32,
    pub TCD_CSR13: u16,
    pub TCD_BITER_ELINKNO13: u16,
    pub TCD_SADDR14: u32,
    pub TCD_SOFF14: u16,
    pub TCD_ATTR14: u16,
    pub TCD_NBYTES_MLNO14: u32,
    pub TCD_SLAST14: u32,
    pub TCD_DADDR14: u32,
    pub TCD_DOFF14: u16,
    pub TCD_CITER_ELINKNO14: u16,
    pub TCD_DLASTSGA14: u32,
    pub TCD_CSR14: u16,
    pub TCD_BITER_ELINKNO14: u16,
    pub TCD_SADDR15: u32,
    pub TCD_SOFF15: u16,
    pub TCD_ATTR15: u16,
    pub TCD_NBYTES_MLNO15: u32,
    pub TCD_SLAST15: u32,
    pub TCD_DADDR15: u32,
    pub TCD_DOFF15: u16,
    pub TCD_CITER_ELINKNO15: u16,
    pub TCD_DLASTSGA15: u32,
    pub TCD_CSR15: u16,
    pub TCD_BITER_ELINKNO15: u16,
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

/// Access functions for the DMA0 peripheral instance
pub mod DMA0 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400e8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA0
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        ES: 0x00000000,
        ERQ: 0x00000000,
        EEI: 0x00000000,
        CEEI: 0x00000000,
        SEEI: 0x00000000,
        CERQ: 0x00000000,
        SERQ: 0x00000000,
        CDNE: 0x00000000,
        SSRT: 0x00000000,
        CERR: 0x00000000,
        CINT: 0x00000000,
        INT: 0x00000000,
        ERR: 0x00000000,
        HRS: 0x00000000,
        EARS: 0x00000000,
        DCHPRI3: 0x00000003,
        DCHPRI2: 0x00000002,
        DCHPRI1: 0x00000001,
        DCHPRI0: 0x00000000,
        DCHPRI7: 0x00000007,
        DCHPRI6: 0x00000006,
        DCHPRI5: 0x00000005,
        DCHPRI4: 0x00000004,
        DCHPRI11: 0x0000000B,
        DCHPRI10: 0x0000000A,
        DCHPRI9: 0x00000009,
        DCHPRI8: 0x00000008,
        DCHPRI15: 0x0000000F,
        DCHPRI14: 0x0000000E,
        DCHPRI13: 0x0000000D,
        DCHPRI12: 0x0000000C,
        TCD_SADDR0: 0x00000000,
        TCD_SOFF0: 0x00000000,
        TCD_ATTR0: 0x00000000,
        TCD_NBYTES_ML: 0x00000000,
        TCD_SLAST0: 0x00000000,
        TCD_DADDR0: 0x00000000,
        TCD_DOFF0: 0x00000000,
        TCD_CITER_ELINK: 0x00000000,
        TCD_DLASTSGA0: 0x00000000,
        TCD_CSR0: 0x00000000,
        TCD_BITER_ELINK: 0x00000000,
        TCD_SADDR1: 0x00000000,
        TCD_SOFF1: 0x00000000,
        TCD_ATTR1: 0x00000000,
        TCD_NBYTES_MLNO1: 0x00000000,
        TCD_SLAST1: 0x00000000,
        TCD_DADDR1: 0x00000000,
        TCD_DOFF1: 0x00000000,
        TCD_CITER_ELINKNO1: 0x00000000,
        TCD_DLASTSGA1: 0x00000000,
        TCD_CSR1: 0x00000000,
        TCD_BITER_ELINKNO1: 0x00000000,
        TCD_SADDR2: 0x00000000,
        TCD_SOFF2: 0x00000000,
        TCD_ATTR2: 0x00000000,
        TCD_NBYTES_MLNO2: 0x00000000,
        TCD_SLAST2: 0x00000000,
        TCD_DADDR2: 0x00000000,
        TCD_DOFF2: 0x00000000,
        TCD_CITER_ELINKNO2: 0x00000000,
        TCD_DLASTSGA2: 0x00000000,
        TCD_CSR2: 0x00000000,
        TCD_BITER_ELINKNO2: 0x00000000,
        TCD_SADDR3: 0x00000000,
        TCD_SOFF3: 0x00000000,
        TCD_ATTR3: 0x00000000,
        TCD_NBYTES_MLNO3: 0x00000000,
        TCD_SLAST3: 0x00000000,
        TCD_DADDR3: 0x00000000,
        TCD_DOFF3: 0x00000000,
        TCD_CITER_ELINKNO3: 0x00000000,
        TCD_DLASTSGA3: 0x00000000,
        TCD_CSR3: 0x00000000,
        TCD_BITER_ELINKNO3: 0x00000000,
        TCD_SADDR4: 0x00000000,
        TCD_SOFF4: 0x00000000,
        TCD_ATTR4: 0x00000000,
        TCD_NBYTES_MLNO4: 0x00000000,
        TCD_SLAST4: 0x00000000,
        TCD_DADDR4: 0x00000000,
        TCD_DOFF4: 0x00000000,
        TCD_CITER_ELINKNO4: 0x00000000,
        TCD_DLASTSGA4: 0x00000000,
        TCD_CSR4: 0x00000000,
        TCD_BITER_ELINKNO4: 0x00000000,
        TCD_SADDR5: 0x00000000,
        TCD_SOFF5: 0x00000000,
        TCD_ATTR5: 0x00000000,
        TCD_NBYTES_MLNO5: 0x00000000,
        TCD_SLAST5: 0x00000000,
        TCD_DADDR5: 0x00000000,
        TCD_DOFF5: 0x00000000,
        TCD_CITER_ELINKNO5: 0x00000000,
        TCD_DLASTSGA5: 0x00000000,
        TCD_CSR5: 0x00000000,
        TCD_BITER_ELINKNO5: 0x00000000,
        TCD_SADDR6: 0x00000000,
        TCD_SOFF6: 0x00000000,
        TCD_ATTR6: 0x00000000,
        TCD_NBYTES_MLNO6: 0x00000000,
        TCD_SLAST6: 0x00000000,
        TCD_DADDR6: 0x00000000,
        TCD_DOFF6: 0x00000000,
        TCD_CITER_ELINKNO6: 0x00000000,
        TCD_DLASTSGA6: 0x00000000,
        TCD_CSR6: 0x00000000,
        TCD_BITER_ELINKNO6: 0x00000000,
        TCD_SADDR7: 0x00000000,
        TCD_SOFF7: 0x00000000,
        TCD_ATTR7: 0x00000000,
        TCD_NBYTES_MLNO7: 0x00000000,
        TCD_SLAST7: 0x00000000,
        TCD_DADDR7: 0x00000000,
        TCD_DOFF7: 0x00000000,
        TCD_CITER_ELINKNO7: 0x00000000,
        TCD_DLASTSGA7: 0x00000000,
        TCD_CSR7: 0x00000000,
        TCD_BITER_ELINKNO7: 0x00000000,
        TCD_SADDR8: 0x00000000,
        TCD_SOFF8: 0x00000000,
        TCD_ATTR8: 0x00000000,
        TCD_NBYTES_MLNO8: 0x00000000,
        TCD_SLAST8: 0x00000000,
        TCD_DADDR8: 0x00000000,
        TCD_DOFF8: 0x00000000,
        TCD_CITER_ELINKNO8: 0x00000000,
        TCD_DLASTSGA8: 0x00000000,
        TCD_CSR8: 0x00000000,
        TCD_BITER_ELINKNO8: 0x00000000,
        TCD_SADDR9: 0x00000000,
        TCD_SOFF9: 0x00000000,
        TCD_ATTR9: 0x00000000,
        TCD_NBYTES_MLNO9: 0x00000000,
        TCD_SLAST9: 0x00000000,
        TCD_DADDR9: 0x00000000,
        TCD_DOFF9: 0x00000000,
        TCD_CITER_ELINKNO9: 0x00000000,
        TCD_DLASTSGA9: 0x00000000,
        TCD_CSR9: 0x00000000,
        TCD_BITER_ELINKNO9: 0x00000000,
        TCD_SADDR10: 0x00000000,
        TCD_SOFF10: 0x00000000,
        TCD_ATTR10: 0x00000000,
        TCD_NBYTES_MLNO10: 0x00000000,
        TCD_SLAST10: 0x00000000,
        TCD_DADDR10: 0x00000000,
        TCD_DOFF10: 0x00000000,
        TCD_CITER_ELINKNO10: 0x00000000,
        TCD_DLASTSGA10: 0x00000000,
        TCD_CSR10: 0x00000000,
        TCD_BITER_ELINKNO10: 0x00000000,
        TCD_SADDR11: 0x00000000,
        TCD_SOFF11: 0x00000000,
        TCD_ATTR11: 0x00000000,
        TCD_NBYTES_MLNO11: 0x00000000,
        TCD_SLAST11: 0x00000000,
        TCD_DADDR11: 0x00000000,
        TCD_DOFF11: 0x00000000,
        TCD_CITER_ELINKNO11: 0x00000000,
        TCD_DLASTSGA11: 0x00000000,
        TCD_CSR11: 0x00000000,
        TCD_BITER_ELINKNO11: 0x00000000,
        TCD_SADDR12: 0x00000000,
        TCD_SOFF12: 0x00000000,
        TCD_ATTR12: 0x00000000,
        TCD_NBYTES_MLNO12: 0x00000000,
        TCD_SLAST12: 0x00000000,
        TCD_DADDR12: 0x00000000,
        TCD_DOFF12: 0x00000000,
        TCD_CITER_ELINKNO12: 0x00000000,
        TCD_DLASTSGA12: 0x00000000,
        TCD_CSR12: 0x00000000,
        TCD_BITER_ELINKNO12: 0x00000000,
        TCD_SADDR13: 0x00000000,
        TCD_SOFF13: 0x00000000,
        TCD_ATTR13: 0x00000000,
        TCD_NBYTES_MLNO13: 0x00000000,
        TCD_SLAST13: 0x00000000,
        TCD_DADDR13: 0x00000000,
        TCD_DOFF13: 0x00000000,
        TCD_CITER_ELINKNO13: 0x00000000,
        TCD_DLASTSGA13: 0x00000000,
        TCD_CSR13: 0x00000000,
        TCD_BITER_ELINKNO13: 0x00000000,
        TCD_SADDR14: 0x00000000,
        TCD_SOFF14: 0x00000000,
        TCD_ATTR14: 0x00000000,
        TCD_NBYTES_MLNO14: 0x00000000,
        TCD_SLAST14: 0x00000000,
        TCD_DADDR14: 0x00000000,
        TCD_DOFF14: 0x00000000,
        TCD_CITER_ELINKNO14: 0x00000000,
        TCD_DLASTSGA14: 0x00000000,
        TCD_CSR14: 0x00000000,
        TCD_BITER_ELINKNO14: 0x00000000,
        TCD_SADDR15: 0x00000000,
        TCD_SOFF15: 0x00000000,
        TCD_ATTR15: 0x00000000,
        TCD_NBYTES_MLNO15: 0x00000000,
        TCD_SLAST15: 0x00000000,
        TCD_DADDR15: 0x00000000,
        TCD_DOFF15: 0x00000000,
        TCD_CITER_ELINKNO15: 0x00000000,
        TCD_DLASTSGA15: 0x00000000,
        TCD_CSR15: 0x00000000,
        TCD_BITER_ELINKNO15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA0_TAKEN: bool = false;

    /// Safe access to DMA0
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
            if DMA0_TAKEN {
                None
            } else {
                DMA0_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA0_TAKEN && inst.addr == INSTANCE.addr {
                DMA0_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA0_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA0: *const RegisterBlock = 0x400e8000 as *const _;
