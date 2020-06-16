#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! PGC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::pgc::Instance;
pub use crate::imxrt106::peripherals::pgc::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::pgc::{
    CPU_CTRL, CPU_PDNSCR, CPU_PUPSCR, CPU_SR, MEGA_CTRL, MEGA_PDNSCR, MEGA_PUPSCR, MEGA_SR,
};

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
