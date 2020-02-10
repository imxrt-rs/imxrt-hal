#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_00 {

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA00 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI2_SCK of instance: lpspi2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_XBAR_IN02 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO00 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO00 of instance: gpio4
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

            /// 0b1: Force input path of pad GPIO_EMC_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_01 {

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA01 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMB00 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI2_PCS0 of instance: lpspi2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_IN03 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO01 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO01 of instance: gpio4
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

            /// 0b1: Force input path of pad GPIO_EMC_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_02 {

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA02 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI2_SDO of instance: lpspi2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT04 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO02 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO02 of instance: gpio4
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

            /// 0b1: Force input path of pad GPIO_EMC_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_03 {

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA03 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMB01 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI2_SDI of instance: lpspi2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT05 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO03 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO03 of instance: gpio4
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

            /// 0b1: Force input path of pad GPIO_EMC_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA04 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SAI2_TX_DATA of instance: sai2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT06 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO04 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO04 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA05 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMB02 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SAI2_TX_SYNC of instance: sai2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT07 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO05 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO05 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA06 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SAI2_TX_BCLK of instance: sai2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT08 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO06 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO06 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DATA07 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SAI2_MCLK of instance: sai2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT09 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO07 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO07 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_DM00 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: SAI2_RX_DATA of instance: sai2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: XBAR1_INOUT17 of instance: xbar1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO08 of instance: flexio1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO08 of instance: gpio4
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR00 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_RX_SYNC of instance: sai2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXCAN2_TX of instance: flexcan2
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO09 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO09 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

/// SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR01 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: SAI2_RX_BCLK of instance: sai2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: FLEXCAN2_RX of instance: flexcan2
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO10 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO10 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR02 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C4_SDA of instance: lpi2c4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: USDHC2_RESET_B of instance: usdhc2
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO11 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO11 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR03 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN24 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C4_SCL of instance: lpi2c4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: USDHC1_WP of instance: usdhc1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO12 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR04 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN25 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MQS_RIGHT of instance: mqs
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO13 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR05 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT19 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: MQS_LEFT of instance: mqs
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI2_PCS1 of instance: lpspi2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO14 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR06 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN20 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: QTIMER3_TIMER0 of instance: qtimer3
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO15 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_15
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR07 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN21 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_IN of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: QTIMER3_TIMER1 of instance: qtimer3
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO16 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_ADDR08 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_CTS_B of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: FLEXCAN1_TX of instance: flexcan1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: QTIMER3_TIMER2 of instance: qtimer3
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO17 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_ADDR09 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMB03 of instance: flexpwm4
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_RTS_B of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: FLEXCAN1_RX of instance: flexcan1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: QTIMER3_TIMER3 of instance: qtimer3
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO18 of instance: gpio4
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SNVS_VIO_5_CTL of instance: snvs_hp
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_ADDR11 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: ENET_RDATA01 of instance: enet
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: QTIMER2_TIMER0 of instance: qtimer2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO19 of instance: gpio4
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SNVS_VIO_5 of instance: snvs_hp
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_ADDR12 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: ENET_RDATA00 of instance: enet
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: QTIMER2_TIMER1 of instance: qtimer2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO20 of instance: gpio4
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_BA0 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMA03 of instance: flexpwm3
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPI2C3_SDA of instance: lpi2c3
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: ENET_TDATA01 of instance: enet
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: QTIMER2_TIMER2 of instance: qtimer2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO4_IO21 of instance: gpio4
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_BA1 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMB03 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C3_SCL of instance: lpi2c3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TDATA00 of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: QTIMER2_TIMER3 of instance: qtimer2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO22 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_ADDR10 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART5_TX of instance: lpuart5
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_EN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT1_CAPTURE2 of instance: gpt1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO23 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CAS of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART5_RX of instance: lpuart5
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_EN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: GPT1_CAPTURE1 of instance: gpt1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO24 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_RAS of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_CLK of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_REF_CLK of instance: enet
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO25 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CLK of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_ER of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO12 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO26 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CKE of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART5_RTS_B of instance: lpuart5
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI1_SCK of instance: lpspi1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO13 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO27 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

/// SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_28 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_WE of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART5_CTS_B of instance: lpuart5
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI1_SDO of instance: lpspi1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO14 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO28 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_28
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_29 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_CS0 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMA00 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART6_RTS_B of instance: lpuart6
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI1_SDI of instance: lpspi1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO1_FLEXIO15 of instance: flexio1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO29 of instance: gpio4
            pub const ALT5: u32 = 0b0101;
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

            /// 0b1: Force input path of pad GPIO_EMC_29
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_30 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA08 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMB00 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART6_CTS_B of instance: lpuart6
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI1_PCS0 of instance: lpspi1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA23 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO30 of instance: gpio4
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_EMC_30
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_31 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA09 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMA01 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI1_PCS1 of instance: lpspi1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA22 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO4_IO31 of instance: gpio4
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_EMC_31
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA10 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMB01 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: CCM_PMIC_RDY of instance: ccm
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA21 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO18 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA11 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMA02 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: USDHC1_RESET_B of instance: usdhc1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_DATA of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA20 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO19 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA12 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM3_PWMB02 of instance: flexpwm3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: USDHC1_VSELECT of instance: usdhc1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_SYNC of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA19 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO20 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA13 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT18 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE1 of instance: gpt1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_RX_BCLK of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA18 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO21 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

/// SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_36 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN22 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA17 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXCAN3_TX of instance: flexcan3/canfd
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_EMC_36
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_37 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DATA15 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN23 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: GPT1_COMPARE3 of instance: gpt1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_MCLK of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA16 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO23 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_WP of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXCAN3_RX of instance: flexcan3/canfd
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_EMC_37
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_38 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DM01 of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_BCLK of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_FIELD of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO24 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_VSELECT of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_MDC of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_EMC_38
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_39 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_DQS of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI3_TX_SYNC of instance: sai3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: WDOG1_WDOG_B of instance: wdog1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO25 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_CD_B of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_MDIO of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_EMC_39
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_40 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: SEMC_RDY of instance: semc
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: GPT2_CAPTURE2 of instance: gpt2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI1_PCS2 of instance: lpspi1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: USB_OTG2_OC of instance: usb
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_MDC of instance: enet
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO26 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_RESET_B of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_CLK5 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_EMC_40
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_EMC_41 {

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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_CSX00 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT2_CAPTURE1 of instance: gpt2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPSPI1_PCS3 of instance: lpspi1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG2_PWR of instance: usb
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: ENET_MDIO of instance: enet
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO27 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1
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

            /// 0b1: Force input path of pad GPIO_EMC_41
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

            /// 0b000: Select mux mode: ALT0 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_INOUT14 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: REF_CLK_32K of instance: xtalosc
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG2_ID of instance: usb
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPI2C1_SCLS of instance: lpi2c1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: USDHC1_RESET_B of instance: usdhc1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_SCK of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_INOUT15 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: REF_CLK_24M of instance: anatop
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG1_ID of instance: anatop
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPI2C1_SDAS of instance: lpi2c1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO01 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: EWM_OUT_B of instance: ewm
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_SDO of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: FLEXCAN2_TX of instance: flexcan2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: usb
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: FLEXCAN2_RX of instance: flexcan2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: XBAR1_INOUT17 of instance: xbar1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART6_RX of instance: lpuart6
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: USB_OTG1_OC of instance: usb
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMX01 of instance: flexpwm1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO03 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: REF_CLK_24M of instance: anatop
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_PCS0 of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: SRC_BOOT_MODE00 of instance: src
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: MQS_RIGHT of instance: mqs
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ENET_TX_DATA03 of instance: enet
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_TX_SYNC of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSI_DATA09 of instance: csi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO04 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: PIT_TRIGGER00 of instance: pit
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_PCS1 of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: SRC_BOOT_MODE01 of instance: src
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: MQS_LEFT of instance: mqs
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ENET_TX_DATA02 of instance: enet
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_TX_BCLK of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSI_DATA08 of instance: csi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO05 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: XBAR1_INOUT17 of instance: xbar1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_PCS2 of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_TMS of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT2_COMPARE1 of instance: gpt2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ENET_RX_CLK of instance: enet
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_BCLK of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSI_DATA07 of instance: csi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO06 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: XBAR1_INOUT18 of instance: xbar1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: LPSPI3_PCS3 of instance: lpspi3
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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_TCK of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT2_COMPARE2 of instance: gpt2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ENET_TX_ER of instance: enet
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_SYNC of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSI_DATA06 of instance: csi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO07 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: XBAR1_INOUT19 of instance: xbar1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ENET_1588_EVENT3_OUT of instance: enet
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

            /// 0b000: Select mux mode: ALT0 mux port: JTAG_MOD of instance: jtag_mux
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT2_COMPARE3 of instance: gpt2
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: ENET_RX_DATA03 of instance: enet
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI2_RX_DATA of instance: sai2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: CSI_DATA05 of instance: csi
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO08 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: XBAR1_IN20 of instance: xbar1
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: ENET_1588_EVENT3_IN of instance: enet
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_TDI of instance: jtag_mux
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_RX_DATA02 of instance: enet
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI2_TX_DATA of instance: sai2
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA04 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO09 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_IN21 of instance: xbar1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: GPT2_CLK of instance: gpt2
            pub const ALT7: u32 = 0b0111;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_TDO of instance: jtag_mux
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_CRS of instance: enet
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI2_MCLK of instance: sai2
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA03 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO10 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_IN22 of instance: xbar1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1588_EVENT0_OUT of instance: enet
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXCAN3_TX of instance: flexcan3/canfd
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ARM_TRACE_SWO of instance: cm7_mx6rt
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: JTAG_TRSTB of instance: jtag_mux
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ENET_COL of instance: enet
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: WDOG1_WDOG_B of instance: wdog1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA02 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO11 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: XBAR1_IN23 of instance: xbar1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: ENET_1588_EVENT0_IN of instance: enet
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXCAN3_RX of instance: flexcan3/canfd
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_CLK6 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C4_SCL of instance: lpi2c4
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: CCM_PMIC_READY of instance: ccm
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_TX of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: WDOG2_WDOG_B of instance: wdog2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMX02 of instance: flexpwm1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO12 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: ENET_1588_EVENT1_OUT of instance: enet
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

            /// 0b000: Select mux mode: ALT0 mux port: LPI2C4_SDA of instance: lpi2c4
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: GPT1_CLK of instance: gpt1
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART1_RX of instance: lpuart1
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: EWM_OUT_B of instance: ewm
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: FLEXPWM1_PWMX03 of instance: flexpwm1
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO1_IO13 of instance: gpio1
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: ENET_1588_EVENT1_IN of instance: enet
            pub const ALT6: u32 = 0b110;

            /// 0b111: Select mux mode: ALT7 mux port: REF_CLK_24M of instance: anatop
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG2_OC of instance: usb
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN24 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART1_CTS_B of instance: lpuart1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_OUT of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_VSYNC of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO14 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN2_TX of instance: flexcan2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXCAN3_TX of instance: flexcan3/canfd
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG2_PWR of instance: usb
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_IN25 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART1_RTS_B of instance: lpuart1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_IN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_HSYNC of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO15 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN2_RX of instance: flexcan2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: WDOG1_WDOG_RST_B_DEB of instance: wdog1
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: FLEXCAN3_RX of instance: flexcan3/canfd
            pub const ALT8: u32 = 0b1000;
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

/// SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG2_ID of instance: anatop
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER0 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_CTS_B of instance: lpuart2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPI2C1_SCL of instance: lpi2c1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: WDOG1_B of instance: wdog1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO16 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW07 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_OUT of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO00 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG1_PWR of instance: usb
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER1 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_RTS_B of instance: lpuart2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPI2C1_SDA of instance: lpi2c1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CCM_PMIC_READY of instance: ccm
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO17 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL07 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_IN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO01 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG1_ID of instance: anatop
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER2 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_1588_EVENT2_OUT of instance: enet
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO18 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW06 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_CLK of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO02 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USB_OTG1_OC of instance: usb
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER3 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_IN of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: ENET_1588_EVENT2_IN of instance: enet
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO19 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_CD_B of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL06 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_CAPTURE1 of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO03 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIB_DATA03 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_MDC of instance: enet
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_CTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_SR_CLK of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_PIXCLK of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO20 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA0 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW05 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_CAPTURE2 of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO04 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIB_DATA02 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ENET_MDIO of instance: enet
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_RTS_B of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_OUT of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_MCLK of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO21 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA1 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL05 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_COMPARE1 of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO05 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIB_DATA01 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C3_SDA of instance: lpi2c3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_TX of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_LOCK of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_VSYNC of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO22 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA2 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW04 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_COMPARE2 of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO06 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIB_DATA00 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPI2C3_SCL of instance: lpi2c3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART3_RX of instance: lpuart3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SPDIF_EXT_CLK of instance: spdif
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_HSYNC of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO23 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA3 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL04 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT2_COMPARE3 of instance: gpt2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO07 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_SS1_B of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA00 of instance: flexpwm4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_TX of instance: flexcan1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: CCM_PMIC_READY of instance: ccm
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA09 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO24 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_CMD of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW03 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO08 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_AD_B1_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_DQS of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA01 of instance: flexpwm4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_RX of instance: flexcan1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA08 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO25 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_CLK of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL03 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO09 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_AD_B1_09
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_DATA03 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: WDOG1_B of instance: wdog1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA07 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO26 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_WP of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW02 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT1_OUT of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO10 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_DATA02 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: EWM_OUT_B of instance: ewm
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA06 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO27 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_RESET_B of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL02 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT1_IN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO11 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_DATA01 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP_OUT00 of instance: acmp
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI3_PCS0 of instance: lpspi3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA05 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO28 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA4 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW01 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT2_OUT of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO12 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_DATA00 of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP_OUT01 of instance: acmp
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI3_SDI of instance: lpspi3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA04 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO29 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA5 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL01 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT2_IN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO13 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_SCLK of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP_OUT02 of instance: acmp
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI3_SDO of instance: lpspi3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA03 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO30 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA6 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_ROW00 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT3_OUT of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO14 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: FLEXSPIA_SS0_B of instance: flexspi
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: ACMP_OUT03 of instance: acmp
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI3_SCK of instance: lpspi3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: CSI_DATA02 of instance: csi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO1_IO31 of instance: gpio1
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC2_DATA7 of instance: usdhc2
            pub const ALT6: u32 = 0b0110;

            /// 0b0111: Select mux mode: ALT7 mux port: KPP_COL00 of instance: kpp
            pub const ALT7: u32 = 0b0111;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT3_IN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO15 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

/// SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_CLK of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER1_TIMER0 of instance: qtimer1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_RIGHT of instance: mqs
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI4_PCS0 of instance: lpspi4
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO00 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO00 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SEMC_CSX01 of instance: semc
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_MDC of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_ENABLE of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER1_TIMER1 of instance: qtimer1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: MQS_LEFT of instance: mqs
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI4_SDI of instance: lpspi4
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO01 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO01 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SEMC_CSX02 of instance: semc
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_MDIO of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_HSYNC of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER1_TIMER2 of instance: qtimer1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_TX of instance: flexcan1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI4_SDO of instance: lpspi4
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO02 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO02 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SEMC_CSX03 of instance: semc
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_OUT of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_VSYNC of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER2_TIMER0 of instance: qtimer2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXCAN1_RX of instance: flexcan1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPSPI4_SCK of instance: lpspi4
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO03 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO03 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: WDOG2_RESET_B_DEB of instance: wdog2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_1588_EVENT0_IN of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA00 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER2_TIMER1 of instance: qtimer2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C2_SCL of instance: lpi2c2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE0 of instance: cm7_mx6rt
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO04 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO04 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG00 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA03 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA01 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER2_TIMER2 of instance: qtimer2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C2_SDA of instance: lpi2c2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE1 of instance: cm7_mx6rt
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO05 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO05 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG01 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA02 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA02 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER0 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMA00 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE2 of instance: cm7_mx6rt
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO06 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO06 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG02 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_CLK of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA03 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER1 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMB00 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ARM_TRACE3 of instance: cm7_mx6rt
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO07 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO07 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG03 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_ER of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA04 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER2 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMA01 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART3_TX of instance: lpuart3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO08 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO08 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG04 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA03 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA05 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER4_TIMER0 of instance: qtimer4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMB01 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: LPUART3_RX of instance: lpuart3
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO09 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO09 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG05 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA02 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA06 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER4_TIMER1 of instance: qtimer4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMA02 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO10 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO10 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG06 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_CRS of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA07 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER4_TIMER2 of instance: qtimer4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMB02 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO11 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO11 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG07 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_COL of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA08 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT10 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ARM_TRACE_CLK of instance: cm7_mx6rt
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO12 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO12 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG08 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA09 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT11 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ARM_TRACE_SWO of instance: cm7_mx6rt
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO13 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO13 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG09 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA10 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT12 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ARM_TXEV of instance: cm7_mx6rt
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO14 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO14 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG10 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_B0_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B0_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA11 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT13 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: ARM_RXEV of instance: cm7_mx6rt
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO15 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO15 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: SRC_BOOT_CFG11 of instance: src
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B0_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA12 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT14 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_TX of instance: lpuart4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO16 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO16 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO16 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA13 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT15 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART4_RX of instance: lpuart4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO17 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO17 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO17 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA14 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI4_PCS2 of instance: lpspi4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO18 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO18 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO18 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA15 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: XBAR1_INOUT17 of instance: xbar1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPSPI4_PCS1 of instance: lpspi4
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO19 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO19 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO19 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA16 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI4_PCS0 of instance: lpspi4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA15 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_DATA00 of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO20 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO20 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_CLK of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO20 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA17 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI4_SDI of instance: lpspi4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA14 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_DATA01 of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO21 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO21 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_CAPTURE1 of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO21 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_05
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_06 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA18 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI4_SDO of instance: lpspi4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA13 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_EN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO22 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO22 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_CAPTURE2 of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO22 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_06
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_07 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA19 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPSPI4_SCK of instance: lpspi4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA12 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_DATA00 of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO23 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO23 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_COMPARE1 of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO23 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_07
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_08 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA20 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER1_TIMER3 of instance: qtimer1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA11 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_DATA01 of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO24 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO24 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN2_TX of instance: flexcan2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_COMPARE2 of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO24 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_08
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_09 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA21 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER2_TIMER3 of instance: qtimer2
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA10 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_EN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO25 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO25 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXCAN2_RX of instance: flexcan2
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: GPT1_COMPARE3 of instance: gpt1
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO25 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_09
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_10 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA22 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER3_TIMER3 of instance: qtimer3
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA00 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_TX_CLK of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO26 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO26 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: ENET_REF_CLK of instance: enet
            pub const ALT6: u32 = 0b0110;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO26 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_10
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_11 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: LCD_DATA23 of instance: lcdif
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: QTIMER4_TIMER3 of instance: qtimer4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_DATA01 of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_RX_ER of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO27 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO27 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: LPSPI4_PCS3 of instance: lpspi4
            pub const ALT6: u32 = 0b0110;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO27 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_11
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_12 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART5_TX of instance: lpuart5
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_PIXCLK of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_IN of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO28 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO28 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_CD_B of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO28 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_12
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_13 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: WDOG1_B of instance: wdog1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: LPUART5_RX of instance: lpuart5
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_VSYNC of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: ENET_1588_EVENT0_OUT of instance: enet
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO29 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO29 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: SEMC_DQS4 of instance: semc
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO29 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_13
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_14 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: ENET_MDC of instance: enet
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA02 of instance: flexpwm4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_HSYNC of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_IN02 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO30 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO30 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_VSELECT of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO30 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_14
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_B1_15 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: ENET_MDIO of instance: enet
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM4_PWMA03 of instance: flexpwm4
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: CSI_MCLK of instance: csi
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_IN03 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXIO2_FLEXIO31 of instance: flexio2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO2_IO31 of instance: gpio2
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: USDHC1_RESET_B of instance: usdhc1
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: FLEXIO3_FLEXIO31 of instance: flexio3
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_B1_15
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_00 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_CMD of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA00 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C3_SCL of instance: lpi2c3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT04 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI1_SCK of instance: lpspi1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO12 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPIA_SS1_B of instance: flexspi
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_DQS4 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_00
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_01 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_CLK of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB00 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C3_SDA of instance: lpi2c3
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT05 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI1_PCS0 of instance: lpspi1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO13 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: FLEXSPIB_SS1_B of instance: flexspi
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_TX_CLK of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: ENET2_REF_CLK2 of instance: enet2
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_01
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_02 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA0 of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA01 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_CTS_B of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT06 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI1_SDO of instance: lpspi1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO14 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_ER of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_CLK5 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_02
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_03 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA1 of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB01 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_RTS_B of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT07 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI1_SDI of instance: lpspi1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO15 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA00 of instance: enet2
            pub const ALT8: u32 = 0b1000;

            /// 0b1001: Select mux mode: ALT9 mux port: SEMC_CLK6 of instance: semc
            pub const ALT9: u32 = 0b1001;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_03
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_04 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA2 of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMA02 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_TX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT08 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPIB_SS0_B of instance: flexspi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO16 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: CCM_CLKO1 of instance: ccm
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_04
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register
pub mod SW_MUX_CTL_PAD_GPIO_SD_B0_05 {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC1_DATA3 of instance: usdhc1
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXPWM1_PWMB02 of instance: flexpwm1
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART8_RX of instance: lpuart8
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: XBAR1_INOUT09 of instance: xbar1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPIB_DQS of instance: flexspi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO17 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: CCM_CLKO2 of instance: ccm
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: ENET2_RX_EN of instance: enet2
            pub const ALT8: u32 = 0b1000;
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

            /// 0b1: Force input path of pad GPIO_SD_B0_05
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA3 of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIB_DATA03 of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM1_PWMA03 of instance: flexpwm1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA03 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPUART4_TX of instance: lpuart4
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO00 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_RX_DATA of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA2 of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIB_DATA02 of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM1_PWMB03 of instance: flexpwm1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA02 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPUART4_RX of instance: lpuart4
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO01 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_TX_DATA of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA1 of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIB_DATA01 of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMA03 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_TX_DATA01 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXCAN1_TX of instance: flexcan1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO02 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: CCM_WAIT of instance: ccm
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_TX_SYNC of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_DATA0 of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIB_DATA00 of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: FLEXPWM2_PWMB03 of instance: flexpwm2
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_MCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXCAN1_RX of instance: flexcan1
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO03 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: CCM_PMIC_READY of instance: ccm
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_TX_BCLK of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_CLK of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIB_SCLK of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C1_SCL of instance: lpi2c1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPIA_SS1_B of instance: flexspi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO04 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b0110: Select mux mode: ALT6 mux port: CCM_STOP of instance: ccm
            pub const ALT6: u32 = 0b0110;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_MCLK of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_CMD of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIA_DQS of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPI2C1_SDA of instance: lpi2c1
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: FLEXSPIB_SS0_B of instance: flexspi
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO05 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_RX_SYNC of instance: sai3
            pub const ALT8: u32 = 0b1000;
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
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Select mux mode: ALT0 mux port: USDHC2_RESET_B of instance: usdhc2
            pub const ALT0: u32 = 0b0000;

            /// 0b0001: Select mux mode: ALT1 mux port: FLEXSPIA_SS0_B of instance: flexspi
            pub const ALT1: u32 = 0b0001;

            /// 0b0010: Select mux mode: ALT2 mux port: LPUART7_CTS_B of instance: lpuart7
            pub const ALT2: u32 = 0b0010;

            /// 0b0011: Select mux mode: ALT3 mux port: SAI1_RX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b0011;

            /// 0b0100: Select mux mode: ALT4 mux port: LPSPI2_PCS0 of instance: lpspi2
            pub const ALT4: u32 = 0b0100;

            /// 0b0101: Select mux mode: ALT5 mux port: GPIO3_IO06 of instance: gpio3
            pub const ALT5: u32 = 0b0101;

            /// 0b1000: Select mux mode: ALT8 mux port: SAI3_RX_BCLK of instance: sai3
            pub const ALT8: u32 = 0b1000;
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

            /// 0b000: Select mux mode: ALT0 mux port: SEMC_CSX01 of instance: semc
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPIA_SCLK of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART7_RTS_B of instance: lpuart7
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_DATA00 of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SCK of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO07 of instance: gpio3
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

            /// 0b000: Select mux mode: ALT0 mux port: USDHC2_DATA4 of instance: usdhc2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPIA_DATA00 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART7_TX of instance: lpuart7
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_BCLK of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SD0 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO08 of instance: gpio3
            pub const ALT5: u32 = 0b101;

            /// 0b110: Select mux mode: ALT6 mux port: SEMC_CSX02 of instance: semc
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

            /// 0b000: Select mux mode: ALT0 mux port: USDHC2_DATA5 of instance: usdhc2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPIA_DATA01 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART7_RX of instance: lpuart7
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: SAI1_TX_SYNC of instance: sai1
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_SDI of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO09 of instance: gpio3
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

            /// 0b000: Select mux mode: ALT0 mux port: USDHC2_DATA6 of instance: usdhc2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPIA_DATA02 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_RX of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SDA of instance: lpi2c2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_PCS2 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO10 of instance: gpio3
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

            /// 0b000: Select mux mode: ALT0 mux port: USDHC2_DATA7 of instance: usdhc2
            pub const ALT0: u32 = 0b000;

            /// 0b001: Select mux mode: ALT1 mux port: FLEXSPIA_DATA03 of instance: flexspi
            pub const ALT1: u32 = 0b001;

            /// 0b010: Select mux mode: ALT2 mux port: LPUART2_TX of instance: lpuart2
            pub const ALT2: u32 = 0b010;

            /// 0b011: Select mux mode: ALT3 mux port: LPI2C2_SCL of instance: lpi2c2
            pub const ALT3: u32 = 0b011;

            /// 0b100: Select mux mode: ALT4 mux port: LPSPI2_PCS3 of instance: lpspi2
            pub const ALT4: u32 = 0b100;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO3_IO11 of instance: gpio3
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

/// SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_00 {

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
            pub const DSE_0_output_driver_disabled: u32 = 0b000;

            /// 0b001: R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)
            pub const DSE_1_R0_150_Ohm_3_3V_260_Ohm_1_8V: u32 = 0b001;

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

            /// 0b10: medium(100MHz)
            pub const SPEED_2_medium_100MHz: u32 = 0b10;

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

/// SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_16 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_17 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_18 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_19 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_20 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_21 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_22 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_23 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_24 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_25 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_26 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_27 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_28 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_29 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_30 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_31 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_32 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_33 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_34 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_35 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_36 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_37 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_38 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_39 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_40 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_EMC_41 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B0_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_AD_B1_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B0_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_14 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_B1_15 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B0_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SD_B1_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_EMC_00::SRE;
}

/// ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register
pub mod ANATOP_USB_OTG1_ID_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_01 for Mode: ALT3
            pub const GPIO_AD_B0_01_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_02 for Mode: ALT0
            pub const GPIO_AD_B1_02_ALT0: u32 = 0b1;
        }
    }
}

/// ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register
pub mod ANATOP_USB_OTG2_ID_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_00 for Mode: ALT3
            pub const GPIO_AD_B0_00_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_00 for Mode: ALT0
            pub const GPIO_AD_B1_00_ALT0: u32 = 0b1;
        }
    }
}

/// CCM_PMIC_READY_SELECT_INPUT DAISY Register
pub mod CCM_PMIC_READY_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b000: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6
            pub const GPIO_SD_B1_03_ALT6: u32 = 0b000;

            /// 0b001: Selecting Pad: GPIO_AD_B0_12 for Mode: ALT1
            pub const GPIO_AD_B0_12_ALT1: u32 = 0b001;

            /// 0b010: Selecting Pad: GPIO_AD_B1_01 for Mode: ALT4
            pub const GPIO_AD_B1_01_ALT4: u32 = 0b010;

            /// 0b011: Selecting Pad: GPIO_AD_B1_08 for Mode: ALT3
            pub const GPIO_AD_B1_08_ALT3: u32 = 0b011;

            /// 0b100: Selecting Pad: GPIO_EMC_32 for Mode: ALT3
            pub const GPIO_EMC_32_ALT3: u32 = 0b100;
        }
    }
}

/// CSI_DATA02_SELECT_INPUT DAISY Register
pub mod CSI_DATA02_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_15 for Mode: ALT4
            pub const GPIO_AD_B1_15_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT4
            pub const GPIO_AD_B0_11_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA03_SELECT_INPUT DAISY Register
pub mod CSI_DATA03_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT4
            pub const GPIO_AD_B1_14_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_10 for Mode: ALT4
            pub const GPIO_AD_B0_10_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA04_SELECT_INPUT DAISY Register
