#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSI
//!
//! Used by: imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CSI Control Register 1
pub mod CSICR1 {

    /// Pixel Bit
    pub mod PIXEL_BIT {
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

            /// 0b0: 8-bit data for each pixel
            pub const PIXEL_BIT_0: u32 = 0b0;

            /// 0b1: 10-bit data for each pixel
            pub const PIXEL_BIT_1: u32 = 0b1;
        }
    }

    /// Valid Pixel Clock Edge Select
    pub mod REDGE {
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

            /// 0b0: Pixel data is latched at the falling edge of CSI_PIXCLK
            pub const REDGE_0: u32 = 0b0;

            /// 0b1: Pixel data is latched at the rising edge of CSI_PIXCLK
            pub const REDGE_1: u32 = 0b1;
        }
    }

    /// Invert Pixel Clock Input
    pub mod INV_PCLK {
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

            /// 0b0: CSI_PIXCLK is directly applied to internal circuitry
            pub const INV_PCLK_0: u32 = 0b0;

            /// 0b1: CSI_PIXCLK is inverted before applied to internal circuitry
            pub const INV_PCLK_1: u32 = 0b1;
        }
    }

    /// Invert Data Input. This bit enables or disables internal inverters on the data lines.
    pub mod INV_DATA {
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

            /// 0b0: CSI_D\[7:0\] data lines are directly applied to internal circuitry
            pub const INV_DATA_0: u32 = 0b0;

            /// 0b1: CSI_D\[7:0\] data lines are inverted before applied to internal circuitry
            pub const INV_DATA_1: u32 = 0b1;
        }
    }

    /// Gated Clock Mode Enable
    pub mod GCLK_MODE {
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

            /// 0b0: Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored.
            pub const GCLK_MODE_0: u32 = 0b0;

            /// 0b1: Gated clock mode. Pixel clock signal is valid only when HSYNC is active.
            pub const GCLK_MODE_1: u32 = 0b1;
        }
    }

    /// Asynchronous RXFIFO Clear
    pub mod CLR_RXFIFO {
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

    /// Asynchronous STATFIFO Clear
    pub mod CLR_STATFIFO {
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

    /// Data Packing Direction
    pub mod PACK_DIR {
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

            /// 0b0: Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO.
            pub const PACK_DIR_0: u32 = 0b0;

            /// 0b1: Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO.
            pub const PACK_DIR_1: u32 = 0b1;
        }
    }

    /// FIFO Clear Control
    pub mod FCC {
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

            /// 0b0: Asynchronous FIFO clear is selected.
            pub const FCC_0: u32 = 0b0;

            /// 0b1: Synchronous FIFO clear is selected.
            pub const FCC_1: u32 = 0b1;
        }
    }

    /// CCIR656 Interface Enable
    pub mod CCIR_EN {
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

            /// 0b0: Traditional interface is selected. Timing interface logic is used to latch data.
            pub const CCIR_EN_0: u32 = 0b0;

            /// 0b1: CCIR656 interface is selected.
            pub const CCIR_EN_1: u32 = 0b1;
        }
    }

    /// HSYNC Polarity Select
    pub mod HSYNC_POL {
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

            /// 0b0: HSYNC is active low
            pub const HSYNC_POL_0: u32 = 0b0;

            /// 0b1: HSYNC is active high
            pub const HSYNC_POL_1: u32 = 0b1;
        }
    }

    /// Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt.
    pub mod SOF_INTEN {
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

            /// 0b0: SOF interrupt disable
            pub const SOF_INTEN_0: u32 = 0b0;

            /// 0b1: SOF interrupt enable
            pub const SOF_INTEN_1: u32 = 0b1;
        }
    }

    /// SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt.
    pub mod SOF_POL {
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

            /// 0b0: SOF interrupt is generated on SOF falling edge
            pub const SOF_POL_0: u32 = 0b0;

            /// 0b1: SOF interrupt is generated on SOF rising edge
            pub const SOF_POL_1: u32 = 0b1;
        }
    }

    /// RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt.
    pub mod RXFF_INTEN {
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

            /// 0b0: RxFIFO full interrupt disable
            pub const RXFF_INTEN_0: u32 = 0b0;

            /// 0b1: RxFIFO full interrupt enable
            pub const RXFF_INTEN_1: u32 = 0b1;
        }
    }

    /// Frame Buffer1 DMA Transfer Done Interrupt Enable
    pub mod FB1_DMA_DONE_INTEN {
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

            /// 0b0: Frame Buffer1 DMA Transfer Done interrupt disable
            pub const FB1_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: Frame Buffer1 DMA Transfer Done interrupt enable
            pub const FB1_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// Frame Buffer2 DMA Transfer Done Interrupt Enable
    pub mod FB2_DMA_DONE_INTEN {
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

            /// 0b0: Frame Buffer2 DMA Transfer Done interrupt disable
            pub const FB2_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: Frame Buffer2 DMA Transfer Done interrupt enable
            pub const FB2_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt.
    pub mod STATFF_INTEN {
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

            /// 0b0: STATFIFO full interrupt disable
            pub const STATFF_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO full interrupt enable
            pub const STATFF_INTEN_1: u32 = 0b1;
        }
    }

    /// STATFIFO DMA Transfer Done Interrupt Enable
    pub mod SFF_DMA_DONE_INTEN {
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

            /// 0b0: STATFIFO DMA Transfer Done interrupt disable
            pub const SFF_DMA_DONE_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO DMA Transfer Done interrupt enable
            pub const SFF_DMA_DONE_INTEN_1: u32 = 0b1;
        }
    }

    /// RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt.
    pub mod RF_OR_INTEN {
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

            /// 0b0: RxFIFO overrun interrupt is disabled
            pub const RF_OR_INTEN_0: u32 = 0b0;

            /// 0b1: RxFIFO overrun interrupt is enabled
            pub const RF_OR_INTEN_1: u32 = 0b1;
        }
    }

    /// STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt.
    pub mod SF_OR_INTEN {
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

            /// 0b0: STATFIFO overrun interrupt is disabled
            pub const SF_OR_INTEN_0: u32 = 0b0;

            /// 0b1: STATFIFO overrun interrupt is enabled
            pub const SF_OR_INTEN_1: u32 = 0b1;
        }
    }

    /// Change Of Image Field (COF) Interrupt Enable
    pub mod COF_INT_EN {
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

            /// 0b0: COF interrupt is disabled
            pub const COF_INT_EN_0: u32 = 0b0;

            /// 0b1: COF interrupt is enabled
            pub const COF_INT_EN_1: u32 = 0b1;
        }
    }

    /// CCIR Mode Select
    pub mod CCIR_MODE {
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

            /// 0b0: Progressive mode is selected
            pub const CCIR_MODE_0: u32 = 0b0;

            /// 0b1: Interlace mode is selected
            pub const CCIR_MODE_1: u32 = 0b1;
        }
    }

    /// CSI-PrP Interface Enable
    pub mod PrP_IF_EN {
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

            /// 0b0: CSI to PrP bus is disabled
            pub const PrP_IF_EN_0: u32 = 0b0;

            /// 0b1: CSI to PrP bus is enabled
            pub const PrP_IF_EN_1: u32 = 0b1;
        }
    }

    /// End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt.
    pub mod EOF_INT_EN {
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

            /// 0b0: EOF interrupt is disabled.
            pub const EOF_INT_EN_0: u32 = 0b0;

            /// 0b1: EOF interrupt is generated when RX count value is reached.
            pub const EOF_INT_EN_1: u32 = 0b1;
        }
    }

    /// External VSYNC Enable
    pub mod EXT_VSYNC {
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

            /// 0b0: Internal VSYNC mode
            pub const EXT_VSYNC_0: u32 = 0b0;

            /// 0b1: External VSYNC mode
            pub const EXT_VSYNC_1: u32 = 0b1;
        }
    }

    /// SWAP 16-Bit Enable
    pub mod SWAP16_EN {
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

            /// 0b0: Disable swapping
            pub const SWAP16_EN_0: u32 = 0b0;

            /// 0b1: Enable swapping
            pub const SWAP16_EN_1: u32 = 0b1;
        }
    }
}

