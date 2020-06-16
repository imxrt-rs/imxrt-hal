#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Temperature Monitor

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Tempsensor Control Register 0
pub mod TEMPSENSE0 {

    /// This bit powers down the temperature sensor.
    pub mod POWER_DOWN {
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

            /// 0b0: Enable power to the temperature sensor.
            pub const POWER_UP: u32 = 0b0;

            /// 0b1: Power down the temperature sensor.
            pub const POWER_DOWN: u32 = 0b1;
        }
    }

    /// Starts the measurement process
    pub mod MEASURE_TEMP {
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

            /// 0b0: Do not start the measurement process.
            pub const STOP: u32 = 0b0;

            /// 0b1: Start the measurement process.
            pub const START: u32 = 0b1;
        }
    }

    /// Indicates that the latest temp is valid
    pub mod FINISHED {
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

            /// 0b0: Last measurement is not ready yet.
            pub const INVALID: u32 = 0b0;

            /// 0b1: Last measurement is valid.
            pub const VALID: u32 = 0b1;
        }
    }

    /// This bit field contains the last measured temperature count.
    pub mod TEMP_CNT {
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

    /// This bit field contains the temperature count (raw sensor output) that will generate an alarm interrupt
    pub mod ALARM_VALUE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tempsensor Control Register 0
pub mod TEMPSENSE0_SET {
    pub use super::TEMPSENSE0::ALARM_VALUE;
    pub use super::TEMPSENSE0::FINISHED;
    pub use super::TEMPSENSE0::MEASURE_TEMP;
    pub use super::TEMPSENSE0::POWER_DOWN;
    pub use super::TEMPSENSE0::TEMP_CNT;
}

/// Tempsensor Control Register 0
pub mod TEMPSENSE0_CLR {
    pub use super::TEMPSENSE0::ALARM_VALUE;
    pub use super::TEMPSENSE0::FINISHED;
    pub use super::TEMPSENSE0::MEASURE_TEMP;
    pub use super::TEMPSENSE0::POWER_DOWN;
    pub use super::TEMPSENSE0::TEMP_CNT;
}

/// Tempsensor Control Register 0
pub mod TEMPSENSE0_TOG {
    pub use super::TEMPSENSE0::ALARM_VALUE;
    pub use super::TEMPSENSE0::FINISHED;
    pub use super::TEMPSENSE0::MEASURE_TEMP;
    pub use super::TEMPSENSE0::POWER_DOWN;
    pub use super::TEMPSENSE0::TEMP_CNT;
}

/// Tempsensor Control Register 1
pub mod TEMPSENSE1 {

    /// This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement
    pub mod MEASURE_FREQ {
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

/// Tempsensor Control Register 1
pub mod TEMPSENSE1_SET {
    pub use super::TEMPSENSE1::MEASURE_FREQ;
}

/// Tempsensor Control Register 1
pub mod TEMPSENSE1_CLR {
    pub use super::TEMPSENSE1::MEASURE_FREQ;
}

/// Tempsensor Control Register 1
pub mod TEMPSENSE1_TOG {
    pub use super::TEMPSENSE1::MEASURE_FREQ;
}

/// Tempsensor Control Register 2
pub mod TEMPSENSE2 {

    /// This bit field contains the temperature that will generate a low alarm interrupt when the field is greater than the temperature measurement
    pub mod LOW_ALARM_VALUE {
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

    /// This bit field contains the temperature that will generate a panic interrupt when exceeded by the temperature measurement
    pub mod PANIC_ALARM_VALUE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tempsensor Control Register 2
pub mod TEMPSENSE2_SET {
    pub use super::TEMPSENSE2::LOW_ALARM_VALUE;
    pub use super::TEMPSENSE2::PANIC_ALARM_VALUE;
}

/// Tempsensor Control Register 2
pub mod TEMPSENSE2_CLR {
    pub use super::TEMPSENSE2::LOW_ALARM_VALUE;
    pub use super::TEMPSENSE2::PANIC_ALARM_VALUE;
}

/// Tempsensor Control Register 2
pub mod TEMPSENSE2_TOG {
    pub use super::TEMPSENSE2::LOW_ALARM_VALUE;
    pub use super::TEMPSENSE2::PANIC_ALARM_VALUE;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 96],

    /// Tempsensor Control Register 0
    pub TEMPSENSE0: RWRegister<u32>,

    /// Tempsensor Control Register 0
    pub TEMPSENSE0_SET: RWRegister<u32>,

    /// Tempsensor Control Register 0
    pub TEMPSENSE0_CLR: RWRegister<u32>,

    /// Tempsensor Control Register 0
    pub TEMPSENSE0_TOG: RWRegister<u32>,

    /// Tempsensor Control Register 1
    pub TEMPSENSE1: RWRegister<u32>,

    /// Tempsensor Control Register 1
    pub TEMPSENSE1_SET: RWRegister<u32>,

    /// Tempsensor Control Register 1
    pub TEMPSENSE1_CLR: RWRegister<u32>,

    /// Tempsensor Control Register 1
    pub TEMPSENSE1_TOG: RWRegister<u32>,

    _reserved2: [u32; 60],

    /// Tempsensor Control Register 2
    pub TEMPSENSE2: RWRegister<u32>,

    /// Tempsensor Control Register 2
    pub TEMPSENSE2_SET: RWRegister<u32>,

    /// Tempsensor Control Register 2
    pub TEMPSENSE2_CLR: RWRegister<u32>,

    /// Tempsensor Control Register 2
    pub TEMPSENSE2_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub TEMPSENSE0: u32,
    pub TEMPSENSE0_SET: u32,
    pub TEMPSENSE0_CLR: u32,
    pub TEMPSENSE0_TOG: u32,
    pub TEMPSENSE1: u32,
    pub TEMPSENSE1_SET: u32,
    pub TEMPSENSE1_CLR: u32,
    pub TEMPSENSE1_TOG: u32,
    pub TEMPSENSE2: u32,
    pub TEMPSENSE2_SET: u32,
    pub TEMPSENSE2_CLR: u32,
    pub TEMPSENSE2_TOG: u32,
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

/// Access functions for the TEMPMON peripheral instance
pub mod TEMPMON {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TEMPMON
    pub const reset: ResetValues = ResetValues {
        TEMPSENSE0: 0x00000001,
        TEMPSENSE0_SET: 0x00000001,
        TEMPSENSE0_CLR: 0x00000001,
        TEMPSENSE0_TOG: 0x00000001,
        TEMPSENSE1: 0x00000001,
        TEMPSENSE1_SET: 0x00000001,
        TEMPSENSE1_CLR: 0x00000001,
        TEMPSENSE1_TOG: 0x00000001,
        TEMPSENSE2: 0x00000000,
        TEMPSENSE2_SET: 0x00000000,
        TEMPSENSE2_CLR: 0x00000000,
        TEMPSENSE2_TOG: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TEMPMON_TAKEN: bool = false;

    /// Safe access to TEMPMON
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
            if TEMPMON_TAKEN {
                None
            } else {
                TEMPMON_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TEMPMON
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TEMPMON_TAKEN && inst.addr == INSTANCE.addr {
                TEMPMON_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TEMPMON
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TEMPMON_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TEMPMON
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TEMPMON: *const RegisterBlock = 0x400d8000 as *const _;
