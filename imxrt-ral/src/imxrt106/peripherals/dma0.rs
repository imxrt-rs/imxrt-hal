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
pub mod TCD0_SADDR {

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
pub mod TCD0_SOFF {

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
pub mod TCD0_ATTR {

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

/// TCD0_NBYTES_ML and TCD0_NBYTES_MLOFFYES
/// TCD0_NBYTES_ML: TCD0_NBYTES_MLNO and TCD0_NBYTES_MLOFFNO
/// TCD0_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD0_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD0_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD0_NBYTES_ML {

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
pub mod TCD0_SLAST {

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
pub mod TCD0_DADDR {

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
pub mod TCD0_DOFF {

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

/// TCD0_CITER_ELINKNO and TCD0_CITER_ELINKYES
/// TCD0_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD0_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD0_CITER_ELINK {

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
pub mod TCD0_DLASTSGA {

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
pub mod TCD0_CSR {

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

/// TCD0_BITER_ELINKNO and TCD0_BITER_ELINKYES
/// TCD0_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD0_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD0_BITER_ELINK {

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
pub mod TCD1_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD1_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD1_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD1_NBYTES_ML and TCD1_NBYTES_MLOFFYES
/// TCD1_NBYTES_ML: TCD1_NBYTES_MLNO and TCD1_NBYTES_MLOFFNO
/// TCD1_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD1_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD1_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD1_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD1_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD1_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD1_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD1_CITER_ELINKNO and TCD1_CITER_ELINKYES
/// TCD1_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD1_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD1_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD1_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD1_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD1_BITER_ELINKNO and TCD1_BITER_ELINKYES
/// TCD1_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD1_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD1_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD2_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD2_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD2_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD2_NBYTES_ML and TCD2_NBYTES_MLOFFYES
/// TCD2_NBYTES_ML: TCD2_NBYTES_MLNO and TCD2_NBYTES_MLOFFNO
/// TCD2_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD2_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD2_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD2_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD2_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD2_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD2_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD2_CITER_ELINKNO and TCD2_CITER_ELINKYES
/// TCD2_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD2_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD2_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD2_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD2_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD2_BITER_ELINKNO and TCD2_BITER_ELINKYES
/// TCD2_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD2_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD2_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD3_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD3_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD3_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD3_NBYTES_ML and TCD3_NBYTES_MLOFFYES
/// TCD3_NBYTES_ML: TCD3_NBYTES_MLNO and TCD3_NBYTES_MLOFFNO
/// TCD3_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD3_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD3_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD3_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD3_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD3_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD3_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD3_CITER_ELINKNO and TCD3_CITER_ELINKYES
/// TCD3_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD3_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD3_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD3_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD3_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD3_BITER_ELINKNO and TCD3_BITER_ELINKYES
/// TCD3_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD3_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD3_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD4_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD4_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD4_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD4_NBYTES_ML and TCD4_NBYTES_MLOFFYES
/// TCD4_NBYTES_ML: TCD4_NBYTES_MLNO and TCD4_NBYTES_MLOFFNO
/// TCD4_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD4_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD4_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD4_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD4_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD4_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD4_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD4_CITER_ELINKNO and TCD4_CITER_ELINKYES
/// TCD4_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD4_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD4_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD4_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD4_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD4_BITER_ELINKNO and TCD4_BITER_ELINKYES
/// TCD4_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD4_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD4_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD5_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD5_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD5_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD5_NBYTES_ML and TCD5_NBYTES_MLOFFYES
/// TCD5_NBYTES_ML: TCD5_NBYTES_MLNO and TCD5_NBYTES_MLOFFNO
/// TCD5_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD5_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD5_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD5_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD5_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD5_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD5_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD5_CITER_ELINKNO and TCD5_CITER_ELINKYES
/// TCD5_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD5_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD5_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD5_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD5_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD5_BITER_ELINKNO and TCD5_BITER_ELINKYES
/// TCD5_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD5_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD5_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD6_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD6_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD6_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD6_NBYTES_ML and TCD6_NBYTES_MLOFFYES
/// TCD6_NBYTES_ML: TCD6_NBYTES_MLNO and TCD6_NBYTES_MLOFFNO
/// TCD6_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD6_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD6_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD6_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD6_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD6_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD6_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD6_CITER_ELINKNO and TCD6_CITER_ELINKYES
/// TCD6_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD6_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD6_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD6_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD6_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD6_BITER_ELINKNO and TCD6_BITER_ELINKYES
/// TCD6_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD6_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD6_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD7_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD7_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD7_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD7_NBYTES_ML and TCD7_NBYTES_MLOFFYES
/// TCD7_NBYTES_ML: TCD7_NBYTES_MLNO and TCD7_NBYTES_MLOFFNO
/// TCD7_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD7_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD7_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD7_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD7_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD7_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD7_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD7_CITER_ELINKNO and TCD7_CITER_ELINKYES
/// TCD7_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD7_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD7_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD7_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD7_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD7_BITER_ELINKNO and TCD7_BITER_ELINKYES
/// TCD7_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD7_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD7_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD8_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD8_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD8_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD8_NBYTES_ML and TCD8_NBYTES_MLOFFYES
/// TCD8_NBYTES_ML: TCD8_NBYTES_MLNO and TCD8_NBYTES_MLOFFNO
/// TCD8_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD8_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD8_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD8_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD8_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD8_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD8_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD8_CITER_ELINKNO and TCD8_CITER_ELINKYES
/// TCD8_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD8_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD8_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD8_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD8_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD8_BITER_ELINKNO and TCD8_BITER_ELINKYES
/// TCD8_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD8_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD8_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD9_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD9_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD9_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD9_NBYTES_ML and TCD9_NBYTES_MLOFFYES
/// TCD9_NBYTES_ML: TCD9_NBYTES_MLNO and TCD9_NBYTES_MLOFFNO
/// TCD9_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD9_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD9_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD9_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD9_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD9_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD9_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD9_CITER_ELINKNO and TCD9_CITER_ELINKYES
/// TCD9_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD9_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD9_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD9_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD9_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD9_BITER_ELINKNO and TCD9_BITER_ELINKYES
/// TCD9_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD9_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD9_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD10_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD10_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD10_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD10_NBYTES_ML and TCD10_NBYTES_MLOFFYES
/// TCD10_NBYTES_ML: TCD10_NBYTES_MLNO and TCD10_NBYTES_MLOFFNO
/// TCD10_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD10_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD10_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD10_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD10_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD10_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD10_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD10_CITER_ELINKNO and TCD10_CITER_ELINKYES
/// TCD10_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD10_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD10_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD10_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD10_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD10_BITER_ELINKNO and TCD10_BITER_ELINKYES
/// TCD10_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD10_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD10_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD11_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD11_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD11_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD11_NBYTES_ML and TCD11_NBYTES_MLOFFYES
/// TCD11_NBYTES_ML: TCD11_NBYTES_MLNO and TCD11_NBYTES_MLOFFNO
/// TCD11_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD11_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD11_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD11_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD11_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD11_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD11_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD11_CITER_ELINKNO and TCD11_CITER_ELINKYES
/// TCD11_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD11_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD11_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD11_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD11_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD11_BITER_ELINKNO and TCD11_BITER_ELINKYES
/// TCD11_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD11_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD11_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD12_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD12_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD12_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD12_NBYTES_ML and TCD12_NBYTES_MLOFFYES
/// TCD12_NBYTES_ML: TCD12_NBYTES_MLNO and TCD12_NBYTES_MLOFFNO
/// TCD12_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD12_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD12_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD12_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD12_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD12_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD12_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD12_CITER_ELINKNO and TCD12_CITER_ELINKYES
/// TCD12_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD12_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD12_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD12_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD12_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD12_BITER_ELINKNO and TCD12_BITER_ELINKYES
/// TCD12_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD12_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD12_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD13_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD13_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD13_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD13_NBYTES_ML and TCD13_NBYTES_MLOFFYES
/// TCD13_NBYTES_ML: TCD13_NBYTES_MLNO and TCD13_NBYTES_MLOFFNO
/// TCD13_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD13_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD13_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD13_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD13_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD13_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD13_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD13_CITER_ELINKNO and TCD13_CITER_ELINKYES
/// TCD13_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD13_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD13_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD13_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD13_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD13_BITER_ELINKNO and TCD13_BITER_ELINKYES
/// TCD13_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD13_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD13_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD14_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD14_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD14_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD14_NBYTES_ML and TCD14_NBYTES_MLOFFYES
/// TCD14_NBYTES_ML: TCD14_NBYTES_MLNO and TCD14_NBYTES_MLOFFNO
/// TCD14_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD14_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD14_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD14_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD14_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD14_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD14_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD14_CITER_ELINKNO and TCD14_CITER_ELINKYES
/// TCD14_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD14_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD14_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD14_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD14_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD14_BITER_ELINKNO and TCD14_BITER_ELINKYES
/// TCD14_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD14_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD14_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD15_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD15_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD15_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD15_NBYTES_ML and TCD15_NBYTES_MLOFFYES
/// TCD15_NBYTES_ML: TCD15_NBYTES_MLNO and TCD15_NBYTES_MLOFFNO
/// TCD15_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD15_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD15_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD15_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD15_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD15_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD15_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD15_CITER_ELINKNO and TCD15_CITER_ELINKYES
/// TCD15_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD15_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD15_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD15_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD15_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD15_BITER_ELINKNO and TCD15_BITER_ELINKYES
/// TCD15_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD15_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD15_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD16_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD16_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD16_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD16_NBYTES_ML and TCD16_NBYTES_MLOFFYES
/// TCD16_NBYTES_ML: TCD16_NBYTES_MLNO and TCD16_NBYTES_MLOFFNO
/// TCD16_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD16_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD16_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD16_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD16_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD16_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD16_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD16_CITER_ELINKNO and TCD16_CITER_ELINKYES
/// TCD16_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD16_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD16_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD16_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD16_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD16_BITER_ELINKNO and TCD16_BITER_ELINKYES
/// TCD16_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD16_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD16_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD17_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD17_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD17_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD17_NBYTES_ML and TCD17_NBYTES_MLOFFYES
/// TCD17_NBYTES_ML: TCD17_NBYTES_MLNO and TCD17_NBYTES_MLOFFNO
/// TCD17_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD17_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD17_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD17_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD17_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD17_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD17_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD17_CITER_ELINKNO and TCD17_CITER_ELINKYES
/// TCD17_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD17_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD17_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD17_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD17_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD17_BITER_ELINKNO and TCD17_BITER_ELINKYES
/// TCD17_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD17_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD17_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD18_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD18_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD18_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD18_NBYTES_ML and TCD18_NBYTES_MLOFFYES
/// TCD18_NBYTES_ML: TCD18_NBYTES_MLNO and TCD18_NBYTES_MLOFFNO
/// TCD18_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD18_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD18_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD18_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD18_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD18_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD18_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD18_CITER_ELINKNO and TCD18_CITER_ELINKYES
/// TCD18_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD18_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD18_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD18_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD18_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD18_BITER_ELINKNO and TCD18_BITER_ELINKYES
/// TCD18_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD18_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD18_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD19_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD19_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD19_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD19_NBYTES_ML and TCD19_NBYTES_MLOFFYES
/// TCD19_NBYTES_ML: TCD19_NBYTES_MLNO and TCD19_NBYTES_MLOFFNO
/// TCD19_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD19_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD19_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD19_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD19_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD19_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD19_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD19_CITER_ELINKNO and TCD19_CITER_ELINKYES
/// TCD19_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD19_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD19_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD19_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD19_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD19_BITER_ELINKNO and TCD19_BITER_ELINKYES
/// TCD19_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD19_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD19_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD20_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD20_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD20_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD20_NBYTES_ML and TCD20_NBYTES_MLOFFYES
/// TCD20_NBYTES_ML: TCD20_NBYTES_MLNO and TCD20_NBYTES_MLOFFNO
/// TCD20_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD20_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD20_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD20_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD20_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD20_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD20_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD20_CITER_ELINKNO and TCD20_CITER_ELINKYES
/// TCD20_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD20_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD20_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD20_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD20_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD20_BITER_ELINKNO and TCD20_BITER_ELINKYES
/// TCD20_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD20_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD20_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD21_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD21_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD21_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD21_NBYTES_ML and TCD21_NBYTES_MLOFFYES
/// TCD21_NBYTES_ML: TCD21_NBYTES_MLNO and TCD21_NBYTES_MLOFFNO
/// TCD21_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD21_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD21_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD21_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD21_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD21_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD21_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD21_CITER_ELINKNO and TCD21_CITER_ELINKYES
/// TCD21_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD21_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD21_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD21_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD21_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD21_BITER_ELINKNO and TCD21_BITER_ELINKYES
/// TCD21_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD21_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD21_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD22_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD22_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD22_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD22_NBYTES_ML and TCD22_NBYTES_MLOFFYES
/// TCD22_NBYTES_ML: TCD22_NBYTES_MLNO and TCD22_NBYTES_MLOFFNO
/// TCD22_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD22_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD22_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD22_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD22_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD22_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD22_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD22_CITER_ELINKNO and TCD22_CITER_ELINKYES
/// TCD22_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD22_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD22_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD22_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD22_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD22_BITER_ELINKNO and TCD22_BITER_ELINKYES
/// TCD22_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD22_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD22_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD23_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD23_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD23_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD23_NBYTES_ML and TCD23_NBYTES_MLOFFYES
/// TCD23_NBYTES_ML: TCD23_NBYTES_MLNO and TCD23_NBYTES_MLOFFNO
/// TCD23_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD23_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD23_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD23_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD23_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD23_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD23_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD23_CITER_ELINKNO and TCD23_CITER_ELINKYES
/// TCD23_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD23_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD23_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD23_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD23_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD23_BITER_ELINKNO and TCD23_BITER_ELINKYES
/// TCD23_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD23_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD23_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD24_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD24_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD24_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD24_NBYTES_ML and TCD24_NBYTES_MLOFFYES
/// TCD24_NBYTES_ML: TCD24_NBYTES_MLNO and TCD24_NBYTES_MLOFFNO
/// TCD24_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD24_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD24_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD24_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD24_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD24_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD24_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD24_CITER_ELINKNO and TCD24_CITER_ELINKYES
/// TCD24_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD24_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD24_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD24_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD24_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD24_BITER_ELINKNO and TCD24_BITER_ELINKYES
/// TCD24_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD24_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD24_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD25_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD25_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD25_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD25_NBYTES_ML and TCD25_NBYTES_MLOFFYES
/// TCD25_NBYTES_ML: TCD25_NBYTES_MLNO and TCD25_NBYTES_MLOFFNO
/// TCD25_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD25_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD25_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD25_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD25_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD25_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD25_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD25_CITER_ELINKNO and TCD25_CITER_ELINKYES
/// TCD25_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD25_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD25_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD25_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD25_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD25_BITER_ELINKNO and TCD25_BITER_ELINKYES
/// TCD25_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD25_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD25_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD26_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD26_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD26_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD26_NBYTES_ML and TCD26_NBYTES_MLOFFYES
/// TCD26_NBYTES_ML: TCD26_NBYTES_MLNO and TCD26_NBYTES_MLOFFNO
/// TCD26_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD26_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD26_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD26_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD26_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD26_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD26_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD26_CITER_ELINKNO and TCD26_CITER_ELINKYES
/// TCD26_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD26_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD26_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD26_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD26_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD26_BITER_ELINKNO and TCD26_BITER_ELINKYES
/// TCD26_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD26_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD26_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD27_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD27_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD27_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD27_NBYTES_ML and TCD27_NBYTES_MLOFFYES
/// TCD27_NBYTES_ML: TCD27_NBYTES_MLNO and TCD27_NBYTES_MLOFFNO
/// TCD27_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD27_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD27_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD27_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD27_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD27_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD27_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD27_CITER_ELINKNO and TCD27_CITER_ELINKYES
/// TCD27_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD27_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD27_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD27_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD27_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD27_BITER_ELINKNO and TCD27_BITER_ELINKYES
/// TCD27_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD27_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD27_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD28_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD28_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD28_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD28_NBYTES_ML and TCD28_NBYTES_MLOFFYES
/// TCD28_NBYTES_ML: TCD28_NBYTES_MLNO and TCD28_NBYTES_MLOFFNO
/// TCD28_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD28_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD28_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD28_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD28_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD28_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD28_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD28_CITER_ELINKNO and TCD28_CITER_ELINKYES
/// TCD28_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD28_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD28_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD28_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD28_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD28_BITER_ELINKNO and TCD28_BITER_ELINKYES
/// TCD28_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD28_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD28_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD29_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD29_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD29_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD29_NBYTES_ML and TCD29_NBYTES_MLOFFYES
/// TCD29_NBYTES_ML: TCD29_NBYTES_MLNO and TCD29_NBYTES_MLOFFNO
/// TCD29_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD29_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD29_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD29_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD29_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD29_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD29_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD29_CITER_ELINKNO and TCD29_CITER_ELINKYES
/// TCD29_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD29_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD29_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD29_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD29_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD29_BITER_ELINKNO and TCD29_BITER_ELINKYES
/// TCD29_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD29_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD29_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD30_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD30_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD30_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD30_NBYTES_ML and TCD30_NBYTES_MLOFFYES
/// TCD30_NBYTES_ML: TCD30_NBYTES_MLNO and TCD30_NBYTES_MLOFFNO
/// TCD30_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD30_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD30_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD30_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD30_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD30_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD30_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD30_CITER_ELINKNO and TCD30_CITER_ELINKYES
/// TCD30_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD30_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD30_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD30_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD30_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD30_BITER_ELINKNO and TCD30_BITER_ELINKYES
/// TCD30_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD30_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD30_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
}

/// TCD Source Address
pub mod TCD31_SADDR {
    pub use super::TCD0_SADDR::SADDR;
}

/// TCD Signed Source Address Offset
pub mod TCD31_SOFF {
    pub use super::TCD0_SOFF::SOFF;
}

/// TCD Transfer Attributes
pub mod TCD31_ATTR {
    pub use super::TCD0_ATTR::DMOD;
    pub use super::TCD0_ATTR::DSIZE;
    pub use super::TCD0_ATTR::SMOD;
    pub use super::TCD0_ATTR::SSIZE;
}

/// TCD31_NBYTES_ML and TCD31_NBYTES_MLOFFYES
/// TCD31_NBYTES_ML: TCD31_NBYTES_MLNO and TCD31_NBYTES_MLOFFNO
/// TCD31_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
/// TCD31_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
/// TCD31_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
pub mod TCD31_NBYTES_ML {
    pub use super::TCD0_NBYTES_ML::DMLOE;
    pub use super::TCD0_NBYTES_ML::MLOFF;
    pub use super::TCD0_NBYTES_ML::NBYTES;
    pub use super::TCD0_NBYTES_ML::SMLOE;
}

/// TCD Last Source Address Adjustment
pub mod TCD31_SLAST {
    pub use super::TCD0_SLAST::SLAST;
}

/// TCD Destination Address
pub mod TCD31_DADDR {
    pub use super::TCD0_DADDR::DADDR;
}

/// TCD Signed Destination Address Offset
pub mod TCD31_DOFF {
    pub use super::TCD0_DOFF::DOFF;
}

/// TCD31_CITER_ELINKNO and TCD31_CITER_ELINKYES
/// TCD31_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD31_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD31_CITER_ELINK {
    pub use super::TCD0_CITER_ELINK::CITER;
    pub use super::TCD0_CITER_ELINK::ELINK;
    pub use super::TCD0_CITER_ELINK::LINKCH;
}

/// TCD Last Destination Address Adjustment/Scatter Gather Address
pub mod TCD31_DLASTSGA {
    pub use super::TCD0_DLASTSGA::DLASTSGA;
}

/// TCD Control and Status
pub mod TCD31_CSR {
    pub use super::TCD0_CSR::ACTIVE;
    pub use super::TCD0_CSR::BWC;
    pub use super::TCD0_CSR::DONE;
    pub use super::TCD0_CSR::DREQ;
    pub use super::TCD0_CSR::ESG;
    pub use super::TCD0_CSR::INTHALF;
    pub use super::TCD0_CSR::INTMAJOR;
    pub use super::TCD0_CSR::MAJORELINK;
    pub use super::TCD0_CSR::MAJORLINKCH;
    pub use super::TCD0_CSR::START;
}

/// TCD31_BITER_ELINKNO and TCD31_BITER_ELINKYES
/// TCD31_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
/// TCD31_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
pub mod TCD31_BITER_ELINK {
    pub use super::TCD0_BITER_ELINK::BITER;
    pub use super::TCD0_BITER_ELINK::ELINK;
    pub use super::TCD0_BITER_ELINK::LINKCH;
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
    pub TCD0_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD0_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD0_ATTR: RWRegister<u16>,

