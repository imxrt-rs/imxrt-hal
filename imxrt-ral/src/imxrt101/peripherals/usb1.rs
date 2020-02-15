#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB
//!
//! Used by: imxrt1011, imxrt1015

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Identification register
pub mod ID {

    /// Configuration number
    pub mod ID {
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

    /// Complement version of ID
    pub mod NID {
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

    /// Revision number of the controller core.
    pub mod REVISION {
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

/// Hardware General
pub mod HWGENERAL {

    /// Data width of the transciever connected to the controller core. PHYW bit reset value is
    pub mod PHYW {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8 bit wide data bus Software non-programmable
            pub const PHYW_0: u32 = 0b00;

            /// 0b01: 16 bit wide data bus Software non-programmable
            pub const PHYW_1: u32 = 0b01;

            /// 0b10: Reset to 8 bit wide data bus Software programmable
            pub const PHYW_2: u32 = 0b10;

            /// 0b11: Reset to 16 bit wide data bus Software programmable
            pub const PHYW_3: u32 = 0b11;
        }
    }

    /// Transciever type
    pub mod PHYM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: UTMI/UMTI+
            pub const PHYM_0: u32 = 0b000;

            /// 0b001: ULPI DDR
            pub const PHYM_1: u32 = 0b001;

            /// 0b010: ULPI
            pub const PHYM_2: u32 = 0b010;

            /// 0b011: Serial Only
            pub const PHYM_3: u32 = 0b011;

            /// 0b100: Software programmable - reset to UTMI/UTMI+
            pub const PHYM_4: u32 = 0b100;

            /// 0b101: Software programmable - reset to ULPI DDR
            pub const PHYM_5: u32 = 0b101;

            /// 0b110: Software programmable - reset to ULPI
            pub const PHYM_6: u32 = 0b110;

            /// 0b111: Software programmable - reset to Serial
            pub const PHYM_7: u32 = 0b111;
        }
    }

    /// Serial interface mode capability
    pub mod SM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No Serial Engine, always use parallel signalling.
            pub const SM_0: u32 = 0b00;

            /// 0b01: Serial Engine present, always use serial signalling for FS/LS.
            pub const SM_1: u32 = 0b01;

            /// 0b10: Software programmable - Reset to use parallel signalling for FS/LS
            pub const SM_2: u32 = 0b10;

            /// 0b11: Software programmable - Reset to use serial signalling for FS/LS
            pub const SM_3: u32 = 0b11;
        }
    }
}

/// Host Hardware Parameters
pub mod HWHOST {

    /// Host Capable. Indicating whether host operation mode is supported or not.
    pub mod HC {
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

            /// 0b0: Not supported
            pub const HC_0: u32 = 0b0;

            /// 0b1: Supported
            pub const HC_1: u32 = 0b1;
        }
    }

    /// The Nmber of downstream ports supported by the host controller is (NPORT+1)
    pub mod NPORT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Device Hardware Parameters
pub mod HWDEVICE {

    /// Device Capable. Indicating whether device operation mode is supported or not.
    pub mod DC {
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

            /// 0b0: Not supported
            pub const DC_0: u32 = 0b0;

