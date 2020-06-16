#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::ccm::Instance;
pub use crate::imxrt106::peripherals::ccm::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::ccm::{
    CACRR, CBCDR, CBCMR, CCGR0, CCGR1, CCGR2, CCGR3, CCGR4, CCGR5, CCGR6, CCGR7, CCOSR, CCR, CCSR,
    CDCDR, CDHIPR, CGPR, CIMR, CISR, CLPCR, CMEOR, CS1CDR, CS2CDR, CSCDR1, CSCDR2, CSCDR3, CSCMR1,
    CSCMR2, CSR,
};

/// Access functions for the CCM peripheral instance
pub mod CCM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400fc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CCM
    pub const reset: ResetValues = ResetValues {
        CCR: 0x0401107F,
        CSR: 0x00000010,
        CCSR: 0x00000100,
        CACRR: 0x00000001,
        CBCDR: 0x000A8300,
        CBCMR: 0x2DAE8324,
        CSCMR1: 0x04900000,
        CSCMR2: 0x13192F06,
        CSCDR1: 0x06490B00,
        CS1CDR: 0x0EC102C1,
        CS2CDR: 0x007336C1,
        CDCDR: 0x33F71F92,
        CSCDR2: 0x00029150,
        CSCDR3: 0x00030841,
        CDHIPR: 0x00000000,
        CLPCR: 0x00000079,
        CISR: 0x00000000,
        CIMR: 0xFFFFFFFF,
        CCOSR: 0x000A0001,
        CGPR: 0x0000FE62,
        CCGR0: 0xFFFFFFFF,
        CCGR1: 0xFFFFFFFF,
        CCGR2: 0xFC3FFFFF,
        CCGR3: 0xFFFFFFCF,
        CCGR4: 0xFFFFFFFF,
        CCGR5: 0xFFFFFFFF,
        CCGR6: 0xFFFFFFFF,
        CCGR7: 0xFFFFFFFF,
        CMEOR: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CCM_TAKEN: bool = false;

    /// Safe access to CCM
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
            if CCM_TAKEN {
                None
            } else {
                CCM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CCM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CCM_TAKEN && inst.addr == INSTANCE.addr {
                CCM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CCM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CCM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CCM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CCM: *const RegisterBlock = 0x400fc000 as *const _;
