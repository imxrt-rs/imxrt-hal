//! Tools for the imxrt-hal project.
//!
//! Right now, it only contains routines for flashing various boards.

use std::path::Path;

/// Result of flashing.
type Result = std::result::Result<(), Box<dyn std::error::Error>>;

/// Routine to flash a Teensy 4.
///
/// # Dependencies
///
/// - `rust-objcopy`
/// - `teensy_loader_cli`
///
/// Modified from the teensy4-rs project.
mod teensy4 {
    use std::{
        env,
        path::{Path, PathBuf},
        process::Command,
    };

    /// Loader configurations.
    ///
    /// You may override these values using environment variables.
    struct Configuration {
        /// `objcopy` program name.
        objcopy: String,
        /// `teensy_loader_cli` program name.
        loader: String,
    }

    impl Configuration {
        fn new() -> Self {
            let objcopy = env::var("TEENSY4RS_OBJCOPY").unwrap_or_else(|_| "rust-objcopy".into());
            let loader =
                env::var("TEENSY4RS_LOADER").unwrap_or_else(|_| "teensy_loader_cli".into());
            Self { objcopy, loader }
        }
    }

    /// Flash a Teensy 4.
    pub fn flash(elf_path: &Path) -> crate::Result {
        let mut hex_path = PathBuf::from(elf_path).clone();
        hex_path.set_extension("hex");

        let cfg = Configuration::new();

        Command::new(cfg.objcopy)
            .args(&["-O", "ihex"])
            .arg(&elf_path)
            .arg(&hex_path)
            .output()?;

        Command::new(cfg.loader)
            .args(&["-w", "-v", "--mcu=imxrt1062"])
            .arg(&hex_path)
            .spawn()?
            .wait()?;

        Ok(())
    }
}

/// Use pyOCD as the flashing tool.
///
/// # Dependencies
///
/// - pyOCD
///
mod pyocd {
    use std::{path::Path, process::Command};

    /// Flash a target using pyOCD.
    pub fn flash(elf_path: &Path, target: &str) -> crate::Result {
        Command::new("pyocd")
            .arg("load")
            .arg(&format!("--target={}", target))
            .arg("--format=elf")
            .arg(&elf_path)
            .spawn()?
            .wait()?;

        Ok(())
    }
}

/// Select a flasher for the provided binary.
///
/// The provided `elf_path` is expected to have the name of a board.
/// This is configured in the build / run aliases of the project.
pub fn flash(elf_path: &Path) -> crate::Result {
    for path in elf_path.ancestors() {
        if let Some(path) = path.file_name().and_then(|path| path.to_str()) {
            match path {
                "imxrt1010evk" => return pyocd::flash(elf_path, "mimxrt1010"),
                "teensy4" => return teensy4::flash(elf_path),
                _ => {}
            }
        }
    }

    Err(format!("No flasher defined for {}", elf_path.display()).into())
}
