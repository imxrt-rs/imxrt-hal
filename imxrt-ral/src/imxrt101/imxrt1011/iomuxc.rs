#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPUART3_CTS_B of instance: LPUART3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART4_CTS_B of instance: LPUART4
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO26 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO28 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: REF_CLK_24M of instance: XTAL OSC
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: XBAR1_INOUT02 of instance: XBAR1
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPUART3_RTS_B of instance: LPUART3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART4_RTS_B of instance: LPUART4
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO25 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO27 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: NMI_GLUE_NMI of instance: NMI_GLUE
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_TMS of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI2_SCK of instance: LPSPI2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM0_X of instance: FLEXPWM1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_COL01 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: PIT_TRIGGER01 of instance: PIT
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO24 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO26 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USB_OTG1_PWR of instance: USB
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_TCK of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI2_PCS0 of instance: LPSPI2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM1_X of instance: FLEXPWM1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_ROW01 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: PIT_TRIGGER02 of instance: PIT
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO23 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO25 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: WDOG1_B of instance: WDOG1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_MOD of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI2_SDO of instance: LPSPI2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM2_X of instance: FLEXPWM1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_COL02 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: PIT_TRIGGER03 of instance: PIT
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO22 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO24 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: OTG1_ID of instance: anatop
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_TDI of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI2_SDI of instance: LPSPI2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWM3_X of instance: FLEXPWM1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_ROW02 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: ARM_TRACE_SWO of instance: cm7_mxrt
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO21 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO23 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: REF_32K_OUT of instance: anatop
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_TDO of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPUART3_TXD of instance: LPUART3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART2_CTS_B of instance: LPUART2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_COMPARE3 of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO22 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: EWM_OUT_B of instance: EWM
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_TRSTB of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPUART3_RXD of instance: LPUART3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART2_RTS_B of instance: LPUART2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_CAPTURE2 of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO21 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: OCOTP_FUSE_LATCHED of instance: OCOTP
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: XBAR1_INOUT03 of instance: XBAR1
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI1_SCK of instance: LPSPI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: PIT_TRIGGER00 of instance: PIT
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL01 of instance: KPP
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_COMPARE2 of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO20 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: LPI2C1
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI1_PCS0 of instance: LPSPI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: PIT_TRIGGER01 of instance: PIT
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW01 of instance: KPP
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_CAPTURE1 of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO19 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI1_SDO of instance: LPSPI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: PIT_TRIGGER02 of instance: PIT
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL02 of instance: KPP
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_COMPARE1 of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO18 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SNVS_HP_VIO_5_CTL of instance: snvs_hp
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPSPI1_SDI of instance: LPSPI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: PIT_TRIGGER03 of instance: PIT
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW02 of instance: KPP
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: GPT2_CLK of instance: GPT2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO17 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SNVS_HP_VIO_5_B of instance: snvs_hp
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: JTAG_DE_B of instance: JTAG
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART4_TXD of instance: LPUART4
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_PCS1 of instance: LPSPI1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: WDOG2_B of instance: WDOG2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: MQS_RIGHT of instance: MQS
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO16 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE_CLK of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART4_RXD of instance: LPUART4
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_PCS1 of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: WDOG1_ANY of instance: WDOG1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: MQS_LEFT of instance: MQS
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO15 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USB_OTG1_OC of instance: USB
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE_SWO of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART2_TXD of instance: LPUART2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_PCS2 of instance: LPSPI1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_COL03 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: USB
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO20 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO14 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: NMI_GLUE_NMI of instance: NMI_GLUE
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE00 of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_AD_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Select mux mode: ALT0 mux port: FLEXSPI_A_DQS of instance: FLEXSPI
            pub const ALT0: u32 = 0b0;

            /// 0b1: Select mux mode: ALT1 mux port: FLEXSPI_B_DQS of instance: FLEXSPI
            pub const ALT1: u32 = 0b1;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_SCLK of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_RX_BCLK of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ARM_CM7_TXEV of instance: cm7_mxrt
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_PMIC_RDY of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO19 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_DQS of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_PCS0 of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_TXD of instance: LPUART1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO18 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: WDOG2_RST_B_DEB of instance: WDOG2
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_DATA3 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_SCK of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_RXD of instance: LPUART1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO17 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: WDOG1_RST_B_DEB of instance: WDOG1
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_SCLK of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_SDO of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_TXD of instance: LPUART2
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO16 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_DATA0 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_SDI of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_RXD of instance: LPUART2
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO15 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_DATA2 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI1_SCK of instance: LPSPI1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO14 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_DATA1 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI1_PCS0 of instance: LPSPI1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO13 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_SS0_B of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI1_SDO of instance: LPSPI1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO12 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI1_SDI of instance: LPSPI1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO11 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: GPIO2
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_DATA03 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_RX_SYNC of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_WAIT of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO10 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BOOT_MODE00 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_DATA00 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_RX_DATA of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_REF_EN_B of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO09 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BOOT_MODE01 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_DATA02 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_TX_DATA of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_CLKO1 of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO08 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_DATA01 of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_TX_BCLK of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_CLKO2 of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO07 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_SS0_B of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_TX_SYNC of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ARM_CM7_RXEV of instance: cm7_mxrt
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: CCM_STOP of instance: CCM
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO06 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: GPIO2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: SRC
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_SD_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART2_RXD of instance: LPUART2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI2_PCS2 of instance: LPSPI2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_ROW03 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: OTG1_ID of instance: anatop
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO05 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO13 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SPDIF_LOCK of instance: SPDIF
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE01 of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART3_TXD of instance: LPUART3
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_COL00 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG1_OC of instance: USB
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO04 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO12 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SPDIF_EXT_CLK of instance: SPDIF
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE02 of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART3_RXD of instance: LPUART3
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: KPP_ROW00 of instance: KPP
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: FLEXSPI_B_SS1_B of instance: FLEXSPI
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO03 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO11 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SPDIF_OUT of instance: SPDIF
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ARM_CM7_TRACE03 of instance: cm7_mxrt
            pub const ALT7: u32 = 0b111;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART1_TXD of instance: LPUART1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: LPI2C1_HREQ of instance: LPI2C1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: EWM_OUT_B of instance: EWM
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: LPI2C2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO02 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO10 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SPDIF_IN of instance: SPDIF
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: LPUART1_RXD of instance: LPUART1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: WDOG1_B of instance: WDOG1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXSPI_A_SS1_B of instance: FLEXSPI
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: LPI2C2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO01 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO09 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SPDIF_SR_CLK of instance: SPDIF
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_MCLK of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_CLK of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART3_TXD of instance: LPUART3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_IO00 of instance: FLEXIO1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO08 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: LPUART1_CTS_B of instance: LPUART1
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_TX_SYNC of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_COMPARE1 of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM3_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART3_RXD of instance: LPUART3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: SPDIF_LOCK of instance: SPDIF
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO07 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: LPUART1_RTS_B of instance: LPUART1
            pub const ALT6: u32 = 0b110;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_TX_BCLK of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_CAPTURE1 of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART4_TXD of instance: LPUART4
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: SPDIF_EXT_CLK of instance: SPDIF
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO06 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_TX_DATA01 of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_COMPARE2 of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM2_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPUART4_RXD of instance: LPUART4
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: SPDIF_OUT of instance: SPDIF
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO05 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_TX_DATA00 of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_CAPTURE2 of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: SPDIF_IN of instance: SPDIF
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO04 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_RX_DATA00 of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_COMPARE3 of instance: GPT1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM1_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: SPDIF_SR_CLK of instance: SPDIF
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO03 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_RX_SYNC of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: WDOG2_B of instance: WDOG2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_A of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: LPI2C1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: KPP_COL03 of instance: KPP
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO02 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SAI1_RX_BCLK of instance: SAI1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: WDOG1_ANY of instance: WDOG1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: FLEXPWM1_PWM0_B of instance: FLEXPWM1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: LPI2C1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: KPP_ROW03 of instance: KPP
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO01 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: FLEXSPI_B_DQS of instance: FLEXSPI
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: SAI3_MCLK of instance: SAI3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI2_PCS3 of instance: LPSPI2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPSPI1_PCS3 of instance: LPSPI1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: PIT_TRIGGER00 of instance: PIT
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIOMUX_IO00 of instance: GPIOMUX
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad GPIO_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_14 {

    /// Slew Rate Field
    pub mod SRE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slow Slew Rate
            pub const SRE_0_Slow_Slew_Rate: u32 = 0b0;

            /// 0b1: Fast Slew Rate
            pub const SRE_1_Fast_Slew_Rate: u32 = 0b1;
        }
    }

    /// Drive Strength Field
    pub mod DSE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: output driver disabled;
            pub const DSE_0_output_driver_disabled_: u32 = 0b000;

            /// 0b001: R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)
            pub const DSE_1_R0_150_Ohm___3_3V__260_Ohm_1_8V__240_Ohm_for_DDR_: u32 = 0b001;

            /// 0b010: R0/2
            pub const DSE_2_R0_2: u32 = 0b010;

            /// 0b011: R0/3
            pub const DSE_3_R0_3: u32 = 0b011;

            /// 0b100: R0/4
            pub const DSE_4_R0_4: u32 = 0b100;

            /// 0b101: R0/5
            pub const DSE_5_R0_5: u32 = 0b101;

            /// 0b110: R0/6
            pub const DSE_6_R0_6: u32 = 0b110;

            /// 0b111: R0/7
            pub const DSE_7_R0_7: u32 = 0b111;
        }
    }

    /// Speed Field
    pub mod SPEED {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: low(50MHz)
            pub const SPEED_0_low_50MHz: u32 = 0b00;

            /// 0b01: medium(100MHz)
            pub const SPEED_1_medium_100MHz: u32 = 0b01;

            /// 0b10: fast(150MHz)
            pub const SPEED_2_fast_150MHz: u32 = 0b10;

            /// 0b11: max(200MHz)
            pub const SPEED_3_max_200MHz: u32 = 0b11;
        }
    }

    /// Open Drain Enable Field
    pub mod ODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Open Drain Disabled
            pub const ODE_0_Open_Drain_Disabled: u32 = 0b0;

            /// 0b1: Open Drain Enabled
            pub const ODE_1_Open_Drain_Enabled: u32 = 0b1;
        }
    }

    /// Pull / Keep Enable Field
    pub mod PKE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Pull/Keeper Disabled
            pub const PKE_0_Pull_Keeper_Disabled: u32 = 0b0;

            /// 0b1: Pull/Keeper Enabled
            pub const PKE_1_Pull_Keeper_Enabled: u32 = 0b1;
        }
    }

    /// Pull / Keep Select Field
    pub mod PUE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Keeper
            pub const PUE_0_Keeper: u32 = 0b0;

            /// 0b1: Pull
            pub const PUE_1_Pull: u32 = 0b1;
        }
    }

    /// Pull Up / Down Config. Field
    pub mod PUS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 100K Ohm Pull Down
            pub const PUS_0_100K_Ohm_Pull_Down: u32 = 0b00;

            /// 0b01: 47K Ohm Pull Up
            pub const PUS_1_47K_Ohm_Pull_Up: u32 = 0b01;

            /// 0b10: 100K Ohm Pull Up
            pub const PUS_2_100K_Ohm_Pull_Up: u32 = 0b10;

            /// 0b11: 22K Ohm Pull Up
            pub const PUS_3_22K_Ohm_Pull_Up: u32 = 0b11;
        }
    }

    /// Hyst. Enable Field
    pub mod HYS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hysteresis Disabled
            pub const HYS_0_Hysteresis_Disabled: u32 = 0b0;

            /// 0b1: Hysteresis Enabled
            pub const HYS_1_Hysteresis_Enabled: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_AD_14::SRE;
}

