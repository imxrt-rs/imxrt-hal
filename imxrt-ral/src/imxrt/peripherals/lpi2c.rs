#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPI2C
//!
//! Used by: imxrt1011, imxrt1015, imxrt1021, imxrt1051, imxrt1052, imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Version ID Register
pub mod VERID {

    /// Feature Specification Number
    pub mod FEATURE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000010: Master only, with standard feature set
            pub const FEATURE_2: u32 = 0b0000000000000010;

            /// 0b0000000000000011: Master and slave, with standard feature set
            pub const FEATURE_3: u32 = 0b0000000000000011;
        }
    }

    /// Minor Version Number
    pub mod MINOR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Major Version Number
    pub mod MAJOR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Parameter Register
pub mod PARAM {

    /// Master Transmit FIFO Size
    pub mod MTXFIFO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Master Receive FIFO Size
    pub mod MRXFIFO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Control Register
pub mod MCR {

    /// Master Enable
    pub mod MEN {
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

            /// 0b0: Master logic is disabled
            pub const MEN_0: u32 = 0b0;

            /// 0b1: Master logic is enabled
            pub const MEN_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master logic is not reset
            pub const RST_0: u32 = 0b0;

            /// 0b1: Master logic is reset
            pub const RST_1: u32 = 0b1;
        }
    }

    /// Doze mode enable
    pub mod DOZEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master is enabled in Doze mode
            pub const DOZEN_0: u32 = 0b0;

            /// 0b1: Master is disabled in Doze mode
            pub const DOZEN_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBGEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master is disabled in debug mode
            pub const DBGEN_0: u32 = 0b0;

            /// 0b1: Master is enabled in debug mode
            pub const DBGEN_1: u32 = 0b1;
        }
    }

    /// Reset Transmit FIFO
    pub mod RTF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const RTF_0: u32 = 0b0;

            /// 0b1: Transmit FIFO is reset
            pub const RTF_1: u32 = 0b1;
        }
    }

    /// Reset Receive FIFO
    pub mod RRF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const RRF_0: u32 = 0b0;

            /// 0b1: Receive FIFO is reset
            pub const RRF_1: u32 = 0b1;
        }
    }
}

/// Master Status Register
pub mod MSR {

    /// Transmit Data Flag
    pub mod TDF {
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

            /// 0b0: Transmit data is not requested
            pub const TDF_0: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const TDF_1: u32 = 0b1;
        }
    }

    /// Receive Data Flag
    pub mod RDF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive Data is not ready
            pub const RDF_0: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const RDF_1: u32 = 0b1;
        }
    }

    /// End Packet Flag
    pub mod EPF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master has not generated a STOP or Repeated START condition
            pub const EPF_0: u32 = 0b0;

            /// 0b1: Master has generated a STOP or Repeated START condition
            pub const EPF_1: u32 = 0b1;
        }
    }

    /// STOP Detect Flag
    pub mod SDF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Master has not generated a STOP condition
            pub const SDF_0: u32 = 0b0;

            /// 0b1: Master has generated a STOP condition
            pub const SDF_1: u32 = 0b1;
        }
    }

    /// NACK Detect Flag
    pub mod NDF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Unexpected NACK was not detected
            pub const NDF_0: u32 = 0b0;

            /// 0b1: Unexpected NACK was detected
            pub const NDF_1: u32 = 0b1;
        }
    }

    /// Arbitration Lost Flag
    pub mod ALF {
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

            /// 0b0: Master has not lost arbitration
            pub const ALF_0: u32 = 0b0;

            /// 0b1: Master has lost arbitration
            pub const ALF_1: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: No error
            pub const FEF_0: u32 = 0b0;

            /// 0b1: Master sending or receiving data without a START condition
            pub const FEF_1: u32 = 0b1;
        }
    }

    /// Pin Low Timeout Flag
    pub mod PLTF {
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

            /// 0b0: Pin low timeout has not occurred or is disabled
            pub const PLTF_0: u32 = 0b0;

            /// 0b1: Pin low timeout has occurred
            pub const PLTF_1: u32 = 0b1;
        }
    }

    /// Data Match Flag
    pub mod DMF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Have not received matching data
            pub const DMF_0: u32 = 0b0;

            /// 0b1: Have received matching data
            pub const DMF_1: u32 = 0b1;
        }
    }

    /// Master Busy Flag
    pub mod MBF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I2C Master is idle
            pub const MBF_0: u32 = 0b0;

            /// 0b1: I2C Master is busy
            pub const MBF_1: u32 = 0b1;
        }
    }

    /// Bus Busy Flag
    pub mod BBF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I2C Bus is idle
            pub const BBF_0: u32 = 0b0;

            /// 0b1: I2C Bus is busy
            pub const BBF_1: u32 = 0b1;
        }
    }
}

