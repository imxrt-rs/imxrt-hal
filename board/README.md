`board` provides a thin board support package for `imxrt-hal`. The package
provides cross-board compatibility for all `imxrt-hal` hardware examples. It
supports `imxrt-hal` development and testing, and is not intended as a general
BSP.

`board` supports

- Teensy 4.0 and Teensy 4.1 boards with the `teensy4` feature.
- the IMXRT1010EVK board with the `imxrt1010evk` feature.
- the IMXRT1060EVK board with the `imxrt1060evk` feature.
- the Cortex M7 on the IMXRT1170EVK board with the `imxrt1170evk-cm7` feature.
- the Cortex M33 on the IMXRT1180EVK board with the `imxrt180evk-cm33` feature.

A board may only support a subset of all examples. To understand which examples
are supported by your board, consult the CI testing matrix.

## Board configurations

Each board configures its supported hardware based on its pinout. To understand
your board's configuration, see the relevant module in `src/`. You'll want this
information to understand which pins are configured as LPUART TX and RX pins,
which pin is the LED, etc.

Boards may share configurations, like baud rates and timer resolutions.
Consult a board module to understand if it's sharing these configurations.

`board` plays tricks so that a single RTIC application can compile and run
across all supported boards. This means that the RTIC examples are coupled to
the `board` package, and interrupts like `BOARD_CONSOLE` may look peculiar.
Make sure you're familiar with RTIC before deeply studying these examples.

## Building hardware examples

Hardware examples for `imxrt-hal` depend on `board` and a board selection. This
section describes how to build an example for your board. It focuses on building
examples for the Teensy 4, but the concept generalizes for all supported boards.

To build the `hal_led` example for a Teensy 4, run the command from the repo's
root:

```
cargo build --example=hal_led --features=board/teensy4 --target=thumbv7em-none-eabihf
```

Generally, you select the example with `--example`, and specify the board with
`--features=board/[your-board]`. To build the same example for the
IMXRT1010EVK, change `--features=board/teensy4` to
`--features=board/imxrt1010evk`.

To build _most_ of the hardware examples for the Teensy 4, run

```
cargo build --examples --features=board/teensy4 --target=thumbv7em-none-eabihf
```

As before, change the `board` feature to select a different board.

Some examples require additional board features. To understand those extra board
features, see the top-level `Cargo.toml` and the required features of each
example. For instance, examples with SPI may require an additional feature. To
build the SPI example for the Teensy 4, run

```
cargo build --example=rtic_spi --features=board/teensy4,board/spi --target=thumbv7em-none-eabihf
```

Note that enabling `board/spi` for other examples may not be supported. So, you
should only include extra board features when building specific examples, as
required.

Depending on the target, some debug builds may fail to link due to large data or
text regions. Try building these examples with `--release` to enable size
optimizations.

Artifacts are available under
`target/thumbv7em-none-eabihf/[debug|release]/examples`. Keep this in mind when
flashing your board.

## Flashing hardware examples

The tools required to flash an example depend on the board you're using. This
section recommends tooling to flash hardware examples on your board.

### NXP IMXRT EVKs

If you're using an NXP IMXRT EVK, you can use either the on-board CMSIS debug
access probe (DAP), or an external JTAG / SWD tool. For recommended software
support, see the [flashing and debugging tool recommendations][flash-debug] in
the imxrt-rs book.

[flash-debug]: https://imxrt-rs.github.io/book/

### Teensy 4

If you're using a Teensy 4 board, you'll need all of the following:

- An `objcopy` capable of transforming ELF files into Intel HEX. Consider using
  `rust-objcopy` provided by [`cargo-binutils`]. The rest of this documentation
  assumes you're using `cargo-binutils`.
- Either a build of [`teensy_loader_cli`], or the [Teensy Loader
  Application]. The latter is available with the Teensyduino add-ons.

After building your example, use `rust-objcopy` to convert the program into
HEX. For the `hal_led` example above, that command resembles

```
rust-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/hal_led hal_led.hex
```

Finally, load the HEX file onto your board using your preferred loader.

[`cargo-binutils`]: https://github.com/rust-embedded/cargo-binutils
[`teensy_loader_cli`]: https://github.com/PaulStoffregen/teensy_loader_cli
[Teensy Loader Application]: https://www.pjrc.com/teensy/loader.html

## Tips and tricks

- Use the `--target-dir` option of `cargo build` to select output directories on
  a board-by-board basis. This is useful to automatically track artifacts for
  different boards.
- If you're using `probe-run` or `pyOCD` to flash an EVK, use the tool as a
  runner. See the [Cargo
  Configuration](https://doc.rust-lang.org/cargo/reference/config.html)
  documentation for more information. Please do not check your runner setting
  into the repository; consider using environment variables or hierarchical
  configuration files to configure your runner and any other useful command
  aliases.

## Adding a new board

See the hardware testing section of the [CONTRIBUTING](../CONTRIBUTING.md)
guide.
