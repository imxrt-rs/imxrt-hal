//! This build script prepares the HAL build.

use imxrt_rt::{Family, Memory, RuntimeBuilder};
use std::{collections::HashSet, env, fs, path::PathBuf};

fn extract_features() -> HashSet<String> {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .collect()
}

fn emit_cfg_checks<F>(cfg: &str, values: impl IntoIterator<Item = F>)
where
    F: std::fmt::Display,
{
    let quoted: Vec<String> = values
        .into_iter()
        .map(|value| format!("\"{value}\""))
        .collect();
    let joined = quoted.join(", ");
    // Single ":" permitted for backwards compatibility.
    println!("cargo:rustc-check-cfg=cfg({cfg}, values({joined}))");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from)?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::write(out_dir.join("device.x"), DEVICE_X)?;

    emit_cfg_checks(
        "board",
        [
            "teensy4",
            "imxrt1010evk",
            "imxrt1170evk-cm7",
            "imxrt1060evk",
            "imxrt1180evk-cm33",
            "vmu-rt1170-cm7",
            "coral-dev-board-micro-cm7",
        ],
    );
    emit_cfg_checks("chip", ["imxrt1010", "imxrt1060", "imxrt1170", "imxrt1180"]);

    let features = extract_features();
    for feature in features {
        match feature.as_str() {
            "teensy4" => {
                RuntimeBuilder::from_flexspi(Family::Imxrt1060, 1984 * 1024)
                    .rodata(Memory::Dtcm)
                    .stack_size(32 * 1024)
                    .build()?;
                println!("cargo:rustc-cfg=board=\"teensy4\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1060\"");
            }
            "imxrt1010evk" => {
                RuntimeBuilder::from_flexspi(Family::Imxrt1010, 16 * 1024 * 1024)
                    .flexram_banks(imxrt_rt::FlexRamBanks {
                        ocram: 1,
                        itcm: 2,
                        dtcm: 1,
                    })
                    .uninit(Memory::Dtcm)
                    .stack_size(16 * 1024)
                    .build()?;
                println!("cargo:rustc-cfg=board=\"imxrt1010evk\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1010\"");
            }
            "imxrt1060evk" => {
                RuntimeBuilder::from_flexspi(Family::Imxrt1060, 8 * 1024 * 1024)
                    .rodata(Memory::Dtcm)
                    .stack_size(32 * 1024)
                    .build()?;
                println!("cargo:rustc-cfg=board=\"imxrt1060evk\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1060\"");
            }
            // Dashes replaced by underscores when signaled through
            // environment variables.
            "imxrt1170evk_cm7" => {
                imxrt_rt::RuntimeBuilder::from_flexspi(
                    imxrt_rt::Family::Imxrt1170,
                    16 * 1024 * 1024,
                )
                .rodata(imxrt_rt::Memory::Dtcm)
                .stack_size(32 * 1024)
                .build()?;
                println!("cargo:rustc-cfg=board=\"imxrt1170evk-cm7\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1170\"");
            }
            "imxrt1180evk_cm33" => {
                imxrt_rt::RuntimeBuilder::from_flexspi(
                    imxrt_rt::Family::Imxrt1180,
                    16 * 1024 * 1024,
                )
                .rodata(imxrt_rt::Memory::Dtcm)
                .build()?;
                println!("cargo:rustc-cfg=board=\"imxrt1180evk-cm33\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1180\"");
            }
            "vmu_rt1170_cm7" => {
                imxrt_rt::RuntimeBuilder::from_flexspi(
                    imxrt_rt::Family::Imxrt1170,
                    16 * 1024 * 1024,
                )
                .rodata(imxrt_rt::Memory::Dtcm)
                .stack_size(32 * 1024)
                .build()?;
                println!("cargo:rustc-cfg=board=\"vmu-rt1170-cm7\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1170\"");
                println!("cargo:rustc-cfg=family=\"imxrt11xx\"");
            }
            "coral_dev_board_micro_cm7" => {
                // The map mirrors `coralmicro`'s own application linker script,
                // libs/nxp/rt1176-sdk/MIMXRT1176xxxxx_cm7_ram.ld. Vectors and
                // text go in ITCM from 0x800: coralmicro reserves the bottom of
                // ITCM, and `cortex-m-rt` places the vector table at
                // ORIGIN(FLASH). Data, bss and the stack go in the 256K of DTCM.
                //
                // Neither OCRAM2, which holds the loader, nor SDRAM, where the
                // loader stages this image, is mapped.
                loader_booted_runtime(
                    &out_dir,
                    Region {
                        origin: 0x0000_0800,
                        length: 0x0003_F800,
                    },
                    Region {
                        origin: 0x2000_0000,
                        length: 0x0004_0000,
                    },
                )?;
                println!("cargo:rustc-cfg=board=\"coral-dev-board-micro-cm7\"");
                println!("cargo:rustc-cfg=chip=\"imxrt1170\"");
            }

            _ => continue,
        }
        return Ok(());
    }
    Err("Board feature not associated to a runtime.".into())
}

/// A memory region for [`loader_booted_runtime`].
struct Region {
    origin: u32,
    length: u32,
}

/// Emit a runtime for a board that an external loader starts.
///
/// `RuntimeBuilder` only produces self-booting XIP images, and a loader has done
/// that work before it hands over. What's left is a plain RAM image linked for
/// the addresses it runs at: `code` holds the vector table and `.text`, `data`
/// holds `.data`, `.bss` and the stack.
///
/// `ENTRY` is stated on both sides of the `INCLUDE` because LLD keeps the first
/// one it sees and GNU ld keeps the last.
fn loader_booted_runtime(
    out_dir: &std::path::Path,
    code: Region,
    data: Region,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(
        out_dir.join("memory.x"),
        format!(
            "MEMORY
{{
  FLASH (rx)  : ORIGIN = {:#010X}, LENGTH = {:#010X}
  RAM   (rwx) : ORIGIN = {:#010X}, LENGTH = {:#010X}
}}
",
            code.origin, code.length, data.origin, data.length
        ),
    )?;
    fs::write(
        out_dir.join("imxrt-link.x"),
        r#"ENTRY(_loader_entry)
EXTERN(_loader_entry)
INCLUDE link.x
ENTRY(_loader_entry)
"#,
    )?;
    Ok(())
}

const DEVICE_X: &str = r#"
PROVIDE(BOARD_CONSOLE = DefaultHandler);
PROVIDE(BOARD_BUTTON = DefaultHandler);
PROVIDE(BOARD_SPI = DefaultHandler);
PROVIDE(BOARD_PWM = DefaultHandler);
PROVIDE(BOARD_DMA_A = DefaultHandler);
PROVIDE(BOARD_DMA_B = DefaultHandler);
PROVIDE(BOARD_PIT = DefaultHandler);
PROVIDE(BOARD_SAI1 = DefaultHandler);
PROVIDE(BOARD_GPT1 = DefaultHandler);
PROVIDE(BOARD_GPT2 = DefaultHandler);
PROVIDE(BOARD_USB1 = DefaultHandler);
PROVIDE(BOARD_SWTASK0 = DefaultHandler);
"#;
