//! This build script prepares the HAL build.

use imxrt_rt::{Family, Memory, RuntimeBuilder};
use std::{collections::HashSet, env, fs, path::PathBuf};

fn extract_features() -> HashSet<String> {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from)?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::write(out_dir.join("device.x"), DEVICE_X)?;

    let features = extract_features();
    for feature in features {
        match feature.as_str() {
            "teensy4" => {
                RuntimeBuilder::from_flexspi(Family::Imxrt1060, 1984 * 1024)
                    .rodata(Memory::Dtcm)
                    .build()?;
                println!("cargo:rustc-cfg=board=\"teensy4\"");
                println!("cargo:rustc-cfg=family=\"imxrt1060\"");
            }
            "imxrt1010evk" => {
                RuntimeBuilder::from_flexspi(Family::Imxrt1010, 16 * 1024 * 1024)
                    .rodata(Memory::Dtcm)
                    .build()?;
                println!("cargo:rustc-cfg=board=\"imxrt1010evk\"");
                println!("cargo:rustc-cfg=family=\"imxrt1010\"");
            }
            _ => continue,
        }
        return Ok(());
    }
    Err("Board feature not associated to a runtime.".into())
}

const DEVICE_X: &str = r#"
PROVIDE(BOARD_CONSOLE = DefaultHandler);
PROVIDE(BOARD_SPI = DefaultHandler);
PROVIDE(BOARD_PWM = DefaultHandler);
PROVIDE(BOARD_DMA_A = DefaultHandler);
PROVIDE(BOARD_PIT = DefaultHandler);
PROVIDE(BOARD_GPT1 = DefaultHandler);
PROVIDE(BOARD_GPT2 = DefaultHandler);
PROVIDE(BOARD_USB1 = DefaultHandler);
PROVIDE(BOARD_SWTASK0 = DefaultHandler);
"#;
