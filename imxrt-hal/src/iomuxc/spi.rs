//! SPI pin multiplexing

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
pub struct SCK;
impl Wire for SCK {}
pub struct SDO;
impl Wire for SDO {}
pub struct SDI;
impl Wire for SDI {}
pub struct PCS0;
impl Wire for PCS0 {}

pub trait Pin {
    type Wire: Wire;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(&mut self);
}

use crate::iomuxc::{
    gpio::{
        GPIO_AD_B0_00, GPIO_AD_B0_01, GPIO_AD_B0_02, GPIO_AD_B0_03, GPIO_B0_00, GPIO_B0_01,
        GPIO_B0_02, GPIO_B0_03, GPIO_EMC_30, GPIO_SD_B0_00, GPIO_SD_B0_01, GPIO_SD_B0_02,
        GPIO_SD_B0_03, GPIO_SD_B1_06, GPIO_SD_B1_07, GPIO_SD_B1_08, GPIO_SD_B1_09,
    },
    Alt3, Alt4, Alt7,
};
use crate::ral;

macro_rules! pin_config {
    ($reg:ident) => {
        ral::write_reg!(
            ral::iomuxc,
            ral::iomuxc::IOMUXC,
            $reg,
            DSE: DSE_7_R0_7,
            SPEED: SPEED_2_medium_100MHz
        )
    };
}

//
// Note: The pad mapping below is *not* complete
//

// SPI 1

impl Pin for GPIO_SD_B0_00<Alt4> {
    type Wire = SCK;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_00);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI1_SCK_SELECT_INPUT,
                DAISY: GPIO_SD_B0_00_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B0_02<Alt4> {
    type Wire = SDO;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_02);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI1_SDO_SELECT_INPUT,
                DAISY: GPIO_SD_B0_02_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B0_03<Alt4> {
    type Wire = SDI;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_03);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI1_SDI_SELECT_INPUT,
                DAISY: GPIO_SD_B0_03_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B0_01<Alt4> {
    type Wire = PCS0;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B0_01);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI1_PCS0_SELECT_INPUT,
                DAISY: GPIO_SD_B0_01_ALT4
            );
        }
    }
}

impl Pin for GPIO_EMC_30<Alt3> {
    type Wire = PCS0;
    type Module = module::_1;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_EMC_30);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI1_PCS0_SELECT_INPUT,
                DAISY: GPIO_EMC_30_ALT3
            );
        }
    }
}

// SPI 2

impl Pin for GPIO_SD_B1_07<Alt4> {
    type Wire = SCK;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B1_07);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI2_SCK_SELECT_INPUT,
                DAISY: GPIO_SD_B1_07_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B1_08<Alt4> {
    type Wire = SDO;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B1_08);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI2_SDO_SELECT_INPUT,
                DAISY: GPIO_SD_B1_08_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B1_09<Alt4> {
    type Wire = SDI;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B1_09);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI2_SDI_SELECT_INPUT,
                DAISY: GPIO_SD_B1_09_ALT4
            );
        }
    }
}

impl Pin for GPIO_SD_B1_06<Alt4> {
    type Wire = PCS0;
    type Module = module::_2;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_SD_B1_06);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI2_PCS0_SELECT_INPUT,
                DAISY: GPIO_SD_B1_06_ALT4
            );
        }
    }
}

// SPI 3

impl Pin for GPIO_AD_B0_00<Alt7> {
    type Wire = SCK;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_00);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI3_SCK_SELECT_INPUT,
                DAISY: GPIO_AD_B0_00_ALT7
            );
        }
    }
}

impl Pin for GPIO_AD_B0_01<Alt7> {
    type Wire = SDO;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_01);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI3_SDO_SELECT_INPUT,
                DAISY: GPIO_AD_B0_01_ALT7
            );
        }
    }
}

impl Pin for GPIO_AD_B0_02<Alt7> {
    type Wire = SDI;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_02);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI3_SDI_SELECT_INPUT,
                DAISY: GPIO_AD_B0_02_ALT7
            );
        }
    }
}

impl Pin for GPIO_AD_B0_03<Alt7> {
    type Wire = PCS0;
    type Module = module::_3;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_AD_B0_03);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI3_PCS0_SELECT_INPUT,
                DAISY: GPIO_AD_B0_03_ALT7
            );
        }
    }
}

// SPI 4

impl Pin for GPIO_B0_03<Alt3> {
    type Wire = SCK;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_B0_03);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI4_SCK_SELECT_INPUT,
                DAISY: GPIO_B0_03_ALT3
            );
        }
    }
}

impl Pin for GPIO_B0_02<Alt3> {
    type Wire = SDO;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_B0_02);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI4_SDO_SELECT_INPUT,
                DAISY: GPIO_B0_02_ALT3
            );
        }
    }
}

impl Pin for GPIO_B0_01<Alt3> {
    type Wire = SDI;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_B0_01);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI4_SDI_SELECT_INPUT,
                DAISY: GPIO_B0_01_ALT3
            );
        }
    }
}

impl Pin for GPIO_B0_00<Alt3> {
    type Wire = PCS0;
    type Module = module::_4;

    #[inline(always)]
    fn configure(&mut self) {
        self.sion_enable();
        unsafe {
            pin_config!(SW_PAD_CTL_PAD_GPIO_B0_00);
            ral::write_reg!(
                ral::iomuxc,
                ral::iomuxc::IOMUXC,
                LPSPI4_PCS0_SELECT_INPUT,
                DAISY: GPIO_B0_00_ALT3
            );
        }
    }
}