pub mod CSI_DATA04_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_13 for Mode: ALT4
            pub const GPIO_AD_B1_13_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT4
            pub const GPIO_AD_B0_09_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA05_SELECT_INPUT DAISY Register
pub mod CSI_DATA05_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT4
            pub const GPIO_AD_B1_12_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_08 for Mode: ALT4
            pub const GPIO_AD_B0_08_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA06_SELECT_INPUT DAISY Register
pub mod CSI_DATA06_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT4
            pub const GPIO_AD_B1_11_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_07 for Mode: ALT4
            pub const GPIO_AD_B0_07_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA07_SELECT_INPUT DAISY Register
pub mod CSI_DATA07_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT4
            pub const GPIO_AD_B1_10_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_06 for Mode: ALT4
            pub const GPIO_AD_B0_06_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA08_SELECT_INPUT DAISY Register
pub mod CSI_DATA08_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT4
            pub const GPIO_AD_B1_09_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT4
            pub const GPIO_AD_B0_05_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_DATA09_SELECT_INPUT DAISY Register
pub mod CSI_DATA09_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_08 for Mode: ALT4
            pub const GPIO_AD_B1_08_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_04 for Mode: ALT4
            pub const GPIO_AD_B0_04_ALT4: u32 = 0b1;
        }
    }
}

/// CSI_HSYNC_SELECT_INPUT DAISY Register
pub mod CSI_HSYNC_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT4
            pub const GPIO_AD_B0_15_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT4
            pub const GPIO_AD_B1_07_ALT4: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_14 for Mode: ALT2
            pub const GPIO_B1_14_ALT2: u32 = 0b10;
        }
    }
}

/// CSI_PIXCLK_SELECT_INPUT DAISY Register
pub mod CSI_PIXCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_04 for Mode: ALT4
            pub const GPIO_AD_B1_04_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_12 for Mode: ALT2
            pub const GPIO_B1_12_ALT2: u32 = 0b1;
        }
    }
}

/// CSI_VSYNC_SELECT_INPUT DAISY Register
pub mod CSI_VSYNC_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B0_14 for Mode: ALT4
            pub const GPIO_AD_B0_14_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_06 for Mode: ALT4
            pub const GPIO_AD_B1_06_ALT4: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_13 for Mode: ALT2
            pub const GPIO_B1_13_ALT2: u32 = 0b10;
        }
    }
}

/// ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register
pub mod ENET_IPG_CLK_RMII_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_25 for Mode: ALT4
            pub const GPIO_EMC_25_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_10 for Mode: ALT6
            pub const GPIO_B1_10_ALT6: u32 = 0b1;
        }
    }
}

/// ENET_MDIO_SELECT_INPUT DAISY Register
pub mod ENET_MDIO_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B1_05 for Mode: ALT1
            pub const GPIO_AD_B1_05_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_41 for Mode: ALT4
            pub const GPIO_EMC_41_ALT4: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_15 for Mode: ALT0
            pub const GPIO_B1_15_ALT0: u32 = 0b10;
        }
    }
}

/// ENET0_RXDATA_SELECT_INPUT DAISY Register
pub mod ENET0_RXDATA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_20 for Mode: ALT3
            pub const GPIO_EMC_20_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_04 for Mode: ALT3
            pub const GPIO_B1_04_ALT3: u32 = 0b1;
        }
    }
}

/// ENET1_RXDATA_SELECT_INPUT DAISY Register
pub mod ENET1_RXDATA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_19 for Mode: ALT3
            pub const GPIO_EMC_19_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_05 for Mode: ALT3
            pub const GPIO_B1_05_ALT3: u32 = 0b1;
        }
    }
}

/// ENET_RXEN_SELECT_INPUT DAISY Register
pub mod ENET_RXEN_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_23 for Mode: ALT3
            pub const GPIO_EMC_23_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_06 for Mode: ALT3
            pub const GPIO_B1_06_ALT3: u32 = 0b1;
        }
    }
}

/// ENET_RXERR_SELECT_INPUT DAISY Register
pub mod ENET_RXERR_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_26 for Mode: ALT3
            pub const GPIO_EMC_26_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_11 for Mode: ALT3
            pub const GPIO_B1_11_ALT3: u32 = 0b1;
        }
    }
}

/// ENET0_TIMER_SELECT_INPUT DAISY Register
pub mod ENET0_TIMER_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT3
            pub const GPIO_AD_B0_15_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT7
            pub const GPIO_AD_B0_11_ALT7: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_12 for Mode: ALT3
            pub const GPIO_B1_12_ALT3: u32 = 0b10;
        }
    }
}

/// ENET_TXCLK_SELECT_INPUT DAISY Register
pub mod ENET_TXCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_25 for Mode: ALT3
            pub const GPIO_EMC_25_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_10 for Mode: ALT3
            pub const GPIO_B1_10_ALT3: u32 = 0b1;
        }
    }
}

/// FLEXCAN1_RX_SELECT_INPUT DAISY Register
pub mod FLEXCAN1_RX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT4
            pub const GPIO_SD_B1_03_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_18 for Mode: ALT3
            pub const GPIO_EMC_18_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT2
            pub const GPIO_AD_B1_09_ALT2: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B0_03 for Mode: ALT2
            pub const GPIO_B0_03_ALT2: u32 = 0b11;
        }
    }
}

/// FLEXCAN2_RX_SELECT_INPUT DAISY Register
pub mod FLEXCAN2_RX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_10 for Mode: ALT3
            pub const GPIO_EMC_10_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0
            pub const GPIO_AD_B0_03_ALT0: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6
            pub const GPIO_AD_B0_15_ALT6: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B1_09 for Mode: ALT6
            pub const GPIO_B1_09_ALT6: u32 = 0b11;
        }
    }
}

/// FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMA3_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b000: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2
            pub const GPIO_SD_B1_00_ALT2: u32 = 0b000;

            /// 0b001: Selecting Pad: GPIO_EMC_12 for Mode: ALT4
            pub const GPIO_EMC_12_ALT4: u32 = 0b001;

            /// 0b010: Selecting Pad: GPIO_EMC_38 for Mode: ALT1
            pub const GPIO_EMC_38_ALT1: u32 = 0b010;

            /// 0b011: Selecting Pad: GPIO_AD_B0_10 for Mode: ALT1
            pub const GPIO_AD_B0_10_ALT1: u32 = 0b011;

            /// 0b100: Selecting Pad: GPIO_B1_00 for Mode: ALT6
            pub const GPIO_B1_00_ALT6: u32 = 0b100;
        }
    }
}

/// FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_23 for Mode: ALT1
            pub const GPIO_EMC_23_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT1
            pub const GPIO_SD_B0_00_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_25 for Mode: ALT1
            pub const GPIO_EMC_25_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_02 for Mode: ALT1
            pub const GPIO_SD_B0_02_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_27 for Mode: ALT1
            pub const GPIO_EMC_27_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_04 for Mode: ALT1
            pub const GPIO_SD_B0_04_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMB3_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b000: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2
            pub const GPIO_SD_B1_01_ALT2: u32 = 0b000;

            /// 0b001: Selecting Pad: GPIO_EMC_13 for Mode: ALT4
            pub const GPIO_EMC_13_ALT4: u32 = 0b001;

            /// 0b010: Selecting Pad: GPIO_EMC_39 for Mode: ALT1
            pub const GPIO_EMC_39_ALT1: u32 = 0b010;

            /// 0b011: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1
            pub const GPIO_AD_B0_11_ALT1: u32 = 0b011;

            /// 0b100: Selecting Pad: GPIO_B1_01 for Mode: ALT6
            pub const GPIO_B1_01_ALT6: u32 = 0b100;
        }
    }
}

/// FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMB0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_24 for Mode: ALT1
            pub const GPIO_EMC_24_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT1
            pub const GPIO_SD_B0_01_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMB1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_26 for Mode: ALT1
            pub const GPIO_EMC_26_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_03 for Mode: ALT1
            pub const GPIO_SD_B0_03_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register
pub mod FLEXPWM1_PWMB2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_28 for Mode: ALT1
            pub const GPIO_EMC_28_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_05 for Mode: ALT1
            pub const GPIO_SD_B0_05_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMA3_SELECT_INPUT {

    /// Selecting Pads Involved in Daisy Chain.
    pub mod DAISY {
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

            /// 0b000: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2
            pub const GPIO_SD_B1_02_ALT2: u32 = 0b000;

            /// 0b001: Selecting Pad: GPIO_EMC_19 for Mode: ALT1
            pub const GPIO_EMC_19_ALT1: u32 = 0b001;

            /// 0b010: Selecting Pad: GPIO_AD_B0_00 for Mode: ALT0
            pub const GPIO_AD_B0_00_ALT0: u32 = 0b010;

            /// 0b011: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT1
            pub const GPIO_AD_B0_09_ALT1: u32 = 0b011;

            /// 0b100: Selecting Pad: GPIO_B1_02 for Mode: ALT6
            pub const GPIO_B1_02_ALT6: u32 = 0b100;
        }
    }
}

/// FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_06 for Mode: ALT1
            pub const GPIO_EMC_06_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_06 for Mode: ALT2
            pub const GPIO_B0_06_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_08 for Mode: ALT1
            pub const GPIO_EMC_08_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_08 for Mode: ALT2
            pub const GPIO_B0_08_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_10 for Mode: ALT1
            pub const GPIO_EMC_10_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_10 for Mode: ALT2
            pub const GPIO_B0_10_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMB3_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2
            pub const GPIO_SD_B1_03_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_20 for Mode: ALT1
            pub const GPIO_EMC_20_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_01 for Mode: ALT0
            pub const GPIO_AD_B0_01_ALT0: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B1_03 for Mode: ALT6
            pub const GPIO_B1_03_ALT6: u32 = 0b11;
        }
    }
}

/// FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMB0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_07 for Mode: ALT1
            pub const GPIO_EMC_07_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_07 for Mode: ALT2
            pub const GPIO_B0_07_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMB1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_09 for Mode: ALT1
            pub const GPIO_EMC_09_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_09 for Mode: ALT2
            pub const GPIO_B0_09_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register
pub mod FLEXPWM2_PWMB2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_11 for Mode: ALT1
            pub const GPIO_EMC_11_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_11 for Mode: ALT2
            pub const GPIO_B0_11_ALT2: u32 = 0b1;
        }
    }
}

/// FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register
pub mod FLEXPWM4_PWMA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_00 for Mode: ALT1
            pub const GPIO_EMC_00_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_08 for Mode: ALT1
            pub const GPIO_AD_B1_08_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register
pub mod FLEXPWM4_PWMA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_02 for Mode: ALT1
            pub const GPIO_EMC_02_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT1
            pub const GPIO_AD_B1_09_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register
pub mod FLEXPWM4_PWMA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_04 for Mode: ALT1
            pub const GPIO_EMC_04_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_14 for Mode: ALT1
            pub const GPIO_B1_14_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register
pub mod FLEXPWM4_PWMA3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_17 for Mode: ALT1
            pub const GPIO_EMC_17_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_15 for Mode: ALT1
            pub const GPIO_B1_15_ALT1: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_DQS_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_DQS_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT1
            pub const GPIO_SD_B1_05_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT0
            pub const GPIO_AD_B1_09_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_DATA0_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_DATA0_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_13 for Mode: ALT0
            pub const GPIO_AD_B1_13_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_DATA1_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_DATA1_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT0
            pub const GPIO_AD_B1_12_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_DATA2_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_DATA2_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT0
            pub const GPIO_AD_B1_11_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_DATA3_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_DATA3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_11 for Mode: ALT1
            pub const GPIO_SD_B1_11_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT0
            pub const GPIO_AD_B1_10_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIB_DATA0_SELECT_INPUT DAISY Register
pub mod FLEXSPIB_DATA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT1
            pub const GPIO_SD_B1_03_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT0
            pub const GPIO_AD_B1_07_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIB_DATA1_SELECT_INPUT DAISY Register
pub mod FLEXSPIB_DATA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT1
            pub const GPIO_SD_B1_02_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_06 for Mode: ALT0
            pub const GPIO_AD_B1_06_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIB_DATA2_SELECT_INPUT DAISY Register
pub mod FLEXSPIB_DATA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT1
            pub const GPIO_SD_B1_01_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_05 for Mode: ALT0
            pub const GPIO_AD_B1_05_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIB_DATA3_SELECT_INPUT DAISY Register
pub mod FLEXSPIB_DATA3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT1
            pub const GPIO_SD_B1_00_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_04 for Mode: ALT0
            pub const GPIO_AD_B1_04_ALT0: u32 = 0b1;
        }
    }
}

/// FLEXSPIA_SCK_SELECT_INPUT DAISY Register
pub mod FLEXSPIA_SCK_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT0
            pub const GPIO_AD_B1_14_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C1_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C1_SCL_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT2
            pub const GPIO_SD_B1_04_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_00 for Mode: ALT3
            pub const GPIO_AD_B1_00_ALT3: u32 = 0b1;
        }
    }
}

/// LPI2C1_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C1_SDA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT2
            pub const GPIO_SD_B1_05_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_01 for Mode: ALT3
            pub const GPIO_AD_B1_01_ALT3: u32 = 0b1;
        }
    }
}

/// LPI2C2_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C2_SCL_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_B0_04 for Mode: ALT2
            pub const GPIO_B0_04_ALT2: u32 = 0b1;
        }
    }
}

/// LPI2C2_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C2_SDA_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_B0_05 for Mode: ALT2
            pub const GPIO_B0_05_ALT2: u32 = 0b1;
        }
    }
}

/// LPI2C3_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C3_SCL_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_22 for Mode: ALT2
            pub const GPIO_EMC_22_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT2
            pub const GPIO_SD_B0_00_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT1
            pub const GPIO_AD_B1_07_ALT1: u32 = 0b10;
        }
    }
}

/// LPI2C3_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C3_SDA_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_21 for Mode: ALT2
            pub const GPIO_EMC_21_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT2
            pub const GPIO_SD_B0_01_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B1_06 for Mode: ALT1
            pub const GPIO_AD_B1_06_ALT1: u32 = 0b10;
        }
    }
}

/// LPI2C4_SCL_SELECT_INPUT DAISY Register
pub mod LPI2C4_SCL_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_12 for Mode: ALT2
            pub const GPIO_EMC_12_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_12 for Mode: ALT0
            pub const GPIO_AD_B0_12_ALT0: u32 = 0b1;
        }
    }
}

