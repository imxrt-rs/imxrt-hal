//! I2C pin implementations

use super::pads::{ad::*, gpio::*, sd::*};
use crate::{
    consts::*,
    i2c::{Pin, SCL, SDA},
    Daisy,
};

//
// I2C1
//

// SCL

impl Pin for AD_14 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C1_SCL_AD_14;
    type Signal = SCL;
    type Module = U1;
}

impl Pin for SD_06 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C1_SCL_SD_06;
    type Signal = SCL;
    type Module = U1;
}

impl Pin for GPIO_12 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C1_SCL_12;
    type Signal = SCL;
    type Module = U1;
}

impl Pin for GPIO_02 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C1_SCL_02;
    type Signal = SCL;
    type Module = U1;
}

// SDA

impl Pin for AD_13 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C1_SDA_AD_13;
    type Signal = SDA;
    type Module = U1;
}

impl Pin for SD_05 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C1_SDA_SD_05;
    type Signal = SDA;
    type Module = U1;
}

impl Pin for GPIO_11 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C1_SDA_11;
    type Signal = SDA;
    type Module = U1;
}

impl Pin for GPIO_01 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C1_SDA_01;
    type Signal = SDA;
    type Module = U1;
}

//
// I2C2
//

// SCL

impl Pin for AD_08 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C2_SCL_AD_08;
    type Signal = SCL;
    type Module = U2;
}

impl Pin for SD_08 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C2_SCL_SD_08;
    type Signal = SCL;
    type Module = U2;
}

impl Pin for AD_02 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C2_SCL_AD_02;
    type Signal = SCL;
    type Module = U2;
}

impl Pin for GPIO_10 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C2_SCL_10;
    type Signal = SCL;
    type Module = U2;
}

// SDA

impl Pin for AD_07 {
    const ALT: u32 = 0;
    const DAISY: Daisy = DAISY_LPI2C2_SDA_AD_07;
    type Signal = SDA;
    type Module = U2;
}

impl Pin for SD_07 {
    const ALT: u32 = 1;
    const DAISY: Daisy = DAISY_LPI2C2_SDA_SD_07;
    type Signal = SDA;
    type Module = U2;
}

impl Pin for AD_01 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C2_SDA_AD_01;
    type Signal = SDA;
    type Module = U2;
}

impl Pin for GPIO_09 {
    const ALT: u32 = 3;
    const DAISY: Daisy = DAISY_LPI2C2_SDA_09;
    type Signal = SDA;
    type Module = U2;
}

mod daisy {
    #![allow(unused)]

    use super::Daisy;
    pub const DAISY_LPI2C1_HREQ_AD_06: Daisy = Daisy::new(0x401f81bc as *mut u32, 0);
    pub const DAISY_LPI2C1_HREQ_10: Daisy = Daisy::new(0x401f81bc as *mut u32, 1);
    pub const DAISY_LPI2C1_SCL_AD_14: Daisy = Daisy::new(0x401f81c0 as *mut u32, 0);
    pub const DAISY_LPI2C1_SCL_SD_06: Daisy = Daisy::new(0x401f81c0 as *mut u32, 1);
    pub const DAISY_LPI2C1_SCL_12: Daisy = Daisy::new(0x401f81c0 as *mut u32, 2);
    pub const DAISY_LPI2C1_SCL_02: Daisy = Daisy::new(0x401f81c0 as *mut u32, 3);
    pub const DAISY_LPI2C1_SDA_AD_13: Daisy = Daisy::new(0x401f81c4 as *mut u32, 0);
    pub const DAISY_LPI2C1_SDA_SD_05: Daisy = Daisy::new(0x401f81c4 as *mut u32, 1);
    pub const DAISY_LPI2C1_SDA_11: Daisy = Daisy::new(0x401f81c4 as *mut u32, 2);
    pub const DAISY_LPI2C1_SDA_01: Daisy = Daisy::new(0x401f81c4 as *mut u32, 3);
    pub const DAISY_LPI2C2_SCL_AD_08: Daisy = Daisy::new(0x401f81c8 as *mut u32, 0);
    pub const DAISY_LPI2C2_SCL_AD_02: Daisy = Daisy::new(0x401f81c8 as *mut u32, 1);
    pub const DAISY_LPI2C2_SCL_SD_08: Daisy = Daisy::new(0x401f81c8 as *mut u32, 2);
    pub const DAISY_LPI2C2_SCL_10: Daisy = Daisy::new(0x401f81c8 as *mut u32, 3);
    pub const DAISY_LPI2C2_SDA_AD_07: Daisy = Daisy::new(0x401f81cc as *mut u32, 0);
    pub const DAISY_LPI2C2_SDA_AD_01: Daisy = Daisy::new(0x401f81cc as *mut u32, 1);
    pub const DAISY_LPI2C2_SDA_SD_07: Daisy = Daisy::new(0x401f81cc as *mut u32, 2);
    pub const DAISY_LPI2C2_SDA_09: Daisy = Daisy::new(0x401f81cc as *mut u32, 3);
}
use daisy::*;
