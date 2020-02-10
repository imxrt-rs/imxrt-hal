#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PXP v2.0 Register Reference Index
//!
//! Used by: imxrt1062, imxrt1064

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control Register 0
pub mod CTRL {

    /// Enables PXP operation with specified parameters
    pub mod ENABLE {
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

    /// Interrupt enable
    pub mod IRQ_ENABLE {
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

    /// Next command interrupt enable
    pub mod NEXT_IRQ_ENABLE {
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

    /// Enable handshake with LCD controller
    pub mod ENABLE_LCD_HANDSHAKE {
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

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the clockwise rotation to be applied at the output buffer
    pub mod ROTATE {
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

            /// 0b00: ROT_0
            pub const ROT_0: u32 = 0b00;

            /// 0b01: ROT_90
            pub const ROT_90: u32 = 0b01;

            /// 0b10: ROT_180
            pub const ROT_180: u32 = 0b10;

            /// 0b11: ROT_270
            pub const ROT_270: u32 = 0b11;
        }
    }

    /// Indicates that the output buffer should be flipped horizontally (effect applied before rotation).
    pub mod HFLIP {
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

    /// Indicates that the output buffer should be flipped vertically (effect applied before rotation).
    pub mod VFLIP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (10 bits: 0x3ff << 12)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit controls where rotation will occur in the PXP datapath
    pub mod ROT_POS {
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

    /// Select the block size to process.
    pub mod BLOCK_SIZE {
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

            /// 0b0: Process 8x8 pixel blocks.
            pub const _8X8: u32 = 0b0;

            /// 0b1: Process 16x16 pixel blocks.
            pub const _16X16: u32 = 0b1;
        }
    }

    /// Reserved, always set to zero.
    pub mod RSVD3 {
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

    /// Enable the PXP to run continuously
    pub mod EN_REPEAT {
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

    /// Reserved, always set to zero.
    pub mod RSVD4 {
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

    /// Set this bit to zero to enable normal PXP operation
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

/// Control Register 0
pub mod CTRL_SET {
    pub use super::CTRL::BLOCK_SIZE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE;
    pub use super::CTRL::ENABLE_LCD_HANDSHAKE;
    pub use super::CTRL::EN_REPEAT;
    pub use super::CTRL::HFLIP;
    pub use super::CTRL::IRQ_ENABLE;
    pub use super::CTRL::NEXT_IRQ_ENABLE;
    pub use super::CTRL::ROTATE;
    pub use super::CTRL::ROT_POS;
    pub use super::CTRL::RSVD0;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::RSVD3;
    pub use super::CTRL::RSVD4;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::VFLIP;
}

/// Control Register 0
pub mod CTRL_CLR {
    pub use super::CTRL::BLOCK_SIZE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE;
    pub use super::CTRL::ENABLE_LCD_HANDSHAKE;
    pub use super::CTRL::EN_REPEAT;
    pub use super::CTRL::HFLIP;
    pub use super::CTRL::IRQ_ENABLE;
    pub use super::CTRL::NEXT_IRQ_ENABLE;
    pub use super::CTRL::ROTATE;
    pub use super::CTRL::ROT_POS;
    pub use super::CTRL::RSVD0;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::RSVD3;
    pub use super::CTRL::RSVD4;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::VFLIP;
}

/// Control Register 0
pub mod CTRL_TOG {
    pub use super::CTRL::BLOCK_SIZE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE;
    pub use super::CTRL::ENABLE_LCD_HANDSHAKE;
    pub use super::CTRL::EN_REPEAT;
    pub use super::CTRL::HFLIP;
    pub use super::CTRL::IRQ_ENABLE;
    pub use super::CTRL::NEXT_IRQ_ENABLE;
    pub use super::CTRL::ROTATE;
    pub use super::CTRL::ROT_POS;
    pub use super::CTRL::RSVD0;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::RSVD3;
    pub use super::CTRL::RSVD4;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::VFLIP;
}

/// Status Register
pub mod STAT {

    /// Indicates current PXP interrupt status
    pub mod IRQ {
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

    /// Indicates PXP encountered an AXI write error and processing has been terminated.
    pub mod AXI_WRITE_ERROR {
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

    /// Indicates PXP encountered an AXI read error and processing has been terminated.
    pub mod AXI_READ_ERROR {
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

    /// Indicates that a command issued with the "Next Command" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register
    pub mod NEXT_IRQ {
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