    /// TCD0_NBYTES_ML and TCD0_NBYTES_MLOFFYES
    /// TCD0_NBYTES_ML: TCD0_NBYTES_MLNO and TCD0_NBYTES_MLOFFNO
    /// TCD0_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD0_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD0_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD0_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD0_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD0_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD0_DOFF: RWRegister<u16>,

    /// TCD0_CITER_ELINKNO and TCD0_CITER_ELINKYES
    /// TCD0_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD0_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD0_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD0_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD0_CSR: RWRegister<u16>,

    /// TCD0_BITER_ELINKNO and TCD0_BITER_ELINKYES
    /// TCD0_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD0_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD0_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD1_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD1_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD1_ATTR: RWRegister<u16>,

    /// TCD1_NBYTES_ML and TCD1_NBYTES_MLOFFYES
    /// TCD1_NBYTES_ML: TCD1_NBYTES_MLNO and TCD1_NBYTES_MLOFFNO
    /// TCD1_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD1_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD1_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD1_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD1_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD1_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD1_DOFF: RWRegister<u16>,

    /// TCD1_CITER_ELINKNO and TCD1_CITER_ELINKYES
    /// TCD1_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD1_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD1_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD1_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD1_CSR: RWRegister<u16>,

    /// TCD1_BITER_ELINKNO and TCD1_BITER_ELINKYES
    /// TCD1_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD1_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD1_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD2_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD2_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD2_ATTR: RWRegister<u16>,

