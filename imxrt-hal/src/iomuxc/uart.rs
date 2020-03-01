//! UART pin multiplexing

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
    pub struct _5;
    impl Module for _5 {
        const IDX: usize = 5;
    }
    pub struct _6;
    impl Module for _6 {
        const IDX: usize = 6;
    }
    pub struct _7;
    impl Module for _7 {
        const IDX: usize = 7;
    }
    pub struct _8;
    impl Module for _8 {
        const IDX: usize = 8;
    }
}

pub trait Direction {}
pub struct TX;
impl Direction for TX {}
pub struct RX;
impl Direction for RX {}

pub trait Pin {
    type Direction: Direction;
    type Module: module::Module;

    /// Perform IOMUXC configurations for this pin
    fn configure(&mut self);
}

macro_rules! _rx_config {
    // Pad RX configuration and daisy selection
    ($pad_reg:ident, $daisy_reg:ident, $daisy_value:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            unsafe {
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $daisy_reg,
                    DAISY: $daisy_value
                );
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $pad_reg,
                    DSE: DSE_7_R0_7,
                    PKE: PKE_1_Pull_Keeper_Enabled,
                    PUE: PUE_1_Pull,
                    PUS: PUS_3_22K_Ohm_Pull_Up,
                    HYS: HYS_1_Hysteresis_Enabled
                );
            }
        }
    };
    // Only RX pad configuration
    ($pad_reg:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            unsafe {
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $pad_reg,
                    DSE: DSE_7_R0_7,
                    PKE: PKE_1_Pull_Keeper_Enabled,
                    PUE: PUE_1_Pull,
                    PUS: PUS_3_22K_Ohm_Pull_Up,
                    HYS: HYS_1_Hysteresis_Enabled
                );
            }
        }
    };
}

macro_rules! _tx_config {
    // TX Pad configuration and daisy selection
    ($pad_reg:ident, $daisy_reg:ident, $daisy_value:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            unsafe {
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $daisy_reg,
                    DAISY: $daisy_value
                );
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $pad_reg,
                    SRE: SRE_1_Fast_Slew_Rate,
                    DSE: DSE_3_R0_3,
                    SPEED: SPEED_3_max_200MHz
                );
            }
        }
    };
    // Only TX pad configuration
    ($pad_reg:ident) => {
        #[inline(always)]
        fn configure(&mut self) {
            unsafe {
                ral::write_reg!(
                    ral::iomuxc,
                    ral::iomuxc::IOMUXC,
                    $pad_reg,
                    SRE: SRE_1_Fast_Slew_Rate,
                    DSE: DSE_3_R0_3,
                    SPEED: SPEED_3_max_200MHz
                );
            }
        }
    };
}

use crate::iomuxc::{
    gpio::{
        GPIO_AD_B0_02, GPIO_AD_B0_03, GPIO_AD_B0_12, GPIO_AD_B0_13, GPIO_AD_B1_02, GPIO_AD_B1_03,
        GPIO_AD_B1_06, GPIO_AD_B1_07, GPIO_AD_B1_10, GPIO_AD_B1_11, GPIO_B1_00, GPIO_B1_01,
        GPIO_EMC_31, GPIO_EMC_32,
    },
    Alt2,
};

macro_rules! _impl_rx {
    // Implement a RX pad that needs daisy selection
    ($pad:ty, $pad_reg:ident, $module:ty, $daisy_reg:ident, $daisy_val:ident) => {
        impl Pin for $pad {
            type Direction = RX;
            type Module = $module;

            _rx_config!($pad_reg, $daisy_reg, $daisy_val);
        }
    };
    // Implement a RX pad
    ($pad:ty, $pad_reg:ident, $module:ty) => {
        impl Pin for $pad {
            type Direction = RX;
            type Module = $module;

            _rx_config!($pad_reg);
        }
    };
}

