#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEMC
//!
//! Used by: imxrt1051, imxrt1052

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Module Control Register
pub mod MCR {

    /// Software Reset
    pub mod SWRST {
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

    /// Module Disable
    pub mod MDIS {
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

            /// 0b0: Module enabled
            pub const MDIS_0: u32 = 0b0;

            /// 0b1: Module disabled.
            pub const MDIS_1: u32 = 0b1;
        }
    }

    /// DQS (read strobe) mode
    pub mod DQSMD {
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

            /// 0b0: Dummy read strobe loopbacked internally
            pub const DQSMD_0: u32 = 0b0;

            /// 0b1: Dummy read strobe loopbacked from DQS pad
            pub const DQSMD_1: u32 = 0b1;
        }
    }

    /// WAIT/RDY# polarity for NOR/PSRAM
    pub mod WPOL0 {
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

            /// 0b0: Low active
            pub const WPOL0_0: u32 = 0b0;

            /// 0b1: High active
            pub const WPOL0_1: u32 = 0b1;
        }
    }

    /// WAIT/RDY# polarity for NAND
    pub mod WPOL1 {
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

            /// 0b0: Low active
            pub const WPOL1_0: u32 = 0b0;

            /// 0b1: High active
            pub const WPOL1_1: u32 = 0b1;
        }
    }

    /// Command Execution timeout cycles
    pub mod CTO {
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

    /// Bus timeout cycles
    pub mod BTO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 255*1
            pub const BTO_0: u32 = 0b00000;

            /// 0b00001: 255*2 - 255*2^30
            pub const BTO_1: u32 = 0b00001;

            /// 0b00010: 255*2 - 255*2^30
            pub const BTO_2: u32 = 0b00010;

            /// 0b00011: 255*2 - 255*2^30
            pub const BTO_3: u32 = 0b00011;

            /// 0b00100: 255*2 - 255*2^30
            pub const BTO_4: u32 = 0b00100;

            /// 0b00101: 255*2 - 255*2^30
            pub const BTO_5: u32 = 0b00101;

            /// 0b00110: 255*2 - 255*2^30
            pub const BTO_6: u32 = 0b00110;

            /// 0b00111: 255*2 - 255*2^30
            pub const BTO_7: u32 = 0b00111;

            /// 0b01000: 255*2 - 255*2^30
            pub const BTO_8: u32 = 0b01000;

            /// 0b01001: 255*2 - 255*2^30
            pub const BTO_9: u32 = 0b01001;

            /// 0b11111: 255*2^31
            pub const BTO_31: u32 = 0b11111;
        }
    }
}

/// IO Mux Control Register
pub mod IOCR {

    /// SEMC_A8 output selection
    pub mod MUX_A8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: SDRAM Address bit (A8)
            pub const MUX_A8_0: u32 = 0b000;

            /// 0b001: NAND CE#
            pub const MUX_A8_1: u32 = 0b001;

            /// 0b010: NOR CE#
            pub const MUX_A8_2: u32 = 0b010;

            /// 0b011: PSRAM CE#
            pub const MUX_A8_3: u32 = 0b011;

            /// 0b100: DBI CSX
            pub const MUX_A8_4: u32 = 0b100;

            /// 0b101: SDRAM Address bit (A8)
            pub const MUX_A8_5: u32 = 0b101;

            /// 0b110: SDRAM Address bit (A8)
            pub const MUX_A8_6: u32 = 0b110;

