//! Include when there's just one USB peripheral.

use crate::{
    ral::{usb, usb_analog, usbnc, usbphy},
    usbd::UsbInstances,
};

impl<'a> UsbInstances<'a, 0> {
    /// Take the peripherals that require ownership for safe `imxrt-usbd`
    /// usage.
    ///
    /// Returns `None` if any of the owned instances are already taken.
    pub fn take(usb_analog: &'a mut usb_analog::USB_ANALOG) -> Option<Self> {
        Some(Self {
            usb: usb::USB::take()?,
            usbphy: usbphy::USBPHY::take()?,
            usbnc: usbnc::USBNC::take()?,
            usb_analog,
        })
    }
}
