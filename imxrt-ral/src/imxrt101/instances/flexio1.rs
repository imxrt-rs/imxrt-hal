#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXIO
//!
//! Used by: imxrt1011, imxrt1015

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::flexio1::Instance;
pub use crate::imxrt101::peripherals::flexio1::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::flexio1::{
    CTRL, PARAM, PIN, SHIFTBUF0, SHIFTBUF1, SHIFTBUF2, SHIFTBUF3, SHIFTBUF4, SHIFTBUF5, SHIFTBUF6,
    SHIFTBUF7, SHIFTBUFBBS0, SHIFTBUFBBS1, SHIFTBUFBBS2, SHIFTBUFBBS3, SHIFTBUFBBS4, SHIFTBUFBBS5,
    SHIFTBUFBBS6, SHIFTBUFBBS7, SHIFTBUFBIS0, SHIFTBUFBIS1, SHIFTBUFBIS2, SHIFTBUFBIS3,
    SHIFTBUFBIS4, SHIFTBUFBIS5, SHIFTBUFBIS6, SHIFTBUFBIS7, SHIFTBUFBYS0, SHIFTBUFBYS1,
    SHIFTBUFBYS2, SHIFTBUFBYS3, SHIFTBUFBYS4, SHIFTBUFBYS5, SHIFTBUFBYS6, SHIFTBUFBYS7,
    SHIFTBUFHWS0, SHIFTBUFHWS1, SHIFTBUFHWS2, SHIFTBUFHWS3, SHIFTBUFHWS4, SHIFTBUFHWS5,
    SHIFTBUFHWS6, SHIFTBUFHWS7, SHIFTBUFNBS0, SHIFTBUFNBS1, SHIFTBUFNBS2, SHIFTBUFNBS3,
    SHIFTBUFNBS4, SHIFTBUFNBS5, SHIFTBUFNBS6, SHIFTBUFNBS7, SHIFTBUFNIS0, SHIFTBUFNIS1,
    SHIFTBUFNIS2, SHIFTBUFNIS3, SHIFTBUFNIS4, SHIFTBUFNIS5, SHIFTBUFNIS6, SHIFTBUFNIS7, SHIFTCFG0,
    SHIFTCFG1, SHIFTCFG2, SHIFTCFG3, SHIFTCFG4, SHIFTCFG5, SHIFTCFG6, SHIFTCFG7, SHIFTCTL0,
    SHIFTCTL1, SHIFTCTL2, SHIFTCTL3, SHIFTCTL4, SHIFTCTL5, SHIFTCTL6, SHIFTCTL7, SHIFTEIEN,
    SHIFTERR, SHIFTSDEN, SHIFTSIEN, SHIFTSTAT, SHIFTSTATE, TIMCFG0, TIMCFG1, TIMCFG2, TIMCFG3,
    TIMCFG4, TIMCFG5, TIMCFG6, TIMCFG7, TIMCMP0, TIMCMP1, TIMCMP2, TIMCMP3, TIMCMP4, TIMCMP5,
    TIMCMP6, TIMCMP7, TIMCTL0, TIMCTL1, TIMCTL2, TIMCTL3, TIMCTL4, TIMCTL5, TIMCTL6, TIMCTL7,
    TIMIEN, TIMSTAT, VERID,
};