            /// 0b111: SDRAM Address bit (A8)
            pub const MUX_A8_7: u32 = 0b111;
        }
    }

    /// SEMC_CSX0 output selection
    pub mod MUX_CSX0 {
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

            /// 0b000: NOR/PSRAM Address bit 24 (A24)
            pub const MUX_CSX0_0: u32 = 0b000;

            /// 0b001: SDRAM CS1
            pub const MUX_CSX0_1: u32 = 0b001;

            /// 0b010: SDRAM CS2
            pub const MUX_CSX0_2: u32 = 0b010;

            /// 0b011: SDRAM CS3
            pub const MUX_CSX0_3: u32 = 0b011;

            /// 0b100: NAND CE#
            pub const MUX_CSX0_4: u32 = 0b100;

            /// 0b101: NOR CE#
            pub const MUX_CSX0_5: u32 = 0b101;

            /// 0b110: PSRAM CE#
            pub const MUX_CSX0_6: u32 = 0b110;

            /// 0b111: DBI CSX
            pub const MUX_CSX0_7: u32 = 0b111;
        }
    }

    /// SEMC_CSX1 output selection
    pub mod MUX_CSX1 {
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

            /// 0b000: NOR/PSRAM Address bit 25 (A25)
            pub const MUX_CSX1_0: u32 = 0b000;

            /// 0b001: SDRAM CS1
            pub const MUX_CSX1_1: u32 = 0b001;

            /// 0b010: SDRAM CS2
            pub const MUX_CSX1_2: u32 = 0b010;

            /// 0b011: SDRAM CS3
            pub const MUX_CSX1_3: u32 = 0b011;

            /// 0b100: NAND CE#
            pub const MUX_CSX1_4: u32 = 0b100;

            /// 0b101: NOR CE#
            pub const MUX_CSX1_5: u32 = 0b101;

            /// 0b110: PSRAM CE#
            pub const MUX_CSX1_6: u32 = 0b110;

            /// 0b111: DBI CSX
            pub const MUX_CSX1_7: u32 = 0b111;
        }
    }

    /// SEMC_CSX2 output selection
    pub mod MUX_CSX2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: NOR/PSRAM Address bit 26 (A26)
            pub const MUX_CSX2_0: u32 = 0b000;

            /// 0b001: SDRAM CS1
            pub const MUX_CSX2_1: u32 = 0b001;

            /// 0b010: SDRAM CS2
            pub const MUX_CSX2_2: u32 = 0b010;

            /// 0b011: SDRAM CS3
            pub const MUX_CSX2_3: u32 = 0b011;

            /// 0b100: NAND CE#
            pub const MUX_CSX2_4: u32 = 0b100;

            /// 0b101: NOR CE#
            pub const MUX_CSX2_5: u32 = 0b101;

            /// 0b110: PSRAM CE#
            pub const MUX_CSX2_6: u32 = 0b110;

            /// 0b111: DBI CSX
            pub const MUX_CSX2_7: u32 = 0b111;
        }
    }

    /// SEMC_CSX3 output selection
    pub mod MUX_CSX3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: NOR/PSRAM Address bit 27 (A27)
            pub const MUX_CSX3_0: u32 = 0b000;

            /// 0b001: SDRAM CS1
            pub const MUX_CSX3_1: u32 = 0b001;

            /// 0b010: SDRAM CS2
            pub const MUX_CSX3_2: u32 = 0b010;

            /// 0b011: SDRAM CS3
            pub const MUX_CSX3_3: u32 = 0b011;

            /// 0b100: NAND CE#
            pub const MUX_CSX3_4: u32 = 0b100;

            /// 0b101: NOR CE#
            pub const MUX_CSX3_5: u32 = 0b101;

            /// 0b110: PSRAM CE#
            pub const MUX_CSX3_6: u32 = 0b110;

            /// 0b111: DBI CSX
            pub const MUX_CSX3_7: u32 = 0b111;
        }
    }

    /// SEMC_RDY function selection
    pub mod MUX_RDY {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: NAND Ready/Wait# input
            pub const MUX_RDY_0: u32 = 0b000;

            /// 0b001: SDRAM CS1
            pub const MUX_RDY_1: u32 = 0b001;

            /// 0b010: SDRAM CS2
            pub const MUX_RDY_2: u32 = 0b010;

            /// 0b011: SDRAM CS3
            pub const MUX_RDY_3: u32 = 0b011;

            /// 0b100: NOR CE#
            pub const MUX_RDY_4: u32 = 0b100;

            /// 0b101: PSRAM CE#
            pub const MUX_RDY_5: u32 = 0b101;

            /// 0b110: DBI CSX
            pub const MUX_RDY_6: u32 = 0b110;

            /// 0b111: NOR/PSRAM Address bit 27
            pub const MUX_RDY_7: u32 = 0b111;
        }
    }
}

/// Master Bus (AXI) Control Register 0
pub mod BMCR0 {

    /// Weight of QoS
    pub mod WQOS {
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

    /// Weight of Aging
    pub mod WAGE {
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

    /// Weight of Slave Hit (no read/write switch)
    pub mod WSH {
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

    /// Weight of Slave Hit (Read/Write switch)
    pub mod WRWS {
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

/// Master Bus (AXI) Control Register 1
pub mod BMCR1 {

    /// Weight of QoS
    pub mod WQOS {
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

    /// Weight of Aging
    pub mod WAGE {
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

    /// Weight of Page Hit
    pub mod WPH {
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

    /// Weight of Read/Write switch
    pub mod WRWS {
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

    /// Weight of Bank Rotation
    pub mod WBR {
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

/// Base Register 0 (For SDRAM CS0 device)
pub mod BR0 {

    /// Valid
    pub mod VLD {
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

    /// Memory size
    pub mod MS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (5 bits: 0b11111 << 1)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: 4KB
            pub const MS_0: u32 = 0b00000;

            /// 0b00001: 8KB
            pub const MS_1: u32 = 0b00001;

            /// 0b00010: 16KB
            pub const MS_2: u32 = 0b00010;

            /// 0b00011: 32KB
            pub const MS_3: u32 = 0b00011;

            /// 0b00100: 64KB
            pub const MS_4: u32 = 0b00100;

            /// 0b00101: 128KB
            pub const MS_5: u32 = 0b00101;

            /// 0b00110: 256KB
            pub const MS_6: u32 = 0b00110;

            /// 0b00111: 512KB
            pub const MS_7: u32 = 0b00111;

            /// 0b01000: 1MB
            pub const MS_8: u32 = 0b01000;

            /// 0b01001: 2MB
            pub const MS_9: u32 = 0b01001;

            /// 0b01010: 4MB
            pub const MS_10: u32 = 0b01010;

            /// 0b01011: 8MB
            pub const MS_11: u32 = 0b01011;

            /// 0b01100: 16MB
            pub const MS_12: u32 = 0b01100;

            /// 0b01101: 32MB
            pub const MS_13: u32 = 0b01101;

