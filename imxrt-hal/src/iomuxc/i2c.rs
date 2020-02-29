//! I2C pin multiplexing

use crate::ral;

pub mod module {
    pub trait Module {
        const IDX: usize;
    }
    pub struct _1;
    impl Module for _1 {
        const IDX: usize = 1;
    }
    pub struct _2;
    impl Module for _2 {
        const IDX: usize = 2;
    }
    pub struct _3;
    impl Module for _3 {
        const IDX: usize = 3;
    }
    pub struct _4;
    impl Module for _4 {
        const IDX: usize = 4;
    }
}

pub trait Wire {}
pub struct SCL;
impl Wire for SCL {}
pub struct SDA;
impl Wire for SDA {}

pub trait Pin {
    type Wire: Wire;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(self);
}

use crate::iomuxc::{
    gpio::{
        GPIO_AD_B0_12, GPIO_AD_B0_13, GPIO_AD_B1_00, GPIO_AD_B1_01, GPIO_AD_B1_06, GPIO_AD_B1_07,
        GPIO_SD_B0_00, GPIO_SD_B0_01,
    },
    Alt0, Alt1, Alt2, Alt3,
};

/// Writes the pad configuration settings at `reg`
///
/// This macro must be wrapped in an `unsafe` block.
macro_rules! pad_config {
    ($reg:ident) => {
        ral::write_reg!(
            ral::iomuxc,
            ral::iomuxc::IOMUXC,
            $reg,
            ODE: ODE_1_Open_Drain_Enabled,
            SRE: SRE_1_Fast_Slew_Rate,
            DSE: DSE_4_R0_4,
            SPEED: SPEED_2_medium_100MHz,
            PKE: PKE_1_Pull_Keeper_Enabled,
            PUE: PUE_1_Pull,
            PUS: PUS_3_22K_Ohm_Pull_Up
        )
    };
}

// Here's how you should implement and review a pin...
//
// 1. the pin name at (A) must resemble the SW_PAD_CTL_PAD_* symbol at (D)
// 2. the combination of (B) and (C) should be in the *_*_SELECT_INPUT symbol at (E)
// 3. the name and alternative of (A) should be present in the value at (F)
//
// A correct step 3 will not compile if step 2 is incorrect, since (F) is dependent
// on (E). So, we should double-check steps 1 and 2 if things change.
impl Pin for GPIO_AD_B1_07<Alt1> /* (A) */ {
    type Wire = SCL; // (B)
    type Module = module::_3; // (C)
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B1_07); // (D)
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C3_SCL_SELECT_INPUT,   // (E)
                DAISY: GPIO_AD_B1_07_ALT1  // (F)
            );
        };
    }
}

impl Pin for GPIO_AD_B1_06<Alt1> {
    type Wire = SDA;
    type Module = module::_3;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B1_06);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C3_SDA_SELECT_INPUT,
                DAISY: GPIO_AD_B1_06_ALT1
            );
        };
    }
}

impl Pin for GPIO_AD_B1_01<Alt3> {
    type Wire = SDA;
    type Module = module::_1;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B1_01);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C1_SDA_SELECT_INPUT,
                DAISY: GPIO_AD_B1_01_ALT3
            );
        };
    }
}

impl Pin for GPIO_AD_B1_00<Alt3> {
    type Wire = SCL;
    type Module = module::_1;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B1_00);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C1_SCL_SELECT_INPUT,
                DAISY: GPIO_AD_B1_00_ALT3
            );
        };
    }
}

impl Pin for GPIO_AD_B0_12<Alt0> {
    type Wire = SCL;
    type Module = module::_4;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_12);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C4_SCL_SELECT_INPUT,
                DAISY: GPIO_AD_B0_12_ALT0
            );
        };
    }
}

impl Pin for GPIO_AD_B0_13<Alt0> {
    type Wire = SDA;
    type Module = module::_4;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_13);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C4_SDA_SELECT_INPUT,
                DAISY: GPIO_AD_B0_13_ALT0
            );
        };
    }
}

impl Pin for GPIO_SD_B0_01<Alt2> {
    type Wire = SDA;
    type Module = module::_3;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_01);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C3_SDA_SELECT_INPUT,
                DAISY: GPIO_SD_B0_01_ALT2
            );
        };
    }
}

impl Pin for GPIO_SD_B0_00<Alt2> {
    type Wire = SCL;
    type Module = module::_3;
    #[inline(always)]
    fn configure(self) {
        self.sion_enable();
        unsafe {
            pad_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_00);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPI2C3_SCL_SELECT_INPUT,
                DAISY: GPIO_SD_B0_00_ALT2
            );
        };
    }
}