/// Master Interrupt Enable Register
pub mod MIER {

    /// Transmit Data Interrupt Enable
    pub mod TDIE {
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

            /// 0b0: Disabled
            pub const TDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TDIE_1: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Enable
    pub mod RDIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const RDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RDIE_1: u32 = 0b1;
        }
    }

    /// End Packet Interrupt Enable
    pub mod EPIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const EPIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const EPIE_1: u32 = 0b1;
        }
    }

    /// STOP Detect Interrupt Enable
    pub mod SDIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SDIE_1: u32 = 0b1;
        }
    }

    /// NACK Detect Interrupt Enable
    pub mod NDIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const NDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const NDIE_1: u32 = 0b1;
        }
    }

    /// Arbitration Lost Interrupt Enable
    pub mod ALIE {
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

            /// 0b0: Disabled
            pub const ALIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ALIE_1: u32 = 0b1;
        }
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: Enabled
            pub const FEIE_0: u32 = 0b0;

            /// 0b1: Disabled
            pub const FEIE_1: u32 = 0b1;
        }
    }

    /// Pin Low Timeout Interrupt Enable
    pub mod PLTIE {
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

            /// 0b0: Disabled
            pub const PLTIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const PLTIE_1: u32 = 0b1;
        }
    }

    /// Data Match Interrupt Enable
    pub mod DMIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const DMIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DMIE_1: u32 = 0b1;
        }
    }
}

/// Master DMA Enable Register
pub mod MDER {

    /// Transmit Data DMA Enable
    pub mod TDDE {
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

            /// 0b0: DMA request is disabled
            pub const TDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const TDDE_1: u32 = 0b1;
        }
    }

    /// Receive Data DMA Enable
    pub mod RDDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA request is disabled
            pub const RDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const RDDE_1: u32 = 0b1;
        }
    }
}

/// Master Configuration Register 0
pub mod MCFGR0 {

    /// Host Request Enable
    pub mod HREN {
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

            /// 0b0: Host request input is disabled
            pub const HREN_0: u32 = 0b0;

            /// 0b1: Host request input is enabled
            pub const HREN_1: u32 = 0b1;
        }
    }

    /// Host Request Polarity
    pub mod HRPOL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Active low
            pub const HRPOL_0: u32 = 0b0;

            /// 0b1: Active high
            pub const HRPOL_1: u32 = 0b1;
        }
    }

    /// Host Request Select
    pub mod HRSEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Host request input is pin HREQ
            pub const HRSEL_0: u32 = 0b0;

            /// 0b1: Host request input is input trigger
            pub const HRSEL_1: u32 = 0b1;
        }
    }

    /// Circular FIFO Enable
    pub mod CIRFIFO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Circular FIFO is disabled
            pub const CIRFIFO_0: u32 = 0b0;

            /// 0b1: Circular FIFO is enabled
            pub const CIRFIFO_1: u32 = 0b1;
        }
    }

    /// Receive Data Match Only
    pub mod RDMO {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Received data is stored in the receive FIFO
            pub const RDMO_0: u32 = 0b0;

            /// 0b1: Received data is discarded unless the the Data Match Flag (MSR\[DMF\]) is set
            pub const RDMO_1: u32 = 0b1;
        }
    }
}