            /// 0b1: Supported
            pub const DC_1: u32 = 0b1;
        }
    }

    /// Device Endpoint Number
    pub mod DEVEP {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (5 bits: 0b11111 << 1)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TX Buffer Hardware Parameters
pub mod HWTXBUF {

    /// Default burst size for memory to TX buffer transfer
    pub mod TXBURST {
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

    /// TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes
    pub mod TXCHANADD {
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

/// RX Buffer Hardware Parameters
pub mod HWRXBUF {

    /// Default burst size for memory to RX buffer transfer
    pub mod RXBURST {
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

    /// Buffer total size for all receive endpoints is (2^RXADD)
    pub mod RXADD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// General Purpose Timer #0 Load
pub mod GPTIMER0LD {

    /// General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'
    pub mod GPTLD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// General Purpose Timer #0 Controller
pub mod GPTIMER0CTRL {

    /// General Purpose Timer Counter. This field is the count value of the countdown timer.
    pub mod GPTCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again
    pub mod GPTMODE {
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

            /// 0b0: One Shot Mode
            pub const GPTMODE_0: u32 = 0b0;

            /// 0b1: Repeat Mode
            pub const GPTMODE_1: u32 = 0b1;
        }
    }

    /// General Purpose Timer Reset
    pub mod GPTRST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action
            pub const GPTRST_0: u32 = 0b0;

            /// 0b1: Load counter value from GPTLD bits in n_GPTIMER0LD
            pub const GPTRST_1: u32 = 0b1;
        }
    }

    /// General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit.
    pub mod GPTRUN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stop counting
            pub const GPTRUN_0: u32 = 0b0;

            /// 0b1: Run
            pub const GPTRUN_1: u32 = 0b1;
        }
    }
}

/// General Purpose Timer #1 Load
pub mod GPTIMER1LD {
    pub use super::GPTIMER0LD::GPTLD;
}

/// General Purpose Timer #1 Controller
pub mod GPTIMER1CTRL {

    /// General Purpose Timer Counter. This field is the count value of the countdown timer.
    pub mod GPTCNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software
    pub mod GPTMODE {
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

            /// 0b0: One Shot Mode
            pub const GPTMODE_0: u32 = 0b0;

            /// 0b1: Repeat Mode
            pub const GPTMODE_1: u32 = 0b1;
        }
    }

    /// General Purpose Timer Reset
    pub mod GPTRST {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action
            pub const GPTRST_0: u32 = 0b0;

            /// 0b1: Load counter value from GPTLD bits in USB_n_GPTIMER0LD
            pub const GPTRST_1: u32 = 0b1;
        }
    }

    /// General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit.
    pub mod GPTRUN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Stop counting
            pub const GPTRUN_0: u32 = 0b0;

            /// 0b1: Run
            pub const GPTRUN_1: u32 = 0b1;
        }
    }
}

/// System Bus Config
pub mod SBUSCFG {

    /// AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)
    pub mod AHBBRST {
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

            /// 0b000: Incremental burst of unspecified length only
            pub const AHBBRST_0: u32 = 0b000;

            /// 0b001: INCR4 burst, then single transfer
            pub const AHBBRST_1: u32 = 0b001;

            /// 0b010: INCR8 burst, INCR4 burst, then single transfer
            pub const AHBBRST_2: u32 = 0b010;

            /// 0b011: INCR16 burst, INCR8 burst, INCR4 burst, then single transfer
            pub const AHBBRST_3: u32 = 0b011;

            /// 0b101: INCR4 burst, then incremental burst of unspecified length
            pub const AHBBRST_5: u32 = 0b101;

            /// 0b110: INCR8 burst, INCR4 burst, then incremental burst of unspecified length
            pub const AHBBRST_6: u32 = 0b110;

            /// 0b111: INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length
            pub const AHBBRST_7: u32 = 0b111;
        }
    }
}

/// Capability Registers Length
pub mod CAPLENGTH {

    /// These bits are used as an offset to add to register base to find the beginning of the Operational Register
    pub mod CAPLENGTH {
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

/// Host Controller Interface Version
pub mod HCIVERSION {

    /// Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0.
    pub mod HCIVERSION {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Host Controller Structural Parameters
pub mod HCSPARAMS {

    /// Number of downstream ports
    pub mod N_PORTS {
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

    /// Port Power Control This field indicates whether the host controller implementation includes port power control
    pub mod PPC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller
    pub mod N_PCC {
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

    /// Number of Companion Controller (N_CC)
    pub mod N_CC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: There is no internal Companion Controller and port-ownership hand-off is not supported.
            pub const N_CC_0: u32 = 0b0000;

            /// 0b0001: There are internal companion controller(s) and port-ownership hand-offs is supported.
            pub const N_CC_1: u32 = 0b0001;
        }
    }

    /// Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control
    pub mod PI {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Ports per Transaction Translator (N_PTT)
    pub mod N_PTT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Transaction Translators (N_TT)
    pub mod N_TT {
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

/// Host Controller Capability Parameters
pub mod HCCPARAMS {

    /// 64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported
    pub mod ADC {
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

