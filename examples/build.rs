//! This build script prepares the HAL build.

type Board = &'static str;
type ChipFamily = &'static str;

/// When adding a new board, update this collection.
///
/// A "chip family" represents a group of related MCU parts.
/// For example, the 1051 and 1052 belong to the "1050" chip family.
static BOARD_TO_CHIP_FAMILY: &[(Board, ChipFamily)] =
    &[("imxrt1010evk", "imxrt1010"), ("teensy4", "imxrt1060")];

//
// Add linker script memory regions here.
//

type MemoryMap = &'static str;

const IMXRT1010_MEMORY: MemoryMap = r#"
MEMORY
{
    FLASH      (RX) : ORIGIN = 0x60000000, LENGTH = 16M
    /* Lengths determined by default fuse values. */
    /* See note in 9.6.1.1 to understand why this isn't zero. */
    ITCM       (RX) : ORIGIN = 0x00000004, LENGTH = 32K - 4
    DTCM       (RW) : ORIGIN = 0x20000000, LENGTH = 32K
    /* 32K reserved, starting at 0x20200000, for boot ROM. */
    OCRAM     (RWX) : ORIGIN = 0x20208000, LENGTH = 32K
}

__fcb_offset = 0x400;
"#;

const IMXRT1060_MEMORY: MemoryMap = r#"
MEMORY
{
    FLASH      (RX) : ORIGIN = 0x60000000, LENGTH = 1984K
    /* Lengths determined by default fuse values. */
    ITCM       (RX) : ORIGIN = 0x00000000, LENGTH = 128K
    DTCM       (RW) : ORIGIN = 0x20000000, LENGTH = 128K
    /* 32K reserved, starting at 0x20200000, for boot ROM. */
    OCRAM     (RWX) : ORIGIN = 0x20208000, LENGTH = 256K - 32K
}

__fcb_offset = 0;
"#;

/// Once you define your memory regions, updat this map.
static CHIP_TO_MEMORY: &[(ChipFamily, MemoryMap)] = &[
    ("imxrt1010", IMXRT1010_MEMORY),
    ("imxrt1060", IMXRT1060_MEMORY),
];

////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Write,
    path::PathBuf,
};

fn required_features() -> HashSet<String> {
    board_to_chip_family().keys().cloned().collect()
}

fn board_to_chip_family() -> HashMap<String, String> {
    BOARD_TO_CHIP_FAMILY
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn extract_features() -> HashSet<String> {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .collect()
}

fn chip_family_to_memory(chip_family: &str) -> Result<&'static str, String> {
    CHIP_TO_MEMORY
        .iter()
        .find(|(k, _)| *k == chip_family)
        .map(|(_, memory_region)| *memory_region)
        .ok_or_else(|| format!("Chip family {} has no memory regions! This is a maintainer error; please let us know.", chip_family))
}

fn check_required_features(
    features: &HashSet<String>,
    required: &HashSet<String>,
) -> Result<String, String> {
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
        Ok(enabled_required_features
            .into_iter()
            .next()
            .cloned()
            .expect("Has exactly one element"))
    }
}

fn emit_chip_config(board: &str) -> Result<String, String> {
    let chip_families = board_to_chip_family();
    if let Some(chip_family) = chip_families.get(board) {
        println!("cargo:rustc-cfg={}", chip_family);
        Ok(chip_family.clone())
    } else {
        Err(format!("Board {} was not configured with a chip family! This is a maintainer error; please let us know.", board))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let features = extract_features();
    let required = required_features();

    let board = check_required_features(&features, &required)?;
    let chip_family = emit_chip_config(&board)?;
    let chip_memory = chip_family_to_memory(&chip_family)?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    println!("cargo:rustc-link-search={}", out_dir.display());

    let mut board_file = File::create(out_dir.join("board.x"))?;
    board_file.write_all(chip_memory.as_bytes())?;

    let memory_script = include_bytes!("memory.x");
    let memory_script_xip = include_bytes!("memory_xip.x");
    let mut memory_file = File::create(out_dir.join("memory.x"))?;
    memory_file.write_all(if env::var("CARGO_FEATURE_XIP").is_ok() {
        memory_script_xip
    } else {
        memory_script
    })?;

    Ok(())
}
