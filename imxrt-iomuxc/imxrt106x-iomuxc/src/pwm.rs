//! PWM implementation

use super::pads::{ad_b0::*, b0::*, b1::*, emc::*, sd_b0::*};
use imxrt_iomuxc::{
    consts::*,
    pwm::{Pin, A, B},
};

impl Pin for SD_B0_00 {
    const ALT: u32 = 1;
    type Output = A;
    type Module = U1; // FlexPWM1
    type Submodule = U0; // FlexPWM1
}

impl Pin for SD_B0_01 {
    const ALT: u32 = 1;
    type Output = B;
    type Module = U1; // FlexPWM1
    type Submodule = U0; // FlexPWM1
}

impl Pin for AD_B0_10 {
    const ALT: u32 = 1;
    type Output = A;
    type Module = U1; // FlexPWM1
    type Submodule = U3; // PWM3
}

impl Pin for AD_B0_11 {
    const ALT: u32 = 1;
    type Output = B;
    type Module = U1; // FlexPWM1
    type Submodule = U3; // PWM3
}

impl Pin for B0_10 {
    const ALT: u32 = 2;
    type Output = A;
    type Module = U2; // FlexPWM2
    type Submodule = U2; // FlexPWM2
}

impl Pin for B0_11 {
    const ALT: u32 = 2;
    type Output = B;
    type Module = U2; // FlexPWM2
    type Submodule = U2; // FlexPWM2
}

impl Pin for B1_01 {
    const ALT: u32 = 6;
    type Output = B;
    type Module = U1;
    type Submodule = U3;
}

impl Pin for B1_00 {
    const ALT: u32 = 6;
    type Output = A;
    type Module = U1;
    type Submodule = U3;
}

impl Pin for EMC_04 {
    const ALT: u32 = 1;
    type Output = A;
    type Module = U4;
    type Submodule = U2;
}

impl Pin for EMC_05 {
    const ALT: u32 = 1;
    type Output = B;
    type Module = U4;
    type Submodule = U2;
}

impl Pin for EMC_06 {
    const ALT: u32 = 1;
    type Output = A;
    type Module = U2;
    type Submodule = U0;
}

impl Pin for EMC_08 {
    const ALT: u32 = 1;
    type Output = A;
    type Module = U2;
    type Submodule = U1;
}