    /// Indicates the AXI ID of the failing bus operation.
    pub mod AXI_ERROR_ID {
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

    /// Indicates that the LUT DMA transfer has completed.
    pub mod LUT_DMA_LOAD_DONE_IRQ {
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

    /// Reserved, always set to zero.
    pub mod RSVD2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (7 bits: 0x7f << 9)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the X coordinate of the block currently being rendered.
    pub mod BLOCKY {
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

    /// Indicates the X coordinate of the block currently being rendered.
    pub mod BLOCKX {
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

/// Status Register
pub mod STAT_SET {
    pub use super::STAT::AXI_ERROR_ID;
    pub use super::STAT::AXI_READ_ERROR;
    pub use super::STAT::AXI_WRITE_ERROR;
    pub use super::STAT::BLOCKX;
    pub use super::STAT::BLOCKY;
    pub use super::STAT::IRQ;
    pub use super::STAT::LUT_DMA_LOAD_DONE_IRQ;
    pub use super::STAT::NEXT_IRQ;
    pub use super::STAT::RSVD2;
}

/// Status Register
pub mod STAT_CLR {
    pub use super::STAT::AXI_ERROR_ID;
    pub use super::STAT::AXI_READ_ERROR;
    pub use super::STAT::AXI_WRITE_ERROR;
    pub use super::STAT::BLOCKX;
    pub use super::STAT::BLOCKY;
    pub use super::STAT::IRQ;
    pub use super::STAT::LUT_DMA_LOAD_DONE_IRQ;
    pub use super::STAT::NEXT_IRQ;
    pub use super::STAT::RSVD2;
}

/// Status Register
pub mod STAT_TOG {
    pub use super::STAT::AXI_ERROR_ID;
    pub use super::STAT::AXI_READ_ERROR;
    pub use super::STAT::AXI_WRITE_ERROR;
    pub use super::STAT::BLOCKX;
    pub use super::STAT::BLOCKY;
    pub use super::STAT::IRQ;
    pub use super::STAT::LUT_DMA_LOAD_DONE_IRQ;
    pub use super::STAT::NEXT_IRQ;
    pub use super::STAT::RSVD2;
}

/// Output Buffer Control Register
pub mod OUT_CTRL {

    /// Output framebuffer format
    pub mod FORMAT {
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

            /// 0b00000: 32-bit pixels
            pub const ARGB8888: u32 = 0b00000;

            /// 0b00100: 32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)
            pub const RGB888: u32 = 0b00100;

            /// 0b00101: 24-bit pixels (packed 24-bit format)
            pub const RGB888P: u32 = 0b00101;

            /// 0b01000: 16-bit pixels
            pub const ARGB1555: u32 = 0b01000;

            /// 0b01001: 16-bit pixels
            pub const ARGB4444: u32 = 0b01001;

            /// 0b01100: 16-bit pixels
            pub const RGB555: u32 = 0b01100;

            /// 0b01101: 16-bit pixels
            pub const RGB444: u32 = 0b01101;

            /// 0b01110: 16-bit pixels
            pub const RGB565: u32 = 0b01110;

            /// 0b10000: 32-bit pixels (1-plane XYUV unpacked)
            pub const YUV1P444: u32 = 0b10000;

            /// 0b10010: 16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)
            pub const UYVY1P422: u32 = 0b10010;

            /// 0b10011: 16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)
            pub const VYUY1P422: u32 = 0b10011;

            /// 0b10100: 8-bit monochrome pixels (1-plane Y luma output)
            pub const Y8: u32 = 0b10100;

            /// 0b10101: 4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)
            pub const Y4: u32 = 0b10101;

            /// 0b11000: 16-bit pixels (2-plane UV interleaved bytes)
            pub const YUV2P422: u32 = 0b11000;

            /// 0b11001: 16-bit pixels (2-plane UV)
            pub const YUV2P420: u32 = 0b11001;

            /// 0b11010: 16-bit pixels (2-plane VU interleaved bytes)
            pub const YVU2P422: u32 = 0b11010;

            /// 0b11011: 16-bit pixels (2-plane VU)
            pub const YVU2P420: u32 = 0b11011;
        }
    }

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Determines how the PXP writes it's output data
    pub mod INTERLACED_OUTPUT {
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

            /// 0b00: All data written in progressive format to the OUTBUF Pointer.
            pub const PROGRESSIVE: u32 = 0b00;

