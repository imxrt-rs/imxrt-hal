#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Crossbar A Select Register 0
pub mod SEL0 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)
    pub mod SEL0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)
    pub mod SEL1 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 1
pub mod SEL1 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT2 (refer to Functional Description section for input/output assignment)
    pub mod SEL2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT3 (refer to Functional Description section for input/output assignment)
    pub mod SEL3 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 2
pub mod SEL2 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT4 (refer to Functional Description section for input/output assignment)
    pub mod SEL4 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT5 (refer to Functional Description section for input/output assignment)
    pub mod SEL5 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 3
pub mod SEL3 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT6 (refer to Functional Description section for input/output assignment)
    pub mod SEL6 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT7 (refer to Functional Description section for input/output assignment)
    pub mod SEL7 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 4
pub mod SEL4 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)
    pub mod SEL8 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)
    pub mod SEL9 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 5
pub mod SEL5 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT10 (refer to Functional Description section for input/output assignment)
    pub mod SEL10 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT11 (refer to Functional Description section for input/output assignment)
    pub mod SEL11 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 6
pub mod SEL6 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)
    pub mod SEL12 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)
    pub mod SEL13 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 7
pub mod SEL7 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT14 (refer to Functional Description section for input/output assignment)
    pub mod SEL14 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT15 (refer to Functional Description section for input/output assignment)
    pub mod SEL15 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 8
pub mod SEL8 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)
    pub mod SEL16 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)
    pub mod SEL17 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 9
pub mod SEL9 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)
    pub mod SEL18 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)
    pub mod SEL19 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 10
pub mod SEL10 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)
    pub mod SEL20 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)
    pub mod SEL21 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 11
pub mod SEL11 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT22 (refer to Functional Description section for input/output assignment)
    pub mod SEL22 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT23 (refer to Functional Description section for input/output assignment)
    pub mod SEL23 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 12
pub mod SEL12 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)
    pub mod SEL24 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)
    pub mod SEL25 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 13
pub mod SEL13 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)
    pub mod SEL26 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)
    pub mod SEL27 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 14
pub mod SEL14 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)
    pub mod SEL28 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)
    pub mod SEL29 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 15
pub mod SEL15 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)
    pub mod SEL30 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)
    pub mod SEL31 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 16
pub mod SEL16 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)
    pub mod SEL32 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)
    pub mod SEL33 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 17
pub mod SEL17 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)
    pub mod SEL34 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)
    pub mod SEL35 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 18
pub mod SEL18 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)
    pub mod SEL36 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)
    pub mod SEL37 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 19
pub mod SEL19 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)
    pub mod SEL38 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)
    pub mod SEL39 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 20
pub mod SEL20 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)
    pub mod SEL40 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)
    pub mod SEL41 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 21
pub mod SEL21 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)
    pub mod SEL42 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)
    pub mod SEL43 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 22
pub mod SEL22 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)
    pub mod SEL44 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)
    pub mod SEL45 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 23
pub mod SEL23 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)
    pub mod SEL46 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)
    pub mod SEL47 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 24
pub mod SEL24 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)
    pub mod SEL48 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)
    pub mod SEL49 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 25
pub mod SEL25 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT50 (refer to Functional Description section for input/output assignment)
    pub mod SEL50 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT51 (refer to Functional Description section for input/output assignment)
    pub mod SEL51 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 26
pub mod SEL26 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT52 (refer to Functional Description section for input/output assignment)
    pub mod SEL52 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT53 (refer to Functional Description section for input/output assignment)
    pub mod SEL53 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 27
pub mod SEL27 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)
    pub mod SEL54 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)
    pub mod SEL55 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 28
pub mod SEL28 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)
    pub mod SEL56 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)
    pub mod SEL57 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 29
pub mod SEL29 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)
    pub mod SEL58 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)
    pub mod SEL59 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 30
pub mod SEL30 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)
    pub mod SEL60 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)
    pub mod SEL61 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 31
pub mod SEL31 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)
    pub mod SEL62 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)
    pub mod SEL63 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 32
pub mod SEL32 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)
    pub mod SEL64 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)
    pub mod SEL65 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 33
pub mod SEL33 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)
    pub mod SEL66 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)
    pub mod SEL67 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 34
pub mod SEL34 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)
    pub mod SEL68 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)
    pub mod SEL69 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 35
