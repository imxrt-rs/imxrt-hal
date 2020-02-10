#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPUART
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Version ID Register
pub mod VERID {

    /// Feature Identification Number
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

            /// 0b0000000000000001: Standard feature set.
            pub const FEATURE_1: u32 = 0b0000000000000001;

            /// 0b0000000000000011: Standard feature set with MODEM/IrDA support.
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

    /// Transmit FIFO Size
    pub mod TXFIFO {
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

    /// Receive FIFO Size
    pub mod RXFIFO {
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

/// LPUART Global Register
pub mod GLOBAL {

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

            /// 0b0: Module is not reset.
            pub const RST_0: u32 = 0b0;

            /// 0b1: Module is reset.
            pub const RST_1: u32 = 0b1;
        }
    }
}

/// LPUART Pin Configuration Register
pub mod PINCFG {

    /// Trigger Select
    pub mod TRGSEL {
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

            /// 0b00: Input trigger is disabled.
            pub const TRGSEL_0: u32 = 0b00;

            /// 0b01: Input trigger is used instead of RXD pin input.
            pub const TRGSEL_1: u32 = 0b01;

            /// 0b10: Input trigger is used instead of CTS_B pin input.
            pub const TRGSEL_2: u32 = 0b10;

            /// 0b11: Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger.
            pub const TRGSEL_3: u32 = 0b11;
        }
    }
}

/// LPUART Baud Rate Register
pub mod BAUD {

    /// Baud Rate Modulo Divisor.
    pub mod SBR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (13 bits: 0x1fff << 0)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Stop Bit Number Select
    pub mod SBNS {
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

            /// 0b0: One stop bit.
            pub const SBNS_0: u32 = 0b0;

