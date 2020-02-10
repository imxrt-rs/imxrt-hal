#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHY Register Reference Index
//!
//! Used by: imxrt1051, imxrt1052

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// USB PHY Power-Down Register
pub mod PWD {

    /// Reserved.
    pub mod RSVD0 {
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

    /// 0 = Normal operation
    pub mod TXPWDFS {
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

    /// 0 = Normal operation
    pub mod TXPWDIBIAS {
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

    /// 0 = Normal operation
    pub mod TXPWDV2I {
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

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 0 = Normal operation
    pub mod RXPWDENV {
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

    /// 0 = Normal operation
    pub mod RXPWD1PT1 {
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

    /// 0 = Normal operation
    pub mod RXPWDDIFF {
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

    /// 0 = Normal operation
    pub mod RXPWDRX {
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

    /// Reserved.
    pub mod RSVD2 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY Power-Down Register
pub mod PWD_SET {
    pub use super::PWD::RSVD0;
    pub use super::PWD::RSVD1;
    pub use super::PWD::RSVD2;
    pub use super::PWD::RXPWD1PT1;
    pub use super::PWD::RXPWDDIFF;
    pub use super::PWD::RXPWDENV;
    pub use super::PWD::RXPWDRX;
    pub use super::PWD::TXPWDFS;
    pub use super::PWD::TXPWDIBIAS;
    pub use super::PWD::TXPWDV2I;
}

/// USB PHY Power-Down Register
pub mod PWD_CLR {
    pub use super::PWD::RSVD0;
    pub use super::PWD::RSVD1;
    pub use super::PWD::RSVD2;
    pub use super::PWD::RXPWD1PT1;
    pub use super::PWD::RXPWDDIFF;
    pub use super::PWD::RXPWDENV;
    pub use super::PWD::RXPWDRX;
    pub use super::PWD::TXPWDFS;
    pub use super::PWD::TXPWDIBIAS;
    pub use super::PWD::TXPWDV2I;
}

/// USB PHY Power-Down Register
pub mod PWD_TOG {
    pub use super::PWD::RSVD0;
    pub use super::PWD::RSVD1;
    pub use super::PWD::RSVD2;
    pub use super::PWD::RXPWD1PT1;
    pub use super::PWD::RXPWDDIFF;
    pub use super::PWD::RXPWDENV;
    pub use super::PWD::RXPWDRX;
    pub use super::PWD::TXPWDFS;
    pub use super::PWD::TXPWDIBIAS;
    pub use super::PWD::TXPWDV2I;
}

/// USB PHY Transmitter Control Register
pub mod TX {

    /// Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%
    pub mod D_CAL {
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

    /// Reserved. Note: This bit should remain clear.
    pub mod RSVD0 {
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

    /// Decode to select a 45-Ohm resistance to the USB_DN output pin
    pub mod TXCAL45DN {
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

    /// Reserved. Note: This bit should remain clear.
    pub mod RSVD1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Decode to select a 45-Ohm resistance to the USB_DP output pin
    pub mod TXCAL45DP {
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

    /// Reserved.
    pub mod RSVD2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (6 bits: 0x3f << 20)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Controls the edge-rate of the current sensing transistors used in HS transmit
    pub mod USBPHY_TX_EDGECTRL {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved.
    pub mod RSVD5 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY Transmitter Control Register
pub mod TX_SET {
    pub use super::TX::D_CAL;
    pub use super::TX::RSVD0;
    pub use super::TX::RSVD1;
    pub use super::TX::RSVD2;
    pub use super::TX::RSVD5;
    pub use super::TX::TXCAL45DN;
    pub use super::TX::TXCAL45DP;
    pub use super::TX::USBPHY_TX_EDGECTRL;
}

/// USB PHY Transmitter Control Register
pub mod TX_CLR {
    pub use super::TX::D_CAL;
    pub use super::TX::RSVD0;
    pub use super::TX::RSVD1;
    pub use super::TX::RSVD2;
    pub use super::TX::RSVD5;
    pub use super::TX::TXCAL45DN;
    pub use super::TX::TXCAL45DP;
    pub use super::TX::USBPHY_TX_EDGECTRL;
}

/// USB PHY Transmitter Control Register
pub mod TX_TOG {
    pub use super::TX::D_CAL;
    pub use super::TX::RSVD0;
    pub use super::TX::RSVD1;
    pub use super::TX::RSVD2;
    pub use super::TX::RSVD5;
    pub use super::TX::TXCAL45DN;
    pub use super::TX::TXCAL45DP;
    pub use super::TX::USBPHY_TX_EDGECTRL;
}

/// USB PHY Receiver Control Register
pub mod RX {

