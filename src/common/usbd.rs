//! Common USB device APIs.

pub use imxrt_usbd::{gpt, BusAdapter, Speed};

use crate::ral::{usb, usb_analog::USB_ANALOG, usbnc, usbphy};

/// USB peripheral instances.
///
/// This type safely implements the `imxrt_usbd::Peripherals` trait,
/// allowing you to easily create a [`BusAdapter`](crate::usbd::BusAdapter).
/// When a chip family feature is enabled, various `take()` constructors
/// appear.
pub struct UsbInstances<'a, const N: u8> {
    /// Core registers.
    pub usb: usb::Instance<N>,
    /// PHY registers.
    pub usbphy: usbphy::Instance<N>,
    /// Non-core registers.
    pub usbnc: usbnc::Instance<N>,
    /// PLL registers.
    pub usb_analog: &'a mut USB_ANALOG,
}

// Safety: expressible for the 1010 family. There's only one USB
// peripheral, and it's USB1.
unsafe impl imxrt_usbd::Peripherals for UsbInstances<'_, 0> {
    fn instance(&self) -> imxrt_usbd::Instance {
        imxrt_usbd::Instance::USB1
    }
}

// Safety: expressible for all other families. It's definitely USB1.
unsafe impl imxrt_usbd::Peripherals for UsbInstances<'_, 1> {
    fn instance(&self) -> imxrt_usbd::Instance {
        imxrt_usbd::Instance::USB1
    }
}

// Safety: expressible for all other families. It's definitely USB2.
unsafe impl imxrt_usbd::Peripherals for UsbInstances<'_, 2> {
    fn instance(&self) -> imxrt_usbd::Instance {
        imxrt_usbd::Instance::USB2
    }
}
