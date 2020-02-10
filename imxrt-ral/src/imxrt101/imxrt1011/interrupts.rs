extern crate bare_metal;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn DMA4();
    fn DMA5();
    fn DMA6();
    fn DMA7();
    fn DMA8();
    fn DMA9();
    fn DMA10();
    fn DMA11();
    fn DMA12();
    fn DMA13();
    fn DMA14();
    fn DMA15();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn PIT();
    fn USB_OTG1();
    fn FLEXSPI();
    fn FLEXRAM();
    fn LPI2C1();
    fn LPI2C2();
    fn GPT1();
    fn GPT2();
    fn LPSPI1();
    fn LPSPI2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn KPP();
    fn SRC();
    fn GPR_IRQ();
    fn CCM_1();
    fn CCM_2();
    fn EWM();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn Reserved68();
    fn TRNG();
    fn Reserved70();
    fn Reserved71();
    fn SAI1();
    fn RTWDOG();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU();
    fn XBAR1_IRQ_0_1_2_3();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY();
    fn GPC();
    fn ADC1();
    fn FLEXIO1();
    fn DCDC();
    fn GPIO1_Combined_0_15();
    fn GPIO1_Combined_16_31();
    fn GPIO2_Combined_0_15();
    fn GPIO5_Combined_0_15();
    fn WDOG1();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_IRQ3();
    fn ADC_ETC_ERROR_IRQ();
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
pub static __INTERRUPTS: [Vector; 80] = [
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _handler: DMA4 },
    Vector { _handler: DMA5 },
    Vector { _handler: DMA6 },
    Vector { _handler: DMA7 },
    Vector { _handler: DMA8 },
    Vector { _handler: DMA9 },
    Vector { _handler: DMA10 },
    Vector { _handler: DMA11 },
    Vector { _handler: DMA12 },
    Vector { _handler: DMA13 },
    Vector { _handler: DMA14 },
    Vector { _handler: DMA15 },
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
    Vector { _handler: PIT },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: FLEXSPI },
    Vector { _handler: FLEXRAM },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: KPP },
    Vector { _handler: SRC },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: EWM },
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
    Vector {
        _handler: Reserved70,
    },
    Vector {
        _handler: Reserved71,
    },
    Vector { _handler: SAI1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector { _handler: PMU },
    Vector {
        _handler: XBAR1_IRQ_0_1_2_3,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY },
    Vector { _handler: GPC },
    Vector { _handler: ADC1 },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: DCDC },
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
        _handler: GPIO5_Combined_0_15,
    },
    Vector { _handler: WDOG1 },
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
        _handler: ADC_ETC_IRQ3,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
];

/// Available interrupts for this device
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0:
    DMA0 = 0,
    /// 1:
    DMA1 = 1,
    /// 2:
    DMA2 = 2,
    /// 3:
    DMA3 = 3,
    /// 4:
    DMA4 = 4,
    /// 5:
    DMA5 = 5,
    /// 6:
    DMA6 = 6,
    /// 7:
    DMA7 = 7,
    /// 8:
    DMA8 = 8,
    /// 9:
    DMA9 = 9,
    /// 10:
    DMA10 = 10,
    /// 11:
    DMA11 = 11,
    /// 12:
    DMA12 = 12,
    /// 13:
    DMA13 = 13,
    /// 14:
    DMA14 = 14,
    /// 15:
    DMA15 = 15,
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
    PIT = 24,
    /// 25:
    USB_OTG1 = 25,
    /// 26:
    FLEXSPI = 26,
    /// 27:
    FLEXRAM = 27,
    /// 28:
    LPI2C1 = 28,
    /// 29:
    LPI2C2 = 29,
    /// 30:
    GPT1 = 30,
    /// 31:
    GPT2 = 31,
    /// 32:
    LPSPI1 = 32,
    /// 33:
    LPSPI2 = 33,
    /// 34:
    PWM1_0 = 34,
    /// 35:
    PWM1_1 = 35,
    /// 36:
    PWM1_2 = 36,
    /// 37:
    PWM1_3 = 37,
    /// 38:
    PWM1_FAULT = 38,
    /// 39:
    KPP = 39,
    /// 40:
    SRC = 40,
    /// 41:
    GPR_IRQ = 41,
    /// 42:
    CCM_1 = 42,
    /// 43:
    CCM_2 = 43,
    /// 44:
    EWM = 44,
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
    Reserved70 = 54,
    /// 55:
    Reserved71 = 55,
    /// 56:
    SAI1 = 56,
    /// 57:
    RTWDOG = 57,
    /// 58:
    SAI3_RX = 58,
    /// 59:
    SAI3_TX = 59,
    /// 60:
    SPDIF = 60,
    /// 61:
    PMU = 61,
    /// 62:
    XBAR1_IRQ_0_1_2_3 = 62,
    /// 63:
    TEMP_LOW_HIGH = 63,
    /// 64:
    TEMP_PANIC = 64,
    /// 65:
    USB_PHY = 65,
    /// 66:
    GPC = 66,
    /// 67:
    ADC1 = 67,
    /// 68:
    FLEXIO1 = 68,
    /// 69:
    DCDC = 69,
    /// 70:
    GPIO1_Combined_0_15 = 70,
    /// 71:
    GPIO1_Combined_16_31 = 71,
    /// 72:
    GPIO2_Combined_0_15 = 72,
    /// 73:
    GPIO5_Combined_0_15 = 73,
    /// 74:
    WDOG1 = 74,
    /// 75:
    ADC_ETC_IRQ0 = 75,
    /// 76:
    ADC_ETC_IRQ1 = 76,
    /// 77:
    ADC_ETC_IRQ2 = 77,
    /// 78:
    ADC_ETC_IRQ3 = 78,
    /// 79:
    ADC_ETC_ERROR_IRQ = 79,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
