//! Implements the routine to configure the processor speed
//!
//! Reimplementation of the [`set_arm_clock` routine] from the Teensy 4
//! Arduino Libraries.
//!
//! [`set_arm_clock` routine]: https://github.com/PaulStoffregen/cores/blob/master/teensy4/clockspeed.c

use imxrt_ral as ral;
use ral::{modify_reg, read_reg, write_reg};

/// Note that while the value is limited to u8 the return
/// is u32 to easily compare against register read values which are always u32
#[inline(always)]
fn reg3_trg(mv: u32) -> u32 {
    (((mv - 800) / 25) as u8) as u32
}

/// Sets the main system clock to as close to `hz` as possible.
/// Returns the `(ARM, IPG)` clock frequencies based on the input frequency
/// and selected prescalars.
pub fn set_arm_clock(
    mut hz: u32,
    ccm: &ral::ccm::Instance,
    ccm_analog: &ral::ccm_analog::Instance,
    dcdc: &ral::dcdc::Instance,
) -> (u32, u32) {
    let millivolts: u32 = if hz > 528_000_000 {
        1250 // 1.25V
    } else if hz <= 24_000_000 {
        950 // 0.95V
    } else {
        1150 // 1.15V, default
    };

    // Enable clocks to the DCDC module
    // Safety: CG3 field is two bits
    modify_reg!(ral::ccm, ccm, CCGR6, CG3: 0x3);

    // Set VDD_SOC, voltage for the chip
    let reg3_trg_mv = reg3_trg(millivolts);
    if read_reg!(ral::dcdc, dcdc, REG3, TRG) < reg3_trg_mv {
        log::debug!("Increasing voltage to {}mv", millivolts);
        // Safety: the possible values of millivolts after going through
        // reg3_trg fits in 5 bits.
        modify_reg!(ral::dcdc, dcdc, REG3, TRG: reg3_trg_mv);
        while read_reg!(ral::dcdc, dcdc, REG0, STS_DC_OK) == 0 {
            #[allow(deprecated)]
            core::sync::atomic::spin_loop_hint();
        }
    }

    select_alt_clock(ccm, ccm_analog);

    let (mut div_arm, mut div_ahb) = (1, 1);
    while hz * div_arm * div_ahb < 648_000_000 {
        if div_arm < 8 {
            div_arm += 1;
        } else if div_ahb < 5 {
            div_ahb += 1;
            div_arm = 1;
        } else {
            break;
        }
    }
    let mult = (hz * div_arm * div_ahb + 6_000_000) / 12_000_000;
    let mult = mult.clamp(54, 108);
    log::debug!(
        "Frequency 12MHz * {mult} / {div_arm} / {div_ahb}",
        mult = mult,
        div_arm = div_arm,
        div_ahb = div_ahb
    );
    hz = mult * 12_000_000 / div_arm / div_ahb;

    let pll_arm = read_reg!(ral::ccm_analog, ccm_analog, PLL_ARM);
    log::debug!("ARM PLL = 0x{:x}", pll_arm);
    //TODO create a write_with_zero_reg!() to avoid setting fields to 0 here
    write_reg!(ral::ccm_analog, ccm_analog, PLL_ARM,
               DIV_SELECT: 0,
               POWERDOWN: 1,
               ENABLE: 0,
               BYPASS_CLK_SRC: 0,
               BYPASS: 0,
               PLL_SEL: 0,
               LOCK: 0
    );
    write_reg!(ral::ccm_analog, ccm_analog, PLL_ARM,
               DIV_SELECT: mult,
               POWERDOWN: 0,
               ENABLE: 1,
               BYPASS_CLK_SRC: 0,
               BYPASS: 0,
               PLL_SEL: 0,
               LOCK: 0
    );
    while read_reg!(ral::ccm_analog, ccm_analog, PLL_ARM, LOCK) == 0 {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    }
    log::debug!(
        "ARM PLL = 0x{:x}",
        read_reg!(ral::ccm_analog, ccm_analog, PLL_ARM)
    );

    write_reg!(ral::ccm, ccm, CACRR, ARM_PODF: (div_arm - 1));
    while read_reg!(ral::ccm, ccm, CDHIPR, ARM_PODF_BUSY) > 0 {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    }
    modify_reg!(ral::ccm, ccm, CBCDR, AHB_PODF: (div_ahb - 1));
    while read_reg!(ral::ccm, ccm, CDHIPR, ARM_PODF_BUSY) > 0 {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    }

    let div_ipg = (hz + 149_999_999) / 150_000_000;
    let div_ipg = div_ipg.min(4);
    modify_reg!(ral::ccm, ccm, CBCDR, IPG_PODF: (div_ipg - 1));
    modify_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK_SEL: 0);
    while read_reg!(ral::ccm, ccm, CDHIPR, PERIPH_CLK_SEL_BUSY) > 0 {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    }

    log::debug!("ARM={}, IPG={}", hz, hz / div_ipg);
    let reg3_trg_mv = reg3_trg(millivolts);
    if read_reg!(ral::dcdc, dcdc, REG3, TRG) > reg3_trg_mv {
        log::debug!("Decreasing voltage to {}mv", millivolts);
        modify_reg!(ral::dcdc, dcdc, REG3, TRG: reg3_trg_mv);
        while read_reg!(ral::dcdc, dcdc, REG0, STS_DC_OK) == 0 {
            #[allow(deprecated)]
            core::sync::atomic::spin_loop_hint();
        }
    }

    (hz, hz / div_ipg)
}