/// Master Configuration Register 1
pub mod MCFGR1 {

    /// Prescaler
    pub mod PRESCALE {
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

            /// 0b000: Divide by 1
            pub const PRESCALE_0: u32 = 0b000;

            /// 0b001: Divide by 2
            pub const PRESCALE_1: u32 = 0b001;

            /// 0b010: Divide by 4
            pub const PRESCALE_2: u32 = 0b010;

            /// 0b011: Divide by 8
            pub const PRESCALE_3: u32 = 0b011;

            /// 0b100: Divide by 16
            pub const PRESCALE_4: u32 = 0b100;

            /// 0b101: Divide by 32
            pub const PRESCALE_5: u32 = 0b101;

            /// 0b110: Divide by 64
            pub const PRESCALE_6: u32 = 0b110;

            /// 0b111: Divide by 128
            pub const PRESCALE_7: u32 = 0b111;
        }
    }

    /// Automatic STOP Generation
    pub mod AUTOSTOP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const AUTOSTOP_0: u32 = 0b0;

            /// 0b1: STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy
            pub const AUTOSTOP_1: u32 = 0b1;
        }
    }

    /// IGNACK
    pub mod IGNACK {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPI2C Master will receive ACK and NACK normally
            pub const IGNACK_0: u32 = 0b0;

            /// 0b1: LPI2C Master will treat a received NACK as if it (NACK) was an ACK
            pub const IGNACK_1: u32 = 0b1;
        }
    }

    /// Timeout Configuration
    pub mod TIMECFG {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout
            pub const TIMECFG_0: u32 = 0b0;

            /// 0b1: Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout
            pub const TIMECFG_1: u32 = 0b1;
        }
    }

    /// Match Configuration
    pub mod MATCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Match is disabled
            pub const MATCFG_0: u32 = 0b000;

            /// 0b010: Match is enabled (1st data word equals MATCH0 OR MATCH1)
            pub const MATCFG_2: u32 = 0b010;

            /// 0b011: Match is enabled (any data word equals MATCH0 OR MATCH1)
            pub const MATCFG_3: u32 = 0b011;

            /// 0b100: Match is enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)
            pub const MATCFG_4: u32 = 0b100;

            /// 0b101: Match is enabled (any data word equals MATCH0 AND next data word equals MATCH1)
            pub const MATCFG_5: u32 = 0b101;

            /// 0b110: Match is enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)
            pub const MATCFG_6: u32 = 0b110;

            /// 0b111: Match is enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)
            pub const MATCFG_7: u32 = 0b111;
        }
    }

    /// Pin Configuration
    pub mod PINCFG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 2-pin open drain mode
            pub const PINCFG_0: u32 = 0b000;

            /// 0b001: 2-pin output only mode (ultra-fast mode)
            pub const PINCFG_1: u32 = 0b001;

            /// 0b010: 2-pin push-pull mode
            pub const PINCFG_2: u32 = 0b010;

            /// 0b011: 4-pin push-pull mode
            pub const PINCFG_3: u32 = 0b011;

            /// 0b100: 2-pin open drain mode with separate LPI2C slave
            pub const PINCFG_4: u32 = 0b100;

            /// 0b101: 2-pin output only mode (ultra-fast mode) with separate LPI2C slave
            pub const PINCFG_5: u32 = 0b101;

            /// 0b110: 2-pin push-pull mode with separate LPI2C slave
            pub const PINCFG_6: u32 = 0b110;

            /// 0b111: 4-pin push-pull mode (inverted outputs)
            pub const PINCFG_7: u32 = 0b111;
        }
    }
}

/// Master Configuration Register 2
pub mod MCFGR2 {

    /// Bus Idle Timeout
    pub mod BUSIDLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Glitch Filter SCL
    pub mod FILTSCL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Glitch Filter SDA
    pub mod FILTSDA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Configuration Register 3
pub mod MCFGR3 {

