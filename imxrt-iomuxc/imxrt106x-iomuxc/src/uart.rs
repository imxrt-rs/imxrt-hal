//! UART pin implementations

use super::pads::{ad_b0::*, ad_b1::*, b1::*, emc::*};
use crate::iomuxc::{
    consts::*,
    uart::{Pin, RX, TX},
    Daisy,
};

//
// UART1
//

impl Pin for AD_B0_13 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = None;
    type Direction = RX;
    type Module = U1;
}

impl Pin for AD_B0_12 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = None;
    type Direction = TX;
    type Module = U1;
}

//
// UART2
//

impl Pin for AD_B1_03 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_RX_AD_B1_03);
    type Direction = RX;
    type Module = U2;
}

impl Pin for AD_B1_02 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_TX_AD_B1_02);
    type Direction = TX;
    type Module = U2;
}

//
// UART3
//

impl Pin for AD_B1_07 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_RX_AD_B1_07);
    type Direction = RX;
    type Module = U3;
}

impl Pin for AD_B1_06 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_TX_AD_B1_06);
    type Direction = TX;
    type Module = U3;
}

//
// UART4
//

impl Pin for B1_01 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_RX_B1_01);
    type Direction = RX;
    type Module = U4;
}

impl Pin for B1_00 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_TX_B1_00);
    type Direction = TX;
    type Module = U4;
}

//
// UART5
//

// TODO

//
// UART6
//

impl Pin for AD_B0_03 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART6_RX_AD_B0_03);
    type Direction = RX;
    type Module = U6;
}

impl Pin for AD_B0_02 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART6_TX_AD_B0_02);
    type Direction = TX;
    type Module = U6;
}

//
// UART7
//

impl Pin for EMC_32 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART7_RX_EMC_32);
    type Direction = RX;
    type Module = U7;
}

impl Pin for EMC_31 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART7_TX_EMC_31);
    type Direction = TX;
    type Module = U7;
}

//
// UART8
//

impl Pin for AD_B1_11 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART8_RX_AD_B1_11);
    type Direction = RX;
    type Module = U8;
}

impl Pin for AD_B1_10 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART8_TX_AD_B1_10);
    type Direction = TX;
    type Module = U8;
}

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPUART2_RX_SD_B1_10: Daisy = Daisy::new(0x401f852c as *mut u32, 0);
    pub const DAISY_LPUART2_RX_AD_B1_03: Daisy = Daisy::new(0x401f852c as *mut u32, 1);
    pub const DAISY_LPUART2_TX_SD_B1_11: Daisy = Daisy::new(0x401f8530 as *mut u32, 0);
    pub const DAISY_LPUART2_TX_AD_B1_02: Daisy = Daisy::new(0x401f8530 as *mut u32, 1);
    pub const DAISY_LPUART3_CTS_B_EMC_15: Daisy = Daisy::new(0x401f8534 as *mut u32, 0);
    pub const DAISY_LPUART3_CTS_B_AD_B1_04: Daisy = Daisy::new(0x401f8534 as *mut u32, 1);
    pub const DAISY_LPUART3_RX_AD_B1_07: Daisy = Daisy::new(0x401f8538 as *mut u32, 0);
    pub const DAISY_LPUART3_RX_EMC_14: Daisy = Daisy::new(0x401f8538 as *mut u32, 1);
    pub const DAISY_LPUART3_RX_B0_09: Daisy = Daisy::new(0x401f8538 as *mut u32, 2);
    pub const DAISY_LPUART3_TX_AD_B1_06: Daisy = Daisy::new(0x401f853c as *mut u32, 0);
    pub const DAISY_LPUART3_TX_EMC_13: Daisy = Daisy::new(0x401f853c as *mut u32, 1);
    pub const DAISY_LPUART3_TX_B0_08: Daisy = Daisy::new(0x401f853c as *mut u32, 2);
    pub const DAISY_LPUART4_RX_SD_B1_01: Daisy = Daisy::new(0x401f8540 as *mut u32, 0);
    pub const DAISY_LPUART4_RX_EMC_20: Daisy = Daisy::new(0x401f8540 as *mut u32, 1);
    pub const DAISY_LPUART4_RX_B1_01: Daisy = Daisy::new(0x401f8540 as *mut u32, 2);
    pub const DAISY_LPUART4_TX_SD_B1_00: Daisy = Daisy::new(0x401f8544 as *mut u32, 0);
    pub const DAISY_LPUART4_TX_EMC_19: Daisy = Daisy::new(0x401f8544 as *mut u32, 1);
    pub const DAISY_LPUART4_TX_B1_00: Daisy = Daisy::new(0x401f8544 as *mut u32, 2);
    pub const DAISY_LPUART5_RX_EMC_24: Daisy = Daisy::new(0x401f8548 as *mut u32, 0);
    pub const DAISY_LPUART5_RX_B1_13: Daisy = Daisy::new(0x401f8548 as *mut u32, 1);
    pub const DAISY_LPUART5_TX_EMC_23: Daisy = Daisy::new(0x401f854c as *mut u32, 0);
    pub const DAISY_LPUART5_TX_B1_12: Daisy = Daisy::new(0x401f854c as *mut u32, 1);
    pub const DAISY_LPUART6_RX_EMC_26: Daisy = Daisy::new(0x401f8550 as *mut u32, 0);
    pub const DAISY_LPUART6_RX_AD_B0_03: Daisy = Daisy::new(0x401f8550 as *mut u32, 1);
    pub const DAISY_LPUART6_TX_EMC_25: Daisy = Daisy::new(0x401f8554 as *mut u32, 0);
    pub const DAISY_LPUART6_TX_AD_B0_02: Daisy = Daisy::new(0x401f8554 as *mut u32, 1);
    pub const DAISY_LPUART7_RX_SD_B1_09: Daisy = Daisy::new(0x401f8558 as *mut u32, 0);
    pub const DAISY_LPUART7_RX_EMC_32: Daisy = Daisy::new(0x401f8558 as *mut u32, 1);
    pub const DAISY_LPUART7_TX_SD_B1_08: Daisy = Daisy::new(0x401f855c as *mut u32, 0);
    pub const DAISY_LPUART7_TX_EMC_31: Daisy = Daisy::new(0x401f855c as *mut u32, 1);
    pub const DAISY_LPUART8_RX_SD_B0_05: Daisy = Daisy::new(0x401f8560 as *mut u32, 0);
    pub const DAISY_LPUART8_RX_AD_B1_11: Daisy = Daisy::new(0x401f8560 as *mut u32, 1);
    pub const DAISY_LPUART8_RX_EMC_39: Daisy = Daisy::new(0x401f8560 as *mut u32, 2);
    pub const DAISY_LPUART8_TX_SD_B0_04: Daisy = Daisy::new(0x401f8564 as *mut u32, 0);
    pub const DAISY_LPUART8_TX_AD_B1_10: Daisy = Daisy::new(0x401f8564 as *mut u32, 1);
    pub const DAISY_LPUART8_TX_EMC_38: Daisy = Daisy::new(0x401f8564 as *mut u32, 2);
}

use daisy::*;
