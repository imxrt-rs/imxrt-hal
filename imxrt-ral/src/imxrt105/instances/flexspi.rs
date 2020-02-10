#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FlexSPI
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::flexspi::Instance;
pub use crate::imxrt105::peripherals::flexspi::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::flexspi::{
    AHBCR, AHBRXBUF0CR0, AHBRXBUF1CR0, AHBRXBUF2CR0, AHBRXBUF3CR0, AHBSPNDSTS, DLLCRA, DLLCRB,
    FLSHA1CR0, FLSHA2CR0, FLSHB1CR0, FLSHB2CR0, FLSHCR1A1, FLSHCR1A2, FLSHCR1B1, FLSHCR1B2,
    FLSHCR2A1, FLSHCR2A2, FLSHCR2B1, FLSHCR2B2, FLSHCR4, INTEN, INTR, IPCMD, IPCR0, IPCR1, IPRXFCR,
    IPRXFSTS, IPTXFCR, IPTXFSTS, LUT0, LUT1, LUT10, LUT11, LUT12, LUT13, LUT14, LUT15, LUT16,
    LUT17, LUT18, LUT19, LUT2, LUT20, LUT21, LUT22, LUT23, LUT24, LUT25, LUT26, LUT27, LUT28,
    LUT29, LUT3, LUT30, LUT31, LUT32, LUT33, LUT34, LUT35, LUT36, LUT37, LUT38, LUT39, LUT4, LUT40,
    LUT41, LUT42, LUT43, LUT44, LUT45, LUT46, LUT47, LUT48, LUT49, LUT5, LUT50, LUT51, LUT52,
    LUT53, LUT54, LUT55, LUT56, LUT57, LUT58, LUT59, LUT6, LUT60, LUT61, LUT62, LUT63, LUT7, LUT8,
    LUT9, LUTCR, LUTKEY, MCR0, MCR1, MCR2, RFDR0, RFDR1, RFDR10, RFDR11, RFDR12, RFDR13, RFDR14,
    RFDR15, RFDR16, RFDR17, RFDR18, RFDR19, RFDR2, RFDR20, RFDR21, RFDR22, RFDR23, RFDR24, RFDR25,
    RFDR26, RFDR27, RFDR28, RFDR29, RFDR3, RFDR30, RFDR31, RFDR4, RFDR5, RFDR6, RFDR7, RFDR8,
    RFDR9, STS0, STS1, STS2, TFDR0, TFDR1, TFDR10, TFDR11, TFDR12, TFDR13, TFDR14, TFDR15, TFDR16,
    TFDR17, TFDR18, TFDR19, TFDR2, TFDR20, TFDR21, TFDR22, TFDR23, TFDR24, TFDR25, TFDR26, TFDR27,
    TFDR28, TFDR29, TFDR3, TFDR30, TFDR31, TFDR4, TFDR5, TFDR6, TFDR7, TFDR8, TFDR9,
};