    /// Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller
    pub mod PFL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule
    pub mod ASP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Isochronous Scheduling Threshold
    pub mod IST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EHCI Extended Capabilities Pointer
    pub mod EECP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Device Controller Interface Version
pub mod DCIVERSION {

    /// Device Controller Interface Version Number Default value is '01h', which means rev0.1.
    pub mod DCIVERSION {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Device Controller Capability Parameters
pub mod DCCPARAMS {

    /// Device Endpoint Number This field indicates the number of endpoints built into the device controller
    pub mod DEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device.
    pub mod DC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2
    pub mod HC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB Command Register
pub mod USBCMD {

    /// Run/Stop (RS) - Read/Write
    pub mod RS {
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

    /// Controller Reset (RESET) - Read/Write
    pub mod RST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// See description at bit 15
    pub mod FS_1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Periodic Schedule Enable- Read/Write
    pub mod PSE {
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

            /// 0b0: Do not process the Periodic Schedule
            pub const PSE_0: u32 = 0b0;

            /// 0b1: Use the PERIODICLISTBASE register to access the Periodic Schedule.
            pub const PSE_1: u32 = 0b1;
        }
    }

    /// Asynchronous Schedule Enable - Read/Write
    pub mod ASE {
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

            /// 0b0: Do not process the Asynchronous Schedule.
            pub const ASE_0: u32 = 0b0;

            /// 0b1: Use the ASYNCLISTADDR register to access the Asynchronous Schedule.
            pub const ASE_1: u32 = 0b1;
        }
    }

    /// Interrupt on Async Advance Doorbell - Read/Write
    pub mod IAA {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous Schedule Park Mode Count - Read/Write
    pub mod ASP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous Schedule Park Mode Enable - Read/Write
    pub mod ASPE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setup TripWire - Read/Write
    pub mod SUTW {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Add dTD TripWire - Read/Write
    pub mod ATDTW {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame List Size - (Read/Write or Read Only)
    pub mod FS_2 {
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

            /// 0b0: 1024 elements (4096 bytes) Default value
            pub const FS_2_0: u32 = 0b0;

            /// 0b1: 512 elements (2048 bytes)
            pub const FS_2_1: u32 = 0b1;
        }
    }

    /// Interrupt Threshold Control -Read/Write
    pub mod ITC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Immediate (no threshold)
            pub const ITC_0: u32 = 0b00000000;

            /// 0b00000001: 1 micro-frame
            pub const ITC_1: u32 = 0b00000001;

            /// 0b00000010: 2 micro-frames
            pub const ITC_2: u32 = 0b00000010;

            /// 0b00000100: 4 micro-frames
            pub const ITC_4: u32 = 0b00000100;

            /// 0b00001000: 8 micro-frames
            pub const ITC_8: u32 = 0b00001000;

            /// 0b00010000: 16 micro-frames
            pub const ITC_16: u32 = 0b00010000;

            /// 0b00100000: 32 micro-frames
            pub const ITC_32: u32 = 0b00100000;

            /// 0b01000000: 64 micro-frames
            pub const ITC_64: u32 = 0b01000000;
        }
    }
}

/// USB Status Register
pub mod USBSTS {

    /// USB Interrupt (USBINT) - R/WC
    pub mod UI {
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

    /// USB Error Interrupt (USBERRINT) - R/WC
    pub mod UEI {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Change Detect - R/WC
    pub mod PCI {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame List Rollover - R/WC
    pub mod FRI {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Error- R/WC
    pub mod SEI {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt on Async Advance - R/WC
    pub mod AAI {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB Reset Received - R/WC
    pub mod URI {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF Received - R/WC
    pub mod SRI {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DCSuspend - R/WC
    pub mod SLI {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ULPI Interrupt - R/WC
    pub mod ULPII {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HCHaIted - Read Only
    pub mod HCH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reclamation - Read Only
    pub mod RCL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Periodic Schedule Status - Read Only
    pub mod PS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous Schedule Status - Read Only
    pub mod AS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAK Interrupt Bit--RO
    pub mod NAKI {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer Interrupt 0(GPTINT0)--R/WC
    pub mod TI0 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer Interrupt 1(GPTINT1)--R/WC
    pub mod TI1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Enable Register
pub mod USBINTR {

