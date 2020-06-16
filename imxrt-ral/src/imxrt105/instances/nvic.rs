#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::nvic::Instance;
pub use crate::imxrt105::peripherals::nvic::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::nvic::{
    NVICIABR0, NVICIABR1, NVICIABR2, NVICIABR3, NVICIABR4, NVICICER0, NVICICER1, NVICICER2,
    NVICICER3, NVICICER4, NVICICPR0, NVICICPR1, NVICICPR2, NVICICPR3, NVICICPR4, NVICIP0, NVICIP1,
    NVICIP10, NVICIP100, NVICIP101, NVICIP102, NVICIP103, NVICIP104, NVICIP105, NVICIP106,
    NVICIP107, NVICIP108, NVICIP109, NVICIP11, NVICIP110, NVICIP111, NVICIP112, NVICIP113,
    NVICIP114, NVICIP115, NVICIP116, NVICIP117, NVICIP118, NVICIP119, NVICIP12, NVICIP120,
    NVICIP121, NVICIP122, NVICIP123, NVICIP124, NVICIP125, NVICIP126, NVICIP127, NVICIP128,
    NVICIP129, NVICIP13, NVICIP130, NVICIP131, NVICIP132, NVICIP133, NVICIP134, NVICIP135,
    NVICIP136, NVICIP137, NVICIP138, NVICIP139, NVICIP14, NVICIP140, NVICIP141, NVICIP142,
    NVICIP143, NVICIP144, NVICIP145, NVICIP146, NVICIP147, NVICIP148, NVICIP149, NVICIP15,
    NVICIP150, NVICIP151, NVICIP16, NVICIP17, NVICIP18, NVICIP19, NVICIP2, NVICIP20, NVICIP21,
    NVICIP22, NVICIP23, NVICIP24, NVICIP25, NVICIP26, NVICIP27, NVICIP28, NVICIP29, NVICIP3,
    NVICIP30, NVICIP31, NVICIP32, NVICIP33, NVICIP34, NVICIP35, NVICIP36, NVICIP37, NVICIP38,
    NVICIP39, NVICIP4, NVICIP40, NVICIP41, NVICIP42, NVICIP43, NVICIP44, NVICIP45, NVICIP46,
    NVICIP47, NVICIP48, NVICIP49, NVICIP5, NVICIP50, NVICIP51, NVICIP52, NVICIP53, NVICIP54,
    NVICIP55, NVICIP56, NVICIP57, NVICIP58, NVICIP59, NVICIP6, NVICIP60, NVICIP61, NVICIP62,
    NVICIP63, NVICIP64, NVICIP65, NVICIP66, NVICIP67, NVICIP68, NVICIP69, NVICIP7, NVICIP70,
    NVICIP71, NVICIP72, NVICIP73, NVICIP74, NVICIP75, NVICIP76, NVICIP77, NVICIP78, NVICIP79,
    NVICIP8, NVICIP80, NVICIP81, NVICIP82, NVICIP83, NVICIP84, NVICIP85, NVICIP86, NVICIP87,
    NVICIP88, NVICIP89, NVICIP9, NVICIP90, NVICIP91, NVICIP92, NVICIP93, NVICIP94, NVICIP95,
    NVICIP96, NVICIP97, NVICIP98, NVICIP99, NVICISER0, NVICISER1, NVICISER2, NVICISER3, NVICISER4,
    NVICISPR0, NVICISPR1, NVICISPR2, NVICISPR3, NVICISPR4, NVICSTIR,
};