/// Access functions for the FLEXIO1 peripheral instance
pub mod FLEXIO1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401ac000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLEXIO1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01010001,
        PARAM: 0x02200808,
        CTRL: 0x00000000,
        PIN: 0x00000000,
        SHIFTSTAT: 0x00000000,
        SHIFTERR: 0x00000000,
        TIMSTAT: 0x00000000,
        SHIFTSIEN: 0x00000000,
        SHIFTEIEN: 0x00000000,
        TIMIEN: 0x00000000,
        SHIFTSDEN: 0x00000000,
        SHIFTSTATE: 0x00000000,
        SHIFTCTL0: 0x00000000,
        SHIFTCTL1: 0x00000000,
        SHIFTCTL2: 0x00000000,
        SHIFTCTL3: 0x00000000,
        SHIFTCTL4: 0x00000000,
        SHIFTCTL5: 0x00000000,
        SHIFTCTL6: 0x00000000,
        SHIFTCTL7: 0x00000000,
        SHIFTCFG0: 0x00000000,
        SHIFTCFG1: 0x00000000,
        SHIFTCFG2: 0x00000000,
        SHIFTCFG3: 0x00000000,
        SHIFTCFG4: 0x00000000,
        SHIFTCFG5: 0x00000000,
        SHIFTCFG6: 0x00000000,
        SHIFTCFG7: 0x00000000,
        SHIFTBUF0: 0x00000000,
        SHIFTBUF1: 0x00000000,
        SHIFTBUF2: 0x00000000,
        SHIFTBUF3: 0x00000000,
        SHIFTBUF4: 0x00000000,
        SHIFTBUF5: 0x00000000,
        SHIFTBUF6: 0x00000000,
        SHIFTBUF7: 0x00000000,
        SHIFTBUFBIS0: 0x00000000,
        SHIFTBUFBIS1: 0x00000000,
        SHIFTBUFBIS2: 0x00000000,
        SHIFTBUFBIS3: 0x00000000,
        SHIFTBUFBIS4: 0x00000000,
        SHIFTBUFBIS5: 0x00000000,
        SHIFTBUFBIS6: 0x00000000,
        SHIFTBUFBIS7: 0x00000000,
        SHIFTBUFBYS0: 0x00000000,
        SHIFTBUFBYS1: 0x00000000,
        SHIFTBUFBYS2: 0x00000000,
        SHIFTBUFBYS3: 0x00000000,
        SHIFTBUFBYS4: 0x00000000,
        SHIFTBUFBYS5: 0x00000000,
        SHIFTBUFBYS6: 0x00000000,
        SHIFTBUFBYS7: 0x00000000,
        SHIFTBUFBBS0: 0x00000000,
        SHIFTBUFBBS1: 0x00000000,
        SHIFTBUFBBS2: 0x00000000,
        SHIFTBUFBBS3: 0x00000000,
        SHIFTBUFBBS4: 0x00000000,
        SHIFTBUFBBS5: 0x00000000,
        SHIFTBUFBBS6: 0x00000000,
        SHIFTBUFBBS7: 0x00000000,
        TIMCTL0: 0x00000000,
        TIMCTL1: 0x00000000,
        TIMCTL2: 0x00000000,
        TIMCTL3: 0x00000000,
        TIMCTL4: 0x00000000,
        TIMCTL5: 0x00000000,
        TIMCTL6: 0x00000000,
        TIMCTL7: 0x00000000,
        TIMCFG0: 0x00000000,
        TIMCFG1: 0x00000000,
        TIMCFG2: 0x00000000,
        TIMCFG3: 0x00000000,
        TIMCFG4: 0x00000000,
        TIMCFG5: 0x00000000,
        TIMCFG6: 0x00000000,
        TIMCFG7: 0x00000000,
        TIMCMP0: 0x00000000,
        TIMCMP1: 0x00000000,
        TIMCMP2: 0x00000000,
        TIMCMP3: 0x00000000,
        TIMCMP4: 0x00000000,
        TIMCMP5: 0x00000000,
        TIMCMP6: 0x00000000,
        TIMCMP7: 0x00000000,
        SHIFTBUFNBS0: 0x00000000,
        SHIFTBUFNBS1: 0x00000000,
        SHIFTBUFNBS2: 0x00000000,
        SHIFTBUFNBS3: 0x00000000,
        SHIFTBUFNBS4: 0x00000000,
        SHIFTBUFNBS5: 0x00000000,
        SHIFTBUFNBS6: 0x00000000,
        SHIFTBUFNBS7: 0x00000000,
        SHIFTBUFHWS0: 0x00000000,
        SHIFTBUFHWS1: 0x00000000,
        SHIFTBUFHWS2: 0x00000000,
        SHIFTBUFHWS3: 0x00000000,
        SHIFTBUFHWS4: 0x00000000,
        SHIFTBUFHWS5: 0x00000000,
        SHIFTBUFHWS6: 0x00000000,
        SHIFTBUFHWS7: 0x00000000,
        SHIFTBUFNIS0: 0x00000000,
        SHIFTBUFNIS1: 0x00000000,
        SHIFTBUFNIS2: 0x00000000,
        SHIFTBUFNIS3: 0x00000000,
        SHIFTBUFNIS4: 0x00000000,
        SHIFTBUFNIS5: 0x00000000,
        SHIFTBUFNIS6: 0x00000000,
        SHIFTBUFNIS7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FLEXIO1_TAKEN: bool = false;

    /// Safe access to FLEXIO1
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
            if FLEXIO1_TAKEN {
                None
            } else {
                FLEXIO1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FLEXIO1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FLEXIO1_TAKEN && inst.addr == INSTANCE.addr {
                FLEXIO1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FLEXIO1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FLEXIO1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FLEXIO1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXIO1: *const RegisterBlock = 0x401ac000 as *const _;
