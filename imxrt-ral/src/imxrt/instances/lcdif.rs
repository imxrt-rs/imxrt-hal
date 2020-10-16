#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCDIF Register Reference Index
//!
//! Used by: imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::lcdif::Instance;
pub use crate::imxrt::peripherals::lcdif::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::lcdif::{
    BM_ERROR_STAT, CRC_STAT, CTRL, CTRL1, CTRL1_CLR, CTRL1_SET, CTRL1_TOG, CTRL2, CTRL2_CLR,
    CTRL2_SET, CTRL2_TOG, CTRL_CLR, CTRL_SET, CTRL_TOG, CUR_BUF, LUT0_ADDR, LUT0_DATA, LUT1_ADDR,
    LUT1_DATA, LUT_CTRL, NEXT_BUF, PIGEONCTRL0, PIGEONCTRL0_CLR, PIGEONCTRL0_SET, PIGEONCTRL0_TOG,
    PIGEONCTRL1, PIGEONCTRL1_CLR, PIGEONCTRL1_SET, PIGEONCTRL1_TOG, PIGEONCTRL2, PIGEONCTRL2_CLR,
    PIGEONCTRL2_SET, PIGEONCTRL2_TOG, PIGEON_0_0, PIGEON_0_1, PIGEON_0_2, PIGEON_10_0, PIGEON_10_1,
    PIGEON_10_2, PIGEON_11_0, PIGEON_11_1, PIGEON_11_2, PIGEON_1_0, PIGEON_1_1, PIGEON_1_2,
    PIGEON_2_0, PIGEON_2_1, PIGEON_2_2, PIGEON_3_0, PIGEON_3_1, PIGEON_3_2, PIGEON_4_0, PIGEON_4_1,
    PIGEON_4_2, PIGEON_5_0, PIGEON_5_1, PIGEON_5_2, PIGEON_6_0, PIGEON_6_1, PIGEON_6_2, PIGEON_7_0,
    PIGEON_7_1, PIGEON_7_2, PIGEON_8_0, PIGEON_8_1, PIGEON_8_2, PIGEON_9_0, PIGEON_9_1, PIGEON_9_2,
    STAT, TRANSFER_COUNT, VDCTRL0, VDCTRL0_CLR, VDCTRL0_SET, VDCTRL0_TOG, VDCTRL1, VDCTRL2,
    VDCTRL3, VDCTRL4,
};

/// Access functions for the LCDIF peripheral instance
pub mod LCDIF {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402b8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LCDIF
    pub const reset: ResetValues = ResetValues {
        CTRL: 0xC0000000,
        CTRL_SET: 0xC0000000,
        CTRL_CLR: 0xC0000000,
        CTRL_TOG: 0xC0000000,
        CTRL1: 0x000F0000,
        CTRL1_SET: 0x000F0000,
        CTRL1_CLR: 0x000F0000,
        CTRL1_TOG: 0x000F0000,
        CTRL2: 0x00200000,
        CTRL2_SET: 0x00200000,
        CTRL2_CLR: 0x00200000,
        CTRL2_TOG: 0x00200000,
        TRANSFER_COUNT: 0x00010000,
        CUR_BUF: 0x00000000,
        NEXT_BUF: 0x00000000,
        VDCTRL0: 0x00000000,
        VDCTRL0_SET: 0x00000000,
        VDCTRL0_CLR: 0x00000000,
        VDCTRL0_TOG: 0x00000000,
        VDCTRL1: 0x00000000,
        VDCTRL2: 0x00000000,
        VDCTRL3: 0x00000000,
        VDCTRL4: 0x00000000,
        BM_ERROR_STAT: 0x00000000,
        CRC_STAT: 0x00000000,
        STAT: 0x95000000,
        PIGEONCTRL0: 0x00000000,
        PIGEONCTRL0_SET: 0x00000000,
        PIGEONCTRL0_CLR: 0x00000000,
        PIGEONCTRL0_TOG: 0x00000000,
        PIGEONCTRL1: 0x00000000,
        PIGEONCTRL1_SET: 0x00000000,
        PIGEONCTRL1_CLR: 0x00000000,
        PIGEONCTRL1_TOG: 0x00000000,
        PIGEONCTRL2: 0x00000000,
        PIGEONCTRL2_SET: 0x00000000,
        PIGEONCTRL2_CLR: 0x00000000,
        PIGEONCTRL2_TOG: 0x00000000,
        PIGEON_0_0: 0x00000000,
        PIGEON_0_1: 0x00000000,
        PIGEON_0_2: 0x00000000,
        PIGEON_1_0: 0x00000000,
        PIGEON_1_1: 0x00000000,
        PIGEON_1_2: 0x00000000,
        PIGEON_2_0: 0x00000000,
        PIGEON_2_1: 0x00000000,
        PIGEON_2_2: 0x00000000,
        PIGEON_3_0: 0x00000000,
        PIGEON_3_1: 0x00000000,
        PIGEON_3_2: 0x00000000,
        PIGEON_4_0: 0x00000000,
        PIGEON_4_1: 0x00000000,
        PIGEON_4_2: 0x00000000,
        PIGEON_5_0: 0x00000000,
        PIGEON_5_1: 0x00000000,
        PIGEON_5_2: 0x00000000,
        PIGEON_6_0: 0x00000000,
        PIGEON_6_1: 0x00000000,
        PIGEON_6_2: 0x00000000,
        PIGEON_7_0: 0x00000000,
        PIGEON_7_1: 0x00000000,
        PIGEON_7_2: 0x00000000,
        PIGEON_8_0: 0x00000000,
        PIGEON_8_1: 0x00000000,
        PIGEON_8_2: 0x00000000,
        PIGEON_9_0: 0x00000000,
        PIGEON_9_1: 0x00000000,
        PIGEON_9_2: 0x00000000,
        PIGEON_10_0: 0x00000000,
        PIGEON_10_1: 0x00000000,
        PIGEON_10_2: 0x00000000,
        PIGEON_11_0: 0x00000000,
        PIGEON_11_1: 0x00000000,
        PIGEON_11_2: 0x00000000,
        LUT_CTRL: 0x00000001,
        LUT0_ADDR: 0x00000000,
        LUT0_DATA: 0x00000000,
        LUT1_ADDR: 0x00000000,
        LUT1_DATA: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LCDIF_TAKEN: bool = false;

    /// Safe access to LCDIF
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
            if LCDIF_TAKEN {
                None
            } else {
                LCDIF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LCDIF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LCDIF_TAKEN && inst.addr == INSTANCE.addr {
                LCDIF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LCDIF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LCDIF_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LCDIF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LCDIF: *const RegisterBlock = 0x402b8000 as *const _;