            /// 0b01: Interlaced output: only data for field 0 is written to the OUTBUF Pointer.
            pub const FIELD0: u32 = 0b01;

            /// 0b10: Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer.
            pub const FIELD1: u32 = 0b10;

            /// 0b11: Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2.
            pub const INTERLACED: u32 = 0b11;
        }
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (13 bits: 0x1fff << 10)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\[ALPHA\]
    pub mod ALPHA_OUTPUT {
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

    /// When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline
    pub mod ALPHA {
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

/// Output Buffer Control Register
pub mod OUT_CTRL_SET {
    pub use super::OUT_CTRL::ALPHA;
    pub use super::OUT_CTRL::ALPHA_OUTPUT;
    pub use super::OUT_CTRL::FORMAT;
    pub use super::OUT_CTRL::INTERLACED_OUTPUT;
    pub use super::OUT_CTRL::RSVD0;
    pub use super::OUT_CTRL::RSVD1;
}

/// Output Buffer Control Register
pub mod OUT_CTRL_CLR {
    pub use super::OUT_CTRL::ALPHA;
    pub use super::OUT_CTRL::ALPHA_OUTPUT;
    pub use super::OUT_CTRL::FORMAT;
    pub use super::OUT_CTRL::INTERLACED_OUTPUT;
    pub use super::OUT_CTRL::RSVD0;
    pub use super::OUT_CTRL::RSVD1;
}

/// Output Buffer Control Register
pub mod OUT_CTRL_TOG {
    pub use super::OUT_CTRL::ALPHA;
    pub use super::OUT_CTRL::ALPHA_OUTPUT;
    pub use super::OUT_CTRL::FORMAT;
    pub use super::OUT_CTRL::INTERLACED_OUTPUT;
    pub use super::OUT_CTRL::RSVD0;
    pub use super::OUT_CTRL::RSVD1;
}

/// Output Frame Buffer Pointer
pub mod OUT_BUF {

    /// Current address pointer for the output frame buffer
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

/// Output Frame Buffer Pointer #2
pub mod OUT_BUF2 {
    pub use super::OUT_BUF::ADDR;
}

/// Output Buffer Pitch
pub mod OUT_PITCH {

    /// Indicates the number of bytes in memory between two vertically adjacent pixels.
    pub mod PITCH {
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

    /// Reserved, always set to zero.
    pub mod RSVD {
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

/// Output Surface Lower Right Coordinate
pub mod OUT_LRC {

    /// Indicates the number of vertical PIXELS in the output surface (non-rotated)
    pub mod Y {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates number of horizontal PIXELS in the output surface (non-rotated)
    pub mod X {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

/// Processed Surface Upper Left Coordinate
pub mod OUT_PS_ULC {
    pub use super::OUT_LRC::RSVD0;
    pub use super::OUT_LRC::RSVD1;
    pub use super::OUT_LRC::X;
    pub use super::OUT_LRC::Y;
}

/// Processed Surface Lower Right Coordinate
pub mod OUT_PS_LRC {
    pub use super::OUT_LRC::RSVD0;
    pub use super::OUT_LRC::RSVD1;
    pub use super::OUT_LRC::X;
    pub use super::OUT_LRC::Y;
}

/// Alpha Surface Upper Left Coordinate
pub mod OUT_AS_ULC {
    pub use super::OUT_LRC::RSVD0;
    pub use super::OUT_LRC::RSVD1;
    pub use super::OUT_LRC::X;
    pub use super::OUT_LRC::Y;
}

/// Alpha Surface Lower Right Coordinate
pub mod OUT_AS_LRC {
    pub use super::OUT_LRC::RSVD0;
    pub use super::OUT_LRC::RSVD1;
    pub use super::OUT_LRC::X;
    pub use super::OUT_LRC::Y;
}

/// Processed Surface (PS) Control Register
pub mod PS_CTRL {

    /// PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register.
    pub mod FORMAT {
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

            /// 0b00100: 32-bit pixels (unpacked 24-bit format)
            pub const RGB888: u32 = 0b00100;

            /// 0b01100: 16-bit pixels
            pub const RGB555: u32 = 0b01100;

            /// 0b01101: 16-bit pixels
            pub const RGB444: u32 = 0b01101;

            /// 0b01110: 16-bit pixels
            pub const RGB565: u32 = 0b01110;

            /// 0b10000: 32-bit pixels (1-plane XYUV unpacked)
            pub const YUV1P444: u32 = 0b10000;

