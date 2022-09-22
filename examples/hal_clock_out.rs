//! Evaluate PLL and root clock configurations.
//!
//! This example builds for all platforms. However, due to board layouts, it
//! may not usefully run on all boards. For example, the Teensy 4 does not
//! easily expose the pins, so this example does nothing interesting.
//!
//! Connect over the serial console.

#![no_std]
#![no_main]

use hal::ccm::output_source::{clko1, clko2};
use imxrt_hal as hal;
use imxrt_ral as ral;
use ral::ccm::CCM;

use eh1 as embedded_hal;

use embedded_hal::serial::{blocking::Write as _, nb::Read};

use core::fmt::{self, Write as _};

use board::clock_out::{CLKO1_SELECTIONS, CLKO2_SELECTIONS};

use menu::*;

struct Ctx {
    ccm: CCM,
    writer: Writer,
}

const ROOT_MENU: Menu<Ctx> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: |_, _, _, ctx| {
                    let _ = info(&mut ctx.writer, &mut ctx.ccm);
                },
                parameters: &[],
            },
            command: "info",
            help: Some("Show clock ID => clock output mapping and clock output dividers"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: |_, _, args, ctx| {
                    if args.len() != 2 {
                        return;
                    }
                    let _ = set_divider(args[0], args[1], &mut ctx.writer, &mut ctx.ccm);
                },
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "output_id",
                        help: Some("Clock output ID: 1 or 2"),
                    },
                    Parameter::Mandatory {
                        parameter_name: "divider",
                        help: Some("Divider value between [1-8]"),
                    },
                ],
            },
            command: "divide",
            help: Some("Set clock output dividers"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: |_, _, args, ctx| {
                    if args.len() != 2 {
                        return;
                    }
                    let _ = set_clock(args[0], args[1], &mut ctx.writer, &mut ctx.ccm);
                },
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "output_id",
                        help: Some("Clock output ID: 1 or 2"),
                    },
                    Parameter::Mandatory {
                        parameter_name: "selection",
                        help: Some("Selection ID; use 'info' to see options"),
                    },
                ],
            },
            command: "select",
            help: Some("Set clock output selection"),
        },
    ],
    entry: None,
    exit: None,
};

struct Writer(board::Console);
impl fmt::Write for Writer {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        let mut at_linefeed = false;
        for line in msg.split('\n') {
            if at_linefeed {
                self.0.write(b"\r\n").unwrap();
            }
            let bytes = line.as_bytes();
            if !bytes.is_empty() {
                self.0.write(bytes).unwrap();
            }
            at_linefeed = true;
        }
        Ok(())
    }
}
impl fmt::Write for Ctx {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

fn info(console: &mut Writer, ccm: &mut CCM) -> fmt::Result {
    writeln!(console, "CLKO1 (divider = {})...", clko1::divider(ccm))?;
    for (idx, clock_selection) in CLKO1_SELECTIONS.iter().enumerate() {
        writeln!(console, "\t{} => {:?}", idx, clock_selection)?;
    }
    writeln!(console)?;

    writeln!(console, "CLKO2 (divider = {})...", clko2::divider(ccm))?;
    for (idx, clock_selection) in CLKO2_SELECTIONS.iter().enumerate() {
        writeln!(console, "\t{} => {:?}", idx, clock_selection)?;
    }
    writeln!(console)?;
    Ok(())
}

fn parse_output(output: &str) -> Result<u32, &'static str> {
    match output.parse() {
        Ok(1) => Ok(1),
        Ok(2) => Ok(2),
        Ok(_) => Err("output_id should be 1 or 2"),
        Err(_) => Err("Parse error; was that a number?"),
    }
}

fn set_divider(output: &str, divider: &str, console: &mut Writer, ccm: &mut CCM) -> fmt::Result {
    let output = match parse_output(output) {
        Ok(output) => output,
        Err(msg) => {
            return writeln!(console, "{}", msg);
        }
    };
    let divider: u32 = match divider.parse() {
        Ok(d) if 1 <= d && d <= 8 => d,
        Ok(_) => {
            return writeln!(console, "divider must be between 1 and 8");
        }
        Err(err) => {
            return writeln!(console, "{}", err);
        }
    };
    if 1 == output {
        clko1::set_divider(ccm, divider);
    } else {
        clko2::set_divider(ccm, divider);
    }
    writeln!(console, "Set CLKO{} divider to {}", output, divider)?;
    Ok(())
}

fn set_clock(output: &str, sel: &str, console: &mut Writer, ccm: &mut CCM) -> fmt::Result {
    let output = match parse_output(output) {
        Ok(output) => output,
        Err(msg) => {
            return writeln!(console, "{}", msg);
        }
    };
    let selection_len = if output == 1 {
        CLKO1_SELECTIONS.len()
    } else {
        CLKO2_SELECTIONS.len()
    };
    let selection: usize = match sel.parse() {
        Ok(sel) if sel < selection_len => sel,
        Ok(_) => return writeln!(console, "selection must be less than {}", selection_len),
        Err(err) => return writeln!(console, "{}", err),
    };
    if 1 == output {
        clko1::set_selection(ccm, CLKO1_SELECTIONS[selection]);
        writeln!(
            console,
            "Set CLKO1 selection to {:?}",
            CLKO1_SELECTIONS[selection]
        )?;
    } else {
        clko2::set_selection(ccm, CLKO2_SELECTIONS[selection]);
        writeln!(
            console,
            "Set CLKO2 selection to {:?}",
            CLKO2_SELECTIONS[selection]
        )?;
    }
    Ok(())
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let (_, board::Specifics { led, console, .. }) = board::new();
    let mut ccm = unsafe { imxrt_ral::ccm::CCM::instance() };
    clko1::enable(&mut ccm, true);
    clko2::enable(&mut ccm, true);
    let writer = Writer(console);
    let mut buffer = [0; 64];
    let mut runner = Runner::new(&ROOT_MENU, &mut buffer, Ctx { writer, ccm });
    led.clear();

    loop {
        match embedded_hal::nb::block!(runner.context.writer.0.read()) {
            Err(_) => break,
            Ok(byte) => runner.input_byte(byte),
        }
    }

    loop {
        led.set();
    }
}
