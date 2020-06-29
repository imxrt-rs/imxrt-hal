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
    fn LPI2C1();
    fn LPI2C2();
    fn LPSPI1();
    fn LPSPI2();
    fn FLEXRAM();
    fn KPP();
    fn Reserved56();
    fn GPR_IRQ();
    fn Reserved58();
    fn Reserved59();
    fn Reserved60();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn Reserved68();
    fn TRNG();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU();
    fn Reserved78();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY();
    fn Reserved82();
    fn ADC1();
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
    fn Reserved102();
    fn Reserved103();
    fn GPIO5_Combined_0_15();
    fn GPIO5_Combined_16_31();
    fn FLEXIO1();
    fn Reserved107();
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
    fn Reserved128();
    fn USB_OTG1();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn Reserved143();
    fn Reserved144();
    fn ENC1();
    fn Reserved147();
    fn Reserved148();
    fn TMR1();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[cfg_attr(target_arch = "arm", link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 134] = [
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector {
        _handler: Reserved56,
    },
    Vector { _handler: GPR_IRQ },
    Vector {
        _handler: Reserved58,
    },
    Vector {
        _handler: Reserved59,
    },
    Vector {
        _handler: Reserved60,
    },
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
    Vector { _reserved: 0 },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector { _handler: PMU },
    Vector {
        _handler: Reserved78,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY },
    Vector {
        _handler: Reserved82,
    },
    Vector { _handler: ADC1 },
    Vector { _reserved: 0 },
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
        _handler: Reserved102,
    },
    Vector {
        _handler: Reserved103,
    },
    Vector {
        _handler: GPIO5_Combined_0_15,
    },
    Vector {
        _handler: GPIO5_Combined_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector {
        _handler: Reserved107,
    },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: Reserved128,
    },
    Vector { _handler: USB_OTG1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: Reserved143,
    },
    Vector {
        _handler: Reserved144,
    },
    Vector { _handler: ENC1 },
    Vector { _reserved: 0 },
    Vector {
        _handler: Reserved147,
    },
    Vector {
        _handler: Reserved148,
    },
    Vector { _handler: TMR1 },
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
    /// 28:
    LPI2C1 = 28,
    /// 29:
    LPI2C2 = 29,
    /// 32:
    LPSPI1 = 32,
    /// 33:
    LPSPI2 = 33,
    /// 38:
    FLEXRAM = 38,
    /// 39:
    KPP = 39,
    /// 40:
    Reserved56 = 40,
    /// 41:
    GPR_IRQ = 41,
    /// 42:
    Reserved58 = 42,
    /// 43:
    Reserved59 = 43,
    /// 44:
    Reserved60 = 44,
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
    PMU = 61,
    /// 62:
    Reserved78 = 62,
    /// 63:
    TEMP_LOW_HIGH = 63,
    /// 64:
    TEMP_PANIC = 64,
    /// 65:
    USB_PHY = 65,
    /// 66:
    Reserved82 = 66,
    /// 67:
    ADC1 = 67,
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
    Reserved102 = 86,
    /// 87:
    Reserved103 = 87,
    /// 88:
    GPIO5_Combined_0_15 = 88,
    /// 89:
    GPIO5_Combined_16_31 = 89,
    /// 90:
    FLEXIO1 = 90,
    /// 91:
    Reserved107 = 91,
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
    /// 112:
    Reserved128 = 112,
    /// 113:
    USB_OTG1 = 113,
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
    /// 127:
    Reserved143 = 127,
    /// 128:
    Reserved144 = 128,
    /// 129:
    ENC1 = 129,
    /// 131:
    Reserved147 = 131,
    /// 132:
    Reserved148 = 132,
    /// 133:
    TMR1 = 133,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
