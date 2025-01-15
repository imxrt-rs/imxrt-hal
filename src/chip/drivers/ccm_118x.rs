//! Clock module for IMXRT118x
//!
//! Currently supported:
//! - CM33 core only
//! - Clockroot configurations with following sources only:
//!    - SysPll3 (480 MHz)
//!    - SysPll3Div2 (240 MHz)
//!    - SysPll3 with PFD0 with divider 13 (frequency 480*18/13 -> ~664.6 MHz)
//!        - PFD0 can be reconfigured albeit with extra care as it is used by [`Flexspi1`]
//!    - SysPll3 with PFD1/2/3 and configurable divider (480*18/DIV where DIV is in range [13, 35])
//!    - OscRc24M (24 MHz)
//!    - OscRc400M (~400 MHz)
//!    - Osc24M (24 MHz)
//!
pub use crate::common::ccm::*;

use imxrt_ral::{
    self as ral, anadig_osc::ANADIG_OSC, anadig_pll::ANADIG_PLL, anadig_pmu::ANADIG_PMU, ccm::CCM,
    dcdc::DCDC, modify_reg, phy_ldo::PHY_LDO, read_reg, write_reg,
};

/// M33 core speed after [`init`] call
pub const M33_CORE_FREQ_HZ: u32 = 240_000_000;

fn poor_mans_delay(at_least_us: u32) {
    const MAX_POSSIBLE_CPU_FREQ_HZ: u32 = 240_000_000;
    const CPU_TICKS_PER_US: u32 = MAX_POSSIBLE_CPU_FREQ_HZ / 1_000_000;
    cortex_m::asm::delay(at_least_us * CPU_TICKS_PER_US);
}

mod dcdc {
    use super::*;

    /// Available voltage targets for 1P0 power rail in DCDC converter
    #[allow(unused)]
    #[derive(Copy, Clone)]
    pub(super) enum Vdd1P0TargetVoltage {
        V0600 = 0x0,
        V0625 = 0x1,
        V0650 = 0x2,
        V0675 = 0x3,
        V0700 = 0x4,
        V0725 = 0x5,
        V0750 = 0x6,
        V0775 = 0x7,
        V0800 = 0x8,
        V0825 = 0x9,
        V0850 = 0xa,
        V0875 = 0xb,
        V0900 = 0xc,
        V0925 = 0xd,
        V0950 = 0xe,
        V0975 = 0xf,
        V1000 = 0x10,
        V1025 = 0x11,
        V1050 = 0x12,
        V1075 = 0x13,
        V1100 = 0x14,
        V1125 = 0x15,
        V1150 = 0x16,
        V1175 = 0x17,
        V1200 = 0x18,
        V1225 = 0x19,
        V1250 = 0x1a,
        V1275 = 0x1b,
        V1300 = 0x1c,
        V1325 = 0x1d,
        V1350 = 0x1e,
        V1375 = 0x1f,
    }

    /// Set target voltage for 1P0 power rail in DCDC converter
    ///
    /// Following the C driver; this is also what seemingly BootROM does too
    /// CMSIS Pack: devices/MIMXRT1189/drivers/fsl_dcdc.h:DCDC_SetVDD1P0BuckModeTargetVoltage
    pub(super) fn set_vdd1p0_buckmode_target_voltage(
        dcdc: &mut DCDC,
        target_voltage: Vdd1P0TargetVoltage,
    ) {
        use imxrt_ral::dcdc::REG0::STS_DC_OK::RW::SETTLED;

        modify_reg!(
            ral::dcdc, &dcdc, REG3,
            VDD1P0CTRL_DISABLE_STEP: ENABLE
        );

        // NOTE: Setting both cores, not sure which one is which :|
        modify_reg!(
            ral::dcdc, &dcdc, TRG_SW_0,
            VDD1P0CTRL_TRG: target_voltage as u32,
        );
        modify_reg!(
            ral::dcdc, &dcdc, TRG_SW_1,
            VDD1P0CTRL_TRG: target_voltage as u32,
        );

        poor_mans_delay(123);
        while read_reg!(ral::dcdc, &dcdc, REG0, STS_DC_OK) != SETTLED {}
    }
}

mod osc_24m {
    use super::*;

    /// Initialize external 24 MHz oscillator
    pub(super) fn init(anadig_osc: &mut ANADIG_OSC) {
        use ral::anadig_osc::OSC_24M_CTRL::OSC_24M_STABLE::RW::STABLE;
        write_reg!(
            ral::anadig_osc, &anadig_osc, OSC_24M_CTRL,
            LP_EN: LP,               // changed // No `Rfb` resistor, running in low power mode
            OSC_EN: ENABLE,          // changed // enable external oscillator
        );

        // Wait until stable
        while read_reg!(ral::anadig_osc, &anadig_osc, OSC_24M_CTRL, OSC_24M_STABLE) != STABLE {
            poor_mans_delay(1000);
        }
    }
}