            /// 0b01110: 64MB
            pub const MS_14: u32 = 0b01110;

            /// 0b01111: 128MB
            pub const MS_15: u32 = 0b01111;

            /// 0b10000: 256MB
            pub const MS_16: u32 = 0b10000;

            /// 0b10001: 512MB
            pub const MS_17: u32 = 0b10001;

            /// 0b10010: 1GB
            pub const MS_18: u32 = 0b10010;

            /// 0b10011: 2GB
            pub const MS_19: u32 = 0b10011;

            /// 0b10100: 4GB
            pub const MS_20: u32 = 0b10100;

            /// 0b10101: 4GB
            pub const MS_21: u32 = 0b10101;

            /// 0b10110: 4GB
            pub const MS_22: u32 = 0b10110;

            /// 0b10111: 4GB
            pub const MS_23: u32 = 0b10111;

            /// 0b11000: 4GB
            pub const MS_24: u32 = 0b11000;

            /// 0b11001: 4GB
            pub const MS_25: u32 = 0b11001;

            /// 0b11010: 4GB
            pub const MS_26: u32 = 0b11010;

            /// 0b11011: 4GB
            pub const MS_27: u32 = 0b11011;

            /// 0b11100: 4GB
            pub const MS_28: u32 = 0b11100;

            /// 0b11101: 4GB
            pub const MS_29: u32 = 0b11101;

            /// 0b11110: 4GB
            pub const MS_30: u32 = 0b11110;

            /// 0b11111: 4GB
            pub const MS_31: u32 = 0b11111;
        }
    }

    /// Base Address
    pub mod BA {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Base Register 1 (For SDRAM CS1 device)
pub mod BR1 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 2 (For SDRAM CS2 device)
pub mod BR2 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 3 (For SDRAM CS3 device)
pub mod BR3 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 4 (For NAND device)
pub mod BR4 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 5 (For NOR device)
pub mod BR5 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 6 (For PSRAM device)
pub mod BR6 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)
pub mod BR7 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Base Register 8 (For NAND device)
pub mod BR8 {
    pub use super::BR0::BA;
    pub use super::BR0::MS;
    pub use super::BR0::VLD;
}

/// Interrupt Enable Register
pub mod INTEN {

    /// IP command done interrupt enable
    pub mod IPCMDDONEEN {
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

    /// IP command error interrupt enable
    pub mod IPCMDERREN {
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

    /// AXI command error interrupt enable
    pub mod AXICMDERREN {
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

    /// AXI bus error interrupt enable
    pub mod AXIBUSERREN {
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

    /// This bit enable/disable the NDPAGEEND interrupt generation.
    pub mod NDPAGEENDEN {
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

            /// 0b0: Disable
            pub const NDPAGEENDEN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const NDPAGEENDEN_1: u32 = 0b1;
        }
    }

    /// This bit enable/disable the NDNOPEND interrupt generation.
    pub mod NDNOPENDEN {
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

            /// 0b0: Disable
            pub const NDNOPENDEN_0: u32 = 0b0;