pub mod SEL35 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)
    pub mod SEL70 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)
    pub mod SEL71 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 36
pub mod SEL36 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)
    pub mod SEL72 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)
    pub mod SEL73 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 37
pub mod SEL37 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT74 (refer to Functional Description section for input/output assignment)
    pub mod SEL74 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT75 (refer to Functional Description section for input/output assignment)
    pub mod SEL75 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 38
pub mod SEL38 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT76 (refer to Functional Description section for input/output assignment)
    pub mod SEL76 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT77 (refer to Functional Description section for input/output assignment)
    pub mod SEL77 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 39
pub mod SEL39 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)
    pub mod SEL78 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)
    pub mod SEL79 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 40
pub mod SEL40 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT80 (refer to Functional Description section for input/output assignment)
    pub mod SEL80 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT81 (refer to Functional Description section for input/output assignment)
    pub mod SEL81 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 41
pub mod SEL41 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)
    pub mod SEL82 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)
    pub mod SEL83 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 42
pub mod SEL42 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)
    pub mod SEL84 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)
    pub mod SEL85 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 43
pub mod SEL43 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)
    pub mod SEL86 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)
    pub mod SEL87 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 44
pub mod SEL44 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)
    pub mod SEL88 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)
    pub mod SEL89 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 45
pub mod SEL45 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)
    pub mod SEL90 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)
    pub mod SEL91 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 46
pub mod SEL46 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)
    pub mod SEL92 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)
    pub mod SEL93 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 47
pub mod SEL47 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)
    pub mod SEL94 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)
    pub mod SEL95 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 48
pub mod SEL48 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)
    pub mod SEL96 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)
    pub mod SEL97 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 49
pub mod SEL49 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)
    pub mod SEL98 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)
    pub mod SEL99 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 50
pub mod SEL50 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)
    pub mod SEL100 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)
    pub mod SEL101 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 51
pub mod SEL51 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)
    pub mod SEL102 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)
    pub mod SEL103 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 52
pub mod SEL52 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)
    pub mod SEL104 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)
    pub mod SEL105 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 53
pub mod SEL53 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)
    pub mod SEL106 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)
    pub mod SEL107 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 54
pub mod SEL54 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)
    pub mod SEL108 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)
    pub mod SEL109 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 55
pub mod SEL55 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)
    pub mod SEL110 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)
    pub mod SEL111 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 56
pub mod SEL56 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)
    pub mod SEL112 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)
    pub mod SEL113 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 57
pub mod SEL57 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)
    pub mod SEL114 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)
    pub mod SEL115 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 58
pub mod SEL58 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)
    pub mod SEL116 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)
    pub mod SEL117 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 59
pub mod SEL59 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)
    pub mod SEL118 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)
    pub mod SEL119 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 60
pub mod SEL60 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT120 (refer to Functional Description section for input/output assignment)
    pub mod SEL120 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT121 (refer to Functional Description section for input/output assignment)
    pub mod SEL121 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 61
pub mod SEL61 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)
    pub mod SEL122 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)
    pub mod SEL123 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 62
pub mod SEL62 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)
    pub mod SEL124 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)
    pub mod SEL125 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 63
pub mod SEL63 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)
    pub mod SEL126 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)
    pub mod SEL127 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 64
pub mod SEL64 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)
    pub mod SEL128 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)
    pub mod SEL129 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Select Register 65
pub mod SEL65 {

    /// Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)
    pub mod SEL130 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)
    pub mod SEL131 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (7 bits: 0x7f << 8)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Crossbar A Control Register 0
pub mod CTRL0 {

    /// DMA Enable for XBAR_OUT0
    pub mod DEN0 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled
            pub const DEN0_0: u16 = 0b0;