mod osc_rc_400m {
    use super::*;

    /// Initialize internal 400 MHz RC oscillator
    pub(super) fn init(anadig_osc: &mut ANADIG_OSC) {
        // NOTE:
        // Generated RAL claims there is a `CLKGATE_400MEG` bitfield on position `1`.
        // However, according to reference manual it is reserved.
        // Go figure :|
        write_reg!(
            ral::anadig_osc, &anadig_osc, OSC_400M_CTRL1,
            PWD: 0, // changed // setting it to "no power down"
        );
    }
}

pub mod syspll3 {
    //! Module which deals with SysPll3

    use super::*;

    /// Following the C driver; this is also what seemingly BootROM does too
    /// CMSIS Pack: devices/MIMXRT1189/drivers/fsl_pmu.c:PMU_StaticEnablePllLdo
    fn enable_pll_ldo_static_mode(phy_ldo: &mut PHY_LDO) {
        write_reg!(
            ral::phy_ldo, &phy_ldo, CTRL0,
            LINREG_OUTPUT_TRG: 0x10,
            LINREG_EN: 1,
            LINREG_ILIMIT_EN: 1
        );
        poor_mans_delay(1);
        modify_reg!(
            ral::phy_ldo, &phy_ldo, CTRL0,
            LINREG_ILIMIT_EN: 0
        );
    }

    /// - Reinforce defaults on PMU registers
    /// - Add missing configuration that BootROM does when running from flash
    fn configure_pmu(anadig_pmu: &mut ANADIG_PMU) {
        use ral::anadig_pmu::PMU_LDO_PLL::LDO_PLL_STBY_EN;

        write_reg!(
            ral::anadig_pmu, &anadig_pmu, PMU_BIAS_CTRL,
            FBB_M7_STBY_EN: 1, // default
        );
        write_reg!(ral::anadig_pmu, &anadig_pmu, PMU_BIAS_CTRL2, 0); // default
        write_reg!(
            ral::anadig_pmu, &anadig_pmu, PMU_REF_CTRL,
            EN_PLL_VOL_REF_BUFFER: 1 // changed // BootROM seemingly sets this up, necessary for RAM executables to work
        );
        // reserved "readonly" bit gets zeroed when overwriting the whole register
        // keeping it just for the sake of sanity
        write_reg!(
            ral::anadig_pmu,
            &anadig_pmu,
            PMU_LDO_PLL,
            LDO_PLL_STBY_EN::mask | 1 // default
        );
        write_reg!(
            ral::anadig_pmu, &anadig_pmu, PMU_POWER_DETECT_CTRL,
            CKGB_AON1P0: ENABLE // changed // BootROM seemingly sets this up, necessary for RAM executables to work
        );
    }

    fn configure_power(phy_ldo: &mut PHY_LDO, anadig_pmu: &mut ANADIG_PMU) {
        enable_pll_ldo_static_mode(phy_ldo);

        // adding "some" delay to for `CKGB_AON1P0` write
        // Look: 24.4.1.28 -> PMU_POWER_DETECT_CTRL.CKGB_AON1P -> wait 1ms comment
        // Unclear if it is necessary but this docs piece mentions PHY LDO ON/OFF sequence
        poor_mans_delay(1000);

        configure_pmu(anadig_pmu);
    }

    /// Initialize SysPll3
    ///
    /// (More less) following the C driver
    /// CMSIS Pack: devices/MIMXRT1189/drivers/fsl_clock.c:CLOCK_InitSysPll3
    ///
    /// Exceptions
    /// - Set the whole `SYS_PLL3_PFD` register and reinforce BootROM configuration
    /// - `DIV_SELECT` *must* be `DIV1_1` (default) and it is not meant (?) to be reconfigured.
    ///   - reference manual is inconsistent but default `DIV_SELECT` yields proper `f_ref*20` -> 480MHz
    /// - Power-up routine from C HAL omits PMU register writes that prevent PLL from running in RAM executables
    ///   (ROMBoot does it when running from flash)
    pub(super) fn init(
        anadig_pll: &mut ANADIG_PLL,
        phy_ldo: &mut PHY_LDO,
        anadig_pmu: &mut ANADIG_PMU,
    ) {
        use ral::anadig_pll::SYS_PLL3_CTRL::SYS_PLL3_STABLE;
        configure_power(phy_ldo, anadig_pmu);

        // Configure and ungate PFD0 to follow a default BootROM configuration. PFD0 is used as a source for [`Flexspi1`] clockroot.
        // Other PFDs are not used, disabling them
        write_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_PFD,
            PFD0_FRAC: 13,
            PFD0_DIV1_CLKGATE: ON,
            PFD1_FRAC: 17,
            PFD1_DIV1_CLKGATE: OFF,
            PFD2_FRAC: 32,
            PFD2_DIV1_CLKGATE: OFF,
            PFD3_FRAC: 13,
            PFD3_DIV1_CLKGATE: OFF,
        );

