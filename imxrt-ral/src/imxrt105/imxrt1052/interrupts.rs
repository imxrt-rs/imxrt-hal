extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn CAN1();
    fn CAN2();
    fn FLEXRAM();
    fn KPP();
    fn TSC_DIG();
    fn GPR_IRQ();
    fn LCDIF();
    fn CSI();
    fn PXP();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn Reserved68();
    fn TRNG();
    fn SJC();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU_EVENT();
    fn Reserved78();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY1();
    fn USB_PHY2();
    fn ADC1();
    fn ADC2();
    fn DCDC();
    fn Reserved86();
    fn Reserved87();
    fn GPIO1_INT0();
    fn GPIO1_INT1();
    fn GPIO1_INT2();
    fn GPIO1_INT3();
    fn GPIO1_INT4();
    fn GPIO1_INT5();
    fn GPIO1_INT6();
    fn GPIO1_INT7();
    fn GPIO1_Combined_0_15();
    fn GPIO1_Combined_16_31();
    fn GPIO2_Combined_0_15();
    fn GPIO2_Combined_16_31();
    fn GPIO3_Combined_0_15();
    fn GPIO3_Combined_16_31();
    fn GPIO4_Combined_0_15();
    fn GPIO4_Combined_16_31();
    fn GPIO5_Combined_0_15();
    fn GPIO5_Combined_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG();
    fn EWM();
    fn CCM_1();
    fn CCM_2();
    fn GPC();
    fn SRC();
    fn Reserved115();
    fn GPT1();
    fn GPT2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn Reserved123();
    fn FLEXSPI();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_Timer();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn Reserved143();
    fn Reserved144();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 152] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: CTI0_ERROR,
    },
    Vector {
        _handler: CTI1_ERROR,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector { _handler: TSC_DIG },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: LCDIF },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_WRAPPER,
    },
    Vector {
        _handler: SNVS_HP_WRAPPER_TZ,
    },
    Vector {
        _handler: SNVS_LP_WRAPPER,
    },
    Vector { _handler: CSU },
    Vector { _handler: DCP },
    Vector { _handler: DCP_VMI },
    Vector {
        _handler: Reserved68,
    },
    Vector { _handler: TRNG },
    Vector { _handler: SJC },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: PMU_EVENT,
    },
    Vector {
        _handler: Reserved78,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY1 },
    Vector { _handler: USB_PHY2 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector {
        _handler: Reserved86,
    },
    Vector {
        _handler: Reserved87,
    },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT7,
    },
    Vector {
        _handler: GPIO1_Combined_0_15,
    },
    Vector {
        _handler: GPIO1_Combined_16_31,
    },
    Vector {
        _handler: GPIO2_Combined_0_15,
    },
    Vector {
        _handler: GPIO2_Combined_16_31,
    },
    Vector {
        _handler: GPIO3_Combined_0_15,
    },
    Vector {
        _handler: GPIO3_Combined_16_31,
    },
    Vector {
        _handler: GPIO4_Combined_0_15,
    },
    Vector {
        _handler: GPIO4_Combined_16_31,
    },
    Vector {
        _handler: GPIO5_Combined_0_15,
    },
    Vector {
        _handler: GPIO5_Combined_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: EWM },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: GPC },
    Vector { _handler: SRC },
    Vector {
        _handler: Reserved115,
    },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector {
        _handler: Reserved123,
    },
    Vector { _handler: FLEXSPI },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_Timer,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _handler: PIT },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector {
        _handler: Reserved143,
    },
    Vector {
        _handler: Reserved144,
    },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0:
    DMA0_DMA16 = 0,
    /// 1:
    DMA1_DMA17 = 1,
    /// 2:
    DMA2_DMA18 = 2,
    /// 3:
    DMA3_DMA19 = 3,
    /// 4:
    DMA4_DMA20 = 4,
    /// 5:
    DMA5_DMA21 = 5,
    /// 6:
    DMA6_DMA22 = 6,
    /// 7:
    DMA7_DMA23 = 7,
    /// 8:
    DMA8_DMA24 = 8,
    /// 9:
    DMA9_DMA25 = 9,
    /// 10:
    DMA10_DMA26 = 10,
    /// 11:
    DMA11_DMA27 = 11,
    /// 12:
    DMA12_DMA28 = 12,
    /// 13:
    DMA13_DMA29 = 13,
    /// 14:
    DMA14_DMA30 = 14,
    /// 15:
    DMA15_DMA31 = 15,
    /// 16:
    DMA_ERROR = 16,
    /// 17:
    CTI0_ERROR = 17,
    /// 18:
    CTI1_ERROR = 18,
    /// 19:
    CORE = 19,
    /// 20:
    LPUART1 = 20,
    /// 21:
    LPUART2 = 21,
    /// 22:
    LPUART3 = 22,
    /// 23:
    LPUART4 = 23,
    /// 24:
    LPUART5 = 24,
    /// 25:
    LPUART6 = 25,
    /// 26:
    LPUART7 = 26,
    /// 27:
    LPUART8 = 27,
    /// 28:
    LPI2C1 = 28,
    /// 29:
    LPI2C2 = 29,
    /// 30:
    LPI2C3 = 30,
    /// 31:
    LPI2C4 = 31,
    /// 32:
    LPSPI1 = 32,
    /// 33:
    LPSPI2 = 33,
    /// 34:
    LPSPI3 = 34,
    /// 35:
    LPSPI4 = 35,
    /// 36:
    CAN1 = 36,
    /// 37:
    CAN2 = 37,
    /// 38:
    FLEXRAM = 38,
    /// 39:
    KPP = 39,
    /// 40:
    TSC_DIG = 40,
    /// 41:
    GPR_IRQ = 41,
    /// 42:
    LCDIF = 42,
    /// 43:
    CSI = 43,
    /// 44:
    PXP = 44,
    /// 45:
    WDOG2 = 45,
    /// 46:
    SNVS_HP_WRAPPER = 46,
    /// 47:
    SNVS_HP_WRAPPER_TZ = 47,
    /// 48:
    SNVS_LP_WRAPPER = 48,
    /// 49:
    CSU = 49,
    /// 50:
    DCP = 50,
    /// 51:
    DCP_VMI = 51,
    /// 52:
    Reserved68 = 52,
    /// 53:
    TRNG = 53,
    /// 54:
    SJC = 54,
    /// 55:
    BEE = 55,
    /// 56:
    SAI1 = 56,
    /// 57:
    SAI2 = 57,
    /// 58:
    SAI3_RX = 58,
    /// 59:
    SAI3_TX = 59,
    /// 60:
    SPDIF = 60,
    /// 61:
    PMU_EVENT = 61,
    /// 62:
    Reserved78 = 62,
    /// 63:
    TEMP_LOW_HIGH = 63,
    /// 64:
    TEMP_PANIC = 64,
    /// 65:
    USB_PHY1 = 65,
    /// 66:
    USB_PHY2 = 66,
    /// 67:
    ADC1 = 67,
    /// 68:
    ADC2 = 68,
    /// 69:
    DCDC = 69,
    /// 70:
    Reserved86 = 70,
    /// 71:
    Reserved87 = 71,
    /// 72:
    GPIO1_INT0 = 72,
    /// 73:
    GPIO1_INT1 = 73,
    /// 74:
    GPIO1_INT2 = 74,
    /// 75:
    GPIO1_INT3 = 75,
    /// 76:
    GPIO1_INT4 = 76,
    /// 77:
    GPIO1_INT5 = 77,
    /// 78:
    GPIO1_INT6 = 78,
    /// 79:
    GPIO1_INT7 = 79,
    /// 80:
    GPIO1_Combined_0_15 = 80,
    /// 81:
    GPIO1_Combined_16_31 = 81,
    /// 82:
    GPIO2_Combined_0_15 = 82,
    /// 83:
    GPIO2_Combined_16_31 = 83,
    /// 84:
    GPIO3_Combined_0_15 = 84,
    /// 85:
    GPIO3_Combined_16_31 = 85,
    /// 86:
    GPIO4_Combined_0_15 = 86,
    /// 87:
    GPIO4_Combined_16_31 = 87,
    /// 88:
    GPIO5_Combined_0_15 = 88,
    /// 89:
    GPIO5_Combined_16_31 = 89,
    /// 90:
    FLEXIO1 = 90,
    /// 91:
    FLEXIO2 = 91,
    /// 92:
    WDOG1 = 92,
    /// 93:
    RTWDOG = 93,
    /// 94:
    EWM = 94,
    /// 95:
    CCM_1 = 95,
    /// 96:
    CCM_2 = 96,
    /// 97:
    GPC = 97,
    /// 98:
    SRC = 98,
    /// 99:
    Reserved115 = 99,
    /// 100:
    GPT1 = 100,
    /// 101:
    GPT2 = 101,
    /// 102:
    PWM1_0 = 102,
    /// 103:
    PWM1_1 = 103,
    /// 104:
    PWM1_2 = 104,
    /// 105:
    PWM1_3 = 105,
    /// 106:
    PWM1_FAULT = 106,
    /// 107:
    Reserved123 = 107,
    /// 108:
    FLEXSPI = 108,
    /// 109:
    SEMC = 109,
    /// 110:
    USDHC1 = 110,
    /// 111:
    USDHC2 = 111,
    /// 112:
    USB_OTG2 = 112,
    /// 113:
    USB_OTG1 = 113,
    /// 114:
    ENET = 114,
    /// 115:
    ENET_1588_Timer = 115,
    /// 116:
    XBAR1_IRQ_0_1 = 116,
    /// 117:
    XBAR1_IRQ_2_3 = 117,
    /// 118:
    ADC_ETC_IRQ0 = 118,
    /// 119:
    ADC_ETC_IRQ1 = 119,
    /// 120:
    ADC_ETC_IRQ2 = 120,
    /// 121:
    ADC_ETC_ERROR_IRQ = 121,
    /// 122:
    PIT = 122,
    /// 123:
    ACMP1 = 123,
    /// 124:
    ACMP2 = 124,
    /// 125:
    ACMP3 = 125,
    /// 126:
    ACMP4 = 126,
    /// 127:
    Reserved143 = 127,
    /// 128:
    Reserved144 = 128,
    /// 129:
    ENC1 = 129,
    /// 130:
    ENC2 = 130,
    /// 131:
    ENC3 = 131,
    /// 132:
    ENC4 = 132,
    /// 133:
    TMR1 = 133,
    /// 134:
    TMR2 = 134,
    /// 135:
    TMR3 = 135,
    /// 136:
    TMR4 = 136,
    /// 137:
    PWM2_0 = 137,
    /// 138:
    PWM2_1 = 138,
    /// 139:
    PWM2_2 = 139,
    /// 140:
    PWM2_3 = 140,
    /// 141:
    PWM2_FAULT = 141,
    /// 142:
    PWM3_0 = 142,
    /// 143:
    PWM3_1 = 143,
    /// 144:
    PWM3_2 = 144,
    /// 145:
    PWM3_3 = 145,
    /// 146:
    PWM3_FAULT = 146,
    /// 147:
    PWM4_0 = 147,
    /// 148:
    PWM4_1 = 148,
    /// 149:
    PWM4_2 = 149,
    /// 150:
    PWM4_3 = 150,
    /// 151:
    PWM4_FAULT = 151,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
