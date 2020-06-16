#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGC

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// PGC Mega Control Register
pub mod MEGA_CTRL {

    /// Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up
    pub mod PCR {
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

            /// 0b0: Do not switch off power even if pdn_req is asserted.
            pub const PCR_0: u32 = 0b0;

            /// 0b1: Switch off power when pdn_req is asserted.
            pub const PCR_1: u32 = 0b1;
        }
    }
}

/// PGC Mega Power Up Sequence Control Register
pub mod MEGA_PUPSCR {

    /// After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b)
    pub mod SW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// After asserting power toggle on/off signal (switch_b), the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation
    pub mod SW2ISO {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PGC Mega Pull Down Sequence Control Register
pub mod MEGA_PDNSCR {

    /// After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation
    pub mod ISO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b)
    pub mod ISO2SW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PGC Mega Power Gating Controller Status Register
pub mod MEGA_SR {

    /// Power status
    pub mod PSR {
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

            /// 0b0: The target subsystem was not powered down for the previous power-down request.
            pub const PSR_0: u32 = 0b0;

            /// 0b1: The target subsystem was powered down for the previous power-down request.
            pub const PSR_1: u32 = 0b1;
        }
    }
}

/// PGC CPU Control Register
pub mod CPU_CTRL {
    pub use super::MEGA_CTRL::PCR;
}

/// PGC CPU Power Up Sequence Control Register
pub mod CPU_PUPSCR {
    pub use super::MEGA_PUPSCR::SW;
    pub use super::MEGA_PUPSCR::SW2ISO;
}

/// PGC CPU Pull Down Sequence Control Register
pub mod CPU_PDNSCR {
    pub use super::MEGA_PDNSCR::ISO;
    pub use super::MEGA_PDNSCR::ISO2SW;
}

/// PGC CPU Power Gating Controller Status Register
pub mod CPU_SR {
    pub use super::MEGA_SR::PSR;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 136],

    /// PGC Mega Control Register
    pub MEGA_CTRL: RWRegister<u32>,

    /// PGC Mega Power Up Sequence Control Register
    pub MEGA_PUPSCR: RWRegister<u32>,

    /// PGC Mega Pull Down Sequence Control Register
    pub MEGA_PDNSCR: RWRegister<u32>,

    /// PGC Mega Power Gating Controller Status Register
    pub MEGA_SR: RWRegister<u32>,

    _reserved2: [u32; 28],

    /// PGC CPU Control Register
    pub CPU_CTRL: RWRegister<u32>,

    /// PGC CPU Power Up Sequence Control Register
    pub CPU_PUPSCR: RWRegister<u32>,

    /// PGC CPU Pull Down Sequence Control Register
    pub CPU_PDNSCR: RWRegister<u32>,

    /// PGC CPU Power Gating Controller Status Register
    pub CPU_SR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MEGA_CTRL: u32,
    pub MEGA_PUPSCR: u32,
    pub MEGA_PDNSCR: u32,
    pub MEGA_SR: u32,
    pub CPU_CTRL: u32,
    pub CPU_PUPSCR: u32,
    pub CPU_PDNSCR: u32,
    pub CPU_SR: u32,
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

/// Access functions for the PGC peripheral instance
pub mod PGC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400f4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PGC
    pub const reset: ResetValues = ResetValues {
        MEGA_CTRL: 0x00000000,
        MEGA_PUPSCR: 0x00000F01,
        MEGA_PDNSCR: 0x00000101,
        MEGA_SR: 0x00000000,
        CPU_CTRL: 0x00000000,
        CPU_PUPSCR: 0x00000F01,
        CPU_PDNSCR: 0x00000101,
        CPU_SR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PGC_TAKEN: bool = false;

    /// Safe access to PGC
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
            if PGC_TAKEN {
                None
            } else {
                PGC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PGC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PGC_TAKEN && inst.addr == INSTANCE.addr {
                PGC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PGC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PGC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PGC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PGC: *const RegisterBlock = 0x400f4000 as *const _;