            /// 0b1: Two stop bits.
            pub const SBNS_1: u32 = 0b1;
        }
    }

    /// RX Input Active Edge Interrupt Enable
    pub mod RXEDGIE {
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

            /// 0b0: Hardware interrupts from STAT\[RXEDGIF\] are disabled.
            pub const RXEDGIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt is requested when STAT\[RXEDGIF\] flag is 1.
            pub const RXEDGIE_1: u32 = 0b1;
        }
    }

    /// LIN Break Detect Interrupt Enable
    pub mod LBKDIE {
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

            /// 0b0: Hardware interrupts from STAT\[LBKDIF\] flag are disabled (use polling).
            pub const LBKDIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when STAT\[LBKDIF\] flag is 1.
            pub const LBKDIE_1: u32 = 0b1;
        }
    }

    /// Resynchronization Disable
    pub mod RESYNCDIS {
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

            /// 0b0: Resynchronization during received data word is supported
            pub const RESYNCDIS_0: u32 = 0b0;

            /// 0b1: Resynchronization during received data word is disabled
            pub const RESYNCDIS_1: u32 = 0b1;
        }
    }

    /// Both Edge Sampling
    pub mod BOTHEDGE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receiver samples input data using the rising edge of the baud rate clock.
            pub const BOTHEDGE_0: u32 = 0b0;

            /// 0b1: Receiver samples input data using the rising and falling edge of the baud rate clock.
            pub const BOTHEDGE_1: u32 = 0b1;
        }
    }

    /// Match Configuration
    pub mod MATCFG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Address Match Wakeup
            pub const MATCFG_0: u32 = 0b00;

            /// 0b01: Idle Match Wakeup
            pub const MATCFG_1: u32 = 0b01;

            /// 0b10: Match On and Match Off
            pub const MATCFG_2: u32 = 0b10;

            /// 0b11: Enables RWU on Data Match and Match On/Off for transmitter CTS input
            pub const MATCFG_3: u32 = 0b11;
        }
    }

    /// Receiver Idle DMA Enable
    pub mod RIDMAE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA request disabled.
            pub const RIDMAE_0: u32 = 0b0;

            /// 0b1: DMA request enabled.
            pub const RIDMAE_1: u32 = 0b1;
        }
    }

    /// Receiver Full DMA Enable
    pub mod RDMAE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA request disabled.
            pub const RDMAE_0: u32 = 0b0;

            /// 0b1: DMA request enabled.
            pub const RDMAE_1: u32 = 0b1;
        }
    }

    /// Transmitter DMA Enable
    pub mod TDMAE {
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

            /// 0b0: DMA request disabled.
            pub const TDMAE_0: u32 = 0b0;

            /// 0b1: DMA request enabled.
            pub const TDMAE_1: u32 = 0b1;
        }
    }

    /// Oversampling Ratio
    pub mod OSR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Writing 0 to this field will result in an oversampling ratio of 16
            pub const OSR_0: u32 = 0b00000;

            /// 0b00011: Oversampling ratio of 4, requires BOTHEDGE to be set.
            pub const OSR_3: u32 = 0b00011;

            /// 0b00100: Oversampling ratio of 5, requires BOTHEDGE to be set.
            pub const OSR_4: u32 = 0b00100;

            /// 0b00101: Oversampling ratio of 6, requires BOTHEDGE to be set.
            pub const OSR_5: u32 = 0b00101;

            /// 0b00110: Oversampling ratio of 7, requires BOTHEDGE to be set.
            pub const OSR_6: u32 = 0b00110;

            /// 0b00111: Oversampling ratio of 8.
            pub const OSR_7: u32 = 0b00111;

            /// 0b01000: Oversampling ratio of 9.
            pub const OSR_8: u32 = 0b01000;

            /// 0b01001: Oversampling ratio of 10.
            pub const OSR_9: u32 = 0b01001;

            /// 0b01010: Oversampling ratio of 11.
            pub const OSR_10: u32 = 0b01010;

            /// 0b01011: Oversampling ratio of 12.
            pub const OSR_11: u32 = 0b01011;

            /// 0b01100: Oversampling ratio of 13.
            pub const OSR_12: u32 = 0b01100;

            /// 0b01101: Oversampling ratio of 14.
            pub const OSR_13: u32 = 0b01101;

            /// 0b01110: Oversampling ratio of 15.
            pub const OSR_14: u32 = 0b01110;

            /// 0b01111: Oversampling ratio of 16.
            pub const OSR_15: u32 = 0b01111;

            /// 0b10000: Oversampling ratio of 17.
            pub const OSR_16: u32 = 0b10000;

            /// 0b10001: Oversampling ratio of 18.
            pub const OSR_17: u32 = 0b10001;

            /// 0b10010: Oversampling ratio of 19.
            pub const OSR_18: u32 = 0b10010;

            /// 0b10011: Oversampling ratio of 20.
            pub const OSR_19: u32 = 0b10011;

            /// 0b10100: Oversampling ratio of 21.
            pub const OSR_20: u32 = 0b10100;

            /// 0b10101: Oversampling ratio of 22.
            pub const OSR_21: u32 = 0b10101;

            /// 0b10110: Oversampling ratio of 23.
            pub const OSR_22: u32 = 0b10110;

            /// 0b10111: Oversampling ratio of 24.
            pub const OSR_23: u32 = 0b10111;

            /// 0b11000: Oversampling ratio of 25.
            pub const OSR_24: u32 = 0b11000;

            /// 0b11001: Oversampling ratio of 26.
            pub const OSR_25: u32 = 0b11001;

            /// 0b11010: Oversampling ratio of 27.
            pub const OSR_26: u32 = 0b11010;

            /// 0b11011: Oversampling ratio of 28.
            pub const OSR_27: u32 = 0b11011;

            /// 0b11100: Oversampling ratio of 29.
            pub const OSR_28: u32 = 0b11100;

            /// 0b11101: Oversampling ratio of 30.
            pub const OSR_29: u32 = 0b11101;

            /// 0b11110: Oversampling ratio of 31.
            pub const OSR_30: u32 = 0b11110;

            /// 0b11111: Oversampling ratio of 32.
            pub const OSR_31: u32 = 0b11111;
        }
    }

    /// 10-bit Mode select
    pub mod M10 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receiver and transmitter use 7-bit to 9-bit data characters.
            pub const M10_0: u32 = 0b0;

            /// 0b1: Receiver and transmitter use 10-bit data characters.
            pub const M10_1: u32 = 0b1;
        }
    }

    /// Match Address Mode Enable 2
    pub mod MAEN2 {
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

            /// 0b0: Normal operation.
            pub const MAEN2_0: u32 = 0b0;

            /// 0b1: Enables automatic address matching or data matching mode for MATCH\[MA2\].
            pub const MAEN2_1: u32 = 0b1;
        }
    }

    /// Match Address Mode Enable 1
    pub mod MAEN1 {
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

            /// 0b0: Normal operation.
            pub const MAEN1_0: u32 = 0b0;

            /// 0b1: Enables automatic address matching or data matching mode for MATCH\[MA1\].
            pub const MAEN1_1: u32 = 0b1;
        }
    }
}

