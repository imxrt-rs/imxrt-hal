#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB Analog

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// USB VBUS Detect Register
pub mod USB1_VBUS_DETECT {

    /// Set the threshold for the VBUSVALID comparator
    pub mod VBUSVALID_THRESH {
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

            /// 0b000: 4.0V
            pub const _4V0: u32 = 0b000;

            /// 0b001: 4.1V
            pub const _4V1: u32 = 0b001;

            /// 0b010: 4.2V
            pub const _4V2: u32 = 0b010;

            /// 0b011: 4.3V
            pub const _4V3: u32 = 0b011;

            /// 0b100: 4.4V (default)
            pub const _4V4: u32 = 0b100;

            /// 0b101: 4.5V
            pub const _4V5: u32 = 0b101;

            /// 0b110: 4.6V
            pub const _4V6: u32 = 0b110;

            /// 0b111: 4.7V
            pub const _4V7: u32 = 0b111;
        }
    }

    /// Powers up comparators for vbus_valid detector.
    pub mod VBUSVALID_PWRUP_CMPS {
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

    /// USB OTG discharge VBUS.
    pub mod DISCHARGE_VBUS {
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

    /// USB OTG charge VBUS.
    pub mod CHARGE_VBUS {
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
}

/// USB VBUS Detect Register
pub mod USB1_VBUS_DETECT_SET {
    pub use super::USB1_VBUS_DETECT::CHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::DISCHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_PWRUP_CMPS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_THRESH;
}

/// USB VBUS Detect Register
pub mod USB1_VBUS_DETECT_CLR {
    pub use super::USB1_VBUS_DETECT::CHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::DISCHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_PWRUP_CMPS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_THRESH;
}

/// USB VBUS Detect Register
pub mod USB1_VBUS_DETECT_TOG {
    pub use super::USB1_VBUS_DETECT::CHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::DISCHARGE_VBUS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_PWRUP_CMPS;
    pub use super::USB1_VBUS_DETECT::VBUSVALID_THRESH;
}

/// USB Charger Detect Register
pub mod USB1_CHRG_DETECT {

    /// Check the contact of USB plug
    pub mod CHK_CONTACT {
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

            /// 0b0: Do not check the contact of USB plug.
            pub const NO_CHECK: u32 = 0b0;

            /// 0b1: Check whether the USB plug has been in contact with each other
            pub const CHECK: u32 = 0b1;
        }
    }

    /// Check the charger connection
    pub mod CHK_CHRG_B {
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

            /// 0b0: Check whether a charger (either a dedicated charger or a host charger) is connected to USB port.
            pub const CHECK: u32 = 0b0;

            /// 0b1: Do not check whether a charger is connected to the USB port.
            pub const NO_CHECK: u32 = 0b1;
        }
    }

    /// Control the charger detector.
    pub mod EN_B {
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

            /// 0b0: Enable the charger detector.
            pub const ENABLE: u32 = 0b0;

            /// 0b1: Disable the charger detector.
            pub const DISABLE: u32 = 0b1;
        }
    }
}

/// USB Charger Detect Register
pub mod USB1_CHRG_DETECT_SET {
    pub use super::USB1_CHRG_DETECT::CHK_CHRG_B;
    pub use super::USB1_CHRG_DETECT::CHK_CONTACT;
    pub use super::USB1_CHRG_DETECT::EN_B;
}

/// USB Charger Detect Register
pub mod USB1_CHRG_DETECT_CLR {
    pub use super::USB1_CHRG_DETECT::CHK_CHRG_B;
    pub use super::USB1_CHRG_DETECT::CHK_CONTACT;
    pub use super::USB1_CHRG_DETECT::EN_B;
}

/// USB Charger Detect Register
pub mod USB1_CHRG_DETECT_TOG {
    pub use super::USB1_CHRG_DETECT::CHK_CHRG_B;
    pub use super::USB1_CHRG_DETECT::CHK_CONTACT;
    pub use super::USB1_CHRG_DETECT::EN_B;
}

/// USB VBUS Detect Status Register
pub mod USB1_VBUS_DETECT_STAT {

    /// Session End for USB OTG
    pub mod SESSEND {
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

