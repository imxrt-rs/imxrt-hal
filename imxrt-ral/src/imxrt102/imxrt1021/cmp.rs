#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CMP Control Register 0
pub mod CR0 {

    /// Comparator hard block hysteresis control
    pub mod HYSTCTR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Level 0
            pub const HYSTCTR_0: u32 = 0b00;

            /// 0b01: Level 1
            pub const HYSTCTR_1: u32 = 0b01;

            /// 0b10: Level 2
            pub const HYSTCTR_2: u32 = 0b10;

            /// 0b11: Level 3
            pub const HYSTCTR_3: u32 = 0b11;
        }
    }

    /// Filter Sample Count
    pub mod FILTER_CNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA.
            pub const FILTER_CNT_0: u32 = 0b000;

            /// 0b001: One sample must agree. The comparator output is simply sampled.
            pub const FILTER_CNT_1: u32 = 0b001;

            /// 0b010: 2 consecutive samples must agree.
            pub const FILTER_CNT_2: u32 = 0b010;

            /// 0b011: 3 consecutive samples must agree.
            pub const FILTER_CNT_3: u32 = 0b011;

            /// 0b100: 4 consecutive samples must agree.
            pub const FILTER_CNT_4: u32 = 0b100;

            /// 0b101: 5 consecutive samples must agree.
            pub const FILTER_CNT_5: u32 = 0b101;

            /// 0b110: 6 consecutive samples must agree.
            pub const FILTER_CNT_6: u32 = 0b110;

            /// 0b111: 7 consecutive samples must agree.
            pub const FILTER_CNT_7: u32 = 0b111;
        }
    }
}

/// CMP Control Register 1
pub mod CR1 {

    /// Comparator Module Enable
    pub mod EN {
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

            /// 0b0: Analog Comparator is disabled.
            pub const EN_0: u32 = 0b0;

            /// 0b1: Analog Comparator is enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// Comparator Output Pin Enable
    pub mod OPE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect.
            pub const OPE_0: u32 = 0b0;

            /// 0b1: CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect.
            pub const OPE_1: u32 = 0b1;
        }
    }

    /// Comparator Output Select
    pub mod COS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Set the filtered comparator output (CMPO) to equal COUT.
            pub const COS_0: u32 = 0b0;

            /// 0b1: Set the unfiltered comparator output (CMPO) to equal COUTA.
            pub const COS_1: u32 = 0b1;
        }
    }

    /// Comparator INVERT
    pub mod INV {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Does not invert the comparator output.
            pub const INV_0: u32 = 0b0;

            /// 0b1: Inverts the comparator output.
            pub const INV_1: u32 = 0b1;
        }
    }

    /// Power Mode Select
    pub mod PMODE {
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

            /// 0b0: Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption.
            pub const PMODE_0: u32 = 0b0;

            /// 0b1: High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption.
            pub const PMODE_1: u32 = 0b1;
        }
    }

    /// Windowing Enable
    pub mod WE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Windowing mode is not selected.
            pub const WE_0: u32 = 0b0;

            /// 0b1: Windowing mode is selected.
            pub const WE_1: u32 = 0b1;
        }
    }

    /// Sample Enable
    pub mod SE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Sampling mode is not selected.
            pub const SE_0: u32 = 0b0;

            /// 0b1: Sampling mode is selected.
            pub const SE_1: u32 = 0b1;
        }
    }
}

/// CMP Filter Period Register
pub mod FPR {

    /// Filter Sample Period
    pub mod FILT_PER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CMP Status and Control Register
pub mod SCR {

    /// Analog Comparator Output
    pub mod COUT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog Comparator Flag Falling
    pub mod CFF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Falling-edge on COUT has not been detected.
            pub const CFF_0: u32 = 0b0;

            /// 0b1: Falling-edge on COUT has occurred.
            pub const CFF_1: u32 = 0b1;
        }
    }

    /// Analog Comparator Flag Rising
    pub mod CFR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Rising-edge on COUT has not been detected.
            pub const CFR_0: u32 = 0b0;

            /// 0b1: Rising-edge on COUT has occurred.
            pub const CFR_1: u32 = 0b1;
        }
    }

    /// Comparator Interrupt Enable Falling
    pub mod IEF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Interrupt is disabled.
            pub const IEF_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled.
            pub const IEF_1: u32 = 0b1;
        }
    }

    /// Comparator Interrupt Enable Rising
    pub mod IER {
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

            /// 0b0: Interrupt is disabled.
            pub const IER_0: u32 = 0b0;

            /// 0b1: Interrupt is enabled.
            pub const IER_1: u32 = 0b1;
        }
    }

    /// DMA Enable Control
    pub mod DMAEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA is disabled.
            pub const DMAEN_0: u32 = 0b0;

            /// 0b1: DMA is enabled.
            pub const DMAEN_1: u32 = 0b1;
        }
    }
}