    /// TCD2_NBYTES_ML and TCD2_NBYTES_MLOFFYES
    /// TCD2_NBYTES_ML: TCD2_NBYTES_MLNO and TCD2_NBYTES_MLOFFNO
    /// TCD2_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD2_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD2_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD2_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD2_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD2_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD2_DOFF: RWRegister<u16>,

    /// TCD2_CITER_ELINKNO and TCD2_CITER_ELINKYES
    /// TCD2_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD2_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD2_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD2_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD2_CSR: RWRegister<u16>,

    /// TCD2_BITER_ELINKNO and TCD2_BITER_ELINKYES
    /// TCD2_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD2_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD2_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD3_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD3_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD3_ATTR: RWRegister<u16>,

    /// TCD3_NBYTES_ML and TCD3_NBYTES_MLOFFYES
    /// TCD3_NBYTES_ML: TCD3_NBYTES_MLNO and TCD3_NBYTES_MLOFFNO
    /// TCD3_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD3_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD3_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD3_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD3_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD3_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD3_DOFF: RWRegister<u16>,

    /// TCD3_CITER_ELINKNO and TCD3_CITER_ELINKYES
    /// TCD3_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD3_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD3_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD3_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD3_CSR: RWRegister<u16>,

    /// TCD3_BITER_ELINKNO and TCD3_BITER_ELINKYES
    /// TCD3_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD3_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD3_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD4_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD4_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD4_ATTR: RWRegister<u16>,