/// LPI2C4_SDA_SELECT_INPUT DAISY Register
pub mod LPI2C4_SDA_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_11 for Mode: ALT2
            pub const GPIO_EMC_11_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_13 for Mode: ALT0
            pub const GPIO_AD_B0_13_ALT0: u32 = 0b1;
        }
    }
}

/// LPSPI1_PCS0_SELECT_INPUT DAISY Register
pub mod LPSPI1_PCS0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT4
            pub const GPIO_SD_B0_01_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_30 for Mode: ALT3
            pub const GPIO_EMC_30_ALT3: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_27 for Mode: ALT3
            pub const GPIO_EMC_27_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT4
            pub const GPIO_SD_B0_00_ALT4: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_29 for Mode: ALT3
            pub const GPIO_EMC_29_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT4
            pub const GPIO_SD_B0_03_ALT4: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_28 for Mode: ALT3
            pub const GPIO_EMC_28_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_02 for Mode: ALT4
            pub const GPIO_SD_B0_02_ALT4: u32 = 0b1;
        }
    }
}

/// LPSPI2_PCS0_SELECT_INPUT DAISY Register
pub mod LPSPI2_PCS0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT4
            pub const GPIO_SD_B1_06_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_01 for Mode: ALT2
            pub const GPIO_EMC_01_ALT2: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4
            pub const GPIO_SD_B1_07_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_00 for Mode: ALT2
            pub const GPIO_EMC_00_ALT2: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT4
            pub const GPIO_SD_B1_09_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_03 for Mode: ALT2
            pub const GPIO_EMC_03_ALT2: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT4
            pub const GPIO_SD_B1_08_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_02 for Mode: ALT2
            pub const GPIO_EMC_02_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI3_PCS0_SELECT_INPUT DAISY Register
pub mod LPSPI3_PCS0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT7
            pub const GPIO_AD_B0_03_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT2
            pub const GPIO_AD_B1_12_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI3_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI3_SCK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_00 for Mode: ALT7
            pub const GPIO_AD_B0_00_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_15 for Mode: ALT2
            pub const GPIO_AD_B1_15: u32 = 0b1;
        }
    }
}

/// LPSPI3_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI3_SDI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_02 for Mode: ALT7
            pub const GPIO_AD_B0_02_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_13 for Mode: ALT2
            pub const GPIO_AD_B1_13_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI3_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI3_SDO_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_01 for Mode: ALT7
            pub const GPIO_AD_B0_01_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT2
            pub const GPIO_AD_B1_14_ALT2: u32 = 0b1;
        }
    }
}

/// LPSPI4_PCS0_SELECT_INPUT DAISY Register
pub mod LPSPI4_PCS0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_B0_00 for Mode: ALT3
            pub const GPIO_B0_00_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad:GPIO_B1_04 for Mode: ALT1
            pub const GPIO_B1_04_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI4_SCK_SELECT_INPUT DAISY Register
pub mod LPSPI4_SCK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_B0_03 for Mode: ALT3
            pub const GPIO_B0_03_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_07 for Mode: ALT1
            pub const GPIO_B1_07_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI4_SDI_SELECT_INPUT DAISY Register
pub mod LPSPI4_SDI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_B0_01 for Mode: ALT3
            pub const GPIO_B0_01_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_05 for Mode: ALT1
            pub const GPIO_B1_05_ALT1: u32 = 0b1;
        }
    }
}

/// LPSPI4_SDO_SELECT_INPUT DAISY Register
pub mod LPSPI4_SDO_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_B0_02 for Mode: ALT3
            pub const GPIO_B0_02_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_06 for Mode: ALT1
            pub const GPIO_B1_06_ALT1: u32 = 0b1;
        }
    }
}

/// LPUART2_RX_SELECT_INPUT DAISY Register
pub mod LPUART2_RX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_10 for Mode: ALT2
            pub const GPIO_SD_B1_10_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_03 for Mode: ALT2
            pub const GPIO_AD_B1_03_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART2_TX_SELECT_INPUT DAISY Register
pub mod LPUART2_TX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_11 for Mode: ALT2
            pub const GPIO_SD_B1_11_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_02 for Mode: ALT2
            pub const GPIO_AD_B1_02_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART3_CTS_B_SELECT_INPUT DAISY Register
pub mod LPUART3_CTS_B_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_15 for Mode: ALT2
            pub const GPIO_EMC_15_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_04 for Mode: ALT2
            pub const GPIO_AD_B1_04_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART3_RX_SELECT_INPUT DAISY Register
pub mod LPUART3_RX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT2
            pub const GPIO_AD_B1_07_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_14 for Mode: ALT2
            pub const GPIO_EMC_14_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_09 for Mode: ALT3
            pub const GPIO_B0_09_ALT3: u32 = 0b10;
        }
    }
}

/// LPUART3_TX_SELECT_INPUT DAISY Register
pub mod LPUART3_TX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B1_06 for Mode: ALT2
            pub const GPIO_AD_B1_06_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_13 for Mode: ALT2
            pub const GPIO_EMC_13_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_08 for Mode: ALT3
            pub const GPIO_B0_08_ALT3: u32 = 0b10;
        }
    }
}

/// LPUART4_RX_SELECT_INPUT DAISY Register
pub mod LPUART4_RX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT4
            pub const GPIO_SD_B1_01_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_20 for Mode: ALT2
            pub const GPIO_EMC_20_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_01 for Mode: ALT2
            pub const GPIO_B1_01_ALT2: u32 = 0b10;
        }
    }
}

/// LPUART4_TX_SELECT_INPUT DAISY Register
pub mod LPUART4_TX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT4
            pub const GPIO_SD_B1_00_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_19 for Mode: ALT2
            pub const GPIO_EMC_19_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_00 for Mode: ALT2
            pub const GPIO_B1_00_ALT2: u32 = 0b10;
        }
    }
}

/// LPUART5_RX_SELECT_INPUT DAISY Register
pub mod LPUART5_RX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_24 for Mode: ALT2
            pub const GPIO_EMC_24_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_13 for Mode: ALT1
            pub const GPIO_B1_13_ALT1: u32 = 0b1;
        }
    }
}

/// LPUART5_TX_SELECT_INPUT DAISY Register
pub mod LPUART5_TX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_23 for Mode: ALT2
            pub const GPIO_EMC_23_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_12 for Mode: ALT1
            pub const GPIO_B1_12_ALT1: u32 = 0b1;
        }
    }
}

/// LPUART6_RX_SELECT_INPUT DAISY Register
pub mod LPUART6_RX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_26 for Mode: ALT2
            pub const GPIO_EMC_26_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT2
            pub const GPIO_AD_B0_03_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART6_TX_SELECT_INPUT DAISY Register
pub mod LPUART6_TX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_25 for Mode: ALT2
            pub const GPIO_EMC_25_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_02 for Mode: ALT2
            pub const GPIO_AD_B0_02_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART7_RX_SELECT_INPUT DAISY Register
pub mod LPUART7_RX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT2
            pub const GPIO_SD_B1_09_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_32 for Mode: ALT2
            pub const GPIO_EMC_32_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART7_TX_SELECT_INPUT DAISY Register
pub mod LPUART7_TX_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT2
            pub const GPIO_SD_B1_08_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad:GPIO_EMC_31 for Mode: ALT2
            pub const GPIO_EMC_31_ALT2: u32 = 0b1;
        }
    }
}

/// LPUART8_RX_SELECT_INPUT DAISY Register
pub mod LPUART8_RX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B0_05 for Mode: ALT2
            pub const GPIO_SD_B0_05_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT2
            pub const GPIO_AD_B1_11_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_EMC_39 for Mode: ALT2
            pub const GPIO_EMC_39_ALT2: u32 = 0b10;
        }
    }
}

/// LPUART8_TX_SELECT_INPUT DAISY Register
pub mod LPUART8_TX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B0_04 for Mode: ALT2
            pub const GPIO_SD_B0_04_ALT2: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT2
            pub const GPIO_AD_B1_10_ALT2: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_EMC_38 for Mode: ALT2
            pub const GPIO_EMC_38_ALT2: u32 = 0b10;
        }
    }
}

/// NMI_GLUE_NMI_SELECT_INPUT DAISY Register
pub mod NMI_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_12 for Mode: ALT7
            pub const GPIO_AD_B0_12_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: WAKEUP for Mode: ALT7
            pub const WAKEUP_ALT7: u32 = 0b1;
        }
    }
}

/// QTIMER2_TIMER0_SELECT_INPUT DAISY Register
pub mod QTIMER2_TIMER0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_19 for Mode: ALT4
            pub const GPIO_EMC_19_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_03 for Mode: ALT1
            pub const GPIO_B0_03_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER2_TIMER1_SELECT_INPUT DAISY Register
pub mod QTIMER2_TIMER1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_20 for Mode: ALT4
            pub const GPIO_EMC_20_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_04 for Mode: ALT1
            pub const GPIO_B0_04_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER2_TIMER2_SELECT_INPUT DAISY Register
pub mod QTIMER2_TIMER2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_21 for Mode: ALT4
            pub const GPIO_EMC_21_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_05 for Mode: ALT1
            pub const GPIO_B0_05_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER2_TIMER3_SELECT_INPUT DAISY Register
pub mod QTIMER2_TIMER3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_22 for Mode: ALT4
            pub const GPIO_EMC_22_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_09 for Mode: ALT1
            pub const GPIO_B1_09_ALT1: u32 = 0b1;
        }
    }
}

/// QTIMER3_TIMER0_SELECT_INPUT DAISY Register
pub mod QTIMER3_TIMER0_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_15 for Mode: ALT4
            pub const GPIO_EMC_15_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_00 for Mode: ALT1
            pub const GPIO_AD_B1_00_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_06 for Mode: ALT1
            pub const GPIO_B0_06_ALT1: u32 = 0b10;
        }
    }
}

/// QTIMER3_TIMER1_SELECT_INPUT DAISY Register
pub mod QTIMER3_TIMER1_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_AD_B1_01 for Mode: ALT1
            pub const GPIO_AD_B1_01_ALT1: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_16 for Mode: ALT4
            pub const GPIO_EMC_16_ALT4: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_07 for Mode: ALT1
            pub const GPIO_B0_07_ALT1: u32 = 0b10;
        }
    }
}

/// QTIMER3_TIMER2_SELECT_INPUT DAISY Register
pub mod QTIMER3_TIMER2_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_17 for Mode: ALT4
            pub const GPIO_EMC_17_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_02 for Mode: ALT1
            pub const GPIO_AD_B1_02_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_08 for Mode: ALT1
            pub const GPIO_B0_08_ALT1: u32 = 0b10;
        }
    }
}

/// QTIMER3_TIMER3_SELECT_INPUT DAISY Register
pub mod QTIMER3_TIMER3_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_18 for Mode: ALT4
            pub const GPIO_EMC_18_ALT4: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_03 for Mode: ALT1
            pub const GPIO_AD_B1_03_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_10 for Mode: ALT1
            pub const GPIO_B1_10_ALT1: u32 = 0b10;
        }
    }
}

/// SAI1_MCLK2_SELECT_INPUT DAISY Register
pub mod SAI1_MCLK2_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT3
            pub const GPIO_SD_B1_03_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT3
            pub const GPIO_AD_B1_09_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_13 for Mode: ALT3
            pub const GPIO_B0_13_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_RX_BCLK_SELECT_INPUT DAISY Register
pub mod SAI1_RX_BCLK_SELECT_INPUT {

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

            /// 0b01: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT3
            pub const GPIO_AD_B1_11_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_15 for Mode: ALT3
            pub const GPIO_B0_15_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_RX_DATA0_SELECT_INPUT DAISY Register
pub mod SAI1_RX_DATA0_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT3
            pub const GPIO_SD_B1_06_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT3
            pub const GPIO_AD_B1_12_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_00 for Mode: ALT3
            pub const GPIO_B1_00_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_RX_DATA1_SELECT_INPUT DAISY Register
pub mod SAI1_RX_DATA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT3
            pub const GPIO_SD_B1_00_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_10 for Mode: ALT3
            pub const GPIO_B0_10_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_RX_DATA2_SELECT_INPUT DAISY Register
pub mod SAI1_RX_DATA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT3
            pub const GPIO_SD_B1_01_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_11 for Mode: ALT3
            pub const GPIO_B0_11_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_RX_DATA3_SELECT_INPUT DAISY Register
pub mod SAI1_RX_DATA3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT3
            pub const GPIO_SD_B1_02_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_12 for Mode: ALT3
            pub const GPIO_B0_12_ALT3: u32 = 0b1;
        }
    }
}

/// SAI1_RX_SYNC_SELECT_INPUT DAISY Register
pub mod SAI1_RX_SYNC_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT3
            pub const GPIO_SD_B1_04_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT3
            pub const GPIO_AD_B1_10_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_14 for Mode: ALT3
            pub const GPIO_B0_14_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_TX_BCLK_SELECT_INPUT DAISY Register
pub mod SAI1_TX_BCLK_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT3
            pub const GPIO_SD_B1_08_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT3
            pub const GPIO_AD_B1_14_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_02 for Mode: ALT3
            pub const GPIO_B1_02_ALT3: u32 = 0b10;
        }
    }
}

/// SAI1_TX_SYNC_SELECT_INPUT DAISY Register
pub mod SAI1_TX_SYNC_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT3
            pub const GPIO_SD_B1_09_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_15 for Mode: ALT3
            pub const GPIO_AD_B1_15_ALT3: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_03 for Mode: ALT3
            pub const GPIO_B1_03_ALT3: u32 = 0b10;
        }
    }
}

/// SAI2_MCLK2_SELECT_INPUT DAISY Register
pub mod SAI2_MCLK2_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_10 for Mode: ALT3
            pub const GPIO_AD_B0_10_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_RX_BCLK_SELECT_INPUT DAISY Register
pub mod SAI2_RX_BCLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_10 for Mode: ALT2
            pub const GPIO_EMC_10_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_06 for Mode: ALT3
            pub const GPIO_AD_B0_06_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_RX_DATA0_SELECT_INPUT DAISY Register
pub mod SAI2_RX_DATA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_08 for Mode: ALT2
            pub const GPIO_EMC_08_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_08 for Mode: ALT3
            pub const GPIO_AD_B0_08_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_RX_SYNC_SELECT_INPUT DAISY Register
pub mod SAI2_RX_SYNC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_09 for Mode: ALT2
            pub const GPIO_EMC_09_ALT2: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_07 for Mode: ALT3
            pub const GPIO_AD_B0_07_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_TX_BCLK_SELECT_INPUT DAISY Register