/// Selects an alternative clock so that we can modify the main
/// system clock.
#[inline(always)]
fn select_alt_clock(ccm: &ral::ccm::Instance, ccm_analog: &ral::ccm_analog::Instance) {
    use ral::ccm::CBCDR::PERIPH_CLK_SEL;
    let periph_clk_sel = read_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK_SEL);
    if PERIPH_CLK_SEL::RW::PERIPH_CLK_SEL_0 == periph_clk_sel {
        log::debug!("Choosing alternative clock before reconfiguring ARM PLL...");
        use ral::ccm::{CBCDR::PERIPH_CLK2_PODF, CBCMR::PERIPH_CLK2_SEL};

        let (usb_pll_enable, usb_pll_en_usb_clks, usb_pll_power, usb_pll_lock) = read_reg!(
            ral::ccm_analog,
            ccm_analog,
            PLL_USB1,
            ENABLE,
            EN_USB_CLKS,
            POWER,
            LOCK
        );
        let (sel, div) = if usb_pll_enable > 0
            && usb_pll_en_usb_clks > 0
            && usb_pll_power > 0
            && usb_pll_lock > 0
        {
            log::debug!("Using USB PLL, divided down to 120MHz");
            (
                PERIPH_CLK2_SEL::RW::PERIPH_CLK2_SEL_0,
                PERIPH_CLK2_PODF::RW::PERIPH_CLK2_PODF_3,
            )
        } else {
            log::debug!("USB PLL is off; using 24MHz oscillator");
            (
                PERIPH_CLK2_SEL::RW::PERIPH_CLK2_SEL_1,
                PERIPH_CLK2_PODF::RW::PERIPH_CLK2_PODF_0,
            )
        };
        modify_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK2_PODF: div);
        modify_reg!(ral::ccm, ccm, CBCMR, PERIPH_CLK2_SEL: sel);
        while read_reg!(ral::ccm, ccm, CDHIPR, PERIPH2_CLK_SEL_BUSY) > 0 {
            #[allow(deprecated)]
            core::sync::atomic::spin_loop_hint();
        }

        modify_reg!(ral::ccm, ccm, CBCDR, PERIPH_CLK_SEL: 1);
        while read_reg!(ral::ccm, ccm, CDHIPR, PERIPH_CLK_SEL_BUSY) > 0 {
            #[allow(deprecated)]
            core::sync::atomic::spin_loop_hint();
        }
    } else {
        log::debug!("Already running from PERIPH2_CLK2");
    }
}
