#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_04 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT04 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SPDIF_OUT of instance: spdif
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_TX_BCLK of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO16 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: SJC_JTAG_ACT of instance: sjc
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

            /// 0b1: Force input path of pad GPIO_EMC_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_05 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT05 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SPDIF_IN of instance: spdif
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_TX_SYNC of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO17 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: SJC_DE_B of instance: sjc
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

            /// 0b1: Force input path of pad GPIO_EMC_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_06 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT06 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_TX_DATA of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO18 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2
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

            /// 0b1: Force input path of pad GPIO_EMC_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_07 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT07 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_SYNC of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO19 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2
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

            /// 0b1: Force input path of pad GPIO_EMC_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_08 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT08 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_DATA of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO20 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2
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

            /// 0b1: Force input path of pad GPIO_EMC_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_09 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT09 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_BCLK of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO21 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2
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

            /// 0b1: Force input path of pad GPIO_EMC_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_16 {

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

            /// 0b010: Select mux mode: ALT2 mux port: MQS_RIGHT of instance: mqs
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_MCLK of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BOOT_MODE00 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_16
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_17 {

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

            /// 0b010: Select mux mode: ALT2 mux port: MQS_LEFT of instance: mqs
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BOOT_MODE01 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_17
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_18 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT16 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPI2C2_SDA of instance: lpi2c2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO22 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG00 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_18
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_19 {

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

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_XBAR_INOUT17 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPI2C2_SCL of instance: lpi2c2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO23 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG01 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_19
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_20 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO24 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG02 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_20
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_21 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO25 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG03 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_21
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_22 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO26 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG04 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_22
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_23 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO27 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG05 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_23
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_24 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO28 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG06 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_24
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_25 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO29 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG07 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_25
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_26 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO30 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG08 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_26
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_27 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO31 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SRC_BT_CFG09 of instance: src
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

            /// 0b1: Force input path of pad GPIO_EMC_27
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_32 {

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

            /// 0b001: Select mux mode: ALT1 mux port: QTIMER1_TIMER0 of instance: qtimer1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: REF_24M_OUT of instance: anatop
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

            /// 0b1: Force input path of pad GPIO_EMC_32
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_33 {

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

            /// 0b001: Select mux mode: ALT1 mux port: QTIMER1_TIMER1 of instance: qtimer1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_EMC_33
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_34 {

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

            /// 0b001: Select mux mode: ALT1 mux port: QTIMER1_TIMER2 of instance: qtimer1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_EMC_34
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_35 {

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

            /// 0b001: Select mux mode: ALT1 mux port: QTIMER1_TIMER3 of instance: qtimer1
            pub const ALT1: u32 = 0b001;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_EMC_35
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_00 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_TMS of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: GPT1_COMPARE1 of instance: gpt1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_01 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_TCK of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: GPT1_CAPTURE2 of instance: gpt1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_02 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_MOD of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: GPT1_CAPTURE1 of instance: gpt1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_03 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_TDI of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b010: Select mux mode: ALT2 mux port: WDOG1_WDOG_B of instance: wdog1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USB_OTG1_OC of instance: usb
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: CCM_PMIC_RDY of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_AD_B0_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_04 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_TDO of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USB_OTG1_PWR of instance: usb
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: EWM_EWM_OUT_B of instance: ewm
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

            /// 0b1: Force input path of pad GPIO_AD_B0_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_05 {

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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MUX_TRSTB of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USB_OTG1_ID of instance: anatop
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: NMI_GLUE_NMI of instance: nmi_glue
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

            /// 0b1: Force input path of pad GPIO_AD_B0_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_06 {

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

            /// 0b000: Select mux mode: ALT0 mux port: PIT_TRIGGER00 of instance: pit
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: MQS_RIGHT of instance: mqs
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_TX of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: REF_32K_OUT of instance: anatop
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

            /// 0b1: Force input path of pad GPIO_AD_B0_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_07 {

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

            /// 0b000: Select mux mode: ALT0 mux port: PIT_TRIGGER01 of instance: pit
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: MQS_LEFT of instance: mqs
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_RX of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: REF_24M_OUT of instance: anatop
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

            /// 0b1: Force input path of pad GPIO_AD_B0_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_08 {

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

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL00 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: ARM_CM7_TXEV of instance: cm7_mxrt
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

            /// 0b1: Force input path of pad GPIO_AD_B0_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_09 {

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

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW00 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSU_CSU_INT_DEB of instance: csu
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: ARM_CM7_RXEV of instance: cm7_mxrt
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

            /// 0b1: Force input path of pad GPIO_AD_B0_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_10 {

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

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_SCK of instance: lpspi1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL01 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_11 {

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

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_PCS0 of instance: lpspi1
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW01 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_12 {

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

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_SDO of instance: lpspi1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL02 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: SNVS_HP_VIO_5_CTL of instance: snvs_hp
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

            /// 0b1: Force input path of pad GPIO_AD_B0_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_13 {

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

            /// 0b001: Select mux mode: ALT1 mux port: LPSPI1_SDI of instance: lpspi1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW02 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: SNVS_HP_VIO_5_B of instance: snvs_hp
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

            /// 0b1: Force input path of pad GPIO_AD_B0_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_14 {

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

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_COL03 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b111: Select mux mode: ALT7 mux port: WDOG1_WDOG_ANY of instance: wdog1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B0_15 {

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

            /// 0b010: Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: KPP_ROW03 of instance: kpp
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1
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

            /// 0b1: Force input path of pad GPIO_AD_B0_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_10 {

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

            /// 0b000: Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: usb
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO05 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: GPT2_CAPTURE1 of instance: gpt2
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

            /// 0b1: Force input path of pad GPIO_AD_B1_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_11 {

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

            /// 0b000: Select mux mode: ALT0 mux port: USB_OTG1_ID of instance: anatop
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO04 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: GPT2_COMPARE1 of instance: gpt2
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

            /// 0b1: Force input path of pad GPIO_AD_B1_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_12 {

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

            /// 0b000: Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: usb
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: ACMP_OUT00 of instance: acmp
            pub const ALT1: u32 = 0b001;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO03 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
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

            /// 0b1: Force input path of pad GPIO_AD_B1_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_13 {

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

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C1_HREQ of instance: lpi2c1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: ACMP_OUT01 of instance: acmp
            pub const ALT1: u32 = 0b001;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO02 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO29 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
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

            /// 0b1: Force input path of pad GPIO_AD_B1_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_14 {

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

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C1_SCL of instance: lpi2c1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: ACMP_OUT02 of instance: acmp
            pub const ALT1: u32 = 0b001;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO01 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO30 of instance: gpio1
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

            /// 0b1: Force input path of pad GPIO_AD_B1_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_15 {

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

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C1_SDA of instance: lpi2c1
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: ACMP_OUT03 of instance: acmp
            pub const ALT1: u32 = 0b001;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO00 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO31 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_DI0_EXT_CLK of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_AD_B1_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_00 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_B_DATA03 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_XBAR_INOUT10 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_01 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_B_SCLK of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: FLEXSPI_A_SS1_B of instance: flexspi
            pub const ALT3: u32 = 0b011;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_02 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_B_DATA00 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_CLKO1 of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_03 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_B_DATA02 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_CLKO2 of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_04 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_B_DATA01 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b100: Select mux mode: ALT4 mux port: EWM_EWM_OUT_B of instance: ewm
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_WAIT of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_05 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_DQS of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXSPI_B_SS0_B of instance: flexspi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_PMIC_RDY of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_06 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_DATA03 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_PCS0 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_STOP of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_07 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_SCLK of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SCK of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_08 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_DATA00 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SDO of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO28 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_09 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_DATA02 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_RX_BCLK of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SDI of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO29 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: CCM_REF_EN_B of instance: ccm
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

            /// 0b1: Force input path of pad GPIO_SD_B1_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_10 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_DATA01 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_RX_SYNC of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_PCS2 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO30 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B1_11 {

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

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPI_A_SS0_B of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b011: Select mux mode: ALT3 mux port: SAI3_RX_DATA of instance: sai3
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_PCS3 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO31 of instance: gpio3
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

            /// 0b1: Force input path of pad GPIO_SD_B1_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_04 {

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

            /// 0b001: R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)
            pub const DSE_1_R0_150_Ohm___3_3V__260_Ohm_1_8V: u32 = 0b001;

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
            pub const SPEED_0_low_50MHz_: u32 = 0b00;

            /// 0b01: medium(100MHz)
            pub const SPEED_1_medium_100MHz_: u32 = 0b01;

            /// 0b10: medium(100MHz)
            pub const SPEED_2_medium_100MHz_: u32 = 0b10;

            /// 0b11: max(200MHz)
            pub const SPEED_3_max_200MHz_: u32 = 0b11;
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

/// SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_16 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_17 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_18 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_19 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_20 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_21 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_22 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_23 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_24 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_25 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_26 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_27 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_32 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_33 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_34 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_35 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_04::SRE;
}

/// ANATOP_USB_OTG_ID_SELECT_INPUT DAISY Register
pub mod ANATOP_USB_OTG_ID_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6
            pub const GPIO_AD_B0_05_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT0
            pub const GPIO_AD_B1_11_ALT0: u32 = 0b01;
        }
    }
}

/// CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT DAISY Register
pub mod CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT {

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

            /// 0b01: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT6
            pub const GPIO_SD_B1_05_ALT6: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT7
            pub const GPIO_AD_B0_03_ALT7: u32 = 0b10;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_26 for Mode: ALT1
            pub const GPIO_EMC_26_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_24 for Mode: ALT1
            pub const GPIO_EMC_24_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT1
            pub const GPIO_AD_B1_10_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_22 for Mode: ALT1
            pub const GPIO_EMC_22_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT6
            pub const GPIO_AD_B1_12_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_20 for Mode: ALT1
            pub const GPIO_EMC_20_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_27 for Mode: ALT1
            pub const GPIO_EMC_27_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_25 for Mode: ALT1
            pub const GPIO_EMC_25_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT1
            pub const GPIO_AD_B1_11_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_23 for Mode: ALT1
            pub const GPIO_EMC_23_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register
pub mod FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_13 for Mode: ALT6
            pub const GPIO_AD_B1_13_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_21 for Mode: ALT1
            pub const GPIO_EMC_21_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register
pub mod FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT1
            pub const GPIO_SD_B1_08_ALT1: u32 = 0b0;
        }
    }
}

/// FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register
pub mod FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_10 for Mode: ALT1
            pub const GPIO_SD_B1_10_ALT1: u32 = 0b0;
        }
    }
}

/// FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register
pub mod FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT1
            pub const GPIO_SD_B1_09_ALT1: u32 = 0b0;
        }
    }
}

/// FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register
pub mod FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT1
            pub const GPIO_SD_B1_06_ALT1: u32 = 0b0;
        }
    }
}

/// FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register
pub mod FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_07 for Mode: ALT1
            pub const GPIO_SD_B1_07_ALT1: u32 = 0b0;
        }
    }
}

/// LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT0
            pub const GPIO_AD_B1_14_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_15 for Mode: ALT0
            pub const GPIO_AD_B1_15_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_19 for Mode: ALT2
            pub const GPIO_EMC_19_ALT2: u32 = 0b1;
        }
    }
}

/// LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_18 for Mode: ALT2
            pub const GPIO_EMC_18_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1
            pub const GPIO_AD_B0_11_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_10 for Mode: ALT1
            pub const GPIO_AD_B0_10_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_13 for Mode: ALT1
            pub const GPIO_AD_B0_13_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_12 for Mode: ALT1
            pub const GPIO_AD_B0_12_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
pub mod LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 {

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

            /// 0b10: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT4
            pub const GPIO_SD_B1_06_ALT4: u32 = 0b10;
        }
    }
}

/// LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4
            pub const GPIO_SD_B1_07_ALT4: u32 = 0b10;
        }
    }
}

/// LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT4
            pub const GPIO_SD_B1_09_ALT4: u32 = 0b10;
        }
    }
}

/// LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT4
            pub const GPIO_SD_B1_08_ALT4: u32 = 0b10;
        }
    }
}

/// LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register
pub mod LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_20 for Mode: ALT2
            pub const GPIO_EMC_20_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_23 for Mode: ALT2
            pub const GPIO_EMC_23_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_22 for Mode: ALT2
            pub const GPIO_EMC_22_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_07 for Mode: ALT2
            pub const GPIO_EMC_07_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT2
            pub const GPIO_AD_B0_15_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_06 for Mode: ALT2
            pub const GPIO_EMC_06_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_14 for Mode: ALT2
            pub const GPIO_AD_B0_14_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register
pub mod LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT {

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
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
pub mod LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT {

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

            /// 0b01: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT2
            pub const GPIO_AD_B1_11_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_EMC_33 for Mode: ALT2
            pub const GPIO_EMC_33_ALT2: u32 = 0b10;
        }
    }
}

/// LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
pub mod LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT {

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

            /// 0b01: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT2
            pub const GPIO_AD_B1_10_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_EMC_32 for Mode: ALT2
            pub const GPIO_EMC_32_ALT2: u32 = 0b10;
        }
    }
}

/// NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register
pub mod NMI_GLUE_IPP_IND_NMI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT7
            pub const GPIO_AD_B0_05_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: WAKEUP for Mode: ALT7
            pub const WAKEUP_ALT7: u32 = 0b1;
        }
    }
}

/// QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR0_INPUT_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_32 for Mode: ALT1
            pub const GPIO_EMC_32_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR1_INPUT_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_33 for Mode: ALT1
            pub const GPIO_EMC_33_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR2_INPUT_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_34 for Mode: ALT1
            pub const GPIO_EMC_34_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER1_TMR3_INPUT_SELECT_INPUT DAISY Register
pub mod QTIMER1_TMR3_INPUT_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_35 for Mode: ALT1
            pub const GPIO_EMC_35_ALT1: u32 = 0b1;
        }
    }
}

/// SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
pub mod SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {

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

            /// 0b01: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT3
            pub const GPIO_AD_B0_03_ALT3: u32 = 0b01;

            /// 0b11: Selecting Pad: GPIO_EMC_20 for Mode: ALT3
            pub const GPIO_EMC_20_ALT3: u32 = 0b11;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
pub mod SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_EMC_19 for Mode: ALT3
            pub const GPIO_EMC_19_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {

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

            /// 0b10: Selecting Pad: GPIO_EMC_21 for Mode: ALT3
            pub const GPIO_EMC_21_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_22 for Mode: ALT3
            pub const GPIO_EMC_22_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 DAISY Register
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_23 for Mode: ALT3
            pub const GPIO_EMC_23_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 DAISY Register
pub mod SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_24 for Mode: ALT3
            pub const GPIO_EMC_24_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
pub mod SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_EMC_18 for Mode: ALT3
            pub const GPIO_EMC_18_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
pub mod SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_EMC_26 for Mode: ALT3
            pub const GPIO_EMC_26_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
pub mod SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT {

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

            /// 0b10: Selecting Pad: GPIO_EMC_27 for Mode: ALT3
            pub const GPIO_EMC_27_ALT3: u32 = 0b10;
        }
    }
}

/// SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
pub mod SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_16 for Mode: ALT3
            pub const GPIO_EMC_16_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
pub mod SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_09 for Mode: ALT3
            pub const GPIO_EMC_09_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_08 for Mode: ALT3
            pub const GPIO_EMC_08_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
pub mod SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_07 for Mode: ALT3
            pub const GPIO_EMC_07_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
pub mod SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_04 for Mode: ALT3
            pub const GPIO_EMC_04_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
pub mod SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_EMC_05 for Mode: ALT3
            pub const GPIO_EMC_05_ALT3: u32 = 0b1;
        }
    }
}

/// SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
pub mod SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT3
            pub const GPIO_SD_B1_05_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_17 for Mode: ALT3
            pub const GPIO_EMC_17_ALT3: u32 = 0b01;
        }
    }
}

/// SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
pub mod SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT3
            pub const GPIO_SD_B1_09_ALT3: u32 = 0b0;
        }
    }
}

/// SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_11 for Mode: ALT3
            pub const GPIO_SD_B1_11_ALT3: u32 = 0b0;
        }
    }
}

/// SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
pub mod SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_10 for Mode: ALT3
            pub const GPIO_SD_B1_10_ALT3: u32 = 0b0;
        }
    }
}

/// SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
pub mod SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT3
            pub const GPIO_SD_B1_06_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_33 for Mode: ALT3
            pub const GPIO_EMC_33_ALT3: u32 = 0b1;
        }
    }
}

/// SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
pub mod SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_07 for Mode: ALT3
            pub const GPIO_SD_B1_07_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_34 for Mode: ALT3
            pub const GPIO_EMC_34_ALT3: u32 = 0b1;
        }
    }
}

/// SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register
pub mod SPDIF_SPDIF_IN1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_05 for Mode: ALT2
            pub const GPIO_EMC_05_ALT2: u32 = 0b0;
        }
    }
}

/// USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register
pub mod USB_IPP_IND_OTG_OC_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT6
            pub const GPIO_AD_B0_03_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT0
            pub const GPIO_AD_B1_12_ALT0: u32 = 0b01;
        }
    }
}

/// XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_14 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}

/// XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_15 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}

/// XBAR1_XBAR_IN_SELECT_INPUT_16 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_16 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_18 for Mode: ALT1
            pub const GPIO_EMC_18_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_17 {

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

            /// 0b1: Selecting Pad: GPIO_EMC_19 for Mode: ALT1
            pub const GPIO_EMC_19_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_XBAR_IN_SELECT_INPUT_10 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_10 {

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

            /// 0b1: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT3
            pub const GPIO_SD_B1_00_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_XBAR_IN_SELECT_INPUT_12 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_12 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}

/// XBAR1_XBAR_IN_SELECT_INPUT_13 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_13 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}

/// XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_18 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}

/// XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register
pub mod XBAR1_XBAR_IN_SELECT_INPUT_19 {
    pub use super::LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT::DAISY;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 9],

    /// SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_09: RWRegister<u32>,

    _reserved2: [u32; 6],

    /// SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_16: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_17: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_18: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_19: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_20: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_21: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_22: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_23: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_24: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_25: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_26: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_27: RWRegister<u32>,

    _reserved3: [u32; 4],

    /// SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_32: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_33: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_34: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_35: RWRegister<u32>,

    _reserved4: [u32; 6],

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_15: RWRegister<u32>,

    _reserved5: [u32; 10],

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_15: RWRegister<u32>,

    _reserved6: [u32; 7],

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_11: RWRegister<u32>,

    _reserved7: [u32; 4],

    /// SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_09: RWRegister<u32>,

    _reserved8: [u32; 6],

    /// SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_16: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_17: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_18: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_19: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_20: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_21: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_22: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_23: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_24: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_25: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_26: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_27: RWRegister<u32>,

    _reserved9: [u32; 4],

    /// SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_32: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_33: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_34: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_35: RWRegister<u32>,

    _reserved10: [u32; 6],

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_15: RWRegister<u32>,

    _reserved11: [u32; 10],

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_15: RWRegister<u32>,

    _reserved12: [u32; 7],

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_11: RWRegister<u32>,

    /// ANATOP_USB_OTG_ID_SELECT_INPUT DAISY Register
    pub ANATOP_USB_OTG_ID_SELECT_INPUT: RWRegister<u32>,

    /// CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT DAISY Register
    pub CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT: RWRegister<u32>,

    _reserved13: [u32; 9],

    /// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2: RWRegister<u32>,

    /// FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3 DAISY Register
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3: RWRegister<u32>,

    _reserved14: [u32; 8],

    /// FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register
    pub FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register
    pub FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register
    pub FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register
    pub FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register
    pub FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT DAISY Register
    pub LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT DAISY Register
    pub LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT: RWRegister<u32>,

    _reserved15: [u32; 4],

    /// LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0 DAISY Register
    pub LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: RWRegister<u32>,

    /// LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT DAISY Register
    pub LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT DAISY Register
    pub LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT DAISY Register
    pub LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT: RWRegister<u32>,

    _reserved16: [u32; 4],

    /// LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register
    pub LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT DAISY Register
    pub LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT DAISY Register
    pub LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT DAISY Register
    pub LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT: RWRegister<u32>,

    _reserved17: [u32; 8],

    /// NMI_GLUE_IPP_IND_NMI_SELECT_INPUT DAISY Register
    pub NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR0_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR0_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR1_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR1_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR2_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR2_INPUT_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER1_TMR3_INPUT_SELECT_INPUT DAISY Register
    pub QTIMER1_TMR3_INPUT_SELECT_INPUT: RWRegister<u32>,

    _reserved18: [u32; 4],

    /// SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
    pub SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
    pub SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1 DAISY Register
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2 DAISY Register
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3 DAISY Register
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
    pub SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
    pub SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
    pub SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
    pub SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: RWRegister<u32>,

    /// SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
    pub SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
    pub SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
    pub SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
    pub SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
    pub SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
    pub SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: RWRegister<u32>,

    /// SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register
    pub SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register
    pub SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register
    pub SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register
    pub SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register
    pub SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// SPDIF_SPDIF_IN1_SELECT_INPUT DAISY Register
    pub SPDIF_SPDIF_IN1_SELECT_INPUT: RWRegister<u32>,

    /// USB_IPP_IND_OTG_OC_SELECT_INPUT DAISY Register
    pub USB_IPP_IND_OTG_OC_SELECT_INPUT: RWRegister<u32>,

    _reserved20: [u32; 4],

    /// XBAR1_XBAR_IN_SELECT_INPUT_14 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_14: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_15 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_15: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_16 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_16: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_17 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_17: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_10 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_10: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_12 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_12: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_13 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_13: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_18 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_18: RWRegister<u32>,

    /// XBAR1_XBAR_IN_SELECT_INPUT_19 DAISY Register
    pub XBAR1_XBAR_IN_SELECT_INPUT_19: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_GPIO_EMC_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_16: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_17: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_18: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_19: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_20: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_21: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_22: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_23: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_24: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_25: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_26: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_27: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_32: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_33: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_34: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_35: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B0_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_16: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_17: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_18: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_19: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_20: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_21: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_22: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_23: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_24: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_25: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_26: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_27: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_32: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_33: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_34: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_35: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B0_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B1_11: u32,
    pub ANATOP_USB_OTG_ID_SELECT_INPUT: u32,
    pub CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT: u32,
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0: u32,
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1: u32,
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2: u32,
    pub FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3: u32,
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0: u32,
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1: u32,
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2: u32,
    pub FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3: u32,
    pub FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT: u32,
    pub FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT: u32,
    pub FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT: u32,
    pub FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT: u32,
    pub FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT: u32,
    pub LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT: u32,
    pub LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT: u32,
    pub LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: u32,
    pub LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT: u32,
    pub LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT: u32,
    pub LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT: u32,
    pub LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT: u32,
    pub LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT: u32,
    pub LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT: u32,
    pub LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT: u32,
    pub LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT: u32,
    pub NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: u32,
    pub QTIMER1_TMR0_INPUT_SELECT_INPUT: u32,
    pub QTIMER1_TMR1_INPUT_SELECT_INPUT: u32,
    pub QTIMER1_TMR2_INPUT_SELECT_INPUT: u32,
    pub QTIMER1_TMR3_INPUT_SELECT_INPUT: u32,
    pub SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: u32,
    pub SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1: u32,
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2: u32,
    pub SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3: u32,
    pub SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT: u32,
    pub SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: u32,
    pub SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT: u32,
    pub SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: u32,
    pub SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: u32,
    pub SPDIF_SPDIF_IN1_SELECT_INPUT: u32,
    pub USB_IPP_IND_OTG_OC_SELECT_INPUT: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_14: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_15: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_16: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_17: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_10: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_12: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_13: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_18: u32,
    pub XBAR1_XBAR_IN_SELECT_INPUT_19: u32,
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
        SW_MUX_CTL_PAD_GPIO_EMC_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_16: 0x00000006,
        SW_MUX_CTL_PAD_GPIO_EMC_17: 0x00000006,
        SW_MUX_CTL_PAD_GPIO_EMC_18: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_19: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_20: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_21: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_22: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_23: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_24: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_25: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_26: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_27: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_32: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_33: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_34: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_35: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_00: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_01: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_02: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_03: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_04: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_05: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B1_11: 0x00000005,
        SW_PAD_CTL_PAD_GPIO_EMC_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_16: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_EMC_17: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_EMC_18: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_19: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_20: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_21: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_22: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_23: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_24: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_25: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_26: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_27: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_32: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_33: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_34: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_35: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_00: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_01: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_02: 0x000030A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_03: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_04: 0x000090B1,
        SW_PAD_CTL_PAD_GPIO_AD_B0_05: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B1_11: 0x000010B0,
        ANATOP_USB_OTG_ID_SELECT_INPUT: 0x00000000,
        CCM_PMIC_VFUNCIONAL_READY_SELECT_INPUT: 0x00000000,
        FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_0: 0x00000000,
        FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_1: 0x00000000,
        FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_2: 0x00000000,
        FLEXPWM1_IPP_IND_PWMA_SELECT_INPUT_3: 0x00000000,
        FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_0: 0x00000000,
        FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_1: 0x00000000,
        FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_2: 0x00000000,
        FLEXPWM1_IPP_IND_PWMB_SELECT_INPUT_3: 0x00000000,
        FLEXSPI_IPP_IND_IO_FA_BIT0_SELECT_INPUT: 0x00000000,
        FLEXSPI_IPP_IND_IO_FA_BIT1_SELECT_INPUT: 0x00000000,
        FLEXSPI_IPP_IND_IO_FA_BIT2_SELECT_INPUT: 0x00000000,
        FLEXSPI_IPP_IND_IO_FA_BIT3_SELECT_INPUT: 0x00000000,
        FLEXSPI_IPP_IND_SCK_FA_SELECT_INPUT: 0x00000000,
        LPI2C1_IPP_IND_LPI2C_SCL_SELECT_INPUT: 0x00000000,
        LPI2C1_IPP_IND_LPI2C_SDA_SELECT_INPUT: 0x00000000,
        LPI2C2_IPP_IND_LPI2C_SCL_SELECT_INPUT: 0x00000000,
        LPI2C2_IPP_IND_LPI2C_SDA_SELECT_INPUT: 0x00000000,
        LPSPI1_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: 0x00000000,
        LPSPI1_IPP_IND_LPSPI_SCK_SELECT_INPUT: 0x00000000,
        LPSPI1_IPP_IND_LPSPI_SDI_SELECT_INPUT: 0x00000000,
        LPSPI1_IPP_IND_LPSPI_SDO_SELECT_INPUT: 0x00000000,
        LPSPI2_IPP_IND_LPSPI_PCS_SELECT_INPUT_0: 0x00000000,
        LPSPI2_IPP_IND_LPSPI_SCK_SELECT_INPUT: 0x00000000,
        LPSPI2_IPP_IND_LPSPI_SDI_SELECT_INPUT: 0x00000000,
        LPSPI2_IPP_IND_LPSPI_SDO_SELECT_INPUT: 0x00000000,
        LPUART2_IPP_IND_LPUART_CTS_B_SELECT_INPUT: 0x00000000,
        LPUART2_IPP_IND_LPUART_RXD_SELECT_INPUT: 0x00000000,
        LPUART2_IPP_IND_LPUART_TXD_SELECT_INPUT: 0x00000000,
        LPUART3_IPP_IND_LPUART_RXD_SELECT_INPUT: 0x00000000,
        LPUART3_IPP_IND_LPUART_TXD_SELECT_INPUT: 0x00000000,
        LPUART4_IPP_IND_LPUART_CTS_B_SELECT_INPUT: 0x00000000,
        LPUART4_IPP_IND_LPUART_RXD_SELECT_INPUT: 0x00000000,
        LPUART4_IPP_IND_LPUART_TXD_SELECT_INPUT: 0x00000000,
        NMI_GLUE_IPP_IND_NMI_SELECT_INPUT: 0x00000000,
        QTIMER1_TMR0_INPUT_SELECT_INPUT: 0x00000000,
        QTIMER1_TMR1_INPUT_SELECT_INPUT: 0x00000000,
        QTIMER1_TMR2_INPUT_SELECT_INPUT: 0x00000000,
        QTIMER1_TMR3_INPUT_SELECT_INPUT: 0x00000000,
        SAI1_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: 0x00000000,
        SAI1_IPP_IND_SAI_RXBCLK_SELECT_INPUT: 0x00000000,
        SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: 0x00000000,
        SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_1: 0x00000000,
        SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_2: 0x00000000,
        SAI1_IPP_IND_SAI_RXDATA_SELECT_INPUT_3: 0x00000000,
        SAI1_IPP_IND_SAI_RXSYNC_SELECT_INPUT: 0x00000000,
        SAI1_IPP_IND_SAI_TXBCLK_SELECT_INPUT: 0x00000000,
        SAI1_IPP_IND_SAI_TXSYNC_SELECT_INPUT: 0x00000000,
        SAI2_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: 0x00000000,
        SAI2_IPP_IND_SAI_RXBCLK_SELECT_INPUT: 0x00000000,
        SAI2_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: 0x00000000,
        SAI2_IPP_IND_SAI_RXSYNC_SELECT_INPUT: 0x00000000,
        SAI2_IPP_IND_SAI_TXBCLK_SELECT_INPUT: 0x00000000,
        SAI2_IPP_IND_SAI_TXSYNC_SELECT_INPUT: 0x00000000,
        SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: 0x00000000,
        SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: 0x00000000,
        SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: 0x00000000,
        SPDIF_SPDIF_IN1_SELECT_INPUT: 0x00000000,
        USB_IPP_IND_OTG_OC_SELECT_INPUT: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_14: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_15: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_16: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_17: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_10: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_12: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_13: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_18: 0x00000000,
        XBAR1_XBAR_IN_SELECT_INPUT_19: 0x00000000,
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