/// USB_OTG_ID_SELECT_INPUT DAISY Register
pub mod USB_OTG_ID_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_10 for Mode: ALT6
            pub const GPIO_AD_10_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_13 for Mode: ALT3
            pub const GPIO_13_ALT3: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_02 for Mode: ALT2
            pub const GPIO_SD_02_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_02 for Mode: ALT2
            pub const GPIO_02_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_04 for Mode: ALT2
            pub const GPIO_SD_04_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_04 for Mode: ALT2
            pub const GPIO_04_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_04 for Mode: ALT2
            pub const GPIO_AD_04_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_06 for Mode: ALT2
            pub const GPIO_06_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM1_PWMA_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT2
            pub const GPIO_AD_06_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_08 for Mode: ALT2
            pub const GPIO_08_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_01 for Mode: ALT2
            pub const GPIO_SD_01_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_01 for Mode: ALT2
            pub const GPIO_01_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_03 for Mode: ALT2
            pub const GPIO_SD_03_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_03 for Mode: ALT2
            pub const GPIO_03_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_03 for Mode: ALT2
            pub const GPIO_AD_03_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_05 for Mode: ALT2
            pub const GPIO_05_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM1_PWMB_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_05 for Mode: ALT2
            pub const GPIO_AD_05_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_07 for Mode: ALT2
            pub const GPIO_07_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register
