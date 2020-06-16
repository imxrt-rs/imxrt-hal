#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCP register reference index
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DCP control register 0
pub mod CTRL {

    /// Per-channel interrupt enable bit
    pub mod CHANNEL_INTERRUPT_ENABLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: CH0
            pub const CH0: u32 = 0b00000001;

            /// 0b00000010: CH1
            pub const CH1: u32 = 0b00000010;

            /// 0b00000100: CH2
            pub const CH2: u32 = 0b00000100;

            /// 0b00001000: CH3
            pub const CH3: u32 = 0b00001000;
        }
    }

    /// Enable automatic context switching for the channels
    pub mod ENABLE_CONTEXT_SWITCHING {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The software must set this bit to enable the caching of contexts between the operations
    pub mod ENABLE_CONTEXT_CACHING {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations
    pub mod GATHER_RESIDUAL_WRITES {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates whether the SHA1/SHA2 functions are present.
    pub mod PRESENT_SHA {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Absent
            pub const Absent: u32 = 0b0;

            /// 0b1: Present
            pub const Present: u32 = 0b1;
        }
    }

    /// Indicates whether the crypto (cipher/hash) functions are present.
    pub mod PRESENT_CRYPTO {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRESENT_SHA::RW;
    }

    /// This bit must be set to zero for a normal operation
    pub mod CLKGATE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set this bit to zero to enable a normal DCP operation
    pub mod SFTRST {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP control register 0
pub mod CTRL_SET {
    pub use super::CTRL::CHANNEL_INTERRUPT_ENABLE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE_CONTEXT_CACHING;
    pub use super::CTRL::ENABLE_CONTEXT_SWITCHING;
    pub use super::CTRL::GATHER_RESIDUAL_WRITES;
    pub use super::CTRL::PRESENT_CRYPTO;
    pub use super::CTRL::PRESENT_SHA;
    pub use super::CTRL::SFTRST;
}

/// DCP control register 0
pub mod CTRL_CLR {
    pub use super::CTRL::CHANNEL_INTERRUPT_ENABLE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE_CONTEXT_CACHING;
    pub use super::CTRL::ENABLE_CONTEXT_SWITCHING;
    pub use super::CTRL::GATHER_RESIDUAL_WRITES;
    pub use super::CTRL::PRESENT_CRYPTO;
    pub use super::CTRL::PRESENT_SHA;
    pub use super::CTRL::SFTRST;
}

/// DCP control register 0
pub mod CTRL_TOG {
    pub use super::CTRL::CHANNEL_INTERRUPT_ENABLE;
    pub use super::CTRL::CLKGATE;
    pub use super::CTRL::ENABLE_CONTEXT_CACHING;
    pub use super::CTRL::ENABLE_CONTEXT_SWITCHING;
    pub use super::CTRL::GATHER_RESIDUAL_WRITES;
    pub use super::CTRL::PRESENT_CRYPTO;
    pub use super::CTRL::PRESENT_SHA;
    pub use super::CTRL::SFTRST;
}

/// DCP status register
pub mod STAT {

    /// Indicates which channels have pending interrupt requests
    pub mod IRQ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates which channels are ready to proceed with a transfer (the active channel is also included)
    pub mod READY_CHANNELS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: CH0
            pub const CH0: u32 = 0b00000001;

            /// 0b00000010: CH1
            pub const CH1: u32 = 0b00000010;

            /// 0b00000100: CH2
            pub const CH2: u32 = 0b00000100;

            /// 0b00001000: CH3
            pub const CH3: u32 = 0b00001000;
        }
    }

    /// Current (active) channel (encoded)
    pub mod CUR_CHANNEL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None
            pub const None: u32 = 0b0000;

            /// 0b0001: CH0
            pub const CH0: u32 = 0b0001;

            /// 0b0010: CH1
            pub const CH1: u32 = 0b0010;

            /// 0b0011: CH2
            pub const CH2: u32 = 0b0011;

            /// 0b0100: CH3
            pub const CH3: u32 = 0b0100;
        }
    }

    /// When set, it indicates that the OTP key is shifted from the fuse block and is ready for use.
    pub mod OTP_KEY_READY {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP status register
pub mod STAT_SET {
    pub use super::STAT::CUR_CHANNEL;
    pub use super::STAT::IRQ;
    pub use super::STAT::OTP_KEY_READY;
    pub use super::STAT::READY_CHANNELS;
}

/// DCP status register
pub mod STAT_CLR {
    pub use super::STAT::CUR_CHANNEL;
    pub use super::STAT::IRQ;
    pub use super::STAT::OTP_KEY_READY;
    pub use super::STAT::READY_CHANNELS;
}

/// DCP status register
pub mod STAT_TOG {
    pub use super::STAT::CUR_CHANNEL;
    pub use super::STAT::IRQ;
    pub use super::STAT::OTP_KEY_READY;
    pub use super::STAT::READY_CHANNELS;
}

/// DCP channel control register
pub mod CHANNELCTRL {

    /// Setting a bit in this field enables the DMA channel associated with it
    pub mod ENABLE_CHANNEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: CH0
            pub const CH0: u32 = 0b00000001;

            /// 0b00000010: CH1
            pub const CH1: u32 = 0b00000010;

            /// 0b00000100: CH2
            pub const CH2: u32 = 0b00000100;

            /// 0b00001000: CH3
            pub const CH3: u32 = 0b00001000;
        }
    }

    /// Setting a bit in this field causes the corresponding channel to have high-priority arbitration
    pub mod HIGH_PRIORITY_CHANNEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ENABLE_CHANNEL::RW;
    }

    /// Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt
    pub mod CH0_IRQ_MERGED {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel control register
pub mod CHANNELCTRL_SET {
    pub use super::CHANNELCTRL::CH0_IRQ_MERGED;
    pub use super::CHANNELCTRL::ENABLE_CHANNEL;
    pub use super::CHANNELCTRL::HIGH_PRIORITY_CHANNEL;
}

/// DCP channel control register
pub mod CHANNELCTRL_CLR {
    pub use super::CHANNELCTRL::CH0_IRQ_MERGED;
    pub use super::CHANNELCTRL::ENABLE_CHANNEL;
    pub use super::CHANNELCTRL::HIGH_PRIORITY_CHANNEL;
}

/// DCP channel control register
pub mod CHANNELCTRL_TOG {
    pub use super::CHANNELCTRL::CH0_IRQ_MERGED;
    pub use super::CHANNELCTRL::ENABLE_CHANNEL;
    pub use super::CHANNELCTRL::HIGH_PRIORITY_CHANNEL;
}

/// DCP capability 0 register
pub mod CAPABILITY0 {

    /// Encoded value indicating the number of key-storage locations implemented in the design
    pub mod NUM_KEYS {
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

    /// Encoded value indicating the number of channels implemented in the design
    pub mod NUM_CHANNELS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write to a 1 to disable the per-device unique key
    pub mod DISABLE_UNIQUE_KEY {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write to 1 to disable the decryption
    pub mod DISABLE_DECRYPT {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP capability 1 register
pub mod CAPABILITY1 {

    /// One-hot field indicating which cipher algorithms are available
    pub mod CIPHER_ALGORITHMS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000001: AES128
            pub const AES128: u32 = 0b0000000000000001;
        }
    }

    /// One-hot field indicating which hashing features are implemented in the hardware
    pub mod HASH_ALGORITHMS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000000000000001: SHA1
            pub const SHA1: u32 = 0b0000000000000001;

            /// 0b0000000000000010: CRC32
            pub const CRC32: u32 = 0b0000000000000010;

            /// 0b0000000000000100: SHA256
            pub const SHA256: u32 = 0b0000000000000100;
        }
    }
}

/// DCP context buffer pointer
pub mod CONTEXT {

    /// Context pointer address
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP key index
pub mod KEY {

    /// Key subword pointer
    pub mod SUBWORD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Key index pointer. The valid indices are 0-\[number_keys\].
    pub mod INDEX {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP key data
pub mod KEYDATA {

    /// Word 0 data for the key. This is the least-significant word.
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP work packet 0 status register
pub mod PACKET0 {

    /// Next pointer register
    pub mod ADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP work packet 1 status register
pub mod PACKET1 {

    /// Reflects whether the channel must issue an interrupt upon the completion of the packet.
    pub mod INTERRUPT {
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

    /// Reflects whether the channel's semaphore must be decremented at the end of the current operation
    pub mod DECR_SEMAPHORE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer
    pub mod CHAIN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the next packet's address is located following this packet's payload.
    pub mod CHAIN_CONTIGUOUS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the selected hashing function should be enabled for this operation.
    pub mod ENABLE_MEMCOPY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the selected cipher function must be enabled for this operation.
    pub mod ENABLE_CIPHER {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the selected hashing function must be enabled for this operation.
    pub mod ENABLE_HASH {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP must perform a blit operation
    pub mod ENABLE_BLIT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption
    pub mod CIPHER_ENCRYPT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DECRYPT
            pub const DECRYPT: u32 = 0b0;

            /// 0b1: ENCRYPT
            pub const ENCRYPT: u32 = 0b1;
        }
    }

    /// Reflects whether the cipher block must load the initialization vector from the payload for this operation
    pub mod CIPHER_INIT {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether a hardware-based key must be used
    pub mod OTP_KEY {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When set, it indicates the payload contains the key
    pub mod PAYLOAD_KEY {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation
    pub mod HASH_INIT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware
    pub mod HASH_TERM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the calculated hash value must be compared to the hash provided in the payload.
    pub mod CHECK_HASH {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When the hashing is enabled, this bit controls whether the input or output data is hashed.
    pub mod HASH_OUTPUT {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: INPUT
            pub const INPUT: u32 = 0b0;

            /// 0b1: OUTPUT
            pub const OUTPUT: u32 = 0b1;
        }
    }

    /// When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field
    pub mod CONSTANT_FILL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!
    pub mod TEST_SEMA_IRQ {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine swaps the key bytes (big-endian key).
    pub mod KEY_BYTESWAP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine swaps the key words (big-endian key).
    pub mod KEY_WORDSWAP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine byteswaps the input data (big-endian data).
    pub mod INPUT_BYTESWAP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine wordswaps the input data (big-endian data).
    pub mod INPUT_WORDSWAP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine byteswaps the output data (big-endian data).
    pub mod OUTPUT_BYTESWAP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reflects whether the DCP engine wordswaps the output data (big-endian data).
    pub mod OUTPUT_WORDSWAP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet Tag
    pub mod TAG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP work packet 2 status register
pub mod PACKET2 {

    /// Cipher selection field
    pub mod CIPHER_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AES128
            pub const AES128: u32 = 0b0000;
        }
    }

    /// Cipher mode selection field. Reflects the mode of operation for the cipher operations.
    pub mod CIPHER_MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: ECB
            pub const ECB: u32 = 0b0000;

            /// 0b0001: CBC
            pub const CBC: u32 = 0b0001;
        }
    }

    /// Key selection field
    pub mod KEY_SELECT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000000: KEY0
            pub const KEY0: u32 = 0b00000000;

            /// 0b00000001: KEY1
            pub const KEY1: u32 = 0b00000001;

            /// 0b00000010: KEY2
            pub const KEY2: u32 = 0b00000010;

            /// 0b00000011: KEY3
            pub const KEY3: u32 = 0b00000011;

            /// 0b11111110: UNIQUE_KEY
            pub const UNIQUE_KEY: u32 = 0b11111110;

            /// 0b11111111: OTP_KEY
            pub const OTP_KEY: u32 = 0b11111111;
        }
    }

    /// Hash Selection Field
    pub mod HASH_SELECT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SHA1
            pub const SHA1: u32 = 0b0000;

            /// 0b0001: CRC32
            pub const CRC32: u32 = 0b0001;

            /// 0b0010: SHA256
            pub const SHA256: u32 = 0b0010;
        }
    }

    /// Cipher configuration bits. Optional configuration bits are required for the ciphers.
    pub mod CIPHER_CFG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP work packet 3 status register
pub mod PACKET3 {
    pub use super::PACKET0::ADDR;
}

/// DCP work packet 4 status register
pub mod PACKET4 {
    pub use super::PACKET0::ADDR;
}

/// DCP work packet 5 status register
pub mod PACKET5 {

    /// Byte count register. This value is the working value and updates as the operation proceeds.
    pub mod COUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP work packet 6 status register
pub mod PACKET6 {
    pub use super::PACKET0::ADDR;
}

/// DCP channel 0 command pointer address register
pub mod CH0CMDPTR {
    pub use super::CONTEXT::ADDR;
}

/// DCP channel 0 semaphore register
pub mod CH0SEMA {

    /// The value written to this field is added to the semaphore count in an atomic way such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected
    pub mod INCREMENT {
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

    /// This read-only field shows the current (instantaneous) value of the semaphore counter.
    pub mod VALUE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 0 status register
pub mod CH0STAT {

    /// This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit
    pub mod HASH_MISMATCH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)
    pub mod ERROR_SETUP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload
    pub mod ERROR_PACKET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading from the source buffer
    pub mod ERROR_SRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when storing to the destination buffer
    pub mod ERROR_DST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a page fault occurred while converting a virtual address to a physical address
    pub mod ERROR_PAGEFAULT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the additional error codes for some of the error conditions
    pub mod ERROR_CODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: Error signalled because the next pointer is 0x00000000
            pub const NEXT_CHAIN_IS_0: u32 = 0b00000001;

            /// 0b00000010: Error signalled because the semaphore is non-zero and neither chain bit is set
            pub const NO_CHAIN: u32 = 0b00000010;

            /// 0b00000011: Error signalled because an error is reported reading/writing the context buffer
            pub const CONTEXT_ERROR: u32 = 0b00000011;

            /// 0b00000100: Error signalled because an error is reported reading/writing the payload
            pub const PAYLOAD_ERROR: u32 = 0b00000100;

            /// 0b00000101: Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)
            pub const INVALID_MODE: u32 = 0b00000101;
        }
    }

    /// Indicates the tag from the last completed packet in the command structure
    pub mod TAG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 0 status register
pub mod CH0STAT_SET {
    pub use super::CH0STAT::ERROR_CODE;
    pub use super::CH0STAT::ERROR_DST;
    pub use super::CH0STAT::ERROR_PACKET;
    pub use super::CH0STAT::ERROR_PAGEFAULT;
    pub use super::CH0STAT::ERROR_SETUP;
    pub use super::CH0STAT::ERROR_SRC;
    pub use super::CH0STAT::HASH_MISMATCH;
    pub use super::CH0STAT::TAG;
}

/// DCP channel 0 status register
pub mod CH0STAT_CLR {
    pub use super::CH0STAT::ERROR_CODE;
    pub use super::CH0STAT::ERROR_DST;
    pub use super::CH0STAT::ERROR_PACKET;
    pub use super::CH0STAT::ERROR_PAGEFAULT;
    pub use super::CH0STAT::ERROR_SETUP;
    pub use super::CH0STAT::ERROR_SRC;
    pub use super::CH0STAT::HASH_MISMATCH;
    pub use super::CH0STAT::TAG;
}

/// DCP channel 0 status register
pub mod CH0STAT_TOG {
    pub use super::CH0STAT::ERROR_CODE;
    pub use super::CH0STAT::ERROR_DST;
    pub use super::CH0STAT::ERROR_PACKET;
    pub use super::CH0STAT::ERROR_PAGEFAULT;
    pub use super::CH0STAT::ERROR_SETUP;
    pub use super::CH0STAT::ERROR_SRC;
    pub use super::CH0STAT::HASH_MISMATCH;
    pub use super::CH0STAT::TAG;
}

/// DCP channel 0 options register
pub mod CH0OPTS {

    /// This field indicates the recovery time for the channel
    pub mod RECOVERY_TIMER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 0 options register
pub mod CH0OPTS_SET {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 0 options register
pub mod CH0OPTS_CLR {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 0 options register
pub mod CH0OPTS_TOG {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 1 command pointer address register
pub mod CH1CMDPTR {
    pub use super::CONTEXT::ADDR;
}

/// DCP channel 1 semaphore register
pub mod CH1SEMA {
    pub use super::CH0SEMA::INCREMENT;
    pub use super::CH0SEMA::VALUE;
}

/// DCP channel 1 status register
pub mod CH1STAT {

    /// This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit
    pub mod HASH_MISMATCH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)
    pub mod ERROR_SETUP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod
    pub mod ERROR_PACKET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading from the source buffer
    pub mod ERROR_SRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when storing to the destination buffer
    pub mod ERROR_DST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a page fault occurred while converting a virtual address to a physical address
    pub mod ERROR_PAGEFAULT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the additional error codes for some of the error conditions.
    pub mod ERROR_CODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: Error is signalled because the next pointer is 0x00000000.
            pub const NEXT_CHAIN_IS_0: u32 = 0b00000001;

            /// 0b00000010: Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set.
            pub const NO_CHAIN: u32 = 0b00000010;

            /// 0b00000011: Error is signalled because an error was reported when reading/writing the context buffer.
            pub const CONTEXT_ERROR: u32 = 0b00000011;

            /// 0b00000100: Error is signalled because an error was reported when reading/writing the payload.
            pub const PAYLOAD_ERROR: u32 = 0b00000100;

            /// 0b00000101: Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash).
            pub const INVALID_MODE: u32 = 0b00000101;
        }
    }

    /// Indicates the tag from the last completed packet in the command structure.
    pub mod TAG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 1 status register
pub mod CH1STAT_SET {
    pub use super::CH1STAT::ERROR_CODE;
    pub use super::CH1STAT::ERROR_DST;
    pub use super::CH1STAT::ERROR_PACKET;
    pub use super::CH1STAT::ERROR_PAGEFAULT;
    pub use super::CH1STAT::ERROR_SETUP;
    pub use super::CH1STAT::ERROR_SRC;
    pub use super::CH1STAT::HASH_MISMATCH;
    pub use super::CH1STAT::TAG;
}

/// DCP channel 1 status register
pub mod CH1STAT_CLR {
    pub use super::CH1STAT::ERROR_CODE;
    pub use super::CH1STAT::ERROR_DST;
    pub use super::CH1STAT::ERROR_PACKET;
    pub use super::CH1STAT::ERROR_PAGEFAULT;
    pub use super::CH1STAT::ERROR_SETUP;
    pub use super::CH1STAT::ERROR_SRC;
    pub use super::CH1STAT::HASH_MISMATCH;
    pub use super::CH1STAT::TAG;
}

/// DCP channel 1 status register
pub mod CH1STAT_TOG {
    pub use super::CH1STAT::ERROR_CODE;
    pub use super::CH1STAT::ERROR_DST;
    pub use super::CH1STAT::ERROR_PACKET;
    pub use super::CH1STAT::ERROR_PAGEFAULT;
    pub use super::CH1STAT::ERROR_SETUP;
    pub use super::CH1STAT::ERROR_SRC;
    pub use super::CH1STAT::HASH_MISMATCH;
    pub use super::CH1STAT::TAG;
}

/// DCP channel 1 options register
pub mod CH1OPTS {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 1 options register
pub mod CH1OPTS_SET {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 1 options register
pub mod CH1OPTS_CLR {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 1 options register
pub mod CH1OPTS_TOG {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 2 command pointer address register
pub mod CH2CMDPTR {
    pub use super::CONTEXT::ADDR;
}

/// DCP channel 2 semaphore register
pub mod CH2SEMA {
    pub use super::CH0SEMA::INCREMENT;
    pub use super::CH0SEMA::VALUE;
}

/// DCP channel 2 status register
pub mod CH2STAT {

    /// This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit
    pub mod HASH_MISMATCH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)
    pub mod ERROR_SETUP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod
    pub mod ERROR_PACKET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading from the source buffer
    pub mod ERROR_SRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when storing to the destination buffer
    pub mod ERROR_DST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a page fault occurred while converting a virtual address to a physical address
    pub mod ERROR_PAGEFAULT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates additional error codes for some of the error conditions.
    pub mod ERROR_CODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: Error is signalled because the next pointer is 0x00000000.
            pub const NEXT_CHAIN_IS_0: u32 = 0b00000001;

            /// 0b00000010: Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set.
            pub const NO_CHAIN: u32 = 0b00000010;

            /// 0b00000011: Error is signalled because an error was reported while reading/writing the context buffer.
            pub const CONTEXT_ERROR: u32 = 0b00000011;

            /// 0b00000100: Error is signalled because an error was reported while reading/writing the payload.
            pub const PAYLOAD_ERROR: u32 = 0b00000100;

            /// 0b00000101: Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash).
            pub const INVALID_MODE: u32 = 0b00000101;
        }
    }

    /// Indicates the tag from the last completed packet in the command structure.
    pub mod TAG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 2 status register
pub mod CH2STAT_SET {
    pub use super::CH2STAT::ERROR_CODE;
    pub use super::CH2STAT::ERROR_DST;
    pub use super::CH2STAT::ERROR_PACKET;
    pub use super::CH2STAT::ERROR_PAGEFAULT;
    pub use super::CH2STAT::ERROR_SETUP;
    pub use super::CH2STAT::ERROR_SRC;
    pub use super::CH2STAT::HASH_MISMATCH;
    pub use super::CH2STAT::TAG;
}

/// DCP channel 2 status register
pub mod CH2STAT_CLR {
    pub use super::CH2STAT::ERROR_CODE;
    pub use super::CH2STAT::ERROR_DST;
    pub use super::CH2STAT::ERROR_PACKET;
    pub use super::CH2STAT::ERROR_PAGEFAULT;
    pub use super::CH2STAT::ERROR_SETUP;
    pub use super::CH2STAT::ERROR_SRC;
    pub use super::CH2STAT::HASH_MISMATCH;
    pub use super::CH2STAT::TAG;
}

/// DCP channel 2 status register
pub mod CH2STAT_TOG {
    pub use super::CH2STAT::ERROR_CODE;
    pub use super::CH2STAT::ERROR_DST;
    pub use super::CH2STAT::ERROR_PACKET;
    pub use super::CH2STAT::ERROR_PAGEFAULT;
    pub use super::CH2STAT::ERROR_SETUP;
    pub use super::CH2STAT::ERROR_SRC;
    pub use super::CH2STAT::HASH_MISMATCH;
    pub use super::CH2STAT::TAG;
}

/// DCP channel 2 options register
pub mod CH2OPTS {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 2 options register
pub mod CH2OPTS_SET {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 2 options register
pub mod CH2OPTS_CLR {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 2 options register
pub mod CH2OPTS_TOG {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 3 command pointer address register
pub mod CH3CMDPTR {
    pub use super::CONTEXT::ADDR;
}

/// DCP channel 3 semaphore register
pub mod CH3SEMA {
    pub use super::CH0SEMA::INCREMENT;
    pub use super::CH0SEMA::VALUE;
}

/// DCP channel 3 status register
pub mod CH3STAT {

    /// This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit
    pub mod HASH_MISMATCH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)
    pub mod ERROR_SETUP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading the packet or payload or when writing the status back to the packet paylaod
    pub mod ERROR_PACKET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when reading from the source buffer
    pub mod ERROR_SRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a bus error occurred when storing to the destination buffer
    pub mod ERROR_DST {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit indicates that a page fault occurred while converting a virtual address to a physical address
    pub mod ERROR_PAGEFAULT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates additional error codes for some of the error conditions.
    pub mod ERROR_CODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: Error is signalled because the next pointer is 0x00000000.
            pub const NEXT_CHAIN_IS_0: u32 = 0b00000001;

            /// 0b00000010: Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set.
            pub const NO_CHAIN: u32 = 0b00000010;

            /// 0b00000011: Error is signalled because an error was reported while reading/writing the context buffer.
            pub const CONTEXT_ERROR: u32 = 0b00000011;

            /// 0b00000100: Error is signalled because an error was reported while reading/writing the payload.
            pub const PAYLOAD_ERROR: u32 = 0b00000100;

            /// 0b00000101: Error is signalled because the control packet specifies an invalid mode select (for example, blit + hash).
            pub const INVALID_MODE: u32 = 0b00000101;
        }
    }

    /// Indicates the tag from the last completed packet in the command structure.
    pub mod TAG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP channel 3 status register
pub mod CH3STAT_SET {
    pub use super::CH3STAT::ERROR_CODE;
    pub use super::CH3STAT::ERROR_DST;
    pub use super::CH3STAT::ERROR_PACKET;
    pub use super::CH3STAT::ERROR_PAGEFAULT;
    pub use super::CH3STAT::ERROR_SETUP;
    pub use super::CH3STAT::ERROR_SRC;
    pub use super::CH3STAT::HASH_MISMATCH;
    pub use super::CH3STAT::TAG;
}

/// DCP channel 3 status register
pub mod CH3STAT_CLR {
    pub use super::CH3STAT::ERROR_CODE;
    pub use super::CH3STAT::ERROR_DST;
    pub use super::CH3STAT::ERROR_PACKET;
    pub use super::CH3STAT::ERROR_PAGEFAULT;
    pub use super::CH3STAT::ERROR_SETUP;
    pub use super::CH3STAT::ERROR_SRC;
    pub use super::CH3STAT::HASH_MISMATCH;
    pub use super::CH3STAT::TAG;
}

/// DCP channel 3 status register
pub mod CH3STAT_TOG {
    pub use super::CH3STAT::ERROR_CODE;
    pub use super::CH3STAT::ERROR_DST;
    pub use super::CH3STAT::ERROR_PACKET;
    pub use super::CH3STAT::ERROR_PAGEFAULT;
    pub use super::CH3STAT::ERROR_SETUP;
    pub use super::CH3STAT::ERROR_SRC;
    pub use super::CH3STAT::HASH_MISMATCH;
    pub use super::CH3STAT::TAG;
}

/// DCP channel 3 options register
pub mod CH3OPTS {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 3 options register
pub mod CH3OPTS_SET {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 3 options register
pub mod CH3OPTS_CLR {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP channel 3 options register
pub mod CH3OPTS_TOG {
    pub use super::CH0OPTS::RECOVERY_TIMER;
}

/// DCP debug select register
pub mod DBGSELECT {

    /// Selects a value to read via the debug data register.
    pub mod INDEX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000001: CONTROL
            pub const CONTROL: u32 = 0b00000001;

            /// 0b00010000: OTPKEY0
            pub const OTPKEY0: u32 = 0b00010000;

            /// 0b00010001: OTPKEY1
            pub const OTPKEY1: u32 = 0b00010001;

            /// 0b00010010: OTPKEY2
            pub const OTPKEY2: u32 = 0b00010010;

            /// 0b00010011: OTPKEY3
            pub const OTPKEY3: u32 = 0b00010011;
        }
    }
}

/// DCP debug data register
pub mod DBGDATA {

    /// Debug data
    pub mod DATA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP page table register
pub mod PAGETABLE {

    /// Page table enable control
    pub mod ENABLE {
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

    /// Page table flush control. To flush the TLB, write this bit to 1 and then back to 0.
    pub mod FLUSH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Page table base address
    pub mod BASE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (30 bits: 0x3fffffff << 2)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DCP version register
pub mod VERSION {

    /// Fixed read-only value reflecting the stepping of the version of the design implementation.
    pub mod STEP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fixed read-only value reflecting the MINOR version of the design implementation.
    pub mod MINOR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fixed read-only value reflecting the MAJOR version of the design implementation.
    pub mod MAJOR {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// DCP control register 0
    pub CTRL: RWRegister<u32>,

    /// DCP control register 0
    pub CTRL_SET: RWRegister<u32>,

    /// DCP control register 0
    pub CTRL_CLR: RWRegister<u32>,

    /// DCP control register 0
    pub CTRL_TOG: RWRegister<u32>,

    /// DCP status register
    pub STAT: RWRegister<u32>,

    /// DCP status register
    pub STAT_SET: RWRegister<u32>,

    /// DCP status register
    pub STAT_CLR: RWRegister<u32>,

    /// DCP status register
    pub STAT_TOG: RWRegister<u32>,

    /// DCP channel control register
    pub CHANNELCTRL: RWRegister<u32>,

    /// DCP channel control register
    pub CHANNELCTRL_SET: RWRegister<u32>,

    /// DCP channel control register
    pub CHANNELCTRL_CLR: RWRegister<u32>,

    /// DCP channel control register
    pub CHANNELCTRL_TOG: RWRegister<u32>,

    /// DCP capability 0 register
    pub CAPABILITY0: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// DCP capability 1 register
    pub CAPABILITY1: RORegister<u32>,

    _reserved2: [u32; 3],

    /// DCP context buffer pointer
    pub CONTEXT: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// DCP key index
    pub KEY: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// DCP key data
    pub KEYDATA: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// DCP work packet 0 status register
    pub PACKET0: RORegister<u32>,

    _reserved6: [u32; 3],

    /// DCP work packet 1 status register
    pub PACKET1: RORegister<u32>,

    _reserved7: [u32; 3],

    /// DCP work packet 2 status register
    pub PACKET2: RORegister<u32>,

    _reserved8: [u32; 3],

    /// DCP work packet 3 status register
    pub PACKET3: RORegister<u32>,

    _reserved9: [u32; 3],

    /// DCP work packet 4 status register
    pub PACKET4: RORegister<u32>,

    _reserved10: [u32; 3],

    /// DCP work packet 5 status register
    pub PACKET5: RORegister<u32>,

    _reserved11: [u32; 3],

    /// DCP work packet 6 status register
    pub PACKET6: RORegister<u32>,

    _reserved12: [u32; 7],

    /// DCP channel 0 command pointer address register
    pub CH0CMDPTR: RWRegister<u32>,

    _reserved13: [u32; 3],

    /// DCP channel 0 semaphore register
    pub CH0SEMA: RWRegister<u32>,

    _reserved14: [u32; 3],

    /// DCP channel 0 status register
    pub CH0STAT: RWRegister<u32>,

    /// DCP channel 0 status register
    pub CH0STAT_SET: RWRegister<u32>,

    /// DCP channel 0 status register
    pub CH0STAT_CLR: RWRegister<u32>,

    /// DCP channel 0 status register
    pub CH0STAT_TOG: RWRegister<u32>,

    /// DCP channel 0 options register
    pub CH0OPTS: RWRegister<u32>,

    /// DCP channel 0 options register
    pub CH0OPTS_SET: RWRegister<u32>,

    /// DCP channel 0 options register
    pub CH0OPTS_CLR: RWRegister<u32>,

    /// DCP channel 0 options register
    pub CH0OPTS_TOG: RWRegister<u32>,

    /// DCP channel 1 command pointer address register
    pub CH1CMDPTR: RWRegister<u32>,

    _reserved15: [u32; 3],

    /// DCP channel 1 semaphore register
    pub CH1SEMA: RWRegister<u32>,

    _reserved16: [u32; 3],

    /// DCP channel 1 status register
    pub CH1STAT: RWRegister<u32>,

    /// DCP channel 1 status register
    pub CH1STAT_SET: RWRegister<u32>,

    /// DCP channel 1 status register
    pub CH1STAT_CLR: RWRegister<u32>,

    /// DCP channel 1 status register
    pub CH1STAT_TOG: RWRegister<u32>,

    /// DCP channel 1 options register
    pub CH1OPTS: RWRegister<u32>,

    /// DCP channel 1 options register
    pub CH1OPTS_SET: RWRegister<u32>,

    /// DCP channel 1 options register
    pub CH1OPTS_CLR: RWRegister<u32>,

    /// DCP channel 1 options register
    pub CH1OPTS_TOG: RWRegister<u32>,

    /// DCP channel 2 command pointer address register
    pub CH2CMDPTR: RWRegister<u32>,

    _reserved17: [u32; 3],

    /// DCP channel 2 semaphore register
    pub CH2SEMA: RWRegister<u32>,

    _reserved18: [u32; 3],

    /// DCP channel 2 status register
    pub CH2STAT: RWRegister<u32>,

    /// DCP channel 2 status register
    pub CH2STAT_SET: RWRegister<u32>,

    /// DCP channel 2 status register
    pub CH2STAT_CLR: RWRegister<u32>,

    /// DCP channel 2 status register
    pub CH2STAT_TOG: RWRegister<u32>,

    /// DCP channel 2 options register
    pub CH2OPTS: RWRegister<u32>,

    /// DCP channel 2 options register
    pub CH2OPTS_SET: RWRegister<u32>,

    /// DCP channel 2 options register
    pub CH2OPTS_CLR: RWRegister<u32>,

    /// DCP channel 2 options register
    pub CH2OPTS_TOG: RWRegister<u32>,

    /// DCP channel 3 command pointer address register
    pub CH3CMDPTR: RWRegister<u32>,

    _reserved19: [u32; 3],

    /// DCP channel 3 semaphore register
    pub CH3SEMA: RWRegister<u32>,

    _reserved20: [u32; 3],

    /// DCP channel 3 status register
    pub CH3STAT: RWRegister<u32>,

    /// DCP channel 3 status register
    pub CH3STAT_SET: RWRegister<u32>,

    /// DCP channel 3 status register
    pub CH3STAT_CLR: RWRegister<u32>,

    /// DCP channel 3 status register
    pub CH3STAT_TOG: RWRegister<u32>,

    /// DCP channel 3 options register
    pub CH3OPTS: RWRegister<u32>,

    /// DCP channel 3 options register
    pub CH3OPTS_SET: RWRegister<u32>,

    /// DCP channel 3 options register
    pub CH3OPTS_CLR: RWRegister<u32>,

    /// DCP channel 3 options register
    pub CH3OPTS_TOG: RWRegister<u32>,

    _reserved21: [u32; 128],

    /// DCP debug select register
    pub DBGSELECT: RWRegister<u32>,

    _reserved22: [u32; 3],

    /// DCP debug data register
    pub DBGDATA: RORegister<u32>,

    _reserved23: [u32; 3],

    /// DCP page table register
    pub PAGETABLE: RWRegister<u32>,

    _reserved24: [u32; 3],

    /// DCP version register
    pub VERSION: RORegister<u32>,
}
pub struct ResetValues {
    pub CTRL: u32,
    pub CTRL_SET: u32,
    pub CTRL_CLR: u32,
    pub CTRL_TOG: u32,
    pub STAT: u32,
    pub STAT_SET: u32,
    pub STAT_CLR: u32,
    pub STAT_TOG: u32,
    pub CHANNELCTRL: u32,
    pub CHANNELCTRL_SET: u32,
    pub CHANNELCTRL_CLR: u32,
    pub CHANNELCTRL_TOG: u32,
    pub CAPABILITY0: u32,
    pub CAPABILITY1: u32,
    pub CONTEXT: u32,
    pub KEY: u32,
    pub KEYDATA: u32,
    pub PACKET0: u32,
    pub PACKET1: u32,
    pub PACKET2: u32,
    pub PACKET3: u32,
    pub PACKET4: u32,
    pub PACKET5: u32,
    pub PACKET6: u32,
    pub CH0CMDPTR: u32,
    pub CH0SEMA: u32,
    pub CH0STAT: u32,
    pub CH0STAT_SET: u32,
    pub CH0STAT_CLR: u32,
    pub CH0STAT_TOG: u32,
    pub CH0OPTS: u32,
    pub CH0OPTS_SET: u32,
    pub CH0OPTS_CLR: u32,
    pub CH0OPTS_TOG: u32,
    pub CH1CMDPTR: u32,
    pub CH1SEMA: u32,
    pub CH1STAT: u32,
    pub CH1STAT_SET: u32,
    pub CH1STAT_CLR: u32,
    pub CH1STAT_TOG: u32,
    pub CH1OPTS: u32,
    pub CH1OPTS_SET: u32,
    pub CH1OPTS_CLR: u32,
    pub CH1OPTS_TOG: u32,
    pub CH2CMDPTR: u32,
    pub CH2SEMA: u32,
    pub CH2STAT: u32,
    pub CH2STAT_SET: u32,
    pub CH2STAT_CLR: u32,
    pub CH2STAT_TOG: u32,
    pub CH2OPTS: u32,
    pub CH2OPTS_SET: u32,
    pub CH2OPTS_CLR: u32,
    pub CH2OPTS_TOG: u32,
    pub CH3CMDPTR: u32,
    pub CH3SEMA: u32,
    pub CH3STAT: u32,
    pub CH3STAT_SET: u32,
    pub CH3STAT_CLR: u32,
    pub CH3STAT_TOG: u32,
    pub CH3OPTS: u32,
    pub CH3OPTS_SET: u32,
    pub CH3OPTS_CLR: u32,
    pub CH3OPTS_TOG: u32,
    pub DBGSELECT: u32,
    pub DBGDATA: u32,
    pub PAGETABLE: u32,
    pub VERSION: u32,
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