        write_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL,
            PLL_REG_EN: 1,
            SYS_PLL3_GATE: ENABLE,
            DIV_SELECT: DIV1_1
        );
        poor_mans_delay(30);
        modify_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL,
            POWERUP: PUP,
            HOLD_RING_OFF: ENABLE,
        );
        poor_mans_delay(30);
        modify_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL,
            HOLD_RING_OFF: NORMAL,
        );
        while read_reg!(ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL, SYS_PLL3_STABLE)
            != SYS_PLL3_STABLE::RW::STABLE
        {}
        modify_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL,
            ENABLE_CLK: 1,
            SYS_PLL3_DIV2: 1
        );
        modify_reg!(
            ral::anadig_pll, &anadig_pll, SYS_PLL3_CTRL,
            SYS_PLL3_GATE: DISABLE
        );
    }

    /// PFD configuration
    #[derive(Copy, Clone)]
    pub enum PfdConfig {
        /// Enable PFD with a specified divider
        Enable(PfdDiv),
        /// Disable PFD
        Disable,
    }

    /// PFD variant
    #[derive(Copy, Clone)]
    pub enum Pfd {
        /// Variant `0`
        ///
        /// Reconfiguration of Pfd0 must be done with special care as by default
        /// it is used by [`Flexspi1`] which is especially relevant when
        /// using XIP feature.
        Pfd0,
        /// Variant `1`
        Pfd1,
        /// Variant `2`
        Pfd2,
        /// Variant `3`
        Pfd3,
    }

    /// Legal divider values for PFDs
    #[derive(Copy, Clone)]
    pub enum PfdDiv {
        /// Divide the signal by 13
        Div13 = 13,
        /// Divide the signal by 14
        Div14 = 14,
        /// Divide the signal by 15
        Div15 = 15,
        /// Divide the signal by 16
        Div16 = 16,
        /// Divide the signal by 17
        Div17 = 17,
        /// Divide the signal by 18
        Div18 = 18,
        /// Divide the signal by 19
        Div19 = 19,
        /// Divide the signal by 20
        Div20 = 20,
        /// Divide the signal by 21
        Div21 = 21,
        /// Divide the signal by 22
        Div22 = 22,
        /// Divide the signal by 23
        Div23 = 23,
        /// Divide the signal by 24
        Div24 = 24,
        /// Divide the signal by 25
        Div25 = 25,
        /// Divide the signal by 26
        Div26 = 26,
        /// Divide the signal by 27
        Div27 = 27,
        /// Divide the signal by 28
        Div28 = 28,
        /// Divide the signal by 29
        Div29 = 29,
        /// Divide the signal by 30
        Div30 = 30,
        /// Divide the signal by 31
        Div31 = 31,
        /// Divide the signal by 32
        Div32 = 32,
        /// Divide the signal by 33
        Div33 = 33,
        /// Divide the signal by 34
        Div34 = 34,
        /// Divide the signal by 35
        Div35 = 35,
    }

    /// Configure PFD for SysPll3
    ///
    /// Reconfiguration of Pfd0 must be done with special care as by default
    /// it is used by [`Flexspi1`] which is especially relevant when
    /// using XIP feature.
    ///
    /// Calling [`super::init`] will overwrite PFD configuration, thus must be called afterwards.
    pub fn configure_pfd(anadig_pll: &mut ANADIG_PLL, pfd: Pfd, config: PfdConfig) {
        let (en_bit, div) = match config {
            PfdConfig::Enable(pfd_div) => (0b0, pfd_div as u32),
            PfdConfig::Disable => (0b1, 20), // 20 is an arbitrary value in a legal range
        };

        match pfd {
            Pfd::Pfd0 => modify_reg!(
                ral::anadig_pll, &anadig_pll, SYS_PLL3_PFD,
                PFD0_FRAC: div,
                PFD0_DIV1_CLKGATE: en_bit,
            ),
            Pfd::Pfd1 => modify_reg!(
                ral::anadig_pll, &anadig_pll, SYS_PLL3_PFD,
                PFD1_FRAC: div,
                PFD1_DIV1_CLKGATE: en_bit,
            ),
            Pfd::Pfd2 => modify_reg!(
                ral::anadig_pll, &anadig_pll, SYS_PLL3_PFD,
                PFD2_FRAC: div,
                PFD2_DIV1_CLKGATE: en_bit,
            ),
            Pfd::Pfd3 => modify_reg!(
                ral::anadig_pll, &anadig_pll, SYS_PLL3_PFD,
                PFD3_FRAC: div,
                PFD3_DIV1_CLKGATE: en_bit,
            ),
        }
    }
}

pub mod clockroot {
    //! Module that provides means of clock root configuration