    /// USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod UE {
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

    /// USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod UEE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod PCE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod FRE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod SEE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod AAE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod URE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod SRE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt
    pub mod SLE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod ULPIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod NAKE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold
    pub mod UAIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold
    pub mod UPIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod TIE0 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt
    pub mod TIE1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB Frame Index
pub mod FRINDEX {

    /// Frame Index
    pub mod FRINDEX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000000000: (1024) 12
            pub const FRINDEX_0: u32 = 0b00000000000000;

            /// 0b00000000000001: (512) 11
            pub const FRINDEX_1: u32 = 0b00000000000001;

            /// 0b00000000000010: (256) 10
            pub const FRINDEX_2: u32 = 0b00000000000010;

            /// 0b00000000000011: (128) 9
            pub const FRINDEX_3: u32 = 0b00000000000011;

            /// 0b00000000000100: (64) 8
            pub const FRINDEX_4: u32 = 0b00000000000100;

            /// 0b00000000000101: (32) 7
            pub const FRINDEX_5: u32 = 0b00000000000101;

            /// 0b00000000000110: (16) 6
            pub const FRINDEX_6: u32 = 0b00000000000110;

            /// 0b00000000000111: (8) 5
            pub const FRINDEX_7: u32 = 0b00000000000111;
        }
    }
}

/// DEVICEADDR and PERIODICLISTBASE
/// DEVICEADDR: Device Address
/// PERIODICLISTBASE: Frame List Base Address
pub mod DEVICEADDR {

    /// Device Address Advance
    pub mod USBADRA {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Device Address. These bits correspond to the USB device address
    pub mod USBADR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (7 bits: 0x7f << 25)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Base Address (Low)
    pub mod BASEADR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (20 bits: 0xfffff << 12)
        pub const mask: u32 = 0xfffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ASYNCLISTADDR and ENDPTLISTADDR
/// ASYNCLISTADDR: Next Asynch. Address
/// ENDPTLISTADDR: Endpoint List Address
pub mod ASYNCLISTADDR {

    /// Link Pointer Low (LPL)
    pub mod ASYBASE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (27 bits: 0x7ffffff << 5)
        pub const mask: u32 = 0x7ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint List Pointer(Low)
    pub mod EPBASE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (21 bits: 0x1fffff << 11)
        pub const mask: u32 = 0x1fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Programmable Burst Size
pub mod BURSTSIZE {

    /// Programmable RX Burst Size
    pub mod RXPBURST {
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

    /// Programmable TX Burst Size
    pub mod TXPBURST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (9 bits: 0x1ff << 8)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TX FIFO Fill Tuning
pub mod TXFILLTUNING {

    /// Scheduler Overhead
    pub mod TXSCHOH {
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

    /// Scheduler Health Counter
    pub mod TXSCHHEALTH {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FIFO Burst Threshold
    pub mod TXFIFOTHRES {
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
}

/// Endpoint NAK
pub mod ENDPTNAK {

    /// RX Endpoint NAK - R/WC
    pub mod EPRN {
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

    /// TX Endpoint NAK - R/WC
    pub mod EPTN {
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

/// Endpoint NAK Enable
pub mod ENDPTNAKEN {

    /// RX Endpoint NAK Enable - R/W
    pub mod EPRNE {
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

    /// TX Endpoint NAK Enable - R/W
    pub mod EPTNE {
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

/// Configure Flag Register
pub mod CONFIGFLAG {

    /// Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller
    pub mod CF {
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

            /// 0b0: Port routing control logic default-routes each port to an implementation dependent classic host controller.
            pub const CF_0: u32 = 0b0;

            /// 0b1: Port routing control logic default-routes all ports to this host controller.
            pub const CF_1: u32 = 0b1;
        }
    }
}

/// Port Status & Control
pub mod PORTSC1 {

    /// Current Connect Status-Read Only
    pub mod CCS {
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

    /// Connect Status Change-R/WC
    pub mod CSC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Enabled/Disabled-Read/Write
    pub mod PE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Enable/Disable Change-R/WC
    pub mod PEC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Over-current Active-Read Only
    pub mod OCA {
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