    /// Pin Low Timeout
    pub mod PINLOW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (12 bits: 0xfff << 8)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Data Match Register
pub mod MDMR {

    /// Match 0 Value
    pub mod MATCH0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Match 1 Value
    pub mod MATCH1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Clock Configuration Register 0
pub mod MCCR0 {

    /// Clock Low Period
    pub mod CLKLO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clock High Period
    pub mod CLKHI {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setup Hold Delay
    pub mod SETHOLD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Valid Delay
    pub mod DATAVD {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Clock Configuration Register 1
pub mod MCCR1 {
    pub use super::MCCR0::CLKHI;
    pub use super::MCCR0::CLKLO;
    pub use super::MCCR0::DATAVD;
    pub use super::MCCR0::SETHOLD;
}

/// Master FIFO Control Register
pub mod MFCR {

    /// Transmit FIFO Watermark
    pub mod TXWATER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive FIFO Watermark
    pub mod RXWATER {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master FIFO Status Register
pub mod MFSR {

    /// Transmit FIFO Count
    pub mod TXCOUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive FIFO Count
    pub mod RXCOUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Master Transmit Data Register
pub mod MTDR {

    /// Transmit Data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command Data
    pub mod CMD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Transmit DATA\[7:0\]
            pub const CMD_0: u32 = 0b000;

            /// 0b001: Receive (DATA\[7:0\] + 1) bytes
            pub const CMD_1: u32 = 0b001;

            /// 0b010: Generate STOP condition
            pub const CMD_2: u32 = 0b010;

            /// 0b011: Receive and discard (DATA\[7:0\] + 1) bytes
            pub const CMD_3: u32 = 0b011;

            /// 0b100: Generate (repeated) START and transmit address in DATA\[7:0\]
            pub const CMD_4: u32 = 0b100;

            /// 0b101: Generate (repeated) START and transmit address in DATA\[7:0\]. This transfer expects a NACK to be returned.
            pub const CMD_5: u32 = 0b101;

            /// 0b110: Generate (repeated) START and transmit address in DATA\[7:0\] using high speed mode
            pub const CMD_6: u32 = 0b110;

            /// 0b111: Generate (repeated) START and transmit address in DATA\[7:0\] using high speed mode. This transfer expects a NACK to be returned.
            pub const CMD_7: u32 = 0b111;
        }
    }
}

/// Master Receive Data Register
pub mod MRDR {

    /// Receive Data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Empty
    pub mod RXEMPTY {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive FIFO is not empty
            pub const RXEMPTY_0: u32 = 0b0;

            /// 0b1: Receive FIFO is empty
            pub const RXEMPTY_1: u32 = 0b1;
        }
    }
}

/// Slave Control Register
pub mod SCR {

    /// Slave Enable
    pub mod SEN {
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

            /// 0b0: I2C Slave mode is disabled
            pub const SEN_0: u32 = 0b0;

            /// 0b1: I2C Slave mode is enabled
            pub const SEN_1: u32 = 0b1;
        }
    }

    /// Software Reset
    pub mod RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slave mode logic is not reset
            pub const RST_0: u32 = 0b0;

            /// 0b1: Slave mode logic is reset
            pub const RST_1: u32 = 0b1;
        }
    }

    /// Filter Enable
    pub mod FILTEN {
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

            /// 0b0: Disable digital filter and output delay counter for slave mode
            pub const FILTEN_0: u32 = 0b0;

            /// 0b1: Enable digital filter and output delay counter for slave mode
            pub const FILTEN_1: u32 = 0b1;
        }
    }

    /// Filter Doze Enable
    pub mod FILTDZ {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Filter remains enabled in Doze mode
            pub const FILTDZ_0: u32 = 0b0;

            /// 0b1: Filter is disabled in Doze mode
            pub const FILTDZ_1: u32 = 0b1;
        }
    }

    /// Reset Transmit FIFO
    pub mod RTF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const RTF_0: u32 = 0b0;

