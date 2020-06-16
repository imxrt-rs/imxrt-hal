#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Instrumentation Trace Macrocell
//!
//! Used by: armv7em, armv7m

#[cfg(not(feature = "nosync"))]
pub use crate::cortex_m::peripherals::itm::Instance;
pub use crate::cortex_m::peripherals::itm::{RegisterBlock, ResetValues};
pub use crate::cortex_m::peripherals::itm::{
    LAR, LSR, STIM0, STIM1, STIM10, STIM100, STIM101, STIM102, STIM103, STIM104, STIM105, STIM106,
    STIM107, STIM108, STIM109, STIM11, STIM110, STIM111, STIM112, STIM113, STIM114, STIM115,
    STIM116, STIM117, STIM118, STIM119, STIM12, STIM120, STIM121, STIM122, STIM123, STIM124,
    STIM125, STIM126, STIM127, STIM128, STIM129, STIM13, STIM130, STIM131, STIM132, STIM133,
    STIM134, STIM135, STIM136, STIM137, STIM138, STIM139, STIM14, STIM140, STIM141, STIM142,
    STIM143, STIM144, STIM145, STIM146, STIM147, STIM148, STIM149, STIM15, STIM150, STIM151,
    STIM152, STIM153, STIM154, STIM155, STIM156, STIM157, STIM158, STIM159, STIM16, STIM160,
    STIM161, STIM162, STIM163, STIM164, STIM165, STIM166, STIM167, STIM168, STIM169, STIM17,
    STIM170, STIM171, STIM172, STIM173, STIM174, STIM175, STIM176, STIM177, STIM178, STIM179,
    STIM18, STIM180, STIM181, STIM182, STIM183, STIM184, STIM185, STIM186, STIM187, STIM188,
    STIM189, STIM19, STIM190, STIM191, STIM192, STIM193, STIM194, STIM195, STIM196, STIM197,
    STIM198, STIM199, STIM2, STIM20, STIM200, STIM201, STIM202, STIM203, STIM204, STIM205, STIM206,
    STIM207, STIM208, STIM209, STIM21, STIM210, STIM211, STIM212, STIM213, STIM214, STIM215,
    STIM216, STIM217, STIM218, STIM219, STIM22, STIM220, STIM221, STIM222, STIM223, STIM224,
    STIM225, STIM226, STIM227, STIM228, STIM229, STIM23, STIM230, STIM231, STIM232, STIM233,
    STIM234, STIM235, STIM236, STIM237, STIM238, STIM239, STIM24, STIM240, STIM241, STIM242,
    STIM243, STIM244, STIM245, STIM246, STIM247, STIM248, STIM249, STIM25, STIM250, STIM251,
    STIM252, STIM253, STIM254, STIM255, STIM26, STIM27, STIM28, STIM29, STIM3, STIM30, STIM31,
    STIM32, STIM33, STIM34, STIM35, STIM36, STIM37, STIM38, STIM39, STIM4, STIM40, STIM41, STIM42,
    STIM43, STIM44, STIM45, STIM46, STIM47, STIM48, STIM49, STIM5, STIM50, STIM51, STIM52, STIM53,
    STIM54, STIM55, STIM56, STIM57, STIM58, STIM59, STIM6, STIM60, STIM61, STIM62, STIM63, STIM64,
    STIM65, STIM66, STIM67, STIM68, STIM69, STIM7, STIM70, STIM71, STIM72, STIM73, STIM74, STIM75,
    STIM76, STIM77, STIM78, STIM79, STIM8, STIM80, STIM81, STIM82, STIM83, STIM84, STIM85, STIM86,
    STIM87, STIM88, STIM89, STIM9, STIM90, STIM91, STIM92, STIM93, STIM94, STIM95, STIM96, STIM97,
    STIM98, STIM99, TCR, TER0, TER1, TER2, TER3, TER4, TER5, TER6, TER7, TPR,
};

