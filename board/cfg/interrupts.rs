//! Implements interrupt renaming, and code generation for that renaming.
//!
//! See the 'ral_shim' module for more information.

use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    path::Path,
};

/// Use this as your interrpt name to skip code generation
/// and generate a link error. This can help when you're
/// incrementally adding a board. Just don't build the
/// examples that need the interrupt.
pub const NOT_YET_IMPLEMENTED: &str = "";

/// Specifies the interrupt names for board-specific interrupts.
pub struct Board {
    /// Which interrupt handles the serial console?
    pub console: &'static str,
    /// Which interrupt handles the LPSPI master?
    pub spi: &'static str,
    /// Which interrupt handles the PWM module & submodule events?
    pub pwm: &'static str,
}

/// Mapping of board interrupt names to imxrt-ral interrupt names.
type Map = HashMap<&'static str, &'static str>;

impl Board {
    fn fill_map(&self, map: &mut Map) {
        map.insert("BOARD_CONSOLE", self.console);
        map.insert("BOARD_SPI", self.spi);
        map.insert("BOARD_PWM", self.pwm);
    }
}

/// Specifies the alias of interrupts shared across families.
pub struct Chip {
    /// Which interrupt services DMA channel A?
    pub dma_a: &'static str,
}

impl Chip {
    fn fill_map(&self, map: &mut Map) {
        map.insert("BOARD_DMA_A", self.dma_a);
    }
}

/// Add commonly-named interrupts here.
const COMMON: &[(&str, &str)] = &[
    ("BOARD_PIT", "PIT"),
    ("BOARD_GPT1", "GPT1"),
    ("BOARD_GPT2", "GPT2"),
    ("__BOARD_FAKE_DO_NOT_USE", NOT_YET_IMPLEMENTED),
];

fn fill_map_with_common(map: &mut Map) {
    map.extend(COMMON.iter().copied())
}

/// Generates
///
/// - a mapping from Interrupt items to function pointers of the same name.
/// - extern "C" declarations of those function pointers.
/// - the module of interrupt numbers.
fn generate_interrupt_shims(writer: &mut dyn Write, map: &Map) -> io::Result<()> {
    // In PACs and RAL, 'interrupt' is typically an enum.
    // Bit RTIC doesn't care if 'interrupt' is an enum or a module.
    // It only cares that it's accessible with path-like syntax.
    // Using a module allows us to meet RTIC's requirements without
    // introducing a new type.
    writeln!(writer, "#[doc(hidden)]\npub mod interrupt_shims {{")?;
    writeln!(writer, "    use super::ISR;")?;
    for (board_isr, ral_isr) in map.iter() {
        writeln!(writer, "    pub const {board_isr}: ISR = ISR::{ral_isr};")?;
    }
    writeln!(writer, "}}\n")?;

    writeln!(writer, "const INT_TO_VEC: &[(ISR, Vector)] = &[")?;
    for intr in map.keys() {
        writeln!(writer, "    (interrupt_shims::{intr}, {intr}),")?;
    }
    writeln!(writer, "];\n")?;

    writeln!(writer, "extern \"C\" {{")?;
    for intr in map.keys() {
        writeln!(writer, "    fn {intr}();")?;
    }
    writeln!(writer, "}}")?;
    Ok(())
}

/// Generates PROVIDEs for a linker script.
fn generate_linkerscript_provides(writer: &mut dyn Write, map: &Map) -> io::Result<()> {
    for intr in map.keys() {
        writeln!(writer, "PROVIDE({intr} = DefaultHandler);")?;
    }
    Ok(())
}

pub fn generate(board: &Board, chip: &Chip, out_dir: &Path) -> io::Result<()> {
    let mut map = Map::new();
    fill_map_with_common(&mut map);
    chip.fill_map(&mut map);
    board.fill_map(&mut map);

    // Remove all ISRs that are not yet implemented.
    map = map
        .into_iter()
        .filter(|(_, ral_isr)| !ral_isr.is_empty())
        .collect();

    let mut interrupt_shims = File::create(out_dir.join("interrupt_shims.rs"))?;
    generate_interrupt_shims(&mut interrupt_shims, &map)?;

    let mut board_shims = File::create(out_dir.join("interrupt_shims.x"))?;
    generate_linkerscript_provides(&mut board_shims, &map)?;

    Ok(())
}