            /// 0b10010: 16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)
            pub const UYVY1P422: u32 = 0b10010;

            /// 0b10011: 16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)
            pub const VYUY1P422: u32 = 0b10011;

            /// 0b10100: 8-bit monochrome pixels (1-plane Y luma output)
            pub const Y8: u32 = 0b10100;

            /// 0b10101: 4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)
            pub const Y4: u32 = 0b10101;

            /// 0b11000: 16-bit pixels (2-plane UV interleaved bytes)
            pub const YUV2P422: u32 = 0b11000;

            /// 0b11001: 16-bit pixels (2-plane UV)
            pub const YUV2P420: u32 = 0b11001;

            /// 0b11010: 16-bit pixels (2-plane VU interleaved bytes)
            pub const YVU2P422: u32 = 0b11010;

            /// 0b11011: 16-bit pixels (2-plane VU)
            pub const YVU2P420: u32 = 0b11011;

            /// 0b11110: 16-bit pixels (3-plane format)
            pub const YUV422: u32 = 0b11110;

            /// 0b11111: 16-bit pixels (3-plane format)
            pub const YUV420: u32 = 0b11111;
        }
    }

    /// Swap bytes in words. For each 16 bit word, the two bytes will be swapped.
    pub mod WB_SWAP {
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

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Verticle pre decimation filter control.
    pub mod DECY {
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

            /// 0b00: Disable pre-decimation filter.
            pub const DISABLE: u32 = 0b00;

            /// 0b01: Decimate PS by 2.
            pub const DECY2: u32 = 0b01;

            /// 0b10: Decimate PS by 4.
            pub const DECY4: u32 = 0b10;

            /// 0b11: Decimate PS by 8.
            pub const DECY8: u32 = 0b11;
        }
    }

    /// Horizontal pre decimation filter control.
    pub mod DECX {
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

            /// 0b00: Disable pre-decimation filter.
            pub const DISABLE: u32 = 0b00;

            /// 0b01: Decimate PS by 2.
            pub const DECX2: u32 = 0b01;

            /// 0b10: Decimate PS by 4.
            pub const DECX4: u32 = 0b10;

            /// 0b11: Decimate PS by 8.
            pub const DECX8: u32 = 0b11;
        }
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

/// Processed Surface (PS) Control Register
pub mod PS_CTRL_SET {
    pub use super::PS_CTRL::DECX;
    pub use super::PS_CTRL::DECY;
    pub use super::PS_CTRL::FORMAT;
    pub use super::PS_CTRL::RSVD0;
    pub use super::PS_CTRL::RSVD1;
    pub use super::PS_CTRL::WB_SWAP;
}

/// Processed Surface (PS) Control Register
pub mod PS_CTRL_CLR {
    pub use super::PS_CTRL::DECX;
    pub use super::PS_CTRL::DECY;
    pub use super::PS_CTRL::FORMAT;
    pub use super::PS_CTRL::RSVD0;
    pub use super::PS_CTRL::RSVD1;
    pub use super::PS_CTRL::WB_SWAP;
}

/// Processed Surface (PS) Control Register
pub mod PS_CTRL_TOG {
    pub use super::PS_CTRL::DECX;
    pub use super::PS_CTRL::DECY;
    pub use super::PS_CTRL::FORMAT;
    pub use super::PS_CTRL::RSVD0;
    pub use super::PS_CTRL::RSVD1;
    pub use super::PS_CTRL::WB_SWAP;
}

/// PS Input Buffer Address
pub mod PS_BUF {
    pub use super::OUT_BUF::ADDR;
}

/// PS U/Cb or 2 Plane UV Input Buffer Address
pub mod PS_UBUF {
    pub use super::OUT_BUF::ADDR;
}

/// PS V/Cr Input Buffer Address
pub mod PS_VBUF {
    pub use super::OUT_BUF::ADDR;
}

/// Processed Surface Pitch
pub mod PS_PITCH {
    pub use super::OUT_PITCH::PITCH;
    pub use super::OUT_PITCH::RSVD;
}

/// PS Background Color
pub mod PS_BACKGROUND {

    /// Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC
    pub mod COLOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD {
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

/// PS Scale Factor Register
pub mod PS_SCALE {

    /// This is a two bit integer and 12 bit fractional representation (##
    pub mod XSCALE {
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

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

