#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ADC_ETC
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::adc_etc::Instance;
pub use crate::imxrt106::peripherals::adc_etc::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::adc_etc::{
    CTRL, DMA_CTRL, DONE0_1_IRQ, DONE2_ERR_IRQ, TRIG0_CHAIN_1_0, TRIG0_CHAIN_3_2, TRIG0_CHAIN_5_4,
    TRIG0_CHAIN_7_6, TRIG0_COUNTER, TRIG0_CTRL, TRIG0_RESULT_1_0, TRIG0_RESULT_3_2,
    TRIG0_RESULT_5_4, TRIG0_RESULT_7_6, TRIG1_CHAIN_1_0, TRIG1_CHAIN_3_2, TRIG1_CHAIN_5_4,
    TRIG1_CHAIN_7_6, TRIG1_COUNTER, TRIG1_CTRL, TRIG1_RESULT_1_0, TRIG1_RESULT_3_2,
    TRIG1_RESULT_5_4, TRIG1_RESULT_7_6, TRIG2_CHAIN_1_0, TRIG2_CHAIN_3_2, TRIG2_CHAIN_5_4,
    TRIG2_CHAIN_7_6, TRIG2_COUNTER, TRIG2_CTRL, TRIG2_RESULT_1_0, TRIG2_RESULT_3_2,
    TRIG2_RESULT_5_4, TRIG2_RESULT_7_6, TRIG3_CHAIN_1_0, TRIG3_CHAIN_3_2, TRIG3_CHAIN_5_4,
    TRIG3_CHAIN_7_6, TRIG3_COUNTER, TRIG3_CTRL, TRIG3_RESULT_1_0, TRIG3_RESULT_3_2,
    TRIG3_RESULT_5_4, TRIG3_RESULT_7_6, TRIG4_CHAIN_1_0, TRIG4_CHAIN_3_2, TRIG4_CHAIN_5_4,
    TRIG4_CHAIN_7_6, TRIG4_COUNTER, TRIG4_CTRL, TRIG4_RESULT_1_0, TRIG4_RESULT_3_2,
    TRIG4_RESULT_5_4, TRIG4_RESULT_7_6, TRIG5_CHAIN_1_0, TRIG5_CHAIN_3_2, TRIG5_CHAIN_5_4,
    TRIG5_CHAIN_7_6, TRIG5_COUNTER, TRIG5_CTRL, TRIG5_RESULT_1_0, TRIG5_RESULT_3_2,
    TRIG5_RESULT_5_4, TRIG5_RESULT_7_6, TRIG6_CHAIN_1_0, TRIG6_CHAIN_3_2, TRIG6_CHAIN_5_4,
    TRIG6_CHAIN_7_6, TRIG6_COUNTER, TRIG6_CTRL, TRIG6_RESULT_1_0, TRIG6_RESULT_3_2,
    TRIG6_RESULT_5_4, TRIG6_RESULT_7_6, TRIG7_CHAIN_1_0, TRIG7_CHAIN_3_2, TRIG7_CHAIN_5_4,
    TRIG7_CHAIN_7_6, TRIG7_COUNTER, TRIG7_CTRL, TRIG7_RESULT_1_0, TRIG7_RESULT_3_2,
    TRIG7_RESULT_5_4, TRIG7_RESULT_7_6,
};

