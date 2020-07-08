//! SPI pin implementation

use super::pads::{ad_b0::*, b0::*, emc::*, sd_b0::*, sd_b1::*};
use crate::iomuxc::{
    consts::*,
    spi::{Pin, PCS0, SCK, SDI, SDO},
    Daisy,
};

//
// SPI1
//

impl Pin for SD_B0_01 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI1_PCS0_SD_B0_01;
    type Signal = PCS0;
    type Module = U1;
}

impl Pin for EMC_30 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPSPI1_PCS0_EMC_30;
    type Signal = PCS0;
    type Module = U1;
}

impl Pin for SD_B0_00 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI1_SCK_SD_B0_00;
    type Signal = SCK;
    type Module = U1;
}

impl Pin for SD_B0_03 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI1_SDI_SD_B0_03;
    type Signal = SDI;
    type Module = U1;
}

impl Pin for SD_B0_02 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI1_SDO_SD_B0_02;
    type Signal = SDO;
    type Module = U1;
}

//
// SPI2
//

impl Pin for SD_B1_07 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI2_SCK_SD_B1_07;
    type Signal = SCK;
    type Module = U2;
}

impl Pin for SD_B1_08 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI2_SDO_SD_B1_08;
    type Signal = SDO;
    type Module = U2;
}

impl Pin for SD_B1_09 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI2_SDI_SD_B1_09;
    type Signal = SDI;
    type Module = U2;
}

impl Pin for SD_B1_06 {
    const ALT: u32 = 4;
    const DAISY: Daisy = DAISY_LPSPI2_PCS0_SD_B1_06;
    type Signal = PCS0;
    type Module = U2;
}

//
// SPI3
//

impl Pin for AD_B0_00 {
    const ALT: u32 = 7;
    const DAISY: Daisy = DAISY_LPSPI3_SCK_AD_B0_00;
    type Signal = SCK;
    type Module = U3;
}

impl Pin for AD_B0_01 {
    const ALT: u32 = 7;
    const DAISY: Daisy = DAISY_LPSPI3_SDO_AD_B0_01;
    type Signal = SDO;
    type Module = U3;
}

impl Pin for AD_B0_02 {
    const ALT: u32 = 7;
    const DAISY: Daisy = DAISY_LPSPI3_SDI_AD_B0_02;
    type Signal = SDI;
    type Module = U3;
}

impl Pin for AD_B0_03 {
    const ALT: u32 = 7;
    const DAISY: Daisy = DAISY_LPSPI3_PCS0_AD_B0_03;
    type Signal = PCS0;
    type Module = U3;
}

//
// SPI4
//

impl Pin for B0_03 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPSPI4_SCK_B0_03;
    type Signal = SCK;
    type Module = U4;
}

impl Pin for B0_02 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPSPI4_SDO_B0_02;
    type Signal = SDO;
    type Module = U4;
}

impl Pin for B0_01 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPSPI4_SDI_B0_01;
    type Signal = SDI;
    type Module = U4;
}

impl Pin for B0_00 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPSPI4_PCS0_B0_00;
    type Signal = PCS0;
    type Module = U4;
}

/// Auto-generated DAISY values
mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPSPI1_PCS0_SD_B0_01: Daisy = Daisy::new(0x401f84ec as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS0_EMC_30: Daisy = Daisy::new(0x401f84ec as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_EMC_27: Daisy = Daisy::new(0x401f84f0 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_SD_B0_00: Daisy = Daisy::new(0x401f84f0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_EMC_29: Daisy = Daisy::new(0x401f84f4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_SD_B0_03: Daisy = Daisy::new(0x401f84f4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_EMC_28: Daisy = Daisy::new(0x401f84f8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_SD_B0_02: Daisy = Daisy::new(0x401f84f8 as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS0_SD_B1_06: Daisy = Daisy::new(0x401f84fc as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS0_EMC_01: Daisy = Daisy::new(0x401f84fc as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_SD_B1_07: Daisy = Daisy::new(0x401f8500 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_EMC_00: Daisy = Daisy::new(0x401f8500 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_SD_B1_09: Daisy = Daisy::new(0x401f8504 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_EMC_03: Daisy = Daisy::new(0x401f8504 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_SD_B1_08: Daisy = Daisy::new(0x401f8508 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_EMC_02: Daisy = Daisy::new(0x401f8508 as *mut u32, 1);
    pub const DAISY_LPSPI3_PCS0_AD_B0_03: Daisy = Daisy::new(0x401f850c as *mut u32, 0);
    pub const DAISY_LPSPI3_PCS0_AD_B1_12: Daisy = Daisy::new(0x401f850c as *mut u32, 1);
    pub const DAISY_LPSPI3_SCK_AD_B0_00: Daisy = Daisy::new(0x401f8510 as *mut u32, 0);
    pub const DAISY_LPSPI3_SCK_AD_B1_15: Daisy = Daisy::new(0x401f8510 as *mut u32, 1);
    pub const DAISY_LPSPI3_SDI_AD_B0_02: Daisy = Daisy::new(0x401f8514 as *mut u32, 0);
    pub const DAISY_LPSPI3_SDI_AD_B1_13: Daisy = Daisy::new(0x401f8514 as *mut u32, 1);
    pub const DAISY_LPSPI3_SDO_AD_B0_01: Daisy = Daisy::new(0x401f8518 as *mut u32, 0);
    pub const DAISY_LPSPI3_SDO_AD_B1_14: Daisy = Daisy::new(0x401f8518 as *mut u32, 1);
    pub const DAISY_LPSPI4_PCS0_B0_00: Daisy = Daisy::new(0x401f851c as *mut u32, 0);
    pub const DAISY_LPSPI4_PCS0_B1_04: Daisy = Daisy::new(0x401f851c as *mut u32, 1);
    pub const DAISY_LPSPI4_SCK_B0_03: Daisy = Daisy::new(0x401f8520 as *mut u32, 0);
    pub const DAISY_LPSPI4_SCK_B1_07: Daisy = Daisy::new(0x401f8520 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDI_B0_01: Daisy = Daisy::new(0x401f8524 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDI_B1_05: Daisy = Daisy::new(0x401f8524 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDO_B0_02: Daisy = Daisy::new(0x401f8528 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDO_B1_06: Daisy = Daisy::new(0x401f8528 as *mut u32, 1);
}

use daisy::*;
