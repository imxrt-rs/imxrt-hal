#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPSPI
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Version ID Register
pub mod VERID {

    /// Module Identification Number
    pub mod FEATURE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000100: Standard feature set supporting a 32-bit shift register.
            pub const FEATURE_4: u32 = 0b0000000000000100;
        }
    }

    /// Minor Version Number
    pub mod MINOR {
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

    /// Major Version Number
    pub mod MAJOR {
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

/// Parameter Register
pub mod PARAM {

    /// Transmit FIFO Size
    pub mod TXFIFO {
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

    /// Receive FIFO Size
    pub mod RXFIFO {
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

    /// PCS Number
    pub mod PCSNUM {
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
}

/// Control Register
pub mod CR {

    /// Module Enable
    pub mod MEN {
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

            /// 0b0: Module is disabled
            pub const MEN_0: u32 = 0b0;

            /// 0b1: Module is enabled
            pub const MEN_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod RST {
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

            /// 0b0: Module is not reset
            pub const RST_0: u32 = 0b0;

            /// 0b1: Module is reset
            pub const RST_1: u32 = 0b1;
        }
    }

    /// Doze mode enable
    pub mod DOZEN {
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

            /// 0b0: Module is enabled in Doze mode
            pub const DOZEN_0: u32 = 0b0;

            /// 0b1: Module is disabled in Doze mode
            pub const DOZEN_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGEN {
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

            /// 0b0: Module is disabled in debug mode
            pub const DBGEN_0: u32 = 0b0;

            /// 0b1: Module is enabled in debug mode
            pub const DBGEN_1: u32 = 0b1;
        }
    }

    /// Reset Transmit FIFO
    pub mod RTF {
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

            /// 0b0: No effect
            pub const RTF_0: u32 = 0b0;

            /// 0b1: Transmit FIFO is reset
            pub const RTF_1: u32 = 0b1;
        }
    }

    /// Reset Receive FIFO
    pub mod RRF {
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

            /// 0b0: No effect
            pub const RRF_0: u32 = 0b0;

            /// 0b1: Receive FIFO is reset
            pub const RRF_1: u32 = 0b1;
        }
    }
}

/// Status Register
pub mod SR {

    /// Transmit Data Flag
    pub mod TDF {
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

            /// 0b0: Transmit data not requested
            pub const TDF_0: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const TDF_1: u32 = 0b1;
        }
    }

    /// Receive Data Flag
    pub mod RDF {
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

            /// 0b0: Receive Data is not ready
            pub const RDF_0: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const RDF_1: u32 = 0b1;
        }
    }

    /// Word Complete Flag
    pub mod WCF {
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

            /// 0b0: Transfer of a received word has not yet completed
            pub const WCF_0: u32 = 0b0;

            /// 0b1: Transfer of a received word has completed
            pub const WCF_1: u32 = 0b1;
        }
    }

    /// Frame Complete Flag
    pub mod FCF {
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

            /// 0b0: Frame transfer has not completed
            pub const FCF_0: u32 = 0b0;

            /// 0b1: Frame transfer has completed
            pub const FCF_1: u32 = 0b1;
        }
    }

    /// Transfer Complete Flag
    pub mod TCF {
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

            /// 0b0: All transfers have not completed
            pub const TCF_0: u32 = 0b0;

            /// 0b1: All transfers have completed
            pub const TCF_1: u32 = 0b1;
        }
    }

    /// Transmit Error Flag
    pub mod TEF {
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

            /// 0b0: Transmit FIFO underrun has not occurred
            pub const TEF_0: u32 = 0b0;

            /// 0b1: Transmit FIFO underrun has occurred
            pub const TEF_1: u32 = 0b1;
        }
    }

    /// Receive Error Flag
    pub mod REF {
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

            /// 0b0: Receive FIFO has not overflowed
            pub const REF_0: u32 = 0b0;

            /// 0b1: Receive FIFO has overflowed
            pub const REF_1: u32 = 0b1;
        }
    }

    /// Data Match Flag
    pub mod DMF {
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

            /// 0b0: Have not received matching data
            pub const DMF_0: u32 = 0b0;

            /// 0b1: Have received matching data
            pub const DMF_1: u32 = 0b1;
        }
    }

    /// Module Busy Flag
    pub mod MBF {
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

            /// 0b0: LPSPI is idle
            pub const MBF_0: u32 = 0b0;

            /// 0b1: LPSPI is busy
            pub const MBF_1: u32 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod IER {

    /// Transmit Data Interrupt Enable
    pub mod TDIE {
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

            /// 0b0: Disabled
            pub const TDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TDIE_1: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Enable
    pub mod RDIE {
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

            /// 0b0: Disabled
            pub const RDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RDIE_1: u32 = 0b1;
        }
    }

    /// Word Complete Interrupt Enable
    pub mod WCIE {
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

            /// 0b0: Disabled
            pub const WCIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const WCIE_1: u32 = 0b1;
        }
    }

    /// Frame Complete Interrupt Enable
    pub mod FCIE {
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

            /// 0b0: Disabled
            pub const FCIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const FCIE_1: u32 = 0b1;
        }
    }

    /// Transfer Complete Interrupt Enable
    pub mod TCIE {
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

            /// 0b0: Disabled
            pub const TCIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TCIE_1: u32 = 0b1;
        }
    }

    /// Transmit Error Interrupt Enable
    pub mod TEIE {
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

            /// 0b0: Disabled
            pub const TEIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TEIE_1: u32 = 0b1;
        }
    }

    /// Receive Error Interrupt Enable
    pub mod REIE {
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

            /// 0b0: Disabled
            pub const REIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const REIE_1: u32 = 0b1;
        }
    }

    /// Data Match Interrupt Enable
    pub mod DMIE {
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

            /// 0b0: Disabled
            pub const DMIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DMIE_1: u32 = 0b1;
        }
    }
}