    use super::*;

    macro_rules! impl_clockroot {
        ($($name:ident: $clockroot_index:literal),+ $(,)?) => {
            $(
                impl Clockroot for $name {
                    const NAME: &str = stringify!($name);
                    const INDEX: usize = $clockroot_index;

                    fn mux(&self) -> u32 {
                        *self as u32
                    }
                }
            )+
        }
    }

    /// Trait representing a clock root
    pub trait Clockroot: Copy {
        /// Name of the clockroot
        const NAME: &str;
        /// Index in the CCM.CLOCK_ROOT[] array
        ///
        /// Value outside of range [0, 73] will yield a panic
        const INDEX: usize;

        /// Raw value of the multiplexer ([`CLOCK_ROOT[Self::INDEX].CLOCK_ROOT_CONTROL`]) that given `self` represents
        ///
        /// Valid values are in range [0, 3]
        fn mux(&self) -> u32;
    }

    /// Clock root for [`M7`] peripheral
    #[derive(Copy, Clone)]
    pub enum M7 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `ArmPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromArmPllOut = 2,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 3,
    }

    /// Clock root for [`M33`] peripheral
    #[derive(Copy, Clone)]
    pub enum M33 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `ArmPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromArmPllOut = 3,
    }

    /// Clock root for [`Edgelock`] peripheral
    #[derive(Copy, Clone)]
    pub enum Edgelock {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 2,
        /// `SysPll2Pfd1` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd1 = 3,
    }

    /// Clock root for [`BusAon`] peripheral
    #[derive(Copy, Clone)]
    pub enum BusAon {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`BusWakeup`] peripheral
    #[derive(Copy, Clone)]
    pub enum BusWakeup {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 2,
        /// `SysPll3Pfd1` as clock root source
        FromSysPll3Pfd1 = 3,
    }

    /// Clock root for [`WakeupAxi`] peripheral
    #[derive(Copy, Clone)]
    pub enum WakeupAxi {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `SysPll2Pfd1` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd1 = 3,
    }

    /// Clock root for [`SwoTrace`] peripheral
    #[derive(Copy, Clone)]
    pub enum SwoTrace {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`M33Systick`] peripheral
    #[derive(Copy, Clone)]
    pub enum M33Systick {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 2,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 3,
    }

    /// Clock root for [`M7Systick`] peripheral
    #[derive(Copy, Clone)]
    pub enum M7Systick {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 2,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 3,
    }

    /// Clock root for [`Flexio1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Flexio1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Flexio2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Flexio2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Lpit3`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpit3 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lptimer1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lptimer1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lptimer2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lptimer2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lptimer3`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lptimer3 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Tpm2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Tpm2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Tpm4`] peripheral
    #[derive(Copy, Clone)]
    pub enum Tpm4 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Tpm5`] peripheral
    #[derive(Copy, Clone)]
    pub enum Tpm5 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Tpm6`] peripheral
    #[derive(Copy, Clone)]
    pub enum Tpm6 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Gpt1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Gpt1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Gpt2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Gpt2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Flexspi1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Flexspi1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd0` as clock root source
        FromSysPll3Pfd0 = 2,
        /// `SysPll2Pfd0` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd0 = 3,
    }

    /// Clock root for [`Flexspi2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Flexspi2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 2,
        /// `SysPll2Pfd1` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd1 = 3,
    }

    /// Clock root for [`FlexspiSlv`] peripheral
    #[derive(Copy, Clone)]
    pub enum FlexspiSlv {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 2,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 3,
    }

    /// Clock root for [`Can1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Can1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 3,
    }

    /// Clock root for [`Can2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Can2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 3,
    }

    /// Clock root for [`Can3`] peripheral
    #[derive(Copy, Clone)]
    pub enum Can3 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 3,
    }

    /// Clock root for [`Lpuart0102`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart0102 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpuart0304`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart0304 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpuart0506`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart0506 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpuart0708`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart0708 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpuart0910`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart0910 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpuart1112`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpuart1112 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpi2c0102`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpi2c0102 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpi2c0304`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpi2c0304 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpi2c0506`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpi2c0506 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Lpspi0102`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpspi0102 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd1` as clock root source
        FromSysPll3Pfd1 = 2,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 3,
    }

    /// Clock root for [`Lpspi0304`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpspi0304 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd1` as clock root source
        FromSysPll3Pfd1 = 2,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 3,
    }

    /// Clock root for [`Lpspi0506`] peripheral
    #[derive(Copy, Clone)]
    pub enum Lpspi0506 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd1` as clock root source
        FromSysPll3Pfd1 = 2,
        /// `SysPll2Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Out = 3,
    }

    /// Clock root for [`I3c1`] peripheral
    #[derive(Copy, Clone)]
    pub enum I3c1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`I3c2`] peripheral
    #[derive(Copy, Clone)]
    pub enum I3c2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Usdhc1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Usdhc1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll2Pfd2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Usdhc2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Usdhc2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll2Pfd2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Semc`] peripheral
    #[derive(Copy, Clone)]
    pub enum Semc {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 2,
        /// `SysPll2Pfd0` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd0 = 3,
    }

    /// Clock root for [`Adc1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Adc1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Adc2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Adc2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Acmp`] peripheral
    #[derive(Copy, Clone)]
    pub enum Acmp {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Ecat`] peripheral
    #[derive(Copy, Clone)]
    pub enum Ecat {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Enet`] peripheral
    #[derive(Copy, Clone)]
    pub enum Enet {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Tmr1588`] peripheral
    #[derive(Copy, Clone)]
    pub enum Tmr1588 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `SysPll2Pfd3` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd3 = 3,
    }

    /// Clock root for [`Netc`] peripheral
    #[derive(Copy, Clone)]
    pub enum Netc {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Pfd3` as clock root source
        FromSysPll3Pfd3 = 2,
        /// `SysPll2Pfd1` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll2Pfd1 = 3,
    }

    /// Clock root for [`Mac0`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mac0 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Mac1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mac1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Mac2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mac2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Mac3`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mac3 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Mac4`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mac4 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Serdes0`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes0 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Serdes1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Serdes2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 2,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 3,
    }

    /// Clock root for [`Serdes01g`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes01g {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Serdes11g`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes11g {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Serdes21g`] peripheral
    #[derive(Copy, Clone)]
    pub enum Serdes21g {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Out` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Out = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Xcelbusx`] peripheral
    #[derive(Copy, Clone)]
    pub enum Xcelbusx {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `SysPll3Pfd1` as clock root source
        FromSysPll3Pfd1 = 3,
    }

    /// Clock root for [`Xriocu4`] peripheral
    #[derive(Copy, Clone)]
    pub enum Xriocu4 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `Osc24MOut` as clock root source
        FromOsc24MOut = 2,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 3,
    }

    /// Clock root for [`Mctrl`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mctrl {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Sai1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Sai1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`Sai2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Sai2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`Sai3`] peripheral
    #[derive(Copy, Clone)]
    pub enum Sai3 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`Sai4`] peripheral
    #[derive(Copy, Clone)]
    pub enum Sai4 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`Spdif`] peripheral
    #[derive(Copy, Clone)]
    pub enum Spdif {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 2,
        /// `SysPll3Pfd2` as clock root source
        FromSysPll3Pfd2 = 3,
    }

    /// Clock root for [`Asrc`] peripheral
    #[derive(Copy, Clone)]
    pub enum Asrc {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Out` as clock root source
        FromSysPll3Out = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Mic`] peripheral
    #[derive(Copy, Clone)]
    pub enum Mic {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `AudioPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromAudioPllOut = 3,
    }

    /// Clock root for [`Cko1`] peripheral
    #[derive(Copy, Clone)]
    pub enum Cko1 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll3Div2` as clock root source
        FromSysPll3Div2 = 2,
        /// `SysPll1Div2` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div2 = 3,
    }

    /// Clock root for [`Cko2`] peripheral
    #[derive(Copy, Clone)]
    pub enum Cko2 {
        /// `OscRc24M` as clock root source
        FromOscRc24M = 0,
        /// `OscRc400M` as clock root source
        FromOscRc400M = 1,
        /// `SysPll1Div5` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromSysPll1Div5 = 2,
        /// `ArmPllOut` as clock root source
        ///
        /// Currently not supported OOTB, requires manual clock source configuration
        FromArmPllOut = 3,
    }

    impl_clockroot!(
        M7: 0,
        M33: 1,
        Edgelock: 2,
        BusAon: 3,
        BusWakeup: 4,
        WakeupAxi: 5,
        SwoTrace: 6,
        M33Systick: 7,
        M7Systick: 8,
        Flexio1: 9,
        Flexio2: 10,
        Lpit3: 11,
        Lptimer1: 12,
        Lptimer2: 13,
        Lptimer3: 14,
        Tpm2: 15,
        Tpm4: 16,
        Tpm5: 17,
        Tpm6: 18,
        Gpt1: 19,
        Gpt2: 20,
        Flexspi1: 21,
        Flexspi2: 22,
        FlexspiSlv: 23,
        Can1: 24,
        Can2: 25,
        Can3: 26,
        Lpuart0102: 27,
        Lpuart0304: 28,
        Lpuart0506: 29,
        Lpuart0708: 30,
        Lpuart0910: 31,
        Lpuart1112: 32,
        Lpi2c0102: 33,
        Lpi2c0304: 34,
        Lpi2c0506: 35,
        Lpspi0102: 36,
        Lpspi0304: 37,
        Lpspi0506: 38,
        I3c1: 39,
        I3c2: 40,
        Usdhc1: 41,
        Usdhc2: 42,
        Semc: 43,
        Adc1: 44,
        Adc2: 45,
        Acmp: 46,
        Ecat: 47,
        Enet: 48,
        Tmr1588: 49,
        Netc: 50,
        Mac0: 51,
        Mac1: 52,
        Mac2: 53,
        Mac3: 54,
        Mac4: 55,
        Serdes0: 56,
        Serdes1: 57,
        Serdes2: 58,
        Serdes01g: 59,
        Serdes11g: 60,
        Serdes21g: 61,
        Xcelbusx: 62,
        Xriocu4: 63,
        Mctrl: 64,
        Sai1: 65,
        Sai2: 66,
        Sai3: 67,
        Sai4: 68,
        Spdif: 69,
        Asrc: 70,
        Mic: 71,
        Cko1: 72,
        Cko2: 73,
    );

    /// Configure a clock root
    ///
    /// Divider ([`div`]) must be reasonable, follow table 138 in 20.4.3 chapter regarding legal frequencies.
    pub fn configure<T: Clockroot>(ccm: &mut CCM, clockroot: T, div: u8) {
        write_reg!(
            ral::ccm::clockroot, &ccm.CLOCK_ROOT[T::INDEX], CLOCK_ROOT_CONTROL,
            MUX: clockroot.mux(),
            DIV: div.saturating_sub(1) as u32, // actualDIV == DIV+1
        );
    }

    /// Inspect clock root registers and print out values via `defmt`
    #[cfg(feature = "defmt")]
    pub fn inspect<T: Clockroot>(ccm: &mut CCM) {
        let val = read_reg!(
            ral::ccm::clockroot,
            &ccm.CLOCK_ROOT[T::INDEX],
            CLOCK_ROOT_CONTROL
        );
        defmt::info!(
            "CLOCK_ROOT[{} / {}].CLOCK_ROOT_CONTROL: {:#x}",
            T::INDEX,
            T::NAME,
            val
        );
        let val = read_reg!(
            ral::ccm::clockroot,
            &ccm.CLOCK_ROOT[T::INDEX],
            CLOCK_ROOT_STATUS0
        );
        defmt::info!(
            "CLOCK_ROOT[{} / {}].CLOCK_ROOT_STATUS0: {:#x}",
            T::INDEX,
            T::NAME,
            val
        );
    }
}

