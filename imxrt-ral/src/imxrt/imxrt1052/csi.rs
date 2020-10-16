#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CSI

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt::peripherals::csi::Instance;
pub use crate::imxrt::peripherals::csi::{RegisterBlock, ResetValues};
pub use crate::imxrt::peripherals::csi::{
    CSICR1, CSICR18, CSICR19, CSICR2, CSICR3, CSIDMASA_FB1, CSIDMASA_FB2, CSIDMASA_STATFIFO,
    CSIDMATS_STATFIFO, CSIFBUF_PARA, CSIIMAG_PARA, CSIRFIFO, CSIRXCNT, CSISR, CSISTATFIFO,
};

/// Access functions for the CSI peripheral instance
pub mod CSI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402bc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CSI
    pub const reset: ResetValues = ResetValues {
        CSICR1: 0x40000800,
        CSICR2: 0x00000000,
        CSICR3: 0x00000000,
        CSISTATFIFO: 0x00000000,
        CSIRFIFO: 0x00000000,
        CSIRXCNT: 0x00009600,
        CSISR: 0x00004000,
        CSIDMASA_STATFIFO: 0x00000000,
        CSIDMATS_STATFIFO: 0x00000000,
        CSIDMASA_FB1: 0x00000000,
        CSIDMASA_FB2: 0x00000000,
        CSIFBUF_PARA: 0x00000000,
        CSIIMAG_PARA: 0x00000000,
        CSICR18: 0x0002D000,
        CSICR19: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CSI_TAKEN: bool = false;

    /// Safe access to CSI
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
            if CSI_TAKEN {
                None
            } else {
                CSI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CSI_TAKEN && inst.addr == INSTANCE.addr {
                CSI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CSI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CSI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CSI: *const RegisterBlock = 0x402bc000 as *const _;
