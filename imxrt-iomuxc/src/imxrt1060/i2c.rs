//! I2C pin implementations

use super::pads::{ad_b0::*, ad_b1::*, sd_b0::*};
use crate::{
    consts::*,
    i2c::{Pin, SCL, SDA},
    Daisy,
};

//
// I2C1
//

impl Pin for AD_B1_00 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C1_SCL_AD_B1_00;
    type Signal = SCL;
    type Module = U1;
}

impl Pin for AD_B1_01 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C1_SDA_AD_B1_01;
    type Signal = SDA;
    type Module = U1;
}

//
// I2C2
//

// TODO

//
// I2C3
//

impl Pin for AD_B1_07 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C3_SCL_AD_B1_07;
    type Signal = SCL;
    type Module = U3;
}

impl Pin for AD_B1_06 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C3_SDA_AD_B1_06;
    type Signal = SDA;
    type Module = U3;
}

impl Pin for SD_B0_00 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPI2C3_SCL_SD_B0_00;
    type Signal = SCL;
    type Module = U3;
}

impl Pin for SD_B0_01 {
    const ALT: u32 = 2;
    const DAISY: Daisy = DAISY_LPI2C3_SDA_SD_B0_01;
    type Signal = SDA;
    type Module = U3;
}

//
// I2C4
//

impl Pin for AD_B0_12 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C4_SCL_AD_B0_12;
    type Signal = SCL;
    type Module = U4;
}

impl Pin for AD_B0_13 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C4_SDA_AD_B0_13;
    type Signal = SDA;
    type Module = U4;
}

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPI2C1_SCL_SD_B1_04: Daisy = Daisy::new(0x401f84cc as *mut u32, 0);
    pub const DAISY_LPI2C1_SCL_AD_B1_00: Daisy = Daisy::new(0x401f84cc as *mut u32, 1);
    pub const DAISY_LPI2C1_SDA_SD_B1_05: Daisy = Daisy::new(0x401f84d0 as *mut u32, 0);
    pub const DAISY_LPI2C1_SDA_AD_B1_01: Daisy = Daisy::new(0x401f84d0 as *mut u32, 1);
    pub const DAISY_LPI2C2_SCL_SD_B1_11: Daisy = Daisy::new(0x401f84d4 as *mut u32, 0);
    pub const DAISY_LPI2C2_SCL_B0_04: Daisy = Daisy::new(0x401f84d4 as *mut u32, 1);
    pub const DAISY_LPI2C2_SDA_SD_B1_10: Daisy = Daisy::new(0x401f84d8 as *mut u32, 0);
    pub const DAISY_LPI2C2_SDA_B0_05: Daisy = Daisy::new(0x401f84d8 as *mut u32, 1);
    pub const DAISY_LPI2C3_SCL_EMC_22: Daisy = Daisy::new(0x401f84dc as *mut u32, 0);
    pub const DAISY_LPI2C3_SCL_SD_B0_00: Daisy = Daisy::new(0x401f84dc as *mut u32, 1);
    pub const DAISY_LPI2C3_SCL_AD_B1_07: Daisy = Daisy::new(0x401f84dc as *mut u32, 2);
    pub const DAISY_LPI2C3_SDA_EMC_21: Daisy = Daisy::new(0x401f84e0 as *mut u32, 0);
    pub const DAISY_LPI2C3_SDA_SD_B0_01: Daisy = Daisy::new(0x401f84e0 as *mut u32, 1);
    pub const DAISY_LPI2C3_SDA_AD_B1_06: Daisy = Daisy::new(0x401f84e0 as *mut u32, 2);
    pub const DAISY_LPI2C4_SCL_EMC_12: Daisy = Daisy::new(0x401f84e4 as *mut u32, 0);
    pub const DAISY_LPI2C4_SCL_AD_B0_12: Daisy = Daisy::new(0x401f84e4 as *mut u32, 1);
    pub const DAISY_LPI2C4_SDA_EMC_11: Daisy = Daisy::new(0x401f84e8 as *mut u32, 0);
    pub const DAISY_LPI2C4_SDA_AD_B0_13: Daisy = Daisy::new(0x401f84e8 as *mut u32, 1);
}

use daisy::*;