    /// The ENVADJ field adjusts the trip point for the envelope detector
    pub mod ENVADJ {
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

    /// Reserved.
    pub mod RSVD0 {
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

    /// The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0
    pub mod DISCONADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (15 bits: 0x7fff << 7)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 0 = Normal operation
    pub mod RXDBYPASS {
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

    /// Reserved.
    pub mod RSVD2 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (9 bits: 0x1ff << 23)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// USB PHY Receiver Control Register
pub mod RX_SET {
    pub use super::RX::DISCONADJ;
    pub use super::RX::ENVADJ;
    pub use super::RX::RSVD0;
    pub use super::RX::RSVD1;
    pub use super::RX::RSVD2;
    pub use super::RX::RXDBYPASS;
}

/// USB PHY Receiver Control Register
pub mod RX_CLR {
    pub use super::RX::DISCONADJ;
    pub use super::RX::ENVADJ;
    pub use super::RX::RSVD0;
    pub use super::RX::RSVD1;
    pub use super::RX::RSVD2;
    pub use super::RX::RXDBYPASS;
}

/// USB PHY Receiver Control Register
pub mod RX_TOG {
    pub use super::RX::DISCONADJ;
    pub use super::RX::ENVADJ;
    pub use super::RX::RSVD0;
    pub use super::RX::RSVD1;
    pub use super::RX::RSVD2;
    pub use super::RX::RXDBYPASS;
}

/// USB PHY General Control Register
pub mod CTRL {

    /// Enable OTG_ID_CHG_IRQ.
    pub mod ENOTG_ID_CHG_IRQ {
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

    /// For host mode, enables high-speed disconnect detector
    pub mod ENHOSTDISCONDETECT {
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

    /// Enables interrupt for detection of disconnection to Device when in high-speed host mode
    pub mod ENIRQHOSTDISCON {
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

    /// Indicates that the device has disconnected in high-speed mode
    pub mod HOSTDISCONDETECT_IRQ {
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

    /// For device mode, enables 200-KOhm pullups for detecting connectivity to the host.
    pub mod ENDEVPLUGINDETECT {
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

    /// For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in
    pub mod DEVPLUGIN_POLARITY {
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

    /// OTG ID change interrupt. Indicates the value of ID pin changed.
    pub mod OTG_ID_CHG_IRQ {
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

    /// Enables circuit to detect resistance of MiniAB ID pin.
    pub mod ENOTGIDDETECT {
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

    /// Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it
    pub mod RESUMEIRQSTICKY {
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

    /// Enables interrupt for detection of a non-J state on the USB line
    pub mod ENIRQRESUMEDETECT {
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

    /// Indicates that the host is sending a wake-up after suspend
    pub mod RESUME_IRQ {
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

    /// Enables interrupt for the detection of connectivity to the USB line.
    pub mod ENIRQDEVPLUGIN {
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

    /// Indicates that the device is connected
    pub mod DEVPLUGIN_IRQ {
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

    /// Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only.
    pub mod DATA_ON_LRADC {
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

    /// Enables UTMI+ Level2. This should be enabled if needs to support LS device
    pub mod ENUTMILEVEL2 {
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

    /// Enables UTMI+ Level3
    pub mod ENUTMILEVEL3 {
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

    /// Enables interrupt for the wakeup events.
    pub mod ENIRQWAKEUP {
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

    /// Indicates that there is a wakeup event
    pub mod WAKEUP_IRQ {
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

    /// Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended
    pub mod ENAUTO_PWRON_PLL {
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

    /// Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended
    pub mod ENAUTOCLR_CLKGATE {
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

    /// Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended
    pub mod ENAUTOCLR_PHY_PWD {
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

    /// Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended
    pub mod ENDPDMCHG_WKUP {
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

    /// Enables the feature to wakeup USB if ID is toggled when USB is suspended.
    pub mod ENIDCHG_WKUP {
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

    /// Enables the feature to wakeup USB if VBUS is toggled when USB is suspended.
    pub mod ENVBUSCHG_WKUP {
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

    /// Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet.
    pub mod FSDLL_RST_EN {
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

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (2 bits: 0b11 << 25)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Almost same as OTGID_STATUS in USBPHYx_STATUS Register
    pub mod OTG_ID_VALUE {
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

    /// Forces the next FS packet that is transmitted to have a EOP with LS timing
    pub mod HOST_FORCE_LS_SE0 {
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