            /// 0b1: DMA enabled
            pub const DEN0_1: u16 = 0b1;
        }
    }

    /// Interrupt Enable for XBAR_OUT0
    pub mod IEN0 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt disabled
            pub const IEN0_0: u16 = 0b0;

            /// 0b1: Interrupt enabled
            pub const IEN0_1: u16 = 0b1;
        }
    }

    /// Active edge for edge detection on XBAR_OUT0
    pub mod EDGE0 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: STS0 never asserts
            pub const EDGE0_0: u16 = 0b00;

            /// 0b01: STS0 asserts on rising edges of XBAR_OUT0
            pub const EDGE0_1: u16 = 0b01;

            /// 0b10: STS0 asserts on falling edges of XBAR_OUT0
            pub const EDGE0_2: u16 = 0b10;

            /// 0b11: STS0 asserts on rising and falling edges of XBAR_OUT0
            pub const EDGE0_3: u16 = 0b11;
        }
    }

    /// Edge detection status for XBAR_OUT0
    pub mod STS0 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active edge not yet detected on XBAR_OUT0
            pub const STS0_0: u16 = 0b0;

            /// 0b1: Active edge detected on XBAR_OUT0
            pub const STS0_1: u16 = 0b1;
        }
    }

    /// DMA Enable for XBAR_OUT1
    pub mod DEN1 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled
            pub const DEN1_0: u16 = 0b0;

            /// 0b1: DMA enabled
            pub const DEN1_1: u16 = 0b1;
        }
    }

    /// Interrupt Enable for XBAR_OUT1
    pub mod IEN1 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt disabled
            pub const IEN1_0: u16 = 0b0;

            /// 0b1: Interrupt enabled
            pub const IEN1_1: u16 = 0b1;
        }
    }

    /// Active edge for edge detection on XBAR_OUT1
    pub mod EDGE1 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: STS1 never asserts
            pub const EDGE1_0: u16 = 0b00;

            /// 0b01: STS1 asserts on rising edges of XBAR_OUT1
            pub const EDGE1_1: u16 = 0b01;

            /// 0b10: STS1 asserts on falling edges of XBAR_OUT1
            pub const EDGE1_2: u16 = 0b10;

            /// 0b11: STS1 asserts on rising and falling edges of XBAR_OUT1
            pub const EDGE1_3: u16 = 0b11;
        }
    }

    /// Edge detection status for XBAR_OUT1
    pub mod STS1 {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active edge not yet detected on XBAR_OUT1
            pub const STS1_0: u16 = 0b0;

            /// 0b1: Active edge detected on XBAR_OUT1
            pub const STS1_1: u16 = 0b1;
        }
    }
}

/// Crossbar A Control Register 1
pub mod CTRL1 {

    /// DMA Enable for XBAR_OUT2
    pub mod DEN2 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled
            pub const DEN2_0: u16 = 0b0;