/// DAC Control Register
pub mod DACCR {

    /// DAC Output Voltage Select
    pub mod VOSEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Supply Voltage Reference Source Select
    pub mod VRSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Vin1 is selected as resistor ladder network supply reference.
            pub const VRSEL_0: u32 = 0b0;

            /// 0b1: Vin2 is selected as resistor ladder network supply reference.
            pub const VRSEL_1: u32 = 0b1;
        }
    }

    /// DAC Enable
    pub mod DACEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC is disabled.
            pub const DACEN_0: u32 = 0b0;

            /// 0b1: DAC is enabled.
            pub const DACEN_1: u32 = 0b1;
        }
    }
}

/// MUX Control Register
pub mod MUXCR {

    /// Minus Input Mux Control
    pub mod MSEL {
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

            /// 0b000: IN0
            pub const MSEL_0: u32 = 0b000;

            /// 0b001: IN1
            pub const MSEL_1: u32 = 0b001;

            /// 0b010: IN2
            pub const MSEL_2: u32 = 0b010;

            /// 0b011: IN3
            pub const MSEL_3: u32 = 0b011;

            /// 0b100: IN4
            pub const MSEL_4: u32 = 0b100;

            /// 0b101: IN5
            pub const MSEL_5: u32 = 0b101;

            /// 0b110: IN6
            pub const MSEL_6: u32 = 0b110;

            /// 0b111: IN7
            pub const MSEL_7: u32 = 0b111;
        }
    }

    /// Plus Input Mux Control
    pub mod PSEL {
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

            /// 0b000: IN0
            pub const PSEL_0: u32 = 0b000;

            /// 0b001: IN1
            pub const PSEL_1: u32 = 0b001;

            /// 0b010: IN2
            pub const PSEL_2: u32 = 0b010;

            /// 0b011: IN3
            pub const PSEL_3: u32 = 0b011;

            /// 0b100: IN4
            pub const PSEL_4: u32 = 0b100;

            /// 0b101: IN5
            pub const PSEL_5: u32 = 0b101;

            /// 0b110: IN6
            pub const PSEL_6: u32 = 0b110;

            /// 0b111: IN7
            pub const PSEL_7: u32 = 0b111;
        }
    }
}
pub struct RegisterBlock {
    /// CMP Control Register 0
    pub CR0: RWRegister<u8>,

    /// CMP Control Register 1
    pub CR1: RWRegister<u8>,

    /// CMP Filter Period Register
    pub FPR: RWRegister<u8>,

    /// CMP Status and Control Register
    pub SCR: RWRegister<u8>,

    /// DAC Control Register
    pub DACCR: RWRegister<u8>,

    /// MUX Control Register
    pub MUXCR: RWRegister<u8>,
}
pub struct ResetValues {
    pub CR0: u8,
    pub CR1: u8,
    pub FPR: u8,
    pub SCR: u8,
    pub DACCR: u8,
    pub MUXCR: u8,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}

/// Access functions for the CMP1 peripheral instance
pub mod CMP1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP1
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP1_TAKEN: bool = false;

    /// Safe access to CMP1
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
            if CMP1_TAKEN {
                None
            } else {
                CMP1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP1_TAKEN && inst.addr == INSTANCE.addr {
                CMP1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP1: *const RegisterBlock = 0x40094000 as *const _;

/// Access functions for the CMP2 peripheral instance
pub mod CMP2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094008,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP2
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP2_TAKEN: bool = false;

    /// Safe access to CMP2
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
            if CMP2_TAKEN {
                None
            } else {
                CMP2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP2_TAKEN && inst.addr == INSTANCE.addr {
                CMP2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP2: *const RegisterBlock = 0x40094008 as *const _;

/// Access functions for the CMP3 peripheral instance
pub mod CMP3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094010,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP3
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP3_TAKEN: bool = false;

    /// Safe access to CMP3
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
            if CMP3_TAKEN {
                None
            } else {
                CMP3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP3_TAKEN && inst.addr == INSTANCE.addr {
                CMP3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP3: *const RegisterBlock = 0x40094010 as *const _;

/// Access functions for the CMP4 peripheral instance
pub mod CMP4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40094018,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CMP4
    pub const reset: ResetValues = ResetValues {
        CR0: 0x00000000,
        CR1: 0x00000000,
        FPR: 0x00000000,
        SCR: 0x00000000,
        DACCR: 0x00000000,
        MUXCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CMP4_TAKEN: bool = false;

    /// Safe access to CMP4
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
            if CMP4_TAKEN {
                None
            } else {
                CMP4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CMP4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CMP4_TAKEN && inst.addr == INSTANCE.addr {
                CMP4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CMP4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CMP4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CMP4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CMP4: *const RegisterBlock = 0x40094018 as *const _;