            /// 0b1: Transmit Data Register is now empty
            pub const RTF_1: u32 = 0b1;
        }
    }

    /// Reset Receive FIFO
    pub mod RRF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const RRF_0: u32 = 0b0;

            /// 0b1: Receive Data Register is now empty
            pub const RRF_1: u32 = 0b1;
        }
    }
}

/// Slave Status Register
pub mod SSR {

    /// Transmit Data Flag
    pub mod TDF {
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

            /// 0b0: Transmit data not requested
            pub const TDF_0: u32 = 0b0;

            /// 0b1: Transmit data is requested
            pub const TDF_1: u32 = 0b1;
        }
    }

    /// Receive Data Flag
    pub mod RDF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive data is not ready
            pub const RDF_0: u32 = 0b0;

            /// 0b1: Receive data is ready
            pub const RDF_1: u32 = 0b1;
        }
    }

    /// Address Valid Flag
    pub mod AVF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Address Status Register is not valid
            pub const AVF_0: u32 = 0b0;

            /// 0b1: Address Status Register is valid
            pub const AVF_1: u32 = 0b1;
        }
    }

    /// Transmit ACK Flag
    pub mod TAF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit ACK/NACK is not required
            pub const TAF_0: u32 = 0b0;

            /// 0b1: Transmit ACK/NACK is required
            pub const TAF_1: u32 = 0b1;
        }
    }

    /// Repeated Start Flag
    pub mod RSF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slave has not detected a Repeated START condition
            pub const RSF_0: u32 = 0b0;

            /// 0b1: Slave has detected a Repeated START condition
            pub const RSF_1: u32 = 0b1;
        }
    }

    /// STOP Detect Flag
    pub mod SDF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slave has not detected a STOP condition
            pub const SDF_0: u32 = 0b0;

            /// 0b1: Slave has detected a STOP condition
            pub const SDF_1: u32 = 0b1;
        }
    }

    /// Bit Error Flag
    pub mod BEF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slave has not detected a bit error
            pub const BEF_0: u32 = 0b0;

            /// 0b1: Slave has detected a bit error
            pub const BEF_1: u32 = 0b1;
        }
    }

    /// FIFO Error Flag
    pub mod FEF {
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

            /// 0b0: FIFO underflow or overflow was not detected
            pub const FEF_0: u32 = 0b0;

            /// 0b1: FIFO underflow or overflow was detected
            pub const FEF_1: u32 = 0b1;
        }
    }

    /// Address Match 0 Flag
    pub mod AM0F {
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

            /// 0b0: Have not received an ADDR0 matching address
            pub const AM0F_0: u32 = 0b0;

            /// 0b1: Have received an ADDR0 matching address
            pub const AM0F_1: u32 = 0b1;
        }
    }

    /// Address Match 1 Flag
    pub mod AM1F {
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

            /// 0b0: Have not received an ADDR1 or ADDR0/ADDR1 range matching address
            pub const AM1F_0: u32 = 0b0;

            /// 0b1: Have received an ADDR1 or ADDR0/ADDR1 range matching address
            pub const AM1F_1: u32 = 0b1;
        }
    }

    /// General Call Flag
    pub mod GCF {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slave has not detected the General Call Address or the General Call Address is disabled
            pub const GCF_0: u32 = 0b0;

            /// 0b1: Slave has detected the General Call Address
            pub const GCF_1: u32 = 0b1;
        }
    }

    /// SMBus Alert Response Flag
    pub mod SARF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SMBus Alert Response is disabled or not detected
            pub const SARF_0: u32 = 0b0;

            /// 0b1: SMBus Alert Response is enabled and detected
            pub const SARF_1: u32 = 0b1;
        }
    }

    /// Slave Busy Flag
    pub mod SBF {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I2C Slave is idle
            pub const SBF_0: u32 = 0b0;

            /// 0b1: I2C Slave is busy
            pub const SBF_1: u32 = 0b1;
        }
    }

    /// Bus Busy Flag
    pub mod BBF {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: I2C Bus is idle
            pub const BBF_0: u32 = 0b0;

            /// 0b1: I2C Bus is busy
            pub const BBF_1: u32 = 0b1;
        }
    }
}

