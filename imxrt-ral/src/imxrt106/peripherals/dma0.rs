#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

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

            /// 0b0: Fixed priority arbitration is used for channel selection within each group.
            pub const ERCA_0: u32 = 0b0;

            /// 0b1: Round robin arbitration is used for channel selection within each group.
            pub const ERCA_1: u32 = 0b1;
        }
    }

    /// Enable Round Robin Group Arbitration
    pub mod ERGA {
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

            /// 0b0: Fixed priority arbitration is used for selection among the groups.
            pub const ERGA_0: u32 = 0b0;

            /// 0b1: Round robin arbitration is used for selection among the groups.
            pub const ERGA_1: u32 = 0b1;
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

    /// Channel Group 0 Priority
    pub mod GRP0PRI {
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

    /// Channel Group 1 Priority
    pub mod GRP1PRI {
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
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
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

            /// 0b1: The last recorded error was a configuration error in the channel priorities within a group. Channel priorities within a group are not unique.
            pub const CPE_1: u32 = 0b1;
        }
    }

    /// Group Priority Error
    pub mod GPE {
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

            /// 0b0: No group priority error
            pub const GPE_0: u32 = 0b0;

            /// 0b1: The last recorded error was a configuration error among the group priorities. All group priorities are not unique.
            pub const GPE_1: u32 = 0b1;
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

    /// Enable DMA Request 16
    pub mod ERQ16 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ16_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ16_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 17
    pub mod ERQ17 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ17_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ17_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 18
    pub mod ERQ18 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ18_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ18_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 19
    pub mod ERQ19 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ19_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ19_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 20
    pub mod ERQ20 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ20_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ20_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 21
    pub mod ERQ21 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ21_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ21_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 22
    pub mod ERQ22 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ22_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ22_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 23
    pub mod ERQ23 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ23_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ23_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 24
    pub mod ERQ24 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ24_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ24_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 25
    pub mod ERQ25 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ25_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ25_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 26
    pub mod ERQ26 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ26_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ26_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 27
    pub mod ERQ27 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ27_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ27_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 28
    pub mod ERQ28 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ28_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ28_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 29
    pub mod ERQ29 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ29_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ29_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 30
    pub mod ERQ30 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ30_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ30_1: u32 = 0b1;
        }
    }

    /// Enable DMA Request 31
    pub mod ERQ31 {
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

            /// 0b0: The DMA request signal for the corresponding channel is disabled
            pub const ERQ31_0: u32 = 0b0;

            /// 0b1: The DMA request signal for the corresponding channel is enabled
            pub const ERQ31_1: u32 = 0b1;
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

    /// Enable Error Interrupt 16
    pub mod EEI16 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI16_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI16_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 17
    pub mod EEI17 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI17_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI17_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 18
    pub mod EEI18 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI18_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI18_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 19
    pub mod EEI19 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI19_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI19_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 20
    pub mod EEI20 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI20_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI20_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 21
    pub mod EEI21 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI21_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI21_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 22
    pub mod EEI22 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI22_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI22_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 23
    pub mod EEI23 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI23_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI23_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 24
    pub mod EEI24 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI24_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI24_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 25
    pub mod EEI25 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI25_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI25_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 26
    pub mod EEI26 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI26_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI26_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 27
    pub mod EEI27 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI27_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI27_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 28
    pub mod EEI28 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI28_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI28_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 29
    pub mod EEI29 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI29_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI29_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 30
    pub mod EEI30 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI30_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI30_1: u32 = 0b1;
        }
    }

    /// Enable Error Interrupt 31
    pub mod EEI31 {
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

            /// 0b0: The error signal for corresponding channel does not generate an error interrupt
            pub const EEI31_0: u32 = 0b0;

            /// 0b1: The assertion of the error signal for corresponding channel generates an error interrupt request
            pub const EEI31_1: u32 = 0b1;
        }
    }
}

/// Clear Enable Error Interrupt Register
pub mod CEEI {

    /// Clear Enable Error Interrupt
    pub mod CEEI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
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

