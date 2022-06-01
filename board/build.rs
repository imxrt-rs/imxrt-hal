//! This build script prepares the HAL build.

use std::{collections::HashSet, env, fs::File, io::Write as _, path::PathBuf};

/// Configurations for imxrt-hal boards and chips
/// are in these submodules.
mod cfg {
    pub mod boards;
    pub mod family;
    pub mod interrupts;
}

use cfg::boards::{Board, BOARDS};

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
    board.emit_configuration();

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    println!("cargo:rustc-link-search={}", out_dir.display());

    let mut board_file = File::create(out_dir.join("board.x"))?;
    board.generate_memory_map(&mut board_file)?;

    let memory_script = include_bytes!("memory.x");
    let mut memory_file = File::create(out_dir.join("memory.x"))?;
    memory_file.write_all(memory_script)?;

    cfg::interrupts::generate(&board.interrupts, &board.family.interrupts, &out_dir)?;

    Ok(())
}