pub mod SAI2_TX_BCLK_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT3
            pub const GPIO_AD_B0_05_ALT3: u32 = 0b1;
        }
    }
}

/// SAI2_TX_SYNC_SELECT_INPUT DAISY Register
pub mod SAI2_TX_SYNC_SELECT_INPUT {

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

            /// 0b1: Selecting Pad: GPIO_AD_B0_04 for Mode: ALT3
            pub const GPIO_AD_B0_04_ALT3: u32 = 0b1;
        }
    }
}

/// SPDIF_IN_SELECT_INPUT DAISY Register
pub mod SPDIF_IN_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_03 for Mode: ALT3
            pub const GPIO_AD_B1_03_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_16 for Mode: ALT3
            pub const GPIO_EMC_16_ALT3: u32 = 0b1;
        }
    }
}

/// USB_OTG2_OC_SELECT_INPUT DAISY Register
pub mod USB_OTG2_OC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_14 for Mode: ALT0
            pub const GPIO_AD_B0_14_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_40 for Mode: ALT3
            pub const GPIO_EMC_40_ALT3: u32 = 0b1;
        }
    }
}

/// USB_OTG1_OC_SELECT_INPUT DAISY Register
pub mod USB_OTG1_OC_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT3
            pub const GPIO_AD_B0_03_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_03 for Mode: ALT0
            pub const GPIO_AD_B1_03_ALT0: u32 = 0b1;
        }
    }
}

/// USDHC1_CD_B_SELECT_INPUT DAISY Register
pub mod USDHC1_CD_B_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_35 for Mode: ALT6
            pub const GPIO_EMC_35_ALT6: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B1_02 for Mode: ALT6
            pub const GPIO_AD_B1_02_ALT6: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_12 for Mode: ALT6
            pub const GPIO_B1_12_ALT6: u32 = 0b10;
        }
    }
}

/// USDHC1_WP_SELECT_INPUT DAISY Register
pub mod USDHC1_WP_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_12 for Mode: ALT3
            pub const GPIO_EMC_12_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_36for Mode: ALT6
            pub const GPIO_EMC_36_ALT6: u32 = 0b01;

            /// 0b10: Selecting Pad:GPIO_AD_B1_00 for Mode: ALT6
            pub const GPIO_AD_B1_00_ALT6: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B1_13 for Mode: ALT6
            pub const GPIO_B1_13_ALT6: u32 = 0b11;
        }
    }
}

/// USDHC2_CLK_SELECT_INPUT DAISY Register
pub mod USDHC2_CLK_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT0
            pub const GPIO_SD_B1_04_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_09 for Mode: ALT6
            pub const GPIO_AD_B1_09_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_CD_B_SELECT_INPUT DAISY Register
pub mod USDHC2_CD_B_SELECT_INPUT {

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

            /// 0b0: Selecting Pad:GPIO_AD_B1_03 for Mode: ALT6
            pub const GPIO_AD_B1_03_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_39 for Mode: ALT6
            pub const GPIO_EMC_39_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_CMD_SELECT_INPUT DAISY Register
pub mod USDHC2_CMD_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT0
            pub const GPIO_SD_B1_05_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_08 for Mode: ALT6
            pub const GPIO_AD_B1_08_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA0_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA0_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT0
            pub const GPIO_SD_B1_03_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad:GPIO_AD_B1_04 for Mode: ALT6
            pub const GPIO_AD_B1_04_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA1_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT0
            pub const GPIO_SD_B1_02_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_05 for Mode: ALT6
            pub const GPIO_AD_B1_05_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA2_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT0
            pub const GPIO_SD_B1_01_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_06 for Mode: ALT6
            pub const GPIO_AD_B1_06_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA3_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA3_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT0
            pub const GPIO_SD_B1_00_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT6
            pub const GPIO_AD_B1_07_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA4_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA4_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_08 for Mode: ALT0
            pub const GPIO_SD_B1_08_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT6
            pub const GPIO_AD_B1_12_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA5_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA5_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_09 for Mode: ALT0
            pub const GPIO_SD_B1_09_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_13 for Mode: ALT6
            pub const GPIO_AD_B1_13_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA6_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA6_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_10 for Mode: ALT0
            pub const GPIO_SD_B1_10_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_14 for Mode: ALT6
            pub const GPIO_AD_B1_14_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_DATA7_SELECT_INPUT DAISY Register
pub mod USDHC2_DATA7_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_SD_B1_11 for Mode: ALT0
            pub const GPIO_SD_B1_11_ALT0: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_15 for Mode: ALT6
            pub const GPIO_AD_B1_15_ALT6: u32 = 0b1;
        }
    }
}

/// USDHC2_WP_SELECT_INPUT DAISY Register
pub mod USDHC2_WP_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_37 for Mode: ALT6
            pub const GPIO_EMC_37_ALT6: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_10 for Mode: ALT6
            pub const GPIO_AD_B1_10_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN02_SELECT_INPUT DAISY Register
pub mod XBAR1_IN02_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_00 for Mode: ALT3
            pub const GPIO_EMC_00_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_14 for Mode: ALT3
            pub const GPIO_B1_14_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN03_SELECT_INPUT DAISY Register
pub mod XBAR1_IN03_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_01 for Mode: ALT3
            pub const GPIO_EMC_01_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_15 for Mode: ALT3
            pub const GPIO_B1_15_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN04_SELECT_INPUT DAISY Register
pub mod XBAR1_IN04_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_02 for Mode: ALT3
            pub const GPIO_EMC_02_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT3
            pub const GPIO_SD_B0_00_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN05_SELECT_INPUT DAISY Register
pub mod XBAR1_IN05_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_03 for Mode: ALT3
            pub const GPIO_EMC_03_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT3
            pub const GPIO_SD_B0_01_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN06_SELECT_INPUT DAISY Register
pub mod XBAR1_IN06_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_04 for Mode: ALT3
            pub const GPIO_EMC_04_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_02 for Mode: ALT3
            pub const GPIO_SD_B0_02_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN07_SELECT_INPUT DAISY Register
pub mod XBAR1_IN07_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_05 for Mode: ALT3
            pub const GPIO_EMC_05_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_03 for Mode: ALT3
            pub const GPIO_SD_B0_03_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN08_SELECT_INPUT DAISY Register
pub mod XBAR1_IN08_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_06 for Mode: ALT3
            pub const GPIO_EMC_06_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_04 for Mode: ALT3
            pub const GPIO_SD_B0_04_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN09_SELECT_INPUT DAISY Register
pub mod XBAR1_IN09_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_07 for Mode: ALT3
            pub const GPIO_EMC_07_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B0_05 for Mode: ALT3
            pub const GPIO_SD_B0_05_ALT3: u32 = 0b1;
        }
    }
}

/// XBAR1_IN17_SELECT_INPUT DAISY Register
pub mod XBAR1_IN17_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_08 for Mode: ALT3
            pub const GPIO_EMC_08_ALT3: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1
            pub const GPIO_AD_B0_03_ALT1: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6
            pub const GPIO_AD_B0_05_ALT6: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B1_03 for Mode: ALT1
            pub const GPIO_B1_03_ALT1: u32 = 0b11;
        }
    }
}

/// XBAR1_IN18_SELECT_INPUT DAISY Register
pub mod XBAR1_IN18_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_35 for Mode: ALT1
            pub const GPIO_EMC_35_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_06 for Mode: ALT6
            pub const GPIO_AD_B0_06_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN20_SELECT_INPUT DAISY Register
pub mod XBAR1_IN20_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_15 for Mode: ALT1
            pub const GPIO_EMC_15_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_08 for Mode: ALT6
            pub const GPIO_AD_B0_08_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN22_SELECT_INPUT DAISY Register
pub mod XBAR1_IN22_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_36 for Mode: ALT1
            pub const GPIO_EMC_36_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_10 for Mode: ALT6
            pub const GPIO_AD_B0_10_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN23_SELECT_INPUT DAISY Register
pub mod XBAR1_IN23_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_37 for Mode: ALT1
            pub const GPIO_EMC_37_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT6
            pub const GPIO_AD_B0_11_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN24_SELECT_INPUT DAISY Register
pub mod XBAR1_IN24_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_12 for Mode: ALT1
            pub const GPIO_EMC_12_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_14 for Mode: ALT1
            pub const GPIO_AD_B0_14_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_IN14_SELECT_INPUT DAISY Register
pub mod XBAR1_IN14_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_00 for Mode: ALT1
            pub const GPIO_AD_B0_00_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad:GPIO_B1_00 for Mode: ALT1
            pub const GPIO_B1_00_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_IN15_SELECT_INPUT DAISY Register
pub mod XBAR1_IN15_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_01 for Mode: ALT1
            pub const GPIO_AD_B0_01_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_01 for Mode: ALT1
            pub const GPIO_B1_01_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_IN16_SELECT_INPUT DAISY Register
pub mod XBAR1_IN16_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_02 for Mode: ALT1
            pub const GPIO_AD_B0_02_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_02 for Mode: ALT1
            pub const GPIO_B1_02_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_IN25_SELECT_INPUT DAISY Register
pub mod XBAR1_IN25_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT1
            pub const GPIO_AD_B0_15_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_EMC_13 for Mode: ALT1
            pub const GPIO_EMC_13_ALT1: u32 = 0b1;
        }
    }
}

/// XBAR1_IN19_SELECT_INPUT DAISY Register
pub mod XBAR1_IN19_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_14 for Mode: ALT1
            pub const GPIO_EMC_14_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_07 for Mode: ALT6
            pub const GPIO_AD_B0_07_ALT6: u32 = 0b1;
        }
    }
}

/// XBAR1_IN23_SELECT_INPUT DAISY Register
pub mod XBAR1_IN21_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_16 for Mode: ALT1
            pub const GPIO_EMC_16_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT6
            pub const GPIO_AD_B0_09_ALT6: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_00 {

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
            pub const DSE_1_R0_150_Ohm___3_3V__260_Ohm_1_8V_: u32 = 0b001;

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

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_08 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_09 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_10 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_11 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_12 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B0_13 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_00 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_01 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_02 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_03 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_04 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_05 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_06 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register
pub mod SW_PAD_CTL_PAD_GPIO_SPI_B1_07 {
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::DSE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::HYS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::ODE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PKE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUE;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::PUS;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SPEED;
    pub use super::SW_PAD_CTL_PAD_GPIO_SPI_B0_00::SRE;
}

/// ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register
pub mod ENET2_IPG_CLK_RMII_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_33 for Mode: ALT9
            pub const GPIO_EMC_33_ALT9: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT9
            pub const GPIO_SD_B0_01_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_15 for Mode: ALT9
            pub const GPIO_B0_15_ALT9: u32 = 0b10;
        }
    }
}

/// ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register
pub mod ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_39 for Mode: ALT8
            pub const GPIO_EMC_39_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_01 for Mode: ALT8
            pub const GPIO_B0_01_ALT8: u32 = 0b1;
        }
    }
}

/// ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register
pub mod ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 {

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

            /// 0b00: Selecting Pad: GPIO_EMC_35 for Mode: ALT8
            pub const GPIO_EMC_35_ALT8: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_03 for Mode: ALT8
            pub const GPIO_SD_B0_03_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_01 for Mode: ALT8
            pub const GPIO_B1_01_ALT8: u32 = 0b10;
        }
    }
}

/// ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register
pub mod ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 {

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

            /// 0b00: Selecting Pad: GPIO_EMC_36 for Mode: ALT8
            pub const GPIO_EMC_36_ALT8: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_04 for Mode: ALT8
            pub const GPIO_SD_B0_04_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_02 for Mode: ALT8
            pub const GPIO_B1_02_ALT8: u32 = 0b10;
        }
    }
}

/// ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register
pub mod ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_37 for Mode: ALT8
            pub const GPIO_EMC_37_ALT8: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_05 for Mode: ALT8
            pub const GPIO_SD_B0_05_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_03 for Mode: ALT8
            pub const GPIO_B1_03_ALT8: u32 = 0b10;
        }
    }
}

/// ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register
pub mod ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_34 for Mode: ALT8
            pub const GPIO_EMC_34_ALT8: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_02 for Mode: ALT8
            pub const GPIO_SD_B0_02_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B1_00 for Mode: ALT8
            pub const GPIO_B1_00_ALT8: u32 = 0b10;
        }
    }
}

/// ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register
pub mod ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 {

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

            /// 0b0: Selecting Pad: GPIO_AD_B1_01 for Mode: ALT8
            pub const GPIO_AD_B1_01_ALT8: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B0_03 for Mode: ALT8
            pub const GPIO_B0_03_ALT8: u32 = 0b1;
        }
    }
}

/// ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register
pub mod ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_33 for Mode: ALT8
            pub const GPIO_EMC_33_ALT8: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_SD_B0_01 for Mode: ALT8
            pub const GPIO_SD_B0_01_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_B0_15 for Mode: ALT8
            pub const GPIO_B0_15_ALT8: u32 = 0b10;
        }
    }
}

/// GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register
pub mod GPT1_IPP_IND_CAPIN1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_24 for Mode: ALT4
            pub const GPIO_EMC_24_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_05 for Mode: ALT8
            pub const GPIO_B1_05_ALT8: u32 = 0b1;
        }
    }
}

/// GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register
pub mod GPT1_IPP_IND_CAPIN2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_23 for Mode: ALT4
            pub const GPIO_EMC_23_ALT4: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_06 for Mode: ALT8
            pub const GPIO_B1_06_ALT8: u32 = 0b1;
        }
    }
}

/// GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register
pub mod GPT1_IPP_IND_CLKIN_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_13 for Mode: ALT1
            pub const GPIO_AD_B0_13_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_B1_04 for Mode: ALT8
            pub const GPIO_B1_04_ALT8: u32 = 0b1;
        }
    }
}

/// GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register
pub mod GPT2_IPP_IND_CAPIN1_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_41 for Mode: ALT1
            pub const GPIO_EMC_41_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_03 for Mode: ALT8
            pub const GPIO_AD_B1_03_ALT8: u32 = 0b1;
        }
    }
}

/// GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register
pub mod GPT2_IPP_IND_CAPIN2_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_EMC_40 for Mode: ALT1
            pub const GPIO_EMC_40_ALT1: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_04 for Mode: ALT8
            pub const GPIO_AD_B1_04_ALT8: u32 = 0b1;
        }
    }
}

/// GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register
pub mod GPT2_IPP_IND_CLKIN_SELECT_INPUT {

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

            /// 0b0: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT7
            pub const GPIO_AD_B0_09_ALT7: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_AD_B1_02 for Mode: ALT8
            pub const GPIO_AD_B1_02_ALT8: u32 = 0b1;
        }
    }
}

/// SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register
pub mod SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {

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

            /// 0b0: Selecting Pad: GPIO_EMC_37 for Mode: ALT3
            pub const GPIO_EMC_37_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_04 for Mode: ALT8
            pub const GPIO_SD_B1_04_ALT8: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_35 for Mode: ALT3
            pub const GPIO_EMC_35_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT8
            pub const GPIO_SD_B1_06_ALT8: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_33 for Mode: ALT3
            pub const GPIO_EMC_33_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_00 for Mode: ALT8
            pub const GPIO_SD_B1_00_ALT8: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_34 for Mode: ALT3
            pub const GPIO_EMC_34_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_05 for Mode: ALT8
            pub const GPIO_SD_B1_05_ALT8: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_38 for Mode: ALT3
            pub const GPIO_EMC_38_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_03 for Mode: ALT8
            pub const GPIO_SD_B1_03_ALT8: u32 = 0b1;
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

            /// 0b0: Selecting Pad: GPIO_EMC_39 for Mode: ALT3
            pub const GPIO_EMC_39_ALT3: u32 = 0b0;

            /// 0b1: Selecting Pad: GPIO_SD_B1_02 for Mode: ALT8
            pub const GPIO_SD_B1_02_ALT8: u32 = 0b1;
        }
    }
}

/// SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register
pub mod SEMC_I_IPP_IND_DQS4_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT9
            pub const GPIO_SD_B0_00_ALT9: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_EMC_39 for Mode: ALT9
            pub const GPIO_EMC_39_ALT9: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT9
            pub const GPIO_AD_B0_09_ALT9: u32 = 0b10;

            /// 0b11: Selecting Pad: GPIO_B1_13 for Mode: ALT8
            pub const GPIO_B1_13_ALT8: u32 = 0b11;
        }
    }
}

/// CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register
pub mod CANFD_IPP_IND_CANRX_SELECT_INPUT {

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

            /// 0b00: Selecting Pad: GPIO_EMC_37 for Mode: ALT9
            pub const GPIO_EMC_37_ALT9: u32 = 0b00;

            /// 0b01: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT8
            pub const GPIO_AD_B0_15_ALT8: u32 = 0b01;

