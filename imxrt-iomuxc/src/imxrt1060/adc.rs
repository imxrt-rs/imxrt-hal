//! ADC pin implementations
//!
//! Implementation derived from Table 66-2: ADC External Signals
//! from the iMXRT1060 Reference Manual, Rev 2. There is a similar
//! information available in Table 10-1: Muxing Options, in the IOMUXC
//! section of the reference manual.

use super::{ad_b0::*, ad_b1::*};
use crate::{
    adc::{Pin, ADC1, ADC2},
    consts::*,
};

//
// ADC1
//

impl Pin<ADC1> for AD_B1_11 {
    type Input = U0;
}

impl Pin<ADC1> for AD_B0_12 {
    type Input = U1;
}

impl Pin<ADC1> for AD_B0_13 {
    type Input = U2;
}

impl Pin<ADC1> for AD_B0_14 {
    type Input = U3;
}

impl Pin<ADC1> for AD_B0_15 {
    type Input = U4;
}

impl Pin<ADC1> for AD_B1_00 {
    type Input = U5;
}

impl Pin<ADC1> for AD_B1_01 {
    type Input = U6;
}

impl Pin<ADC1> for AD_B1_02 {
    type Input = U7;
}

impl Pin<ADC1> for AD_B1_03 {
    type Input = U8;
}

impl Pin<ADC1> for AD_B1_04 {
    type Input = U9;
}

impl Pin<ADC1> for AD_B1_05 {
    type Input = U10;
}

impl Pin<ADC1> for AD_B1_06 {
    type Input = U11;
}

impl Pin<ADC1> for AD_B1_07 {
    type Input = U12;
}

impl Pin<ADC1> for AD_B1_08 {
    type Input = U13;
}

impl Pin<ADC1> for AD_B1_09 {
    type Input = U14;
}

impl Pin<ADC1> for AD_B1_10 {
    type Input = U15;
}

//
// ADC2
//

impl Pin<ADC2> for AD_B1_11 {
    type Input = U0;
}

impl Pin<ADC2> for AD_B1_12 {
    type Input = U1;
}

impl Pin<ADC2> for AD_B1_13 {
    type Input = U2;
}

impl Pin<ADC2> for AD_B1_14 {
    type Input = U3;
}

impl Pin<ADC2> for AD_B1_15 {
    type Input = U4;
}

impl Pin<ADC2> for AD_B1_00 {
    type Input = U5;
}

impl Pin<ADC2> for AD_B1_01 {
    type Input = U6;
}

impl Pin<ADC2> for AD_B1_02 {
    type Input = U7;
}

impl Pin<ADC2> for AD_B1_03 {
    type Input = U8;
}

impl Pin<ADC2> for AD_B1_04 {
    type Input = U9;
}

impl Pin<ADC2> for AD_B1_05 {
    type Input = U10;
}

impl Pin<ADC2> for AD_B1_06 {
    type Input = U11;
}

impl Pin<ADC2> for AD_B1_07 {
    type Input = U12;
}

impl Pin<ADC2> for AD_B1_08 {
    type Input = U13;
}

impl Pin<ADC2> for AD_B1_09 {
    type Input = U14;
}

impl Pin<ADC2> for AD_B1_10 {
    type Input = U15;
}