/// Access functions for the FLEXSPI peripheral instance
pub mod FLEXSPI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x402a8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLEXSPI
    pub const reset: ResetValues = ResetValues {
        MCR0: 0xFFFF80C2,
        MCR1: 0xFFFFFFFF,
        MCR2: 0x200081F7,
        AHBCR: 0x00000018,
        INTEN: 0x00000000,
        INTR: 0x00000040,
        LUTKEY: 0x5AF05AF0,
        LUTCR: 0x00000002,
        AHBRXBUF0CR0: 0x80000020,
        AHBRXBUF1CR0: 0x80010020,
        AHBRXBUF2CR0: 0x80020020,
        AHBRXBUF3CR0: 0x80030020,
        FLSHA1CR0: 0x00010000,
        FLSHA2CR0: 0x00010000,
        FLSHB1CR0: 0x00010000,
        FLSHB2CR0: 0x00010000,
        FLSHCR1A1: 0x00000063,
        FLSHCR1A2: 0x00000063,
        FLSHCR1B1: 0x00000063,
        FLSHCR1B2: 0x00000063,
        FLSHCR2A1: 0x00000000,
        FLSHCR2A2: 0x00000000,
        FLSHCR2B1: 0x00000000,
        FLSHCR2B2: 0x00000000,
        FLSHCR4: 0x00000000,
        IPCR0: 0x00000000,
        IPCR1: 0x00000000,
        IPCMD: 0x00000000,
        IPRXFCR: 0x00000000,
        IPTXFCR: 0x00000000,
        DLLCRA: 0x00000100,
        DLLCRB: 0x00000100,
        STS0: 0x00000003,
        STS1: 0x00000000,
        STS2: 0x01000100,
        AHBSPNDSTS: 0x00000000,
        IPRXFSTS: 0x00000000,
        IPTXFSTS: 0x00000000,
        RFDR0: 0x00000000,
        RFDR1: 0x00000000,
        RFDR2: 0x00000000,
        RFDR3: 0x00000000,
        RFDR4: 0x00000000,
        RFDR5: 0x00000000,
        RFDR6: 0x00000000,
        RFDR7: 0x00000000,
        RFDR8: 0x00000000,
        RFDR9: 0x00000000,
        RFDR10: 0x00000000,
        RFDR11: 0x00000000,
        RFDR12: 0x00000000,
        RFDR13: 0x00000000,
        RFDR14: 0x00000000,
        RFDR15: 0x00000000,
        RFDR16: 0x00000000,
        RFDR17: 0x00000000,
        RFDR18: 0x00000000,
        RFDR19: 0x00000000,
        RFDR20: 0x00000000,
        RFDR21: 0x00000000,
        RFDR22: 0x00000000,
        RFDR23: 0x00000000,
        RFDR24: 0x00000000,
        RFDR25: 0x00000000,
        RFDR26: 0x00000000,
        RFDR27: 0x00000000,
        RFDR28: 0x00000000,
        RFDR29: 0x00000000,
        RFDR30: 0x00000000,
        RFDR31: 0x00000000,
        TFDR0: 0x00000000,
        TFDR1: 0x00000000,
        TFDR2: 0x00000000,
        TFDR3: 0x00000000,
        TFDR4: 0x00000000,
        TFDR5: 0x00000000,
        TFDR6: 0x00000000,
        TFDR7: 0x00000000,
        TFDR8: 0x00000000,
        TFDR9: 0x00000000,
        TFDR10: 0x00000000,
        TFDR11: 0x00000000,
        TFDR12: 0x00000000,
        TFDR13: 0x00000000,
        TFDR14: 0x00000000,
        TFDR15: 0x00000000,
        TFDR16: 0x00000000,
        TFDR17: 0x00000000,
        TFDR18: 0x00000000,
        TFDR19: 0x00000000,
        TFDR20: 0x00000000,
        TFDR21: 0x00000000,
        TFDR22: 0x00000000,
        TFDR23: 0x00000000,
        TFDR24: 0x00000000,
        TFDR25: 0x00000000,
        TFDR26: 0x00000000,
        TFDR27: 0x00000000,
        TFDR28: 0x00000000,
        TFDR29: 0x00000000,
        TFDR30: 0x00000000,
        TFDR31: 0x00000000,
        LUT0: 0x00000000,
        LUT1: 0x00000000,
        LUT2: 0x00000000,
        LUT3: 0x00000000,
        LUT4: 0x00000000,
        LUT5: 0x00000000,
        LUT6: 0x00000000,
        LUT7: 0x00000000,
        LUT8: 0x00000000,
        LUT9: 0x00000000,
        LUT10: 0x00000000,
        LUT11: 0x00000000,
        LUT12: 0x00000000,
        LUT13: 0x00000000,
        LUT14: 0x00000000,
        LUT15: 0x00000000,
        LUT16: 0x00000000,
        LUT17: 0x00000000,
        LUT18: 0x00000000,
        LUT19: 0x00000000,
        LUT20: 0x00000000,
        LUT21: 0x00000000,
        LUT22: 0x00000000,
        LUT23: 0x00000000,
        LUT24: 0x00000000,
        LUT25: 0x00000000,
        LUT26: 0x00000000,
        LUT27: 0x00000000,
        LUT28: 0x00000000,
        LUT29: 0x00000000,
        LUT30: 0x00000000,
        LUT31: 0x00000000,
        LUT32: 0x00000000,
        LUT33: 0x00000000,
        LUT34: 0x00000000,
        LUT35: 0x00000000,
        LUT36: 0x00000000,
        LUT37: 0x00000000,
        LUT38: 0x00000000,
        LUT39: 0x00000000,
        LUT40: 0x00000000,
        LUT41: 0x00000000,
        LUT42: 0x00000000,
        LUT43: 0x00000000,
        LUT44: 0x00000000,
        LUT45: 0x00000000,
        LUT46: 0x00000000,
        LUT47: 0x00000000,
        LUT48: 0x00000000,
        LUT49: 0x00000000,
        LUT50: 0x00000000,
        LUT51: 0x00000000,
        LUT52: 0x00000000,
        LUT53: 0x00000000,
        LUT54: 0x00000000,
        LUT55: 0x00000000,
        LUT56: 0x00000000,
        LUT57: 0x00000000,
        LUT58: 0x00000000,
        LUT59: 0x00000000,
        LUT60: 0x00000000,
        LUT61: 0x00000000,
        LUT62: 0x00000000,
        LUT63: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FLEXSPI_TAKEN: bool = false;

    /// Safe access to FLEXSPI
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
            if FLEXSPI_TAKEN {
                None
            } else {
                FLEXSPI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FLEXSPI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FLEXSPI_TAKEN && inst.addr == INSTANCE.addr {
                FLEXSPI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FLEXSPI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FLEXSPI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FLEXSPI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXSPI: *const RegisterBlock = 0x402a8000 as *const _;