macro_rules! _impl_tx {
    // Implement a TX pad that needs daisy configuration
    ($pad:ty, $pad_reg:ident, $module:ty, $daisy_reg:ident, $daisy_val:ident) => {
        impl Pin for $pad {
            type Direction = TX;
            type Module = $module;

            _tx_config!($pad_reg, $daisy_reg, $daisy_val);
        }
    };
    // Implement a TX pad
    ($pad:ty, $pad_reg:ident, $module:ty) => {
        impl Pin for $pad {
            type Direction = TX;
            type Module = $module;

            _tx_config!($pad_reg);
        }
    };
}

macro_rules! uart {
    // UART TX and RX pins, both which need a daisy selection
    ($module:ty,
        tx: $tx_pad:ty, $tx_pad_reg:ident, $tx_daisy_reg:ident, $tx_daisy_val:ident,
        rx: $rx_pad:ty, $rx_pad_reg:ident, $rx_daisy_reg:ident, $rx_daisy_val:ident,) => {
        _impl_tx!($tx_pad, $tx_pad_reg, $module, $tx_daisy_reg, $tx_daisy_val);
        _impl_rx!($rx_pad, $rx_pad_reg, $module, $rx_daisy_reg, $rx_daisy_val);
    };
    // UART TX and RX pins
    ($module:ty, tx: $tx_pad:ty, $tx_pad_reg:ident, rx: $rx_pad:ty, $rx_pad_reg:ident,) => {
        _impl_tx!($tx_pad, $tx_pad_reg, $module);
        _impl_rx!($rx_pad, $rx_pad_reg, $module);
    };
}

uart! {
    module::_6,
    tx: GPIO_AD_B0_02<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B0_02, LPUART6_TX_SELECT_INPUT, GPIO_AD_B0_02_ALT2,
    rx: GPIO_AD_B0_03<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B0_03, LPUART6_RX_SELECT_INPUT, GPIO_AD_B0_03_ALT2,
}

uart! {
    module::_1,
    tx: GPIO_AD_B0_12<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B0_12,
    rx: GPIO_AD_B0_13<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B0_13,
}

uart! {
    module::_3,
    tx: GPIO_AD_B1_06<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_06, LPUART3_TX_SELECT_INPUT, GPIO_AD_B1_06_ALT2,
    rx: GPIO_AD_B1_07<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_07, LPUART3_RX_SELECT_INPUT, GPIO_AD_B1_07_ALT2,
}

uart! {
    module::_4,
    tx: GPIO_B1_00<Alt2>, SW_PAD_CTL_PAD_GPIO_B1_00, LPUART4_TX_SELECT_INPUT, GPIO_B1_00_ALT2,
    rx: GPIO_B1_01<Alt2>, SW_PAD_CTL_PAD_GPIO_B1_01, LPUART4_RX_SELECT_INPUT, GPIO_B1_01_ALT2,
}

uart! {
    module::_2,
    tx: GPIO_AD_B1_02<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_02, LPUART2_TX_SELECT_INPUT, GPIO_AD_B1_02_ALT2,
    rx: GPIO_AD_B1_03<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_03, LPUART2_RX_SELECT_INPUT, GPIO_AD_B1_03_ALT2,
}

uart! {
    module::_7,
    tx: GPIO_EMC_31<Alt2>, SW_PAD_CTL_PAD_GPIO_EMC_31, LPUART7_TX_SELECT_INPUT, GPIO_EMC_31_ALT2,
    rx: GPIO_EMC_32<Alt2>, SW_PAD_CTL_PAD_GPIO_EMC_32, LPUART7_RX_SELECT_INPUT, GPIO_EMC_32_ALT2,
}

uart! {
    module::_8,
    tx: GPIO_AD_B1_10<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_10, LPUART8_TX_SELECT_INPUT, GPIO_AD_B1_10_ALT2,
    rx: GPIO_AD_B1_11<Alt2>, SW_PAD_CTL_PAD_GPIO_AD_B1_11, LPUART8_RX_SELECT_INPUT, GPIO_AD_B1_11_ALT2,
}