    /// Used by the PHY to indicate a powered-down state
    pub mod UTMI_SUSPENDM {
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

    /// Gate UTMI Clocks
    pub mod CLKGATE {
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

    /// Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers
    pub mod SFTRST {
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

/// USB PHY General Control Register
pub mod CTRL_SET {
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::DATA_ON_LRADC;
    pub use super::CTRL::DEVPLUGIN_IRQ;
    pub use super::CTRL::DEVPLUGIN_POLARITY;
    pub use super::CTRL::ENAUTOCLR_CLKGATE;
    pub use super::CTRL::ENAUTOCLR_PHY_PWD;
    pub use super::CTRL::ENAUTO_PWRON_PLL;
    pub use super::CTRL::ENDEVPLUGINDETECT;
    pub use super::CTRL::ENDPDMCHG_WKUP;
    pub use super::CTRL::ENHOSTDISCONDETECT;
    pub use super::CTRL::ENIDCHG_WKUP;
    pub use super::CTRL::ENIRQDEVPLUGIN;
    pub use super::CTRL::ENIRQHOSTDISCON;
    pub use super::CTRL::ENIRQRESUMEDETECT;
    pub use super::CTRL::ENIRQWAKEUP;
    pub use super::CTRL::ENOTGIDDETECT;
    pub use super::CTRL::ENOTG_ID_CHG_IRQ;
    pub use super::CTRL::ENUTMILEVEL2;
    pub use super::CTRL::ENUTMILEVEL3;
    pub use super::CTRL::ENVBUSCHG_WKUP;
    pub use super::CTRL::FSDLL_RST_EN;
    pub use super::CTRL::HOSTDISCONDETECT_IRQ;
    pub use super::CTRL::HOST_FORCE_LS_SE0;
    pub use super::CTRL::OTG_ID_CHG_IRQ;
    pub use super::CTRL::OTG_ID_VALUE;
    pub use super::CTRL::RESUMEIRQSTICKY;
    pub use super::CTRL::RESUME_IRQ;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::UTMI_SUSPENDM;
    pub use super::CTRL::WAKEUP_IRQ;
}

/// USB PHY General Control Register
pub mod CTRL_CLR {
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::DATA_ON_LRADC;
    pub use super::CTRL::DEVPLUGIN_IRQ;
    pub use super::CTRL::DEVPLUGIN_POLARITY;
    pub use super::CTRL::ENAUTOCLR_CLKGATE;
    pub use super::CTRL::ENAUTOCLR_PHY_PWD;
    pub use super::CTRL::ENAUTO_PWRON_PLL;
    pub use super::CTRL::ENDEVPLUGINDETECT;
    pub use super::CTRL::ENDPDMCHG_WKUP;
    pub use super::CTRL::ENHOSTDISCONDETECT;
    pub use super::CTRL::ENIDCHG_WKUP;
    pub use super::CTRL::ENIRQDEVPLUGIN;
    pub use super::CTRL::ENIRQHOSTDISCON;
    pub use super::CTRL::ENIRQRESUMEDETECT;
    pub use super::CTRL::ENIRQWAKEUP;
    pub use super::CTRL::ENOTGIDDETECT;
    pub use super::CTRL::ENOTG_ID_CHG_IRQ;
    pub use super::CTRL::ENUTMILEVEL2;
    pub use super::CTRL::ENUTMILEVEL3;
    pub use super::CTRL::ENVBUSCHG_WKUP;
    pub use super::CTRL::FSDLL_RST_EN;
    pub use super::CTRL::HOSTDISCONDETECT_IRQ;
    pub use super::CTRL::HOST_FORCE_LS_SE0;
    pub use super::CTRL::OTG_ID_CHG_IRQ;
    pub use super::CTRL::OTG_ID_VALUE;
    pub use super::CTRL::RESUMEIRQSTICKY;
    pub use super::CTRL::RESUME_IRQ;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::UTMI_SUSPENDM;
    pub use super::CTRL::WAKEUP_IRQ;
}

/// USB PHY General Control Register
pub mod CTRL_TOG {
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::DATA_ON_LRADC;
    pub use super::CTRL::DEVPLUGIN_IRQ;
    pub use super::CTRL::DEVPLUGIN_POLARITY;
    pub use super::CTRL::ENAUTOCLR_CLKGATE;
    pub use super::CTRL::ENAUTOCLR_PHY_PWD;
    pub use super::CTRL::ENAUTO_PWRON_PLL;
    pub use super::CTRL::ENDEVPLUGINDETECT;
    pub use super::CTRL::ENDPDMCHG_WKUP;
    pub use super::CTRL::ENHOSTDISCONDETECT;
    pub use super::CTRL::ENIDCHG_WKUP;
    pub use super::CTRL::ENIRQDEVPLUGIN;
    pub use super::CTRL::ENIRQHOSTDISCON;
    pub use super::CTRL::ENIRQRESUMEDETECT;
    pub use super::CTRL::ENIRQWAKEUP;
    pub use super::CTRL::ENOTGIDDETECT;
    pub use super::CTRL::ENOTG_ID_CHG_IRQ;
    pub use super::CTRL::ENUTMILEVEL2;
    pub use super::CTRL::ENUTMILEVEL3;
    pub use super::CTRL::ENVBUSCHG_WKUP;
    pub use super::CTRL::FSDLL_RST_EN;
    pub use super::CTRL::HOSTDISCONDETECT_IRQ;
    pub use super::CTRL::HOST_FORCE_LS_SE0;
    pub use super::CTRL::OTG_ID_CHG_IRQ;
    pub use super::CTRL::OTG_ID_VALUE;
    pub use super::CTRL::RESUMEIRQSTICKY;
    pub use super::CTRL::RESUME_IRQ;
    pub use super::CTRL::RSVD1;
    pub use super::CTRL::SFTRST;
    pub use super::CTRL::UTMI_SUSPENDM;
    pub use super::CTRL::WAKEUP_IRQ;
}

/// USB PHY Status Register
pub mod STATUS {