    /// Indicates VBus is valid for a B-peripheral
    pub mod BVALID {
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

    /// Indicates VBus is valid for a A-peripheral
    pub mod AVALID {
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

    /// VBus valid for USB OTG
    pub mod VBUS_VALID {
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
}

/// USB Charger Detect Status Register
pub mod USB1_CHRG_DETECT_STAT {

    /// State of the USB plug contact detector.
    pub mod PLUG_CONTACT {
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

            /// 0b0: The USB plug has not made contact.
            pub const NO_CONTACT: u32 = 0b0;

            /// 0b1: The USB plug has made good contact.
            pub const GOOD_CONTACT: u32 = 0b1;
        }
    }

    /// State of charger detection. This bit is a read only version of the state of the analog signal.
    pub mod CHRG_DETECTED {
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

            /// 0b0: The USB port is not connected to a charger.
            pub const CHARGER_NOT_PRESENT: u32 = 0b0;

            /// 0b1: A charger (either a dedicated charger or a host charger) is connected to the USB port.
            pub const CHARGER_PRESENT: u32 = 0b1;
        }
    }

    /// DM line state output of the charger detector.
    pub mod DM_STATE {
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

    /// DP line state output of the charger detector.
    pub mod DP_STATE {
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
}

/// USB Loopback Test Register
pub mod USB1_LOOPBACK {

    /// Setting this bit can enable 1
    pub mod UTMI_TESTSTART {
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

/// USB Loopback Test Register
pub mod USB1_LOOPBACK_SET {
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
}

/// USB Loopback Test Register
pub mod USB1_LOOPBACK_CLR {
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
}

/// USB Loopback Test Register
pub mod USB1_LOOPBACK_TOG {
    pub use super::USB1_LOOPBACK::UTMI_TESTSTART;
}

/// USB Misc Register
pub mod USB1_MISC {

    /// Use external resistor to generate the current bias for the high speed transmitter
    pub mod HS_USE_EXTERNAL_R {
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

    /// Enable the deglitching circuit of the USB PLL output.
    pub mod EN_DEGLITCH {
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

    /// Enables the clk to the UTMI block.
    pub mod EN_CLK_UTMI {
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

/// USB Misc Register
pub mod USB1_MISC_SET {
    pub use super::USB1_MISC::EN_CLK_UTMI;
    pub use super::USB1_MISC::EN_DEGLITCH;
    pub use super::USB1_MISC::HS_USE_EXTERNAL_R;
}

/// USB Misc Register
pub mod USB1_MISC_CLR {
    pub use super::USB1_MISC::EN_CLK_UTMI;
    pub use super::USB1_MISC::EN_DEGLITCH;
    pub use super::USB1_MISC::HS_USE_EXTERNAL_R;
}

/// USB Misc Register
pub mod USB1_MISC_TOG {
    pub use super::USB1_MISC::EN_CLK_UTMI;
    pub use super::USB1_MISC::EN_DEGLITCH;
    pub use super::USB1_MISC::HS_USE_EXTERNAL_R;
}

/// Chip Silicon Version
pub mod DIGPROG {

    /// Chip silicon revision
    pub mod SILICON_REVISION {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000011010110000000000000000: Silicon revision 1.0
            pub const SILICON_REVISION_7012352: u32 = 0b00000000011010110000000000000000;
        }
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 104],

    /// USB VBUS Detect Register
    pub USB1_VBUS_DETECT: RWRegister<u32>,

    /// USB VBUS Detect Register
    pub USB1_VBUS_DETECT_SET: RWRegister<u32>,

    /// USB VBUS Detect Register
    pub USB1_VBUS_DETECT_CLR: RWRegister<u32>,

    /// USB VBUS Detect Register
    pub USB1_VBUS_DETECT_TOG: RWRegister<u32>,

    /// USB Charger Detect Register
    pub USB1_CHRG_DETECT: RWRegister<u32>,

    /// USB Charger Detect Register
    pub USB1_CHRG_DETECT_SET: RWRegister<u32>,

    /// USB Charger Detect Register
    pub USB1_CHRG_DETECT_CLR: RWRegister<u32>,