/// Initialize and configure clocks
///
/// - switch M33 core temporarily to RC 400MHz
/// - configure DCDC 1p0 power rail to serve 1.1V
/// - initialize external 24MHz oscillator
/// - initialize SysPLL3
/// - switch M33 core to SysPLL3 with div: 2 (240 MHz)
/// - switch FlexSPI1 to SysPll3Pfd0 with div: 5 (~133MHz)
///
/// Use [`clockroot::configure`] function in order to configure more clockroots
pub fn init(
    anadig_osc: &mut ANADIG_OSC,
    anadig_pll: &mut ANADIG_PLL,
    anadig_pmu: &mut ANADIG_PMU,
    ccm: &mut CCM,
    dcdc: &mut DCDC,
    phy_ldo: &mut PHY_LDO,
) {
    // Unclear if it is necessary
    // - C codegen from clock tool does this
    // - BootROM does this when running from flash
    // Doing the same thing for the sake of completeness.
    osc_rc_400m::init(anadig_osc);
    clockroot::configure(ccm, clockroot::M33::FromOscRc400M, 2);

    // Prerequisite for running a core with frequency > 200MHz
    dcdc::set_vdd1p0_buckmode_target_voltage(dcdc, dcdc::Vdd1P0TargetVoltage::V1100);

    osc_24m::init(anadig_osc);

    syspll3::init(anadig_pll, phy_ldo, anadig_pmu);

    clockroot::configure(ccm, clockroot::M33::FromSysPll3Out, 2); // 480/2 -> 240MHz

    clockroot::configure(ccm, clockroot::Flexspi1::FromSysPll3Pfd0, 5); // 480*18/13/5 -> ~133MHz // Restoring BootROM configuration
}