    /// Reserved.
    pub mod RSVD0 {
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

    /// Indicates that the device has disconnected while in high-speed host mode.
    pub mod HOSTDISCONDETECT_STATUS {
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

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates that the device has been connected on the USB_DP and USB_DM lines.
    pub mod DEVPLUGIN_STATUS {
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

    /// Reserved.
    pub mod RSVD2 {
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

    /// Indicates the results of ID pin on MiniAB plug
    pub mod OTGID_STATUS {
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

    /// Reserved.
    pub mod RSVD3 {
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

    /// Indicates that the host is sending a wake-up after suspend and has triggered an interrupt.
    pub mod RESUME_STATUS {
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

    /// Reserved.
    pub mod RSVD4 {
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

/// USB PHY Debug Register
pub mod DEBUG {

    /// Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value
    pub mod OTGIDPIOLOCK {
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

    /// Use holding registers to assist in timing for external UTMI interface.
    pub mod DEBUG_INTERFACE_HOLD {
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

    /// Set bit 3 to 1 to pull down 15-KOhm on USB_DP line
    pub mod HSTPULLDOWN {
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

    /// Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown
    pub mod ENHSTPULLDOWN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved.
    pub mod RSVD0 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay in between the end of transmit to the beginning of receive
    pub mod TX2RXCOUNT {
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

    /// Set this bit to allow a countdown to transition in between TX and RX.
    pub mod ENTX2RXCOUNT {
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

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Delay in between the detection of squelch to the reset of high-speed RX.
    pub mod SQUELCHRESETCOUNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved.
    pub mod RSVD2 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set bit to allow squelch to reset high-speed receive.
    pub mod ENSQUELCHRESET {
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

    /// Duration of RESET in terms of the number of 480-MHz cycles.
    pub mod SQUELCHRESETLENGTH {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (4 bits: 0b1111 << 25)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1.
    pub mod HOST_RESUME_DEBUG {
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

    /// Gate Test Clocks
    pub mod CLKGATE {
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

    /// Reserved.
    pub mod RSVD3 {
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

/// USB PHY Debug Register
pub mod DEBUG_SET {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::RSVD0;
    pub use super::DEBUG::RSVD1;
    pub use super::DEBUG::RSVD2;
    pub use super::DEBUG::RSVD3;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// USB PHY Debug Register
pub mod DEBUG_CLR {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::RSVD0;
    pub use super::DEBUG::RSVD1;
    pub use super::DEBUG::RSVD2;
    pub use super::DEBUG::RSVD3;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// USB PHY Debug Register
pub mod DEBUG_TOG {
    pub use super::DEBUG::CLKGATE;
    pub use super::DEBUG::DEBUG_INTERFACE_HOLD;
    pub use super::DEBUG::ENHSTPULLDOWN;
    pub use super::DEBUG::ENSQUELCHRESET;
    pub use super::DEBUG::ENTX2RXCOUNT;
    pub use super::DEBUG::HOST_RESUME_DEBUG;
    pub use super::DEBUG::HSTPULLDOWN;
    pub use super::DEBUG::OTGIDPIOLOCK;
    pub use super::DEBUG::RSVD0;
    pub use super::DEBUG::RSVD1;
    pub use super::DEBUG::RSVD2;
    pub use super::DEBUG::RSVD3;
    pub use super::DEBUG::SQUELCHRESETCOUNT;
    pub use super::DEBUG::SQUELCHRESETLENGTH;
    pub use super::DEBUG::TX2RXCOUNT;
}

/// UTMI Debug Status Register 0
pub mod DEBUG0_STATUS {