    /// This is a two bit integer and 12 bit fractional representation (##
    pub mod YSCALE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (15 bits: 0x7fff << 16)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD2 {
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

/// PS Scale Offset Register
pub mod PS_OFFSET {

    /// This is a 12 bit fractional representation (0
    pub mod XOFFSET {
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

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

    /// This is a 12 bit fractional representation (0
    pub mod YOFFSET {
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

    /// Reserved, always set to zero.
    pub mod RSVD2 {
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

/// PS Color Key Low
pub mod PS_CLRKEYLOW {

    /// Low range of color key applied to PS buffer
    pub mod PIXEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

/// PS Color Key High
pub mod PS_CLRKEYHIGH {
    pub use super::PS_CLRKEYLOW::PIXEL;
    pub use super::PS_CLRKEYLOW::RSVD1;
}

/// Alpha Surface Control
pub mod AS_CTRL {

    /// Reserved, always set to zero.
    pub mod RSVD0 {
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

    /// Determines how the alpha value is constructed for this alpha surface
    pub mod ALPHA_CTRL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored.
            pub const Embedded: u32 = 0b00;

            /// 0b01: Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels.
            pub const Override: u32 = 0b01;

            /// 0b10: Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field.
            pub const Multiply: u32 = 0b10;

            /// 0b11: Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels.
            pub const ROPs: u32 = 0b11;
        }
    }

    /// Indicates that colorkey functionality is enabled for this alpha surface
    pub mod ENABLE_COLORKEY {
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

    /// Indicates the input buffer format for AS.
    pub mod FORMAT {
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

            /// 0b0000: 32-bit pixels with alpha
            pub const ARGB8888: u32 = 0b0000;

            /// 0b0100: 32-bit pixels without alpha (unpacked 24-bit format)
            pub const RGB888: u32 = 0b0100;

            /// 0b1000: 16-bit pixels with alpha
            pub const ARGB1555: u32 = 0b1000;

            /// 0b1001: 16-bit pixels with alpha
            pub const ARGB4444: u32 = 0b1001;

            /// 0b1100: 16-bit pixels without alpha
            pub const RGB555: u32 = 0b1100;

            /// 0b1101: 16-bit pixels without alpha
            pub const RGB444: u32 = 0b1101;

            /// 0b1110: 16-bit pixels without alpha
            pub const RGB565: u32 = 0b1110;
        }
    }

    /// Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\[ALPHA_CTRL\]
    pub mod ALPHA {
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

    /// Indicates a raster operation to perform when enabled
    pub mod ROP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AS AND PS
            pub const MASKAS: u32 = 0b0000;

            /// 0b0001: nAS AND PS
            pub const MASKNOTAS: u32 = 0b0001;

            /// 0b0010: AS AND nPS
            pub const MASKASNOT: u32 = 0b0010;

            /// 0b0011: AS OR PS
            pub const MERGEAS: u32 = 0b0011;

            /// 0b0100: nAS OR PS
            pub const MERGENOTAS: u32 = 0b0100;

            /// 0b0101: AS OR nPS
            pub const MERGEASNOT: u32 = 0b0101;

            /// 0b0110: nAS
            pub const NOTCOPYAS: u32 = 0b0110;

            /// 0b0111: nPS
            pub const NOT: u32 = 0b0111;

            /// 0b1000: AS NAND PS
            pub const NOTMASKAS: u32 = 0b1000;

            /// 0b1001: AS NOR PS
            pub const NOTMERGEAS: u32 = 0b1001;

            /// 0b1010: AS XOR PS
            pub const XORAS: u32 = 0b1010;

            /// 0b1011: AS XNOR PS
            pub const NOTXORAS: u32 = 0b1011;
        }
    }

    /// Setting this bit to logic 0 will not alter the alpha value
    pub mod ALPHA_INVERT {
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

    /// Reserved, always set to zero.
    pub mod RSVD1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Alpha Surface Buffer Pointer
pub mod AS_BUF {
    pub use super::OUT_BUF::ADDR;
}

/// Alpha Surface Pitch
pub mod AS_PITCH {
    pub use super::OUT_PITCH::PITCH;
    pub use super::OUT_PITCH::RSVD;
}

/// Overlay Color Key Low
pub mod AS_CLRKEYLOW {
    pub use super::PS_CLRKEYLOW::PIXEL;
    pub use super::PS_CLRKEYLOW::RSVD1;
}

/// Overlay Color Key High
pub mod AS_CLRKEYHIGH {
    pub use super::PS_CLRKEYLOW::PIXEL;
    pub use super::PS_CLRKEYLOW::RSVD1;
}

/// Color Space Conversion Coefficient Register 0
pub mod CSC1_COEF0 {