/// CSI Control Register 2
pub mod CSICR2 {

    /// Horizontal Skip Count
    pub mod HSC {
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

    /// Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored.
    pub mod VSC {
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

    /// Live View Resolution Mode. Selects the grid size used for live view resolution.
    pub mod LVRM {
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

            /// 0b000: 512 x 384
            pub const LVRM_0: u32 = 0b000;

            /// 0b001: 448 x 336
            pub const LVRM_1: u32 = 0b001;

            /// 0b010: 384 x 288
            pub const LVRM_2: u32 = 0b010;

            /// 0b011: 384 x 256
            pub const LVRM_3: u32 = 0b011;

            /// 0b100: 320 x 240
            pub const LVRM_4: u32 = 0b100;

            /// 0b101: 288 x 216
            pub const LVRM_5: u32 = 0b101;

            /// 0b110: 400 x 300
            pub const LVRM_6: u32 = 0b110;
        }
    }

    /// Bayer Tile Start. Controls the Bayer pattern starting point.
    pub mod BTS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: GR
            pub const BTS_0: u32 = 0b00;

            /// 0b01: RG
            pub const BTS_1: u32 = 0b01;

            /// 0b10: BG
            pub const BTS_2: u32 = 0b10;

            /// 0b11: GB
            pub const BTS_3: u32 = 0b11;
        }
    }

    /// Skip Count Enable. Enables or disables the skip count feature.
    pub mod SCE {
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

            /// 0b0: Skip count disable
            pub const SCE_0: u32 = 0b0;

            /// 0b1: Skip count enable
            pub const SCE_1: u32 = 0b1;
        }
    }

    /// Auto Focus Spread. Selects which green pixels are used for auto-focus.
    pub mod AFS {
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

            /// 0b00: Abs Diff on consecutive green pixels
            pub const AFS_0: u32 = 0b00;

            /// 0b01: Abs Diff on every third green pixels
            pub const AFS_1: u32 = 0b01;

            /// 0b00: Abs Diff on every four green pixels
            pub const AFS_2: u32 = 0b00;
        }
    }

    /// Double Resolution Mode. Controls size of statistics grid.
    pub mod DRM {
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

            /// 0b0: Stats grid of 8 x 6
            pub const DRM_0: u32 = 0b0;

            /// 0b1: Stats grid of 8 x 12
            pub const DRM_1: u32 = 0b1;
        }
    }

    /// Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO.
    pub mod DMA_BURST_TYPE_SFF {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: INCR8
            pub const DMA_BURST_TYPE_SFF_0: u32 = 0b00;

            /// 0b01: INCR4
            pub const DMA_BURST_TYPE_SFF_1: u32 = 0b01;

            /// 0b11: INCR16
            pub const DMA_BURST_TYPE_SFF_3: u32 = 0b11;
        }
    }

    /// Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO.
    pub mod DMA_BURST_TYPE_RFF {
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

            /// 0b00: INCR8
            pub const DMA_BURST_TYPE_RFF_0: u32 = 0b00;

            /// 0b01: INCR4
            pub const DMA_BURST_TYPE_RFF_1: u32 = 0b01;

            /// 0b11: INCR16
            pub const DMA_BURST_TYPE_RFF_3: u32 = 0b11;
        }
    }
}

