//! DMA multiplexer

use super::{register::RWRegister, CHANNEL_COUNT};

/// DMA multiplexer configuration registers
#[repr(C)]
pub struct RegisterBlock {
    /// Multiplexer configuration registers, one per channel
    pub chcfg: [RWRegister<u32>; CHANNEL_COUNT],
}

impl RegisterBlock {
    pub const ENBL: u32 = 1 << 31;
    pub const A_ON: u32 = 1 << 29;
}