/// DMA Enable Register
pub mod DER {

    /// Transmit Data DMA Enable
    pub mod TDDE {
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

            /// 0b0: DMA request is disabled
            pub const TDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const TDDE_1: u32 = 0b1;
        }
    }

    /// Receive Data DMA Enable
    pub mod RDDE {
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

            /// 0b0: DMA request is disabled
            pub const RDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const RDDE_1: u32 = 0b1;
        }
    }
}

/// Configuration Register 0
pub mod CFGR0 {

    /// Host Request Enable
    pub mod HREN {
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

            /// 0b0: Host request is disabled
            pub const HREN_0: u32 = 0b0;

            /// 0b1: Host request is enabled
            pub const HREN_1: u32 = 0b1;
        }
    }

    /// Host Request Polarity
    pub mod HRPOL {
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

            /// 0b0: Active low
            pub const HRPOL_0: u32 = 0b0;

            /// 0b1: Active high
            pub const HRPOL_1: u32 = 0b1;
        }
    }

    /// Host Request Select
    pub mod HRSEL {
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

            /// 0b0: Host request input is the LPSPI_HREQ pin
            pub const HRSEL_0: u32 = 0b0;

            /// 0b1: Host request input is the input trigger
            pub const HRSEL_1: u32 = 0b1;
        }
    }

    /// Circular FIFO Enable
    pub mod CIRFIFO {
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

            /// 0b0: Circular FIFO is disabled
            pub const CIRFIFO_0: u32 = 0b0;

            /// 0b1: Circular FIFO is enabled
            pub const CIRFIFO_1: u32 = 0b1;
        }
    }

    /// Receive Data Match Only
    pub mod RDMO {
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

            /// 0b0: Received data is stored in the receive FIFO as in normal operations
            pub const RDMO_0: u32 = 0b0;

            /// 0b1: Received data is discarded unless the Data Match Flag (DMF) is set
            pub const RDMO_1: u32 = 0b1;
        }
    }
}

/// Configuration Register 1
pub mod CFGR1 {

    /// Master Mode
    pub mod MASTER {
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

            /// 0b0: Slave mode
            pub const MASTER_0: u32 = 0b0;