            /// 0b0: This port does not have an over-current condition.
            pub const OCA_0: u32 = 0b0;

            /// 0b1: This port currently has an over-current condition
            pub const OCA_1: u32 = 0b1;
        }
    }

    /// Over-current Change-R/WC
    pub mod OCC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force Port Resume -Read/Write
    pub mod FPR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Suspend - Read/Write or Read Only
    pub mod SUSP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Reset - Read/Write or Read Only
    pub mod PR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// High-Speed Port - Read Only
    pub mod HSP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Line Status-Read Only
    pub mod LS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SE0
            pub const LS_0: u32 = 0b00;

            /// 0b01: K-state
            pub const LS_1: u32 = 0b01;

            /// 0b10: J-state
            pub const LS_2: u32 = 0b10;

            /// 0b11: Undefined
            pub const LS_3: u32 = 0b11;
        }
    }

    /// Port Power (PP)-Read/Write or Read Only
    pub mod PP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Owner-Read/Write
    pub mod PO {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Indicator Control - Read/Write
    pub mod PIC {
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

            /// 0b00: Port indicators are off
            pub const PIC_0: u32 = 0b00;

            /// 0b01: Amber
            pub const PIC_1: u32 = 0b01;

            /// 0b10: Green
            pub const PIC_2: u32 = 0b10;

            /// 0b11: Undefined
            pub const PIC_3: u32 = 0b11;
        }
    }

    /// Port Test Control - Read/Write
    pub mod PTC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: TEST_MODE_DISABLE
            pub const PTC_0: u32 = 0b0000;

            /// 0b0001: J_STATE
            pub const PTC_1: u32 = 0b0001;

            /// 0b0010: K_STATE
            pub const PTC_2: u32 = 0b0010;

            /// 0b0011: SE0 (host) / NAK (device)
            pub const PTC_3: u32 = 0b0011;

            /// 0b0100: Packet
            pub const PTC_4: u32 = 0b0100;

            /// 0b0101: FORCE_ENABLE_HS
            pub const PTC_5: u32 = 0b0101;

            /// 0b0110: FORCE_ENABLE_FS
            pub const PTC_6: u32 = 0b0110;

            /// 0b0111: FORCE_ENABLE_LS
            pub const PTC_7: u32 = 0b0111;
        }
    }

    /// Wake on Connect Enable (WKCNNT_E) - Read/Write
    pub mod WKCN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write
    pub mod WKDC {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wake on Over-current Enable (WKOC_E) - Read/Write
    pub mod WKOC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write
    pub mod PHCD {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enable PHY clock
            pub const PHCD_0: u32 = 0b0;

            /// 0b1: Disable PHY clock
            pub const PHCD_1: u32 = 0b1;
        }
    }

    /// Port Force Full Speed Connect - Read/Write
    pub mod PFSC {
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

            /// 0b0: Normal operation
            pub const PFSC_0: u32 = 0b0;

            /// 0b1: Forced to full speed
            pub const PFSC_1: u32 = 0b1;
        }
    }

    /// See description at bits 31-30
    pub mod PTS_2 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port Speed - Read Only. This register field indicates the speed at which the port is operating.
    pub mod PSPD {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Full Speed
            pub const PSPD_0: u32 = 0b00;

            /// 0b01: Low Speed
            pub const PSPD_1: u32 = 0b01;

            /// 0b10: High Speed
            pub const PSPD_2: u32 = 0b10;

            /// 0b11: Undefined
            pub const PSPD_3: u32 = 0b11;
        }
    }

    /// Parallel Transceiver Width This bit has no effect if serial interface engine is used
    pub mod PTW {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Select the 8-bit UTMI interface \[60MHz\]
            pub const PTW_0: u32 = 0b0;

            /// 0b1: Select the 16-bit UTMI interface \[30MHz\]
            pub const PTW_1: u32 = 0b1;
        }
    }

    /// Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals
    pub mod STS {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// All USB port interface modes are listed in this field description, but not all are supported
    pub mod PTS_1 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// On-The-Go Status & control
pub mod OTGSC {

    /// VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor.
    pub mod VD {
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