/// Slave Interrupt Enable Register
pub mod SIER {

    /// Transmit Data Interrupt Enable
    pub mod TDIE {
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

            /// 0b0: Disabled
            pub const TDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TDIE_1: u32 = 0b1;
        }
    }

    /// Receive Data Interrupt Enable
    pub mod RDIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const RDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RDIE_1: u32 = 0b1;
        }
    }

    /// Address Valid Interrupt Enable
    pub mod AVIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const AVIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const AVIE_1: u32 = 0b1;
        }
    }

    /// Transmit ACK Interrupt Enable
    pub mod TAIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const TAIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const TAIE_1: u32 = 0b1;
        }
    }

    /// Repeated Start Interrupt Enable
    pub mod RSIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const RSIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const RSIE_1: u32 = 0b1;
        }
    }

    /// STOP Detect Interrupt Enable
    pub mod SDIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SDIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SDIE_1: u32 = 0b1;
        }
    }

    /// Bit Error Interrupt Enable
    pub mod BEIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const BEIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const BEIE_1: u32 = 0b1;
        }
    }

    /// FIFO Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: Disabled
            pub const FEIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const FEIE_1: u32 = 0b1;
        }
    }

    /// Address Match 0 Interrupt Enable
    pub mod AM0IE {
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

            /// 0b0: Enabled
            pub const AM0IE_0: u32 = 0b0;

            /// 0b1: Disabled
            pub const AM0IE_1: u32 = 0b1;
        }
    }

    /// Address Match 1 Interrupt Enable
    pub mod AM1F {
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

            /// 0b0: Disabled
            pub const AM1F_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const AM1F_1: u32 = 0b1;
        }
    }

    /// General Call Interrupt Enable
    pub mod GCIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const GCIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const GCIE_1: u32 = 0b1;
        }
    }

    /// SMBus Alert Response Interrupt Enable
    pub mod SARIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disabled
            pub const SARIE_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const SARIE_1: u32 = 0b1;
        }
    }
}

/// Slave DMA Enable Register
pub mod SDER {

    /// Transmit Data DMA Enable
    pub mod TDDE {
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

            /// 0b0: DMA request is disabled
            pub const TDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const TDDE_1: u32 = 0b1;
        }
    }

    /// Receive Data DMA Enable
    pub mod RDDE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA request is disabled
            pub const RDDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const RDDE_1: u32 = 0b1;
        }
    }

    /// Address Valid DMA Enable
    pub mod AVDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA request is disabled
            pub const AVDE_0: u32 = 0b0;

            /// 0b1: DMA request is enabled
            pub const AVDE_1: u32 = 0b1;
        }
    }
}

/// Slave Configuration Register 1
pub mod SCFGR1 {

    /// Address SCL Stall
    pub mod ADRSTALL {
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

            /// 0b0: Clock stretching is disabled
            pub const ADRSTALL_0: u32 = 0b0;

