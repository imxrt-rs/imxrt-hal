use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let elf_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("Supply the path to an ELF program")?;
    tools::flash(&elf_path)?;
    Ok(())
}