pub mod FLEXSPI_DQS_FA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_14 for Mode: ALT0
            pub const GPIO_SD_14_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_12 for Mode: ALT0
            pub const GPIO_SD_12_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register
pub mod FLEXSPI_DQS_FB_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_14 for Mode: ALT1
            pub const GPIO_SD_14_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_00 for Mode: ALT0
            pub const GPIO_00_ALT0: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_0 DAISY Register
pub mod KPP_COL_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_14 for Mode: ALT2
            pub const GPIO_AD_14_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_12 for Mode: ALT2
            pub const GPIO_12_ALT2: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_1 DAISY Register
pub mod KPP_COL_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_12 for Mode: ALT2
            pub const GPIO_AD_12_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_06 for Mode: ALT3
            pub const GPIO_AD_06_ALT3: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_2 DAISY Register
pub mod KPP_COL_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_10 for Mode: ALT2
            pub const GPIO_AD_10_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_04 for Mode: ALT3
            pub const GPIO_AD_04_ALT3: u32 = 0b1;
        }
    }
}

/// KPP_COL_SELECT_INPUT_3 DAISY Register
pub mod KPP_COL_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_00 for Mode: ALT2
            pub const GPIO_AD_00_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_02 for Mode: ALT4
            pub const GPIO_02_ALT4: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_0 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_13 for Mode: ALT2
            pub const GPIO_AD_13_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_11 for Mode: ALT2
            pub const GPIO_11_ALT2: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_1 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_1 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_11 for Mode: ALT2
            pub const GPIO_AD_11_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_05 for Mode: ALT3
            pub const GPIO_AD_05_ALT3: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_2 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_2 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_09 for Mode: ALT2
            pub const GPIO_AD_09_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_03 for Mode: ALT3
            pub const GPIO_AD_03_ALT3: u32 = 0b1;
        }
    }
}