            /// 0b1: Clock stretching is enabled
            pub const ADRSTALL_1: u32 = 0b1;
        }
    }

    /// RX SCL Stall
    pub mod RXSTALL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock stretching is disabled
            pub const RXSTALL_0: u32 = 0b0;

            /// 0b1: Clock stretching is enabled
            pub const RXSTALL_1: u32 = 0b1;
        }
    }

    /// TX Data SCL Stall
    pub mod TXDSTALL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock stretching is disabled
            pub const TXDSTALL_0: u32 = 0b0;

            /// 0b1: Clock stretching is enabled
            pub const TXDSTALL_1: u32 = 0b1;
        }
    }

    /// ACK SCL Stall
    pub mod ACKSTALL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock stretching is disabled
            pub const ACKSTALL_0: u32 = 0b0;

            /// 0b1: Clock stretching is enabled
            pub const ACKSTALL_1: u32 = 0b1;
        }
    }

    /// General Call Enable
    pub mod GCEN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: General Call address is disabled
            pub const GCEN_0: u32 = 0b0;

            /// 0b1: General Call address is enabled
            pub const GCEN_1: u32 = 0b1;
        }
    }

    /// SMBus Alert Enable
    pub mod SAEN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disables match on SMBus Alert
            pub const SAEN_0: u32 = 0b0;

            /// 0b1: Enables match on SMBus Alert
            pub const SAEN_1: u32 = 0b1;
        }
    }

    /// Transmit Flag Configuration
    pub mod TXCFG {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty
            pub const TXCFG_0: u32 = 0b0;

            /// 0b1: Transmit Data Flag will assert whenever the Transmit Data register is empty
            pub const TXCFG_1: u32 = 0b1;
        }
    }

    /// Receive Data Configuration
    pub mod RXCFG {
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

            /// 0b0: Reading the Receive Data register will return received data and clear the Receive Data flag (MSR\[RDF\]).
            pub const RXCFG_0: u32 = 0b0;

            /// 0b1: Reading the Receive Data register when the Address Valid flag (SSR\[AVF\])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR\[RDF\]).
            pub const RXCFG_1: u32 = 0b1;
        }
    }

    /// Ignore NACK
    pub mod IGNACK {
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

            /// 0b0: Slave will end transfer when NACK is detected
            pub const IGNACK_0: u32 = 0b0;

            /// 0b1: Slave will not end transfer when NACK detected
            pub const IGNACK_1: u32 = 0b1;
        }
    }

    /// High Speed Mode Enable
    pub mod HSMEN {
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

            /// 0b0: Disables detection of HS-mode master code
            pub const HSMEN_0: u32 = 0b0;

            /// 0b1: Enables detection of HS-mode master code
            pub const HSMEN_1: u32 = 0b1;
        }
    }

    /// Address Configuration
    pub mod ADDRCFG {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Address match 0 (7-bit)
            pub const ADDRCFG_0: u32 = 0b000;

            /// 0b001: Address match 0 (10-bit)
            pub const ADDRCFG_1: u32 = 0b001;

            /// 0b010: Address match 0 (7-bit) or Address match 1 (7-bit)
            pub const ADDRCFG_2: u32 = 0b010;

            /// 0b011: Address match 0 (10-bit) or Address match 1 (10-bit)
            pub const ADDRCFG_3: u32 = 0b011;

            /// 0b100: Address match 0 (7-bit) or Address match 1 (10-bit)
            pub const ADDRCFG_4: u32 = 0b100;

            /// 0b101: Address match 0 (10-bit) or Address match 1 (7-bit)
            pub const ADDRCFG_5: u32 = 0b101;

            /// 0b110: From Address match 0 (7-bit) to Address match 1 (7-bit)
            pub const ADDRCFG_6: u32 = 0b110;

            /// 0b111: From Address match 0 (10-bit) to Address match 1 (10-bit)
            pub const ADDRCFG_7: u32 = 0b111;
        }
    }
}

/// Slave Configuration Register 2
pub mod SCFGR2 {

    /// Clock Hold Time
    pub mod CLKHOLD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Valid Delay
    pub mod DATAVD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Glitch Filter SCL
    pub mod FILTSCL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Glitch Filter SDA
    pub mod FILTSDA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Slave Address Match Register
pub mod SAMR {

    /// Address 0 Value
    pub mod ADDR0 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (10 bits: 0x3ff << 1)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address 1 Value
    pub mod ADDR1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (10 bits: 0x3ff << 17)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Slave Address Status Register
pub mod SASR {

    /// Received Address
    pub mod RADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Address Not Valid
    pub mod ANV {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Received Address (RADDR) is valid
            pub const ANV_0: u32 = 0b0;

            /// 0b1: Received Address (RADDR) is not valid
            pub const ANV_1: u32 = 0b1;
        }
    }
}

/// Slave Transmit ACK Register
pub mod STAR {