            /// 0b1: Master mode
            pub const MASTER_1: u32 = 0b1;
        }
    }

    /// Sample Point
    pub mod SAMPLE {
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

            /// 0b0: Input data is sampled on SCK edge
            pub const SAMPLE_0: u32 = 0b0;

            /// 0b1: Input data is sampled on delayed SCK edge
            pub const SAMPLE_1: u32 = 0b1;
        }
    }

    /// Automatic PCS
    pub mod AUTOPCS {
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

            /// 0b0: Automatic PCS generation is disabled
            pub const AUTOPCS_0: u32 = 0b0;

            /// 0b1: Automatic PCS generation is enabled
            pub const AUTOPCS_1: u32 = 0b1;
        }
    }

    /// No Stall
    pub mod NOSTALL {
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

            /// 0b0: Transfers will stall when the transmit FIFO is empty or the receive FIFO is full
            pub const NOSTALL_0: u32 = 0b0;

            /// 0b1: Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur
            pub const NOSTALL_1: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select Polarity
    pub mod PCSPOL {
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

            /// 0b0000: The Peripheral Chip Select pin PCSx is active low
            pub const PCSPOL_0: u32 = 0b0000;

            /// 0b0001: The Peripheral Chip Select pin PCSx is active high
            pub const PCSPOL_1: u32 = 0b0001;
        }
    }

    /// Match Configuration
    pub mod MATCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Match is disabled
            pub const MATCFG_0: u32 = 0b000;

            /// 0b010: 010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)
            pub const MATCFG_2: u32 = 0b010;

            /// 0b011: 011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)
            pub const MATCFG_3: u32 = 0b011;

            /// 0b100: 100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \[(1st data word = MATCH0) * (2nd data word = MATCH1)\]
            pub const MATCFG_4: u32 = 0b100;

            /// 0b101: 101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \[(any data word = MATCH0) * (next data word = MATCH1)\]
            pub const MATCFG_5: u32 = 0b101;

            /// 0b110: 110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \[(1st data word * MATCH1) = (MATCH0 * MATCH1)\]
            pub const MATCFG_6: u32 = 0b110;

            /// 0b111: 111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \[(any data word * MATCH1) = (MATCH0 * MATCH1)\]
            pub const MATCFG_7: u32 = 0b111;
        }
    }

    /// Pin Configuration
    pub mod PINCFG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SIN is used for input data and SOUT is used for output data
            pub const PINCFG_0: u32 = 0b00;

            /// 0b01: SIN is used for both input and output data
            pub const PINCFG_1: u32 = 0b01;

            /// 0b10: SOUT is used for both input and output data
            pub const PINCFG_2: u32 = 0b10;

            /// 0b11: SOUT is used for input data and SIN is used for output data
            pub const PINCFG_3: u32 = 0b11;
        }
    }

    /// Output Config
    pub mod OUTCFG {
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

            /// 0b0: Output data retains last value when chip select is negated
            pub const OUTCFG_0: u32 = 0b0;

            /// 0b1: Output data is tristated when chip select is negated
            pub const OUTCFG_1: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select Configuration
    pub mod PCSCFG {
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

            /// 0b0: PCS\[3:2\] are enabled
            pub const PCSCFG_0: u32 = 0b0;

            /// 0b1: PCS\[3:2\] are disabled
            pub const PCSCFG_1: u32 = 0b1;
        }
    }
}

/// Data Match Register 0
pub mod DMR0 {

    /// Match 0 Value
    pub mod MATCH0 {
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

/// Data Match Register 1
pub mod DMR1 {

    /// Match 1 Value
    pub mod MATCH1 {
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

/// Clock Configuration Register
pub mod CCR {

    /// SCK Divider
    pub mod SCKDIV {
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

    /// Delay Between Transfers
    pub mod DBT {
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

    /// PCS-to-SCK Delay
    pub mod PCSSCK {
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

    /// SCK-to-PCS Delay
    pub mod SCKPCS {
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

/// FIFO Control Register
pub mod FCR {

    /// Transmit FIFO Watermark
    pub mod TXWATER {
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

    /// Receive FIFO Watermark
    pub mod RXWATER {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FIFO Status Register
pub mod FSR {

    /// Transmit FIFO Count
    pub mod TXCOUNT {
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

    /// Receive FIFO Count
    pub mod RXCOUNT {
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
}

/// Transmit Command Register
pub mod TCR {

    /// Frame Size
    pub mod FRAMESZ {
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

    /// Transfer Width
    pub mod WIDTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 1 bit transfer
            pub const WIDTH_0: u32 = 0b00;

            /// 0b01: 2 bit transfer
            pub const WIDTH_1: u32 = 0b01;

            /// 0b10: 4 bit transfer
            pub const WIDTH_2: u32 = 0b10;
        }
    }

    /// Transmit Data Mask
    pub mod TXMSK {
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

            /// 0b0: Normal transfer
            pub const TXMSK_0: u32 = 0b0;

            /// 0b1: Mask transmit data
            pub const TXMSK_1: u32 = 0b1;
        }
    }

    /// Receive Data Mask
    pub mod RXMSK {
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

            /// 0b0: Normal transfer
            pub const RXMSK_0: u32 = 0b0;

            /// 0b1: Receive data is masked
            pub const RXMSK_1: u32 = 0b1;
        }
    }

    /// Continuing Command
    pub mod CONTC {
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

            /// 0b0: Command word for start of new transfer
            pub const CONTC_0: u32 = 0b0;

            /// 0b1: Command word for continuing transfer
            pub const CONTC_1: u32 = 0b1;
        }
    }

    /// Continuous Transfer
    pub mod CONT {
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

            /// 0b0: Continuous transfer is disabled
            pub const CONT_0: u32 = 0b0;

            /// 0b1: Continuous transfer is enabled
            pub const CONT_1: u32 = 0b1;
        }
    }

    /// Byte Swap
    pub mod BYSW {
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

            /// 0b0: Byte swap is disabled
            pub const BYSW_0: u32 = 0b0;