    /// Two's compliment amplitude offset implicit in the Y data
    pub mod Y_OFFSET {
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

    /// Two's compliment phase offset implicit for CbCr data
    pub mod UV_OFFSET {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (9 bits: 0x1ff << 9)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)
    pub mod C0 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (11 bits: 0x7ff << 18)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
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

    /// Bypass the CSC unit in the scaling engine
    pub mod BYPASS {
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

    /// Set to 1 when performing YCbCr conversion to RGB
    pub mod YCBCR_MODE {
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

/// Color Space Conversion Coefficient Register 1
pub mod CSC1_COEF1 {

    /// Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)
    pub mod C4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)
    pub mod C1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Color Space Conversion Coefficient Register 2
pub mod CSC1_COEF2 {

    /// Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)
    pub mod C3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD0 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)
    pub mod C2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved, always set to zero.
    pub mod RSVD1 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (5 bits: 0b11111 << 27)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PXP Power Control Register
pub mod POWER {

    /// Select the low power state of the ROT memory.
    pub mod ROT_MEM_LP_STATE {
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

            /// 0b000: Memory is not in low power state.
            pub const NONE: u32 = 0b000;

            /// 0b001: Light Sleep Mode. Low leakage mode, maintain memory contents.
            pub const LS: u32 = 0b001;

            /// 0b010: Deep Sleep Mode. Low leakage mode, maintain memory contents.
            pub const DS: u32 = 0b010;

            /// 0b100: Shut Down Mode. Shut Down periphery and core, no memory retention.
            pub const SD: u32 = 0b100;
        }
    }

    /// Power control for the PXP.
    pub mod CTRL {
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

/// Next Frame Pointer
pub mod NEXT {

    /// Indicates that the "next frame" functionality has been enabled
    pub mod ENABLED {
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

    /// Reserved, always set to zero.
    pub mod RSVD {
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

    /// A pointer to a data structure containing register values to be used when processing the next frame
    pub mod POINTER {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PXP Alpha Engine A Control Register.
pub mod PORTER_DUFF_CTRL {

    /// poter_duff enable
    pub mod POTER_DUFF_ENABLE {
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

    /// s0 to s1 factor mode
    pub mod S0_S1_FACTOR_MODE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// s0 global alpha mode
    pub mod S0_GLOBAL_ALPHA_MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// s0 alpha mode
    pub mod S0_ALPHA_MODE {
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

    /// s0 color mode
    pub mod S0_COLOR_MODE {
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

    /// s1 to s0 factor mode
    pub mod S1_S0_FACTOR_MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// s1 global alpha mode
    pub mod S1_GLOBAL_ALPHA_MODE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// s1 alpha mode
    pub mod S1_ALPHA_MODE {
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

    /// s1 color mode
    pub mod S1_COLOR_MODE {
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

    /// s0 global alpha
    pub mod S0_GLOBAL_ALPHA {
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

    /// s1 global alpha
    pub mod S1_GLOBAL_ALPHA {
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
    /// Control Register 0
    pub CTRL: RWRegister<u32>,

    /// Control Register 0
    pub CTRL_SET: RWRegister<u32>,

    /// Control Register 0
    pub CTRL_CLR: RWRegister<u32>,

    /// Control Register 0
    pub CTRL_TOG: RWRegister<u32>,

    /// Status Register
    pub STAT: RWRegister<u32>,

    /// Status Register
    pub STAT_SET: RWRegister<u32>,

    /// Status Register
    pub STAT_CLR: RWRegister<u32>,

    /// Status Register
    pub STAT_TOG: RWRegister<u32>,

    /// Output Buffer Control Register
    pub OUT_CTRL: RWRegister<u32>,

    /// Output Buffer Control Register
    pub OUT_CTRL_SET: RWRegister<u32>,

    /// Output Buffer Control Register
    pub OUT_CTRL_CLR: RWRegister<u32>,

    /// Output Buffer Control Register
    pub OUT_CTRL_TOG: RWRegister<u32>,