    /// USB Charger Detect Register
    pub USB1_CHRG_DETECT_TOG: RWRegister<u32>,

    /// USB VBUS Detect Status Register
    pub USB1_VBUS_DETECT_STAT: RORegister<u32>,

    _reserved2: [u32; 3],

    /// USB Charger Detect Status Register
    pub USB1_CHRG_DETECT_STAT: RORegister<u32>,

    _reserved3: [u32; 3],

    /// USB Loopback Test Register
    pub USB1_LOOPBACK: RWRegister<u32>,

    /// USB Loopback Test Register
    pub USB1_LOOPBACK_SET: RWRegister<u32>,

    /// USB Loopback Test Register
    pub USB1_LOOPBACK_CLR: RWRegister<u32>,

    /// USB Loopback Test Register
    pub USB1_LOOPBACK_TOG: RWRegister<u32>,

    /// USB Misc Register
    pub USB1_MISC: RWRegister<u32>,

    /// USB Misc Register
    pub USB1_MISC_SET: RWRegister<u32>,

    /// USB Misc Register
    pub USB1_MISC_CLR: RWRegister<u32>,

    /// USB Misc Register
    pub USB1_MISC_TOG: RWRegister<u32>,

    _reserved4: [u32; 24],

    /// Chip Silicon Version
    pub DIGPROG: RORegister<u32>,
}
pub struct ResetValues {
    pub USB1_VBUS_DETECT: u32,
    pub USB1_VBUS_DETECT_SET: u32,
    pub USB1_VBUS_DETECT_CLR: u32,
    pub USB1_VBUS_DETECT_TOG: u32,
    pub USB1_CHRG_DETECT: u32,
    pub USB1_CHRG_DETECT_SET: u32,
    pub USB1_CHRG_DETECT_CLR: u32,
    pub USB1_CHRG_DETECT_TOG: u32,
    pub USB1_VBUS_DETECT_STAT: u32,
    pub USB1_CHRG_DETECT_STAT: u32,
    pub USB1_LOOPBACK: u32,
    pub USB1_LOOPBACK_SET: u32,
    pub USB1_LOOPBACK_CLR: u32,
    pub USB1_LOOPBACK_TOG: u32,
    pub USB1_MISC: u32,
    pub USB1_MISC_SET: u32,
    pub USB1_MISC_CLR: u32,
    pub USB1_MISC_TOG: u32,
    pub DIGPROG: u32,
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

/// Access functions for the USB_ANALOG peripheral instance
pub mod USB_ANALOG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_ANALOG
    pub const reset: ResetValues = ResetValues {
        USB1_VBUS_DETECT: 0x00100004,
        USB1_VBUS_DETECT_SET: 0x00100004,
        USB1_VBUS_DETECT_CLR: 0x00100004,
        USB1_VBUS_DETECT_TOG: 0x00100004,
        USB1_CHRG_DETECT: 0x00000000,
        USB1_CHRG_DETECT_SET: 0x00000000,
        USB1_CHRG_DETECT_CLR: 0x00000000,
        USB1_CHRG_DETECT_TOG: 0x00000000,
        USB1_VBUS_DETECT_STAT: 0x00000000,
        USB1_CHRG_DETECT_STAT: 0x00000000,
        USB1_LOOPBACK: 0x00000000,
        USB1_LOOPBACK_SET: 0x00000000,
        USB1_LOOPBACK_CLR: 0x00000000,
        USB1_LOOPBACK_TOG: 0x00000000,
        USB1_MISC: 0x00000002,
        USB1_MISC_SET: 0x00000002,
        USB1_MISC_CLR: 0x00000002,
        USB1_MISC_TOG: 0x00000002,
        DIGPROG: 0x006B0000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_ANALOG_TAKEN: bool = false;

    /// Safe access to USB_ANALOG
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
            if USB_ANALOG_TAKEN {
                None
            } else {
                USB_ANALOG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB_ANALOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_ANALOG_TAKEN && inst.addr == INSTANCE.addr {
                USB_ANALOG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB_ANALOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_ANALOG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB_ANALOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_ANALOG: *const RegisterBlock = 0x400d8000 as *const _;
