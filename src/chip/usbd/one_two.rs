//! Include when there's two USB peripherals.

use crate::{
    ral::{usb, usb_analog, usbnc, usbphy},
    usbd::UsbInstances,
};

impl<'a> UsbInstances<'a, 1> {
    /// Take the peripherals that require ownership for safe `imxrt-usbd`
    /// usage.
    ///
    /// Returns `None` if any of the owned instances are already taken.
    pub fn take(usb_analog: &'a mut usb_analog::USB_ANALOG) -> Option<Self> {
        Some(Self {
            usb: usb::USB1::take()?,
            usbphy: usbphy::USBPHY1::take()?,
            usbnc: usbnc::USBNC1::take()?,
            usb_analog,
        })
    }
}

impl<'a> UsbInstances<'a, 2> {
    /// Take the peripherals that require ownership for safe `imxrt-usbd`
    /// usage.
    ///
    /// Returns `None` if any of the owned instances are already taken.
    pub fn take(usb_analog: &'a mut usb_analog::USB_ANALOG) -> Option<Self> {
        Some(Self {
            usb: usb::USB2::take()?,
            usbphy: usbphy::USBPHY2::take()?,
            usbnc: usbnc::USBNC2::take()?,
            usb_analog,
        })
    }
}
