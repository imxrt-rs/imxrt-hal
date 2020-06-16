#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! KPP Registers

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Keypad Control Register
pub mod KPCR {

    /// Keypad Row Enable
    pub mod KRE {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Row is not included in the keypad key press detect.
            pub const KRE_0: u16 = 0b00000000;

            /// 0b00000001: Row is included in the keypad key press detect.
            pub const KRE_1: u16 = 0b00000001;
        }
    }

    /// Keypad Column Strobe Open-Drain Enable
    pub mod KCO {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: Column strobe output is totem pole drive.
            pub const TOTEM_POLE: u16 = 0b00000000;

            /// 0b00000001: Column strobe output is open drain.
            pub const OPEN_DRAIN: u16 = 0b00000001;
        }
    }
}

/// Keypad Status Register
pub mod KPSR {

    /// Keypad Key Depress
    pub mod KPKD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No key presses detected
            pub const KPKD_0: u16 = 0b0;

            /// 0b1: A key has been depressed
            pub const KPKD_1: u16 = 0b1;
        }
    }

    /// Keypad Key Release
    pub mod KPKR {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No key release detected
            pub const KPKR_0: u16 = 0b0;

            /// 0b1: All keys have been released
            pub const KPKR_1: u16 = 0b1;
        }
    }

    /// Key Depress Synchronizer Clear
    pub mod KDSC {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const KDSC_0: u16 = 0b0;

            /// 0b1: Set bits that clear the keypad depress synchronizer chain
            pub const KDSC_1: u16 = 0b1;
        }
    }

    /// Key Release Synchronizer Set
    pub mod KRSS {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const KRSS_0: u16 = 0b0;

            /// 0b1: Set bits which sets keypad release synchronizer chain
            pub const KRSS_1: u16 = 0b1;
        }
    }

    /// Keypad Key Depress Interrupt Enable
    pub mod KDIE {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt request is generated when KPKD is set.
            pub const KDIE_0: u16 = 0b0;

            /// 0b1: An interrupt request is generated when KPKD is set.
            pub const KDIE_1: u16 = 0b1;
        }
    }

    /// Keypad Release Interrupt Enable
    pub mod KRIE {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt request is generated when KPKR is set.
            pub const KRIE_0: u16 = 0b0;

            /// 0b1: An interrupt request is generated when KPKR is set.
            pub const KRIE_1: u16 = 0b1;
        }
    }
}

/// Keypad Data Direction Register
pub mod KDDR {

    /// Keypad Row Data Direction
    pub mod KRDD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: ROWn pin configured as an input.
            pub const INPUT: u16 = 0b00000000;

            /// 0b00000001: ROWn pin configured as an output.
            pub const OUTPUT: u16 = 0b00000001;
        }
    }

    /// Keypad Column Data Direction Register
    pub mod KCDD {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: COLn pin is configured as an input.
            pub const INPUT: u16 = 0b00000000;

            /// 0b00000001: COLn pin is configured as an output.
            pub const OUTPUT: u16 = 0b00000001;
        }
    }
}

/// Keypad Data Register
pub mod KPDR {

    /// Keypad Row Data
    pub mod KRD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Keypad Column Data
    pub mod KCD {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u16 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Keypad Control Register
    pub KPCR: RWRegister<u16>,

    /// Keypad Status Register
    pub KPSR: RWRegister<u16>,

    /// Keypad Data Direction Register
    pub KDDR: RWRegister<u16>,

    /// Keypad Data Register
    pub KPDR: RWRegister<u16>,
}
pub struct ResetValues {
    pub KPCR: u16,
    pub KPSR: u16,
    pub KDDR: u16,
    pub KPDR: u16,
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

/// Access functions for the KPP peripheral instance
pub mod KPP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401fc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in KPP
    pub const reset: ResetValues = ResetValues {
        KPCR: 0x00000000,
        KPSR: 0x00000400,
        KDDR: 0x00000000,
        KPDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut KPP_TAKEN: bool = false;

    /// Safe access to KPP
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
            if KPP_TAKEN {
                None
            } else {
                KPP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to KPP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if KPP_TAKEN && inst.addr == INSTANCE.addr {
                KPP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal KPP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        KPP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to KPP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const KPP: *const RegisterBlock = 0x401fc000 as *const _;