    /// TCD4_NBYTES_ML and TCD4_NBYTES_MLOFFYES
    /// TCD4_NBYTES_ML: TCD4_NBYTES_MLNO and TCD4_NBYTES_MLOFFNO
    /// TCD4_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD4_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD4_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD4_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD4_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD4_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD4_DOFF: RWRegister<u16>,

    /// TCD4_CITER_ELINKNO and TCD4_CITER_ELINKYES
    /// TCD4_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD4_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD4_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD4_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD4_CSR: RWRegister<u16>,

    /// TCD4_BITER_ELINKNO and TCD4_BITER_ELINKYES
    /// TCD4_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD4_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD4_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD5_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD5_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD5_ATTR: RWRegister<u16>,

    /// TCD5_NBYTES_ML and TCD5_NBYTES_MLOFFYES
    /// TCD5_NBYTES_ML: TCD5_NBYTES_MLNO and TCD5_NBYTES_MLOFFNO
    /// TCD5_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD5_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD5_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD5_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD5_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD5_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD5_DOFF: RWRegister<u16>,

    /// TCD5_CITER_ELINKNO and TCD5_CITER_ELINKYES
    /// TCD5_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD5_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD5_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD5_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD5_CSR: RWRegister<u16>,

    /// TCD5_BITER_ELINKNO and TCD5_BITER_ELINKYES
    /// TCD5_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD5_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD5_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD6_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD6_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD6_ATTR: RWRegister<u16>,

    /// TCD6_NBYTES_ML and TCD6_NBYTES_MLOFFYES
    /// TCD6_NBYTES_ML: TCD6_NBYTES_MLNO and TCD6_NBYTES_MLOFFNO
    /// TCD6_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD6_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD6_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD6_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD6_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD6_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD6_DOFF: RWRegister<u16>,

    /// TCD6_CITER_ELINKNO and TCD6_CITER_ELINKYES
    /// TCD6_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD6_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD6_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD6_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD6_CSR: RWRegister<u16>,

    /// TCD6_BITER_ELINKNO and TCD6_BITER_ELINKYES
    /// TCD6_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD6_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD6_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD7_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD7_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD7_ATTR: RWRegister<u16>,

    /// TCD7_NBYTES_ML and TCD7_NBYTES_MLOFFYES
    /// TCD7_NBYTES_ML: TCD7_NBYTES_MLNO and TCD7_NBYTES_MLOFFNO
    /// TCD7_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD7_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD7_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD7_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD7_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD7_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD7_DOFF: RWRegister<u16>,

    /// TCD7_CITER_ELINKNO and TCD7_CITER_ELINKYES
    /// TCD7_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD7_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD7_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD7_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD7_CSR: RWRegister<u16>,

    /// TCD7_BITER_ELINKNO and TCD7_BITER_ELINKYES
    /// TCD7_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD7_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD7_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD8_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD8_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD8_ATTR: RWRegister<u16>,

    /// TCD8_NBYTES_ML and TCD8_NBYTES_MLOFFYES
    /// TCD8_NBYTES_ML: TCD8_NBYTES_MLNO and TCD8_NBYTES_MLOFFNO
    /// TCD8_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD8_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD8_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD8_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD8_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD8_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD8_DOFF: RWRegister<u16>,

    /// TCD8_CITER_ELINKNO and TCD8_CITER_ELINKYES
    /// TCD8_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD8_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD8_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD8_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD8_CSR: RWRegister<u16>,

    /// TCD8_BITER_ELINKNO and TCD8_BITER_ELINKYES
    /// TCD8_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD8_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD8_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD9_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD9_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD9_ATTR: RWRegister<u16>,

    /// TCD9_NBYTES_ML and TCD9_NBYTES_MLOFFYES
    /// TCD9_NBYTES_ML: TCD9_NBYTES_MLNO and TCD9_NBYTES_MLOFFNO
    /// TCD9_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD9_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD9_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD9_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD9_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD9_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD9_DOFF: RWRegister<u16>,

    /// TCD9_CITER_ELINKNO and TCD9_CITER_ELINKYES
    /// TCD9_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD9_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD9_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD9_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD9_CSR: RWRegister<u16>,

    /// TCD9_BITER_ELINKNO and TCD9_BITER_ELINKYES
    /// TCD9_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD9_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD9_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD10_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD10_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD10_ATTR: RWRegister<u16>,

    /// TCD10_NBYTES_ML and TCD10_NBYTES_MLOFFYES
    /// TCD10_NBYTES_ML: TCD10_NBYTES_MLNO and TCD10_NBYTES_MLOFFNO
    /// TCD10_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD10_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD10_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD10_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD10_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD10_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD10_DOFF: RWRegister<u16>,

    /// TCD10_CITER_ELINKNO and TCD10_CITER_ELINKYES
    /// TCD10_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD10_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD10_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD10_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD10_CSR: RWRegister<u16>,

    /// TCD10_BITER_ELINKNO and TCD10_BITER_ELINKYES
    /// TCD10_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD10_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD10_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD11_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD11_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD11_ATTR: RWRegister<u16>,

    /// TCD11_NBYTES_ML and TCD11_NBYTES_MLOFFYES
    /// TCD11_NBYTES_ML: TCD11_NBYTES_MLNO and TCD11_NBYTES_MLOFFNO
    /// TCD11_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD11_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD11_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD11_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD11_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD11_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD11_DOFF: RWRegister<u16>,

    /// TCD11_CITER_ELINKNO and TCD11_CITER_ELINKYES
    /// TCD11_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD11_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD11_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD11_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD11_CSR: RWRegister<u16>,

    /// TCD11_BITER_ELINKNO and TCD11_BITER_ELINKYES
    /// TCD11_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD11_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD11_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD12_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD12_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD12_ATTR: RWRegister<u16>,

    /// TCD12_NBYTES_ML and TCD12_NBYTES_MLOFFYES
    /// TCD12_NBYTES_ML: TCD12_NBYTES_MLNO and TCD12_NBYTES_MLOFFNO
    /// TCD12_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD12_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD12_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD12_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD12_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD12_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD12_DOFF: RWRegister<u16>,

    /// TCD12_CITER_ELINKNO and TCD12_CITER_ELINKYES
    /// TCD12_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD12_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD12_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD12_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD12_CSR: RWRegister<u16>,

    /// TCD12_BITER_ELINKNO and TCD12_BITER_ELINKYES
    /// TCD12_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD12_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD12_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD13_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD13_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD13_ATTR: RWRegister<u16>,