    /// VBUS Charge - Read/Write
    pub mod VC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTG Termination - Read/Write
    pub mod OT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Pulsing - Read/Write
    pub mod DP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \[default\]
    pub mod IDPU {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB ID - Read Only. 0 = A device, 1 = B device
    pub mod ID {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold.
    pub mod AVV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A Session Valid - Read Only. Indicates VBus is above the A session valid threshold.
    pub mod ASV {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session Valid - Read Only. Indicates VBus is above the B session valid threshold.
    pub mod BSV {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session End - Read Only. Indicates VBus is below the B session end threshold.
    pub mod BSE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 millisecond timer toggle - Read Only. This bit toggles once per millisecond.
    pub mod TOG_1MS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Bus Pulsing Status - Read Only
    pub mod DPS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB ID Interrupt Status - Read/Write
    pub mod IDIS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A VBus Valid Interrupt Status - Read/Write to Clear
    pub mod AVVIS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A Session Valid Interrupt Status - Read/Write to Clear
    pub mod ASVIS {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session Valid Interrupt Status - Read/Write to Clear
    pub mod BSVIS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session End Interrupt Status - Read/Write to Clear
    pub mod BSEIS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 millisecond timer Interrupt Status - Read/Write to Clear
    pub mod STATUS_1MS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Pulse Interrupt Status - Read/Write to Clear
    pub mod DPIS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt.
    pub mod IDIE {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt.
    pub mod AVVIE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// A Session Valid Interrupt Enable - Read/Write
    pub mod ASVIE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session Valid Interrupt Enable - Read/Write
    pub mod BSVIE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt.
    pub mod BSEIE {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 millisecond timer Interrupt Enable - Read/Write
    pub mod EN_1MS {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Pulse Interrupt Enable
    pub mod DPIE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB Device Mode
pub mod USBMODE {

    /// Controller Mode - R/WO
    pub mod CM {
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

            /// 0b00: Idle \[Default for combination host/device\]
            pub const CM_0: u32 = 0b00;

            /// 0b10: Device Controller \[Default for device only controller\]
            pub const CM_2: u32 = 0b10;

            /// 0b11: Host Controller \[Default for host only controller\]
            pub const CM_3: u32 = 0b11;
        }
    }

    /// Endian Select - Read/Write
    pub mod ES {
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

            /// 0b0: Little Endian \[Default\]
            pub const ES_0: u32 = 0b0;

            /// 0b1: Big Endian
            pub const ES_1: u32 = 0b1;
        }
    }

    /// Setup Lockout Mode
    pub mod SLOM {
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

            /// 0b0: Setup Lockouts On (default);
            pub const SLOM_0: u32 = 0b0;

            /// 0b1: Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register .
            pub const SLOM_1: u32 = 0b1;
        }
    }

    /// Stream Disable Mode
    pub mod SDIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Endpoint Setup Status
pub mod ENDPTSETUPSTAT {

    /// Setup Endpoint Status
    pub mod ENDPTSETUPSTAT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Endpoint Prime
pub mod ENDPTPRIME {

    /// Prime Endpoint Receive Buffer - R/WS
    pub mod PERB {
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

    /// Prime Endpoint Transmit Buffer - R/WS
    pub mod PETB {
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

/// Endpoint Flush
pub mod ENDPTFLUSH {

    /// Flush Endpoint Receive Buffer - R/WS
    pub mod FERB {
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

    /// Flush Endpoint Transmit Buffer - R/WS
    pub mod FETB {
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

/// Endpoint Status
pub mod ENDPTSTAT {

    /// Endpoint Receive Buffer Ready -- Read Only
    pub mod ERBR {
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

    /// Endpoint Transmit Buffer Ready -- Read Only
    pub mod ETBR {
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

/// Endpoint Complete
pub mod ENDPTCOMPLETE {

    /// Endpoint Receive Complete Event - RW/C
    pub mod ERCE {
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

    /// Endpoint Transmit Complete Event - R/WC
    pub mod ETCE {
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

/// Endpoint Control0
pub mod ENDPTCTRL0 {

    /// RX Endpoint Stall - Read/Write 0 End Point OK
    pub mod RXS {
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

