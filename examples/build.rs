//! This build script prepares the HAL build.

/// Defines a new board.
struct Board {
    /// Your board name.
    ///
    /// Must match the Cargo feature.
    name: &'static str,
    /// Select your chip.
    chip: Chip,
    /// Specify your board's flash LENGTH.
    flash_length: usize,
}

/// An i.MX RT chip with RAM memory regions.
struct Chip {
    /// Bytes in ITCM.
    itcm_length: usize,
    /// Bytes in DTCM.
    dtcm_length: usize,
    /// Bytes in OCRAM.
    ///
    /// For chips that support it, this should include
    /// the dedicated OCRAM2 length.
    ocram_length: usize,
    /// Chip family.
    family: &'static str,
    /// FCB offset expected by the hardware.
    ///
    /// Affects our boot section.
    fcb_offset: usize,
}

const IMXRT1010: Chip = Chip {
    itcm_length: kb(32),
    dtcm_length: kb(32),
    ocram_length: kb(64),
    family: "imxrt1010",
    fcb_offset: 0x400,
};

const IMXRT1060: Chip = Chip {
    itcm_length: kb(128),
    dtcm_length: kb(128),
    ocram_length: kb(256),
    family: "imxrt1060",
    fcb_offset: 0,
};

/// Add your boards here.
static BOARDS: &[Board] = &[
    Board {
        name: "teensy4",
        chip: IMXRT1060,
        flash_length: kb(1984),
    },
    Board {
        name: "imxrt1010evk",
        chip: IMXRT1010,
        flash_length: mb(16),
    },
];

////////////////////////////////////////////////////////////////////

use std::{collections::HashSet, env, fs::File, io::Write, path::PathBuf};

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "/* Board configuration for imxrt-hal-examples '{}' */",
            self.name
        )?;
        writeln!(f, "MEMORY {{")?;
        writeln!(
            f,
            "FLASH (RX) : ORIGIN = 0x60000000, LENGTH = {:#X}",
            self.flash_length
        )?;
        writeln!(
            f,
            "ITCM (RX) : ORIGIN = 0x00000000, LENGTH = {:#X}",
            self.chip.itcm_length
        )?;
        writeln!(
            f,
            "DTCM (RW) : ORIGIN = 0x20000000, LENGTH = {:#X}",
            self.chip.dtcm_length
        )?;
        writeln!(
            f,
            "OCRAM (RWX) : ORIGIN = 0x20200000, LENGTH = {:#X}",
            self.chip.ocram_length
        )?;
        writeln!(f, "}}")?;
        writeln!(f, "__fcb_offset = {:#X};", self.chip.fcb_offset)?;
        Ok(())
    }
}

impl Chip {
    fn emit_configuration(&self) {
        println!("cargo:rustc-cfg={}", self.family);
    }
}

const fn kb(kb: usize) -> usize {
    kb * 1024
}

const fn mb(mb: usize) -> usize {
    mb * 1024 * 1024
}

fn required_features() -> HashSet<String> {
    BOARDS.iter().map(|board| board.name.into()).collect()
}

fn extract_features() -> HashSet<String> {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .collect()
}

fn select_board(
    features: &HashSet<String>,
    required: &HashSet<String>,
) -> Result<&'static Board, String> {
    let enabled_required_features: HashSet<_> = required.intersection(features).collect();
    if enabled_required_features.is_empty() {
        Err(format!(
            "No required features selected; choose one from {:?}",
            required
        ))
    } else if enabled_required_features.len() > 1 {
        Err(format!(
            "Too may required features; choose one from {:?}",
            required
        ))
    } else {
        let board_name = enabled_required_features
            .into_iter()
            .next()
            .cloned()
            .expect("Has exactly one element");
        Ok(BOARDS
            .iter()
            .find(|board| board.name == board_name)
            .expect("Board name matches"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let features = extract_features();
    let required = required_features();

    let board = select_board(&features, &required)?;
    board.chip.emit_configuration();

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    println!("cargo:rustc-link-search={}", out_dir.display());

    let mut board_file = File::create(out_dir.join("board.x"))?;
    writeln!(board_file, "{}", board)?;

    let memory_script = include_bytes!("memory.x");
    let mut memory_file = File::create(out_dir.join("memory.x"))?;
    memory_file.write_all(memory_script)?;

    Ok(())
}