/// Access functions for the ADC_ETC peripheral instance
pub mod ADC_ETC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403b0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC_ETC
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        DONE0_1_IRQ: 0x00000000,
        DONE2_ERR_IRQ: 0x00000000,
        DMA_CTRL: 0x00000000,
        TRIG0_CTRL: 0x00000000,
        TRIG0_COUNTER: 0x00000000,
        TRIG0_CHAIN_1_0: 0x00000000,
        TRIG0_CHAIN_3_2: 0x00000000,
        TRIG0_CHAIN_5_4: 0x00000000,
        TRIG0_CHAIN_7_6: 0x00000000,
        TRIG0_RESULT_1_0: 0x00000000,
        TRIG0_RESULT_3_2: 0x00000000,
        TRIG0_RESULT_5_4: 0x00000000,
        TRIG0_RESULT_7_6: 0x00000000,
        TRIG1_CTRL: 0x00000000,
        TRIG1_COUNTER: 0x00000000,
        TRIG1_CHAIN_1_0: 0x00000000,
        TRIG1_CHAIN_3_2: 0x00000000,
        TRIG1_CHAIN_5_4: 0x00000000,
        TRIG1_CHAIN_7_6: 0x00000000,
        TRIG1_RESULT_1_0: 0x00000000,
        TRIG1_RESULT_3_2: 0x00000000,
        TRIG1_RESULT_5_4: 0x00000000,
        TRIG1_RESULT_7_6: 0x00000000,
        TRIG2_CTRL: 0x00000000,
        TRIG2_COUNTER: 0x00000000,
        TRIG2_CHAIN_1_0: 0x00000000,
        TRIG2_CHAIN_3_2: 0x00000000,
        TRIG2_CHAIN_5_4: 0x00000000,
        TRIG2_CHAIN_7_6: 0x00000000,
        TRIG2_RESULT_1_0: 0x00000000,
        TRIG2_RESULT_3_2: 0x00000000,
        TRIG2_RESULT_5_4: 0x00000000,
        TRIG2_RESULT_7_6: 0x00000000,
        TRIG3_CTRL: 0x00000000,
        TRIG3_COUNTER: 0x00000000,
        TRIG3_CHAIN_1_0: 0x00000000,
        TRIG3_CHAIN_3_2: 0x00000000,
        TRIG3_CHAIN_5_4: 0x00000000,
        TRIG3_CHAIN_7_6: 0x00000000,
        TRIG3_RESULT_1_0: 0x00000000,
        TRIG3_RESULT_3_2: 0x00000000,
        TRIG3_RESULT_5_4: 0x00000000,
        TRIG3_RESULT_7_6: 0x00000000,
        TRIG4_CTRL: 0x00000000,
        TRIG4_COUNTER: 0x00000000,
        TRIG4_CHAIN_1_0: 0x00000000,
        TRIG4_CHAIN_3_2: 0x00000000,
        TRIG4_CHAIN_5_4: 0x00000000,
        TRIG4_CHAIN_7_6: 0x00000000,
        TRIG4_RESULT_1_0: 0x00000000,
        TRIG4_RESULT_3_2: 0x00000000,
        TRIG4_RESULT_5_4: 0x00000000,
        TRIG4_RESULT_7_6: 0x00000000,
        TRIG5_CTRL: 0x00000000,
        TRIG5_COUNTER: 0x00000000,
        TRIG5_CHAIN_1_0: 0x00000000,
        TRIG5_CHAIN_3_2: 0x00000000,
        TRIG5_CHAIN_5_4: 0x00000000,
        TRIG5_CHAIN_7_6: 0x00000000,
        TRIG5_RESULT_1_0: 0x00000000,
        TRIG5_RESULT_3_2: 0x00000000,
        TRIG5_RESULT_5_4: 0x00000000,
        TRIG5_RESULT_7_6: 0x00000000,
        TRIG6_CTRL: 0x00000000,
        TRIG6_COUNTER: 0x00000000,
        TRIG6_CHAIN_1_0: 0x00000000,
        TRIG6_CHAIN_3_2: 0x00000000,
        TRIG6_CHAIN_5_4: 0x00000000,
        TRIG6_CHAIN_7_6: 0x00000000,
        TRIG6_RESULT_1_0: 0x00000000,
        TRIG6_RESULT_3_2: 0x00000000,
        TRIG6_RESULT_5_4: 0x00000000,
        TRIG6_RESULT_7_6: 0x00000000,
        TRIG7_CTRL: 0x00000000,
        TRIG7_COUNTER: 0x00000000,
        TRIG7_CHAIN_1_0: 0x00000000,
        TRIG7_CHAIN_3_2: 0x00000000,
        TRIG7_CHAIN_5_4: 0x00000000,
        TRIG7_CHAIN_7_6: 0x00000000,
        TRIG7_RESULT_1_0: 0x00000000,
        TRIG7_RESULT_3_2: 0x00000000,
        TRIG7_RESULT_5_4: 0x00000000,
        TRIG7_RESULT_7_6: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC_ETC_TAKEN: bool = false;

    /// Safe access to ADC_ETC
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
            if ADC_ETC_TAKEN {
                None
            } else {
                ADC_ETC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC_ETC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC_ETC_TAKEN && inst.addr == INSTANCE.addr {
                ADC_ETC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC_ETC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC_ETC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC_ETC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC_ETC: *const RegisterBlock = 0x403b0000 as *const _;
