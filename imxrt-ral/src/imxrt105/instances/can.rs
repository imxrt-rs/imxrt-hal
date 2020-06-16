#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXCAN
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::can::Instance;
pub use crate::imxrt105::peripherals::can::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::can::{
    CRCR, CS0, CS1, CS10, CS11, CS12, CS13, CS14, CS15, CS16, CS17, CS18, CS19, CS2, CS20, CS21,
    CS22, CS23, CS24, CS25, CS26, CS27, CS28, CS29, CS3, CS30, CS31, CS32, CS33, CS34, CS35, CS36,
    CS37, CS38, CS39, CS4, CS40, CS41, CS42, CS43, CS44, CS45, CS46, CS47, CS48, CS49, CS5, CS50,
    CS51, CS52, CS53, CS54, CS55, CS56, CS57, CS58, CS59, CS6, CS60, CS61, CS62, CS63, CS7, CS8,
    CS9, CTRL1, CTRL2, DBG1, DBG2, ECR, ESR1, ESR2, GFWR, ID0, ID1, ID10, ID11, ID12, ID13, ID14,
    ID15, ID16, ID17, ID18, ID19, ID2, ID20, ID21, ID22, ID23, ID24, ID25, ID26, ID27, ID28, ID29,
    ID3, ID30, ID31, ID32, ID33, ID34, ID35, ID36, ID37, ID38, ID39, ID4, ID40, ID41, ID42, ID43,
    ID44, ID45, ID46, ID47, ID48, ID49, ID5, ID50, ID51, ID52, ID53, ID54, ID55, ID56, ID57, ID58,
    ID59, ID6, ID60, ID61, ID62, ID63, ID7, ID8, ID9, IFLAG1, IFLAG2, IMASK1, IMASK2, MCR,
    RX14MASK, RX15MASK, RXFGMASK, RXFIR, RXIMR0, RXIMR1, RXIMR10, RXIMR11, RXIMR12, RXIMR13,
    RXIMR14, RXIMR15, RXIMR16, RXIMR17, RXIMR18, RXIMR19, RXIMR2, RXIMR20, RXIMR21, RXIMR22,
    RXIMR23, RXIMR24, RXIMR25, RXIMR26, RXIMR27, RXIMR28, RXIMR29, RXIMR3, RXIMR30, RXIMR31,
    RXIMR32, RXIMR33, RXIMR34, RXIMR35, RXIMR36, RXIMR37, RXIMR38, RXIMR39, RXIMR4, RXIMR40,
    RXIMR41, RXIMR42, RXIMR43, RXIMR44, RXIMR45, RXIMR46, RXIMR47, RXIMR48, RXIMR49, RXIMR5,
    RXIMR50, RXIMR51, RXIMR52, RXIMR53, RXIMR54, RXIMR55, RXIMR56, RXIMR57, RXIMR58, RXIMR59,
    RXIMR6, RXIMR60, RXIMR61, RXIMR62, RXIMR63, RXIMR7, RXIMR8, RXIMR9, RXMGMASK, TIMER, WORD00,
    WORD01, WORD010, WORD011, WORD012, WORD013, WORD014, WORD015, WORD016, WORD017, WORD018,
    WORD019, WORD02, WORD020, WORD021, WORD022, WORD023, WORD024, WORD025, WORD026, WORD027,
    WORD028, WORD029, WORD03, WORD030, WORD031, WORD032, WORD033, WORD034, WORD035, WORD036,
    WORD037, WORD038, WORD039, WORD04, WORD040, WORD041, WORD042, WORD043, WORD044, WORD045,
    WORD046, WORD047, WORD048, WORD049, WORD05, WORD050, WORD051, WORD052, WORD053, WORD054,
    WORD055, WORD056, WORD057, WORD058, WORD059, WORD06, WORD060, WORD061, WORD062, WORD063,
    WORD07, WORD08, WORD09, WORD10, WORD11, WORD110, WORD111, WORD112, WORD113, WORD114, WORD115,
    WORD116, WORD117, WORD118, WORD119, WORD12, WORD120, WORD121, WORD122, WORD123, WORD124,
    WORD125, WORD126, WORD127, WORD128, WORD129, WORD13, WORD130, WORD131, WORD132, WORD133,
    WORD134, WORD135, WORD136, WORD137, WORD138, WORD139, WORD14, WORD140, WORD141, WORD142,
    WORD143, WORD144, WORD145, WORD146, WORD147, WORD148, WORD149, WORD15, WORD150, WORD151,
    WORD152, WORD153, WORD154, WORD155, WORD156, WORD157, WORD158, WORD159, WORD16, WORD160,
    WORD161, WORD162, WORD163, WORD17, WORD18, WORD19,
};