    /// RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point.
    pub mod RXT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Endpoint Enable 1 Enabled Endpoint0 is always enabled.
    pub mod RXE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Stall - Read/Write 0 End Point OK \[Default\] 1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host
    pub mod TXS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point.
    pub mod TXT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Enable 1 Enabled Endpoint0 is always enabled.
    pub mod TXE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Endpoint Control
pub mod ENDPTCTRL1 {

    /// RX Endpoint Stall - Read/Write 0 End Point OK
    pub mod RXS {
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

    /// RX Endpoint Data Sink - Read/Write 0 Dual Port Memory Buffer/DMA Engine \[Default\] Should always be written as zero
    pub mod RXD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    pub mod RXT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Data Toggle Inhibit 0 Disabled \[Default\] 1 Enabled This bit is only used for test and should always be written as zero
    pub mod RXI {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device
    pub mod RXR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured
    pub mod RXE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared
    pub mod TXS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Data Source - Read/Write 0 Dual Port Memory Buffer/DMA Engine \[DEFAULT\] Should always be written as 0
    pub mod TXD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt
    pub mod TXT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Data Toggle Inhibit 0 PID Sequencing Enabled
    pub mod TXI {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device
    pub mod TXR {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TX Endpoint Enable 0 Disabled \[Default\] 1 Enabled An Endpoint should be enabled only after it has been configured
    pub mod TXE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Endpoint Control
pub mod ENDPTCTRL2 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control
pub mod ENDPTCTRL3 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control
pub mod ENDPTCTRL4 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control
pub mod ENDPTCTRL5 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control
pub mod ENDPTCTRL6 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}

/// Endpoint Control
pub mod ENDPTCTRL7 {
    pub use super::ENDPTCTRL1::RXD;
    pub use super::ENDPTCTRL1::RXE;
    pub use super::ENDPTCTRL1::RXI;
    pub use super::ENDPTCTRL1::RXR;
    pub use super::ENDPTCTRL1::RXS;
    pub use super::ENDPTCTRL1::RXT;
    pub use super::ENDPTCTRL1::TXD;
    pub use super::ENDPTCTRL1::TXE;
    pub use super::ENDPTCTRL1::TXI;
    pub use super::ENDPTCTRL1::TXR;
    pub use super::ENDPTCTRL1::TXS;
    pub use super::ENDPTCTRL1::TXT;
}
pub struct RegisterBlock {
    /// Identification register
    pub ID: RORegister<u32>,

    /// Hardware General
    pub HWGENERAL: RORegister<u32>,

    /// Host Hardware Parameters
    pub HWHOST: RORegister<u32>,

    /// Device Hardware Parameters
    pub HWDEVICE: RORegister<u32>,

    /// TX Buffer Hardware Parameters
    pub HWTXBUF: RORegister<u32>,

    /// RX Buffer Hardware Parameters
    pub HWRXBUF: RORegister<u32>,

    _reserved1: [u32; 26],

    /// General Purpose Timer #0 Load
    pub GPTIMER0LD: RWRegister<u32>,

    /// General Purpose Timer #0 Controller
    pub GPTIMER0CTRL: RWRegister<u32>,

    /// General Purpose Timer #1 Load
    pub GPTIMER1LD: RWRegister<u32>,

    /// General Purpose Timer #1 Controller
    pub GPTIMER1CTRL: RWRegister<u32>,

    /// System Bus Config
    pub SBUSCFG: RWRegister<u32>,

    _reserved2: [u32; 27],

    /// Capability Registers Length
    pub CAPLENGTH: RORegister<u8>,

    _reserved3: [u8; 1],

    /// Host Controller Interface Version
    pub HCIVERSION: RORegister<u16>,

    /// Host Controller Structural Parameters
    pub HCSPARAMS: RORegister<u32>,

    /// Host Controller Capability Parameters
    pub HCCPARAMS: RORegister<u32>,

    _reserved4: [u32; 5],

    /// Device Controller Interface Version
    pub DCIVERSION: RORegister<u16>,

    _reserved5: [u16; 1],

    /// Device Controller Capability Parameters
    pub DCCPARAMS: RORegister<u32>,

