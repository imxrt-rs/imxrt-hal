/// Timing configurations for FlexCAN peripherals

// use super::{ral::ccm, Divider, Frequency, OSCILLATOR_FREQUENCY};
use imxrt_ral::ccm as ccm;

// impl From<Frequency> for core::time::Duration {
//     fn from(hz: Frequency) -> core::time::Duration {
//         core::time::Duration::from_nanos((1_000_000_000u32 / hz.0).into())
//     }
// }

/// High speed oscillator frequency
const OSCILLATOR_FREQUENCY: u32 = 24_000_000; /* 24MHz */

#[derive(Clone, Copy)]
#[non_exhaustive] // Not all variants added
pub enum ClockSelect {
    OSC,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)] // Easier mapping if the names are consistent
#[repr(u32)]
pub enum PrescalarSelect {
    /// 0b000000: Divide by 1
    DIVIDE_1 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_1,
    /// 0b000001: Divide by 2
    DIVIDE_2 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_2,
    /// 0b000010: Divide by 3
    DIVIDE_3 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_3,
    /// 0b000011: Divide by 4
    DIVIDE_4 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_4,
    /// 0b000100: Divide by 5
    DIVIDE_5 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_5,
    /// 0b000101: Divide by 6
    DIVIDE_6 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_6,
    /// 0b000110: Divide by 7
    DIVIDE_7 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_7,
    /// 0b000111: Divide by 8
    DIVIDE_8 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_8,
    /// 0b001000: Divide by 9
    DIVIDE_9 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_9,
    /// 0b001001: Divide by 10
    DIVIDE_10 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_10,
    /// 0b001010: Divide by 11
    DIVIDE_11 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_11,
    /// 0b001011: Divide by 12
    DIVIDE_12 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_12,
    /// 0b001100: Divide by 13
    DIVIDE_13 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_13,
    /// 0b001101: Divide by 14
    DIVIDE_14 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_14,
    /// 0b001110: Divide by 15
    DIVIDE_15 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_15,
    /// 0b001111: Divide by 16
    DIVIDE_16 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_16,
    /// 0b010000: Divide by 17
    DIVIDE_17 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_17,
    /// 0b010001: Divide by 18
    DIVIDE_18 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_18,
    /// 0b010010: Divide by 19
    DIVIDE_19 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_19,
    /// 0b010011: Divide by 20
    DIVIDE_20 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_20,
    /// 0b010100: Divide by 21
    DIVIDE_21 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_21,
    /// 0b010101: Divide by 22
    DIVIDE_22 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_22,
    /// 0b010110: Divide by 23
    DIVIDE_23 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_23,
    /// 0b010111: Divide by 24
    DIVIDE_24 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_24,
    /// 0b011000: Divide by 25
    DIVIDE_25 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_25,
    /// 0b011001: Divide by 26
    DIVIDE_26 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_26,
    /// 0b011010: Divide by 27
    DIVIDE_27 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_27,
    /// 0b011011: Divide by 28
    DIVIDE_28 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_28,
    /// 0b011100: Divide by 29
    DIVIDE_29 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_29,
    /// 0b011101: Divide by 30
    DIVIDE_30 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_30,
    /// 0b011110: Divide by 31
    DIVIDE_31 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_31,
    /// 0b011111: Divide by 32
    DIVIDE_32 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_32,
    /// 0b100000: Divide by 33
    DIVIDE_33 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_33,
    /// 0b100001: Divide by 34
    DIVIDE_34 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_34,
    /// 0b100010: Divide by 35
    DIVIDE_35 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_35,
    /// 0b100011: Divide by 36
    DIVIDE_36 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_36,
    /// 0b100100: Divide by 37
    DIVIDE_37 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_37,
    /// 0b100101: Divide by 38
    DIVIDE_38 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_38,
    /// 0b100110: Divide by 39
    DIVIDE_39 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_39,
    /// 0b100111: Divide by 40
    DIVIDE_40 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_40,
    /// 0b101000: Divide by 41
    DIVIDE_41 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_41,
    /// 0b101001: Divide by 42
    DIVIDE_42 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_42,
    /// 0b101010: Divide by 43
    DIVIDE_43 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_43,
    /// 0b101011: Divide by 44
    DIVIDE_44 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_44,
    /// 0b101100: Divide by 45
    DIVIDE_45 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_45,
    /// 0b101101: Divide by 46
    DIVIDE_46 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_46,
    /// 0b101110: Divide by 47
    DIVIDE_47 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_47,
    /// 0b101111: Divide by 48
    DIVIDE_48 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_48,
    /// 0b110000: Divide by 49
    DIVIDE_49 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_49,
    /// 0b110001: Divide by 50
    DIVIDE_50 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_50,
    /// 0b110010: Divide by 51
    DIVIDE_51 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_51,
    /// 0b110011: Divide by 52
    DIVIDE_52 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_52,
    /// 0b110100: Divide by 53
    DIVIDE_53 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_53,
    /// 0b110101: Divide by 54
    DIVIDE_54 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_54,
    /// 0b110110: Divide by 55
    DIVIDE_55 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_55,
    /// 0b110111: Divide by 56
    DIVIDE_56 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_56,
    /// 0b111000: Divide by 57
    DIVIDE_57 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_57,
    /// 0b111001: Divide by 58
    DIVIDE_58 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_58,
    /// 0b111010: Divide by 59
    DIVIDE_59 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_59,
    /// 0b111011: Divide by 60
    DIVIDE_60 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_60,
    /// 0b111100: Divide by 61
    DIVIDE_61 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_61,
    /// 0b111101: Divide by 62
    DIVIDE_62 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_62,
    /// 0b111110: Divide by 63
    DIVIDE_63 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_63,
    /// 0b111111: Divide by 64
    DIVIDE_64 = ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_64,
}

impl From<ClockSelect> for u32 {
    fn from(clock_select: ClockSelect) -> Self {
        match clock_select {
            ClockSelect::OSC => OSCILLATOR_FREQUENCY,
        }
    }
}

impl From<PrescalarSelect> for u32 {
    fn from(prescalar_select: PrescalarSelect) -> Self {
        (prescalar_select as u32) + 1
    }
}