/// Access functions for the CAN1 peripheral instance
pub mod CAN1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401d0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN1
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0xFFFFFFFF,
        RX14MASK: 0xFFFFFFFF,
        RX15MASK: 0xFFFFFFFF,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00000000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0xFFFFFFFF,
        RXFIR: 0x00000000,
        DBG1: 0x00010000,
        DBG2: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        WORD00: 0x00000000,
        WORD10: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        WORD01: 0x00000000,
        WORD11: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        WORD02: 0x00000000,
        WORD12: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        WORD03: 0x00000000,
        WORD13: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        WORD04: 0x00000000,
        WORD14: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        WORD05: 0x00000000,
        WORD15: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        WORD06: 0x00000000,
        WORD16: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        WORD07: 0x00000000,
        WORD17: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        WORD08: 0x00000000,
        WORD18: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        WORD09: 0x00000000,
        WORD19: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        WORD010: 0x00000000,
        WORD110: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        WORD011: 0x00000000,
        WORD111: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        WORD012: 0x00000000,
        WORD112: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        WORD013: 0x00000000,
        WORD113: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        WORD014: 0x00000000,
        WORD114: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        WORD015: 0x00000000,
        WORD115: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        WORD016: 0x00000000,
        WORD116: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        WORD017: 0x00000000,
        WORD117: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        WORD018: 0x00000000,
        WORD118: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        WORD019: 0x00000000,
        WORD119: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        WORD020: 0x00000000,
        WORD120: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        WORD021: 0x00000000,
        WORD121: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        WORD022: 0x00000000,
        WORD122: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        WORD023: 0x00000000,
        WORD123: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        WORD024: 0x00000000,
        WORD124: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        WORD025: 0x00000000,
        WORD125: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        WORD026: 0x00000000,
        WORD126: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        WORD027: 0x00000000,
        WORD127: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        WORD028: 0x00000000,
        WORD128: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        WORD029: 0x00000000,
        WORD129: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        WORD030: 0x00000000,
        WORD130: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        WORD031: 0x00000000,
        WORD131: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        WORD032: 0x00000000,
        WORD132: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        WORD033: 0x00000000,
        WORD133: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        WORD034: 0x00000000,
        WORD134: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        WORD035: 0x00000000,
        WORD135: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        WORD036: 0x00000000,
        WORD136: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        WORD037: 0x00000000,
        WORD137: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        WORD038: 0x00000000,
        WORD138: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        WORD039: 0x00000000,
        WORD139: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        WORD040: 0x00000000,
        WORD140: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        WORD041: 0x00000000,
        WORD141: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        WORD042: 0x00000000,
        WORD142: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        WORD043: 0x00000000,
        WORD143: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        WORD044: 0x00000000,
        WORD144: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        WORD045: 0x00000000,
        WORD145: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        WORD046: 0x00000000,
        WORD146: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        WORD047: 0x00000000,
        WORD147: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        WORD048: 0x00000000,
        WORD148: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        WORD049: 0x00000000,
        WORD149: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        WORD050: 0x00000000,
        WORD150: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        WORD051: 0x00000000,
        WORD151: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        WORD052: 0x00000000,
        WORD152: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        WORD053: 0x00000000,
        WORD153: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        WORD054: 0x00000000,
        WORD154: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        WORD055: 0x00000000,
        WORD155: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        WORD056: 0x00000000,
        WORD156: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        WORD057: 0x00000000,
        WORD157: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        WORD058: 0x00000000,
        WORD158: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        WORD059: 0x00000000,
        WORD159: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        WORD060: 0x00000000,
        WORD160: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        WORD061: 0x00000000,
        WORD161: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        WORD062: 0x00000000,
        WORD162: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        WORD063: 0x00000000,
        WORD163: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        GFWR: 0x0000007F,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN1_TAKEN: bool = false;

    /// Safe access to CAN1
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
            if CAN1_TAKEN {
                None
            } else {
                CAN1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN1_TAKEN && inst.addr == INSTANCE.addr {
                CAN1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1: *const RegisterBlock = 0x401d0000 as *const _;

/// Access functions for the CAN2 peripheral instance
pub mod CAN2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401d4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN2
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0xFFFFFFFF,
        RX14MASK: 0xFFFFFFFF,
        RX15MASK: 0xFFFFFFFF,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00000000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0xFFFFFFFF,
        RXFIR: 0x00000000,
        DBG1: 0x00010000,
        DBG2: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        WORD00: 0x00000000,
        WORD10: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        WORD01: 0x00000000,
        WORD11: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        WORD02: 0x00000000,
        WORD12: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        WORD03: 0x00000000,
        WORD13: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        WORD04: 0x00000000,
        WORD14: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        WORD05: 0x00000000,
        WORD15: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        WORD06: 0x00000000,
        WORD16: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        WORD07: 0x00000000,
        WORD17: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        WORD08: 0x00000000,
        WORD18: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        WORD09: 0x00000000,
        WORD19: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        WORD010: 0x00000000,
        WORD110: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        WORD011: 0x00000000,
        WORD111: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        WORD012: 0x00000000,
        WORD112: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        WORD013: 0x00000000,
        WORD113: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        WORD014: 0x00000000,
        WORD114: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        WORD015: 0x00000000,
        WORD115: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        WORD016: 0x00000000,
        WORD116: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        WORD017: 0x00000000,
        WORD117: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        WORD018: 0x00000000,
        WORD118: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        WORD019: 0x00000000,
        WORD119: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        WORD020: 0x00000000,
        WORD120: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        WORD021: 0x00000000,
        WORD121: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        WORD022: 0x00000000,
        WORD122: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        WORD023: 0x00000000,
        WORD123: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        WORD024: 0x00000000,
        WORD124: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        WORD025: 0x00000000,
        WORD125: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        WORD026: 0x00000000,
        WORD126: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        WORD027: 0x00000000,
        WORD127: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        WORD028: 0x00000000,
        WORD128: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        WORD029: 0x00000000,
        WORD129: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        WORD030: 0x00000000,
        WORD130: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        WORD031: 0x00000000,
        WORD131: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        WORD032: 0x00000000,
        WORD132: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        WORD033: 0x00000000,
        WORD133: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        WORD034: 0x00000000,
        WORD134: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        WORD035: 0x00000000,
        WORD135: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        WORD036: 0x00000000,
        WORD136: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        WORD037: 0x00000000,
        WORD137: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        WORD038: 0x00000000,
        WORD138: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        WORD039: 0x00000000,
        WORD139: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        WORD040: 0x00000000,
        WORD140: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        WORD041: 0x00000000,
        WORD141: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        WORD042: 0x00000000,
        WORD142: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        WORD043: 0x00000000,
        WORD143: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        WORD044: 0x00000000,
        WORD144: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        WORD045: 0x00000000,
        WORD145: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        WORD046: 0x00000000,
        WORD146: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        WORD047: 0x00000000,
        WORD147: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        WORD048: 0x00000000,
        WORD148: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        WORD049: 0x00000000,
        WORD149: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        WORD050: 0x00000000,
        WORD150: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        WORD051: 0x00000000,
        WORD151: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        WORD052: 0x00000000,
        WORD152: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        WORD053: 0x00000000,
        WORD153: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        WORD054: 0x00000000,
        WORD154: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        WORD055: 0x00000000,
        WORD155: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        WORD056: 0x00000000,
        WORD156: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        WORD057: 0x00000000,
        WORD157: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        WORD058: 0x00000000,
        WORD158: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        WORD059: 0x00000000,
        WORD159: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        WORD060: 0x00000000,
        WORD160: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        WORD061: 0x00000000,
        WORD161: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        WORD062: 0x00000000,
        WORD162: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        WORD063: 0x00000000,
        WORD163: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        GFWR: 0x0000007F,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN2_TAKEN: bool = false;

    /// Safe access to CAN2
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
            if CAN2_TAKEN {
                None
            } else {
                CAN2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN2_TAKEN && inst.addr == INSTANCE.addr {
                CAN2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CAN2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CAN2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN2: *const RegisterBlock = 0x401d4000 as *const _;