#[cfg(feature = "defmt")]
pub mod diag {
    //! Clock diagnostics module

    use super::*;

    /// Available multiplexer values for setting up clock observation.
    #[derive(defmt::Format)]
    pub enum ObserveMux {
        /// Observe `OscRc24m`
        OscRc24m = 2,
        /// Observe `OscRc400m`
        OscRc400m = 3,
        /// Observe `Osc24mOut`
        Osc24mOut = 5,
        /// Observe `PllArmOut`
        PllArmOut = 7,
        /// Observe `Pll528Out`
        Pll528Out = 9,
        /// Observe `Pll528Pfd0`
        Pll528Pfd0 = 10,
        /// Observe `Pll528Pfd1`
        Pll528Pfd1 = 11,
        /// Observe `Pll528Pfd2`
        Pll528Pfd2 = 12,
        /// Observe `Pll528Pfd3`
        Pll528Pfd3 = 13,
        /// Observe `Pll480Out`
        Pll480Out = 15,
        /// Observe `Pll480Div2`
        Pll480Div2 = 16,
        /// Observe `Pll480Pfd0`
        Pll480Pfd0 = 17,
        /// Observe `Pll480Pfd1`
        Pll480Pfd1 = 18,
        /// Observe `Pll480Pfd2`
        Pll480Pfd2 = 19,
        /// Observe `Pll480Pfd3`
        Pll480Pfd3 = 20,
        /// Observe `Pll1gOut`
        Pll1gOut = 22,
        /// Observe `Pll1gDiv2`
        Pll1gDiv2 = 23,
        /// Observe `Pll1gDiv5`
        Pll1gDiv5 = 24,
        /// Observe `PllAudioOut`
        PllAudioOut = 26,
        /// Observe `M7ClkRoot`
        M7ClkRoot = 128,
        /// Observe `M33ClkRoot`
        M33ClkRoot = 129,
        /// Observe `EdgelockClkRoot`
        EdgelockClkRoot = 130,
        /// Observe `BusAonClkRoot`
        BusAonClkRoot = 131,
        /// Observe `BusWakeupClkRoot`
        BusWakeupClkRoot = 132,
        /// Observe `WakeupAxiClkRoot`
        WakeupAxiClkRoot = 133,
        /// Observe `SwoTraceClkRoot`
        SwoTraceClkRoot = 134,
        /// Observe `M33SystickClkRoot`
        M33SystickClkRoot = 135,
        /// Observe `M7SystickClkRoot`
        M7SystickClkRoot = 136,
        /// Observe `Flexio1ClkRoot`
        Flexio1ClkRoot = 137,
        /// Observe `Flexio2ClkRoot`
        Flexio2ClkRoot = 138,
        /// Observe `Lpit3ClkRoot`
        Lpit3ClkRoot = 139,
        /// Observe `Lptmr1ClkRoot`
        Lptmr1ClkRoot = 140,
        /// Observe `Lptmr2ClkRoot`
        Lptmr2ClkRoot = 141,
        /// Observe `Lptmr3ClkRoot`
        Lptmr3ClkRoot = 142,
        /// Observe `Tpm2ClkRoot`
        Tpm2ClkRoot = 143,
        /// Observe `Tpm4ClkRoot`
        Tpm4ClkRoot = 144,
        /// Observe `Tpm5ClkRoot`
        Tpm5ClkRoot = 145,
        /// Observe `Tpm6ClkRoot`
        Tpm6ClkRoot = 146,
        /// Observe `Gpt1ClkRoot`
        Gpt1ClkRoot = 147,
        /// Observe `Gpt2ClkRoot`
        Gpt2ClkRoot = 148,
        /// Observe `Flexspi1ClkRoot`
        Flexspi1ClkRoot = 149,
        /// Observe `Flexspi2ClkRoot`
        Flexspi2ClkRoot = 150,
        /// Observe `FlexspiSlvClkRoot`
        FlexspiSlvClkRoot = 151,
        /// Observe `Can1ClkRoot`
        Can1ClkRoot = 152,
        /// Observe `Can2ClkRoot`
        Can2ClkRoot = 153,
        /// Observe `Can3ClkRoot`
        Can3ClkRoot = 154,
        /// Observe `Lpuart0102ClkRoot`
        Lpuart0102ClkRoot = 155,
        /// Observe `Lpuart0304ClkRoot`
        Lpuart0304ClkRoot = 156,
        /// Observe `Lpuart0506ClkRoot`
        Lpuart0506ClkRoot = 157,
        /// Observe `Lpuart0708ClkRoot`
        Lpuart0708ClkRoot = 158,
        /// Observe `Lpuart0910ClkRoot`
        Lpuart0910ClkRoot = 159,
        /// Observe `Lpuart1112ClkRoot`
        Lpuart1112ClkRoot = 160,
        /// Observe `Lpi2c0102ClkRoot`
        Lpi2c0102ClkRoot = 161,
        /// Observe `Lpi2c0304ClkRoot`
        Lpi2c0304ClkRoot = 162,
        /// Observe `Lpi2c0506ClkRoot`
        Lpi2c0506ClkRoot = 163,
        /// Observe `Lpspi0102ClkRoot`
        Lpspi0102ClkRoot = 164,
        /// Observe `Lpspi0304ClkRoot`
        Lpspi0304ClkRoot = 165,
        /// Observe `Lpspi0506ClkRoot`
        Lpspi0506ClkRoot = 166,
        /// Observe `I3c1ClkRoot`
        I3c1ClkRoot = 167,
        /// Observe `I3c2ClkRoot`
        I3c2ClkRoot = 168,
        /// Observe `Usdhc1ClkRoot`
        Usdhc1ClkRoot = 169,
        /// Observe `Usdhc2ClkRoot`
        Usdhc2ClkRoot = 170,
        /// Observe `SemcClkRoot`
        SemcClkRoot = 171,
        /// Observe `Adc1ClkRoot`
        Adc1ClkRoot = 172,
        /// Observe `Adc2ClkRoot`
        Adc2ClkRoot = 173,
        /// Observe `AcmpClkRoot`
        AcmpClkRoot = 174,
        /// Observe `EcatClkRoot`
        EcatClkRoot = 175,
        /// Observe `EnetRefclkRoot`
        EnetRefclkRoot = 176,
        /// Observe `Tmr1588ClkRoot`
        Tmr1588ClkRoot = 177,
        /// Observe `NetcClkRoot`
        NetcClkRoot = 178,
        /// Observe `Mac0ClkRoot`
        Mac0ClkRoot = 179,
        /// Observe `Mac1ClkRoot`
        Mac1ClkRoot = 180,
        /// Observe `Mac2ClkRoot`
        Mac2ClkRoot = 181,
        /// Observe `Mac3ClkRoot`
        Mac3ClkRoot = 182,
        /// Observe `Mac4ClkRoot`
        Mac4ClkRoot = 183,
        /// Observe `Serdes0ClkRoot`
        Serdes0ClkRoot = 184,
        /// Observe `Serdes1ClkRoot`
        Serdes1ClkRoot = 185,
        /// Observe `Serdes2ClkRoot`
        Serdes2ClkRoot = 186,
        /// Observe `Serdes01gClkRoot`
        Serdes01gClkRoot = 187,
        /// Observe `Serdes11gClkRoot`
        Serdes11gClkRoot = 188,
        /// Observe `Serdes21gClkRoot`
        Serdes21gClkRoot = 189,
        /// Observe `XcelbusxClkRoot`
        XcelbusxClkRoot = 190,
        /// Observe `Xriocu4ClkRoot`
        Xriocu4ClkRoot = 191,
        /// Observe `MotorctrlClkRoot`
        MotorctrlClkRoot = 192,
        /// Observe `Sai1ClkRoot`
        Sai1ClkRoot = 193,
        /// Observe `Sai2ClkRoot`
        Sai2ClkRoot = 194,
        /// Observe `Sai3ClkRoot`
        Sai3ClkRoot = 195,
        /// Observe `Sai4ClkRoot`
        Sai4ClkRoot = 196,
        /// Observe `SpdifClkRoot`
        SpdifClkRoot = 197,
        /// Observe `AsrcClkRoot`
        AsrcClkRoot = 198,
        /// Observe `MicClkRoot`
        MicClkRoot = 199,
        /// Observe `CcmCko1ClkRoot`
        CcmCko1ClkRoot = 200,
        /// Observe `CcmCko2ClkRoot`
        CcmCko2ClkRoot = 201,
    }