/// KPP_ROW_SELECT_INPUT_3 DAISY Register
pub mod KPP_ROW_SELECT_INPUT_3 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_13 for Mode: ALT2
            pub const GPIO_13_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_01 for Mode: ALT4
            pub const GPIO_01_ALT4: u32 = 0b1;
        }
    }
}

/// LPI2C1_HREQ_SELECT_INPUT DAISY Register
pub mod LPI2C1_HREQ_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT6
            pub const GPIO_AD_06_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_10 for Mode: ALT1
            pub const GPIO_10_ALT1: u32 = 0b1;
        }
    }
}

/// LPI2C1_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C1_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_14 for Mode: ALT0
            pub const GPIO_AD_14_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_06 for Mode: ALT1
            pub const GPIO_SD_06_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_12 for Mode: ALT1
            pub const GPIO_12_ALT1: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_02 for Mode: ALT3
            pub const GPIO_02_ALT3: u32 = 0b11;
        }
    }
}

/// LPI2C1_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C1_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_13 for Mode: ALT0
            pub const GPIO_AD_13_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_05 for Mode: ALT1
            pub const GPIO_SD_05_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_11 for Mode: ALT1
            pub const GPIO_11_ALT1: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_01 for Mode: ALT3
            pub const GPIO_01_ALT3: u32 = 0b11;
        }
    }
}

/// LPI2C2_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C2_SCL_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_08 for Mode: ALT0
            pub const GPIO_AD_08_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_02 for Mode: ALT3
            pub const GPIO_AD_02_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_SD_08 for Mode: ALT1
            pub const GPIO_SD_08_ALT1: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_10 for Mode: ALT3
            pub const GPIO_10_ALT3: u32 = 0b11;
        }
    }
}