            /// 0b1: Enable
            pub const NDNOPENDEN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod INTR {

    /// IP command normal done interrupt
    pub mod IPCMDDONE {
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

    /// IP command error done interrupt
    pub mod IPCMDERR {
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

    /// AXI command error interrupt
    pub mod AXICMDERR {
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

    /// AXI bus error interrupt
    pub mod AXIBUSERR {
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

    /// This interrupt is generated when the last address of one page in NAND device is written by AXI command
    pub mod NDPAGEEND {
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

    /// This interrupt is generated when all pending AXI write command to NAND is finished on NAND interface.
    pub mod NDNOPEND {
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
}

/// SDRAM control register 0
pub mod SDRAMCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 8
            pub const BL_4: u32 = 0b100;

            /// 0b101: 8
            pub const BL_5: u32 = 0b101;

            /// 0b110: 8
            pub const BL_6: u32 = 0b110;

            /// 0b111: 8
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Column address bit number
    pub mod COL {
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

            /// 0b00: 12 bit
            pub const COL_0: u32 = 0b00;

            /// 0b01: 11 bit
            pub const COL_1: u32 = 0b01;

            /// 0b10: 10 bit
            pub const COL_2: u32 = 0b10;

            /// 0b11: 9 bit
            pub const COL_3: u32 = 0b11;
        }
    }

    /// CAS Latency
    pub mod CL {
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

            /// 0b00: 1
            pub const CL_0: u32 = 0b00;

            /// 0b01: 1
            pub const CL_1: u32 = 0b01;

            /// 0b10: 2
            pub const CL_2: u32 = 0b10;

            /// 0b11: 3
            pub const CL_3: u32 = 0b11;
        }
    }
}

/// SDRAM control register 1
pub mod SDRAMCR1 {

    /// PRECHARGE to ACT/Refresh wait time
    pub mod PRE2ACT {
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

    /// ACT to Read/Write wait time
    pub mod ACT2RW {
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

    /// Refresh recovery time
    pub mod RFRC {
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

    /// Write recovery time
    pub mod WRC {
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

    /// CKE OFF minimum time
    pub mod CKEOFF {
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

    /// ACT to Precharge minimum time
    pub mod ACT2PRE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SDRAM control register 2
pub mod SDRAMCR2 {

    /// Self Refresh Recovery time
    pub mod SRRC {
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

    /// Refresh to Refresh wait time
    pub mod REF2REF {
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

    /// ACT to ACT wait time
    pub mod ACT2ACT {
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

    /// SDRAM Idle timeout
    pub mod ITO {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: IDLE timeout period is 256*Prescale period.
            pub const ITO_0: u32 = 0b00000000;

            /// 0b00000001: IDLE timeout period is ITO*Prescale period.
            pub const ITO_1: u32 = 0b00000001;

            /// 0b00000010: IDLE timeout period is ITO*Prescale period.
            pub const ITO_2: u32 = 0b00000010;

            /// 0b00000011: IDLE timeout period is ITO*Prescale period.
            pub const ITO_3: u32 = 0b00000011;

            /// 0b00000100: IDLE timeout period is ITO*Prescale period.
            pub const ITO_4: u32 = 0b00000100;

            /// 0b00000101: IDLE timeout period is ITO*Prescale period.
            pub const ITO_5: u32 = 0b00000101;

            /// 0b00000110: IDLE timeout period is ITO*Prescale period.
            pub const ITO_6: u32 = 0b00000110;

            /// 0b00000111: IDLE timeout period is ITO*Prescale period.
            pub const ITO_7: u32 = 0b00000111;

            /// 0b00001000: IDLE timeout period is ITO*Prescale period.
            pub const ITO_8: u32 = 0b00001000;

            /// 0b00001001: IDLE timeout period is ITO*Prescale period.
            pub const ITO_9: u32 = 0b00001001;
        }
    }
}

/// SDRAM control register 3
pub mod SDRAMCR3 {

    /// Refresh enable
    pub mod REN {
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

    /// Refresh burst length
    pub mod REBL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1
            pub const REBL_0: u32 = 0b000;

            /// 0b001: 2
            pub const REBL_1: u32 = 0b001;

            /// 0b010: 3
            pub const REBL_2: u32 = 0b010;

            /// 0b011: 4
            pub const REBL_3: u32 = 0b011;

            /// 0b100: 5
            pub const REBL_4: u32 = 0b100;

            /// 0b101: 6
            pub const REBL_5: u32 = 0b101;

            /// 0b110: 7
            pub const REBL_6: u32 = 0b110;

            /// 0b111: 8
            pub const REBL_7: u32 = 0b111;
        }
    }

    /// Prescaler timer period
    pub mod PRESCALE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: 256*16 cycle
            pub const PRESCALE_0: u32 = 0b00000000;

            /// 0b00000001: PRESCALE*16 cycle
            pub const PRESCALE_1: u32 = 0b00000001;

            /// 0b00000010: PRESCALE*16 cycle
            pub const PRESCALE_2: u32 = 0b00000010;

            /// 0b00000011: PRESCALE*16 cycle
            pub const PRESCALE_3: u32 = 0b00000011;

            /// 0b00000100: PRESCALE*16 cycle
            pub const PRESCALE_4: u32 = 0b00000100;

            /// 0b00000101: PRESCALE*16 cycle
            pub const PRESCALE_5: u32 = 0b00000101;

            /// 0b00000110: PRESCALE*16 cycle
            pub const PRESCALE_6: u32 = 0b00000110;

            /// 0b00000111: PRESCALE*16 cycle
            pub const PRESCALE_7: u32 = 0b00000111;

            /// 0b00001000: PRESCALE*16 cycle
            pub const PRESCALE_8: u32 = 0b00001000;

            /// 0b00001001: PRESCALE*16 cycle
            pub const PRESCALE_9: u32 = 0b00001001;
        }
    }

    /// Refresh timer period
    pub mod RT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: 256*Prescaler period
            pub const RT_0: u32 = 0b00000000;

            /// 0b00000001: RT*Prescaler period
            pub const RT_1: u32 = 0b00000001;

            /// 0b00000010: RT*Prescaler period
            pub const RT_2: u32 = 0b00000010;

            /// 0b00000011: RT*Prescaler period
            pub const RT_3: u32 = 0b00000011;

            /// 0b00000100: RT*Prescaler period
            pub const RT_4: u32 = 0b00000100;

            /// 0b00000101: RT*Prescaler period
            pub const RT_5: u32 = 0b00000101;

            /// 0b00000110: RT*Prescaler period
            pub const RT_6: u32 = 0b00000110;

            /// 0b00000111: RT*Prescaler period
            pub const RT_7: u32 = 0b00000111;

            /// 0b00001000: RT*Prescaler period
            pub const RT_8: u32 = 0b00001000;

            /// 0b00001001: RT*Prescaler period
            pub const RT_9: u32 = 0b00001001;
        }
    }

    /// Refresh urgent threshold
    pub mod UT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: 256*Prescaler period
            pub const UT_0: u32 = 0b00000000;

            /// 0b00000001: UT*Prescaler period
            pub const UT_1: u32 = 0b00000001;

            /// 0b00000010: UT*Prescaler period
            pub const UT_2: u32 = 0b00000010;

            /// 0b00000011: UT*Prescaler period
            pub const UT_3: u32 = 0b00000011;

            /// 0b00000100: UT*Prescaler period
            pub const UT_4: u32 = 0b00000100;

            /// 0b00000101: UT*Prescaler period
            pub const UT_5: u32 = 0b00000101;

            /// 0b00000110: UT*Prescaler period
            pub const UT_6: u32 = 0b00000110;

            /// 0b00000111: UT*Prescaler period
            pub const UT_7: u32 = 0b00000111;

            /// 0b00001000: UT*Prescaler period
            pub const UT_8: u32 = 0b00001000;

            /// 0b00001001: UT*Prescaler period
            pub const UT_9: u32 = 0b00001001;
        }
    }
}

/// NAND control register 0
pub mod NANDCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// EDO mode enabled
    pub mod EDO {
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

            /// 0b0: EDO mode disabled
            pub const EDO_0: u32 = 0b0;

            /// 0b1: EDO mode enabled
            pub const EDO_1: u32 = 0b1;
        }
    }

    /// Column address bit number
    pub mod COL {
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

            /// 0b000: 16
            pub const COL_0: u32 = 0b000;

            /// 0b001: 15
            pub const COL_1: u32 = 0b001;

            /// 0b010: 14
            pub const COL_2: u32 = 0b010;

            /// 0b011: 13
            pub const COL_3: u32 = 0b011;

            /// 0b100: 12
            pub const COL_4: u32 = 0b100;

            /// 0b101: 11
            pub const COL_5: u32 = 0b101;

            /// 0b110: 10
            pub const COL_6: u32 = 0b110;

            /// 0b111: 9
            pub const COL_7: u32 = 0b111;
        }
    }
}

/// NAND control register 1
pub mod NANDCR1 {

    /// CE setup time
    pub mod CES {
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

    /// CE hold time
    pub mod CEH {
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

    /// WE# LOW time
    pub mod WEL {
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

    /// WE# HIGH time
    pub mod WEH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RE# LOW time
    pub mod REL {
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

    /// RE# HIGH time
    pub mod REH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Turnaround time
    pub mod TA {
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

    /// CE# interval time
    pub mod CEITV {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NAND control register 2
pub mod NANDCR2 {

    /// WE# HIGH to RE# LOW wait time
    pub mod TWHR {
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

    /// RE# HIGH to WE# LOW wait time
    pub mod TRHW {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (6 bits: 0x3f << 6)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ALE to WRITE Data start wait time
    pub mod TADL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (6 bits: 0x3f << 12)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Ready to RE# LOW min wait time
    pub mod TRR {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (6 bits: 0x3f << 18)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WE# HIGH to busy wait time
    pub mod TWB {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NAND control register 3
pub mod NANDCR3 {

    /// NAND option bit 1
    pub mod NDOPT1 {
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

    /// NAND option bit 2
    pub mod NDOPT2 {
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

    /// NAND option bit 3
    pub mod NDOPT3 {
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
}

/// NOR control register 0
pub mod NORCR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Address Mode
    pub mod AM {
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

            /// 0b00: Address/Data MUX mode
            pub const AM_0: u32 = 0b00;

            /// 0b01: Advanced Address/Data MUX mode
            pub const AM_1: u32 = 0b01;

            /// 0b10: Address/Data non-MUX mode
            pub const AM_2: u32 = 0b10;

            /// 0b11: Address/Data non-MUX mode
            pub const AM_3: u32 = 0b11;
        }
    }

    /// ADV# polarity
    pub mod ADVP {
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

            /// 0b0: ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW.
            pub const ADVP_0: u32 = 0b0;

            /// 0b1: ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH.
            pub const ADVP_1: u32 = 0b1;
        }
    }

    /// Column Address bit width
    pub mod COL {
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

            /// 0b0000: 12 Bits
            pub const COL_0: u32 = 0b0000;

            /// 0b0001: 11 Bits
            pub const COL_1: u32 = 0b0001;

            /// 0b0010: 10 Bits
            pub const COL_2: u32 = 0b0010;

            /// 0b0011: 9 Bits
            pub const COL_3: u32 = 0b0011;

            /// 0b0100: 8 Bits
            pub const COL_4: u32 = 0b0100;

            /// 0b0101: 7 Bits
            pub const COL_5: u32 = 0b0101;

            /// 0b0110: 6 Bits
            pub const COL_6: u32 = 0b0110;

            /// 0b0111: 5 Bits
            pub const COL_7: u32 = 0b0111;

            /// 0b1000: 4 Bits
            pub const COL_8: u32 = 0b1000;

            /// 0b1001: 3 Bits
            pub const COL_9: u32 = 0b1001;

            /// 0b1010: 2 Bits
            pub const COL_10: u32 = 0b1010;

            /// 0b1011: 12 Bits
            pub const COL_11: u32 = 0b1011;

            /// 0b1100: 12 Bits
            pub const COL_12: u32 = 0b1100;

            /// 0b1101: 12 Bits
            pub const COL_13: u32 = 0b1101;

            /// 0b1110: 12 Bits
            pub const COL_14: u32 = 0b1110;

            /// 0b1111: 12 Bits
            pub const COL_15: u32 = 0b1111;
        }
    }
}

/// NOR control register 1
pub mod NORCR1 {

    /// CE setup time cycle
    pub mod CES {
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

    /// CE hold min time (CEH+1) cycle
    pub mod CEH {
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

    /// Address setup time
    pub mod AS {
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

    /// Address hold time
    pub mod AH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WE LOW time (WEL+1) cycle
    pub mod WEL {
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

    /// WE HIGH time (WEH+1) cycle
    pub mod WEH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RE LOW time (REL+1) cycle
    pub mod REL {
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

    /// RE HIGH time (REH+1) cycle
    pub mod REH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// NOR control register 2
pub mod NORCR2 {

    /// Write Data setup time (WDS+1) cycle
    pub mod WDS {
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

    /// Write Data hold time (WDH+1) cycle
    pub mod WDH {
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

    /// Turnaround time cycle
    pub mod TA {
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

    /// Address to write data hold time cycle
    pub mod AWDH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Latency count
    pub mod LC {
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

    /// Read cycle time
    pub mod RD {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CE# interval min time
    pub mod CEITV {
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
}

/// NOR control register 3
pub mod NORCR3 {}

/// SRAM control register 0
pub mod SRAMCR0 {
    pub use super::NORCR0::ADVP;
    pub use super::NORCR0::AM;
    pub use super::NORCR0::BL;
    pub use super::NORCR0::COL;
    pub use super::NORCR0::PS;
}

/// SRAM control register 1
pub mod SRAMCR1 {
    pub use super::NORCR1::AH;
    pub use super::NORCR1::AS;
    pub use super::NORCR1::CEH;
    pub use super::NORCR1::CES;
    pub use super::NORCR1::REH;
    pub use super::NORCR1::REL;
    pub use super::NORCR1::WEH;
    pub use super::NORCR1::WEL;
}

/// SRAM control register 2
pub mod SRAMCR2 {
    pub use super::NORCR2::AWDH;
    pub use super::NORCR2::CEITV;
    pub use super::NORCR2::LC;
    pub use super::NORCR2::RD;
    pub use super::NORCR2::TA;
    pub use super::NORCR2::WDH;
    pub use super::NORCR2::WDS;
}

/// SRAM control register 3
pub mod SRAMCR3 {}

/// DBI-B control register 0
pub mod DBICR0 {

    /// Port Size
    pub mod PS {
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

            /// 0b0: 8bit
            pub const PS_0: u32 = 0b0;

            /// 0b1: 16bit
            pub const PS_1: u32 = 0b1;
        }
    }

    /// Burst Length
    pub mod BL {
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

            /// 0b000: 1
            pub const BL_0: u32 = 0b000;

            /// 0b001: 2
            pub const BL_1: u32 = 0b001;

            /// 0b010: 4
            pub const BL_2: u32 = 0b010;

            /// 0b011: 8
            pub const BL_3: u32 = 0b011;

            /// 0b100: 16
            pub const BL_4: u32 = 0b100;

            /// 0b101: 32
            pub const BL_5: u32 = 0b101;

            /// 0b110: 64
            pub const BL_6: u32 = 0b110;

            /// 0b111: 64
            pub const BL_7: u32 = 0b111;
        }
    }

    /// Column Address bit width
    pub mod COL {
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

            /// 0b0000: 12 Bits
            pub const COL_0: u32 = 0b0000;

            /// 0b0001: 11 Bits
            pub const COL_1: u32 = 0b0001;

            /// 0b0010: 10 Bits
            pub const COL_2: u32 = 0b0010;

            /// 0b0011: 9 Bits
            pub const COL_3: u32 = 0b0011;

            /// 0b0100: 8 Bits
            pub const COL_4: u32 = 0b0100;

            /// 0b0101: 7 Bits
            pub const COL_5: u32 = 0b0101;

            /// 0b0110: 6 Bits
            pub const COL_6: u32 = 0b0110;

            /// 0b0111: 5 Bits
            pub const COL_7: u32 = 0b0111;

            /// 0b1000: 4 Bits
            pub const COL_8: u32 = 0b1000;

            /// 0b1001: 3 Bits
            pub const COL_9: u32 = 0b1001;

            /// 0b1010: 2 Bits
            pub const COL_10: u32 = 0b1010;

            /// 0b1011: 12 Bits
            pub const COL_11: u32 = 0b1011;

            /// 0b1100: 12 Bits
            pub const COL_12: u32 = 0b1100;

            /// 0b1101: 12 Bits
            pub const COL_13: u32 = 0b1101;

            /// 0b1110: 12 Bits
            pub const COL_14: u32 = 0b1110;

            /// 0b1111: 12 Bits
            pub const COL_15: u32 = 0b1111;
        }
    }
}

/// DBI-B control register 1
pub mod DBICR1 {

    /// CSX Setup Time
    pub mod CES {
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

    /// CSX Hold Time
    pub mod CEH {
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

    /// WRX Low Time
    pub mod WEL {
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

    /// WRX High Time
    pub mod WEH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RDX Low Time bit \[3:0\]
    pub mod REL {
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

    /// RDX High Time bit \[3:0\]
    pub mod REH {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CSX interval min time
    pub mod CEITV {
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

    /// RDX Low Time bit \[5:4\]
    pub mod REL2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RDX High Time bit \[5:4\]
    pub mod REH2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IP Command control register 0
pub mod IPCR0 {

    /// Slave address
    pub mod SA {
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

/// IP Command control register 1
pub mod IPCR1 {

    /// Data Size in Byte
    pub mod DATSZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 4
            pub const DATSZ_0: u32 = 0b000;

            /// 0b001: 1
            pub const DATSZ_1: u32 = 0b001;

            /// 0b010: 2
            pub const DATSZ_2: u32 = 0b010;

            /// 0b011: 3
            pub const DATSZ_3: u32 = 0b011;

            /// 0b100: 4
            pub const DATSZ_4: u32 = 0b100;

            /// 0b101: 4
            pub const DATSZ_5: u32 = 0b101;

            /// 0b110: 4
            pub const DATSZ_6: u32 = 0b110;

            /// 0b111: 4
            pub const DATSZ_7: u32 = 0b111;
        }
    }
}

/// IP Command control register 2
pub mod IPCR2 {

    /// Byte Mask for Byte 0 (IPTXD bit 7:0)
    pub mod BM0 {
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

            /// 0b0: Byte Unmasked
            pub const BM0_0: u32 = 0b0;

            /// 0b1: Byte Masked
            pub const BM0_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 1 (IPTXD bit 15:8)
    pub mod BM1 {
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

            /// 0b0: Byte Unmasked
            pub const BM1_0: u32 = 0b0;

            /// 0b1: Byte Masked
            pub const BM1_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 2 (IPTXD bit 23:16)
    pub mod BM2 {
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

            /// 0b0: Byte Unmasked
            pub const BM2_0: u32 = 0b0;

            /// 0b1: Byte Masked
            pub const BM2_1: u32 = 0b1;
        }
    }

    /// Byte Mask for Byte 3 (IPTXD bit 31:24)
    pub mod BM3 {
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

            /// 0b0: Byte Unmasked
            pub const BM3_0: u32 = 0b0;

            /// 0b1: Byte Masked
            pub const BM3_1: u32 = 0b1;
        }
    }
}

/// IP Command register
pub mod IPCMD {

    /// SDRAM Commands: 0x8: READ 0x9: WRITE 0xA: MODESET 0xB: ACTIVE 0xC: AUTO REFRESH 0xD: SELF REFRESH 0xE: PRECHARGE 0xF: PRECHARGE ALL Others: RSVD SELF REFRESH will be sent to all SDRAM devices because they shared same SEMC_CLK pin
    pub mod CMD {
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

    /// This field should be written with 0xA55A when trigging an IP command.
    pub mod KEY {
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

/// TX DATA register (for IP Command)
pub mod IPTXDAT {

    /// no description available
    pub mod DAT {
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

/// RX DATA register (for IP Command)
pub mod IPRXDAT {

    /// no description available
    pub mod DAT {
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

/// Status register 0
pub mod STS0 {

    /// Indicating whether SEMC is in IDLE state.
    pub mod IDLE {
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

    /// Indicating NAND device Ready/WAIT# pin level.
    pub mod NARDY {
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

            /// 0b0: NAND device is not ready
            pub const NARDY_0: u32 = 0b0;

            /// 0b1: NAND device is ready
            pub const NARDY_1: u32 = 0b1;
        }
    }
}

/// Status register 1
pub mod STS1 {}

/// Status register 2
pub mod STS2 {

    /// This field indicating whether there is pending AXI command (write) to NAND device.
    pub mod NDWRPEND {
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

            /// 0b0: No pending
            pub const NDWRPEND_0: u32 = 0b0;

            /// 0b1: Pending
            pub const NDWRPEND_1: u32 = 0b1;
        }
    }
}

/// Status register 3
pub mod STS3 {}

/// Status register 4
pub mod STS4 {}

/// Status register 5
pub mod STS5 {}

/// Status register 6
pub mod STS6 {}

/// Status register 7
pub mod STS7 {}

/// Status register 8
pub mod STS8 {}

/// Status register 9
pub mod STS9 {}

/// Status register 10
pub mod STS10 {}

/// Status register 11
pub mod STS11 {}

/// Status register 12
pub mod STS12 {

    /// This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4).
    pub mod NDADDR {
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

/// Status register 13
pub mod STS13 {}

/// Status register 14
pub mod STS14 {}

/// Status register 15
pub mod STS15 {}
pub struct RegisterBlock {
    /// Module Control Register
    pub MCR: RWRegister<u32>,

    /// IO Mux Control Register
    pub IOCR: RWRegister<u32>,

    /// Master Bus (AXI) Control Register 0
    pub BMCR0: RWRegister<u32>,

    /// Master Bus (AXI) Control Register 1
    pub BMCR1: RWRegister<u32>,

    /// Base Register 0 (For SDRAM CS0 device)
    pub BR0: RWRegister<u32>,

    /// Base Register 1 (For SDRAM CS1 device)
    pub BR1: RWRegister<u32>,

    /// Base Register 2 (For SDRAM CS2 device)
    pub BR2: RWRegister<u32>,

    /// Base Register 3 (For SDRAM CS3 device)
    pub BR3: RWRegister<u32>,

    /// Base Register 4 (For NAND device)
    pub BR4: RWRegister<u32>,

    /// Base Register 5 (For NOR device)
    pub BR5: RWRegister<u32>,

    /// Base Register 6 (For PSRAM device)
    pub BR6: RWRegister<u32>,

    /// Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)
    pub BR7: RWRegister<u32>,

    /// Base Register 8 (For NAND device)
    pub BR8: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// Interrupt Enable Register
    pub INTEN: RWRegister<u32>,

    /// Interrupt Enable Register
    pub INTR: RWRegister<u32>,

    /// SDRAM control register 0
    pub SDRAMCR0: RWRegister<u32>,

    /// SDRAM control register 1
    pub SDRAMCR1: RWRegister<u32>,

    /// SDRAM control register 2
    pub SDRAMCR2: RWRegister<u32>,

    /// SDRAM control register 3
    pub SDRAMCR3: RWRegister<u32>,

    /// NAND control register 0
    pub NANDCR0: RWRegister<u32>,

    /// NAND control register 1
    pub NANDCR1: RWRegister<u32>,

    /// NAND control register 2
    pub NANDCR2: RWRegister<u32>,

    /// NAND control register 3
    pub NANDCR3: RWRegister<u32>,

    /// NOR control register 0
    pub NORCR0: RWRegister<u32>,

    /// NOR control register 1
    pub NORCR1: RWRegister<u32>,

    /// NOR control register 2
    pub NORCR2: RWRegister<u32>,

    /// NOR control register 3
    pub NORCR3: RWRegister<u32>,

    /// SRAM control register 0
    pub SRAMCR0: RWRegister<u32>,

    /// SRAM control register 1
    pub SRAMCR1: RWRegister<u32>,

    /// SRAM control register 2
    pub SRAMCR2: RWRegister<u32>,

    /// SRAM control register 3
    pub SRAMCR3: RWRegister<u32>,

    /// DBI-B control register 0
    pub DBICR0: RWRegister<u32>,

    /// DBI-B control register 1
    pub DBICR1: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// IP Command control register 0
    pub IPCR0: RWRegister<u32>,

    /// IP Command control register 1
    pub IPCR1: RWRegister<u32>,

    /// IP Command control register 2
    pub IPCR2: RWRegister<u32>,

    /// IP Command register
    pub IPCMD: RWRegister<u32>,

    /// TX DATA register (for IP Command)
    pub IPTXDAT: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// RX DATA register (for IP Command)
    pub IPRXDAT: RORegister<u32>,

    _reserved4: [u32; 3],

    /// Status register 0
    pub STS0: RORegister<u32>,

    /// Status register 1
    pub STS1: RORegister<u32>,

    /// Status register 2
    pub STS2: RORegister<u32>,

    /// Status register 3
    pub STS3: RORegister<u32>,

    /// Status register 4
    pub STS4: RORegister<u32>,

    /// Status register 5
    pub STS5: RORegister<u32>,

    /// Status register 6
    pub STS6: RORegister<u32>,

    /// Status register 7
    pub STS7: RORegister<u32>,

    /// Status register 8
    pub STS8: RORegister<u32>,

    /// Status register 9
    pub STS9: RORegister<u32>,

    /// Status register 10
    pub STS10: RORegister<u32>,

    /// Status register 11
    pub STS11: RORegister<u32>,

    /// Status register 12
    pub STS12: RORegister<u32>,

    /// Status register 13
    pub STS13: RORegister<u32>,

    /// Status register 14
    pub STS14: RORegister<u32>,

    /// Status register 15
    pub STS15: RORegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub IOCR: u32,
    pub BMCR0: u32,
    pub BMCR1: u32,
    pub BR0: u32,
    pub BR1: u32,
    pub BR2: u32,
    pub BR3: u32,
    pub BR4: u32,
    pub BR5: u32,
    pub BR6: u32,
    pub BR7: u32,
    pub BR8: u32,
    pub INTEN: u32,
    pub INTR: u32,
    pub SDRAMCR0: u32,
    pub SDRAMCR1: u32,
    pub SDRAMCR2: u32,
    pub SDRAMCR3: u32,
    pub NANDCR0: u32,
    pub NANDCR1: u32,
    pub NANDCR2: u32,
    pub NANDCR3: u32,
    pub NORCR0: u32,
    pub NORCR1: u32,
    pub NORCR2: u32,
    pub NORCR3: u32,
    pub SRAMCR0: u32,
    pub SRAMCR1: u32,
    pub SRAMCR2: u32,
    pub SRAMCR3: u32,
    pub DBICR0: u32,
    pub DBICR1: u32,
    pub IPCR0: u32,
    pub IPCR1: u32,
    pub IPCR2: u32,
    pub IPCMD: u32,
    pub IPTXDAT: u32,
    pub IPRXDAT: u32,
    pub STS0: u32,
    pub STS1: u32,
    pub STS2: u32,
    pub STS3: u32,
    pub STS4: u32,
    pub STS5: u32,
    pub STS6: u32,
    pub STS7: u32,
    pub STS8: u32,
    pub STS9: u32,
    pub STS10: u32,
    pub STS11: u32,
    pub STS12: u32,
    pub STS13: u32,
    pub STS14: u32,
    pub STS15: u32,
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