    // TODO: Seems to be broken after the write to TRDC1.MDA_W0_2_DFMT1 register
    // Has to be looked further into maybe.
    //
    /// Observe chosen clock and print out frequencies via `defmt`
    pub fn observe_clock(ccm: &mut CCM, mux: ObserveMux) {
        defmt::info!("Observing: {:?}..", mux);
        // Configure
        write_reg!(
            ral::ccm::observe, &ccm.OBSERVE[0],
            OBSERVE_CONTROL,
            SELECT: mux as u32,
            DIVIDE: 0,
            RESET: 1
        );
        poor_mans_delay(10);

        // Reset and start the measurement
        modify_reg!(
            ral::ccm::observe, &ccm.OBSERVE[0],
            OBSERVE_CONTROL,
            RESET: 0, OFF: 0
        );
        poor_mans_delay(30000);

        // Stop the measurement
        modify_reg!(
            ral::ccm::observe, &ccm.OBSERVE[0],
            OBSERVE_CONTROL,
            OFF: 1
        );
        poor_mans_delay(10);

        let observe_status = read_reg!(ral::ccm::observe, &ccm.OBSERVE[0], OBSERVE_STATUS);
        defmt::info!("Status: {:#x}", observe_status);

        let observe_frequency_current = read_reg!(
            ral::ccm::observe,
            &ccm.OBSERVE[0],
            OBSERVE_FREQUENCY_CURRENT
        );
        defmt::info!("Frequency Current: {} Hz", observe_frequency_current);

        let observe_frequency_min =
            read_reg!(ral::ccm::observe, &ccm.OBSERVE[0], OBSERVE_FREQUENCY_MIN);
        defmt::info!("Frequency Min: {} Hz", observe_frequency_min);

        let observe_frequency_max =
            read_reg!(ral::ccm::observe, &ccm.OBSERVE[0], OBSERVE_FREQUENCY_MAX);
        defmt::info!("Frequency Max: {} Hz", observe_frequency_max);
    }
}