    /// TCD13_NBYTES_ML and TCD13_NBYTES_MLOFFYES
    /// TCD13_NBYTES_ML: TCD13_NBYTES_MLNO and TCD13_NBYTES_MLOFFNO
    /// TCD13_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD13_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD13_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD13_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD13_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD13_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD13_DOFF: RWRegister<u16>,

    /// TCD13_CITER_ELINKNO and TCD13_CITER_ELINKYES
    /// TCD13_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD13_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD13_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD13_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD13_CSR: RWRegister<u16>,

    /// TCD13_BITER_ELINKNO and TCD13_BITER_ELINKYES
    /// TCD13_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD13_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD13_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD14_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD14_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD14_ATTR: RWRegister<u16>,

    /// TCD14_NBYTES_ML and TCD14_NBYTES_MLOFFYES
    /// TCD14_NBYTES_ML: TCD14_NBYTES_MLNO and TCD14_NBYTES_MLOFFNO
    /// TCD14_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD14_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD14_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD14_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD14_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD14_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD14_DOFF: RWRegister<u16>,

    /// TCD14_CITER_ELINKNO and TCD14_CITER_ELINKYES
    /// TCD14_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD14_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD14_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD14_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD14_CSR: RWRegister<u16>,

    /// TCD14_BITER_ELINKNO and TCD14_BITER_ELINKYES
    /// TCD14_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD14_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD14_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD15_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD15_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD15_ATTR: RWRegister<u16>,

    /// TCD15_NBYTES_ML and TCD15_NBYTES_MLOFFYES
    /// TCD15_NBYTES_ML: TCD15_NBYTES_MLNO and TCD15_NBYTES_MLOFFNO
    /// TCD15_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD15_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD15_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD15_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD15_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD15_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD15_DOFF: RWRegister<u16>,

    /// TCD15_CITER_ELINKNO and TCD15_CITER_ELINKYES
    /// TCD15_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD15_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD15_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD15_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD15_CSR: RWRegister<u16>,

    /// TCD15_BITER_ELINKNO and TCD15_BITER_ELINKYES
    /// TCD15_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD15_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD15_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD16_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD16_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD16_ATTR: RWRegister<u16>,

    /// TCD16_NBYTES_ML and TCD16_NBYTES_MLOFFYES
    /// TCD16_NBYTES_ML: TCD16_NBYTES_MLNO and TCD16_NBYTES_MLOFFNO
    /// TCD16_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD16_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD16_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD16_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD16_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD16_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD16_DOFF: RWRegister<u16>,

    /// TCD16_CITER_ELINKNO and TCD16_CITER_ELINKYES
    /// TCD16_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD16_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD16_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD16_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD16_CSR: RWRegister<u16>,

    /// TCD16_BITER_ELINKNO and TCD16_BITER_ELINKYES
    /// TCD16_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD16_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD16_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD17_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD17_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD17_ATTR: RWRegister<u16>,

    /// TCD17_NBYTES_ML and TCD17_NBYTES_MLOFFYES
    /// TCD17_NBYTES_ML: TCD17_NBYTES_MLNO and TCD17_NBYTES_MLOFFNO
    /// TCD17_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD17_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD17_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD17_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD17_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD17_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD17_DOFF: RWRegister<u16>,

    /// TCD17_CITER_ELINKNO and TCD17_CITER_ELINKYES
    /// TCD17_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD17_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD17_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD17_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD17_CSR: RWRegister<u16>,

    /// TCD17_BITER_ELINKNO and TCD17_BITER_ELINKYES
    /// TCD17_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD17_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD17_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD18_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD18_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD18_ATTR: RWRegister<u16>,

    /// TCD18_NBYTES_ML and TCD18_NBYTES_MLOFFYES
    /// TCD18_NBYTES_ML: TCD18_NBYTES_MLNO and TCD18_NBYTES_MLOFFNO
    /// TCD18_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD18_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD18_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD18_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD18_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD18_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD18_DOFF: RWRegister<u16>,

    /// TCD18_CITER_ELINKNO and TCD18_CITER_ELINKYES
    /// TCD18_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD18_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD18_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD18_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD18_CSR: RWRegister<u16>,

    /// TCD18_BITER_ELINKNO and TCD18_BITER_ELINKYES
    /// TCD18_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD18_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD18_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD19_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD19_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD19_ATTR: RWRegister<u16>,

    /// TCD19_NBYTES_ML and TCD19_NBYTES_MLOFFYES
    /// TCD19_NBYTES_ML: TCD19_NBYTES_MLNO and TCD19_NBYTES_MLOFFNO
    /// TCD19_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD19_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD19_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD19_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD19_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD19_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD19_DOFF: RWRegister<u16>,

    /// TCD19_CITER_ELINKNO and TCD19_CITER_ELINKYES
    /// TCD19_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD19_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD19_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD19_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD19_CSR: RWRegister<u16>,

    /// TCD19_BITER_ELINKNO and TCD19_BITER_ELINKYES
    /// TCD19_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD19_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD19_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD20_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD20_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD20_ATTR: RWRegister<u16>,

    /// TCD20_NBYTES_ML and TCD20_NBYTES_MLOFFYES
    /// TCD20_NBYTES_ML: TCD20_NBYTES_MLNO and TCD20_NBYTES_MLOFFNO
    /// TCD20_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD20_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD20_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD20_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD20_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD20_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD20_DOFF: RWRegister<u16>,

    /// TCD20_CITER_ELINKNO and TCD20_CITER_ELINKYES
    /// TCD20_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD20_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD20_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD20_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD20_CSR: RWRegister<u16>,

    /// TCD20_BITER_ELINKNO and TCD20_BITER_ELINKYES
    /// TCD20_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD20_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD20_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD21_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD21_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD21_ATTR: RWRegister<u16>,

    /// TCD21_NBYTES_ML and TCD21_NBYTES_MLOFFYES
    /// TCD21_NBYTES_ML: TCD21_NBYTES_MLNO and TCD21_NBYTES_MLOFFNO
    /// TCD21_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD21_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD21_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD21_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD21_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD21_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD21_DOFF: RWRegister<u16>,

    /// TCD21_CITER_ELINKNO and TCD21_CITER_ELINKYES
    /// TCD21_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD21_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD21_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD21_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD21_CSR: RWRegister<u16>,

    /// TCD21_BITER_ELINKNO and TCD21_BITER_ELINKYES
    /// TCD21_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD21_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD21_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD22_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD22_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD22_ATTR: RWRegister<u16>,

    /// TCD22_NBYTES_ML and TCD22_NBYTES_MLOFFYES
    /// TCD22_NBYTES_ML: TCD22_NBYTES_MLNO and TCD22_NBYTES_MLOFFNO
    /// TCD22_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD22_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD22_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD22_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD22_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD22_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD22_DOFF: RWRegister<u16>,

    /// TCD22_CITER_ELINKNO and TCD22_CITER_ELINKYES
    /// TCD22_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD22_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD22_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD22_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD22_CSR: RWRegister<u16>,

    /// TCD22_BITER_ELINKNO and TCD22_BITER_ELINKYES
    /// TCD22_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD22_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD22_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD23_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD23_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD23_ATTR: RWRegister<u16>,

    /// TCD23_NBYTES_ML and TCD23_NBYTES_MLOFFYES
    /// TCD23_NBYTES_ML: TCD23_NBYTES_MLNO and TCD23_NBYTES_MLOFFNO
    /// TCD23_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD23_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD23_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD23_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD23_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD23_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD23_DOFF: RWRegister<u16>,

    /// TCD23_CITER_ELINKNO and TCD23_CITER_ELINKYES
    /// TCD23_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD23_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD23_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD23_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD23_CSR: RWRegister<u16>,

    /// TCD23_BITER_ELINKNO and TCD23_BITER_ELINKYES
    /// TCD23_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD23_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD23_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD24_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD24_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD24_ATTR: RWRegister<u16>,

