#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC_ETC

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC_ETC Global Control Register
pub mod CTRL {

    /// TRIG enable register
    pub mod TRIG_ENABLE {
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

    /// TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger.
    pub mod EXT0_TRIG_ENABLE {
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

    /// External TSC0 trigger priority, 7 is Highest, 0 is lowest .
    pub mod EXT0_TRIG_PRIORITY {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger.
    pub mod EXT1_TRIG_ENABLE {
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

    /// External TSC1 trigger priority, 7 is Highest, 0 is lowest .
    pub mod EXT1_TRIG_PRIORITY {
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

    /// Pre-divider for trig delay and interval .
    pub mod PRE_DIVIDER {
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

    /// 1'b0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared
    pub mod DMA_MODE_SEL {
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

    /// 1'b1: TSC is bypassed to ADC2. 1'b0: TSC not bypassed. To use ADC2, this bit should be cleared.
    pub mod TSC_BYPASS {
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

    /// Software reset, high active. When write 1 ,all logical will be reset.
    pub mod SOFTRST {
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

/// ETC DONE0 and DONE1 IRQ State Register
pub mod DONE0_1_IRQ {

    /// TRIG0 done0 interrupt detection
    pub mod TRIG0_DONE0 {
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

    /// TRIG1 done0 interrupt detection
    pub mod TRIG1_DONE0 {
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

    /// TRIG2 done0 interrupt detection
    pub mod TRIG2_DONE0 {
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

    /// TRIG3 done0 interrupt detection
    pub mod TRIG3_DONE0 {
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

    /// TRIG4 done0 interrupt detection
    pub mod TRIG4_DONE0 {
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

    /// TRIG5 done0 interrupt detection
    pub mod TRIG5_DONE0 {
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

    /// TRIG6 done0 interrupt detection
    pub mod TRIG6_DONE0 {
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

    /// TRIG7 done0 interrupt detection
    pub mod TRIG7_DONE0 {
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

    /// TRIG0 done1 interrupt detection
    pub mod TRIG0_DONE1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIG1 done1 interrupt detection
    pub mod TRIG1_DONE1 {
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

    /// TRIG2 done1 interrupt detection
    pub mod TRIG2_DONE1 {
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

    /// TRIG3 done1 interrupt detection
    pub mod TRIG3_DONE1 {
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

    /// TRIG4 done1 interrupt detection
    pub mod TRIG4_DONE1 {
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

    /// TRIG5 done1 interrupt detection
    pub mod TRIG5_DONE1 {
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

    /// TRIG6 done1 interrupt detection
    pub mod TRIG6_DONE1 {
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

    /// TRIG7 done1 interrupt detection
    pub mod TRIG7_DONE1 {
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
}

/// ETC DONE_2 and DONE_ERR IRQ State Register
pub mod DONE2_ERR_IRQ {

    /// TRIG0 done2 interrupt detection
    pub mod TRIG0_DONE2 {
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

    /// TRIG1 done2 interrupt detection
    pub mod TRIG1_DONE2 {
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

    /// TRIG2 done2 interrupt detection
    pub mod TRIG2_DONE2 {
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

    /// TRIG3 done2 interrupt detection
    pub mod TRIG3_DONE2 {
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

    /// TRIG4 done2 interrupt detection
    pub mod TRIG4_DONE2 {
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

    /// TRIG5 done2 interrupt detection
    pub mod TRIG5_DONE2 {
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

    /// TRIG6 done2 interrupt detection
    pub mod TRIG6_DONE2 {
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

    /// TRIG7 done2 interrupt detection
    pub mod TRIG7_DONE2 {
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

    /// TRIG0 error interrupt detection
    pub mod TRIG0_ERR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIG1 error interrupt detection
    pub mod TRIG1_ERR {
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

    /// TRIG2 error interrupt detection
    pub mod TRIG2_ERR {
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

    /// TRIG3 error interrupt detection
    pub mod TRIG3_ERR {
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

    /// TRIG4 error interrupt detection
    pub mod TRIG4_ERR {
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

    /// TRIG5 error interrupt detection
    pub mod TRIG5_ERR {
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

    /// TRIG6 error interrupt detection
    pub mod TRIG6_ERR {
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

    /// TRIG7 error interrupt detection
    pub mod TRIG7_ERR {
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
}

/// ETC DMA control Register
pub mod DMA_CTRL {

    /// When TRIG0 done enable DMA request
    pub mod TRIG0_ENABLE {
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

    /// When TRIG1 done enable DMA request
    pub mod TRIG1_ENABLE {
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