/// LPUART Status Register
pub mod STAT {

    /// Match 2 Flag
    pub mod MA2F {
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

            /// 0b0: Received data is not equal to MA2
            pub const MA2F_0: u32 = 0b0;

            /// 0b1: Received data is equal to MA2
            pub const MA2F_1: u32 = 0b1;
        }
    }

    /// Match 1 Flag
    pub mod MA1F {
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

            /// 0b0: Received data is not equal to MA1
            pub const MA1F_0: u32 = 0b0;

            /// 0b1: Received data is equal to MA1
            pub const MA1F_1: u32 = 0b1;
        }
    }

    /// Parity Error Flag
    pub mod PF {
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

            /// 0b0: No parity error.
            pub const PF_0: u32 = 0b0;

            /// 0b1: Parity error.
            pub const PF_1: u32 = 0b1;
        }
    }

    /// Framing Error Flag
    pub mod FE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No framing error detected. This does not guarantee the framing is correct.
            pub const FE_0: u32 = 0b0;

            /// 0b1: Framing error.
            pub const FE_1: u32 = 0b1;
        }
    }

    /// Noise Flag
    pub mod NF {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No noise detected.
            pub const NF_0: u32 = 0b0;

            /// 0b1: Noise detected in the received character in the DATA register.
            pub const NF_1: u32 = 0b1;
        }
    }

    /// Receiver Overrun Flag
    pub mod OR {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No overrun.
            pub const OR_0: u32 = 0b0;

            /// 0b1: Receive overrun (new LPUART data lost).
            pub const OR_1: u32 = 0b1;
        }
    }

    /// Idle Line Flag
    pub mod IDLE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No idle line detected.
            pub const IDLE_0: u32 = 0b0;

            /// 0b1: Idle line was detected.
            pub const IDLE_1: u32 = 0b1;
        }
    }

    /// Receive Data Register Full Flag
    pub mod RDRF {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive data buffer empty.
            pub const RDRF_0: u32 = 0b0;

            /// 0b1: Receive data buffer full.
            pub const RDRF_1: u32 = 0b1;
        }
    }

    /// Transmission Complete Flag
    pub mod TC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitter active (sending data, a preamble, or a break).
            pub const TC_0: u32 = 0b0;

            /// 0b1: Transmitter idle (transmission activity complete).
            pub const TC_1: u32 = 0b1;
        }
    }

    /// Transmit Data Register Empty Flag
    pub mod TDRE {
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

            /// 0b0: Transmit data buffer full.
            pub const TDRE_0: u32 = 0b0;

            /// 0b1: Transmit data buffer empty.
            pub const TDRE_1: u32 = 0b1;
        }
    }

    /// Receiver Active Flag
    pub mod RAF {
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

            /// 0b0: LPUART receiver idle waiting for a start bit.
            pub const RAF_0: u32 = 0b0;

            /// 0b1: LPUART receiver active (RXD input not idle).
            pub const RAF_1: u32 = 0b1;
        }
    }

    /// LIN Break Detection Enable
    pub mod LBKDE {
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

            /// 0b0: LIN break detect is disabled, normal break character can be detected.
            pub const LBKDE_0: u32 = 0b0;

            /// 0b1: LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1).
            pub const LBKDE_1: u32 = 0b1;
        }
    }

    /// Break Character Generation Length
    pub mod BRK13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Break character is transmitted with length of 9 to 13 bit times.
            pub const BRK13_0: u32 = 0b0;

            /// 0b1: Break character is transmitted with length of 12 to 15 bit times.
            pub const BRK13_1: u32 = 0b1;
        }
    }

    /// Receive Wake Up Idle Detect
    pub mod RWUID {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match.
            pub const RWUID_0: u32 = 0b0;

            /// 0b1: During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match.
            pub const RWUID_1: u32 = 0b1;
        }
    }

    /// Receive Data Inversion
    pub mod RXINV {
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

            /// 0b0: Receive data not inverted.
            pub const RXINV_0: u32 = 0b0;

            /// 0b1: Receive data inverted.
            pub const RXINV_1: u32 = 0b1;
        }
    }

    /// MSB First
    pub mod MSBF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0.
            pub const MSBF_0: u32 = 0b0;

            /// 0b1: MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\[M\], CTRL\[PE\] and BAUD\[M10\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\[M\] and CTRL\[PE\].
            pub const MSBF_1: u32 = 0b1;
        }
    }

    /// RXD Pin Active Edge Interrupt Flag
    pub mod RXEDGIF {
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

            /// 0b0: No active edge on the receive pin has occurred.
            pub const RXEDGIF_0: u32 = 0b0;

            /// 0b1: An active edge on the receive pin has occurred.
            pub const RXEDGIF_1: u32 = 0b1;
        }
    }

    /// LIN Break Detect Interrupt Flag
    pub mod LBKDIF {
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

            /// 0b0: No LIN break character has been detected.
            pub const LBKDIF_0: u32 = 0b0;

            /// 0b1: LIN break character has been detected.
            pub const LBKDIF_1: u32 = 0b1;
        }
    }
}

