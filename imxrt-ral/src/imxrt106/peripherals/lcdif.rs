#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCDIF Register Reference Index
//!
//! Used by: imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// LCDIF General Control Register
pub mod CTRL {

    /// When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display
    pub mod RUN {
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

    /// Used only when WORD_LENGTH = 3, i
    pub mod DATA_FORMAT_24_BIT {
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

            /// 0b0: Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits.
            pub const ALL_24_BITS_VALID: u32 = 0b0;

            /// 0b1: Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped.
            pub const DROP_UPPER_2_BITS_PER_BYTE: u32 = 0b1;
        }
    }

    /// Used only when WORD_LENGTH = 2, i.e. 18-bit.
    pub mod DATA_FORMAT_18_BIT {
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

            /// 0b0: Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data.
            pub const LOWER_18_BITS_VALID: u32 = 0b0;

            /// 0b1: Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data.
            pub const UPPER_18_BITS_VALID: u32 = 0b1;
        }
    }

    /// When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format
    pub mod DATA_FORMAT_16_BIT {
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

    /// Set this bit to make the LCDIF act as a bus master
    pub mod MASTER {
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

    /// If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on
    pub mod ENABLE_PXP_HANDSHAKE {
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

    /// Input data format.
    pub mod WORD_LENGTH {
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

            /// 0b00: Input data is 16 bits per pixel.
            pub const _16_BIT: u32 = 0b00;

            /// 0b01: Input data is 8 bits wide.
            pub const _8_BIT: u32 = 0b01;

            /// 0b10: Input data is 18 bits per pixel.
            pub const _18_BIT: u32 = 0b10;

            /// 0b11: Input data is 24 bits per pixel.
            pub const _24_BIT: u32 = 0b11;
        }
    }

    /// LCD Data bus transfer width.
    pub mod LCD_DATABUS_WIDTH {
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

            /// 0b00: 16-bit data bus mode.
            pub const _16_BIT: u32 = 0b00;

            /// 0b01: 8-bit data bus mode.
            pub const _8_BIT: u32 = 0b01;

            /// 0b10: 18-bit data bus mode.
            pub const _18_BIT: u32 = 0b10;

            /// 0b11: 24-bit data bus mode.
            pub const _24_BIT: u32 = 0b11;
        }
    }

    /// This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus
    pub mod CSC_DATA_SWIZZLE {
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

            /// 0b00: No byte swapping.(Little endian)
            pub const NO_SWAP: u32 = 0b00;

            /// 0b01: Big Endian swap (swap bytes 0,3 and 1,2).
            pub const BIG_ENDIAN_SWAP: u32 = 0b01;

            /// 0b10: Swap half-words.
            pub const HWD_SWAP: u32 = 0b10;

            /// 0b11: Swap bytes within each half-word.
            pub const HWD_BYTE_SWAP: u32 = 0b11;
        }
    }

    /// This field specifies how to swap the bytes fetched by the bus master interface
    pub mod INPUT_DATA_SWIZZLE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CSC_DATA_SWIZZLE::RW;
    }

    /// Set this bit to 1 to make the hardware go into the DOTCLK mode, i
    pub mod DOTCLK_MODE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out
    pub mod BYPASS_COUNT {
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

    /// The data to be transmitted is shifted left or right by this number of bits.
    pub mod SHIFT_NUM_BITS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (5 bits: 0b11111 << 21)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Use this bit to determine the direction of shift of transmit data.
    pub mod DATA_SHIFT_DIR {
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

            /// 0b0: Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits.
            pub const TXDATA_SHIFT_LEFT: u32 = 0b0;

            /// 0b1: Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits.
            pub const TXDATA_SHIFT_RIGHT: u32 = 0b1;
        }
    }

    /// This bit must be set to zero for normal operation
    pub mod CLKGATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit must be set to zero to enable normal operation of the LCDIF
    pub mod SFTRST {
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

/// LCDIF General Control Register
pub mod CTRL_SET {
    pub use super::CTRL::BYPASS_COUNT;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::CSC_DATA_SWIZZLE;
    pub use super::CTRL::DATA_FORMAT_16_BIT;
    pub use super::CTRL::DATA_FORMAT_18_BIT;
    pub use super::CTRL::DATA_FORMAT_24_BIT;
    pub use super::CTRL::DATA_SHIFT_DIR;
    pub use super::CTRL::DOTCLK_MODE;
    pub use super::CTRL::ENABLE_PXP_HANDSHAKE;
    pub use super::CTRL::INPUT_DATA_SWIZZLE;
    pub use super::CTRL::LCD_DATABUS_WIDTH;
    pub use super::CTRL::MASTER;
    pub use super::CTRL::RUN;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::SHIFT_NUM_BITS;
    pub use super::CTRL::WORD_LENGTH;
}

/// LCDIF General Control Register
pub mod CTRL_CLR {
    pub use super::CTRL::BYPASS_COUNT;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::CSC_DATA_SWIZZLE;
    pub use super::CTRL::DATA_FORMAT_16_BIT;
    pub use super::CTRL::DATA_FORMAT_18_BIT;
    pub use super::CTRL::DATA_FORMAT_24_BIT;
    pub use super::CTRL::DATA_SHIFT_DIR;
    pub use super::CTRL::DOTCLK_MODE;
    pub use super::CTRL::ENABLE_PXP_HANDSHAKE;
    pub use super::CTRL::INPUT_DATA_SWIZZLE;
    pub use super::CTRL::LCD_DATABUS_WIDTH;
    pub use super::CTRL::MASTER;
    pub use super::CTRL::RUN;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::SHIFT_NUM_BITS;
    pub use super::CTRL::WORD_LENGTH;
}

/// LCDIF General Control Register
pub mod CTRL_TOG {
    pub use super::CTRL::BYPASS_COUNT;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::CSC_DATA_SWIZZLE;
    pub use super::CTRL::DATA_FORMAT_16_BIT;
    pub use super::CTRL::DATA_FORMAT_18_BIT;
    pub use super::CTRL::DATA_FORMAT_24_BIT;
    pub use super::CTRL::DATA_SHIFT_DIR;
    pub use super::CTRL::DOTCLK_MODE;
    pub use super::CTRL::ENABLE_PXP_HANDSHAKE;
    pub use super::CTRL::INPUT_DATA_SWIZZLE;
    pub use super::CTRL::LCD_DATABUS_WIDTH;
    pub use super::CTRL::MASTER;
    pub use super::CTRL::RUN;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::SHIFT_NUM_BITS;
    pub use super::CTRL::WORD_LENGTH;
}

/// LCDIF General Control1 Register
pub mod CTRL1 {

    /// This bit is set to indicate that an interrupt is requested by the LCDIF block
    pub mod VSYNC_EDGE_IRQ {
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

            /// 0b0: No Interrupt Request Pending.
            pub const NO_REQUEST: u32 = 0b0;

            /// 0b1: Interrupt Request Pending.
            pub const REQUEST: u32 = 0b1;
        }
    }

    /// This bit is set to indicate that an interrupt is requested by the LCDIF block
    pub mod CUR_FRAME_DONE_IRQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VSYNC_EDGE_IRQ::RW;
    }

    /// This bit is set to indicate that an interrupt is requested by the LCDIF block
    pub mod UNDERFLOW_IRQ {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VSYNC_EDGE_IRQ::RW;
    }

    /// This bit is set to indicate that an interrupt is requested by the LCDIF block
    pub mod OVERFLOW_IRQ {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VSYNC_EDGE_IRQ::RW;
    }

    /// This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode
    pub mod VSYNC_EDGE_IRQ_EN {
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

    /// This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state
    pub mod CUR_FRAME_DONE_IRQ_EN {
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

    /// This bit is set to enable an underflow interrupt in the TXFIFO in the write mode.
    pub mod UNDERFLOW_IRQ_EN {
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

    /// This bit is set to enable an overflow interrupt in the TXFIFO in the write mode.
    pub mod OVERFLOW_IRQ_EN {
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

    /// This bitfield is used to show which data bytes in a 32-bit word are valid
    pub mod BYTE_PACKING_FORMAT {
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

    /// If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field
    pub mod IRQ_ON_ALTERNATE_FIELDS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO.
    pub mod FIFO_CLEAR {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The default is to grab the odd lines first and then the even lines
    pub mod START_INTERLACE_FROM_SECOND_FIELD {
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

    /// Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field
    pub mod INTERLACE_FIELDS {
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

    /// Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame
    pub mod RECOVER_ON_UNDERFLOW {
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

    /// This bit is set to indicate that an interrupt is requested by the LCDIF block
    pub mod BM_ERROR_IRQ {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::VSYNC_EDGE_IRQ::RW;
    }

    /// This bit is set to enable bus master error interrupt in the LCDIF master mode.
    pub mod BM_ERROR_IRQ_EN {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit is CS0/CS1 valid select signals
    pub mod CS_OUT_SELECT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command Mode MIPI image data select bit
    pub mod IMAGE_DATA_SELECT {
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

/// LCDIF General Control1 Register
pub mod CTRL1_SET {
    pub use super::CTRL1::BM_ERROR_IRQ;
    pub use super::CTRL1::BM_ERROR_IRQ_EN;
    pub use super::CTRL1::BYTE_PACKING_FORMAT;
    pub use super::CTRL1::CS_OUT_SELECT;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ_EN;
    pub use super::CTRL1::FIFO_CLEAR;
    pub use super::CTRL1::IMAGE_DATA_SELECT;
    pub use super::CTRL1::INTERLACE_FIELDS;
    pub use super::CTRL1::IRQ_ON_ALTERNATE_FIELDS;
    pub use super::CTRL1::OVERFLOW_IRQ;
    pub use super::CTRL1::OVERFLOW_IRQ_EN;
    pub use super::CTRL1::RECOVER_ON_UNDERFLOW;
    pub use super::CTRL1::START_INTERLACE_FROM_SECOND_FIELD;
    pub use super::CTRL1::UNDERFLOW_IRQ;
    pub use super::CTRL1::UNDERFLOW_IRQ_EN;
    pub use super::CTRL1::VSYNC_EDGE_IRQ;
    pub use super::CTRL1::VSYNC_EDGE_IRQ_EN;
}

/// LCDIF General Control1 Register
pub mod CTRL1_CLR {
    pub use super::CTRL1::BM_ERROR_IRQ;
    pub use super::CTRL1::BM_ERROR_IRQ_EN;
    pub use super::CTRL1::BYTE_PACKING_FORMAT;
    pub use super::CTRL1::CS_OUT_SELECT;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ_EN;
    pub use super::CTRL1::FIFO_CLEAR;
    pub use super::CTRL1::IMAGE_DATA_SELECT;
    pub use super::CTRL1::INTERLACE_FIELDS;
    pub use super::CTRL1::IRQ_ON_ALTERNATE_FIELDS;
    pub use super::CTRL1::OVERFLOW_IRQ;
    pub use super::CTRL1::OVERFLOW_IRQ_EN;
    pub use super::CTRL1::RECOVER_ON_UNDERFLOW;
    pub use super::CTRL1::START_INTERLACE_FROM_SECOND_FIELD;
    pub use super::CTRL1::UNDERFLOW_IRQ;
    pub use super::CTRL1::UNDERFLOW_IRQ_EN;
    pub use super::CTRL1::VSYNC_EDGE_IRQ;
    pub use super::CTRL1::VSYNC_EDGE_IRQ_EN;
}

/// LCDIF General Control1 Register
pub mod CTRL1_TOG {
    pub use super::CTRL1::BM_ERROR_IRQ;
    pub use super::CTRL1::BM_ERROR_IRQ_EN;
    pub use super::CTRL1::BYTE_PACKING_FORMAT;
    pub use super::CTRL1::CS_OUT_SELECT;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ;
    pub use super::CTRL1::CUR_FRAME_DONE_IRQ_EN;
    pub use super::CTRL1::FIFO_CLEAR;
    pub use super::CTRL1::IMAGE_DATA_SELECT;
    pub use super::CTRL1::INTERLACE_FIELDS;
    pub use super::CTRL1::IRQ_ON_ALTERNATE_FIELDS;
    pub use super::CTRL1::OVERFLOW_IRQ;
    pub use super::CTRL1::OVERFLOW_IRQ_EN;
    pub use super::CTRL1::RECOVER_ON_UNDERFLOW;
    pub use super::CTRL1::START_INTERLACE_FROM_SECOND_FIELD;
    pub use super::CTRL1::UNDERFLOW_IRQ;
    pub use super::CTRL1::UNDERFLOW_IRQ_EN;
    pub use super::CTRL1::VSYNC_EDGE_IRQ;
    pub use super::CTRL1::VSYNC_EDGE_IRQ_EN;
}

/// LCDIF General Control2 Register
pub mod CTRL2 {

    /// This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,
    pub mod EVEN_LINE_PATTERN {
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

            /// 0b000: RGB
            pub const RGB: u32 = 0b000;

            /// 0b001: RBG
            pub const RBG: u32 = 0b001;

            /// 0b010: GBR
            pub const GBR: u32 = 0b010;

            /// 0b011: GRB
            pub const GRB: u32 = 0b011;

            /// 0b100: BRG
            pub const BRG: u32 = 0b100;

            /// 0b101: BGR
            pub const BGR: u32 = 0b101;
        }
    }

    /// This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,
    pub mod ODD_LINE_PATTERN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EVEN_LINE_PATTERN::RW;
    }

    /// By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)
    pub mod BURST_LEN_8 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master
    pub mod OUTSTANDING_REQS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: REQ_1
            pub const REQ_1: u32 = 0b000;

            /// 0b001: REQ_2
            pub const REQ_2: u32 = 0b001;

            /// 0b010: REQ_4
            pub const REQ_4: u32 = 0b010;

            /// 0b011: REQ_8
            pub const REQ_8: u32 = 0b011;

            /// 0b100: REQ_16
            pub const REQ_16: u32 = 0b100;
        }
    }
}

/// LCDIF General Control2 Register
pub mod CTRL2_SET {
    pub use super::CTRL2::BURST_LEN_8;
    pub use super::CTRL2::EVEN_LINE_PATTERN;
    pub use super::CTRL2::ODD_LINE_PATTERN;
    pub use super::CTRL2::OUTSTANDING_REQS;
}

/// LCDIF General Control2 Register
pub mod CTRL2_CLR {
    pub use super::CTRL2::BURST_LEN_8;
    pub use super::CTRL2::EVEN_LINE_PATTERN;
    pub use super::CTRL2::ODD_LINE_PATTERN;
    pub use super::CTRL2::OUTSTANDING_REQS;
}

/// LCDIF General Control2 Register
pub mod CTRL2_TOG {
    pub use super::CTRL2::BURST_LEN_8;
    pub use super::CTRL2::EVEN_LINE_PATTERN;
    pub use super::CTRL2::ODD_LINE_PATTERN;
    pub use super::CTRL2::OUTSTANDING_REQS;
}

/// LCDIF Horizontal and Vertical Valid Data Count Register
pub mod TRANSFER_COUNT {

    /// Total valid data (pixels) in each horizontal line
    pub mod H_COUNT {
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

    /// Number of horizontal lines per frame which contain valid data
    pub mod V_COUNT {
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

/// LCD Interface Current Buffer Address Register
pub mod CUR_BUF {

    /// Address of the current frame being transmitted by LCDIF.
    pub mod ADDR {
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

/// LCD Interface Next Buffer Address Register
pub mod NEXT_BUF {
    pub use super::CUR_BUF::ADDR;
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register0
pub mod VDCTRL0 {

    /// Number of units for which VSYNC signal is active
    pub mod VSYNC_PULSE_WIDTH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line
    pub mod HALF_LINE_MODE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i
    pub mod HALF_LINE {
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

    /// Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles
    pub mod VSYNC_PULSE_WIDTH_UNIT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles
    pub mod VSYNC_PERIOD_UNIT {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Default 0 active low during valid data transfer on each horizontal line.
    pub mod ENABLE_POL {
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

    /// Default is data launched at negative edge of DOTCLK and captured at positive edge
    pub mod DOTCLK_POL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period
    pub mod HSYNC_POL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period
    pub mod VSYNC_POL {
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

    /// Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK
    pub mod ENABLE_PRESENT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register0
pub mod VDCTRL0_SET {
    pub use super::VDCTRL0::DOTCLK_POL;
    pub use super::VDCTRL0::ENABLE_POL;
    pub use super::VDCTRL0::ENABLE_PRESENT;
    pub use super::VDCTRL0::HALF_LINE;
    pub use super::VDCTRL0::HALF_LINE_MODE;
    pub use super::VDCTRL0::HSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PERIOD_UNIT;
    pub use super::VDCTRL0::VSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH_UNIT;
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register0
pub mod VDCTRL0_CLR {
    pub use super::VDCTRL0::DOTCLK_POL;
    pub use super::VDCTRL0::ENABLE_POL;
    pub use super::VDCTRL0::ENABLE_PRESENT;
    pub use super::VDCTRL0::HALF_LINE;
    pub use super::VDCTRL0::HALF_LINE_MODE;
    pub use super::VDCTRL0::HSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PERIOD_UNIT;
    pub use super::VDCTRL0::VSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH_UNIT;
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register0
pub mod VDCTRL0_TOG {
    pub use super::VDCTRL0::DOTCLK_POL;
    pub use super::VDCTRL0::ENABLE_POL;
    pub use super::VDCTRL0::ENABLE_PRESENT;
    pub use super::VDCTRL0::HALF_LINE;
    pub use super::VDCTRL0::HALF_LINE_MODE;
    pub use super::VDCTRL0::HSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PERIOD_UNIT;
    pub use super::VDCTRL0::VSYNC_POL;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH;
    pub use super::VDCTRL0::VSYNC_PULSE_WIDTH_UNIT;
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register1
pub mod VDCTRL1 {

    /// Total number of units between two positive or two negative edges of the VSYNC signal
    pub mod VSYNC_PERIOD {
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

/// LCDIF VSYNC Mode and Dotclk Mode Control Register2
pub mod VDCTRL2 {

    /// Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal
    pub mod HSYNC_PERIOD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active.
    pub mod HSYNC_PULSE_WIDTH {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (14 bits: 0x3fff << 18)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register3
pub mod VDCTRL3 {

    /// In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set
    pub mod VERTICAL_WAIT_CNT {
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

    /// In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins
    pub mod HORIZONTAL_WAIT_CNT {
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

    /// This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation.
    pub mod VSYNC_ONLY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins
    pub mod MUX_SYNC_SIGNALS {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LCDIF VSYNC Mode and Dotclk Mode Control Register4
pub mod VDCTRL4 {

    /// Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode
    pub mod DOTCLK_H_VALID_DATA_CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end
    pub mod SYNC_SIGNALS_ON {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin
    pub mod DOTCLK_DLY_SEL {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Bus Master Error Status Register
pub mod BM_ERROR_STAT {
    pub use super::CUR_BUF::ADDR;
}

/// CRC Status Register
pub mod CRC_STAT {

    /// Calculated CRC value.
    pub mod CRC_VALUE {
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

/// LCD Interface Status Register
pub mod STAT {

    /// Read only view of the current count in Latency buffer (LFIFO).
    pub mod LFIFO_COUNT {
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

    /// Read only view of the signals that indicates LCD TXFIFO is empty.
    pub mod TXFIFO_EMPTY {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read only view of the signals that indicates LCD TXFIFO is full.
    pub mod TXFIFO_FULL {
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

    /// Read only view of the signals that indicates LCD LFIFO is empty.
    pub mod LFIFO_EMPTY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read only view of the signals that indicates LCD LFIFO is full.
    pub mod LFIFO_FULL {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects the current state of the DMA Request line for the LCDIF
    pub mod DMA_REQ {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 0: LCDIF not present on this product 1: LCDIF is present.
    pub mod PRESENT {
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

/// LCDIF Pigeon Mode Control0 Register
pub mod PIGEONCTRL0 {

    /// Period of line counter during FD phase
    pub mod FD_PERIOD {
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

    /// Period of pclk counter during LD phase
    pub mod LD_PERIOD {
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

/// LCDIF Pigeon Mode Control0 Register
pub mod PIGEONCTRL0_SET {
    pub use super::PIGEONCTRL0::FD_PERIOD;
    pub use super::PIGEONCTRL0::LD_PERIOD;
}

/// LCDIF Pigeon Mode Control0 Register
pub mod PIGEONCTRL0_CLR {
    pub use super::PIGEONCTRL0::FD_PERIOD;
    pub use super::PIGEONCTRL0::LD_PERIOD;
}

/// LCDIF Pigeon Mode Control0 Register
pub mod PIGEONCTRL0_TOG {
    pub use super::PIGEONCTRL0::FD_PERIOD;
    pub use super::PIGEONCTRL0::LD_PERIOD;
}

/// LCDIF Pigeon Mode Control1 Register
pub mod PIGEONCTRL1 {

    /// Period of frame counter
    pub mod FRAME_CNT_PERIOD {
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

    /// Max cycles of frame counter
    pub mod FRAME_CNT_CYCLES {
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

/// LCDIF Pigeon Mode Control1 Register
pub mod PIGEONCTRL1_SET {
    pub use super::PIGEONCTRL1::FRAME_CNT_CYCLES;
    pub use super::PIGEONCTRL1::FRAME_CNT_PERIOD;
}

/// LCDIF Pigeon Mode Control1 Register
pub mod PIGEONCTRL1_CLR {
    pub use super::PIGEONCTRL1::FRAME_CNT_CYCLES;
    pub use super::PIGEONCTRL1::FRAME_CNT_PERIOD;
}

/// LCDIF Pigeon Mode Control1 Register
pub mod PIGEONCTRL1_TOG {
    pub use super::PIGEONCTRL1::FRAME_CNT_CYCLES;
    pub use super::PIGEONCTRL1::FRAME_CNT_PERIOD;
}

/// LCDIF Pigeon Mode Control2 Register
pub mod PIGEONCTRL2 {

    /// Pigeon mode data enable
    pub mod PIGEON_DATA_EN {
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

    /// Pigeon mode dot clock gate enable
    pub mod PIGEON_CLK_GATE {
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
}

/// LCDIF Pigeon Mode Control2 Register
pub mod PIGEONCTRL2_SET {
    pub use super::PIGEONCTRL2::PIGEON_CLK_GATE;
    pub use super::PIGEONCTRL2::PIGEON_DATA_EN;
}

/// LCDIF Pigeon Mode Control2 Register
pub mod PIGEONCTRL2_CLR {
    pub use super::PIGEONCTRL2::PIGEON_CLK_GATE;
    pub use super::PIGEONCTRL2::PIGEON_DATA_EN;
}

/// LCDIF Pigeon Mode Control2 Register
pub mod PIGEONCTRL2_TOG {
    pub use super::PIGEONCTRL2::PIGEON_CLK_GATE;
    pub use super::PIGEONCTRL2::PIGEON_DATA_EN;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_0_0 {

    /// Enable pigeon Mode on this signal
    pub mod EN {
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

    /// Polarity of signal output
    pub mod POL {
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

            /// 0b0: Normal Signal (Active high)
            pub const ACTIVE_HIGH: u32 = 0b0;

            /// 0b1: Inverted signal (Active low)
            pub const ACTIVE_LOW: u32 = 0b1;
        }
    }

    /// Event to incrment local counter
    pub mod INC_SEL {
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

            /// 0b00: pclk
            pub const PCLK: u32 = 0b00;

            /// 0b01: Line start pulse
            pub const LINE: u32 = 0b01;

            /// 0b10: Frame start pulse
            pub const FRAME: u32 = 0b10;

            /// 0b11: Use another signal as tick event
            pub const SIG_ANOTHER: u32 = 0b11;
        }
    }

    /// offset on pclk unit
    pub mod OFFSET {
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

    /// select global counters as mask condition, use together with MASK_CNT
    pub mod MASK_CNT_SEL {
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

            /// 0b0000: pclk counter within one hscan state
            pub const HSTATE_CNT: u32 = 0b0000;

            /// 0b0001: pclk cycle within one hscan state
            pub const HSTATE_CYCLE: u32 = 0b0001;

            /// 0b0010: line counter within one vscan state
            pub const VSTATE_CNT: u32 = 0b0010;

            /// 0b0011: line cycle within one vscan state
            pub const VSTATE_CYCLE: u32 = 0b0011;

            /// 0b0100: frame counter
            pub const FRAME_CNT: u32 = 0b0100;

            /// 0b0101: frame cycle
            pub const FRAME_CYCLE: u32 = 0b0101;

            /// 0b0110: horizontal counter (pclk counter within one line )
            pub const HCNT: u32 = 0b0110;

            /// 0b0111: vertical counter (line counter within one frame)
            pub const VCNT: u32 = 0b0111;
        }
    }

    /// When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking
    pub mod MASK_CNT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (12 bits: 0xfff << 12)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking
    pub mod STATE_MASK {
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

            /// 0b00000001: FRAME SYNC
            pub const FS: u32 = 0b00000001;

            /// 0b00000010: FRAME BEGIN
            pub const FB: u32 = 0b00000010;

            /// 0b00000100: FRAME DATA
            pub const FD: u32 = 0b00000100;

            /// 0b00001000: FRAME END
            pub const FE: u32 = 0b00001000;

            /// 0b00010000: LINE SYNC
            pub const LS: u32 = 0b00010000;

            /// 0b00100000: LINE BEGIN
            pub const LB: u32 = 0b00100000;

            /// 0b01000000: LINE DATA
            pub const LD: u32 = 0b01000000;

            /// 0b10000000: LINE END
            pub const LE: u32 = 0b10000000;
        }
    }
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_0_1 {

    /// Assert signal output when counter match this value
    pub mod SET_CNT {
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

            /// 0b0000000000000000: Start as active
            pub const START_ACTIVE: u32 = 0b0000000000000000;
        }
    }

    /// Deassert signal output when counter match this value
    pub mod CLR_CNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000000: Keep active until mask off
            pub const CLEAR_USING_MASK: u32 = 0b0000000000000000;
        }
    }
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_0_2 {

    /// Logic operation with another signal: DIS/AND/OR/COND
    pub mod SIG_LOGIC {
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

            /// 0b0000: No logic operation
            pub const DIS: u32 = 0b0000;

            /// 0b0001: sigout = sig_another AND this_sig
            pub const AND: u32 = 0b0001;

            /// 0b0010: sigout = sig_another OR this_sig
            pub const OR: u32 = 0b0010;

            /// 0b0011: mask = sig_another AND other_masks
            pub const MASK: u32 = 0b0011;
        }
    }

    /// Select another signal for logic operation or as mask or counter tick event
    pub mod SIG_ANOTHER {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (5 bits: 0b11111 << 4)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Keep active until mask off
            pub const CLEAR_USING_MASK: u32 = 0b00000;
        }
    }
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_1_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_1_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_1_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_2_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_2_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_2_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_3_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_3_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_3_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_4_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_4_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_4_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_5_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_5_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_5_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_6_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_6_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_6_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_7_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_7_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_7_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_8_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_8_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_8_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_9_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_9_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_9_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_10_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_10_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_10_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_11_0 {
    pub use super::PIGEON_0_0::EN;
    pub use super::PIGEON_0_0::INC_SEL;
    pub use super::PIGEON_0_0::MASK_CNT;
    pub use super::PIGEON_0_0::MASK_CNT_SEL;
    pub use super::PIGEON_0_0::OFFSET;
    pub use super::PIGEON_0_0::POL;
    pub use super::PIGEON_0_0::STATE_MASK;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_11_1 {
    pub use super::PIGEON_0_1::CLR_CNT;
    pub use super::PIGEON_0_1::SET_CNT;
}

/// Panel Interface Signal Generator Register
pub mod PIGEON_11_2 {
    pub use super::PIGEON_0_2::SIG_ANOTHER;
    pub use super::PIGEON_0_2::SIG_LOGIC;
}

/// Lookup Table Data Register.
pub mod LUT_CTRL {

    /// Setting this bit will bypass the LUT memory resource completely
    pub mod LUT_BYPASS {
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
}

/// Lookup Table Control Register.
pub mod LUT0_ADDR {

    /// LUT indexed address pointer
    pub mod ADDR {
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

/// Lookup Table Data Register.
pub mod LUT0_DATA {

    /// Writing this field will load 4 bytes, aligned to four byte boundaries, of data indexed by the ADDR field of the REG_LUT_CTRL register
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

/// Lookup Table Control Register.
pub mod LUT1_ADDR {
    pub use super::LUT0_ADDR::ADDR;
}

/// Lookup Table Data Register.
pub mod LUT1_DATA {
    pub use super::LUT0_DATA::DATA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// LCDIF General Control Register
    pub CTRL: RWRegister<u32>,

    /// LCDIF General Control Register
    pub CTRL_SET: RWRegister<u32>,

    /// LCDIF General Control Register
    pub CTRL_CLR: RWRegister<u32>,

    /// LCDIF General Control Register
    pub CTRL_TOG: RWRegister<u32>,

    /// LCDIF General Control1 Register
    pub CTRL1: RWRegister<u32>,

    /// LCDIF General Control1 Register
    pub CTRL1_SET: RWRegister<u32>,

    /// LCDIF General Control1 Register
    pub CTRL1_CLR: RWRegister<u32>,

    /// LCDIF General Control1 Register
    pub CTRL1_TOG: RWRegister<u32>,

    /// LCDIF General Control2 Register
    pub CTRL2: RWRegister<u32>,

    /// LCDIF General Control2 Register
    pub CTRL2_SET: RWRegister<u32>,

    /// LCDIF General Control2 Register
    pub CTRL2_CLR: RWRegister<u32>,

    /// LCDIF General Control2 Register
    pub CTRL2_TOG: RWRegister<u32>,

    /// LCDIF Horizontal and Vertical Valid Data Count Register
    pub TRANSFER_COUNT: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// LCD Interface Current Buffer Address Register
    pub CUR_BUF: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// LCD Interface Next Buffer Address Register
    pub NEXT_BUF: RWRegister<u32>,

    _reserved3: [u32; 7],

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register0
    pub VDCTRL0: RWRegister<u32>,

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register0
    pub VDCTRL0_SET: RWRegister<u32>,

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register0
    pub VDCTRL0_CLR: RWRegister<u32>,

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register0
    pub VDCTRL0_TOG: RWRegister<u32>,

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register1
    pub VDCTRL1: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register2
    pub VDCTRL2: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register3
    pub VDCTRL3: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// LCDIF VSYNC Mode and Dotclk Mode Control Register4
    pub VDCTRL4: RWRegister<u32>,

    _reserved7: [u32; 55],

    /// Bus Master Error Status Register
    pub BM_ERROR_STAT: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// CRC Status Register
    pub CRC_STAT: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// LCD Interface Status Register
    pub STAT: RORegister<u32>,

    _reserved10: [u32; 115],

    /// LCDIF Pigeon Mode Control0 Register
    pub PIGEONCTRL0: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control0 Register
    pub PIGEONCTRL0_SET: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control0 Register
    pub PIGEONCTRL0_CLR: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control0 Register
    pub PIGEONCTRL0_TOG: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control1 Register
    pub PIGEONCTRL1: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control1 Register
    pub PIGEONCTRL1_SET: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control1 Register
    pub PIGEONCTRL1_CLR: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control1 Register
    pub PIGEONCTRL1_TOG: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control2 Register
    pub PIGEONCTRL2: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control2 Register
    pub PIGEONCTRL2_SET: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control2 Register
    pub PIGEONCTRL2_CLR: RWRegister<u32>,

    /// LCDIF Pigeon Mode Control2 Register
    pub PIGEONCTRL2_TOG: RWRegister<u32>,

    _reserved11: [u32; 276],

    /// Panel Interface Signal Generator Register
    pub PIGEON_0_0: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_0_1: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_0_2: RWRegister<u32>,

    _reserved14: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_1_0: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_1_1: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_1_2: RWRegister<u32>,

    _reserved17: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_2_0: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_2_1: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_2_2: RWRegister<u32>,

    _reserved20: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_3_0: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_3_1: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_3_2: RWRegister<u32>,

    _reserved23: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_4_0: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_4_1: RWRegister<u32>,

    _reserved25: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_4_2: RWRegister<u32>,

    _reserved26: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_5_0: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_5_1: RWRegister<u32>,

    _reserved28: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_5_2: RWRegister<u32>,

    _reserved29: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_6_0: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_6_1: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_6_2: RWRegister<u32>,

    _reserved32: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_7_0: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_7_1: RWRegister<u32>,

    _reserved34: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_7_2: RWRegister<u32>,

    _reserved35: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_8_0: RWRegister<u32>,

    _reserved36: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_8_1: RWRegister<u32>,

    _reserved37: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_8_2: RWRegister<u32>,

    _reserved38: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_9_0: RWRegister<u32>,

    _reserved39: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_9_1: RWRegister<u32>,

    _reserved40: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_9_2: RWRegister<u32>,

    _reserved41: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_10_0: RWRegister<u32>,

    _reserved42: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_10_1: RWRegister<u32>,

    _reserved43: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_10_2: RWRegister<u32>,

    _reserved44: [u32; 7],

    /// Panel Interface Signal Generator Register
    pub PIGEON_11_0: RWRegister<u32>,

    _reserved45: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_11_1: RWRegister<u32>,

    _reserved46: [u32; 3],

    /// Panel Interface Signal Generator Register
    pub PIGEON_11_2: RWRegister<u32>,

    _reserved47: [u32; 7],

    /// Lookup Table Data Register.
    pub LUT_CTRL: RWRegister<u32>,

    _reserved48: [u32; 3],

    /// Lookup Table Control Register.
    pub LUT0_ADDR: RWRegister<u32>,

    _reserved49: [u32; 3],

    /// Lookup Table Data Register.
    pub LUT0_DATA: RWRegister<u32>,

    _reserved50: [u32; 3],

    /// Lookup Table Control Register.
    pub LUT1_ADDR: RWRegister<u32>,

    _reserved51: [u32; 3],

    /// Lookup Table Data Register.
    pub LUT1_DATA: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub CTRL1: u32,
    pub CTRL1_SET: u32,
    pub CTRL1_CLR: u32,
    pub CTRL1_TOG: u32,
    pub CTRL2: u32,
    pub CTRL2_SET: u32,
    pub CTRL2_CLR: u32,
    pub CTRL2_TOG: u32,
    pub TRANSFER_COUNT: u32,
    pub CUR_BUF: u32,
    pub NEXT_BUF: u32,
    pub VDCTRL0: u32,
    pub VDCTRL0_SET: u32,
    pub VDCTRL0_CLR: u32,
    pub VDCTRL0_TOG: u32,
    pub VDCTRL1: u32,
    pub VDCTRL2: u32,
    pub VDCTRL3: u32,
    pub VDCTRL4: u32,
    pub BM_ERROR_STAT: u32,
    pub CRC_STAT: u32,
    pub STAT: u32,
    pub PIGEONCTRL0: u32,
    pub PIGEONCTRL0_SET: u32,
    pub PIGEONCTRL0_CLR: u32,
    pub PIGEONCTRL0_TOG: u32,
    pub PIGEONCTRL1: u32,
    pub PIGEONCTRL1_SET: u32,
    pub PIGEONCTRL1_CLR: u32,
    pub PIGEONCTRL1_TOG: u32,
    pub PIGEONCTRL2: u32,
    pub PIGEONCTRL2_SET: u32,
    pub PIGEONCTRL2_CLR: u32,
    pub PIGEONCTRL2_TOG: u32,
    pub PIGEON_0_0: u32,
    pub PIGEON_0_1: u32,
    pub PIGEON_0_2: u32,
    pub PIGEON_1_0: u32,
    pub PIGEON_1_1: u32,
    pub PIGEON_1_2: u32,
    pub PIGEON_2_0: u32,
    pub PIGEON_2_1: u32,
    pub PIGEON_2_2: u32,
    pub PIGEON_3_0: u32,
    pub PIGEON_3_1: u32,
    pub PIGEON_3_2: u32,
    pub PIGEON_4_0: u32,
    pub PIGEON_4_1: u32,
    pub PIGEON_4_2: u32,
    pub PIGEON_5_0: u32,
    pub PIGEON_5_1: u32,
    pub PIGEON_5_2: u32,
    pub PIGEON_6_0: u32,
    pub PIGEON_6_1: u32,
    pub PIGEON_6_2: u32,
    pub PIGEON_7_0: u32,
    pub PIGEON_7_1: u32,
    pub PIGEON_7_2: u32,
    pub PIGEON_8_0: u32,
    pub PIGEON_8_1: u32,
    pub PIGEON_8_2: u32,
    pub PIGEON_9_0: u32,
    pub PIGEON_9_1: u32,
    pub PIGEON_9_2: u32,
    pub PIGEON_10_0: u32,
    pub PIGEON_10_1: u32,
    pub PIGEON_10_2: u32,
    pub PIGEON_11_0: u32,
    pub PIGEON_11_1: u32,
    pub PIGEON_11_2: u32,
    pub LUT_CTRL: u32,
    pub LUT0_ADDR: u32,
    pub LUT0_DATA: u32,
    pub LUT1_ADDR: u32,
    pub LUT1_DATA: u32,
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