    /// When TRIG2 done enable DMA request
    pub mod TRIG2_ENABLE {
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

    /// When TRIG3 done enable DMA request
    pub mod TRIG3_ENABLE {
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

    /// When TRIG4 done enable DMA request
    pub mod TRIG4_ENABLE {
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

    /// When TRIG5 done enable DMA request
    pub mod TRIG5_ENABLE {
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

    /// When TRIG6 done enable DMA request
    pub mod TRIG6_ENABLE {
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

    /// When TRIG7 done enable DMA request
    pub mod TRIG7_ENABLE {
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

    /// When TRIG0 done DMA request detection
    pub mod TRIG0_REQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When TRIG1 done DMA request detection
    pub mod TRIG1_REQ {
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

    /// When TRIG2 done DMA request detection
    pub mod TRIG2_REQ {
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

    /// When TRIG3 done DMA request detection
    pub mod TRIG3_REQ {
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

    /// When TRIG4 done DMA request detection
    pub mod TRIG4_REQ {
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

    /// When TRIG5 done DMA request detection
    pub mod TRIG5_REQ {
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

    /// When TRIG6 done DMA request detection
    pub mod TRIG6_REQ {
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

    /// When TRIG7 done DMA request detection
    pub mod TRIG7_REQ {
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
}

/// ETC_TRIG0 Control Register
pub mod TRIG0_CTRL {

    /// Software write 1 as the TRIGGER. This register is self-clearing.
    pub mod SW_TRIG {
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

    /// TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger.
    pub mod TRIG_MODE {
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

    /// TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;
    pub mod TRIG_CHAIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger priority, 7 is highest, 0 is lowest .
    pub mod TRIG_PRIORITY {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode
    pub mod SYNC_MODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG0 Counter Register
pub mod TRIG0_COUNTER {

    /// TRIGGER initial delay counter
    pub mod INIT_DELAY {
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

