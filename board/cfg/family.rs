//! i.MX RT chip family definitions.

use crate::{cfg::interrupts, kb};

/// An i.MX RT chip family with RAM memory regions.
pub struct Family {
    /// Bytes in ITCM.
    pub itcm_length: usize,
    /// Bytes in DTCM.
    pub dtcm_length: usize,
    /// Bytes in OCRAM.
    ///
    /// For chips that support it, this should include
    /// the dedicated OCRAM2 length.
    pub ocram_length: usize,
    /// Chip family name.
    pub name: &'static str,
    /// FCB offset expected by the hardware.
    ///
    /// Affects our boot section.
    pub fcb_offset: usize,
    /// Interrupts that are common across all boards
    /// using this chip.
    pub interrupts: interrupts::Chip,
}

pub const IMXRT1010: Family = Family {
    itcm_length: kb(32),
    dtcm_length: kb(32),
    ocram_length: kb(64),
    name: "imxrt1010",
    fcb_offset: 0x400,
    interrupts: interrupts::Chip { dma_a: "DMA7" },
};

pub const IMXRT1060: Family = Family {
    itcm_length: kb(128),
    dtcm_length: kb(128),
    ocram_length: kb(256),
    name: "imxrt1060",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};