/// LPI2C2_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C2_SDA_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_07 for Mode: ALT0
            pub const GPIO_AD_07_ALT0: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_01 for Mode: ALT3
            pub const GPIO_AD_01_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_SD_07 for Mode: ALT1
            pub const GPIO_SD_07_ALT1: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_09 for Mode: ALT3
            pub const GPIO_09_ALT3: u32 = 0b11;
        }
    }
}

/// LPSPI1_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI1_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_05 for Mode: ALT0
            pub const GPIO_AD_05_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_07 for Mode: ALT2
            pub const GPIO_SD_07_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI1_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI1_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_06 for Mode: ALT0
            pub const GPIO_AD_06_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_08 for Mode: ALT2
            pub const GPIO_SD_08_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI1_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI1_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_03 for Mode: ALT0
            pub const GPIO_AD_03_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_05 for Mode: ALT2
            pub const GPIO_SD_05_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI1_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI1_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_04 for Mode: ALT0
            pub const GPIO_AD_04_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_06 for Mode: ALT2
            pub const GPIO_SD_06_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI2_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI2_PCS_SELECT_INPUT_0 {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_11 for Mode: ALT0
            pub const GPIO_AD_11_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_12 for Mode: ALT1
            pub const GPIO_SD_12_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI2_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI2_SCK_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_12 for Mode: ALT0
            pub const GPIO_AD_12_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_11 for Mode: ALT1
            pub const GPIO_SD_11_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI2_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI2_SDI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_09 for Mode: ALT0
            pub const GPIO_AD_09_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_09 for Mode: ALT1
            pub const GPIO_SD_09_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI2_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI2_SDO_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_10 for Mode: ALT0
            pub const GPIO_AD_10_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_10 for Mode: ALT1
            pub const GPIO_SD_10_ALT1: u32 = 0b1;
        }
    }
}

/// LPUART1_RXD_SELECT_INPUT DAISY Register
pub mod LPUART1_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_11 for Mode: ALT2
            pub const GPIO_SD_11_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_09 for Mode: ALT0
            pub const GPIO_09_ALT0: u32 = 0b1;
        }
    }
}

/// LPUART1_TXD_SELECT_INPUT DAISY Register
pub mod LPUART1_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_12 for Mode: ALT2
            pub const GPIO_SD_12_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_10 for Mode: ALT0
            pub const GPIO_10_ALT0: u32 = 0b1;
        }
    }
}

/// LPUART2_RXD_SELECT_INPUT DAISY Register
pub mod LPUART2_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_SD_09 for Mode: ALT2
            pub const GPIO_SD_09_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_13 for Mode: ALT0
            pub const GPIO_13_ALT0: u32 = 0b1;
        }
    }
}

/// LPUART2_TXD_SELECT_INPUT DAISY Register
pub mod LPUART2_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_00 for Mode: ALT0
            pub const GPIO_AD_00_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_10 for Mode: ALT2
            pub const GPIO_SD_10_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART3_RXD_SELECT_INPUT DAISY Register
pub mod LPUART3_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_07 for Mode: ALT1
            pub const GPIO_AD_07_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_11 for Mode: ALT0
            pub const GPIO_11_ALT0: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_07 for Mode: ALT3
            pub const GPIO_07_ALT3: u32 = 0b10;
        }
    }
}

/// LPUART3_TXD_SELECT_INPUT DAISY Register
pub mod LPUART3_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Selecting Pad: GPIO_AD_08 for Mode: ALT1
            pub const GPIO_AD_08_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_12 for Mode: ALT0
            pub const GPIO_12_ALT0: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_08 for Mode: ALT3
            pub const GPIO_08_ALT3: u32 = 0b10;
        }
    }
}

/// LPUART4_RXD_SELECT_INPUT DAISY Register
pub mod LPUART4_RXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_01 for Mode: ALT0
            pub const GPIO_AD_01_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_05 for Mode: ALT3
            pub const GPIO_05_ALT3: u32 = 0b1;
        }
    }
}

/// LPUART4_TXD_SELECT_INPUT DAISY Register
pub mod LPUART4_TXD_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_02 for Mode: ALT0
            pub const GPIO_AD_02_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_06 for Mode: ALT3
            pub const GPIO_06_ALT3: u32 = 0b1;
        }
    }
}