            /// 0b1: Byte swap is enabled
            pub const BYSW_1: u32 = 0b1;
        }
    }

    /// LSB First
    pub mod LSBF {
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

            /// 0b0: Data is transferred MSB first
            pub const LSBF_0: u32 = 0b0;

            /// 0b1: Data is transferred LSB first
            pub const LSBF_1: u32 = 0b1;
        }
    }

    /// Peripheral Chip Select
    pub mod PCS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Transfer using LPSPI_PCS\[0\]
            pub const PCS_0: u32 = 0b00;

            /// 0b01: Transfer using LPSPI_PCS\[1\]
            pub const PCS_1: u32 = 0b01;

            /// 0b10: Transfer using LPSPI_PCS\[2\]
            pub const PCS_2: u32 = 0b10;

            /// 0b11: Transfer using LPSPI_PCS\[3\]
            pub const PCS_3: u32 = 0b11;
        }
    }

    /// Prescaler Value
    pub mod PRESCALE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divide by 1
            pub const PRESCALE_0: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const PRESCALE_1: u32 = 0b001;

            /// 0b010: Divide by 4
            pub const PRESCALE_2: u32 = 0b010;

            /// 0b011: Divide by 8
            pub const PRESCALE_3: u32 = 0b011;

            /// 0b100: Divide by 16
            pub const PRESCALE_4: u32 = 0b100;

            /// 0b101: Divide by 32
            pub const PRESCALE_5: u32 = 0b101;

            /// 0b110: Divide by 64
            pub const PRESCALE_6: u32 = 0b110;

            /// 0b111: Divide by 128
            pub const PRESCALE_7: u32 = 0b111;
        }
    }

    /// Clock Phase
    pub mod CPHA {
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

            /// 0b0: Data is captured on the leading edge of SCK and changed on the following edge of SCK
            pub const CPHA_0: u32 = 0b0;

            /// 0b1: Data is changed on the leading edge of SCK and captured on the following edge of SCK
            pub const CPHA_1: u32 = 0b1;
        }
    }

    /// Clock Polarity
    pub mod CPOL {
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

            /// 0b0: The inactive state value of SCK is low
            pub const CPOL_0: u32 = 0b0;

            /// 0b1: The inactive state value of SCK is high
            pub const CPOL_1: u32 = 0b1;
        }
    }
}

/// Transmit Data Register
pub mod TDR {

    /// Transmit Data
    pub mod DATA {
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

/// Receive Status Register
pub mod RSR {

    /// Start Of Frame
    pub mod SOF {
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

            /// 0b0: Subsequent data word received after LPSPI_PCS assertion
            pub const SOF_0: u32 = 0b0;

            /// 0b1: First data word received after LPSPI_PCS assertion
            pub const SOF_1: u32 = 0b1;
        }
    }

    /// RX FIFO Empty
    pub mod RXEMPTY {
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

            /// 0b0: RX FIFO is not empty
            pub const RXEMPTY_0: u32 = 0b0;

            /// 0b1: RX FIFO is empty
            pub const RXEMPTY_1: u32 = 0b1;
        }
    }
}

/// Receive Data Register
pub mod RDR {

    /// Receive Data
    pub mod DATA {
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
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    _reserved1: [u32; 2],

    /// Control Register
    pub CR: RWRegister<u32>,

    /// Status Register
    pub SR: RWRegister<u32>,

    /// Interrupt Enable Register
    pub IER: RWRegister<u32>,

    /// DMA Enable Register
    pub DER: RWRegister<u32>,

    /// Configuration Register 0
    pub CFGR0: RWRegister<u32>,

    /// Configuration Register 1
    pub CFGR1: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// Data Match Register 0
    pub DMR0: RWRegister<u32>,

    /// Data Match Register 1
    pub DMR1: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// Clock Configuration Register
    pub CCR: RWRegister<u32>,

    _reserved4: [u32; 5],

    /// FIFO Control Register
    pub FCR: RWRegister<u32>,

    /// FIFO Status Register
    pub FSR: RORegister<u32>,

    /// Transmit Command Register
    pub TCR: RWRegister<u32>,

    /// Transmit Data Register
    pub TDR: WORegister<u32>,

    _reserved5: [u32; 2],

    /// Receive Status Register
    pub RSR: RORegister<u32>,

    /// Receive Data Register
    pub RDR: RORegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub CR: u32,
    pub SR: u32,
    pub IER: u32,
    pub DER: u32,
    pub CFGR0: u32,
    pub CFGR1: u32,
    pub DMR0: u32,
    pub DMR1: u32,
    pub CCR: u32,
    pub FCR: u32,
    pub FSR: u32,
    pub TCR: u32,
    pub TDR: u32,
    pub RSR: u32,
    pub RDR: u32,
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