    /// Running count of the failed pseudo-random generator loopback
    pub mod LOOP_BACK_FAIL_COUNT {
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

    /// Running count of the UTMI_RXERROR.
    pub mod UTMI_RXERROR_FAIL_COUNT {
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

    /// Running count of the squelch reset instead of normal end for HS RX.
    pub mod SQUELCH_COUNT {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (6 bits: 0x3f << 26)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UTMI Debug Status Register 1
pub mod DEBUG1 {

    /// Reserved. Note: This bit should remain clear.
    pub mod RSVD0 {
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

    /// Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%
    pub mod ENTAILADJVD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reserved.
    pub mod RSVD1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (17 bits: 0x1ffff << 15)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_SET {
    pub use super::DEBUG1::ENTAILADJVD;
    pub use super::DEBUG1::RSVD0;
    pub use super::DEBUG1::RSVD1;
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_CLR {
    pub use super::DEBUG1::ENTAILADJVD;
    pub use super::DEBUG1::RSVD0;
    pub use super::DEBUG1::RSVD1;
}

/// UTMI Debug Status Register 1
pub mod DEBUG1_TOG {
    pub use super::DEBUG1::ENTAILADJVD;
    pub use super::DEBUG1::RSVD0;
    pub use super::DEBUG1::RSVD1;
}

/// UTMI RTL Version
pub mod VERSION {

    /// Fixed read-only value reflecting the stepping of the RTL version.
    pub mod STEP {
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

    /// Fixed read-only value reflecting the MINOR field of the RTL version.
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

    /// Fixed read-only value reflecting the MAJOR field of the RTL version.
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
pub struct RegisterBlock {
    /// USB PHY Power-Down Register
    pub PWD: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_SET: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_CLR: RWRegister<u32>,

    /// USB PHY Power-Down Register
    pub PWD_TOG: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_SET: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_CLR: RWRegister<u32>,

    /// USB PHY Transmitter Control Register
    pub TX_TOG: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_SET: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_CLR: RWRegister<u32>,

    /// USB PHY Receiver Control Register
    pub RX_TOG: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_SET: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_CLR: RWRegister<u32>,

    /// USB PHY General Control Register
    pub CTRL_TOG: RWRegister<u32>,

    /// USB PHY Status Register
    pub STATUS: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// USB PHY Debug Register
    pub DEBUG: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_SET: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_CLR: RWRegister<u32>,

    /// USB PHY Debug Register
    pub DEBUG_TOG: RWRegister<u32>,

    /// UTMI Debug Status Register 0
    pub DEBUG0_STATUS: RORegister<u32>,

    _reserved2: [u32; 3],

    /// UTMI Debug Status Register 1
    pub DEBUG1: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_SET: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_CLR: RWRegister<u32>,

    /// UTMI Debug Status Register 1
    pub DEBUG1_TOG: RWRegister<u32>,

    /// UTMI RTL Version
    pub VERSION: RORegister<u32>,
}
pub struct ResetValues {
    pub PWD: u32,
    pub PWD_SET: u32,
    pub PWD_CLR: u32,
    pub PWD_TOG: u32,
    pub TX: u32,
    pub TX_SET: u32,
    pub TX_CLR: u32,
    pub TX_TOG: u32,
    pub RX: u32,
    pub RX_SET: u32,
    pub RX_CLR: u32,
    pub RX_TOG: u32,
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub STATUS: u32,
    pub DEBUG: u32,
    pub DEBUG_SET: u32,
    pub DEBUG_CLR: u32,
    pub DEBUG_TOG: u32,
    pub DEBUG0_STATUS: u32,
    pub DEBUG1: u32,
    pub DEBUG1_SET: u32,
    pub DEBUG1_CLR: u32,
    pub DEBUG1_TOG: u32,
    pub VERSION: u32,
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