    /// Output Frame Buffer Pointer
    pub OUT_BUF: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// Output Frame Buffer Pointer #2
    pub OUT_BUF2: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// Output Buffer Pitch
    pub OUT_PITCH: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Output Surface Lower Right Coordinate
    pub OUT_LRC: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// Processed Surface Upper Left Coordinate
    pub OUT_PS_ULC: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Processed Surface Lower Right Coordinate
    pub OUT_PS_LRC: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Alpha Surface Upper Left Coordinate
    pub OUT_AS_ULC: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// Alpha Surface Lower Right Coordinate
    pub OUT_AS_LRC: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// Processed Surface (PS) Control Register
    pub PS_CTRL: RWRegister<u32>,

    /// Processed Surface (PS) Control Register
    pub PS_CTRL_SET: RWRegister<u32>,

    /// Processed Surface (PS) Control Register
    pub PS_CTRL_CLR: RWRegister<u32>,

    /// Processed Surface (PS) Control Register
    pub PS_CTRL_TOG: RWRegister<u32>,

    /// PS Input Buffer Address
    pub PS_BUF: RWRegister<u32>,

    _reserved9: [u32; 3],

    /// PS U/Cb or 2 Plane UV Input Buffer Address
    pub PS_UBUF: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// PS V/Cr Input Buffer Address
    pub PS_VBUF: RWRegister<u32>,

    _reserved11: [u32; 3],

    /// Processed Surface Pitch
    pub PS_PITCH: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// PS Background Color
    pub PS_BACKGROUND: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// PS Scale Factor Register
    pub PS_SCALE: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// PS Scale Offset Register
    pub PS_OFFSET: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// PS Color Key Low
    pub PS_CLRKEYLOW: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// PS Color Key High
    pub PS_CLRKEYHIGH: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// Alpha Surface Control
    pub AS_CTRL: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// Alpha Surface Buffer Pointer
    pub AS_BUF: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// Alpha Surface Pitch
    pub AS_PITCH: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// Overlay Color Key Low
    pub AS_CLRKEYLOW: RWRegister<u32>,

    _reserved21: [u32; 3],

    /// Overlay Color Key High
    pub AS_CLRKEYHIGH: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// Color Space Conversion Coefficient Register 0
    pub CSC1_COEF0: RWRegister<u32>,

    _reserved23: [u32; 3],

    /// Color Space Conversion Coefficient Register 1
    pub CSC1_COEF1: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// Color Space Conversion Coefficient Register 2
    pub CSC1_COEF2: RWRegister<u32>,

    _reserved25: [u32; 87],

    /// PXP Power Control Register
    pub POWER: RWRegister<u32>,

    _reserved26: [u32; 55],

    /// Next Frame Pointer
    pub NEXT: RWRegister<u32>,

    _reserved27: [u32; 15],

    /// PXP Alpha Engine A Control Register.
    pub PORTER_DUFF_CTRL: RWRegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub STAT: u32,
    pub STAT_SET: u32,
    pub STAT_CLR: u32,
    pub STAT_TOG: u32,
    pub OUT_CTRL: u32,
    pub OUT_CTRL_SET: u32,
    pub OUT_CTRL_CLR: u32,
    pub OUT_CTRL_TOG: u32,
    pub OUT_BUF: u32,
    pub OUT_BUF2: u32,
    pub OUT_PITCH: u32,
    pub OUT_LRC: u32,
    pub OUT_PS_ULC: u32,
    pub OUT_PS_LRC: u32,
    pub OUT_AS_ULC: u32,
    pub OUT_AS_LRC: u32,
    pub PS_CTRL: u32,
    pub PS_CTRL_SET: u32,
    pub PS_CTRL_CLR: u32,
    pub PS_CTRL_TOG: u32,
    pub PS_BUF: u32,
    pub PS_UBUF: u32,
    pub PS_VBUF: u32,
    pub PS_PITCH: u32,
    pub PS_BACKGROUND: u32,
    pub PS_SCALE: u32,
    pub PS_OFFSET: u32,
    pub PS_CLRKEYLOW: u32,
    pub PS_CLRKEYHIGH: u32,
    pub AS_CTRL: u32,
    pub AS_BUF: u32,
    pub AS_PITCH: u32,
    pub AS_CLRKEYLOW: u32,
    pub AS_CLRKEYHIGH: u32,
    pub CSC1_COEF0: u32,
    pub CSC1_COEF1: u32,
    pub CSC1_COEF2: u32,
    pub POWER: u32,
    pub NEXT: u32,
    pub PORTER_DUFF_CTRL: u32,
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