    _reserved6: [u32; 6],

    /// USB Command Register
    pub USBCMD: RWRegister<u32>,

    /// USB Status Register
    pub USBSTS: RWRegister<u32>,

    /// Interrupt Enable Register
    pub USBINTR: RWRegister<u32>,

    /// USB Frame Index
    pub FRINDEX: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// DEVICEADDR and PERIODICLISTBASE
    /// DEVICEADDR: Device Address
    /// PERIODICLISTBASE: Frame List Base Address
    pub DEVICEADDR: RWRegister<u32>,

    /// ASYNCLISTADDR and ENDPTLISTADDR
    /// ASYNCLISTADDR: Next Asynch. Address
    /// ENDPTLISTADDR: Endpoint List Address
    pub ASYNCLISTADDR: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// Programmable Burst Size
    pub BURSTSIZE: RWRegister<u32>,

    /// TX FIFO Fill Tuning
    pub TXFILLTUNING: RWRegister<u32>,

    _reserved9: [u32; 4],

    /// Endpoint NAK
    pub ENDPTNAK: RWRegister<u32>,

    /// Endpoint NAK Enable
    pub ENDPTNAKEN: RWRegister<u32>,

    /// Configure Flag Register
    pub CONFIGFLAG: RORegister<u32>,

    /// Port Status & Control
    pub PORTSC1: RWRegister<u32>,

    _reserved10: [u32; 7],

    /// On-The-Go Status & control
    pub OTGSC: RWRegister<u32>,

    /// USB Device Mode
    pub USBMODE: RWRegister<u32>,

    /// Endpoint Setup Status
    pub ENDPTSETUPSTAT: RWRegister<u32>,

    /// Endpoint Prime
    pub ENDPTPRIME: RWRegister<u32>,

    /// Endpoint Flush
    pub ENDPTFLUSH: RWRegister<u32>,

    /// Endpoint Status
    pub ENDPTSTAT: RORegister<u32>,

    /// Endpoint Complete
    pub ENDPTCOMPLETE: RWRegister<u32>,

    /// Endpoint Control0
    pub ENDPTCTRL0: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL1: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL2: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL3: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL4: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL5: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL6: RWRegister<u32>,

    /// Endpoint Control
    pub ENDPTCTRL7: RWRegister<u32>,
}
pub struct ResetValues {
    pub ID: u32,
    pub HWGENERAL: u32,
    pub HWHOST: u32,
    pub HWDEVICE: u32,
    pub HWTXBUF: u32,
    pub HWRXBUF: u32,
    pub GPTIMER0LD: u32,
    pub GPTIMER0CTRL: u32,
    pub GPTIMER1LD: u32,
    pub GPTIMER1CTRL: u32,
    pub SBUSCFG: u32,
    pub CAPLENGTH: u8,
    pub HCIVERSION: u16,
    pub HCSPARAMS: u32,
    pub HCCPARAMS: u32,
    pub DCIVERSION: u16,
    pub DCCPARAMS: u32,
    pub USBCMD: u32,
    pub USBSTS: u32,
    pub USBINTR: u32,
    pub FRINDEX: u32,
    pub DEVICEADDR: u32,
    pub ASYNCLISTADDR: u32,
    pub BURSTSIZE: u32,
    pub TXFILLTUNING: u32,
    pub ENDPTNAK: u32,
    pub ENDPTNAKEN: u32,
    pub CONFIGFLAG: u32,
    pub PORTSC1: u32,
    pub OTGSC: u32,
    pub USBMODE: u32,
    pub ENDPTSETUPSTAT: u32,
    pub ENDPTPRIME: u32,
    pub ENDPTFLUSH: u32,
    pub ENDPTSTAT: u32,
    pub ENDPTCOMPLETE: u32,
    pub ENDPTCTRL0: u32,
    pub ENDPTCTRL1: u32,
    pub ENDPTCTRL2: u32,
    pub ENDPTCTRL3: u32,
    pub ENDPTCTRL4: u32,
    pub ENDPTCTRL5: u32,
    pub ENDPTCTRL6: u32,
    pub ENDPTCTRL7: u32,
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