    /// Interrupt Request 16
    pub mod INT16 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT16_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT16_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 17
    pub mod INT17 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT17_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT17_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 18
    pub mod INT18 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT18_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT18_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 19
    pub mod INT19 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT19_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT19_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 20
    pub mod INT20 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT20_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT20_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 21
    pub mod INT21 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT21_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT21_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 22
    pub mod INT22 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT22_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT22_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 23
    pub mod INT23 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT23_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT23_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 24
    pub mod INT24 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT24_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT24_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 25
    pub mod INT25 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT25_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT25_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 26
    pub mod INT26 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT26_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT26_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 27
    pub mod INT27 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT27_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT27_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 28
    pub mod INT28 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT28_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT28_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 29
    pub mod INT29 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT29_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT29_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 30
    pub mod INT30 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT30_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT30_1: u32 = 0b1;
        }
    }

    /// Interrupt Request 31
    pub mod INT31 {
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

            /// 0b0: The interrupt request for corresponding channel is cleared
            pub const INT31_0: u32 = 0b0;

            /// 0b1: The interrupt request for corresponding channel is active
            pub const INT31_1: u32 = 0b1;
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

    /// Error In Channel 16
    pub mod ERR16 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR16_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR16_1: u32 = 0b1;
        }
    }

    /// Error In Channel 17
    pub mod ERR17 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR17_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR17_1: u32 = 0b1;
        }
    }

    /// Error In Channel 18
    pub mod ERR18 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR18_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR18_1: u32 = 0b1;
        }
    }

    /// Error In Channel 19
    pub mod ERR19 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR19_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR19_1: u32 = 0b1;
        }
    }

    /// Error In Channel 20
    pub mod ERR20 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR20_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR20_1: u32 = 0b1;
        }
    }

    /// Error In Channel 21
    pub mod ERR21 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR21_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR21_1: u32 = 0b1;
        }
    }

    /// Error In Channel 22
    pub mod ERR22 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR22_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR22_1: u32 = 0b1;
        }
    }

    /// Error In Channel 23
    pub mod ERR23 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR23_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR23_1: u32 = 0b1;
        }
    }

    /// Error In Channel 24
    pub mod ERR24 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR24_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR24_1: u32 = 0b1;
        }
    }

    /// Error In Channel 25
    pub mod ERR25 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR25_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR25_1: u32 = 0b1;
        }
    }

    /// Error In Channel 26
    pub mod ERR26 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR26_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR26_1: u32 = 0b1;
        }
    }

    /// Error In Channel 27
    pub mod ERR27 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR27_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR27_1: u32 = 0b1;
        }
    }

    /// Error In Channel 28
    pub mod ERR28 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR28_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR28_1: u32 = 0b1;
        }
    }

    /// Error In Channel 29
    pub mod ERR29 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR29_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR29_1: u32 = 0b1;
        }
    }

    /// Error In Channel 30
    pub mod ERR30 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR30_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR30_1: u32 = 0b1;
        }
    }

    /// Error In Channel 31
    pub mod ERR31 {
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

            /// 0b0: An error in this channel has not occurred
            pub const ERR31_0: u32 = 0b0;

            /// 0b1: An error in this channel has occurred
            pub const ERR31_1: u32 = 0b1;
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

    /// Hardware Request Status Channel 16
    pub mod HRS16 {
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

            /// 0b0: A hardware service request for channel 16 is not present
            pub const HRS16_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 16 is present
            pub const HRS16_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 17
    pub mod HRS17 {
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

            /// 0b0: A hardware service request for channel 17 is not present
            pub const HRS17_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 17 is present
            pub const HRS17_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 18
    pub mod HRS18 {
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

            /// 0b0: A hardware service request for channel 18 is not present
            pub const HRS18_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 18 is present
            pub const HRS18_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 19
    pub mod HRS19 {
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

            /// 0b0: A hardware service request for channel 19 is not present
            pub const HRS19_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 19 is present
            pub const HRS19_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 20
    pub mod HRS20 {
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

            /// 0b0: A hardware service request for channel 20 is not present
            pub const HRS20_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 20 is present
            pub const HRS20_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 21
    pub mod HRS21 {
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

            /// 0b0: A hardware service request for channel 21 is not present
            pub const HRS21_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 21 is present
            pub const HRS21_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 22
    pub mod HRS22 {
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

            /// 0b0: A hardware service request for channel 22 is not present
            pub const HRS22_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 22 is present
            pub const HRS22_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 23
    pub mod HRS23 {
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

            /// 0b0: A hardware service request for channel 23 is not present
            pub const HRS23_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 23 is present
            pub const HRS23_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 24
    pub mod HRS24 {
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

            /// 0b0: A hardware service request for channel 24 is not present
            pub const HRS24_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 24 is present
            pub const HRS24_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 25
    pub mod HRS25 {
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

            /// 0b0: A hardware service request for channel 25 is not present
            pub const HRS25_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 25 is present
            pub const HRS25_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 26
    pub mod HRS26 {
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

            /// 0b0: A hardware service request for channel 26 is not present
            pub const HRS26_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 26 is present
            pub const HRS26_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 27
    pub mod HRS27 {
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

            /// 0b0: A hardware service request for channel 27 is not present
            pub const HRS27_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 27 is present
            pub const HRS27_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 28
    pub mod HRS28 {
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

            /// 0b0: A hardware service request for channel 28 is not present
            pub const HRS28_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 28 is present
            pub const HRS28_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 29
    pub mod HRS29 {
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

            /// 0b0: A hardware service request for channel 29 is not preset
            pub const HRS29_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 29 is present
            pub const HRS29_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 30
    pub mod HRS30 {
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

            /// 0b0: A hardware service request for channel 30 is not present
            pub const HRS30_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 30 is present
            pub const HRS30_1: u32 = 0b1;
        }
    }

    /// Hardware Request Status Channel 31
    pub mod HRS31 {
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

            /// 0b0: A hardware service request for channel 31 is not present
            pub const HRS31_0: u32 = 0b0;

            /// 0b1: A hardware service request for channel 31 is present
            pub const HRS31_1: u32 = 0b1;
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

    /// Enable asynchronous DMA request in stop mode for channel 16
    pub mod EDREQ_16 {
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

            /// 0b0: Disable asynchronous DMA request for channel 16
            pub const EDREQ_16_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 16
            pub const EDREQ_16_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 17
    pub mod EDREQ_17 {
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

            /// 0b0: Disable asynchronous DMA request for channel 17
            pub const EDREQ_17_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 17
            pub const EDREQ_17_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 18
    pub mod EDREQ_18 {
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

            /// 0b0: Disable asynchronous DMA request for channel 18
            pub const EDREQ_18_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 18
            pub const EDREQ_18_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 19
    pub mod EDREQ_19 {
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

            /// 0b0: Disable asynchronous DMA request for channel 19
            pub const EDREQ_19_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 19
            pub const EDREQ_19_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 20
    pub mod EDREQ_20 {
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

            /// 0b0: Disable asynchronous DMA request for channel 20
            pub const EDREQ_20_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 20
            pub const EDREQ_20_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 21
    pub mod EDREQ_21 {
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

            /// 0b0: Disable asynchronous DMA request for channel 21
            pub const EDREQ_21_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 21
            pub const EDREQ_21_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 22
    pub mod EDREQ_22 {
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

            /// 0b0: Disable asynchronous DMA request for channel 22
            pub const EDREQ_22_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 22
            pub const EDREQ_22_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 23
    pub mod EDREQ_23 {
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

            /// 0b0: Disable asynchronous DMA request for channel 23
            pub const EDREQ_23_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 23
            pub const EDREQ_23_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 24
    pub mod EDREQ_24 {
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

            /// 0b0: Disable asynchronous DMA request for channel 24
            pub const EDREQ_24_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 24
            pub const EDREQ_24_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 25
    pub mod EDREQ_25 {
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

            /// 0b0: Disable asynchronous DMA request for channel 25
            pub const EDREQ_25_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 25
            pub const EDREQ_25_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 26
    pub mod EDREQ_26 {
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

            /// 0b0: Disable asynchronous DMA request for channel 26
            pub const EDREQ_26_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 26
            pub const EDREQ_26_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 27
    pub mod EDREQ_27 {
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

            /// 0b0: Disable asynchronous DMA request for channel 27
            pub const EDREQ_27_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 27
            pub const EDREQ_27_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 28
    pub mod EDREQ_28 {
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

            /// 0b0: Disable asynchronous DMA request for channel 28
            pub const EDREQ_28_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 28
            pub const EDREQ_28_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 29
    pub mod EDREQ_29 {
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

            /// 0b0: Disable asynchronous DMA request for channel 29
            pub const EDREQ_29_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 29
            pub const EDREQ_29_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 30
    pub mod EDREQ_30 {
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

            /// 0b0: Disable asynchronous DMA request for channel 30
            pub const EDREQ_30_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 30
            pub const EDREQ_30_1: u32 = 0b1;
        }
    }

    /// Enable asynchronous DMA request in stop mode for channel 31
    pub mod EDREQ_31 {
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

            /// 0b0: Disable asynchronous DMA request for channel 31
            pub const EDREQ_31_0: u32 = 0b0;

            /// 0b1: Enable asynchronous DMA request for channel 31
            pub const EDREQ_31_1: u32 = 0b1;
        }
    }
}

/// Channel n Priority Register
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

    /// Channel n Current Group Priority
    pub mod GRPPRI {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
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

/// Channel n Priority Register
pub mod DCHPRI2 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI1 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI0 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI7 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI6 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI5 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI4 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI11 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI10 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI9 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI8 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI15 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI14 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI13 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI12 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI19 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI18 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI17 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI16 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI23 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI22 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI21 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI20 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI27 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI26 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI25 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI24 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI31 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI30 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI29 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
}

/// Channel n Priority Register
pub mod DCHPRI28 {
    pub use super::DCHPRI3::CHPRI;
    pub use super::DCHPRI3::DPA;
    pub use super::DCHPRI3::ECP;
    pub use super::DCHPRI3::GRPPRI;
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
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
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
        /// Mask (5 bits: 0b11111 << 9)
        pub const mask: u32 = 0b11111 << offset;
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

/// TCD Source Address
pub mod TCD_SADDR16 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF16 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR16 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO16 and TCD_NBYTES_MLOFFYES16
/// TCD_NBYTES_MLNO16: TCD_NBYTES_MLNO16 and TCD_NBYTES_MLOFFNO16
/// TCD_NBYTES_MLNO16: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO16: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES16: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO16 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST16 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR16 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF16 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO16 and TCD_CITER_ELINKYES16
/// TCD_CITER_ELINKNO16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO16 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA16 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR16 {
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

/// TCD_BITER_ELINKNO16 and TCD_BITER_ELINKYES16
/// TCD_BITER_ELINKNO16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO16 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR17 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF17 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR17 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO17 and TCD_NBYTES_MLOFFYES17
/// TCD_NBYTES_MLNO17: TCD_NBYTES_MLNO17 and TCD_NBYTES_MLOFFNO17
/// TCD_NBYTES_MLNO17: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO17: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES17: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO17 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST17 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR17 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF17 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO17 and TCD_CITER_ELINKYES17
/// TCD_CITER_ELINKNO17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO17 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA17 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR17 {
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

/// TCD_BITER_ELINKNO17 and TCD_BITER_ELINKYES17
/// TCD_BITER_ELINKNO17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO17 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR18 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF18 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR18 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO18 and TCD_NBYTES_MLOFFYES18
/// TCD_NBYTES_MLNO18: TCD_NBYTES_MLNO18 and TCD_NBYTES_MLOFFNO18
/// TCD_NBYTES_MLNO18: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO18: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES18: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO18 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST18 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR18 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF18 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO18 and TCD_CITER_ELINKYES18
/// TCD_CITER_ELINKNO18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO18 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA18 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR18 {
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

/// TCD_BITER_ELINKNO18 and TCD_BITER_ELINKYES18
/// TCD_BITER_ELINKNO18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO18 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR19 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF19 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR19 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO19 and TCD_NBYTES_MLOFFYES19
/// TCD_NBYTES_MLNO19: TCD_NBYTES_MLNO19 and TCD_NBYTES_MLOFFNO19
/// TCD_NBYTES_MLNO19: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO19: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES19: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO19 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST19 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR19 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF19 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO19 and TCD_CITER_ELINKYES19
/// TCD_CITER_ELINKNO19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO19 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA19 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR19 {
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

/// TCD_BITER_ELINKNO19 and TCD_BITER_ELINKYES19
/// TCD_BITER_ELINKNO19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO19 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR20 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF20 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR20 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO20 and TCD_NBYTES_MLOFFYES20
/// TCD_NBYTES_MLNO20: TCD_NBYTES_MLNO20 and TCD_NBYTES_MLOFFNO20
/// TCD_NBYTES_MLNO20: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO20: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES20: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO20 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST20 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR20 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF20 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO20 and TCD_CITER_ELINKYES20
/// TCD_CITER_ELINKNO20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO20 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA20 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR20 {
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

/// TCD_BITER_ELINKNO20 and TCD_BITER_ELINKYES20
/// TCD_BITER_ELINKNO20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO20 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR21 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF21 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR21 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO21 and TCD_NBYTES_MLOFFYES21
/// TCD_NBYTES_MLNO21: TCD_NBYTES_MLNO21 and TCD_NBYTES_MLOFFNO21
/// TCD_NBYTES_MLNO21: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO21: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES21: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO21 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST21 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR21 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF21 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO21 and TCD_CITER_ELINKYES21
/// TCD_CITER_ELINKNO21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO21 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA21 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR21 {
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

/// TCD_BITER_ELINKNO21 and TCD_BITER_ELINKYES21
/// TCD_BITER_ELINKNO21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO21 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR22 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF22 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR22 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO22 and TCD_NBYTES_MLOFFYES22
/// TCD_NBYTES_MLNO22: TCD_NBYTES_MLNO22 and TCD_NBYTES_MLOFFNO22
/// TCD_NBYTES_MLNO22: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO22: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES22: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO22 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST22 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR22 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF22 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO22 and TCD_CITER_ELINKYES22
/// TCD_CITER_ELINKNO22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO22 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA22 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR22 {
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

/// TCD_BITER_ELINKNO22 and TCD_BITER_ELINKYES22
/// TCD_BITER_ELINKNO22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO22 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR23 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF23 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR23 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO23 and TCD_NBYTES_MLOFFYES23
/// TCD_NBYTES_MLNO23: TCD_NBYTES_MLNO23 and TCD_NBYTES_MLOFFNO23
/// TCD_NBYTES_MLNO23: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO23: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES23: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO23 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST23 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR23 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF23 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO23 and TCD_CITER_ELINKYES23
/// TCD_CITER_ELINKNO23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO23 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA23 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR23 {
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

/// TCD_BITER_ELINKNO23 and TCD_BITER_ELINKYES23
/// TCD_BITER_ELINKNO23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO23 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR24 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF24 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR24 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO24 and TCD_NBYTES_MLOFFYES24
/// TCD_NBYTES_MLNO24: TCD_NBYTES_MLNO24 and TCD_NBYTES_MLOFFNO24
/// TCD_NBYTES_MLNO24: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO24: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES24: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO24 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST24 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR24 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF24 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO24 and TCD_CITER_ELINKYES24
/// TCD_CITER_ELINKNO24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO24 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA24 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR24 {
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

/// TCD_BITER_ELINKNO24 and TCD_BITER_ELINKYES24
/// TCD_BITER_ELINKNO24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO24 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR25 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF25 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR25 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO25 and TCD_NBYTES_MLOFFYES25
/// TCD_NBYTES_MLNO25: TCD_NBYTES_MLNO25 and TCD_NBYTES_MLOFFNO25
/// TCD_NBYTES_MLNO25: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO25: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES25: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO25 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST25 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR25 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF25 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO25 and TCD_CITER_ELINKYES25
/// TCD_CITER_ELINKNO25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO25 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA25 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR25 {
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

/// TCD_BITER_ELINKNO25 and TCD_BITER_ELINKYES25
/// TCD_BITER_ELINKNO25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO25 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR26 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF26 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR26 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO26 and TCD_NBYTES_MLOFFYES26
/// TCD_NBYTES_MLNO26: TCD_NBYTES_MLNO26 and TCD_NBYTES_MLOFFNO26
/// TCD_NBYTES_MLNO26: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO26: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES26: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO26 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST26 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR26 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF26 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO26 and TCD_CITER_ELINKYES26
/// TCD_CITER_ELINKNO26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO26 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA26 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR26 {
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

/// TCD_BITER_ELINKNO26 and TCD_BITER_ELINKYES26
/// TCD_BITER_ELINKNO26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO26 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR27 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF27 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR27 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO27 and TCD_NBYTES_MLOFFYES27
/// TCD_NBYTES_MLNO27: TCD_NBYTES_MLNO27 and TCD_NBYTES_MLOFFNO27
/// TCD_NBYTES_MLNO27: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO27: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES27: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO27 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST27 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR27 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF27 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO27 and TCD_CITER_ELINKYES27
/// TCD_CITER_ELINKNO27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO27 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA27 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR27 {
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

/// TCD_BITER_ELINKNO27 and TCD_BITER_ELINKYES27
/// TCD_BITER_ELINKNO27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO27 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR28 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF28 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR28 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO28 and TCD_NBYTES_MLOFFYES28
/// TCD_NBYTES_MLNO28: TCD_NBYTES_MLNO28 and TCD_NBYTES_MLOFFNO28
/// TCD_NBYTES_MLNO28: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO28: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES28: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO28 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST28 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR28 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF28 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO28 and TCD_CITER_ELINKYES28
/// TCD_CITER_ELINKNO28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO28 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA28 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR28 {
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

/// TCD_BITER_ELINKNO28 and TCD_BITER_ELINKYES28
/// TCD_BITER_ELINKNO28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO28 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR29 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF29 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR29 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO29 and TCD_NBYTES_MLOFFYES29
/// TCD_NBYTES_MLNO29: TCD_NBYTES_MLNO29 and TCD_NBYTES_MLOFFNO29
/// TCD_NBYTES_MLNO29: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO29: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES29: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO29 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST29 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR29 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF29 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO29 and TCD_CITER_ELINKYES29
/// TCD_CITER_ELINKNO29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO29 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA29 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR29 {
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

/// TCD_BITER_ELINKNO29 and TCD_BITER_ELINKYES29
/// TCD_BITER_ELINKNO29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO29 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR30 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF30 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR30 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO30 and TCD_NBYTES_MLOFFYES30
/// TCD_NBYTES_MLNO30: TCD_NBYTES_MLNO30 and TCD_NBYTES_MLOFFNO30
/// TCD_NBYTES_MLNO30: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO30: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES30: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO30 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST30 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR30 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF30 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO30 and TCD_CITER_ELINKYES30
/// TCD_CITER_ELINKNO30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO30 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA30 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR30 {
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

/// TCD_BITER_ELINKNO30 and TCD_BITER_ELINKYES30
/// TCD_BITER_ELINKNO30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO30 {
    pub use super::TCD_BITER_ELINK::BITER;
    pub use super::TCD_BITER_ELINK::ELINK;
    pub use super::TCD_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD_SADDR31 {
    pub use super::TCD_SADDR0::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD_SOFF31 {
    pub use super::TCD_SOFF0::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD_ATTR31 {
    pub use super::TCD_ATTR0::DMOD;
    pub use super::TCD_ATTR0::DSIZE;
    pub use super::TCD_ATTR0::SMOD;
    pub use super::TCD_ATTR0::SSIZE;
}

/// TCD_NBYTES_MLNO31 and TCD_NBYTES_MLOFFYES31
/// TCD_NBYTES_MLNO31: TCD_NBYTES_MLNO31 and TCD_NBYTES_MLOFFNO31
/// TCD_NBYTES_MLNO31: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD_NBYTES_MLOFFNO31: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD_NBYTES_MLOFFYES31: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD_NBYTES_MLNO31 {
    pub use super::TCD_NBYTES_ML::DMLOE;
    pub use super::TCD_NBYTES_ML::MLOFF;
    pub use super::TCD_NBYTES_ML::NBYTES;
    pub use super::TCD_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD_SLAST31 {
    pub use super::TCD_SLAST0::SLAST;
}

/// TCD Destination Address
pub mod TCD_DADDR31 {
    pub use super::TCD_DADDR0::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD_DOFF31 {
    pub use super::TCD_DOFF0::DOFF;
}

/// TCD_CITER_ELINKNO31 and TCD_CITER_ELINKYES31
/// TCD_CITER_ELINKNO31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_CITER_ELINKYES31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_CITER_ELINKNO31 {
    pub use super::TCD_CITER_ELINK::CITER;
    pub use super::TCD_CITER_ELINK::ELINK;
    pub use super::TCD_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD_DLASTSGA31 {
    pub use super::TCD_DLASTSGA0::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD_CSR31 {
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

/// TCD_BITER_ELINKNO31 and TCD_BITER_ELINKYES31
/// TCD_BITER_ELINKNO31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD_BITER_ELINKYES31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD_BITER_ELINKNO31 {
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

    /// Channel n Priority Register
    pub DCHPRI3: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI2: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI1: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI0: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI7: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI6: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI5: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI4: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI11: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI10: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI9: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI8: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI15: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI14: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI13: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI12: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI19: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI18: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI17: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI16: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI23: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI22: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI21: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI20: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI27: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI26: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI25: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI24: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI31: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI30: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI29: RWRegister<u8>,

    /// Channel n Priority Register
    pub DCHPRI28: RWRegister<u8>,

    _reserved8: [u32; 952],

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

    /// TCD Source Address
    pub TCD_SADDR16: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF16: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR16: RWRegister<u16>,

    /// TCD_NBYTES_MLNO16 and TCD_NBYTES_MLOFFYES16
    /// TCD_NBYTES_MLNO16: TCD_NBYTES_MLNO16 and TCD_NBYTES_MLOFFNO16
    /// TCD_NBYTES_MLNO16: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO16: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES16: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO16: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST16: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR16: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF16: RWRegister<u16>,

    /// TCD_CITER_ELINKNO16 and TCD_CITER_ELINKYES16
    /// TCD_CITER_ELINKNO16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES16: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO16: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA16: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR16: RWRegister<u16>,

    /// TCD_BITER_ELINKNO16 and TCD_BITER_ELINKYES16
    /// TCD_BITER_ELINKNO16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES16: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO16: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR17: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF17: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR17: RWRegister<u16>,

    /// TCD_NBYTES_MLNO17 and TCD_NBYTES_MLOFFYES17
    /// TCD_NBYTES_MLNO17: TCD_NBYTES_MLNO17 and TCD_NBYTES_MLOFFNO17
    /// TCD_NBYTES_MLNO17: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO17: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES17: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO17: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST17: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR17: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF17: RWRegister<u16>,

    /// TCD_CITER_ELINKNO17 and TCD_CITER_ELINKYES17
    /// TCD_CITER_ELINKNO17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES17: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO17: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA17: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR17: RWRegister<u16>,

    /// TCD_BITER_ELINKNO17 and TCD_BITER_ELINKYES17
    /// TCD_BITER_ELINKNO17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES17: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO17: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR18: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF18: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR18: RWRegister<u16>,

    /// TCD_NBYTES_MLNO18 and TCD_NBYTES_MLOFFYES18
    /// TCD_NBYTES_MLNO18: TCD_NBYTES_MLNO18 and TCD_NBYTES_MLOFFNO18
    /// TCD_NBYTES_MLNO18: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO18: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES18: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO18: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST18: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR18: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF18: RWRegister<u16>,

    /// TCD_CITER_ELINKNO18 and TCD_CITER_ELINKYES18
    /// TCD_CITER_ELINKNO18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES18: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO18: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA18: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR18: RWRegister<u16>,

    /// TCD_BITER_ELINKNO18 and TCD_BITER_ELINKYES18
    /// TCD_BITER_ELINKNO18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES18: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO18: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR19: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF19: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR19: RWRegister<u16>,

    /// TCD_NBYTES_MLNO19 and TCD_NBYTES_MLOFFYES19
    /// TCD_NBYTES_MLNO19: TCD_NBYTES_MLNO19 and TCD_NBYTES_MLOFFNO19
    /// TCD_NBYTES_MLNO19: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO19: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES19: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO19: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST19: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR19: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF19: RWRegister<u16>,

    /// TCD_CITER_ELINKNO19 and TCD_CITER_ELINKYES19
    /// TCD_CITER_ELINKNO19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES19: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO19: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA19: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR19: RWRegister<u16>,

    /// TCD_BITER_ELINKNO19 and TCD_BITER_ELINKYES19
    /// TCD_BITER_ELINKNO19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES19: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO19: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR20: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF20: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR20: RWRegister<u16>,

    /// TCD_NBYTES_MLNO20 and TCD_NBYTES_MLOFFYES20
    /// TCD_NBYTES_MLNO20: TCD_NBYTES_MLNO20 and TCD_NBYTES_MLOFFNO20
    /// TCD_NBYTES_MLNO20: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO20: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES20: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO20: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST20: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR20: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF20: RWRegister<u16>,

    /// TCD_CITER_ELINKNO20 and TCD_CITER_ELINKYES20
    /// TCD_CITER_ELINKNO20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES20: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO20: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA20: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR20: RWRegister<u16>,

    /// TCD_BITER_ELINKNO20 and TCD_BITER_ELINKYES20
    /// TCD_BITER_ELINKNO20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES20: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO20: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR21: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF21: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR21: RWRegister<u16>,

    /// TCD_NBYTES_MLNO21 and TCD_NBYTES_MLOFFYES21
    /// TCD_NBYTES_MLNO21: TCD_NBYTES_MLNO21 and TCD_NBYTES_MLOFFNO21
    /// TCD_NBYTES_MLNO21: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO21: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES21: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO21: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST21: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR21: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF21: RWRegister<u16>,

    /// TCD_CITER_ELINKNO21 and TCD_CITER_ELINKYES21
    /// TCD_CITER_ELINKNO21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES21: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO21: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA21: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR21: RWRegister<u16>,

    /// TCD_BITER_ELINKNO21 and TCD_BITER_ELINKYES21
    /// TCD_BITER_ELINKNO21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES21: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO21: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR22: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF22: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR22: RWRegister<u16>,

    /// TCD_NBYTES_MLNO22 and TCD_NBYTES_MLOFFYES22
    /// TCD_NBYTES_MLNO22: TCD_NBYTES_MLNO22 and TCD_NBYTES_MLOFFNO22
    /// TCD_NBYTES_MLNO22: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO22: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES22: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO22: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST22: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR22: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF22: RWRegister<u16>,

    /// TCD_CITER_ELINKNO22 and TCD_CITER_ELINKYES22
    /// TCD_CITER_ELINKNO22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES22: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO22: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA22: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR22: RWRegister<u16>,

    /// TCD_BITER_ELINKNO22 and TCD_BITER_ELINKYES22
    /// TCD_BITER_ELINKNO22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES22: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO22: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR23: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF23: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR23: RWRegister<u16>,

    /// TCD_NBYTES_MLNO23 and TCD_NBYTES_MLOFFYES23
    /// TCD_NBYTES_MLNO23: TCD_NBYTES_MLNO23 and TCD_NBYTES_MLOFFNO23
    /// TCD_NBYTES_MLNO23: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO23: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES23: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO23: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST23: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR23: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF23: RWRegister<u16>,

    /// TCD_CITER_ELINKNO23 and TCD_CITER_ELINKYES23
    /// TCD_CITER_ELINKNO23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES23: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO23: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA23: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR23: RWRegister<u16>,

    /// TCD_BITER_ELINKNO23 and TCD_BITER_ELINKYES23
    /// TCD_BITER_ELINKNO23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES23: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO23: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR24: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF24: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR24: RWRegister<u16>,

    /// TCD_NBYTES_MLNO24 and TCD_NBYTES_MLOFFYES24
    /// TCD_NBYTES_MLNO24: TCD_NBYTES_MLNO24 and TCD_NBYTES_MLOFFNO24
    /// TCD_NBYTES_MLNO24: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO24: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES24: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO24: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST24: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR24: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF24: RWRegister<u16>,

    /// TCD_CITER_ELINKNO24 and TCD_CITER_ELINKYES24
    /// TCD_CITER_ELINKNO24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES24: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO24: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA24: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR24: RWRegister<u16>,

    /// TCD_BITER_ELINKNO24 and TCD_BITER_ELINKYES24
    /// TCD_BITER_ELINKNO24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES24: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO24: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR25: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF25: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR25: RWRegister<u16>,

    /// TCD_NBYTES_MLNO25 and TCD_NBYTES_MLOFFYES25
    /// TCD_NBYTES_MLNO25: TCD_NBYTES_MLNO25 and TCD_NBYTES_MLOFFNO25
    /// TCD_NBYTES_MLNO25: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO25: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES25: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO25: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST25: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR25: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF25: RWRegister<u16>,

    /// TCD_CITER_ELINKNO25 and TCD_CITER_ELINKYES25
    /// TCD_CITER_ELINKNO25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES25: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO25: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA25: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR25: RWRegister<u16>,

    /// TCD_BITER_ELINKNO25 and TCD_BITER_ELINKYES25
    /// TCD_BITER_ELINKNO25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES25: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO25: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR26: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF26: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR26: RWRegister<u16>,

    /// TCD_NBYTES_MLNO26 and TCD_NBYTES_MLOFFYES26
    /// TCD_NBYTES_MLNO26: TCD_NBYTES_MLNO26 and TCD_NBYTES_MLOFFNO26
    /// TCD_NBYTES_MLNO26: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO26: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES26: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO26: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST26: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR26: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF26: RWRegister<u16>,

    /// TCD_CITER_ELINKNO26 and TCD_CITER_ELINKYES26
    /// TCD_CITER_ELINKNO26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES26: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO26: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA26: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR26: RWRegister<u16>,

    /// TCD_BITER_ELINKNO26 and TCD_BITER_ELINKYES26
    /// TCD_BITER_ELINKNO26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES26: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO26: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR27: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF27: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR27: RWRegister<u16>,

    /// TCD_NBYTES_MLNO27 and TCD_NBYTES_MLOFFYES27
    /// TCD_NBYTES_MLNO27: TCD_NBYTES_MLNO27 and TCD_NBYTES_MLOFFNO27
    /// TCD_NBYTES_MLNO27: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO27: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES27: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO27: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST27: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR27: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF27: RWRegister<u16>,

    /// TCD_CITER_ELINKNO27 and TCD_CITER_ELINKYES27
    /// TCD_CITER_ELINKNO27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES27: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO27: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA27: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR27: RWRegister<u16>,

    /// TCD_BITER_ELINKNO27 and TCD_BITER_ELINKYES27
    /// TCD_BITER_ELINKNO27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES27: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO27: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR28: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF28: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR28: RWRegister<u16>,

    /// TCD_NBYTES_MLNO28 and TCD_NBYTES_MLOFFYES28
    /// TCD_NBYTES_MLNO28: TCD_NBYTES_MLNO28 and TCD_NBYTES_MLOFFNO28
    /// TCD_NBYTES_MLNO28: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO28: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES28: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO28: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST28: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR28: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF28: RWRegister<u16>,

    /// TCD_CITER_ELINKNO28 and TCD_CITER_ELINKYES28
    /// TCD_CITER_ELINKNO28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES28: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO28: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA28: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR28: RWRegister<u16>,

    /// TCD_BITER_ELINKNO28 and TCD_BITER_ELINKYES28
    /// TCD_BITER_ELINKNO28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES28: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO28: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR29: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF29: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR29: RWRegister<u16>,

    /// TCD_NBYTES_MLNO29 and TCD_NBYTES_MLOFFYES29
    /// TCD_NBYTES_MLNO29: TCD_NBYTES_MLNO29 and TCD_NBYTES_MLOFFNO29
    /// TCD_NBYTES_MLNO29: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO29: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES29: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO29: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST29: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR29: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF29: RWRegister<u16>,

    /// TCD_CITER_ELINKNO29 and TCD_CITER_ELINKYES29
    /// TCD_CITER_ELINKNO29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES29: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO29: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA29: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR29: RWRegister<u16>,

    /// TCD_BITER_ELINKNO29 and TCD_BITER_ELINKYES29
    /// TCD_BITER_ELINKNO29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES29: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO29: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR30: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF30: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR30: RWRegister<u16>,

    /// TCD_NBYTES_MLNO30 and TCD_NBYTES_MLOFFYES30
    /// TCD_NBYTES_MLNO30: TCD_NBYTES_MLNO30 and TCD_NBYTES_MLOFFNO30
    /// TCD_NBYTES_MLNO30: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO30: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES30: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO30: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST30: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR30: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF30: RWRegister<u16>,

    /// TCD_CITER_ELINKNO30 and TCD_CITER_ELINKYES30
    /// TCD_CITER_ELINKNO30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES30: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO30: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA30: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR30: RWRegister<u16>,

    /// TCD_BITER_ELINKNO30 and TCD_BITER_ELINKYES30
    /// TCD_BITER_ELINKNO30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES30: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO30: RWRegister<u16>,

    /// TCD Source Address
    pub TCD_SADDR31: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD_SOFF31: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD_ATTR31: RWRegister<u16>,

    /// TCD_NBYTES_MLNO31 and TCD_NBYTES_MLOFFYES31
    /// TCD_NBYTES_MLNO31: TCD_NBYTES_MLNO31 and TCD_NBYTES_MLOFFNO31
    /// TCD_NBYTES_MLNO31: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD_NBYTES_MLOFFNO31: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD_NBYTES_MLOFFYES31: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD_NBYTES_MLNO31: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD_SLAST31: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD_DADDR31: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD_DOFF31: RWRegister<u16>,

    /// TCD_CITER_ELINKNO31 and TCD_CITER_ELINKYES31
    /// TCD_CITER_ELINKNO31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_CITER_ELINKYES31: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_CITER_ELINKNO31: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD_DLASTSGA31: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD_CSR31: RWRegister<u16>,

    /// TCD_BITER_ELINKNO31 and TCD_BITER_ELINKYES31
    /// TCD_BITER_ELINKNO31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD_BITER_ELINKYES31: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD_BITER_ELINKNO31: RWRegister<u16>,
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
    pub DCHPRI19: u8,
    pub DCHPRI18: u8,
    pub DCHPRI17: u8,
    pub DCHPRI16: u8,
    pub DCHPRI23: u8,
    pub DCHPRI22: u8,
    pub DCHPRI21: u8,
    pub DCHPRI20: u8,
    pub DCHPRI27: u8,
    pub DCHPRI26: u8,
    pub DCHPRI25: u8,
    pub DCHPRI24: u8,
    pub DCHPRI31: u8,
    pub DCHPRI30: u8,
    pub DCHPRI29: u8,
    pub DCHPRI28: u8,
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
    pub TCD_SADDR16: u32,
    pub TCD_SOFF16: u16,
    pub TCD_ATTR16: u16,
    pub TCD_NBYTES_MLNO16: u32,
    pub TCD_SLAST16: u32,
    pub TCD_DADDR16: u32,
    pub TCD_DOFF16: u16,
    pub TCD_CITER_ELINKNO16: u16,
    pub TCD_DLASTSGA16: u32,
    pub TCD_CSR16: u16,
    pub TCD_BITER_ELINKNO16: u16,
    pub TCD_SADDR17: u32,
    pub TCD_SOFF17: u16,
    pub TCD_ATTR17: u16,
    pub TCD_NBYTES_MLNO17: u32,
    pub TCD_SLAST17: u32,
    pub TCD_DADDR17: u32,
    pub TCD_DOFF17: u16,
    pub TCD_CITER_ELINKNO17: u16,
    pub TCD_DLASTSGA17: u32,
    pub TCD_CSR17: u16,
    pub TCD_BITER_ELINKNO17: u16,
    pub TCD_SADDR18: u32,
    pub TCD_SOFF18: u16,
    pub TCD_ATTR18: u16,
    pub TCD_NBYTES_MLNO18: u32,
    pub TCD_SLAST18: u32,
    pub TCD_DADDR18: u32,
    pub TCD_DOFF18: u16,
    pub TCD_CITER_ELINKNO18: u16,
    pub TCD_DLASTSGA18: u32,
    pub TCD_CSR18: u16,
    pub TCD_BITER_ELINKNO18: u16,
    pub TCD_SADDR19: u32,
    pub TCD_SOFF19: u16,
    pub TCD_ATTR19: u16,
    pub TCD_NBYTES_MLNO19: u32,
    pub TCD_SLAST19: u32,
    pub TCD_DADDR19: u32,
    pub TCD_DOFF19: u16,
    pub TCD_CITER_ELINKNO19: u16,
    pub TCD_DLASTSGA19: u32,
    pub TCD_CSR19: u16,
    pub TCD_BITER_ELINKNO19: u16,
    pub TCD_SADDR20: u32,
    pub TCD_SOFF20: u16,
    pub TCD_ATTR20: u16,
    pub TCD_NBYTES_MLNO20: u32,
    pub TCD_SLAST20: u32,
    pub TCD_DADDR20: u32,
    pub TCD_DOFF20: u16,
    pub TCD_CITER_ELINKNO20: u16,
    pub TCD_DLASTSGA20: u32,
    pub TCD_CSR20: u16,
    pub TCD_BITER_ELINKNO20: u16,
    pub TCD_SADDR21: u32,
    pub TCD_SOFF21: u16,
    pub TCD_ATTR21: u16,
    pub TCD_NBYTES_MLNO21: u32,
    pub TCD_SLAST21: u32,
    pub TCD_DADDR21: u32,
    pub TCD_DOFF21: u16,
    pub TCD_CITER_ELINKNO21: u16,
    pub TCD_DLASTSGA21: u32,
    pub TCD_CSR21: u16,
    pub TCD_BITER_ELINKNO21: u16,
    pub TCD_SADDR22: u32,
    pub TCD_SOFF22: u16,
    pub TCD_ATTR22: u16,
    pub TCD_NBYTES_MLNO22: u32,
    pub TCD_SLAST22: u32,
    pub TCD_DADDR22: u32,
    pub TCD_DOFF22: u16,
    pub TCD_CITER_ELINKNO22: u16,
    pub TCD_DLASTSGA22: u32,
    pub TCD_CSR22: u16,
    pub TCD_BITER_ELINKNO22: u16,
    pub TCD_SADDR23: u32,
    pub TCD_SOFF23: u16,
    pub TCD_ATTR23: u16,
    pub TCD_NBYTES_MLNO23: u32,
    pub TCD_SLAST23: u32,
    pub TCD_DADDR23: u32,
    pub TCD_DOFF23: u16,
    pub TCD_CITER_ELINKNO23: u16,
    pub TCD_DLASTSGA23: u32,
    pub TCD_CSR23: u16,
    pub TCD_BITER_ELINKNO23: u16,
    pub TCD_SADDR24: u32,
    pub TCD_SOFF24: u16,
    pub TCD_ATTR24: u16,
    pub TCD_NBYTES_MLNO24: u32,
    pub TCD_SLAST24: u32,
    pub TCD_DADDR24: u32,
    pub TCD_DOFF24: u16,
    pub TCD_CITER_ELINKNO24: u16,
    pub TCD_DLASTSGA24: u32,
    pub TCD_CSR24: u16,
    pub TCD_BITER_ELINKNO24: u16,
    pub TCD_SADDR25: u32,
    pub TCD_SOFF25: u16,
    pub TCD_ATTR25: u16,
    pub TCD_NBYTES_MLNO25: u32,
    pub TCD_SLAST25: u32,
    pub TCD_DADDR25: u32,
    pub TCD_DOFF25: u16,
    pub TCD_CITER_ELINKNO25: u16,
    pub TCD_DLASTSGA25: u32,
    pub TCD_CSR25: u16,
    pub TCD_BITER_ELINKNO25: u16,
    pub TCD_SADDR26: u32,
    pub TCD_SOFF26: u16,
    pub TCD_ATTR26: u16,
    pub TCD_NBYTES_MLNO26: u32,
    pub TCD_SLAST26: u32,
    pub TCD_DADDR26: u32,
    pub TCD_DOFF26: u16,
    pub TCD_CITER_ELINKNO26: u16,
    pub TCD_DLASTSGA26: u32,
    pub TCD_CSR26: u16,
    pub TCD_BITER_ELINKNO26: u16,
    pub TCD_SADDR27: u32,
    pub TCD_SOFF27: u16,
    pub TCD_ATTR27: u16,
    pub TCD_NBYTES_MLNO27: u32,
    pub TCD_SLAST27: u32,
    pub TCD_DADDR27: u32,
    pub TCD_DOFF27: u16,
    pub TCD_CITER_ELINKNO27: u16,
    pub TCD_DLASTSGA27: u32,
    pub TCD_CSR27: u16,
    pub TCD_BITER_ELINKNO27: u16,
    pub TCD_SADDR28: u32,
    pub TCD_SOFF28: u16,
    pub TCD_ATTR28: u16,
    pub TCD_NBYTES_MLNO28: u32,
    pub TCD_SLAST28: u32,
    pub TCD_DADDR28: u32,
    pub TCD_DOFF28: u16,
    pub TCD_CITER_ELINKNO28: u16,
    pub TCD_DLASTSGA28: u32,
    pub TCD_CSR28: u16,
    pub TCD_BITER_ELINKNO28: u16,
    pub TCD_SADDR29: u32,
    pub TCD_SOFF29: u16,
    pub TCD_ATTR29: u16,
    pub TCD_NBYTES_MLNO29: u32,
    pub TCD_SLAST29: u32,
    pub TCD_DADDR29: u32,
    pub TCD_DOFF29: u16,
    pub TCD_CITER_ELINKNO29: u16,
    pub TCD_DLASTSGA29: u32,
    pub TCD_CSR29: u16,
    pub TCD_BITER_ELINKNO29: u16,
    pub TCD_SADDR30: u32,
    pub TCD_SOFF30: u16,
    pub TCD_ATTR30: u16,
    pub TCD_NBYTES_MLNO30: u32,
    pub TCD_SLAST30: u32,
    pub TCD_DADDR30: u32,
    pub TCD_DOFF30: u16,
    pub TCD_CITER_ELINKNO30: u16,
    pub TCD_DLASTSGA30: u32,
    pub TCD_CSR30: u16,
    pub TCD_BITER_ELINKNO30: u16,
    pub TCD_SADDR31: u32,
    pub TCD_SOFF31: u16,
    pub TCD_ATTR31: u16,
    pub TCD_NBYTES_MLNO31: u32,
    pub TCD_SLAST31: u32,
    pub TCD_DADDR31: u32,
    pub TCD_DOFF31: u16,
    pub TCD_CITER_ELINKNO31: u16,
    pub TCD_DLASTSGA31: u32,
    pub TCD_CSR31: u16,
    pub TCD_BITER_ELINKNO31: u16,
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