    /// TRIGGER sampling interval counter
    pub mod SAMPLE_INTERVAL {
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

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG0_CHAIN_1_0 {

    /// CHAIN0 CSEL ADC channel selection
    pub mod CSEL0 {
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

    /// CHAIN0 HWTS ADC hardware trigger selection. For more information, see the ADC chapter.
    pub mod HWTS0 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN0 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger
    pub mod B2B0 {
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

    /// CHAIN0 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2
    pub mod IE0 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN1 CSEL ADC channel selection
    pub mod CSEL1 {
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

    /// CHAIN1 HWTS ADC hardware trigger selection. For more information, see the ADC chapter.
    pub mod HWTS1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN1 B2B 1'b0: Disable B2B, wait until interval is reached 1'b1: Enable B2B, back to back ADC trigger
    pub mod B2B1 {
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

    /// CHAIN1 IE 2'b00: No interrupt when finished 2'b01: Finished Interrupt on Done0 2'b10: Finished Interrupt on Done1 2'b11: Finished Interrupt on Done2
    pub mod IE1 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG0_CHAIN_3_2 {

    /// CHAIN2 CSEL
    pub mod CSEL2 {
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

    /// CHAIN2 HWTS
    pub mod HWTS2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN2 B2B
    pub mod B2B2 {
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

    /// CHAIN2 IE
    pub mod IE2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN3 CSEL
    pub mod CSEL3 {
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

    /// CHAIN3 HWTS
    pub mod HWTS3 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN3 B2B
    pub mod B2B3 {
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

    /// CHAIN3 IE
    pub mod IE3 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG0_CHAIN_5_4 {

    /// CHAIN4 CSEL
    pub mod CSEL4 {
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

    /// CHAIN4 HWTS
    pub mod HWTS4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN4 B2B
    pub mod B2B4 {
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

    /// CHAIN4 IE
    pub mod IE4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN5 CSEL
    pub mod CSEL5 {
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

    /// CHAIN5 HWTS
    pub mod HWTS5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN5 B2B
    pub mod B2B5 {
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

    /// CHAIN5 IE
    pub mod IE5 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG0_CHAIN_7_6 {

    /// CHAIN6 CSEL
    pub mod CSEL6 {
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

    /// CHAIN6 HWTS
    pub mod HWTS6 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (8 bits: 0xff << 4)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN6 B2B
    pub mod B2B6 {
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

    /// CHAIN6 IE
    pub mod IE6 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN7 CSEL
    pub mod CSEL7 {
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

    /// CHAIN7 HWTS
    pub mod HWTS7 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (8 bits: 0xff << 20)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHAIN7 B2B
    pub mod B2B7 {
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

    /// CHAIN7 IE
    pub mod IE7 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG0_RESULT_1_0 {

    /// Result DATA0
    pub mod DATA0 {
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

    /// Result DATA1
    pub mod DATA1 {
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

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG0_RESULT_3_2 {

    /// Result DATA2
    pub mod DATA2 {
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

    /// Result DATA3
    pub mod DATA3 {
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

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG0_RESULT_5_4 {

    /// Result DATA4
    pub mod DATA4 {
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

    /// Result DATA5
    pub mod DATA5 {
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

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG0_RESULT_7_6 {

    /// Result DATA6
    pub mod DATA6 {
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

    /// Result DATA7
    pub mod DATA7 {
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

/// ETC_TRIG1 Control Register
pub mod TRIG1_CTRL {
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG1 Counter Register
pub mod TRIG1_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG1_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE1;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG1_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE3;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG1_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE5;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG1_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE7;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG1_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG1_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG1_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG1_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG2 Control Register
pub mod TRIG2_CTRL {
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG2 Counter Register
pub mod TRIG2_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG2_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE1;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG2_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE3;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG2_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE5;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG2_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE7;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG2_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG2_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG2_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG2_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}

/// ETC_TRIG3 Control Register
pub mod TRIG3_CTRL {
    pub use super::TRIG0_CTRL::SW_TRIG;
    pub use super::TRIG0_CTRL::SYNC_MODE;
    pub use super::TRIG0_CTRL::TRIG_CHAIN;
    pub use super::TRIG0_CTRL::TRIG_MODE;
    pub use super::TRIG0_CTRL::TRIG_PRIORITY;
}

/// ETC_TRIG3 Counter Register
pub mod TRIG3_COUNTER {
    pub use super::TRIG0_COUNTER::INIT_DELAY;
    pub use super::TRIG0_COUNTER::SAMPLE_INTERVAL;
}

/// ETC_TRIG Chain 0/1 Register
pub mod TRIG3_CHAIN_1_0 {
    pub use super::TRIG0_CHAIN_1_0::B2B0;
    pub use super::TRIG0_CHAIN_1_0::B2B1;
    pub use super::TRIG0_CHAIN_1_0::CSEL0;
    pub use super::TRIG0_CHAIN_1_0::CSEL1;
    pub use super::TRIG0_CHAIN_1_0::HWTS0;
    pub use super::TRIG0_CHAIN_1_0::HWTS1;
    pub use super::TRIG0_CHAIN_1_0::IE0;
    pub use super::TRIG0_CHAIN_1_0::IE1;
}

/// ETC_TRIG Chain 2/3 Register
pub mod TRIG3_CHAIN_3_2 {
    pub use super::TRIG0_CHAIN_3_2::B2B2;
    pub use super::TRIG0_CHAIN_3_2::B2B3;
    pub use super::TRIG0_CHAIN_3_2::CSEL2;
    pub use super::TRIG0_CHAIN_3_2::CSEL3;
    pub use super::TRIG0_CHAIN_3_2::HWTS2;
    pub use super::TRIG0_CHAIN_3_2::HWTS3;
    pub use super::TRIG0_CHAIN_3_2::IE2;
    pub use super::TRIG0_CHAIN_3_2::IE3;
}

/// ETC_TRIG Chain 4/5 Register
pub mod TRIG3_CHAIN_5_4 {
    pub use super::TRIG0_CHAIN_5_4::B2B4;
    pub use super::TRIG0_CHAIN_5_4::B2B5;
    pub use super::TRIG0_CHAIN_5_4::CSEL4;
    pub use super::TRIG0_CHAIN_5_4::CSEL5;
    pub use super::TRIG0_CHAIN_5_4::HWTS4;
    pub use super::TRIG0_CHAIN_5_4::HWTS5;
    pub use super::TRIG0_CHAIN_5_4::IE4;
    pub use super::TRIG0_CHAIN_5_4::IE5;
}

/// ETC_TRIG Chain 6/7 Register
pub mod TRIG3_CHAIN_7_6 {
    pub use super::TRIG0_CHAIN_7_6::B2B6;
    pub use super::TRIG0_CHAIN_7_6::B2B7;
    pub use super::TRIG0_CHAIN_7_6::CSEL6;
    pub use super::TRIG0_CHAIN_7_6::CSEL7;
    pub use super::TRIG0_CHAIN_7_6::HWTS6;
    pub use super::TRIG0_CHAIN_7_6::HWTS7;
    pub use super::TRIG0_CHAIN_7_6::IE6;
    pub use super::TRIG0_CHAIN_7_6::IE7;
}

/// ETC_TRIG Result Data 1/0 Register
pub mod TRIG3_RESULT_1_0 {
    pub use super::TRIG0_RESULT_1_0::DATA0;
    pub use super::TRIG0_RESULT_1_0::DATA1;
}

/// ETC_TRIG Result Data 3/2 Register
pub mod TRIG3_RESULT_3_2 {
    pub use super::TRIG0_RESULT_3_2::DATA2;
    pub use super::TRIG0_RESULT_3_2::DATA3;
}

/// ETC_TRIG Result Data 5/4 Register
pub mod TRIG3_RESULT_5_4 {
    pub use super::TRIG0_RESULT_5_4::DATA4;
    pub use super::TRIG0_RESULT_5_4::DATA5;
}

/// ETC_TRIG Result Data 7/6 Register
pub mod TRIG3_RESULT_7_6 {
    pub use super::TRIG0_RESULT_7_6::DATA6;
    pub use super::TRIG0_RESULT_7_6::DATA7;
}
#[repr(C)]
pub struct RegisterBlock {
    /// ADC_ETC Global Control Register
    pub CTRL: RWRegister<u32>,

    /// ETC DONE0 and DONE1 IRQ State Register
    pub DONE0_1_IRQ: RWRegister<u32>,

    /// ETC DONE_2 and DONE_ERR IRQ State Register
    pub DONE2_ERR_IRQ: RWRegister<u32>,

    /// ETC DMA control Register
    pub DMA_CTRL: RWRegister<u32>,

    /// ETC_TRIG0 Control Register
    pub TRIG0_CTRL: RWRegister<u32>,

    /// ETC_TRIG0 Counter Register
    pub TRIG0_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG0_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG0_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG0_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG0_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG0_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG0_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG0_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG0_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG1 Control Register
    pub TRIG1_CTRL: RWRegister<u32>,

    /// ETC_TRIG1 Counter Register
    pub TRIG1_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG1_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG1_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG1_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG1_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG1_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG1_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG1_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG1_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG2 Control Register
    pub TRIG2_CTRL: RWRegister<u32>,

    /// ETC_TRIG2 Counter Register
    pub TRIG2_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG2_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG2_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG2_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG2_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG2_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG2_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG2_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG2_RESULT_7_6: RORegister<u32>,

    /// ETC_TRIG3 Control Register
    pub TRIG3_CTRL: RWRegister<u32>,

    /// ETC_TRIG3 Counter Register
    pub TRIG3_COUNTER: RWRegister<u32>,

    /// ETC_TRIG Chain 0/1 Register
    pub TRIG3_CHAIN_1_0: RWRegister<u32>,

    /// ETC_TRIG Chain 2/3 Register
    pub TRIG3_CHAIN_3_2: RWRegister<u32>,

    /// ETC_TRIG Chain 4/5 Register
    pub TRIG3_CHAIN_5_4: RWRegister<u32>,

    /// ETC_TRIG Chain 6/7 Register
    pub TRIG3_CHAIN_7_6: RWRegister<u32>,

    /// ETC_TRIG Result Data 1/0 Register
    pub TRIG3_RESULT_1_0: RORegister<u32>,

    /// ETC_TRIG Result Data 3/2 Register
    pub TRIG3_RESULT_3_2: RORegister<u32>,

    /// ETC_TRIG Result Data 5/4 Register
    pub TRIG3_RESULT_5_4: RORegister<u32>,

    /// ETC_TRIG Result Data 7/6 Register
    pub TRIG3_RESULT_7_6: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub DONE0_1_IRQ: u32,
    pub DONE2_ERR_IRQ: u32,
    pub DMA_CTRL: u32,
    pub TRIG0_CTRL: u32,
    pub TRIG0_COUNTER: u32,
    pub TRIG0_CHAIN_1_0: u32,
    pub TRIG0_CHAIN_3_2: u32,
    pub TRIG0_CHAIN_5_4: u32,
    pub TRIG0_CHAIN_7_6: u32,
    pub TRIG0_RESULT_1_0: u32,
    pub TRIG0_RESULT_3_2: u32,
    pub TRIG0_RESULT_5_4: u32,
    pub TRIG0_RESULT_7_6: u32,
    pub TRIG1_CTRL: u32,
    pub TRIG1_COUNTER: u32,
    pub TRIG1_CHAIN_1_0: u32,
    pub TRIG1_CHAIN_3_2: u32,
    pub TRIG1_CHAIN_5_4: u32,
    pub TRIG1_CHAIN_7_6: u32,
    pub TRIG1_RESULT_1_0: u32,
    pub TRIG1_RESULT_3_2: u32,
    pub TRIG1_RESULT_5_4: u32,
    pub TRIG1_RESULT_7_6: u32,
    pub TRIG2_CTRL: u32,
    pub TRIG2_COUNTER: u32,
    pub TRIG2_CHAIN_1_0: u32,
    pub TRIG2_CHAIN_3_2: u32,
    pub TRIG2_CHAIN_5_4: u32,
    pub TRIG2_CHAIN_7_6: u32,
    pub TRIG2_RESULT_1_0: u32,
    pub TRIG2_RESULT_3_2: u32,
    pub TRIG2_RESULT_5_4: u32,
    pub TRIG2_RESULT_7_6: u32,
    pub TRIG3_CTRL: u32,
    pub TRIG3_COUNTER: u32,
    pub TRIG3_CHAIN_1_0: u32,
    pub TRIG3_CHAIN_3_2: u32,
    pub TRIG3_CHAIN_5_4: u32,
    pub TRIG3_CHAIN_7_6: u32,
    pub TRIG3_RESULT_1_0: u32,
    pub TRIG3_RESULT_3_2: u32,
    pub TRIG3_RESULT_5_4: u32,
    pub TRIG3_RESULT_7_6: u32,
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

/// Access functions for the ADC_ETC peripheral instance
pub mod ADC_ETC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403b0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC_ETC
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        DONE0_1_IRQ: 0x00000000,
        DONE2_ERR_IRQ: 0x00000000,
        DMA_CTRL: 0x00000000,
        TRIG0_CTRL: 0x00000000,
        TRIG0_COUNTER: 0x00000000,
        TRIG0_CHAIN_1_0: 0x00000000,
        TRIG0_CHAIN_3_2: 0x00000000,
        TRIG0_CHAIN_5_4: 0x00000000,
        TRIG0_CHAIN_7_6: 0x00000000,
        TRIG0_RESULT_1_0: 0x00000000,
        TRIG0_RESULT_3_2: 0x00000000,
        TRIG0_RESULT_5_4: 0x00000000,
        TRIG0_RESULT_7_6: 0x00000000,
        TRIG1_CTRL: 0x00000000,
        TRIG1_COUNTER: 0x00000000,
        TRIG1_CHAIN_1_0: 0x00000000,
        TRIG1_CHAIN_3_2: 0x00000000,
        TRIG1_CHAIN_5_4: 0x00000000,
        TRIG1_CHAIN_7_6: 0x00000000,
        TRIG1_RESULT_1_0: 0x00000000,
        TRIG1_RESULT_3_2: 0x00000000,
        TRIG1_RESULT_5_4: 0x00000000,
        TRIG1_RESULT_7_6: 0x00000000,
        TRIG2_CTRL: 0x00000000,
        TRIG2_COUNTER: 0x00000000,
        TRIG2_CHAIN_1_0: 0x00000000,
        TRIG2_CHAIN_3_2: 0x00000000,
        TRIG2_CHAIN_5_4: 0x00000000,
        TRIG2_CHAIN_7_6: 0x00000000,
        TRIG2_RESULT_1_0: 0x00000000,
        TRIG2_RESULT_3_2: 0x00000000,
        TRIG2_RESULT_5_4: 0x00000000,
        TRIG2_RESULT_7_6: 0x00000000,
        TRIG3_CTRL: 0x00000000,
        TRIG3_COUNTER: 0x00000000,
        TRIG3_CHAIN_1_0: 0x00000000,
        TRIG3_CHAIN_3_2: 0x00000000,
        TRIG3_CHAIN_5_4: 0x00000000,
        TRIG3_CHAIN_7_6: 0x00000000,
        TRIG3_RESULT_1_0: 0x00000000,
        TRIG3_RESULT_3_2: 0x00000000,
        TRIG3_RESULT_5_4: 0x00000000,
        TRIG3_RESULT_7_6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC_ETC_TAKEN: bool = false;

    /// Safe access to ADC_ETC
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
            if ADC_ETC_TAKEN {
                None
            } else {
                ADC_ETC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC_ETC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC_ETC_TAKEN && inst.addr == INSTANCE.addr {
                ADC_ETC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC_ETC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC_ETC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC_ETC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC_ETC: *const RegisterBlock = 0x403b0000 as *const _;