/// NMI_GLUE_NMI_SELECT_INPUT DAISY Register
pub mod NMI_GLUE_NMI_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_13 for Mode: ALT6
            pub const GPIO_AD_13_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_00 for Mode: ALT6
            pub const GPIO_AD_00_ALT6: u32 = 0b1;
        }
    }
}

/// SPDIF_IN1_SELECT_INPUT DAISY Register
pub mod SPDIF_IN1_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_10 for Mode: ALT6
            pub const GPIO_10_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_04 for Mode: ALT4
            pub const GPIO_04_ALT4: u32 = 0b1;
        }
    }
}

/// SPDIF_TX_CLK2_SELECT_INPUT DAISY Register
pub mod SPDIF_TX_CLK2_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_12 for Mode: ALT6
            pub const GPIO_12_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_06 for Mode: ALT4
            pub const GPIO_06_ALT4: u32 = 0b1;
        }
    }
}

/// USB_OTG_OC_SELECT_INPUT DAISY Register
pub mod USB_OTG_OC_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_01 for Mode: ALT6
            pub const GPIO_AD_01_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_12 for Mode: ALT3
            pub const GPIO_12_ALT3: u32 = 0b1;
        }
    }
}

/// XEV_GLUE_RXEV_SELECT_INPUT DAISY Register
pub mod XEV_GLUE_RXEV_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Selecting Pad: GPIO_AD_07 for Mode: ALT2
            pub const GPIO_AD_07_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_00 for Mode: ALT2
            pub const GPIO_SD_00_ALT2: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 4],

    /// SW_MUX_CTL_PAD_GPIO_AD_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_00: RWRegister<u32>,

    /// USB_OTG_ID_SELECT_INPUT DAISY Register
    pub USB_OTG_ID_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM1_PWMA_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM1_PWMB_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register
    pub FLEXSPI_DQS_FA_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register
    pub FLEXSPI_DQS_FB_SELECT_INPUT: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_0 DAISY Register
    pub KPP_COL_SELECT_INPUT_0: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_1 DAISY Register
    pub KPP_COL_SELECT_INPUT_1: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_2 DAISY Register
    pub KPP_COL_SELECT_INPUT_2: RWRegister<u32>,

    /// KPP_COL_SELECT_INPUT_3 DAISY Register
    pub KPP_COL_SELECT_INPUT_3: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_0 DAISY Register
    pub KPP_ROW_SELECT_INPUT_0: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_1 DAISY Register
    pub KPP_ROW_SELECT_INPUT_1: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_2 DAISY Register
    pub KPP_ROW_SELECT_INPUT_2: RWRegister<u32>,

    /// KPP_ROW_SELECT_INPUT_3 DAISY Register
    pub KPP_ROW_SELECT_INPUT_3: RWRegister<u32>,

    /// LPI2C1_HREQ_SELECT_INPUT DAISY Register
    pub LPI2C1_HREQ_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_SCL_SELECT_INPUT DAISY Register
    pub LPI2C1_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_SDA_SELECT_INPUT DAISY Register
    pub LPI2C1_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_SCL_SELECT_INPUT DAISY Register
    pub LPI2C2_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_SDA_SELECT_INPUT DAISY Register
    pub LPI2C2_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI1_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI1_SCK_SELECT_INPUT DAISY Register
    pub LPSPI1_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_SDI_SELECT_INPUT DAISY Register
    pub LPSPI1_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_SDO_SELECT_INPUT DAISY Register
    pub LPSPI1_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI2_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI2_SCK_SELECT_INPUT DAISY Register
    pub LPSPI2_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_SDI_SELECT_INPUT DAISY Register
    pub LPSPI2_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_SDO_SELECT_INPUT DAISY Register
    pub LPSPI2_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPUART1_RXD_SELECT_INPUT DAISY Register
    pub LPUART1_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART1_TXD_SELECT_INPUT DAISY Register
    pub LPUART1_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_RXD_SELECT_INPUT DAISY Register
    pub LPUART2_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_TXD_SELECT_INPUT DAISY Register
    pub LPUART2_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_RXD_SELECT_INPUT DAISY Register
    pub LPUART3_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_TXD_SELECT_INPUT DAISY Register
    pub LPUART3_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_RXD_SELECT_INPUT DAISY Register
    pub LPUART4_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_TXD_SELECT_INPUT DAISY Register
    pub LPUART4_TXD_SELECT_INPUT: RWRegister<u32>,

    /// NMI_GLUE_NMI_SELECT_INPUT DAISY Register
    pub NMI_GLUE_NMI_SELECT_INPUT: RWRegister<u32>,

    /// SPDIF_IN1_SELECT_INPUT DAISY Register
    pub SPDIF_IN1_SELECT_INPUT: RWRegister<u32>,

    /// SPDIF_TX_CLK2_SELECT_INPUT DAISY Register
    pub SPDIF_TX_CLK2_SELECT_INPUT: RWRegister<u32>,

    /// USB_OTG_OC_SELECT_INPUT DAISY Register
    pub USB_OTG_OC_SELECT_INPUT: RWRegister<u32>,

    /// XEV_GLUE_RXEV_SELECT_INPUT DAISY Register
    pub XEV_GLUE_RXEV_SELECT_INPUT: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_GPIO_AD_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_00: u32,
    pub USB_OTG_ID_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_0: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_1: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_2: u32,
    pub FLEXPWM1_PWMA_SELECT_INPUT_3: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_0: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_1: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_2: u32,
    pub FLEXPWM1_PWMB_SELECT_INPUT_3: u32,
    pub FLEXSPI_DQS_FA_SELECT_INPUT: u32,
    pub FLEXSPI_DQS_FB_SELECT_INPUT: u32,
    pub KPP_COL_SELECT_INPUT_0: u32,
    pub KPP_COL_SELECT_INPUT_1: u32,
    pub KPP_COL_SELECT_INPUT_2: u32,
    pub KPP_COL_SELECT_INPUT_3: u32,
    pub KPP_ROW_SELECT_INPUT_0: u32,
    pub KPP_ROW_SELECT_INPUT_1: u32,
    pub KPP_ROW_SELECT_INPUT_2: u32,
    pub KPP_ROW_SELECT_INPUT_3: u32,
    pub LPI2C1_HREQ_SELECT_INPUT: u32,
    pub LPI2C1_SCL_SELECT_INPUT: u32,
    pub LPI2C1_SDA_SELECT_INPUT: u32,
    pub LPI2C2_SCL_SELECT_INPUT: u32,
    pub LPI2C2_SDA_SELECT_INPUT: u32,
    pub LPSPI1_PCS_SELECT_INPUT_0: u32,
    pub LPSPI1_SCK_SELECT_INPUT: u32,
    pub LPSPI1_SDI_SELECT_INPUT: u32,
    pub LPSPI1_SDO_SELECT_INPUT: u32,
    pub LPSPI2_PCS_SELECT_INPUT_0: u32,
    pub LPSPI2_SCK_SELECT_INPUT: u32,
    pub LPSPI2_SDI_SELECT_INPUT: u32,
    pub LPSPI2_SDO_SELECT_INPUT: u32,
    pub LPUART1_RXD_SELECT_INPUT: u32,
    pub LPUART1_TXD_SELECT_INPUT: u32,
    pub LPUART2_RXD_SELECT_INPUT: u32,
    pub LPUART2_TXD_SELECT_INPUT: u32,
    pub LPUART3_RXD_SELECT_INPUT: u32,
    pub LPUART3_TXD_SELECT_INPUT: u32,
    pub LPUART4_RXD_SELECT_INPUT: u32,
    pub LPUART4_TXD_SELECT_INPUT: u32,
    pub NMI_GLUE_NMI_SELECT_INPUT: u32,
    pub SPDIF_IN1_SELECT_INPUT: u32,
    pub SPDIF_TX_CLK2_SELECT_INPUT: u32,
    pub USB_OTG_OC_SELECT_INPUT: u32,
    pub XEV_GLUE_RXEV_SELECT_INPUT: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}