/// Access functions for the ITM peripheral instance
pub mod ITM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ITM
    pub const reset: ResetValues = ResetValues {
        STIM0: 0x00000000,
        STIM1: 0x00000000,
        STIM2: 0x00000000,
        STIM3: 0x00000000,
        STIM4: 0x00000000,
        STIM5: 0x00000000,
        STIM6: 0x00000000,
        STIM7: 0x00000000,
        STIM8: 0x00000000,
        STIM9: 0x00000000,
        STIM10: 0x00000000,
        STIM11: 0x00000000,
        STIM12: 0x00000000,
        STIM13: 0x00000000,
        STIM14: 0x00000000,
        STIM15: 0x00000000,
        STIM16: 0x00000000,
        STIM17: 0x00000000,
        STIM18: 0x00000000,
        STIM19: 0x00000000,
        STIM20: 0x00000000,
        STIM21: 0x00000000,
        STIM22: 0x00000000,
        STIM23: 0x00000000,
        STIM24: 0x00000000,
        STIM25: 0x00000000,
        STIM26: 0x00000000,
        STIM27: 0x00000000,
        STIM28: 0x00000000,
        STIM29: 0x00000000,
        STIM30: 0x00000000,
        STIM31: 0x00000000,
        STIM32: 0x00000000,
        STIM33: 0x00000000,
        STIM34: 0x00000000,
        STIM35: 0x00000000,
        STIM36: 0x00000000,
        STIM37: 0x00000000,
        STIM38: 0x00000000,
        STIM39: 0x00000000,
        STIM40: 0x00000000,
        STIM41: 0x00000000,
        STIM42: 0x00000000,
        STIM43: 0x00000000,
        STIM44: 0x00000000,
        STIM45: 0x00000000,
        STIM46: 0x00000000,
        STIM47: 0x00000000,
        STIM48: 0x00000000,
        STIM49: 0x00000000,
        STIM50: 0x00000000,
        STIM51: 0x00000000,
        STIM52: 0x00000000,
        STIM53: 0x00000000,
        STIM54: 0x00000000,
        STIM55: 0x00000000,
        STIM56: 0x00000000,
        STIM57: 0x00000000,
        STIM58: 0x00000000,
        STIM59: 0x00000000,
        STIM60: 0x00000000,
        STIM61: 0x00000000,
        STIM62: 0x00000000,
        STIM63: 0x00000000,
        STIM64: 0x00000000,
        STIM65: 0x00000000,
        STIM66: 0x00000000,
        STIM67: 0x00000000,
        STIM68: 0x00000000,
        STIM69: 0x00000000,
        STIM70: 0x00000000,
        STIM71: 0x00000000,
        STIM72: 0x00000000,
        STIM73: 0x00000000,
        STIM74: 0x00000000,
        STIM75: 0x00000000,
        STIM76: 0x00000000,
        STIM77: 0x00000000,
        STIM78: 0x00000000,
        STIM79: 0x00000000,
        STIM80: 0x00000000,
        STIM81: 0x00000000,
        STIM82: 0x00000000,
        STIM83: 0x00000000,
        STIM84: 0x00000000,
        STIM85: 0x00000000,
        STIM86: 0x00000000,
        STIM87: 0x00000000,
        STIM88: 0x00000000,
        STIM89: 0x00000000,
        STIM90: 0x00000000,
        STIM91: 0x00000000,
        STIM92: 0x00000000,
        STIM93: 0x00000000,
        STIM94: 0x00000000,
        STIM95: 0x00000000,
        STIM96: 0x00000000,
        STIM97: 0x00000000,
        STIM98: 0x00000000,
        STIM99: 0x00000000,
        STIM100: 0x00000000,
        STIM101: 0x00000000,
        STIM102: 0x00000000,
        STIM103: 0x00000000,
        STIM104: 0x00000000,
        STIM105: 0x00000000,
        STIM106: 0x00000000,
        STIM107: 0x00000000,
        STIM108: 0x00000000,
        STIM109: 0x00000000,
        STIM110: 0x00000000,
        STIM111: 0x00000000,
        STIM112: 0x00000000,
        STIM113: 0x00000000,
        STIM114: 0x00000000,
        STIM115: 0x00000000,
        STIM116: 0x00000000,
        STIM117: 0x00000000,
        STIM118: 0x00000000,
        STIM119: 0x00000000,
        STIM120: 0x00000000,
        STIM121: 0x00000000,
        STIM122: 0x00000000,
        STIM123: 0x00000000,
        STIM124: 0x00000000,
        STIM125: 0x00000000,
        STIM126: 0x00000000,
        STIM127: 0x00000000,
        STIM128: 0x00000000,
        STIM129: 0x00000000,
        STIM130: 0x00000000,
        STIM131: 0x00000000,
        STIM132: 0x00000000,
        STIM133: 0x00000000,
        STIM134: 0x00000000,
        STIM135: 0x00000000,
        STIM136: 0x00000000,
        STIM137: 0x00000000,
        STIM138: 0x00000000,
        STIM139: 0x00000000,
        STIM140: 0x00000000,
        STIM141: 0x00000000,
        STIM142: 0x00000000,
        STIM143: 0x00000000,
        STIM144: 0x00000000,
        STIM145: 0x00000000,
        STIM146: 0x00000000,
        STIM147: 0x00000000,
        STIM148: 0x00000000,
        STIM149: 0x00000000,
        STIM150: 0x00000000,
        STIM151: 0x00000000,
        STIM152: 0x00000000,
        STIM153: 0x00000000,
        STIM154: 0x00000000,
        STIM155: 0x00000000,
        STIM156: 0x00000000,
        STIM157: 0x00000000,
        STIM158: 0x00000000,
        STIM159: 0x00000000,
        STIM160: 0x00000000,
        STIM161: 0x00000000,
        STIM162: 0x00000000,
        STIM163: 0x00000000,
        STIM164: 0x00000000,
        STIM165: 0x00000000,
        STIM166: 0x00000000,
        STIM167: 0x00000000,
        STIM168: 0x00000000,
        STIM169: 0x00000000,
        STIM170: 0x00000000,
        STIM171: 0x00000000,
        STIM172: 0x00000000,
        STIM173: 0x00000000,
        STIM174: 0x00000000,
        STIM175: 0x00000000,
        STIM176: 0x00000000,
        STIM177: 0x00000000,
        STIM178: 0x00000000,
        STIM179: 0x00000000,
        STIM180: 0x00000000,
        STIM181: 0x00000000,
        STIM182: 0x00000000,
        STIM183: 0x00000000,
        STIM184: 0x00000000,
        STIM185: 0x00000000,
        STIM186: 0x00000000,
        STIM187: 0x00000000,
        STIM188: 0x00000000,
        STIM189: 0x00000000,
        STIM190: 0x00000000,
        STIM191: 0x00000000,
        STIM192: 0x00000000,
        STIM193: 0x00000000,
        STIM194: 0x00000000,
        STIM195: 0x00000000,
        STIM196: 0x00000000,
        STIM197: 0x00000000,
        STIM198: 0x00000000,
        STIM199: 0x00000000,
        STIM200: 0x00000000,
        STIM201: 0x00000000,
        STIM202: 0x00000000,
        STIM203: 0x00000000,
        STIM204: 0x00000000,
        STIM205: 0x00000000,
        STIM206: 0x00000000,
        STIM207: 0x00000000,
        STIM208: 0x00000000,
        STIM209: 0x00000000,
        STIM210: 0x00000000,
        STIM211: 0x00000000,
        STIM212: 0x00000000,
        STIM213: 0x00000000,
        STIM214: 0x00000000,
        STIM215: 0x00000000,
        STIM216: 0x00000000,
        STIM217: 0x00000000,
        STIM218: 0x00000000,
        STIM219: 0x00000000,
        STIM220: 0x00000000,
        STIM221: 0x00000000,
        STIM222: 0x00000000,
        STIM223: 0x00000000,
        STIM224: 0x00000000,
        STIM225: 0x00000000,
        STIM226: 0x00000000,
        STIM227: 0x00000000,
        STIM228: 0x00000000,
        STIM229: 0x00000000,
        STIM230: 0x00000000,
        STIM231: 0x00000000,
        STIM232: 0x00000000,
        STIM233: 0x00000000,
        STIM234: 0x00000000,
        STIM235: 0x00000000,
        STIM236: 0x00000000,
        STIM237: 0x00000000,
        STIM238: 0x00000000,
        STIM239: 0x00000000,
        STIM240: 0x00000000,
        STIM241: 0x00000000,
        STIM242: 0x00000000,
        STIM243: 0x00000000,
        STIM244: 0x00000000,
        STIM245: 0x00000000,
        STIM246: 0x00000000,
        STIM247: 0x00000000,
        STIM248: 0x00000000,
        STIM249: 0x00000000,
        STIM250: 0x00000000,
        STIM251: 0x00000000,
        STIM252: 0x00000000,
        STIM253: 0x00000000,
        STIM254: 0x00000000,
        STIM255: 0x00000000,
        TER0: 0x00000000,
        TER1: 0x00000000,
        TER2: 0x00000000,
        TER3: 0x00000000,
        TER4: 0x00000000,
        TER5: 0x00000000,
        TER6: 0x00000000,
        TER7: 0x00000000,
        TPR: 0x00000000,
        TCR: 0x00000000,
        LSR: 0x00000000,
        LAR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ITM_TAKEN: bool = false;

    /// Safe access to ITM
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
            if ITM_TAKEN {
                None
            } else {
                ITM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ITM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ITM_TAKEN && inst.addr == INSTANCE.addr {
                ITM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ITM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ITM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ITM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ITM: *const RegisterBlock = 0xe0000000 as *const _;
