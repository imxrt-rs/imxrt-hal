#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IOMUXC_SNVS

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register
pub mod SW_MUX_CTL_PAD_PMIC_ON_REQ {

    /// MUX Mode Select Field.
    pub mod MUX_MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: snvs_lp
            pub const ALT0: u32 = 0b000;

            /// 0b101: Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5
            pub const ALT5: u32 = 0b101;
        }
    }

    /// Software Input On Field.
    pub mod SION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Input Path is determined by functionality
            pub const DISABLED: u32 = 0b0;

            /// 0b1: Force input path of pad PMIC_ON_REQ
            pub const ENABLED: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register
pub mod SW_PAD_CTL_PAD_TEST_MODE {

    /// Slew Rate Field
    pub mod SRE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Slow Slew Rate
            pub const SRE_0_Slow_Slew_Rate: u32 = 0b0;

            /// 0b1: Fast Slew Rate
            pub const SRE_1_Fast_Slew_Rate: u32 = 0b1;
        }
    }

    /// Drive Strength Field
    pub mod DSE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: output driver disabled;
            pub const DSE_0_output_driver_disabled_: u32 = 0b000;

            /// 0b001: R0(260 Ohm @ 3.3V, 150 Ohm@1.8V, 240 Ohm for DDR)
            pub const DSE_1_R0_260_Ohm___3_3V__150_Ohm_1_8V__240_Ohm_for_DDR_: u32 = 0b001;

            /// 0b010: R0/2
            pub const DSE_2_R0_2: u32 = 0b010;

            /// 0b011: R0/3
            pub const DSE_3_R0_3: u32 = 0b011;

            /// 0b100: R0/4
            pub const DSE_4_R0_4: u32 = 0b100;

            /// 0b101: R0/5
            pub const DSE_5_R0_5: u32 = 0b101;

            /// 0b110: R0/6
            pub const DSE_6_R0_6: u32 = 0b110;

            /// 0b111: R0/7
            pub const DSE_7_R0_7: u32 = 0b111;
        }
    }

    /// Speed Field
    pub mod SPEED {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b10: medium(100MHz)
            pub const SPEED: u32 = 0b10;
        }
    }

    /// Open Drain Enable Field
    pub mod ODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Open Drain Disabled
            pub const ODE_0_Open_Drain_Disabled: u32 = 0b0;

            /// 0b1: Open Drain Enabled
            pub const ODE_1_Open_Drain_Enabled: u32 = 0b1;
        }
    }

    /// Pull / Keep Enable Field
    pub mod PKE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Pull/Keeper Disabled
            pub const PKE_0_Pull_Keeper_Disabled: u32 = 0b0;

            /// 0b1: Pull/Keeper Enabled
            pub const PKE_1_Pull_Keeper_Enabled: u32 = 0b1;
        }
    }

    /// Pull / Keep Select Field
    pub mod PUE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Keeper
            pub const PUE_0_Keeper: u32 = 0b0;

            /// 0b1: Pull
            pub const PUE_1_Pull: u32 = 0b1;
        }
    }

    /// Pull Up / Down Config. Field
    pub mod PUS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 100K Ohm Pull Down
            pub const PUS_0_100K_Ohm_Pull_Down: u32 = 0b00;

            /// 0b01: 47K Ohm Pull Up
            pub const PUS_1_47K_Ohm_Pull_Up: u32 = 0b01;

            /// 0b10: 100K Ohm Pull Up
            pub const PUS_2_100K_Ohm_Pull_Up: u32 = 0b10;

            /// 0b11: 22K Ohm Pull Up
            pub const PUS_3_22K_Ohm_Pull_Up: u32 = 0b11;
        }
    }

    /// Hyst. Enable Field
    pub mod HYS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hysteresis Disabled
            pub const HYS_0_Hysteresis_Disabled: u32 = 0b0;

            /// 0b1: Hysteresis Enabled
            pub const HYS_1_Hysteresis_Enabled: u32 = 0b1;
        }
    }
}

/// SW_PAD_CTL_PAD_POR_B SW PAD Control Register
pub mod SW_PAD_CTL_PAD_POR_B {
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::DSE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::HYS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::ODE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PKE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SPEED;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SRE;
}

/// SW_PAD_CTL_PAD_ONOFF SW PAD Control Register
pub mod SW_PAD_CTL_PAD_ONOFF {
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::DSE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::HYS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::ODE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PKE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SPEED;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SRE;
}

/// SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register
pub mod SW_PAD_CTL_PAD_PMIC_ON_REQ {
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::DSE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::HYS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::ODE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PKE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUE;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::PUS;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SPEED;
    pub use super::SW_PAD_CTL_PAD_TEST_MODE::SRE;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 1],

    /// SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register
    pub SW_PAD_CTL_PAD_TEST_MODE: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_POR_B SW PAD Control Register
    pub SW_PAD_CTL_PAD_POR_B: RWRegister<u32>,

    /// SW_PAD_CTL_PAD_ONOFF SW PAD Control Register
    pub SW_PAD_CTL_PAD_ONOFF: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ: RWRegister<u32>,
}
pub struct ResetValues {
    pub SW_MUX_CTL_PAD_PMIC_ON_REQ: u32,
    pub SW_PAD_CTL_PAD_TEST_MODE: u32,
    pub SW_PAD_CTL_PAD_POR_B: u32,
    pub SW_PAD_CTL_PAD_ONOFF: u32,
    pub SW_PAD_CTL_PAD_PMIC_ON_REQ: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}

/// Access functions for the IOMUXC_SNVS peripheral instance
pub mod IOMUXC_SNVS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400a8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IOMUXC_SNVS
    pub const reset: ResetValues = ResetValues {
        SW_MUX_CTL_PAD_PMIC_ON_REQ: 0x00000000,
        SW_PAD_CTL_PAD_TEST_MODE: 0x000030A0,
        SW_PAD_CTL_PAD_POR_B: 0x0001B0A0,
        SW_PAD_CTL_PAD_ONOFF: 0x0001B0A0,
        SW_PAD_CTL_PAD_PMIC_ON_REQ: 0x0000B8A0,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IOMUXC_SNVS_TAKEN: bool = false;

    /// Safe access to IOMUXC_SNVS
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
            if IOMUXC_SNVS_TAKEN {
                None
            } else {
                IOMUXC_SNVS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IOMUXC_SNVS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IOMUXC_SNVS_TAKEN && inst.addr == INSTANCE.addr {
                IOMUXC_SNVS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IOMUXC_SNVS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IOMUXC_SNVS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IOMUXC_SNVS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IOMUXC_SNVS: *const RegisterBlock = 0x400a8000 as *const _;