            /// 0b10: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT8
            pub const GPIO_AD_B0_11_ALT8: u32 = 0b10;
        }
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 5],

    /// SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_03: RWRegister<u32>,

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

    /// SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_15: RWRegister<u32>,

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

    /// SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_28: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_29: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_30: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_31: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_32: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_33: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_34: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_35: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_36: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_37: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_38: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_39: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_40: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_EMC_41: RWRegister<u32>,

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

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_09: RWRegister<u32>,

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

    /// SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B0_15: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_05: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_06: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_07: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_08: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_09: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_10: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_11: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_12: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_13: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_14: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_B1_15: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_00: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_01: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_02: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_03: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_04: RWRegister<u32>,

    /// SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_05: RWRegister<u32>,

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

    /// SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_03: RWRegister<u32>,

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

    /// SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_15: RWRegister<u32>,

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

    /// SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_28: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_29: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_30: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_31: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_32: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_33: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_34: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_35: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_36: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_37: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_38: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_39: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_40: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_EMC_41: RWRegister<u32>,

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

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_09: RWRegister<u32>,

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

    /// SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B0_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_14: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_B1_15: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_05: RWRegister<u32>,

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

    /// ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register
    pub ANATOP_USB_OTG1_ID_SELECT_INPUT: RWRegister<u32>,

    /// ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register
    pub ANATOP_USB_OTG2_ID_SELECT_INPUT: RWRegister<u32>,

    /// CCM_PMIC_READY_SELECT_INPUT DAISY Register
    pub CCM_PMIC_READY_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA02_SELECT_INPUT DAISY Register
    pub CSI_DATA02_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA03_SELECT_INPUT DAISY Register
    pub CSI_DATA03_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA04_SELECT_INPUT DAISY Register
    pub CSI_DATA04_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA05_SELECT_INPUT DAISY Register
    pub CSI_DATA05_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA06_SELECT_INPUT DAISY Register
    pub CSI_DATA06_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA07_SELECT_INPUT DAISY Register
    pub CSI_DATA07_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA08_SELECT_INPUT DAISY Register
    pub CSI_DATA08_SELECT_INPUT: RWRegister<u32>,

    /// CSI_DATA09_SELECT_INPUT DAISY Register
    pub CSI_DATA09_SELECT_INPUT: RWRegister<u32>,

    /// CSI_HSYNC_SELECT_INPUT DAISY Register
    pub CSI_HSYNC_SELECT_INPUT: RWRegister<u32>,

    /// CSI_PIXCLK_SELECT_INPUT DAISY Register
    pub CSI_PIXCLK_SELECT_INPUT: RWRegister<u32>,

    /// CSI_VSYNC_SELECT_INPUT DAISY Register
    pub CSI_VSYNC_SELECT_INPUT: RWRegister<u32>,

    /// ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register
    pub ENET_IPG_CLK_RMII_SELECT_INPUT: RWRegister<u32>,

    /// ENET_MDIO_SELECT_INPUT DAISY Register
    pub ENET_MDIO_SELECT_INPUT: RWRegister<u32>,

    /// ENET0_RXDATA_SELECT_INPUT DAISY Register
    pub ENET0_RXDATA_SELECT_INPUT: RWRegister<u32>,

    /// ENET1_RXDATA_SELECT_INPUT DAISY Register
    pub ENET1_RXDATA_SELECT_INPUT: RWRegister<u32>,

    /// ENET_RXEN_SELECT_INPUT DAISY Register
    pub ENET_RXEN_SELECT_INPUT: RWRegister<u32>,

    /// ENET_RXERR_SELECT_INPUT DAISY Register
    pub ENET_RXERR_SELECT_INPUT: RWRegister<u32>,

    /// ENET0_TIMER_SELECT_INPUT DAISY Register
    pub ENET0_TIMER_SELECT_INPUT: RWRegister<u32>,

    /// ENET_TXCLK_SELECT_INPUT DAISY Register
    pub ENET_TXCLK_SELECT_INPUT: RWRegister<u32>,

    /// FLEXCAN1_RX_SELECT_INPUT DAISY Register
    pub FLEXCAN1_RX_SELECT_INPUT: RWRegister<u32>,

    /// FLEXCAN2_RX_SELECT_INPUT DAISY Register
    pub FLEXCAN2_RX_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMA3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMA0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMA1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMA2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMB3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMB0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMB1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register
    pub FLEXPWM1_PWMB2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMA3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMA0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMA1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMA2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMB3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMB0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMB1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register
    pub FLEXPWM2_PWMB2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register
    pub FLEXPWM4_PWMA0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register
    pub FLEXPWM4_PWMA1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register
    pub FLEXPWM4_PWMA2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register
    pub FLEXPWM4_PWMA3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_DQS_SELECT_INPUT DAISY Register
    pub FLEXSPIA_DQS_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_DATA0_SELECT_INPUT DAISY Register
    pub FLEXSPIA_DATA0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_DATA1_SELECT_INPUT DAISY Register
    pub FLEXSPIA_DATA1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_DATA2_SELECT_INPUT DAISY Register
    pub FLEXSPIA_DATA2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_DATA3_SELECT_INPUT DAISY Register
    pub FLEXSPIA_DATA3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIB_DATA0_SELECT_INPUT DAISY Register
    pub FLEXSPIB_DATA0_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIB_DATA1_SELECT_INPUT DAISY Register
    pub FLEXSPIB_DATA1_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIB_DATA2_SELECT_INPUT DAISY Register
    pub FLEXSPIB_DATA2_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIB_DATA3_SELECT_INPUT DAISY Register
    pub FLEXSPIB_DATA3_SELECT_INPUT: RWRegister<u32>,

    /// FLEXSPIA_SCK_SELECT_INPUT DAISY Register
    pub FLEXSPIA_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_SCL_SELECT_INPUT DAISY Register
    pub LPI2C1_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C1_SDA_SELECT_INPUT DAISY Register
    pub LPI2C1_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_SCL_SELECT_INPUT DAISY Register
    pub LPI2C2_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C2_SDA_SELECT_INPUT DAISY Register
    pub LPI2C2_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C3_SCL_SELECT_INPUT DAISY Register
    pub LPI2C3_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C3_SDA_SELECT_INPUT DAISY Register
    pub LPI2C3_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C4_SCL_SELECT_INPUT DAISY Register
    pub LPI2C4_SCL_SELECT_INPUT: RWRegister<u32>,

    /// LPI2C4_SDA_SELECT_INPUT DAISY Register
    pub LPI2C4_SDA_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_PCS0_SELECT_INPUT DAISY Register
    pub LPSPI1_PCS0_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_SCK_SELECT_INPUT DAISY Register
    pub LPSPI1_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_SDI_SELECT_INPUT DAISY Register
    pub LPSPI1_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI1_SDO_SELECT_INPUT DAISY Register
    pub LPSPI1_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_PCS0_SELECT_INPUT DAISY Register
    pub LPSPI2_PCS0_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_SCK_SELECT_INPUT DAISY Register
    pub LPSPI2_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_SDI_SELECT_INPUT DAISY Register
    pub LPSPI2_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI2_SDO_SELECT_INPUT DAISY Register
    pub LPSPI2_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_PCS0_SELECT_INPUT DAISY Register
    pub LPSPI3_PCS0_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_SCK_SELECT_INPUT DAISY Register
    pub LPSPI3_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_SDI_SELECT_INPUT DAISY Register
    pub LPSPI3_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI3_SDO_SELECT_INPUT DAISY Register
    pub LPSPI3_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_PCS0_SELECT_INPUT DAISY Register
    pub LPSPI4_PCS0_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_SCK_SELECT_INPUT DAISY Register
    pub LPSPI4_SCK_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_SDI_SELECT_INPUT DAISY Register
    pub LPSPI4_SDI_SELECT_INPUT: RWRegister<u32>,

    /// LPSPI4_SDO_SELECT_INPUT DAISY Register
    pub LPSPI4_SDO_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_RX_SELECT_INPUT DAISY Register
    pub LPUART2_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART2_TX_SELECT_INPUT DAISY Register
    pub LPUART2_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_CTS_B_SELECT_INPUT DAISY Register
    pub LPUART3_CTS_B_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_RX_SELECT_INPUT DAISY Register
    pub LPUART3_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART3_TX_SELECT_INPUT DAISY Register
    pub LPUART3_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_RX_SELECT_INPUT DAISY Register
    pub LPUART4_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART4_TX_SELECT_INPUT DAISY Register
    pub LPUART4_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART5_RX_SELECT_INPUT DAISY Register
    pub LPUART5_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART5_TX_SELECT_INPUT DAISY Register
    pub LPUART5_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART6_RX_SELECT_INPUT DAISY Register
    pub LPUART6_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART6_TX_SELECT_INPUT DAISY Register
    pub LPUART6_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART7_RX_SELECT_INPUT DAISY Register
    pub LPUART7_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART7_TX_SELECT_INPUT DAISY Register
    pub LPUART7_TX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART8_RX_SELECT_INPUT DAISY Register
    pub LPUART8_RX_SELECT_INPUT: RWRegister<u32>,

    /// LPUART8_TX_SELECT_INPUT DAISY Register
    pub LPUART8_TX_SELECT_INPUT: RWRegister<u32>,

    /// NMI_GLUE_NMI_SELECT_INPUT DAISY Register
    pub NMI_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TIMER0_SELECT_INPUT DAISY Register
    pub QTIMER2_TIMER0_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TIMER1_SELECT_INPUT DAISY Register
    pub QTIMER2_TIMER1_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TIMER2_SELECT_INPUT DAISY Register
    pub QTIMER2_TIMER2_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER2_TIMER3_SELECT_INPUT DAISY Register
    pub QTIMER2_TIMER3_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TIMER0_SELECT_INPUT DAISY Register
    pub QTIMER3_TIMER0_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TIMER1_SELECT_INPUT DAISY Register
    pub QTIMER3_TIMER1_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TIMER2_SELECT_INPUT DAISY Register
    pub QTIMER3_TIMER2_SELECT_INPUT: RWRegister<u32>,

    /// QTIMER3_TIMER3_SELECT_INPUT DAISY Register
    pub QTIMER3_TIMER3_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_MCLK2_SELECT_INPUT DAISY Register
    pub SAI1_MCLK2_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_BCLK_SELECT_INPUT DAISY Register
    pub SAI1_RX_BCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_DATA0_SELECT_INPUT DAISY Register
    pub SAI1_RX_DATA0_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_DATA1_SELECT_INPUT DAISY Register
    pub SAI1_RX_DATA1_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_DATA2_SELECT_INPUT DAISY Register
    pub SAI1_RX_DATA2_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_DATA3_SELECT_INPUT DAISY Register
    pub SAI1_RX_DATA3_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_RX_SYNC_SELECT_INPUT DAISY Register
    pub SAI1_RX_SYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_TX_BCLK_SELECT_INPUT DAISY Register
    pub SAI1_TX_BCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI1_TX_SYNC_SELECT_INPUT DAISY Register
    pub SAI1_TX_SYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_MCLK2_SELECT_INPUT DAISY Register
    pub SAI2_MCLK2_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_RX_BCLK_SELECT_INPUT DAISY Register
    pub SAI2_RX_BCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_RX_DATA0_SELECT_INPUT DAISY Register
    pub SAI2_RX_DATA0_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_RX_SYNC_SELECT_INPUT DAISY Register
    pub SAI2_RX_SYNC_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_TX_BCLK_SELECT_INPUT DAISY Register
    pub SAI2_TX_BCLK_SELECT_INPUT: RWRegister<u32>,

    /// SAI2_TX_SYNC_SELECT_INPUT DAISY Register
    pub SAI2_TX_SYNC_SELECT_INPUT: RWRegister<u32>,

    /// SPDIF_IN_SELECT_INPUT DAISY Register
    pub SPDIF_IN_SELECT_INPUT: RWRegister<u32>,

    /// USB_OTG2_OC_SELECT_INPUT DAISY Register
    pub USB_OTG2_OC_SELECT_INPUT: RWRegister<u32>,

    /// USB_OTG1_OC_SELECT_INPUT DAISY Register
    pub USB_OTG1_OC_SELECT_INPUT: RWRegister<u32>,

    /// USDHC1_CD_B_SELECT_INPUT DAISY Register
    pub USDHC1_CD_B_SELECT_INPUT: RWRegister<u32>,

    /// USDHC1_WP_SELECT_INPUT DAISY Register
    pub USDHC1_WP_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_CLK_SELECT_INPUT DAISY Register
    pub USDHC2_CLK_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_CD_B_SELECT_INPUT DAISY Register
    pub USDHC2_CD_B_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_CMD_SELECT_INPUT DAISY Register
    pub USDHC2_CMD_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA0_SELECT_INPUT DAISY Register
    pub USDHC2_DATA0_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA1_SELECT_INPUT DAISY Register
    pub USDHC2_DATA1_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA2_SELECT_INPUT DAISY Register
    pub USDHC2_DATA2_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA3_SELECT_INPUT DAISY Register
    pub USDHC2_DATA3_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA4_SELECT_INPUT DAISY Register
    pub USDHC2_DATA4_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA5_SELECT_INPUT DAISY Register
    pub USDHC2_DATA5_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA6_SELECT_INPUT DAISY Register
    pub USDHC2_DATA6_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_DATA7_SELECT_INPUT DAISY Register
    pub USDHC2_DATA7_SELECT_INPUT: RWRegister<u32>,

    /// USDHC2_WP_SELECT_INPUT DAISY Register
    pub USDHC2_WP_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN02_SELECT_INPUT DAISY Register
    pub XBAR1_IN02_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN03_SELECT_INPUT DAISY Register
    pub XBAR1_IN03_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN04_SELECT_INPUT DAISY Register
    pub XBAR1_IN04_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN05_SELECT_INPUT DAISY Register
    pub XBAR1_IN05_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN06_SELECT_INPUT DAISY Register
    pub XBAR1_IN06_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN07_SELECT_INPUT DAISY Register
    pub XBAR1_IN07_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN08_SELECT_INPUT DAISY Register
    pub XBAR1_IN08_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN09_SELECT_INPUT DAISY Register
    pub XBAR1_IN09_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN17_SELECT_INPUT DAISY Register
    pub XBAR1_IN17_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN18_SELECT_INPUT DAISY Register
    pub XBAR1_IN18_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN20_SELECT_INPUT DAISY Register
    pub XBAR1_IN20_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN22_SELECT_INPUT DAISY Register
    pub XBAR1_IN22_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN23_SELECT_INPUT DAISY Register
    pub XBAR1_IN23_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN24_SELECT_INPUT DAISY Register
    pub XBAR1_IN24_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN14_SELECT_INPUT DAISY Register
    pub XBAR1_IN14_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN15_SELECT_INPUT DAISY Register
    pub XBAR1_IN15_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN16_SELECT_INPUT DAISY Register
    pub XBAR1_IN16_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN25_SELECT_INPUT DAISY Register
    pub XBAR1_IN25_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN19_SELECT_INPUT DAISY Register
    pub XBAR1_IN19_SELECT_INPUT: RWRegister<u32>,

    /// XBAR1_IN23_SELECT_INPUT DAISY Register
    pub XBAR1_IN21_SELECT_INPUT: RWRegister<u32>,

    _reserved2: [u32; 22],

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_07: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_08: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_09: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_10: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_11: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_12: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_13: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_00: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_01: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_02: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_03: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_04: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_05: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_06: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_07: RWRegister<u32>,

    /// ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register
    pub ENET2_IPG_CLK_RMII_SELECT_INPUT: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register
    pub ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register
    pub ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register
    pub ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register
    pub ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register
    pub ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register
    pub ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0: RWRegister<u32>,

    /// ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register
    pub ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT: RWRegister<u32>,

    _reserved3: [u32; 11],

    /// GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register
    pub GPT1_IPP_IND_CAPIN1_SELECT_INPUT: RWRegister<u32>,

    /// GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register
    pub GPT1_IPP_IND_CAPIN2_SELECT_INPUT: RWRegister<u32>,

    /// GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register
    pub GPT1_IPP_IND_CLKIN_SELECT_INPUT: RWRegister<u32>,

    /// GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register
    pub GPT2_IPP_IND_CAPIN1_SELECT_INPUT: RWRegister<u32>,

    /// GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register
    pub GPT2_IPP_IND_CAPIN2_SELECT_INPUT: RWRegister<u32>,

    /// GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register
    pub GPT2_IPP_IND_CLKIN_SELECT_INPUT: RWRegister<u32>,

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

    /// SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register
    pub SEMC_I_IPP_IND_DQS4_SELECT_INPUT: RWRegister<u32>,

    /// CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register
    pub CANFD_IPP_IND_CANRX_SELECT_INPUT: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_GPIO_EMC_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_15: u32,
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
    pub SW_MUX_CTL_PAD_GPIO_EMC_28: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_29: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_30: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_31: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_32: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_33: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_34: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_35: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_36: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_37: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_38: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_39: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_40: u32,
    pub SW_MUX_CTL_PAD_GPIO_EMC_41: u32,
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
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_AD_B1_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_B0_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_05: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_06: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_07: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_08: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_09: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_10: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_11: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_12: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_13: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_14: u32,
    pub SW_MUX_CTL_PAD_GPIO_B1_15: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_00: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_01: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_02: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_03: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_04: u32,
    pub SW_MUX_CTL_PAD_GPIO_SD_B0_05: u32,
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
    pub SW_PAD_CTL_PAD_GPIO_EMC_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_15: u32,
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
    pub SW_PAD_CTL_PAD_GPIO_EMC_28: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_29: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_30: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_31: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_32: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_33: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_34: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_35: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_36: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_37: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_38: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_39: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_40: u32,
    pub SW_PAD_CTL_PAD_GPIO_EMC_41: u32,
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
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_AD_B1_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_B0_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_14: u32,
    pub SW_PAD_CTL_PAD_GPIO_B1_15: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SD_B0_05: u32,
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
    pub ANATOP_USB_OTG1_ID_SELECT_INPUT: u32,
    pub ANATOP_USB_OTG2_ID_SELECT_INPUT: u32,
    pub CCM_PMIC_READY_SELECT_INPUT: u32,
    pub CSI_DATA02_SELECT_INPUT: u32,
    pub CSI_DATA03_SELECT_INPUT: u32,
    pub CSI_DATA04_SELECT_INPUT: u32,
    pub CSI_DATA05_SELECT_INPUT: u32,
    pub CSI_DATA06_SELECT_INPUT: u32,
    pub CSI_DATA07_SELECT_INPUT: u32,
    pub CSI_DATA08_SELECT_INPUT: u32,
    pub CSI_DATA09_SELECT_INPUT: u32,
    pub CSI_HSYNC_SELECT_INPUT: u32,
    pub CSI_PIXCLK_SELECT_INPUT: u32,
    pub CSI_VSYNC_SELECT_INPUT: u32,
    pub ENET_IPG_CLK_RMII_SELECT_INPUT: u32,
    pub ENET_MDIO_SELECT_INPUT: u32,
    pub ENET0_RXDATA_SELECT_INPUT: u32,
    pub ENET1_RXDATA_SELECT_INPUT: u32,
    pub ENET_RXEN_SELECT_INPUT: u32,
    pub ENET_RXERR_SELECT_INPUT: u32,
    pub ENET0_TIMER_SELECT_INPUT: u32,
    pub ENET_TXCLK_SELECT_INPUT: u32,
    pub FLEXCAN1_RX_SELECT_INPUT: u32,
    pub FLEXCAN2_RX_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA3_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA0_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA1_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMA2_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMB3_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMB0_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMB1_SELECT_INPUT: u32,
    pub FLEXPWM1_PWMB2_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMA3_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMA0_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMA1_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMA2_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMB3_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMB0_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMB1_SELECT_INPUT: u32,
    pub FLEXPWM2_PWMB2_SELECT_INPUT: u32,
    pub FLEXPWM4_PWMA0_SELECT_INPUT: u32,
    pub FLEXPWM4_PWMA1_SELECT_INPUT: u32,
    pub FLEXPWM4_PWMA2_SELECT_INPUT: u32,
    pub FLEXPWM4_PWMA3_SELECT_INPUT: u32,
    pub FLEXSPIA_DQS_SELECT_INPUT: u32,
    pub FLEXSPIA_DATA0_SELECT_INPUT: u32,
    pub FLEXSPIA_DATA1_SELECT_INPUT: u32,
    pub FLEXSPIA_DATA2_SELECT_INPUT: u32,
    pub FLEXSPIA_DATA3_SELECT_INPUT: u32,
    pub FLEXSPIB_DATA0_SELECT_INPUT: u32,
    pub FLEXSPIB_DATA1_SELECT_INPUT: u32,
    pub FLEXSPIB_DATA2_SELECT_INPUT: u32,
    pub FLEXSPIB_DATA3_SELECT_INPUT: u32,
    pub FLEXSPIA_SCK_SELECT_INPUT: u32,
    pub LPI2C1_SCL_SELECT_INPUT: u32,
    pub LPI2C1_SDA_SELECT_INPUT: u32,
    pub LPI2C2_SCL_SELECT_INPUT: u32,
    pub LPI2C2_SDA_SELECT_INPUT: u32,
    pub LPI2C3_SCL_SELECT_INPUT: u32,
    pub LPI2C3_SDA_SELECT_INPUT: u32,
    pub LPI2C4_SCL_SELECT_INPUT: u32,
    pub LPI2C4_SDA_SELECT_INPUT: u32,
    pub LPSPI1_PCS0_SELECT_INPUT: u32,
    pub LPSPI1_SCK_SELECT_INPUT: u32,
    pub LPSPI1_SDI_SELECT_INPUT: u32,
    pub LPSPI1_SDO_SELECT_INPUT: u32,
    pub LPSPI2_PCS0_SELECT_INPUT: u32,
    pub LPSPI2_SCK_SELECT_INPUT: u32,
    pub LPSPI2_SDI_SELECT_INPUT: u32,
    pub LPSPI2_SDO_SELECT_INPUT: u32,
    pub LPSPI3_PCS0_SELECT_INPUT: u32,
    pub LPSPI3_SCK_SELECT_INPUT: u32,
    pub LPSPI3_SDI_SELECT_INPUT: u32,
    pub LPSPI3_SDO_SELECT_INPUT: u32,
    pub LPSPI4_PCS0_SELECT_INPUT: u32,
    pub LPSPI4_SCK_SELECT_INPUT: u32,
    pub LPSPI4_SDI_SELECT_INPUT: u32,
    pub LPSPI4_SDO_SELECT_INPUT: u32,
    pub LPUART2_RX_SELECT_INPUT: u32,
    pub LPUART2_TX_SELECT_INPUT: u32,
    pub LPUART3_CTS_B_SELECT_INPUT: u32,
    pub LPUART3_RX_SELECT_INPUT: u32,
    pub LPUART3_TX_SELECT_INPUT: u32,
    pub LPUART4_RX_SELECT_INPUT: u32,
    pub LPUART4_TX_SELECT_INPUT: u32,
    pub LPUART5_RX_SELECT_INPUT: u32,
    pub LPUART5_TX_SELECT_INPUT: u32,
    pub LPUART6_RX_SELECT_INPUT: u32,
    pub LPUART6_TX_SELECT_INPUT: u32,
    pub LPUART7_RX_SELECT_INPUT: u32,
    pub LPUART7_TX_SELECT_INPUT: u32,
    pub LPUART8_RX_SELECT_INPUT: u32,
    pub LPUART8_TX_SELECT_INPUT: u32,
    pub NMI_SELECT_INPUT: u32,
    pub QTIMER2_TIMER0_SELECT_INPUT: u32,
    pub QTIMER2_TIMER1_SELECT_INPUT: u32,
    pub QTIMER2_TIMER2_SELECT_INPUT: u32,
    pub QTIMER2_TIMER3_SELECT_INPUT: u32,
    pub QTIMER3_TIMER0_SELECT_INPUT: u32,
    pub QTIMER3_TIMER1_SELECT_INPUT: u32,
    pub QTIMER3_TIMER2_SELECT_INPUT: u32,
    pub QTIMER3_TIMER3_SELECT_INPUT: u32,
    pub SAI1_MCLK2_SELECT_INPUT: u32,
    pub SAI1_RX_BCLK_SELECT_INPUT: u32,
    pub SAI1_RX_DATA0_SELECT_INPUT: u32,
    pub SAI1_RX_DATA1_SELECT_INPUT: u32,
    pub SAI1_RX_DATA2_SELECT_INPUT: u32,
    pub SAI1_RX_DATA3_SELECT_INPUT: u32,
    pub SAI1_RX_SYNC_SELECT_INPUT: u32,
    pub SAI1_TX_BCLK_SELECT_INPUT: u32,
    pub SAI1_TX_SYNC_SELECT_INPUT: u32,
    pub SAI2_MCLK2_SELECT_INPUT: u32,
    pub SAI2_RX_BCLK_SELECT_INPUT: u32,
    pub SAI2_RX_DATA0_SELECT_INPUT: u32,
    pub SAI2_RX_SYNC_SELECT_INPUT: u32,
    pub SAI2_TX_BCLK_SELECT_INPUT: u32,
    pub SAI2_TX_SYNC_SELECT_INPUT: u32,
    pub SPDIF_IN_SELECT_INPUT: u32,
    pub USB_OTG2_OC_SELECT_INPUT: u32,
    pub USB_OTG1_OC_SELECT_INPUT: u32,
    pub USDHC1_CD_B_SELECT_INPUT: u32,
    pub USDHC1_WP_SELECT_INPUT: u32,
    pub USDHC2_CLK_SELECT_INPUT: u32,
    pub USDHC2_CD_B_SELECT_INPUT: u32,
    pub USDHC2_CMD_SELECT_INPUT: u32,
    pub USDHC2_DATA0_SELECT_INPUT: u32,
    pub USDHC2_DATA1_SELECT_INPUT: u32,
    pub USDHC2_DATA2_SELECT_INPUT: u32,
    pub USDHC2_DATA3_SELECT_INPUT: u32,
    pub USDHC2_DATA4_SELECT_INPUT: u32,
    pub USDHC2_DATA5_SELECT_INPUT: u32,
    pub USDHC2_DATA6_SELECT_INPUT: u32,
    pub USDHC2_DATA7_SELECT_INPUT: u32,
    pub USDHC2_WP_SELECT_INPUT: u32,
    pub XBAR1_IN02_SELECT_INPUT: u32,
    pub XBAR1_IN03_SELECT_INPUT: u32,
    pub XBAR1_IN04_SELECT_INPUT: u32,
    pub XBAR1_IN05_SELECT_INPUT: u32,
    pub XBAR1_IN06_SELECT_INPUT: u32,
    pub XBAR1_IN07_SELECT_INPUT: u32,
    pub XBAR1_IN08_SELECT_INPUT: u32,
    pub XBAR1_IN09_SELECT_INPUT: u32,
    pub XBAR1_IN17_SELECT_INPUT: u32,
    pub XBAR1_IN18_SELECT_INPUT: u32,
    pub XBAR1_IN20_SELECT_INPUT: u32,
    pub XBAR1_IN22_SELECT_INPUT: u32,
    pub XBAR1_IN23_SELECT_INPUT: u32,
    pub XBAR1_IN24_SELECT_INPUT: u32,
    pub XBAR1_IN14_SELECT_INPUT: u32,
    pub XBAR1_IN15_SELECT_INPUT: u32,
    pub XBAR1_IN16_SELECT_INPUT: u32,
    pub XBAR1_IN25_SELECT_INPUT: u32,
    pub XBAR1_IN19_SELECT_INPUT: u32,
    pub XBAR1_IN21_SELECT_INPUT: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_07: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_08: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_09: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_10: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_11: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_12: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B0_13: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_00: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_01: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_02: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_03: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_04: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_05: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_06: u32,
    pub SW_PAD_CTL_PAD_GPIO_SPI_B1_07: u32,
    pub ENET2_IPG_CLK_RMII_SELECT_INPUT: u32,
    pub ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT: u32,
    pub ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0: u32,
    pub ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1: u32,
    pub ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT: u32,
    pub ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT: u32,
    pub ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0: u32,
    pub ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT: u32,
    pub GPT1_IPP_IND_CAPIN1_SELECT_INPUT: u32,
    pub GPT1_IPP_IND_CAPIN2_SELECT_INPUT: u32,
    pub GPT1_IPP_IND_CLKIN_SELECT_INPUT: u32,
    pub GPT2_IPP_IND_CAPIN1_SELECT_INPUT: u32,
    pub GPT2_IPP_IND_CAPIN2_SELECT_INPUT: u32,
    pub GPT2_IPP_IND_CLKIN_SELECT_INPUT: u32,
    pub SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: u32,
    pub SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: u32,
    pub SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: u32,
    pub SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: u32,
    pub SEMC_I_IPP_IND_DQS4_SELECT_INPUT: u32,
    pub CANFD_IPP_IND_CANRX_SELECT_INPUT: u32,
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
        SW_MUX_CTL_PAD_GPIO_EMC_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_16: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_17: 0x00000005,
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
        SW_MUX_CTL_PAD_GPIO_EMC_28: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_29: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_30: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_31: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_32: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_33: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_34: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_35: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_36: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_37: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_38: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_39: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_40: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_EMC_41: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_04: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_05: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_06: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_07: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_08: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_09: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_10: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_11: 0x00000000,
        SW_MUX_CTL_PAD_GPIO_AD_B0_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B0_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_AD_B1_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B0_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_05: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_06: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_07: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_08: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_09: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_10: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_11: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_12: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_13: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_14: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_B1_15: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_00: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_01: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_02: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_03: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_04: 0x00000005,
        SW_MUX_CTL_PAD_GPIO_SD_B0_05: 0x00000005,
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
        SW_PAD_CTL_PAD_GPIO_EMC_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_16: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_17: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_18: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_19: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_20: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_21: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_22: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_23: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_24: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_25: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_26: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_27: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_EMC_28: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_29: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_30: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_31: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_32: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_33: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_34: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_35: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_36: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_37: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_38: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_39: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_40: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_EMC_41: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_04: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_05: 0x000030B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_06: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_07: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_08: 0x0000B0A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_09: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_10: 0x000090B1,
        SW_PAD_CTL_PAD_GPIO_AD_B0_11: 0x000070A0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B0_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_AD_B1_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B0_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_14: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_B1_15: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SD_B0_05: 0x000010B0,
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
        ANATOP_USB_OTG1_ID_SELECT_INPUT: 0x00000000,
        ANATOP_USB_OTG2_ID_SELECT_INPUT: 0x00000000,
        CCM_PMIC_READY_SELECT_INPUT: 0x00000000,
        CSI_DATA02_SELECT_INPUT: 0x00000000,
        CSI_DATA03_SELECT_INPUT: 0x00000000,
        CSI_DATA04_SELECT_INPUT: 0x00000000,
        CSI_DATA05_SELECT_INPUT: 0x00000000,
        CSI_DATA06_SELECT_INPUT: 0x00000000,
        CSI_DATA07_SELECT_INPUT: 0x00000000,
        CSI_DATA08_SELECT_INPUT: 0x00000000,
        CSI_DATA09_SELECT_INPUT: 0x00000000,
        CSI_HSYNC_SELECT_INPUT: 0x00000000,
        CSI_PIXCLK_SELECT_INPUT: 0x00000000,
        CSI_VSYNC_SELECT_INPUT: 0x00000000,
        ENET_IPG_CLK_RMII_SELECT_INPUT: 0x00000000,
        ENET_MDIO_SELECT_INPUT: 0x00000000,
        ENET0_RXDATA_SELECT_INPUT: 0x00000000,
        ENET1_RXDATA_SELECT_INPUT: 0x00000000,
        ENET_RXEN_SELECT_INPUT: 0x00000000,
        ENET_RXERR_SELECT_INPUT: 0x00000000,
        ENET0_TIMER_SELECT_INPUT: 0x00000000,
        ENET_TXCLK_SELECT_INPUT: 0x00000000,
        FLEXCAN1_RX_SELECT_INPUT: 0x00000000,
        FLEXCAN2_RX_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB3_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB0_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB1_SELECT_INPUT: 0x00000000,
        FLEXPWM1_PWMB2_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB3_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB0_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB1_SELECT_INPUT: 0x00000000,
        FLEXPWM2_PWMB2_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA0_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA1_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA2_SELECT_INPUT: 0x00000000,
        FLEXPWM4_PWMA3_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DQS_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA0_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA1_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA2_SELECT_INPUT: 0x00000000,
        FLEXSPIA_DATA3_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA0_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA1_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA2_SELECT_INPUT: 0x00000000,
        FLEXSPIB_DATA3_SELECT_INPUT: 0x00000000,
        FLEXSPIA_SCK_SELECT_INPUT: 0x00000000,
        LPI2C1_SCL_SELECT_INPUT: 0x00000000,
        LPI2C1_SDA_SELECT_INPUT: 0x00000000,
        LPI2C2_SCL_SELECT_INPUT: 0x00000000,
        LPI2C2_SDA_SELECT_INPUT: 0x00000000,
        LPI2C3_SCL_SELECT_INPUT: 0x00000000,
        LPI2C3_SDA_SELECT_INPUT: 0x00000000,
        LPI2C4_SCL_SELECT_INPUT: 0x00000000,
        LPI2C4_SDA_SELECT_INPUT: 0x00000000,
        LPSPI1_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI1_SCK_SELECT_INPUT: 0x00000000,
        LPSPI1_SDI_SELECT_INPUT: 0x00000000,
        LPSPI1_SDO_SELECT_INPUT: 0x00000000,
        LPSPI2_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI2_SCK_SELECT_INPUT: 0x00000000,
        LPSPI2_SDI_SELECT_INPUT: 0x00000000,
        LPSPI2_SDO_SELECT_INPUT: 0x00000000,
        LPSPI3_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI3_SCK_SELECT_INPUT: 0x00000000,
        LPSPI3_SDI_SELECT_INPUT: 0x00000000,
        LPSPI3_SDO_SELECT_INPUT: 0x00000000,
        LPSPI4_PCS0_SELECT_INPUT: 0x00000000,
        LPSPI4_SCK_SELECT_INPUT: 0x00000000,
        LPSPI4_SDI_SELECT_INPUT: 0x00000000,
        LPSPI4_SDO_SELECT_INPUT: 0x00000000,
        LPUART2_RX_SELECT_INPUT: 0x00000000,
        LPUART2_TX_SELECT_INPUT: 0x00000000,
        LPUART3_CTS_B_SELECT_INPUT: 0x00000000,
        LPUART3_RX_SELECT_INPUT: 0x00000000,
        LPUART3_TX_SELECT_INPUT: 0x00000000,
        LPUART4_RX_SELECT_INPUT: 0x00000000,
        LPUART4_TX_SELECT_INPUT: 0x00000000,
        LPUART5_RX_SELECT_INPUT: 0x00000000,
        LPUART5_TX_SELECT_INPUT: 0x00000000,
        LPUART6_RX_SELECT_INPUT: 0x00000000,
        LPUART6_TX_SELECT_INPUT: 0x00000000,
        LPUART7_RX_SELECT_INPUT: 0x00000000,
        LPUART7_TX_SELECT_INPUT: 0x00000000,
        LPUART8_RX_SELECT_INPUT: 0x00000000,
        LPUART8_TX_SELECT_INPUT: 0x00000000,
        NMI_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER0_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER1_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER2_SELECT_INPUT: 0x00000000,
        QTIMER2_TIMER3_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER0_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER1_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER2_SELECT_INPUT: 0x00000000,
        QTIMER3_TIMER3_SELECT_INPUT: 0x00000000,
        SAI1_MCLK2_SELECT_INPUT: 0x00000000,
        SAI1_RX_BCLK_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA0_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA1_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA2_SELECT_INPUT: 0x00000000,
        SAI1_RX_DATA3_SELECT_INPUT: 0x00000000,
        SAI1_RX_SYNC_SELECT_INPUT: 0x00000000,
        SAI1_TX_BCLK_SELECT_INPUT: 0x00000000,
        SAI1_TX_SYNC_SELECT_INPUT: 0x00000000,
        SAI2_MCLK2_SELECT_INPUT: 0x00000000,
        SAI2_RX_BCLK_SELECT_INPUT: 0x00000000,
        SAI2_RX_DATA0_SELECT_INPUT: 0x00000000,
        SAI2_RX_SYNC_SELECT_INPUT: 0x00000000,
        SAI2_TX_BCLK_SELECT_INPUT: 0x00000000,
        SAI2_TX_SYNC_SELECT_INPUT: 0x00000000,
        SPDIF_IN_SELECT_INPUT: 0x00000000,
        USB_OTG2_OC_SELECT_INPUT: 0x00000000,
        USB_OTG1_OC_SELECT_INPUT: 0x00000000,
        USDHC1_CD_B_SELECT_INPUT: 0x00000000,
        USDHC1_WP_SELECT_INPUT: 0x00000000,
        USDHC2_CLK_SELECT_INPUT: 0x00000000,
        USDHC2_CD_B_SELECT_INPUT: 0x00000000,
        USDHC2_CMD_SELECT_INPUT: 0x00000000,
        USDHC2_DATA0_SELECT_INPUT: 0x00000000,
        USDHC2_DATA1_SELECT_INPUT: 0x00000000,
        USDHC2_DATA2_SELECT_INPUT: 0x00000000,
        USDHC2_DATA3_SELECT_INPUT: 0x00000000,
        USDHC2_DATA4_SELECT_INPUT: 0x00000000,
        USDHC2_DATA5_SELECT_INPUT: 0x00000000,
        USDHC2_DATA6_SELECT_INPUT: 0x00000000,
        USDHC2_DATA7_SELECT_INPUT: 0x00000000,
        USDHC2_WP_SELECT_INPUT: 0x00000000,
        XBAR1_IN02_SELECT_INPUT: 0x00000000,
        XBAR1_IN03_SELECT_INPUT: 0x00000000,
        XBAR1_IN04_SELECT_INPUT: 0x00000000,
        XBAR1_IN05_SELECT_INPUT: 0x00000000,
        XBAR1_IN06_SELECT_INPUT: 0x00000000,
        XBAR1_IN07_SELECT_INPUT: 0x00000000,
        XBAR1_IN08_SELECT_INPUT: 0x00000000,
        XBAR1_IN09_SELECT_INPUT: 0x00000000,
        XBAR1_IN17_SELECT_INPUT: 0x00000000,
        XBAR1_IN18_SELECT_INPUT: 0x00000000,
        XBAR1_IN20_SELECT_INPUT: 0x00000000,
        XBAR1_IN22_SELECT_INPUT: 0x00000000,
        XBAR1_IN23_SELECT_INPUT: 0x00000000,
        XBAR1_IN24_SELECT_INPUT: 0x00000000,
        XBAR1_IN14_SELECT_INPUT: 0x00000000,
        XBAR1_IN15_SELECT_INPUT: 0x00000000,
        XBAR1_IN16_SELECT_INPUT: 0x00000000,
        XBAR1_IN25_SELECT_INPUT: 0x00000000,
        XBAR1_IN19_SELECT_INPUT: 0x00000000,
        XBAR1_IN21_SELECT_INPUT: 0x00000000,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_07: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_08: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_09: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_10: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_11: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_12: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B0_13: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_00: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_01: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_02: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_03: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_04: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_05: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_06: 0x000010B0,
        SW_PAD_CTL_PAD_GPIO_SPI_B1_07: 0x000010B0,
        ENET2_IPG_CLK_RMII_SELECT_INPUT: 0x00000000,
        ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT: 0x00000000,
        ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0: 0x00000000,
        ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1: 0x00000000,
        ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT: 0x00000000,
        ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT: 0x00000000,
        ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0: 0x00000000,
        ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT: 0x00000000,
        GPT1_IPP_IND_CAPIN1_SELECT_INPUT: 0x00000000,
        GPT1_IPP_IND_CAPIN2_SELECT_INPUT: 0x00000000,
        GPT1_IPP_IND_CLKIN_SELECT_INPUT: 0x00000000,
        GPT2_IPP_IND_CAPIN1_SELECT_INPUT: 0x00000000,
        GPT2_IPP_IND_CAPIN2_SELECT_INPUT: 0x00000000,
        GPT2_IPP_IND_CLKIN_SELECT_INPUT: 0x00000000,
        SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2: 0x00000000,
        SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0: 0x00000000,
        SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT: 0x00000000,
        SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT: 0x00000000,
        SEMC_I_IPP_IND_DQS4_SELECT_INPUT: 0x00000000,
        CANFD_IPP_IND_CANRX_SELECT_INPUT: 0x00000000,
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