/// CSI Control Register 3
pub mod CSICR3 {

    /// Automatic Error Correction Enable
    pub mod ECC_AUTO_EN {
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

            /// 0b0: Auto Error correction is disabled.
            pub const ECC_AUTO_EN_0: u32 = 0b0;

            /// 0b1: Auto Error correction is enabled.
            pub const ECC_AUTO_EN_1: u32 = 0b1;
        }
    }

    /// Error Detection Interrupt Enable
    pub mod ECC_INT_EN {
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

            /// 0b0: No interrupt is generated when error is detected. Only the status bit ECC_INT is set.
            pub const ECC_INT_EN_0: u32 = 0b0;

            /// 0b1: Interrupt is generated when error is detected.
            pub const ECC_INT_EN_1: u32 = 0b1;
        }
    }

    /// Dummy Zero Packing Enable
    pub mod ZERO_PACK_EN {
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

            /// 0b0: Zero packing disabled
            pub const ZERO_PACK_EN_0: u32 = 0b0;

            /// 0b1: Zero packing enabled
            pub const ZERO_PACK_EN_1: u32 = 0b1;
        }
    }

    /// Two 8-bit Sensor Mode
    pub mod TWO_8BIT_SENSOR {
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

            /// 0b0: Only one sensor is connected.
            pub const TWO_8BIT_SENSOR_0: u32 = 0b0;

            /// 0b1: Two 8-bit sensors are connected or one 16-bit sensor is connected.
            pub const TWO_8BIT_SENSOR_1: u32 = 0b1;
        }
    }

    /// RxFIFO Full Level
    pub mod RxFF_LEVEL {
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

            /// 0b000: 4 Double words
            pub const RxFF_LEVEL_0: u32 = 0b000;

            /// 0b001: 8 Double words
            pub const RxFF_LEVEL_1: u32 = 0b001;

            /// 0b010: 16 Double words
            pub const RxFF_LEVEL_2: u32 = 0b010;

            /// 0b011: 24 Double words
            pub const RxFF_LEVEL_3: u32 = 0b011;

            /// 0b100: 32 Double words
            pub const RxFF_LEVEL_4: u32 = 0b100;

            /// 0b101: 48 Double words
            pub const RxFF_LEVEL_5: u32 = 0b101;

            /// 0b110: 64 Double words
            pub const RxFF_LEVEL_6: u32 = 0b110;

            /// 0b111: 96 Double words
            pub const RxFF_LEVEL_7: u32 = 0b111;
        }
    }

    /// Hresponse Error Enable. This bit enables the hresponse error interrupt.
    pub mod HRESP_ERR_EN {
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

            /// 0b0: Disable hresponse error interrupt
            pub const HRESP_ERR_EN_0: u32 = 0b0;

            /// 0b1: Enable hresponse error interrupt
            pub const HRESP_ERR_EN_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Level
    pub mod STATFF_LEVEL {
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

            /// 0b000: 4 Double words
            pub const STATFF_LEVEL_0: u32 = 0b000;

            /// 0b001: 8 Double words
            pub const STATFF_LEVEL_1: u32 = 0b001;

            /// 0b010: 12 Double words
            pub const STATFF_LEVEL_2: u32 = 0b010;

            /// 0b011: 16 Double words
            pub const STATFF_LEVEL_3: u32 = 0b011;

            /// 0b100: 24 Double words
            pub const STATFF_LEVEL_4: u32 = 0b100;

            /// 0b101: 32 Double words
            pub const STATFF_LEVEL_5: u32 = 0b101;

            /// 0b110: 48 Double words
            pub const STATFF_LEVEL_6: u32 = 0b110;

            /// 0b111: 64 Double words
            pub const STATFF_LEVEL_7: u32 = 0b111;
        }
    }

    /// DMA Request Enable for STATFIFO
    pub mod DMA_REQ_EN_SFF {
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

            /// 0b0: Disable the dma request
            pub const DMA_REQ_EN_SFF_0: u32 = 0b0;

            /// 0b1: Enable the dma request
            pub const DMA_REQ_EN_SFF_1: u32 = 0b1;
        }
    }

    /// DMA Request Enable for RxFIFO
    pub mod DMA_REQ_EN_RFF {
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

            /// 0b0: Disable the dma request
            pub const DMA_REQ_EN_RFF_0: u32 = 0b0;

            /// 0b1: Enable the dma request
            pub const DMA_REQ_EN_RFF_1: u32 = 0b1;
        }
    }

    /// Reflash DMA Controller for STATFIFO
    pub mod DMA_REFLASH_SFF {
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

            /// 0b0: No reflashing
            pub const DMA_REFLASH_SFF_0: u32 = 0b0;

            /// 0b1: Reflash the embedded DMA controller
            pub const DMA_REFLASH_SFF_1: u32 = 0b1;
        }
    }

    /// Reflash DMA Controller for RxFIFO
    pub mod DMA_REFLASH_RFF {
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

            /// 0b0: No reflashing
            pub const DMA_REFLASH_RFF_0: u32 = 0b0;

            /// 0b1: Reflash the embedded DMA controller
            pub const DMA_REFLASH_RFF_1: u32 = 0b1;
        }
    }

    /// Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)
    pub mod FRMCNT_RST {
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

            /// 0b0: Do not reset
            pub const FRMCNT_RST_0: u32 = 0b0;

            /// 0b1: Reset frame counter immediately
            pub const FRMCNT_RST_1: u32 = 0b1;
        }
    }

    /// Frame Counter
    pub mod FRMCNT {
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

/// CSI Statistic FIFO Register
pub mod CSISTATFIFO {

    /// Static data from sensor
    pub mod STAT {
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

/// CSI RX FIFO Register
pub mod CSIRFIFO {

    /// Received image data
    pub mod IMAGE {
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

/// CSI RX Count Register
pub mod CSIRXCNT {

    /// RxFIFO Count
    pub mod RXCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CSI Status Register
pub mod CSISR {

    /// RXFIFO Data Ready
    pub mod DRDY {
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

            /// 0b0: No data (word) is ready
            pub const DRDY_0: u32 = 0b0;

            /// 0b1: At least 1 datum (word) is ready in RXFIFO.
            pub const DRDY_1: u32 = 0b1;
        }
    }

    /// CCIR Error Interrupt
    pub mod ECC_INT {
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

            /// 0b0: No error detected
            pub const ECC_INT_0: u32 = 0b0;

            /// 0b1: Error is detected in CCIR coding
            pub const ECC_INT_1: u32 = 0b1;
        }
    }

    /// Hresponse Error Interrupt Status
    pub mod HRESP_ERR_INT {
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

            /// 0b0: No hresponse error.
            pub const HRESP_ERR_INT_0: u32 = 0b0;

            /// 0b1: Hresponse error is detected.
            pub const HRESP_ERR_INT_1: u32 = 0b1;
        }
    }

    /// Change Of Field Interrupt Status
    pub mod COF_INT {
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

            /// 0b0: Video field has no change.
            pub const COF_INT_0: u32 = 0b0;

            /// 0b1: Change of video field is detected.
            pub const COF_INT_1: u32 = 0b1;
        }
    }

    /// CCIR Field 1 Interrupt Status
    pub mod F1_INT {
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

            /// 0b0: Field 1 of video is not detected.
            pub const F1_INT_0: u32 = 0b0;

            /// 0b1: Field 1 of video is about to start.
            pub const F1_INT_1: u32 = 0b1;
        }
    }

    /// CCIR Field 2 Interrupt Status
    pub mod F2_INT {
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

            /// 0b0: Field 2 of video is not detected
            pub const F2_INT_0: u32 = 0b0;

            /// 0b1: Field 2 of video is about to start
            pub const F2_INT_1: u32 = 0b1;
        }
    }

    /// Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)
    pub mod SOF_INT {
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

            /// 0b0: SOF is not detected.
            pub const SOF_INT_0: u32 = 0b0;

            /// 0b1: SOF is detected.
            pub const SOF_INT_1: u32 = 0b1;
        }
    }

    /// End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)
    pub mod EOF_INT {
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

            /// 0b0: EOF is not detected.
            pub const EOF_INT_0: u32 = 0b0;

            /// 0b1: EOF is detected.
            pub const EOF_INT_1: u32 = 0b1;
        }
    }

    /// RXFIFO Full Interrupt Status
    pub mod RxFF_INT {
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

            /// 0b0: RxFIFO is not full.
            pub const RxFF_INT_0: u32 = 0b0;

            /// 0b1: RxFIFO is full.
            pub const RxFF_INT_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done in Frame Buffer1
    pub mod DMA_TSF_DONE_FB1 {
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

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_FB1_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_FB1_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done in Frame Buffer2
    pub mod DMA_TSF_DONE_FB2 {
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

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_FB2_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_FB2_1: u32 = 0b1;
        }
    }

    /// STATFIFO Full Interrupt Status
    pub mod STATFF_INT {
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

            /// 0b0: STATFIFO is not full.
            pub const STATFF_INT_0: u32 = 0b0;

            /// 0b1: STATFIFO is full.
            pub const STATFF_INT_1: u32 = 0b1;
        }
    }

    /// DMA Transfer Done from StatFIFO
    pub mod DMA_TSF_DONE_SFF {
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

            /// 0b0: DMA transfer is not completed.
            pub const DMA_TSF_DONE_SFF_0: u32 = 0b0;

            /// 0b1: DMA transfer is completed.
            pub const DMA_TSF_DONE_SFF_1: u32 = 0b1;
        }
    }

    /// RxFIFO Overrun Interrupt Status
    pub mod RF_OR_INT {
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

            /// 0b0: RXFIFO has not overflowed.
            pub const RF_OR_INT_0: u32 = 0b0;

            /// 0b1: RXFIFO has overflowed.
            pub const RF_OR_INT_1: u32 = 0b1;
        }
    }

    /// STATFIFO Overrun Interrupt Status
    pub mod SF_OR_INT {
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

            /// 0b0: STATFIFO has not overflowed.
            pub const SF_OR_INT_0: u32 = 0b0;

            /// 0b1: STATFIFO has overflowed.
            pub const SF_OR_INT_1: u32 = 0b1;
        }
    }

    /// When DMA field 0 is complete, this bit will be set to 1(clear by writing 1).
    pub mod DMA_FIELD1_DONE {
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

    /// When DMA field 0 is complete, this bit will be set to 1(clear by writing 1).
    pub mod DMA_FIELD0_DONE {
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

    /// When using base address switching enable, this bit will be 1 when switching occur before DMA complete
    pub mod BASEADDR_CHHANGE_ERROR {
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

/// CSI DMA Start Address Register - for STATFIFO
pub mod CSIDMASA_STATFIFO {

    /// DMA Start Address for STATFIFO
    pub mod DMA_START_ADDR_SFF {
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

/// CSI DMA Transfer Size Register - for STATFIFO
pub mod CSIDMATS_STATFIFO {

    /// DMA Transfer Size for STATFIFO
    pub mod DMA_TSF_SIZE_SFF {
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

/// CSI DMA Start Address Register - for Frame Buffer1
pub mod CSIDMASA_FB1 {

    /// DMA Start Address in Frame Buffer1
    pub mod DMA_START_ADDR_FB1 {
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

/// CSI DMA Transfer Size Register - for Frame Buffer2
pub mod CSIDMASA_FB2 {

    /// DMA Start Address in Frame Buffer2
    pub mod DMA_START_ADDR_FB2 {
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

/// CSI Frame Buffer Parameter Register
pub mod CSIFBUF_PARA {

    /// Frame Buffer Parameter
    pub mod FBUF_STRIDE {
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

    /// DEINTERLACE_STRIDE is only used in the deinterlace mode
    pub mod DEINTERLACE_STRIDE {
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

/// CSI Image Parameter Register
pub mod CSIIMAG_PARA {

    /// Image Height. Indicates how many pixels in a column of the image from the sensor.
    pub mod IMAGE_HEIGHT {
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

    /// Image Width
    pub mod IMAGE_WIDTH {
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

/// CSI Control Register 18
pub mod CSICR18 {

    /// This bit is used to select the output method When input is standard CCIR656 video.
    pub mod DEINTERLACE_EN {
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

            /// 0b0: Deinterlace disabled
            pub const DEINTERLACE_EN_0: u32 = 0b0;

            /// 0b1: Deinterlace enabled
            pub const DEINTERLACE_EN_1: u32 = 0b1;
        }
    }

    /// When input is parallel rgb888/yuv444 24bit, this bit can be enabled.
    pub mod PARALLEL24_EN {
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

    /// When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than atomically by DMA completed
    pub mod BASEADDR_SWITCH_EN {
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

    /// CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1.
    pub mod BASEADDR_SWITCH_SEL {
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

            /// 0b0: Switching base address at the edge of the vsync
            pub const BASEADDR_SWITCH_SEL_0: u32 = 0b0;

            /// 0b1: Switching base address at the edge of the first data of each frame
            pub const BASEADDR_SWITCH_SEL_1: u32 = 0b1;
        }
    }

    /// In interlace mode, fileld 0 means interrupt enabled.
    pub mod FIELD0_DONE_IE {
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

            /// 0b0: Interrupt disabled
            pub const FIELD0_DONE_IE_0: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const FIELD0_DONE_IE_1: u32 = 0b1;
        }
    }

    /// When in interlace mode, field 1 done interrupt enable.
    pub mod DMA_FIELD1_DONE_IE {
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

            /// 0b0: Interrupt disabled
            pub const DMA_FIELD1_DONE_IE_0: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const DMA_FIELD1_DONE_IE_1: u32 = 0b1;
        }
    }

    /// Choosing the last DMA request condition.
    pub mod LAST_DMA_REQ_SEL {
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

            /// 0b0: fifo_full_level
            pub const LAST_DMA_REQ_SEL_0: u32 = 0b0;

            /// 0b1: hburst_length
            pub const LAST_DMA_REQ_SEL_1: u32 = 0b1;
        }
    }

    /// Base address change error interrupt enable signal.
    pub mod BASEADDR_CHANGE_ERROR_IE {
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

    /// Output is 32-bit format.
    pub mod RGB888A_FORMAT_SEL {
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

            /// 0b0: {8'h0, data\[23:0\]}
            pub const RGB888A_FORMAT_SEL_0: u32 = 0b0;

            /// 0b1: {data\[23:0\], 8'h0}
            pub const RGB888A_FORMAT_SEL_1: u32 = 0b1;
        }
    }

    /// Hprot value in AHB bus protocol.
    pub mod AHB_HPROT {
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

    /// These bits used to choose the method to mask the CSI input.
    pub mod MASK_OPTION {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1.
            pub const MASK_OPTION_0: u32 = 0b00;

            /// 0b01: Writing to memory when CSI_ENABLE is 1.
            pub const MASK_OPTION_1: u32 = 0b01;

            /// 0b10: Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1.
            pub const MASK_OPTION_2: u32 = 0b10;

            /// 0b11: Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0.
            pub const MASK_OPTION_3: u32 = 0b11;
        }
    }

    /// CSI global enable signal
    pub mod CSI_ENABLE {
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

/// CSI Control Register 19
pub mod CSICR19 {

    /// This byte stores the highest FIFO level achieved by CSI FIFO timely and will be clear by writing 8'ff to it
    pub mod DMA_RFIFO_HIGHEST_FIFO_LEVEL {
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
pub struct RegisterBlock {
    /// CSI Control Register 1
    pub CSICR1: RWRegister<u32>,

    /// CSI Control Register 2
    pub CSICR2: RWRegister<u32>,

    /// CSI Control Register 3
    pub CSICR3: RWRegister<u32>,

    /// CSI Statistic FIFO Register
    pub CSISTATFIFO: RORegister<u32>,

    /// CSI RX FIFO Register
    pub CSIRFIFO: RORegister<u32>,

    /// CSI RX Count Register
    pub CSIRXCNT: RWRegister<u32>,

    /// CSI Status Register
    pub CSISR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// CSI DMA Start Address Register - for STATFIFO
    pub CSIDMASA_STATFIFO: RWRegister<u32>,

    /// CSI DMA Transfer Size Register - for STATFIFO
    pub CSIDMATS_STATFIFO: RWRegister<u32>,

    /// CSI DMA Start Address Register - for Frame Buffer1
    pub CSIDMASA_FB1: RWRegister<u32>,

    /// CSI DMA Transfer Size Register - for Frame Buffer2
    pub CSIDMASA_FB2: RWRegister<u32>,

    /// CSI Frame Buffer Parameter Register
    pub CSIFBUF_PARA: RWRegister<u32>,

    /// CSI Image Parameter Register
    pub CSIIMAG_PARA: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// CSI Control Register 18
    pub CSICR18: RWRegister<u32>,

    /// CSI Control Register 19
    pub CSICR19: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSICR1: u32,
    pub CSICR2: u32,
    pub CSICR3: u32,
    pub CSISTATFIFO: u32,
    pub CSIRFIFO: u32,
    pub CSIRXCNT: u32,
    pub CSISR: u32,
    pub CSIDMASA_STATFIFO: u32,
    pub CSIDMATS_STATFIFO: u32,
    pub CSIDMASA_FB1: u32,
    pub CSIDMASA_FB2: u32,
    pub CSIFBUF_PARA: u32,
    pub CSIIMAG_PARA: u32,
    pub CSICR18: u32,
    pub CSICR19: u32,
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
