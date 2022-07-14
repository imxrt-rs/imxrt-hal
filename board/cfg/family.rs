//! i.MX RT chip family definitions.

use crate::{cfg::interrupts, kb};

/// An i.MX RT chip family with RAM memory regions.
///
/// Memory region lengths represent the values with default fuses.
/// There's no FlexRAM runtime configuration.
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

#[allow(unused)] // Remove once used.
pub const IMXRT1015: Family = Family {
    // There's no entry for this chip in AN12077, and you'll
    // notice that I'm disagreeing with the fusemap table in the
    // 1015 RM. I think that entry in the fusemap table is wrong.
    // There's no way you can express 8 memory banks with 8 bits,
    // so I'm not sure where the fusemap is getting its information.
    // This memory layout mirrors the 1010.
    itcm_length: kb(32),
    dtcm_length: kb(32),
    ocram_length: kb(64),
    name: "imxrt1015",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};

#[allow(unused)] // Remove once used.
pub const IMXRT1020: Family = Family {
    itcm_length: kb(64),
    dtcm_length: kb(64),
    ocram_length: kb(128),
    name: "imxrt1020",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};

#[allow(unused)] // Remove once used.
pub const IMXRT1050: Family = Family {
    itcm_length: kb(128),
    dtcm_length: kb(128),
    ocram_length: kb(256),
    name: "imxrt1050",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};

pub const IMXRT1060: Family = Family {
    itcm_length: kb(128),
    dtcm_length: kb(128),
    ocram_length: /* FlexRAM OCRAM */ kb(256) + /* OCRAM2 */ kb(512),
    name: "imxrt1060",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};

#[allow(unused)] // Remove once used.
pub const IMXRT1064: Family = Family {
    itcm_length: kb(128),
    dtcm_length: kb(128),
    ocram_length: /* FlexRAM OCRAM */ kb(256) + /* OCRAM2 */ kb(512),
    name: "imxrt1064",
    fcb_offset: 0,
    interrupts: interrupts::Chip {
        dma_a: "DMA7_DMA23",
    },
};