    /// Transmit NACK
    pub mod TXNACK {
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

            /// 0b0: Write a Transmit ACK for each received word
            pub const TXNACK_0: u32 = 0b0;

            /// 0b1: Write a Transmit NACK for each received word
            pub const TXNACK_1: u32 = 0b1;
        }
    }
}

/// Slave Transmit Data Register
pub mod STDR {

    /// Transmit Data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Slave Receive Data Register
pub mod SRDR {

    /// Receive Data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Empty
    pub mod RXEMPTY {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The Receive Data Register is not empty
            pub const RXEMPTY_0: u32 = 0b0;

            /// 0b1: The Receive Data Register is empty
            pub const RXEMPTY_1: u32 = 0b1;
        }
    }

    /// Start Of Frame
    pub mod SOF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Indicates this is not the first data word since a (repeated) START or STOP condition
            pub const SOF_0: u32 = 0b0;

            /// 0b1: Indicates this is the first data word since a (repeated) START or STOP condition
            pub const SOF_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    _reserved1: [u32; 2],

    /// Master Control Register
    pub MCR: RWRegister<u32>,

    /// Master Status Register
    pub MSR: RWRegister<u32>,

    /// Master Interrupt Enable Register
    pub MIER: RWRegister<u32>,

    /// Master DMA Enable Register
    pub MDER: RWRegister<u32>,

    /// Master Configuration Register 0
    pub MCFGR0: RWRegister<u32>,

    /// Master Configuration Register 1
    pub MCFGR1: RWRegister<u32>,

    /// Master Configuration Register 2
    pub MCFGR2: RWRegister<u32>,

    /// Master Configuration Register 3
    pub MCFGR3: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// Master Data Match Register
    pub MDMR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Master Clock Configuration Register 0
    pub MCCR0: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Master Clock Configuration Register 1
    pub MCCR1: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// Master FIFO Control Register
    pub MFCR: RWRegister<u32>,

    /// Master FIFO Status Register
    pub MFSR: RORegister<u32>,

    /// Master Transmit Data Register
    pub MTDR: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Master Receive Data Register
    pub MRDR: RORegister<u32>,

    _reserved7: [u32; 39],

    /// Slave Control Register
    pub SCR: RWRegister<u32>,

    /// Slave Status Register
    pub SSR: RWRegister<u32>,

    /// Slave Interrupt Enable Register
    pub SIER: RWRegister<u32>,

    /// Slave DMA Enable Register
    pub SDER: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Slave Configuration Register 1
    pub SCFGR1: RWRegister<u32>,

    /// Slave Configuration Register 2
    pub SCFGR2: RWRegister<u32>,

    _reserved9: [u32; 5],

    /// Slave Address Match Register
    pub SAMR: RWRegister<u32>,

    _reserved10: [u32; 3],

    /// Slave Address Status Register
    pub SASR: RORegister<u32>,

    /// Slave Transmit ACK Register
    pub STAR: RWRegister<u32>,

    _reserved11: [u32; 2],

    /// Slave Transmit Data Register
    pub STDR: RWRegister<u32>,

    _reserved12: [u32; 3],

    /// Slave Receive Data Register
    pub SRDR: RORegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub MCR: u32,
    pub MSR: u32,
    pub MIER: u32,
    pub MDER: u32,
    pub MCFGR0: u32,
    pub MCFGR1: u32,
    pub MCFGR2: u32,
    pub MCFGR3: u32,
    pub MDMR: u32,
    pub MCCR0: u32,
    pub MCCR1: u32,
    pub MFCR: u32,
    pub MFSR: u32,
    pub MTDR: u32,
    pub MRDR: u32,
    pub SCR: u32,
    pub SSR: u32,
    pub SIER: u32,
    pub SDER: u32,
    pub SCFGR1: u32,
    pub SCFGR2: u32,
    pub SAMR: u32,
    pub SASR: u32,
    pub STAR: u32,
    pub STDR: u32,
    pub SRDR: u32,
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
