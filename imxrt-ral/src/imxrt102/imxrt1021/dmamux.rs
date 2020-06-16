#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Channel 0 Configuration Register
pub mod CHCFG0 {

    /// DMA Channel Source (Slot Number)
    pub mod SOURCE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Channel Always Enable
    pub mod A_ON {
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

            /// 0b0: DMA Channel Always ON function is disabled
            pub const A_ON_0: u32 = 0b0;

            /// 0b1: DMA Channel Always ON function is enabled
            pub const A_ON_1: u32 = 0b1;
        }
    }

    /// DMA Channel Trigger Enable
    pub mod TRIG {
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

            /// 0b0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)
            pub const TRIG_0: u32 = 0b0;

            /// 0b1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode.
            pub const TRIG_1: u32 = 0b1;
        }
    }

    /// DMA Mux Channel Enable
    pub mod ENBL {
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

            /// 0b0: DMA Mux channel is disabled
            pub const ENBL_0: u32 = 0b0;

            /// 0b1: DMA Mux channel is enabled
            pub const ENBL_1: u32 = 0b1;
        }
    }
}

/// Channel 0 Configuration Register
pub mod CHCFG1 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG2 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG3 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG4 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG5 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG6 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG7 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG8 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG9 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG10 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG11 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG12 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG13 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG14 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG15 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG16 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG17 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG18 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG19 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG20 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG21 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG22 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG23 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG24 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG25 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG26 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG27 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG28 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG29 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG30 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}

/// Channel 0 Configuration Register
pub mod CHCFG31 {
    pub use super::CHCFG0::A_ON;
    pub use super::CHCFG0::ENBL;
    pub use super::CHCFG0::SOURCE;
    pub use super::CHCFG0::TRIG;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Channel 0 Configuration Register
    pub CHCFG0: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG1: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG2: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG3: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG4: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG5: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG6: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG7: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG8: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG9: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG10: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG11: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG12: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG13: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG14: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG15: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG16: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG17: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG18: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG19: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG20: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG21: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG22: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG23: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG24: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG25: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG26: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG27: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG28: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG29: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG30: RWRegister<u32>,

    /// Channel 0 Configuration Register
    pub CHCFG31: RWRegister<u32>,
}
pub struct ResetValues {
    pub CHCFG0: u32,
    pub CHCFG1: u32,
    pub CHCFG2: u32,
    pub CHCFG3: u32,
    pub CHCFG4: u32,
    pub CHCFG5: u32,
    pub CHCFG6: u32,
    pub CHCFG7: u32,
    pub CHCFG8: u32,
    pub CHCFG9: u32,
    pub CHCFG10: u32,
    pub CHCFG11: u32,
    pub CHCFG12: u32,
    pub CHCFG13: u32,
    pub CHCFG14: u32,
    pub CHCFG15: u32,
    pub CHCFG16: u32,
    pub CHCFG17: u32,
    pub CHCFG18: u32,
    pub CHCFG19: u32,
    pub CHCFG20: u32,
    pub CHCFG21: u32,
    pub CHCFG22: u32,
    pub CHCFG23: u32,
    pub CHCFG24: u32,
    pub CHCFG25: u32,
    pub CHCFG26: u32,
    pub CHCFG27: u32,
    pub CHCFG28: u32,
    pub CHCFG29: u32,
    pub CHCFG30: u32,
    pub CHCFG31: u32,
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

/// Access functions for the DMAMUX peripheral instance
pub mod DMAMUX {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400ec000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX
    pub const reset: ResetValues = ResetValues {
        CHCFG0: 0x00000000,
        CHCFG1: 0x00000000,
        CHCFG2: 0x00000000,
        CHCFG3: 0x00000000,
        CHCFG4: 0x00000000,
        CHCFG5: 0x00000000,
        CHCFG6: 0x00000000,
        CHCFG7: 0x00000000,
        CHCFG8: 0x00000000,
        CHCFG9: 0x00000000,
        CHCFG10: 0x00000000,
        CHCFG11: 0x00000000,
        CHCFG12: 0x00000000,
        CHCFG13: 0x00000000,
        CHCFG14: 0x00000000,
        CHCFG15: 0x00000000,
        CHCFG16: 0x00000000,
        CHCFG17: 0x00000000,
        CHCFG18: 0x00000000,
        CHCFG19: 0x00000000,
        CHCFG20: 0x00000000,
        CHCFG21: 0x00000000,
        CHCFG22: 0x00000000,
        CHCFG23: 0x00000000,
        CHCFG24: 0x00000000,
        CHCFG25: 0x00000000,
        CHCFG26: 0x00000000,
        CHCFG27: 0x00000000,
        CHCFG28: 0x00000000,
        CHCFG29: 0x00000000,
        CHCFG30: 0x00000000,
        CHCFG31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX_TAKEN: bool = false;

    /// Safe access to DMAMUX
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
            if DMAMUX_TAKEN {
                None
            } else {
                DMAMUX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX: *const RegisterBlock = 0x400ec000 as *const _;