/// LPUART Control Register
pub mod CTRL {

    /// Parity Type
    pub mod PT {
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

            /// 0b0: Even parity.
            pub const PT_0: u32 = 0b0;

            /// 0b1: Odd parity.
            pub const PT_1: u32 = 0b1;
        }
    }

    /// Parity Enable
    pub mod PE {
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

            /// 0b0: No hardware parity generation or checking.
            pub const PE_0: u32 = 0b0;

            /// 0b1: Parity enabled.
            pub const PE_1: u32 = 0b1;
        }
    }

    /// Idle Line Type Select
    pub mod ILT {
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

            /// 0b0: Idle character bit count starts after start bit.
            pub const ILT_0: u32 = 0b0;

            /// 0b1: Idle character bit count starts after stop bit.
            pub const ILT_1: u32 = 0b1;
        }
    }

    /// Receiver Wakeup Method Select
    pub mod WAKE {
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

            /// 0b0: Configures RWU for idle-line wakeup.
            pub const WAKE_0: u32 = 0b0;

            /// 0b1: Configures RWU with address-mark wakeup.
            pub const WAKE_1: u32 = 0b1;
        }
    }

    /// 9-Bit or 8-Bit Mode Select
    pub mod M {
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

            /// 0b0: Receiver and transmitter use 8-bit data characters.
            pub const M_0: u32 = 0b0;

            /// 0b1: Receiver and transmitter use 9-bit data characters.
            pub const M_1: u32 = 0b1;
        }
    }

    /// Receiver Source Select
    pub mod RSRC {
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

            /// 0b0: Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin.
            pub const RSRC_0: u32 = 0b0;

            /// 0b1: Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input.
            pub const RSRC_1: u32 = 0b1;
        }
    }

    /// Doze Enable
    pub mod DOZEEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPUART is enabled in Doze mode.
            pub const DOZEEN_0: u32 = 0b0;

            /// 0b1: LPUART is disabled in Doze mode.
            pub const DOZEEN_1: u32 = 0b1;
        }
    }

    /// Loop Mode Select
    pub mod LOOPS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation - RXD and TXD use separate pins.
            pub const LOOPS_0: u32 = 0b0;

            /// 0b1: Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit).
            pub const LOOPS_1: u32 = 0b1;
        }
    }

    /// Idle Configuration
    pub mod IDLECFG {
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

            /// 0b000: 1 idle character
            pub const IDLECFG_0: u32 = 0b000;

            /// 0b001: 2 idle characters
            pub const IDLECFG_1: u32 = 0b001;

            /// 0b010: 4 idle characters
            pub const IDLECFG_2: u32 = 0b010;

            /// 0b011: 8 idle characters
            pub const IDLECFG_3: u32 = 0b011;

            /// 0b100: 16 idle characters
            pub const IDLECFG_4: u32 = 0b100;

            /// 0b101: 32 idle characters
            pub const IDLECFG_5: u32 = 0b101;

            /// 0b110: 64 idle characters
            pub const IDLECFG_6: u32 = 0b110;

            /// 0b111: 128 idle characters
            pub const IDLECFG_7: u32 = 0b111;
        }
    }

    /// 7-Bit Mode Select
    pub mod M7 {
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

            /// 0b0: Receiver and transmitter use 8-bit to 10-bit data characters.
            pub const M7_0: u32 = 0b0;

            /// 0b1: Receiver and transmitter use 7-bit data characters.
            pub const M7_1: u32 = 0b1;
        }
    }

    /// Match 2 Interrupt Enable
    pub mod MA2IE {
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

            /// 0b0: MA2F interrupt disabled
            pub const MA2IE_0: u32 = 0b0;

            /// 0b1: MA2F interrupt enabled
            pub const MA2IE_1: u32 = 0b1;
        }
    }

    /// Match 1 Interrupt Enable
    pub mod MA1IE {
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

            /// 0b0: MA1F interrupt disabled
            pub const MA1IE_0: u32 = 0b0;

            /// 0b1: MA1F interrupt enabled
            pub const MA1IE_1: u32 = 0b1;
        }
    }

    /// Send Break
    pub mod SBK {
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

            /// 0b0: Normal transmitter operation.
            pub const SBK_0: u32 = 0b0;

            /// 0b1: Queue break character(s) to be sent.
            pub const SBK_1: u32 = 0b1;
        }
    }

    /// Receiver Wakeup Control
    pub mod RWU {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal receiver operation.
            pub const RWU_0: u32 = 0b0;

            /// 0b1: LPUART receiver in standby waiting for wakeup condition.
            pub const RWU_1: u32 = 0b1;
        }
    }

    /// Receiver Enable
    pub mod RE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receiver disabled.
            pub const RE_0: u32 = 0b0;

            /// 0b1: Receiver enabled.
            pub const RE_1: u32 = 0b1;
        }
    }

    /// Transmitter Enable
    pub mod TE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmitter disabled.
            pub const TE_0: u32 = 0b0;

            /// 0b1: Transmitter enabled.
            pub const TE_1: u32 = 0b1;
        }
    }

    /// Idle Line Interrupt Enable
    pub mod ILIE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware interrupts from IDLE disabled; use polling.
            pub const ILIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when IDLE flag is 1.
            pub const ILIE_1: u32 = 0b1;
        }
    }

    /// Receiver Interrupt Enable
    pub mod RIE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware interrupts from RDRF disabled; use polling.
            pub const RIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when RDRF flag is 1.
            pub const RIE_1: u32 = 0b1;
        }
    }

    /// Transmission Complete Interrupt Enable for
    pub mod TCIE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware interrupts from TC disabled; use polling.
            pub const TCIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when TC flag is 1.
            pub const TCIE_1: u32 = 0b1;
        }
    }

    /// Transmit Interrupt Enable
    pub mod TIE {
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

            /// 0b0: Hardware interrupts from TDRE disabled; use polling.
            pub const TIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when TDRE flag is 1.
            pub const TIE_1: u32 = 0b1;
        }
    }

    /// Parity Error Interrupt Enable
    pub mod PEIE {
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

            /// 0b0: PF interrupts disabled; use polling).
            pub const PEIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when PF is set.
            pub const PEIE_1: u32 = 0b1;
        }
    }

    /// Framing Error Interrupt Enable
    pub mod FEIE {
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

            /// 0b0: FE interrupts disabled; use polling.
            pub const FEIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when FE is set.
            pub const FEIE_1: u32 = 0b1;
        }
    }

    /// Noise Error Interrupt Enable
    pub mod NEIE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: NF interrupts disabled; use polling.
            pub const NEIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when NF is set.
            pub const NEIE_1: u32 = 0b1;
        }
    }

    /// Overrun Interrupt Enable
    pub mod ORIE {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OR interrupts disabled; use polling.
            pub const ORIE_0: u32 = 0b0;

            /// 0b1: Hardware interrupt requested when OR is set.
            pub const ORIE_1: u32 = 0b1;
        }
    }

    /// Transmit Data Inversion
    pub mod TXINV {
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

            /// 0b0: Transmit data not inverted.
            pub const TXINV_0: u32 = 0b0;

            /// 0b1: Transmit data inverted.
            pub const TXINV_1: u32 = 0b1;
        }
    }

    /// TXD Pin Direction in Single-Wire Mode
    pub mod TXDIR {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TXD pin is an input in single-wire mode.
            pub const TXDIR_0: u32 = 0b0;

            /// 0b1: TXD pin is an output in single-wire mode.
            pub const TXDIR_1: u32 = 0b1;
        }
    }

    /// Receive Bit 9 / Transmit Bit 8
    pub mod R9T8 {
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

    /// Receive Bit 8 / Transmit Bit 9
    pub mod R8T9 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPUART Data Register
pub mod DATA {

    /// R0T0
    pub mod R0T0 {
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

    /// R1T1
    pub mod R1T1 {
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

    /// R2T2
    pub mod R2T2 {
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

    /// R3T3
    pub mod R3T3 {
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

    /// R4T4
    pub mod R4T4 {
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

    /// R5T5
    pub mod R5T5 {
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

    /// R6T6
    pub mod R6T6 {
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

    /// R7T7
    pub mod R7T7 {
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

    /// R8T8
    pub mod R8T8 {
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

    /// R9T9
    pub mod R9T9 {
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

    /// Idle Line
    pub mod IDLINE {
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

            /// 0b0: Receiver was not idle before receiving this character.
            pub const IDLINE_0: u32 = 0b0;

            /// 0b1: Receiver was idle before receiving this character.
            pub const IDLINE_1: u32 = 0b1;
        }
    }

    /// Receive Buffer Empty
    pub mod RXEMPT {
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

            /// 0b0: Receive buffer contains valid data.
            pub const RXEMPT_0: u32 = 0b0;

            /// 0b1: Receive buffer is empty, data returned on read is not valid.
            pub const RXEMPT_1: u32 = 0b1;
        }
    }

    /// Frame Error / Transmit Special Character
    pub mod FRETSC {
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

            /// 0b0: The dataword was received without a frame error on read, or transmit a normal character on write.
            pub const FRETSC_0: u32 = 0b0;

            /// 0b1: The dataword was received with a frame error, or transmit an idle or break character on transmit.
            pub const FRETSC_1: u32 = 0b1;
        }
    }

    /// PARITYE
    pub mod PARITYE {
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

            /// 0b0: The dataword was received without a parity error.
            pub const PARITYE_0: u32 = 0b0;

            /// 0b1: The dataword was received with a parity error.
            pub const PARITYE_1: u32 = 0b1;
        }
    }

    /// NOISY
    pub mod NOISY {
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

            /// 0b0: The dataword was received without noise.
            pub const NOISY_0: u32 = 0b0;

            /// 0b1: The data was received with noise.
            pub const NOISY_1: u32 = 0b1;
        }
    }
}

/// LPUART Match Address Register
pub mod MATCH {

    /// Match Address 1
    pub mod MA1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Match Address 2
    pub mod MA2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPUART Modem IrDA Register
pub mod MODIR {

    /// Transmitter clear-to-send enable
    pub mod TXCTSE {
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

            /// 0b0: CTS has no effect on the transmitter.
            pub const TXCTSE_0: u32 = 0b0;

            /// 0b1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission.
            pub const TXCTSE_1: u32 = 0b1;
        }
    }

    /// Transmitter request-to-send enable
    pub mod TXRTSE {
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

            /// 0b0: The transmitter has no effect on RTS.
            pub const TXRTSE_0: u32 = 0b0;

            /// 0b1: When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit.
            pub const TXRTSE_1: u32 = 0b1;
        }
    }

    /// Transmitter request-to-send polarity
    pub mod TXRTSPOL {
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

            /// 0b0: Transmitter RTS is active low.
            pub const TXRTSPOL_0: u32 = 0b0;

            /// 0b1: Transmitter RTS is active high.
            pub const TXRTSPOL_1: u32 = 0b1;
        }
    }

    /// Receiver request-to-send enable
    pub mod RXRTSE {
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

            /// 0b0: The receiver has no effect on RTS.
            pub const RXRTSE_0: u32 = 0b0;

            /// 0b1: RTS is deasserted if the receiver data register is full or a start bit has been detected that would cause the receiver data register to become full. RTS is asserted if the receiver data register is not full and has not detected a start bit that would cause the receiver data register to become full.
            pub const RXRTSE_1: u32 = 0b1;
        }
    }

    /// Transmit CTS Configuration
    pub mod TXCTSC {
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

            /// 0b0: CTS input is sampled at the start of each character.
            pub const TXCTSC_0: u32 = 0b0;

            /// 0b1: CTS input is sampled when the transmitter is idle.
            pub const TXCTSC_1: u32 = 0b1;
        }
    }

    /// Transmit CTS Source
    pub mod TXCTSSRC {
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

            /// 0b0: CTS input is the CTS_B pin.
            pub const TXCTSSRC_0: u32 = 0b0;

            /// 0b1: CTS input is the inverted Receiver Match result.
            pub const TXCTSSRC_1: u32 = 0b1;
        }
    }

    /// Receive RTS Configuration
    pub mod RTSWATER {
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

    /// Transmitter narrow pulse
    pub mod TNP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 1/OSR.
            pub const TNP_0: u32 = 0b00;

            /// 0b01: 2/OSR.
            pub const TNP_1: u32 = 0b01;

            /// 0b10: 3/OSR.
            pub const TNP_2: u32 = 0b10;

            /// 0b11: 4/OSR.
            pub const TNP_3: u32 = 0b11;
        }
    }

    /// Infrared enable
    pub mod IREN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: IR disabled.
            pub const IREN_0: u32 = 0b0;

            /// 0b1: IR enabled.
            pub const IREN_1: u32 = 0b1;
        }
    }
}

