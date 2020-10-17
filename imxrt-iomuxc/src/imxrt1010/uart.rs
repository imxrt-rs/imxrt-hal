//! UART pin implementations

use super::pads::{ad::*, gpio::*, sd::*};
use crate::{
    consts::*,
    uart::{Pin, RX, TX},
    Daisy,
};

//
// UART1
//

impl Pin for GPIO_09 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART1_RXD_09);
    type Direction = RX;
    type Module = U1;
}

impl Pin for SD_11 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART1_RXD_SD_11);
    type Direction = RX;
    type Module = U1;
}

impl Pin for GPIO_10 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART1_TXD_10);
    type Direction = TX;
    type Module = U1;
}

impl Pin for SD_12 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART1_TXD_SD_12);
    type Direction = TX;
    type Module = U1;
}

//
// UART2
//

impl Pin for GPIO_13 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_RXD_13);
    type Direction = RX;
    type Module = U2;
}

impl Pin for SD_09 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_RXD_SD_09);
    type Direction = RX;
    type Module = U2;
}

impl Pin for AD_00 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_TXD_AD_00);
    type Direction = TX;
    type Module = U2;
}

impl Pin for SD_10 {
    const ALT: u32 = 2;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART2_TXD_SD_10);
    type Direction = TX;
    type Module = U2;
}

//
// UART3
//

impl Pin for GPIO_11 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_RXD_11);
    type Direction = RX;
    type Module = U3;
}

impl Pin for AD_07 {
    const ALT: u32 = 1;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_RXD_AD_07);
    type Direction = RX;
    type Module = U3;
}

impl Pin for GPIO_07 {
    const ALT: u32 = 3;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_RXD_07);
    type Direction = RX;
    type Module = U3;
}

impl Pin for GPIO_12 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_TXD_12);
    type Direction = TX;
    type Module = U3;
}

impl Pin for AD_08 {
    const ALT: u32 = 1;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_TXD_AD_08);
    type Direction = TX;
    type Module = U3;
}

impl Pin for GPIO_08 {
    const ALT: u32 = 3;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART3_TXD_08);
    type Direction = TX;
    type Module = U3;
}

//
// UART4
//

impl Pin for AD_01 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_RXD_AD_01);
    type Direction = RX;
    type Module = U4;
}

impl Pin for GPIO_05 {
    const ALT: u32 = 3;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_RXD_05);
    type Direction = RX;
    type Module = U4;
}

impl Pin for AD_02 {
    const ALT: u32 = 0;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_TXD_AD_02);
    type Direction = TX;
    type Module = U4;
}

impl Pin for GPIO_06 {
    const ALT: u32 = 3;
    const DAISY: Option<Daisy> = Some(DAISY_LPUART4_TXD_06);
    type Direction = TX;
    type Module = U4;
}

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPUART1_RXD_SD_11: Daisy = Daisy::new(0x401f81f0 as *mut u32, 0);
    pub const DAISY_LPUART1_RXD_09: Daisy = Daisy::new(0x401f81f0 as *mut u32, 1);
    pub const DAISY_LPUART1_TXD_SD_12: Daisy = Daisy::new(0x401f81f4 as *mut u32, 0);
    pub const DAISY_LPUART1_TXD_10: Daisy = Daisy::new(0x401f81f4 as *mut u32, 1);
    pub const DAISY_LPUART2_RXD_SD_09: Daisy = Daisy::new(0x401f81f8 as *mut u32, 0);
    pub const DAISY_LPUART2_RXD_13: Daisy = Daisy::new(0x401f81f8 as *mut u32, 1);
    pub const DAISY_LPUART2_TXD_AD_00: Daisy = Daisy::new(0x401f81fc as *mut u32, 0);
    pub const DAISY_LPUART2_TXD_SD_10: Daisy = Daisy::new(0x401f81fc as *mut u32, 1);
    pub const DAISY_LPUART3_RXD_AD_07: Daisy = Daisy::new(0x401f8200 as *mut u32, 0);
    pub const DAISY_LPUART3_RXD_11: Daisy = Daisy::new(0x401f8200 as *mut u32, 1);
    pub const DAISY_LPUART3_RXD_07: Daisy = Daisy::new(0x401f8200 as *mut u32, 2);
    pub const DAISY_LPUART3_TXD_AD_08: Daisy = Daisy::new(0x401f8204 as *mut u32, 0);
    pub const DAISY_LPUART3_TXD_12: Daisy = Daisy::new(0x401f8204 as *mut u32, 1);
    pub const DAISY_LPUART3_TXD_08: Daisy = Daisy::new(0x401f8204 as *mut u32, 2);
    pub const DAISY_LPUART4_RXD_AD_01: Daisy = Daisy::new(0x401f8208 as *mut u32, 0);
    pub const DAISY_LPUART4_RXD_05: Daisy = Daisy::new(0x401f8208 as *mut u32, 1);
    pub const DAISY_LPUART4_TXD_AD_02: Daisy = Daisy::new(0x401f820c as *mut u32, 0);
    pub const DAISY_LPUART4_TXD_06: Daisy = Daisy::new(0x401f820c as *mut u32, 1);
}
use daisy::*;