    /// TCD24_NBYTES_ML and TCD24_NBYTES_MLOFFYES
    /// TCD24_NBYTES_ML: TCD24_NBYTES_MLNO and TCD24_NBYTES_MLOFFNO
    /// TCD24_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD24_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD24_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD24_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD24_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD24_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD24_DOFF: RWRegister<u16>,

    /// TCD24_CITER_ELINKNO and TCD24_CITER_ELINKYES
    /// TCD24_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD24_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD24_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD24_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD24_CSR: RWRegister<u16>,

    /// TCD24_BITER_ELINKNO and TCD24_BITER_ELINKYES
    /// TCD24_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD24_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD24_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD25_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD25_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD25_ATTR: RWRegister<u16>,

    /// TCD25_NBYTES_ML and TCD25_NBYTES_MLOFFYES
    /// TCD25_NBYTES_ML: TCD25_NBYTES_MLNO and TCD25_NBYTES_MLOFFNO
    /// TCD25_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD25_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD25_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD25_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD25_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD25_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD25_DOFF: RWRegister<u16>,

    /// TCD25_CITER_ELINKNO and TCD25_CITER_ELINKYES
    /// TCD25_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD25_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD25_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD25_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD25_CSR: RWRegister<u16>,

    /// TCD25_BITER_ELINKNO and TCD25_BITER_ELINKYES
    /// TCD25_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD25_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD25_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD26_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD26_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD26_ATTR: RWRegister<u16>,

    /// TCD26_NBYTES_ML and TCD26_NBYTES_MLOFFYES
    /// TCD26_NBYTES_ML: TCD26_NBYTES_MLNO and TCD26_NBYTES_MLOFFNO
    /// TCD26_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD26_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD26_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD26_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD26_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD26_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD26_DOFF: RWRegister<u16>,

    /// TCD26_CITER_ELINKNO and TCD26_CITER_ELINKYES
    /// TCD26_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD26_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD26_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD26_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD26_CSR: RWRegister<u16>,

    /// TCD26_BITER_ELINKNO and TCD26_BITER_ELINKYES
    /// TCD26_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD26_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD26_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD27_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD27_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD27_ATTR: RWRegister<u16>,

    /// TCD27_NBYTES_ML and TCD27_NBYTES_MLOFFYES
    /// TCD27_NBYTES_ML: TCD27_NBYTES_MLNO and TCD27_NBYTES_MLOFFNO
    /// TCD27_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD27_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD27_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD27_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD27_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD27_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD27_DOFF: RWRegister<u16>,

    /// TCD27_CITER_ELINKNO and TCD27_CITER_ELINKYES
    /// TCD27_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD27_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD27_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD27_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD27_CSR: RWRegister<u16>,

    /// TCD27_BITER_ELINKNO and TCD27_BITER_ELINKYES
    /// TCD27_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD27_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD27_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD28_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD28_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD28_ATTR: RWRegister<u16>,

    /// TCD28_NBYTES_ML and TCD28_NBYTES_MLOFFYES
    /// TCD28_NBYTES_ML: TCD28_NBYTES_MLNO and TCD28_NBYTES_MLOFFNO
    /// TCD28_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD28_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD28_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD28_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD28_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD28_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD28_DOFF: RWRegister<u16>,

    /// TCD28_CITER_ELINKNO and TCD28_CITER_ELINKYES
    /// TCD28_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD28_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD28_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD28_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD28_CSR: RWRegister<u16>,

    /// TCD28_BITER_ELINKNO and TCD28_BITER_ELINKYES
    /// TCD28_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD28_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD28_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD29_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD29_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD29_ATTR: RWRegister<u16>,

    /// TCD29_NBYTES_ML and TCD29_NBYTES_MLOFFYES
    /// TCD29_NBYTES_ML: TCD29_NBYTES_MLNO and TCD29_NBYTES_MLOFFNO
    /// TCD29_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD29_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD29_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD29_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD29_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD29_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD29_DOFF: RWRegister<u16>,

    /// TCD29_CITER_ELINKNO and TCD29_CITER_ELINKYES
    /// TCD29_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD29_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD29_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD29_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD29_CSR: RWRegister<u16>,

    /// TCD29_BITER_ELINKNO and TCD29_BITER_ELINKYES
    /// TCD29_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD29_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD29_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD30_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD30_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD30_ATTR: RWRegister<u16>,

    /// TCD30_NBYTES_ML and TCD30_NBYTES_MLOFFYES
    /// TCD30_NBYTES_ML: TCD30_NBYTES_MLNO and TCD30_NBYTES_MLOFFNO
    /// TCD30_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD30_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD30_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD30_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD30_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD30_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD30_DOFF: RWRegister<u16>,

    /// TCD30_CITER_ELINKNO and TCD30_CITER_ELINKYES
    /// TCD30_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD30_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD30_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD30_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD30_CSR: RWRegister<u16>,

    /// TCD30_BITER_ELINKNO and TCD30_BITER_ELINKYES
    /// TCD30_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD30_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD30_BITER_ELINK: RWRegister<u16>,

    /// TCD Source Address
    pub TCD31_SADDR: RWRegister<u32>,

    /// TCD Signed Source Address Offset
    pub TCD31_SOFF: RWRegister<u16>,

    /// TCD Transfer Attributes
    pub TCD31_ATTR: RWRegister<u16>,

    /// TCD31_NBYTES_ML and TCD31_NBYTES_MLOFFYES
    /// TCD31_NBYTES_ML: TCD31_NBYTES_MLNO and TCD31_NBYTES_MLOFFNO
    /// TCD31_NBYTES_MLNO: TCD Minor Byte Count (Minor Loop Mapping Disabled)
    /// TCD31_NBYTES_MLOFFNO: TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)
    /// TCD31_NBYTES_MLOFFYES: TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)
    pub TCD31_NBYTES_ML: RWRegister<u32>,

    /// TCD Last Source Address Adjustment
    pub TCD31_SLAST: RWRegister<u32>,

    /// TCD Destination Address
    pub TCD31_DADDR: RWRegister<u32>,

    /// TCD Signed Destination Address Offset
    pub TCD31_DOFF: RWRegister<u16>,

    /// TCD31_CITER_ELINKNO and TCD31_CITER_ELINKYES
    /// TCD31_CITER_ELINKNO: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD31_CITER_ELINKYES: TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD31_CITER_ELINK: RWRegister<u16>,

    /// TCD Last Destination Address Adjustment/Scatter Gather Address
    pub TCD31_DLASTSGA: RWRegister<u32>,

    /// TCD Control and Status
    pub TCD31_CSR: RWRegister<u16>,

