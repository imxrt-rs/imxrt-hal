//! DMA register blocks and fields

use super::{
    register::{RORegister, RWRegister, WORegister},
    tcd, CHANNEL_COUNT,
};

use core::ops::Index;

/// DMA registers
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register
    pub CR: RWRegister<u32>,
    /// Error Status Register
    pub ES: RORegister<u32>,
    _reserved1: [u32; 1],
    /// Enable Request Register
    pub ERQ: RWRegister<u32>,
    _reserved2: [u32; 1],
    /// Enable Error Interrupt Register
    pub EEI: RWRegister<u32>,
    /// Clear Enable Error Interrupt Register
    pub CEEI: WORegister<u8>,
    /// Set Enable Error Interrupt Register
    pub SEEI: WORegister<u8>,
    /// Clear Enable Request Register
    pub CERQ: WORegister<u8>,
    /// Set Enable Request Register
    pub SERQ: WORegister<u8>,
    /// Clear DONE Status Bit Register
    pub CDNE: WORegister<u8>,
    /// Set START Bit Register
    pub SSRT: WORegister<u8>,
    /// Clear Error Register
    pub CERR: WORegister<u8>,
    /// Clear Interrupt Request Register
    pub CINT: WORegister<u8>,
    _reserved3: [u32; 1],
    /// Interrupt Request Register
    pub INT: RWRegister<u32>,
    _reserved4: [u32; 1],
    /// Error Register
    pub ERR: RWRegister<u32>,
    _reserved5: [u32; 1],
    /// Hardware Request Status Register
    pub HRS: RORegister<u32>,
    _reserved6: [u32; 3],
    /// Enable Asynchronous Request in Stop Register
    pub EARS: RWRegister<u32>,
    _reserved7: [u32; 46],
    /// Channel Priority Registers
    pub DCHPRI: ChannelPriorityRegisters,
    _reserved8: [u32; 952],
    /// Transfer Control Descriptors
    pub TCD: [tcd::RegisterBlock; CHANNEL_COUNT],
}

/// Wrapper for channel priority registers
///
/// Channel priority registers cannot be accessed with
/// normal channel indexes. This adapter makes it so that
/// we *can* access them with channel indexes by converting
/// the channel number to a reference to the priority
/// register.
#[repr(transparent)]
pub struct ChannelPriorityRegisters([RWRegister<u8>; CHANNEL_COUNT]);

impl Index<usize> for ChannelPriorityRegisters {
    type Output = RWRegister<u8>;
    fn index(&self, channel: usize) -> &RWRegister<u8> {
        // Pattern follows
        //
        //   3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, ...
        //
        // for all channels. NXP keeping us on our toes.
        let idx = 4 * (channel / 4) + (3 - (channel % 4));
        &self.0[idx]
    }
}
