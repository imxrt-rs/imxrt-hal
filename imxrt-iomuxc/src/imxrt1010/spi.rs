//! SPI pin implementations

use super::pads::{ad::*, sd::*};
use crate::{
    consts::*,
    spi::{Pin, PCS0, SCK, SDI, SDO},
    Daisy,
};

//
// SPI1
//

// PCS0

impl Pin for AD_05 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI1_PCS_0_AD_05;
    type Signal = PCS0;
    type Module = U1;
}

impl Pin for SD_07 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPSPI1_PCS_0_SD_07;
    type Signal = PCS0;
    type Module = U1;
}

// SCK

impl Pin for AD_06 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI1_SCK_AD_06;
    type Signal = SCK;
    type Module = U1;
}

impl Pin for SD_08 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPSPI1_SCK_SD_08;
    type Signal = SCK;
    type Module = U1;
}

// SDI

impl Pin for AD_03 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI1_SDI_AD_03;
    type Signal = SDI;
    type Module = U1;
}

impl Pin for SD_05 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPSPI1_SDI_SD_05;
    type Signal = SDI;
    type Module = U1;
}

// SDO

impl Pin for AD_04 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI1_SDO_AD_04;
    type Signal = SDO;
    type Module = U1;
}

impl Pin for SD_06 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPSPI1_SDO_SD_06;
    type Signal = SDO;
    type Module = U1;
}

//
// SPI2
//

// PCS0

impl Pin for AD_11 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI2_PCS_0_AD_11;
    type Signal = PCS0;
    type Module = U2;
}

impl Pin for SD_12 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPSPI2_PCS_0_SD_12;
    type Signal = PCS0;
    type Module = U2;
}

// SCK

impl Pin for AD_12 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI2_SCK_AD_12;
    type Signal = SCK;
    type Module = U2;
}

impl Pin for SD_11 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPSPI2_SCK_SD_11;
    type Signal = SCK;
    type Module = U2;
}

// SDI

impl Pin for AD_09 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI2_SDI_AD_09;
    type Signal = SDI;
    type Module = U2;
}

impl Pin for SD_09 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPSPI2_SDI_SD_09;
    type Signal = SDI;
    type Module = U2;
}

// SDO

impl Pin for AD_10 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPSPI2_SDO_AD_10;
    type Signal = SDO;
    type Module = U2;
}

impl Pin for SD_10 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPSPI2_SDO_SD_10;
    type Signal = SDO;
    type Module = U2;
}

mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPSPI1_PCS_0_AD_05: Daisy = Daisy::new(0x401f81d0 as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS_0_SD_07: Daisy = Daisy::new(0x401f81d0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_AD_06: Daisy = Daisy::new(0x401f81d4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_SD_08: Daisy = Daisy::new(0x401f81d4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_AD_03: Daisy = Daisy::new(0x401f81d8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_SD_05: Daisy = Daisy::new(0x401f81d8 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_AD_04: Daisy = Daisy::new(0x401f81dc as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_SD_06: Daisy = Daisy::new(0x401f81dc as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS_0_AD_11: Daisy = Daisy::new(0x401f81e0 as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS_0_SD_12: Daisy = Daisy::new(0x401f81e0 as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_AD_12: Daisy = Daisy::new(0x401f81e4 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_SD_11: Daisy = Daisy::new(0x401f81e4 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_AD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_SD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_AD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_SD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 1);
}

use daisy::*;