            /// 0b1: DMA enabled
            pub const DEN2_1: u16 = 0b1;
        }
    }

    /// Interrupt Enable for XBAR_OUT2
    pub mod IEN2 {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt disabled
            pub const IEN2_0: u16 = 0b0;

            /// 0b1: Interrupt enabled
            pub const IEN2_1: u16 = 0b1;
        }
    }

    /// Active edge for edge detection on XBAR_OUT2
    pub mod EDGE2 {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: STS2 never asserts
            pub const EDGE2_0: u16 = 0b00;

            /// 0b01: STS2 asserts on rising edges of XBAR_OUT2
            pub const EDGE2_1: u16 = 0b01;

            /// 0b10: STS2 asserts on falling edges of XBAR_OUT2
            pub const EDGE2_2: u16 = 0b10;

            /// 0b11: STS2 asserts on rising and falling edges of XBAR_OUT2
            pub const EDGE2_3: u16 = 0b11;
        }
    }

    /// Edge detection status for XBAR_OUT2
    pub mod STS2 {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active edge not yet detected on XBAR_OUT2
            pub const STS2_0: u16 = 0b0;

            /// 0b1: Active edge detected on XBAR_OUT2
            pub const STS2_1: u16 = 0b1;
        }
    }

    /// DMA Enable for XBAR_OUT3
    pub mod DEN3 {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA disabled
            pub const DEN3_0: u16 = 0b0;

            /// 0b1: DMA enabled
            pub const DEN3_1: u16 = 0b1;
        }
    }

    /// Interrupt Enable for XBAR_OUT3
    pub mod IEN3 {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt disabled
            pub const IEN3_0: u16 = 0b0;

            /// 0b1: Interrupt enabled
            pub const IEN3_1: u16 = 0b1;
        }
    }

    /// Active edge for edge detection on XBAR_OUT3
    pub mod EDGE3 {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: STS3 never asserts
            pub const EDGE3_0: u16 = 0b00;

            /// 0b01: STS3 asserts on rising edges of XBAR_OUT3
            pub const EDGE3_1: u16 = 0b01;

            /// 0b10: STS3 asserts on falling edges of XBAR_OUT3
            pub const EDGE3_2: u16 = 0b10;

            /// 0b11: STS3 asserts on rising and falling edges of XBAR_OUT3
            pub const EDGE3_3: u16 = 0b11;
        }
    }

    /// Edge detection status for XBAR_OUT3
    pub mod STS3 {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active edge not yet detected on XBAR_OUT3
            pub const STS3_0: u16 = 0b0;

            /// 0b1: Active edge detected on XBAR_OUT3
            pub const STS3_1: u16 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Crossbar A Select Register 0
    pub SEL0: RWRegister<u16>,

    /// Crossbar A Select Register 1
    pub SEL1: RWRegister<u16>,

    /// Crossbar A Select Register 2
    pub SEL2: RWRegister<u16>,

    /// Crossbar A Select Register 3
    pub SEL3: RWRegister<u16>,

    /// Crossbar A Select Register 4
    pub SEL4: RWRegister<u16>,

    /// Crossbar A Select Register 5
    pub SEL5: RWRegister<u16>,

    /// Crossbar A Select Register 6
    pub SEL6: RWRegister<u16>,

    /// Crossbar A Select Register 7
    pub SEL7: RWRegister<u16>,

    /// Crossbar A Select Register 8
    pub SEL8: RWRegister<u16>,

    /// Crossbar A Select Register 9
    pub SEL9: RWRegister<u16>,

    /// Crossbar A Select Register 10
    pub SEL10: RWRegister<u16>,

    /// Crossbar A Select Register 11
    pub SEL11: RWRegister<u16>,

    /// Crossbar A Select Register 12
    pub SEL12: RWRegister<u16>,

    /// Crossbar A Select Register 13
    pub SEL13: RWRegister<u16>,

    /// Crossbar A Select Register 14
    pub SEL14: RWRegister<u16>,

    /// Crossbar A Select Register 15
    pub SEL15: RWRegister<u16>,

    /// Crossbar A Select Register 16
    pub SEL16: RWRegister<u16>,

    /// Crossbar A Select Register 17
    pub SEL17: RWRegister<u16>,

    /// Crossbar A Select Register 18
    pub SEL18: RWRegister<u16>,

    /// Crossbar A Select Register 19
    pub SEL19: RWRegister<u16>,

    /// Crossbar A Select Register 20
    pub SEL20: RWRegister<u16>,

    /// Crossbar A Select Register 21
    pub SEL21: RWRegister<u16>,

    /// Crossbar A Select Register 22
    pub SEL22: RWRegister<u16>,

    /// Crossbar A Select Register 23
    pub SEL23: RWRegister<u16>,

    /// Crossbar A Select Register 24
    pub SEL24: RWRegister<u16>,

    /// Crossbar A Select Register 25
    pub SEL25: RWRegister<u16>,

    /// Crossbar A Select Register 26
    pub SEL26: RWRegister<u16>,

    /// Crossbar A Select Register 27
    pub SEL27: RWRegister<u16>,

    /// Crossbar A Select Register 28
    pub SEL28: RWRegister<u16>,

    /// Crossbar A Select Register 29
    pub SEL29: RWRegister<u16>,

    /// Crossbar A Select Register 30
    pub SEL30: RWRegister<u16>,

    /// Crossbar A Select Register 31
    pub SEL31: RWRegister<u16>,

    /// Crossbar A Select Register 32
    pub SEL32: RWRegister<u16>,

    /// Crossbar A Select Register 33
    pub SEL33: RWRegister<u16>,

    /// Crossbar A Select Register 34
    pub SEL34: RWRegister<u16>,

    /// Crossbar A Select Register 35
    pub SEL35: RWRegister<u16>,

    /// Crossbar A Select Register 36
    pub SEL36: RWRegister<u16>,

    /// Crossbar A Select Register 37
    pub SEL37: RWRegister<u16>,

    /// Crossbar A Select Register 38
    pub SEL38: RWRegister<u16>,

    /// Crossbar A Select Register 39
    pub SEL39: RWRegister<u16>,

    /// Crossbar A Select Register 40
    pub SEL40: RWRegister<u16>,

    /// Crossbar A Select Register 41
    pub SEL41: RWRegister<u16>,

    /// Crossbar A Select Register 42
    pub SEL42: RWRegister<u16>,

    /// Crossbar A Select Register 43
    pub SEL43: RWRegister<u16>,

    /// Crossbar A Select Register 44
    pub SEL44: RWRegister<u16>,

    /// Crossbar A Select Register 45
    pub SEL45: RWRegister<u16>,

    /// Crossbar A Select Register 46
    pub SEL46: RWRegister<u16>,

    /// Crossbar A Select Register 47
    pub SEL47: RWRegister<u16>,

    /// Crossbar A Select Register 48
    pub SEL48: RWRegister<u16>,

    /// Crossbar A Select Register 49
    pub SEL49: RWRegister<u16>,

    /// Crossbar A Select Register 50
    pub SEL50: RWRegister<u16>,

    /// Crossbar A Select Register 51
    pub SEL51: RWRegister<u16>,

    /// Crossbar A Select Register 52
    pub SEL52: RWRegister<u16>,

    /// Crossbar A Select Register 53
    pub SEL53: RWRegister<u16>,

    /// Crossbar A Select Register 54
    pub SEL54: RWRegister<u16>,

    /// Crossbar A Select Register 55
    pub SEL55: RWRegister<u16>,

    /// Crossbar A Select Register 56
    pub SEL56: RWRegister<u16>,

    /// Crossbar A Select Register 57
    pub SEL57: RWRegister<u16>,

    /// Crossbar A Select Register 58
    pub SEL58: RWRegister<u16>,

    /// Crossbar A Select Register 59
    pub SEL59: RWRegister<u16>,

    /// Crossbar A Select Register 60
    pub SEL60: RWRegister<u16>,

    /// Crossbar A Select Register 61
    pub SEL61: RWRegister<u16>,

    /// Crossbar A Select Register 62
    pub SEL62: RWRegister<u16>,

    /// Crossbar A Select Register 63
    pub SEL63: RWRegister<u16>,

    /// Crossbar A Select Register 64
    pub SEL64: RWRegister<u16>,

    /// Crossbar A Select Register 65
    pub SEL65: RWRegister<u16>,

    /// Crossbar A Control Register 0
    pub CTRL0: RWRegister<u16>,

    /// Crossbar A Control Register 1
    pub CTRL1: RWRegister<u16>,
}
pub struct ResetValues {
    pub SEL0: u16,
    pub SEL1: u16,
    pub SEL2: u16,
    pub SEL3: u16,
    pub SEL4: u16,
    pub SEL5: u16,
    pub SEL6: u16,
    pub SEL7: u16,
    pub SEL8: u16,
    pub SEL9: u16,
    pub SEL10: u16,
    pub SEL11: u16,
    pub SEL12: u16,
    pub SEL13: u16,
    pub SEL14: u16,
    pub SEL15: u16,
    pub SEL16: u16,
    pub SEL17: u16,
    pub SEL18: u16,
    pub SEL19: u16,
    pub SEL20: u16,
    pub SEL21: u16,
    pub SEL22: u16,
    pub SEL23: u16,
    pub SEL24: u16,
    pub SEL25: u16,
    pub SEL26: u16,
    pub SEL27: u16,
    pub SEL28: u16,
    pub SEL29: u16,
    pub SEL30: u16,
    pub SEL31: u16,
    pub SEL32: u16,
    pub SEL33: u16,
    pub SEL34: u16,
    pub SEL35: u16,
    pub SEL36: u16,
    pub SEL37: u16,
    pub SEL38: u16,
    pub SEL39: u16,
    pub SEL40: u16,
    pub SEL41: u16,
    pub SEL42: u16,
    pub SEL43: u16,
    pub SEL44: u16,
    pub SEL45: u16,
    pub SEL46: u16,
    pub SEL47: u16,
    pub SEL48: u16,
    pub SEL49: u16,
    pub SEL50: u16,
    pub SEL51: u16,
    pub SEL52: u16,
    pub SEL53: u16,
    pub SEL54: u16,
    pub SEL55: u16,
    pub SEL56: u16,
    pub SEL57: u16,
    pub SEL58: u16,
    pub SEL59: u16,
    pub SEL60: u16,
    pub SEL61: u16,
    pub SEL62: u16,
    pub SEL63: u16,
    pub SEL64: u16,
    pub SEL65: u16,
    pub CTRL0: u16,
    pub CTRL1: u16,
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
