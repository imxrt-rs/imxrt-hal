//! DMA multiplexer

use super::RWRegister;

/// DMA multiplexer configuration registers
#[repr(C)]
pub struct RegisterBlock {
    /// Multiplexer configuration registers, one per channel
    pub chcfg: [RWRegister<u32>; 32],
}

impl RegisterBlock {
    pub const ENBL: u32 = 1 << 31;
    pub const TRIG: u32 = 1 << 30;
    pub const A_ON: u32 = 1 << 29;
}