/// Access functions for the NVIC peripheral instance
pub mod NVIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in NVIC
    pub const reset: ResetValues = ResetValues {
        NVICISER0: 0x00000000,
        NVICISER1: 0x00000000,
        NVICISER2: 0x00000000,
        NVICISER3: 0x00000000,
        NVICISER4: 0x00000000,
        NVICICER0: 0x00000000,
        NVICICER1: 0x00000000,
        NVICICER2: 0x00000000,
        NVICICER3: 0x00000000,
        NVICICER4: 0x00000000,
        NVICISPR0: 0x00000000,
        NVICISPR1: 0x00000000,
        NVICISPR2: 0x00000000,
        NVICISPR3: 0x00000000,
        NVICISPR4: 0x00000000,
        NVICICPR0: 0x00000000,
        NVICICPR1: 0x00000000,
        NVICICPR2: 0x00000000,
        NVICICPR3: 0x00000000,
        NVICICPR4: 0x00000000,
        NVICIABR0: 0x00000000,
        NVICIABR1: 0x00000000,
        NVICIABR2: 0x00000000,
        NVICIABR3: 0x00000000,
        NVICIABR4: 0x00000000,
        NVICIP0: 0x00000000,
        NVICIP1: 0x00000000,
        NVICIP2: 0x00000000,
        NVICIP3: 0x00000000,
        NVICIP4: 0x00000000,
        NVICIP5: 0x00000000,
        NVICIP6: 0x00000000,
        NVICIP7: 0x00000000,
        NVICIP8: 0x00000000,
        NVICIP9: 0x00000000,
        NVICIP10: 0x00000000,
        NVICIP11: 0x00000000,
        NVICIP12: 0x00000000,
        NVICIP13: 0x00000000,
        NVICIP14: 0x00000000,
        NVICIP15: 0x00000000,
        NVICIP16: 0x00000000,
        NVICIP17: 0x00000000,
        NVICIP18: 0x00000000,
        NVICIP19: 0x00000000,
        NVICIP20: 0x00000000,
        NVICIP21: 0x00000000,
        NVICIP22: 0x00000000,
        NVICIP23: 0x00000000,
        NVICIP24: 0x00000000,
        NVICIP25: 0x00000000,
        NVICIP26: 0x00000000,
        NVICIP27: 0x00000000,
        NVICIP28: 0x00000000,
        NVICIP29: 0x00000000,
        NVICIP30: 0x00000000,
        NVICIP31: 0x00000000,
        NVICIP32: 0x00000000,
        NVICIP33: 0x00000000,
        NVICIP34: 0x00000000,
        NVICIP35: 0x00000000,
        NVICIP36: 0x00000000,
        NVICIP37: 0x00000000,
        NVICIP38: 0x00000000,
        NVICIP39: 0x00000000,
        NVICIP40: 0x00000000,
        NVICIP41: 0x00000000,
        NVICIP42: 0x00000000,
        NVICIP43: 0x00000000,
        NVICIP44: 0x00000000,
        NVICIP45: 0x00000000,
        NVICIP46: 0x00000000,
        NVICIP47: 0x00000000,
        NVICIP48: 0x00000000,
        NVICIP49: 0x00000000,
        NVICIP50: 0x00000000,
        NVICIP51: 0x00000000,
        NVICIP52: 0x00000000,
        NVICIP53: 0x00000000,
        NVICIP54: 0x00000000,
        NVICIP55: 0x00000000,
        NVICIP56: 0x00000000,
        NVICIP57: 0x00000000,
        NVICIP58: 0x00000000,
        NVICIP59: 0x00000000,
        NVICIP60: 0x00000000,
        NVICIP61: 0x00000000,
        NVICIP62: 0x00000000,
        NVICIP63: 0x00000000,
        NVICIP64: 0x00000000,
        NVICIP65: 0x00000000,
        NVICIP66: 0x00000000,
        NVICIP67: 0x00000000,
        NVICIP68: 0x00000000,
        NVICIP69: 0x00000000,
        NVICIP70: 0x00000000,
        NVICIP71: 0x00000000,
        NVICIP72: 0x00000000,
        NVICIP73: 0x00000000,
        NVICIP74: 0x00000000,
        NVICIP75: 0x00000000,
        NVICIP76: 0x00000000,
        NVICIP77: 0x00000000,
        NVICIP78: 0x00000000,
        NVICIP79: 0x00000000,
        NVICIP80: 0x00000000,
        NVICIP81: 0x00000000,
        NVICIP82: 0x00000000,
        NVICIP83: 0x00000000,
        NVICIP84: 0x00000000,
        NVICIP85: 0x00000000,
        NVICIP86: 0x00000000,
        NVICIP87: 0x00000000,
        NVICIP88: 0x00000000,
        NVICIP89: 0x00000000,
        NVICIP90: 0x00000000,
        NVICIP91: 0x00000000,
        NVICIP92: 0x00000000,
        NVICIP93: 0x00000000,
        NVICIP94: 0x00000000,
        NVICIP95: 0x00000000,
        NVICIP96: 0x00000000,
        NVICIP97: 0x00000000,
        NVICIP98: 0x00000000,
        NVICIP99: 0x00000000,
        NVICIP100: 0x00000000,
        NVICIP101: 0x00000000,
        NVICIP102: 0x00000000,
        NVICIP103: 0x00000000,
        NVICIP104: 0x00000000,
        NVICIP105: 0x00000000,
        NVICIP106: 0x00000000,
        NVICIP107: 0x00000000,
        NVICIP108: 0x00000000,
        NVICIP109: 0x00000000,
        NVICIP110: 0x00000000,
        NVICIP111: 0x00000000,
        NVICIP112: 0x00000000,
        NVICIP113: 0x00000000,
        NVICIP114: 0x00000000,
        NVICIP115: 0x00000000,
        NVICIP116: 0x00000000,
        NVICIP117: 0x00000000,
        NVICIP118: 0x00000000,
        NVICIP119: 0x00000000,
        NVICIP120: 0x00000000,
        NVICIP121: 0x00000000,
        NVICIP122: 0x00000000,
        NVICIP123: 0x00000000,
        NVICIP124: 0x00000000,
        NVICIP125: 0x00000000,
        NVICIP126: 0x00000000,
        NVICIP127: 0x00000000,
        NVICIP128: 0x00000000,
        NVICIP129: 0x00000000,
        NVICIP130: 0x00000000,
        NVICIP131: 0x00000000,
        NVICIP132: 0x00000000,
        NVICIP133: 0x00000000,
        NVICIP134: 0x00000000,
        NVICIP135: 0x00000000,
        NVICIP136: 0x00000000,
        NVICIP137: 0x00000000,
        NVICIP138: 0x00000000,
        NVICIP139: 0x00000000,
        NVICIP140: 0x00000000,
        NVICIP141: 0x00000000,
        NVICIP142: 0x00000000,
        NVICIP143: 0x00000000,
        NVICIP144: 0x00000000,
        NVICIP145: 0x00000000,
        NVICIP146: 0x00000000,
        NVICIP147: 0x00000000,
        NVICIP148: 0x00000000,
        NVICIP149: 0x00000000,
        NVICIP150: 0x00000000,
        NVICIP151: 0x00000000,
        NVICSTIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut NVIC_TAKEN: bool = false;

    /// Safe access to NVIC
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
            if NVIC_TAKEN {
                None
            } else {
                NVIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to NVIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN && inst.addr == INSTANCE.addr {
                NVIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal NVIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        NVIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to NVIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const NVIC: *const RegisterBlock = 0xe000e100 as *const _;
