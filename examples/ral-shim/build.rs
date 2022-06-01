use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

/// Keep me in sync with the Interrupt items.
///
/// Order doesn't matter.
static BOARD_INTERRUPTS: &[&str] = &[
    "BOARD_PIT",
    "BOARD_GPT1",
    "BOARD_GPT2",
    "BOARD_CONSOLE",
    "BOARD_SPI",
    "BOARD_DMA_A",
];

/// Generates
///
/// - a mapping from Interrupt items to function pointers of the same name.
/// - extern "C" declarations of those function pointers.
fn generate_interrupt_shims(writer: &mut dyn Write) -> io::Result<()> {
    writeln!(writer, "const INT_TO_VEC: &[(ISR, Vector)] = &[")?;
    for intr in BOARD_INTERRUPTS {
        writeln!(writer, "    (interrupt::{intr}, {intr}),")?;
    }
    writeln!(writer, "];\n")?;

    writeln!(writer, "extern \"C\" {{")?;
    for intr in BOARD_INTERRUPTS {
        writeln!(writer, "    fn {intr}();")?;
    }
    writeln!(writer, "}}")?;
    Ok(())
}

/// Generates PROVIDEs for a linker script.
fn generate_linkerscript_provides(writer: &mut dyn Write) -> io::Result<()> {
    for intr in BOARD_INTERRUPTS {
        writeln!(writer, "PROVIDE({intr} = DefaultHandler);")?;
    }
    Ok(())
}

fn extract_feature() -> String {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .next()
        .expect("There's always one feature")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Add new board support here. Associate your board with its chip family.
    let mut board_to_family = HashMap::new();
    board_to_family.insert("imxrt1010evk", "imxrt1010");
    board_to_family.insert("teensy4", "imxrt1060");

    let board = extract_feature();
    let family = board_to_family[board.as_str()];
    println!("cargo:rustc-cfg=board=\"{}\"", board);
    println!("cargo:rustc-cfg=family=\"{}\"", family);

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let mut interrupt_shims = File::create(out_dir.join("interrupt_shims.rs"))?;
    generate_interrupt_shims(&mut interrupt_shims)?;

    println!("cargo:rustc-link-search={}", out_dir.display());
    let mut board_shims = File::create(out_dir.join("interrupt_shims.x"))?;
    generate_linkerscript_provides(&mut board_shims)?;
    Ok(())
}