/// LPUART FIFO Register
pub mod FIFO {

    /// Receive FIFO Buffer Depth
    pub mod RXFIFOSIZE {
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

            /// 0b000: Receive FIFO/Buffer depth = 1 dataword.
            pub const RXFIFOSIZE_0: u32 = 0b000;

            /// 0b001: Receive FIFO/Buffer depth = 4 datawords.
            pub const RXFIFOSIZE_1: u32 = 0b001;

            /// 0b010: Receive FIFO/Buffer depth = 8 datawords.
            pub const RXFIFOSIZE_2: u32 = 0b010;

            /// 0b011: Receive FIFO/Buffer depth = 16 datawords.
            pub const RXFIFOSIZE_3: u32 = 0b011;

            /// 0b100: Receive FIFO/Buffer depth = 32 datawords.
            pub const RXFIFOSIZE_4: u32 = 0b100;

            /// 0b101: Receive FIFO/Buffer depth = 64 datawords.
            pub const RXFIFOSIZE_5: u32 = 0b101;

            /// 0b110: Receive FIFO/Buffer depth = 128 datawords.
            pub const RXFIFOSIZE_6: u32 = 0b110;

            /// 0b111: Receive FIFO/Buffer depth = 256 datawords.
            pub const RXFIFOSIZE_7: u32 = 0b111;
        }
    }

    /// Receive FIFO Enable
    pub mod RXFE {
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

            /// 0b0: Receive FIFO is not enabled. Buffer is depth 1.
            pub const RXFE_0: u32 = 0b0;

            /// 0b1: Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE.
            pub const RXFE_1: u32 = 0b1;
        }
    }

    /// Transmit FIFO Buffer Depth
    pub mod TXFIFOSIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Transmit FIFO/Buffer depth = 1 dataword.
            pub const TXFIFOSIZE_0: u32 = 0b000;

            /// 0b001: Transmit FIFO/Buffer depth = 4 datawords.
            pub const TXFIFOSIZE_1: u32 = 0b001;

            /// 0b010: Transmit FIFO/Buffer depth = 8 datawords.
            pub const TXFIFOSIZE_2: u32 = 0b010;

            /// 0b011: Transmit FIFO/Buffer depth = 16 datawords.
            pub const TXFIFOSIZE_3: u32 = 0b011;

            /// 0b100: Transmit FIFO/Buffer depth = 32 datawords.
            pub const TXFIFOSIZE_4: u32 = 0b100;

            /// 0b101: Transmit FIFO/Buffer depth = 64 datawords.
            pub const TXFIFOSIZE_5: u32 = 0b101;

            /// 0b110: Transmit FIFO/Buffer depth = 128 datawords.
            pub const TXFIFOSIZE_6: u32 = 0b110;

            /// 0b111: Transmit FIFO/Buffer depth = 256 datawords
            pub const TXFIFOSIZE_7: u32 = 0b111;
        }
    }

    /// Transmit FIFO Enable
    pub mod TXFE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit FIFO is not enabled. Buffer is depth 1.
            pub const TXFE_0: u32 = 0b0;

            /// 0b1: Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE.
            pub const TXFE_1: u32 = 0b1;
        }
    }

    /// Receive FIFO Underflow Interrupt Enable
    pub mod RXUFE {
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

            /// 0b0: RXUF flag does not generate an interrupt to the host.
            pub const RXUFE_0: u32 = 0b0;

            /// 0b1: RXUF flag generates an interrupt to the host.
            pub const RXUFE_1: u32 = 0b1;
        }
    }

    /// Transmit FIFO Overflow Interrupt Enable
    pub mod TXOFE {
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

            /// 0b0: TXOF flag does not generate an interrupt to the host.
            pub const TXOFE_0: u32 = 0b0;

            /// 0b1: TXOF flag generates an interrupt to the host.
            pub const TXOFE_1: u32 = 0b1;
        }
    }

    /// Receiver Idle Empty Enable
    pub mod RXIDEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (3 bits: 0b111 << 10)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Disable RDRF assertion due to partially filled FIFO when receiver is idle.
            pub const RXIDEN_0: u32 = 0b000;

            /// 0b001: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character.
            pub const RXIDEN_1: u32 = 0b001;

            /// 0b010: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters.
            pub const RXIDEN_2: u32 = 0b010;

            /// 0b011: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters.
            pub const RXIDEN_3: u32 = 0b011;

            /// 0b100: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters.
            pub const RXIDEN_4: u32 = 0b100;

            /// 0b101: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters.
            pub const RXIDEN_5: u32 = 0b101;

            /// 0b110: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters.
            pub const RXIDEN_6: u32 = 0b110;

            /// 0b111: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters.
            pub const RXIDEN_7: u32 = 0b111;
        }
    }

    /// Receive FIFO/Buffer Flush
    pub mod RXFLUSH {
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

            /// 0b0: No flush operation occurs.
            pub const RXFLUSH_0: u32 = 0b0;

            /// 0b1: All data in the receive FIFO/buffer is cleared out.
            pub const RXFLUSH_1: u32 = 0b1;
        }
    }

    /// Transmit FIFO/Buffer Flush
    pub mod TXFLUSH {
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

            /// 0b0: No flush operation occurs.
            pub const TXFLUSH_0: u32 = 0b0;

            /// 0b1: All data in the transmit FIFO/Buffer is cleared out.
            pub const TXFLUSH_1: u32 = 0b1;
        }
    }

    /// Receiver Buffer Underflow Flag
    pub mod RXUF {
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

            /// 0b0: No receive buffer underflow has occurred since the last time the flag was cleared.
            pub const RXUF_0: u32 = 0b0;

            /// 0b1: At least one receive buffer underflow has occurred since the last time the flag was cleared.
            pub const RXUF_1: u32 = 0b1;
        }
    }

    /// Transmitter Buffer Overflow Flag
    pub mod TXOF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No transmit buffer overflow has occurred since the last time the flag was cleared.
            pub const TXOF_0: u32 = 0b0;

            /// 0b1: At least one transmit buffer overflow has occurred since the last time the flag was cleared.
            pub const TXOF_1: u32 = 0b1;
        }
    }

    /// Receive Buffer/FIFO Empty
    pub mod RXEMPT {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive buffer is not empty.
            pub const RXEMPT_0: u32 = 0b0;

            /// 0b1: Receive buffer is empty.
            pub const RXEMPT_1: u32 = 0b1;
        }
    }

    /// Transmit Buffer/FIFO Empty
    pub mod TXEMPT {
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

            /// 0b0: Transmit buffer is not empty.
            pub const TXEMPT_0: u32 = 0b0;

            /// 0b1: Transmit buffer is empty.
            pub const TXEMPT_1: u32 = 0b1;
        }
    }
}