    /// TCD31_BITER_ELINKNO and TCD31_BITER_ELINKYES
    /// TCD31_BITER_ELINKNO: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)
    /// TCD31_BITER_ELINKYES: TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)
    pub TCD31_BITER_ELINK: RWRegister<u16>,
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
    pub TCD0_SADDR: u32,
    pub TCD0_SOFF: u16,
    pub TCD0_ATTR: u16,
    pub TCD0_NBYTES_ML: u32,
    pub TCD0_SLAST: u32,
    pub TCD0_DADDR: u32,
    pub TCD0_DOFF: u16,
    pub TCD0_CITER_ELINK: u16,
    pub TCD0_DLASTSGA: u32,
    pub TCD0_CSR: u16,
    pub TCD0_BITER_ELINK: u16,
    pub TCD1_SADDR: u32,
    pub TCD1_SOFF: u16,
    pub TCD1_ATTR: u16,
    pub TCD1_NBYTES_ML: u32,
    pub TCD1_SLAST: u32,
    pub TCD1_DADDR: u32,
    pub TCD1_DOFF: u16,
    pub TCD1_CITER_ELINK: u16,
    pub TCD1_DLASTSGA: u32,
    pub TCD1_CSR: u16,
    pub TCD1_BITER_ELINK: u16,
    pub TCD2_SADDR: u32,
    pub TCD2_SOFF: u16,
    pub TCD2_ATTR: u16,
    pub TCD2_NBYTES_ML: u32,
    pub TCD2_SLAST: u32,
    pub TCD2_DADDR: u32,
    pub TCD2_DOFF: u16,
    pub TCD2_CITER_ELINK: u16,
    pub TCD2_DLASTSGA: u32,
    pub TCD2_CSR: u16,
    pub TCD2_BITER_ELINK: u16,
    pub TCD3_SADDR: u32,
    pub TCD3_SOFF: u16,
    pub TCD3_ATTR: u16,
    pub TCD3_NBYTES_ML: u32,
    pub TCD3_SLAST: u32,
    pub TCD3_DADDR: u32,
    pub TCD3_DOFF: u16,
    pub TCD3_CITER_ELINK: u16,
    pub TCD3_DLASTSGA: u32,
    pub TCD3_CSR: u16,
    pub TCD3_BITER_ELINK: u16,
    pub TCD4_SADDR: u32,
    pub TCD4_SOFF: u16,
    pub TCD4_ATTR: u16,
    pub TCD4_NBYTES_ML: u32,
    pub TCD4_SLAST: u32,
    pub TCD4_DADDR: u32,
    pub TCD4_DOFF: u16,
    pub TCD4_CITER_ELINK: u16,
    pub TCD4_DLASTSGA: u32,
    pub TCD4_CSR: u16,
    pub TCD4_BITER_ELINK: u16,
    pub TCD5_SADDR: u32,
    pub TCD5_SOFF: u16,
    pub TCD5_ATTR: u16,
    pub TCD5_NBYTES_ML: u32,
    pub TCD5_SLAST: u32,
    pub TCD5_DADDR: u32,
    pub TCD5_DOFF: u16,
    pub TCD5_CITER_ELINK: u16,
    pub TCD5_DLASTSGA: u32,
    pub TCD5_CSR: u16,
    pub TCD5_BITER_ELINK: u16,
    pub TCD6_SADDR: u32,
    pub TCD6_SOFF: u16,
    pub TCD6_ATTR: u16,
    pub TCD6_NBYTES_ML: u32,
    pub TCD6_SLAST: u32,
    pub TCD6_DADDR: u32,
    pub TCD6_DOFF: u16,
    pub TCD6_CITER_ELINK: u16,
    pub TCD6_DLASTSGA: u32,
    pub TCD6_CSR: u16,
    pub TCD6_BITER_ELINK: u16,
    pub TCD7_SADDR: u32,
    pub TCD7_SOFF: u16,
    pub TCD7_ATTR: u16,
    pub TCD7_NBYTES_ML: u32,
    pub TCD7_SLAST: u32,
    pub TCD7_DADDR: u32,
    pub TCD7_DOFF: u16,
    pub TCD7_CITER_ELINK: u16,
    pub TCD7_DLASTSGA: u32,
    pub TCD7_CSR: u16,
    pub TCD7_BITER_ELINK: u16,
    pub TCD8_SADDR: u32,
    pub TCD8_SOFF: u16,
    pub TCD8_ATTR: u16,
    pub TCD8_NBYTES_ML: u32,
    pub TCD8_SLAST: u32,
    pub TCD8_DADDR: u32,
    pub TCD8_DOFF: u16,
    pub TCD8_CITER_ELINK: u16,
    pub TCD8_DLASTSGA: u32,
    pub TCD8_CSR: u16,
    pub TCD8_BITER_ELINK: u16,
    pub TCD9_SADDR: u32,
    pub TCD9_SOFF: u16,
    pub TCD9_ATTR: u16,
    pub TCD9_NBYTES_ML: u32,
    pub TCD9_SLAST: u32,
    pub TCD9_DADDR: u32,
    pub TCD9_DOFF: u16,
    pub TCD9_CITER_ELINK: u16,
    pub TCD9_DLASTSGA: u32,
    pub TCD9_CSR: u16,
    pub TCD9_BITER_ELINK: u16,
    pub TCD10_SADDR: u32,
    pub TCD10_SOFF: u16,
    pub TCD10_ATTR: u16,
    pub TCD10_NBYTES_ML: u32,
    pub TCD10_SLAST: u32,
    pub TCD10_DADDR: u32,
    pub TCD10_DOFF: u16,
    pub TCD10_CITER_ELINK: u16,
    pub TCD10_DLASTSGA: u32,
    pub TCD10_CSR: u16,
    pub TCD10_BITER_ELINK: u16,
    pub TCD11_SADDR: u32,
    pub TCD11_SOFF: u16,
    pub TCD11_ATTR: u16,
    pub TCD11_NBYTES_ML: u32,
    pub TCD11_SLAST: u32,
    pub TCD11_DADDR: u32,
    pub TCD11_DOFF: u16,
    pub TCD11_CITER_ELINK: u16,
    pub TCD11_DLASTSGA: u32,
    pub TCD11_CSR: u16,
    pub TCD11_BITER_ELINK: u16,
    pub TCD12_SADDR: u32,
    pub TCD12_SOFF: u16,
    pub TCD12_ATTR: u16,
    pub TCD12_NBYTES_ML: u32,
    pub TCD12_SLAST: u32,
    pub TCD12_DADDR: u32,
    pub TCD12_DOFF: u16,
    pub TCD12_CITER_ELINK: u16,
    pub TCD12_DLASTSGA: u32,
    pub TCD12_CSR: u16,
    pub TCD12_BITER_ELINK: u16,
    pub TCD13_SADDR: u32,
    pub TCD13_SOFF: u16,
    pub TCD13_ATTR: u16,
    pub TCD13_NBYTES_ML: u32,
    pub TCD13_SLAST: u32,
    pub TCD13_DADDR: u32,
    pub TCD13_DOFF: u16,
    pub TCD13_CITER_ELINK: u16,
    pub TCD13_DLASTSGA: u32,
    pub TCD13_CSR: u16,
    pub TCD13_BITER_ELINK: u16,
    pub TCD14_SADDR: u32,
    pub TCD14_SOFF: u16,
    pub TCD14_ATTR: u16,
    pub TCD14_NBYTES_ML: u32,
    pub TCD14_SLAST: u32,
    pub TCD14_DADDR: u32,
    pub TCD14_DOFF: u16,
    pub TCD14_CITER_ELINK: u16,
    pub TCD14_DLASTSGA: u32,
    pub TCD14_CSR: u16,
    pub TCD14_BITER_ELINK: u16,
    pub TCD15_SADDR: u32,
    pub TCD15_SOFF: u16,
    pub TCD15_ATTR: u16,
    pub TCD15_NBYTES_ML: u32,
    pub TCD15_SLAST: u32,
    pub TCD15_DADDR: u32,
    pub TCD15_DOFF: u16,
    pub TCD15_CITER_ELINK: u16,
    pub TCD15_DLASTSGA: u32,
    pub TCD15_CSR: u16,
    pub TCD15_BITER_ELINK: u16,
    pub TCD16_SADDR: u32,
    pub TCD16_SOFF: u16,
    pub TCD16_ATTR: u16,
    pub TCD16_NBYTES_ML: u32,
    pub TCD16_SLAST: u32,
    pub TCD16_DADDR: u32,
    pub TCD16_DOFF: u16,
    pub TCD16_CITER_ELINK: u16,
    pub TCD16_DLASTSGA: u32,
    pub TCD16_CSR: u16,
    pub TCD16_BITER_ELINK: u16,
    pub TCD17_SADDR: u32,
    pub TCD17_SOFF: u16,
    pub TCD17_ATTR: u16,
    pub TCD17_NBYTES_ML: u32,
    pub TCD17_SLAST: u32,
    pub TCD17_DADDR: u32,
    pub TCD17_DOFF: u16,
    pub TCD17_CITER_ELINK: u16,
    pub TCD17_DLASTSGA: u32,
    pub TCD17_CSR: u16,
    pub TCD17_BITER_ELINK: u16,
    pub TCD18_SADDR: u32,
    pub TCD18_SOFF: u16,
    pub TCD18_ATTR: u16,
    pub TCD18_NBYTES_ML: u32,
    pub TCD18_SLAST: u32,
    pub TCD18_DADDR: u32,
    pub TCD18_DOFF: u16,
    pub TCD18_CITER_ELINK: u16,
    pub TCD18_DLASTSGA: u32,
    pub TCD18_CSR: u16,
    pub TCD18_BITER_ELINK: u16,
    pub TCD19_SADDR: u32,
    pub TCD19_SOFF: u16,
    pub TCD19_ATTR: u16,
    pub TCD19_NBYTES_ML: u32,
    pub TCD19_SLAST: u32,
    pub TCD19_DADDR: u32,
    pub TCD19_DOFF: u16,
    pub TCD19_CITER_ELINK: u16,
    pub TCD19_DLASTSGA: u32,
    pub TCD19_CSR: u16,
    pub TCD19_BITER_ELINK: u16,
    pub TCD20_SADDR: u32,
    pub TCD20_SOFF: u16,
    pub TCD20_ATTR: u16,
    pub TCD20_NBYTES_ML: u32,
    pub TCD20_SLAST: u32,
    pub TCD20_DADDR: u32,
    pub TCD20_DOFF: u16,
    pub TCD20_CITER_ELINK: u16,
    pub TCD20_DLASTSGA: u32,
    pub TCD20_CSR: u16,
    pub TCD20_BITER_ELINK: u16,
    pub TCD21_SADDR: u32,
    pub TCD21_SOFF: u16,
    pub TCD21_ATTR: u16,
    pub TCD21_NBYTES_ML: u32,
    pub TCD21_SLAST: u32,
    pub TCD21_DADDR: u32,
    pub TCD21_DOFF: u16,
    pub TCD21_CITER_ELINK: u16,
    pub TCD21_DLASTSGA: u32,
    pub TCD21_CSR: u16,
    pub TCD21_BITER_ELINK: u16,
    pub TCD22_SADDR: u32,
    pub TCD22_SOFF: u16,
    pub TCD22_ATTR: u16,
    pub TCD22_NBYTES_ML: u32,
    pub TCD22_SLAST: u32,
    pub TCD22_DADDR: u32,
    pub TCD22_DOFF: u16,
    pub TCD22_CITER_ELINK: u16,
    pub TCD22_DLASTSGA: u32,
    pub TCD22_CSR: u16,
    pub TCD22_BITER_ELINK: u16,
    pub TCD23_SADDR: u32,
    pub TCD23_SOFF: u16,
    pub TCD23_ATTR: u16,
    pub TCD23_NBYTES_ML: u32,
    pub TCD23_SLAST: u32,
    pub TCD23_DADDR: u32,
    pub TCD23_DOFF: u16,
    pub TCD23_CITER_ELINK: u16,
    pub TCD23_DLASTSGA: u32,
    pub TCD23_CSR: u16,
    pub TCD23_BITER_ELINK: u16,
    pub TCD24_SADDR: u32,
    pub TCD24_SOFF: u16,
    pub TCD24_ATTR: u16,
    pub TCD24_NBYTES_ML: u32,
    pub TCD24_SLAST: u32,
    pub TCD24_DADDR: u32,
    pub TCD24_DOFF: u16,
    pub TCD24_CITER_ELINK: u16,
    pub TCD24_DLASTSGA: u32,
    pub TCD24_CSR: u16,
    pub TCD24_BITER_ELINK: u16,
    pub TCD25_SADDR: u32,
    pub TCD25_SOFF: u16,
    pub TCD25_ATTR: u16,
    pub TCD25_NBYTES_ML: u32,
    pub TCD25_SLAST: u32,
    pub TCD25_DADDR: u32,
    pub TCD25_DOFF: u16,
    pub TCD25_CITER_ELINK: u16,
    pub TCD25_DLASTSGA: u32,
    pub TCD25_CSR: u16,
    pub TCD25_BITER_ELINK: u16,
    pub TCD26_SADDR: u32,
    pub TCD26_SOFF: u16,
    pub TCD26_ATTR: u16,
    pub TCD26_NBYTES_ML: u32,
    pub TCD26_SLAST: u32,
    pub TCD26_DADDR: u32,
    pub TCD26_DOFF: u16,
    pub TCD26_CITER_ELINK: u16,
    pub TCD26_DLASTSGA: u32,
    pub TCD26_CSR: u16,
    pub TCD26_BITER_ELINK: u16,
    pub TCD27_SADDR: u32,
    pub TCD27_SOFF: u16,
    pub TCD27_ATTR: u16,
    pub TCD27_NBYTES_ML: u32,
    pub TCD27_SLAST: u32,
    pub TCD27_DADDR: u32,
    pub TCD27_DOFF: u16,
    pub TCD27_CITER_ELINK: u16,
    pub TCD27_DLASTSGA: u32,
    pub TCD27_CSR: u16,
    pub TCD27_BITER_ELINK: u16,
    pub TCD28_SADDR: u32,
    pub TCD28_SOFF: u16,
    pub TCD28_ATTR: u16,
    pub TCD28_NBYTES_ML: u32,
    pub TCD28_SLAST: u32,
    pub TCD28_DADDR: u32,
    pub TCD28_DOFF: u16,
    pub TCD28_CITER_ELINK: u16,
    pub TCD28_DLASTSGA: u32,
    pub TCD28_CSR: u16,
    pub TCD28_BITER_ELINK: u16,
    pub TCD29_SADDR: u32,
    pub TCD29_SOFF: u16,
    pub TCD29_ATTR: u16,
    pub TCD29_NBYTES_ML: u32,
    pub TCD29_SLAST: u32,
    pub TCD29_DADDR: u32,
    pub TCD29_DOFF: u16,
    pub TCD29_CITER_ELINK: u16,
    pub TCD29_DLASTSGA: u32,
    pub TCD29_CSR: u16,
    pub TCD29_BITER_ELINK: u16,
    pub TCD30_SADDR: u32,
    pub TCD30_SOFF: u16,
    pub TCD30_ATTR: u16,
    pub TCD30_NBYTES_ML: u32,
    pub TCD30_SLAST: u32,
    pub TCD30_DADDR: u32,
    pub TCD30_DOFF: u16,
    pub TCD30_CITER_ELINK: u16,
    pub TCD30_DLASTSGA: u32,
    pub TCD30_CSR: u16,
    pub TCD30_BITER_ELINK: u16,
    pub TCD31_SADDR: u32,
    pub TCD31_SOFF: u16,
    pub TCD31_ATTR: u16,
    pub TCD31_NBYTES_ML: u32,
    pub TCD31_SLAST: u32,
    pub TCD31_DADDR: u32,
    pub TCD31_DOFF: u16,
    pub TCD31_CITER_ELINK: u16,
    pub TCD31_DLASTSGA: u32,
    pub TCD31_CSR: u16,
    pub TCD31_BITER_ELINK: u16,
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
