//! Board definitions for imxrt-hal examples.
//!
//! If you're here to add a new board, see the documentation
//! of [`Board`]. Then, define a `const` for your board. Finally,
//! add your board to the list of `BOARDS`.

use crate::cfg::{family, interrupts};
use std::io::{self, Write};

/// Defines a new board.
pub struct Board {
    /// Your board name.
    ///
    /// Must match the feature flag that you set in Cargo.toml
    pub name: &'static str,
    /// Select your chip family.
    ///
    /// All supported chip families are defined in the `family` module.
    pub family: family::Family,
    /// Specify your board's flash LENGTH.
    ///
    /// This is a property of your board's flash chip.
    flash_length: usize,
    /// Board-specific interrupts.
    ///
    /// When you bring up a board, your exposing various peripherals
    /// that are consistent across all boards. This is where you
    /// associate your peripheral selection with its interrupt. The
    /// configuration library then gives this a generic name for RTIC.
    ///
    /// If you're feeling lazy, use `interrupts::NOT_YET_IMPLEMENTED`
    /// as the value for any / all interrupts. This allows you to build
    /// imxrt-hal examples that don't need RTIC / the interrupt symbol.
    pub interrupts: interrupts::Board,
}

const TEENSY4: Board = Board {
    name: "teensy4",
    family: family::IMXRT1060,
    flash_length: crate::kb(1984),
    interrupts: interrupts::Board {
        console: "LPUART2",
        spi: "LPSPI4",
        pwm: "PWM2_2",
    },
};

const IMXRT1010EVK: Board = Board {
    name: "imxrt1010evk",
    family: family::IMXRT1010,
    flash_length: crate::mb(16),
    interrupts: interrupts::Board {
        console: "LPUART1",
        spi: "LPSPI1",
        pwm: "PWM1_2",
    },
};

/// After you've defined your board, add it here.
pub const BOARDS: &[Board] = &[TEENSY4, IMXRT1010EVK];

impl Board {
    pub fn emit_configuration(&self) {
        println!("cargo:rustc-cfg=board=\"{}\"", self.name);
        println!("cargo:rustc-cfg=family=\"{}\"", self.family.name);
    }
    pub fn generate_memory_map(&self, f: &mut dyn Write) -> io::Result<()> {
        writeln!(f, "/* Board configuration for '{}' */", self.name)?;
        writeln!(f, "MEMORY {{")?;
        writeln!(
            f,
            "FLASH (RX) : ORIGIN = 0x60000000, LENGTH = {:#X}",
            self.flash_length
        )?;
        writeln!(
            f,
            "ITCM (RX) : ORIGIN = 0x00000000, LENGTH = {:#X}",
            self.family.itcm_length
        )?;
        writeln!(
            f,
            "DTCM (RW) : ORIGIN = 0x20000000, LENGTH = {:#X}",
            self.family.dtcm_length
        )?;
        writeln!(
            f,
            "OCRAM (RWX) : ORIGIN = 0x20200000, LENGTH = {:#X}",
            self.family.ocram_length
        )?;
        writeln!(f, "}}")?;
        writeln!(f, "__fcb_offset = {:#X};", self.family.fcb_offset)?;
        Ok(())
    }
}