/// LPUART Watermark Register
pub mod WATER {

    /// Transmit Watermark
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

    /// Transmit Counter
    pub mod TXCOUNT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive Watermark
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

    /// Receive Counter
    pub mod RXCOUNT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Version ID Register
    pub VERID: RORegister<u32>,

    /// Parameter Register
    pub PARAM: RORegister<u32>,

    /// LPUART Global Register
    pub GLOBAL: RWRegister<u32>,

    /// LPUART Pin Configuration Register
    pub PINCFG: RWRegister<u32>,

    /// LPUART Baud Rate Register
    pub BAUD: RWRegister<u32>,

    /// LPUART Status Register
    pub STAT: RWRegister<u32>,

    /// LPUART Control Register
    pub CTRL: RWRegister<u32>,

    /// LPUART Data Register
    pub DATA: RWRegister<u32>,

    /// LPUART Match Address Register
    pub MATCH: RWRegister<u32>,

    /// LPUART Modem IrDA Register
    pub MODIR: RWRegister<u32>,

    /// LPUART FIFO Register
    pub FIFO: RWRegister<u32>,

    /// LPUART Watermark Register
    pub WATER: RWRegister<u32>,
}
pub struct ResetValues {
    pub VERID: u32,
    pub PARAM: u32,
    pub GLOBAL: u32,
    pub PINCFG: u32,
    pub BAUD: u32,
    pub STAT: u32,
    pub CTRL: u32,
    pub DATA: u32,
    pub MATCH: u32,
    pub MODIR: u32,
    pub FIFO: u32,
    pub WATER: u32,
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