/// Access functions for the IOMUXC peripheral instance
pub mod IOMUXC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401f8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_GPIO_AD_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_13: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_12: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_11: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_10: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_09: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_08: 0x00000007,
        SW_MUX_CTL_PAD_GPIO_AD_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_14: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_SD_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_04: 0x00000006,
        SW_MUX_CTL_PAD_GPIO_SD_03: 0x00000006,
        SW_MUX_CTL_PAD_GPIO_SD_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_00: 0x00000005,
        SW_PAD_CTL_PAD_GPIO_AD_14: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_13: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_12: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_AD_11: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_AD_10: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_09: 0x000090B1,
        SW_PAD_CTL_PAD_GPIO_AD_08: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_07: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_06: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_05: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_04: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_03: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_02: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_01: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_AD_00: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_14: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_SD_13: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_12: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_11: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_10: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_09: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_08: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_07: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_06: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_05: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_04: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_SD_03: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_SD_02: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_01: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_SD_00: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_13: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_12: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_11: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_10: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_09: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_08: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_07: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_06: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_05: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_04: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_03: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_02: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_01: 0x000010A0,
        SW_PAD_CTL_PAD_GPIO_00: 0x000010A0,
        USB_OTG_ID_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA_SELECT_INPUT_0: 0x00000000,
        FLEXPWM1_PWMA_SELECT_INPUT_1: 0x00000000,
        FLEXPWM1_PWMA_SELECT_INPUT_2: 0x00000000,
        FLEXPWM1_PWMA_SELECT_INPUT_3: 0x00000000,
        FLEXPWM1_PWMB_SELECT_INPUT_0: 0x00000000,
        FLEXPWM1_PWMB_SELECT_INPUT_1: 0x00000000,
        FLEXPWM1_PWMB_SELECT_INPUT_2: 0x00000000,
        FLEXPWM1_PWMB_SELECT_INPUT_3: 0x00000000,
        FLEXSPI_DQS_FA_SELECT_INPUT: 0x00000000,
        FLEXSPI_DQS_FB_SELECT_INPUT: 0x00000000,
        KPP_COL_SELECT_INPUT_0: 0x00000000,
        KPP_COL_SELECT_INPUT_1: 0x00000000,
        KPP_COL_SELECT_INPUT_2: 0x00000000,
        KPP_COL_SELECT_INPUT_3: 0x00000000,
        KPP_ROW_SELECT_INPUT_0: 0x00000000,
        KPP_ROW_SELECT_INPUT_1: 0x00000000,
        KPP_ROW_SELECT_INPUT_2: 0x00000000,
        KPP_ROW_SELECT_INPUT_3: 0x00000000,
        LPI2C1_HREQ_SELECT_INPUT: 0x00000000,
        LPI2C1_SCL_SELECT_INPUT: 0x00000000,
        LPI2C1_SDA_SELECT_INPUT: 0x00000000,
        LPI2C2_SCL_SELECT_INPUT: 0x00000000,
        LPI2C2_SDA_SELECT_INPUT: 0x00000000,
        LPSPI1_PCS_SELECT_INPUT_0: 0x00000000,
        LPSPI1_SCK_SELECT_INPUT: 0x00000000,
        LPSPI1_SDI_SELECT_INPUT: 0x00000000,
        LPSPI1_SDO_SELECT_INPUT: 0x00000000,
        LPSPI2_PCS_SELECT_INPUT_0: 0x00000000,
        LPSPI2_SCK_SELECT_INPUT: 0x00000000,
        LPSPI2_SDI_SELECT_INPUT: 0x00000000,
        LPSPI2_SDO_SELECT_INPUT: 0x00000000,
        LPUART1_RXD_SELECT_INPUT: 0x00000000,
        LPUART1_TXD_SELECT_INPUT: 0x00000000,
        LPUART2_RXD_SELECT_INPUT: 0x00000000,
        LPUART2_TXD_SELECT_INPUT: 0x00000000,
        LPUART3_RXD_SELECT_INPUT: 0x00000000,
        LPUART3_TXD_SELECT_INPUT: 0x00000000,
        LPUART4_RXD_SELECT_INPUT: 0x00000000,
        LPUART4_TXD_SELECT_INPUT: 0x00000000,
        NMI_GLUE_NMI_SELECT_INPUT: 0x00000000,
        SPDIF_IN1_SELECT_INPUT: 0x00000000,
        SPDIF_TX_CLK2_SELECT_INPUT: 0x00000000,
        USB_OTG_OC_SELECT_INPUT: 0x00000000,
        XEV_GLUE_RXEV_SELECT_INPUT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_TAKEN: bool = false;

    /// Safe access to IOMUXC
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_TAKEN {
                None
            } else {
                IOMUXC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC: *const RegisterBlock = 0x401f8000 as *const _;
