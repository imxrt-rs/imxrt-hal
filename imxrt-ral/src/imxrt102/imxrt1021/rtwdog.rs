#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WDOG

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Watchdog Control and Status Register
pub mod CS {

    /// Stop Enable
    pub mod STOP {
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

            /// 0b0: Watchdog disabled in chip stop mode.
            pub const STOP_0: u32 = 0b0;

            /// 0b1: Watchdog enabled in chip stop mode.
            pub const STOP_1: u32 = 0b1;
        }
    }

    /// Wait Enable
    pub mod WAIT {
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

            /// 0b0: Watchdog disabled in chip wait mode.
            pub const WAIT_0: u32 = 0b0;

            /// 0b1: Watchdog enabled in chip wait mode.
            pub const WAIT_1: u32 = 0b1;
        }
    }

    /// Debug Enable
    pub mod DBG {
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

            /// 0b0: Watchdog disabled in chip debug mode.
            pub const DBG_0: u32 = 0b0;

            /// 0b1: Watchdog enabled in chip debug mode.
            pub const DBG_1: u32 = 0b1;
        }
    }

    /// Watchdog Test
    pub mod TST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Watchdog test mode disabled.
            pub const TST_0: u32 = 0b00;

            /// 0b01: Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode.
            pub const TST_1: u32 = 0b01;

            /// 0b10: Watchdog test mode enabled, only the low byte is used. CNT\[CNTLOW\] is compared with TOVAL\[TOVALLOW\].
            pub const TST_2: u32 = 0b10;

            /// 0b11: Watchdog test mode enabled, only the high byte is used. CNT\[CNTHIGH\] is compared with TOVAL\[TOVALHIGH\].
            pub const TST_3: u32 = 0b11;
        }
    }

    /// Allow updates
    pub mod UPDATE {
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

            /// 0b0: Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset.
            pub const UPDATE_0: u32 = 0b0;

            /// 0b1: Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence.
            pub const UPDATE_1: u32 = 0b1;
        }
    }

    /// Watchdog Interrupt
    pub mod INT {
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

            /// 0b0: Watchdog interrupts are disabled. Watchdog resets are not delayed.
            pub const INT_0: u32 = 0b0;

            /// 0b1: Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch.
            pub const INT_1: u32 = 0b1;
        }
    }

    /// Watchdog Enable
    pub mod EN {
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

            /// 0b0: Watchdog disabled.
            pub const EN_0: u32 = 0b0;

            /// 0b1: Watchdog enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// Watchdog Clock
    pub mod CLK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bus clock
            pub const CLK_0: u32 = 0b00;

            /// 0b01: LPO clock
            pub const CLK_1: u32 = 0b01;

            /// 0b10: INTCLK (internal clock)
            pub const CLK_2: u32 = 0b10;

            /// 0b11: ERCLK (external reference clock)
            pub const CLK_3: u32 = 0b11;
        }
    }

    /// Reconfiguration Success
    pub mod RCS {
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

            /// 0b0: Reconfiguring WDOG.
            pub const RCS_0: u32 = 0b0;

            /// 0b1: Reconfiguration is successful.
            pub const RCS_1: u32 = 0b1;
        }
    }

    /// Unlock status
    pub mod ULK {
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

            /// 0b0: WDOG is locked.
            pub const ULK_0: u32 = 0b0;

            /// 0b1: WDOG is unlocked.
            pub const ULK_1: u32 = 0b1;
        }
    }

    /// Watchdog prescaler
    pub mod PRES {
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

            /// 0b0: 256 prescaler disabled.
            pub const PRES_0: u32 = 0b0;

            /// 0b1: 256 prescaler enabled.
            pub const PRES_1: u32 = 0b1;
        }
    }

    /// Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words
    pub mod CMD32EN {
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

            /// 0b0: Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported.
            pub const CMD32EN_0: u32 = 0b0;

            /// 0b1: Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported.
            pub const CMD32EN_1: u32 = 0b1;
        }
    }

    /// Watchdog Interrupt Flag
    pub mod FLG {
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

            /// 0b0: No interrupt occurred.
            pub const FLG_0: u32 = 0b0;

            /// 0b1: An interrupt occurred.
            pub const FLG_1: u32 = 0b1;
        }
    }

    /// Watchdog Window
    pub mod WIN {
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

            /// 0b0: Window mode disabled.
            pub const WIN_0: u32 = 0b0;

            /// 0b1: Window mode enabled.
            pub const WIN_1: u32 = 0b1;
        }
    }
}

/// Watchdog Counter Register
pub mod CNT {

    /// Low byte of the Watchdog Counter
    pub mod CNTLOW {
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

    /// High byte of the Watchdog Counter
    pub mod CNTHIGH {
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

/// Watchdog Timeout Value Register
pub mod TOVAL {

    /// Low byte of the timeout value
    pub mod TOVALLOW {
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

    /// High byte of the timeout value
    pub mod TOVALHIGH {
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

/// Watchdog Window Register
pub mod WIN {

    /// Low byte of Watchdog Window
    pub mod WINLOW {
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

    /// High byte of Watchdog Window
    pub mod WINHIGH {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Watchdog Control and Status Register
    pub CS: RWRegister<u32>,

    /// Watchdog Counter Register
    pub CNT: RWRegister<u32>,

    /// Watchdog Timeout Value Register
    pub TOVAL: RWRegister<u32>,

    /// Watchdog Window Register
    pub WIN: RWRegister<u32>,
}
pub struct ResetValues {
    pub CS: u32,
    pub CNT: u32,
    pub TOVAL: u32,
    pub WIN: u32,
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

/// Access functions for the RTWDOG peripheral instance
pub mod RTWDOG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400bc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RTWDOG
    pub const reset: ResetValues = ResetValues {
        CS: 0x00002980,
        CNT: 0x00000000,
        TOVAL: 0x00000400,
        WIN: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RTWDOG_TAKEN: bool = false;

    /// Safe access to RTWDOG
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
            if RTWDOG_TAKEN {
                None
            } else {
                RTWDOG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RTWDOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RTWDOG_TAKEN && inst.addr == INSTANCE.addr {
                RTWDOG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RTWDOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RTWDOG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RTWDOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTWDOG: *const RegisterBlock = 0x400bc000 as *const _;
