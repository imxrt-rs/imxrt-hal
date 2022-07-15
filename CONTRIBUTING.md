# Contributing

Thanks for helping us build embedded Rust support for NXP's i.MX RT processors!
Please open an issue if

- you find a bug
- you have an idea for a feature
- something isn't clear in our documentation

## Design

Here's a brief design overview of `imxrt-hal` (HAL). For more details, see
comments in the source files. This section might help if you're adding new
driver or chip family support.

The HAL tries to provide a consistent API for drivers across all i.MX RT
chips. The best way to do that is to write a common driver. These drivers are
available under `common` and exposed directly to the end user.

When contributing new peripherals, try your best to fit them exclusively in
`common`. The criteria is that they build and behave consistently across _all_
chips supported by `imxrt-ral` (RAL). There's restrictions: things under
`common` will not require a HAL family feature; they only require a RAL chip
feature. If this isn't possible, split your modules across `common` and `chip`,
and tie them together in the crate root.

Modules under `chip` require both a RAL and a HAL feature. These modules
implement chip family features, which could be a single function or an entire
driver. Modules under `chip` are allowed to use conditional compilation. Modules
under `chip` are also allowed to reference the special chip configuration
modules.

Modules under `chip` that start with `imxrt` are chip configuration
modules. These modules configure the rest of the chip modules. They use path
attributes and aliases to export shared behaviors. There's a few reasons for
this madness (which we're looking to assess with this design):

- The approach maximizes internal code sharing while minimizing the number of
  `#[cfg(...)]` you, a human, need to parse. Ideally, there's only one
  `#[cfg(...)]` to include a given chip configuration module. Then, separate
  modules -- possibly shared across different families -- are linked into this
  configuration module.
- The approach consolidates the minimum set of behaviors needed for any chip
  family. To bring up a new chip family, implement its configuration
  module. There's no hard spec of what goes here, so what's expected of that
  configuration module is demonstrated by the existing configuration modules.

This may end up as only an aesthetic preference. Other HALs have shown that using
various `#[cfg(...)]`s throughout their modules works. We could go that route
when we consider split HALs.

The approach represents a future of split HALs. This structure makes it easy to
understand what would populate `imxrt-hal-common`, and what features would be
specific to an individual HAL package. The current design expresses this in a
single package with conditionally-compiled modules only for prototyping
convenience.

This HAL makes the interesting decision to depend on a dependency's
feature. Since the RAL is part of the HAL's public API, there didn't seem to be
any value in adding a HAL chip specific feature that simply enabled a RAL chip
specific feature. The end user can simply pick their chip through the RAL,
making the choice explicitly in their build.

This approach has some repercussions; namely, when you build the HAL for the
1060 family, you don't know if you're building for a 1061 and 1062 chip. There's
no need to handle this with today's drivers, so we're punting this
problem. Split HALs with their own optional features could solve this. In lieu
of split HALs, we could build these drivers as their own crates, and the user
could depend on them as needed.

## Development

The steps below are useful for developers who want to build and modify
`imxrt-hal`. All steps assume that you've cloned the repository.

You should be familiar with the API documentation and understand how an end user
integrates `imxrt-hal` into their project. You should also install the
dependencies listed in the top-level README, and you should know how to build
examples for a board.

### Building

To build a HAL that works for any i.MX RT chip, enable any `imxrt-ral`
feature. For instance,

```
cargo build --features=imxrt-ral/imxrt1062
```

builds the HAL for an i.MX RT 1062 chip. Replacing `imxrt1062` with `imxrt1011`
is expected to work, too.

Notice how `--target=thumbv7em-none-eabihf` is not required. The HAL should
build for your target system so that unit and integration tests can execute. Of
course, adding `--target=thumbv7em-none-eabihf` will work as well.

To build a HAL with chip family features, enable that feature along with the RAL
feature:

```
cargo build --features=imxrt-ral/imxrt1062,imxrt1060
```

Given the way Cargo resolves features within a workspace, the above command is
equivalent to building a board for that chip:

```
cargo build --features=board/teensy4
```

This command builds `imxrt-hal` using the features provided by the `board`
package. But, it doesn't build the `board` package. This is either nifty or
confusing, depending on your point of view.

`imxrt-hal` is the root of a non-virtual Cargo workspace. To build all packages
in the workspace, append `--workspace` and specify a `board` feature:

```
cargo build --features=board/teensy4 --workspace
```

Again, all packages should build for your host to support testing.

When you build examples, you must supply `--target`, because the examples are
all expected to run on hardware.

```
cargo build --features=board/teensy4 --examples --target=thumbv7em-none-eabihf
```

### Running automated tests

To run tests, you need to exclude examples from the build. One way to do that is
to explicitly select `--tests` and `--doc` so that the build only builds and
runs the necessary artifacts.

```
cargo test --features=board/teensy4 --workspace --tests
cargo test --features=board/teensy4 --workspace --doc
```

### Chip-family HAL features

We support one HAL family per i.MX RT processor family. A "processor family" is
described by an NXP datasheet and reference manual. For example, the `imxrt1060`
feature the [i.MX RT1060 Crossover
Processors](https://www.nxp.com/docs/en/nxp/data-sheets/IMXRT1060CEC.pdf), which
includes the following processors:

- i.MX RT 1061
- i.MX RT 1062

## Running hardware tests

Our CI system ensures that the HAL build for all processor variants. But, we
can't automatically test against hardware! To test your changes on hardware, use
the examples maintained in the repo. See [the documentation](board/README.md) to
get started.

### Adding a new board

Adding a new board lets you easily develop and test i.MX RT hardware
peripherals, and makes it easier for others to contribute. If you run into
issues, reach out to the imxrt-rs team.

If the HAL doesn't yet support your chip family, you'll first need to add
support for the family. See the design section of this document for
guidance. Essentially, you'll define a new chip configuration module under
`chip`. Use the existing configuration modules as your guide.

Once the HAL has a feature for a chip family, you're ready to add a
board. Here's the `board` files of interest:

- `board/cfg/board.rs` describes your board's chip and other board-specific
  configurations. See the inline documentation for more guidance. Add a
  definition for your new board here.
- `board/Cargo.toml` will need a new feature to describe your board. Use the
  existing features as an example.
- `board/src/[your_board_name].rs` is a module that you'll add to specify the hardware
  configuration. Use the existing board modules as an example. Integrate this
  module into `board/src/lib.rs`.

You shouldn't need to support all `imxrt-hal` hardware examples right away; the
design does its best to allow incremental board support. Start with the
`hal_led` example, adding all the code necessary to turn on the LED. Then, pick
another example, and add more code to your board module. Keep going until your
board supports all hardware examples.

The board uses a build-time configuration library to automatically add startup
and runtime support. However, you're still required to supply the firmware
configuration block (FCB) in your board module. If you're having trouble
defining your FCB, reach out to the imxrt-rs team.

## Tips and tricks

If you're using `rust-analyzer` with VSCode, you only need to supply a board
feature. Add this to your `.vscode/settings.json` at the repo root:

```json
{
    "rust-analyzer.cargo.features": [
        "board/teensy4",
    ],
}

```

Change the board feature when you're ready to work on a different chip
(family). If there's no board for your chip (family), replace that feature with
a RAL feature.

## Resources

- i.MX RT reference manuals are available from NXP. The reference manuals
  describe the i.MX RT registers and peripheral capabilities. Go
  [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES),
  and select your processor. Then, go to "Documentation," and scroll down to
  "Reference Manual." You'll need a free NXP account to access the reference
  manuals.
- i.MX RT data sheets are available as free downloads
  [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES). The
  data sheets are useful for understanding high-level capabilities of the i.MX
  RT processors. Select your processor, then go to "Documentation," then "Data
  Sheet."
- For other code references, consider studying
  - the [Zephyr Project](https://www.zephyrproject.org/).
  - the ARM CMSIS Packs. Here's the [MIMXRT1062
    pack](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA);
    NXP and ARM also provide CMSIS packs for the other i.MX RT variants.
  - NXP's MCUXpresso SDK, available
    [here](https://www.nxp.com/design/software/development-software/mcuxpresso-software-and-tools/mcuxpresso-software-development-kit-sdk:MCUXpresso-SDK).

## Release Steps

To create a release of the RAL and HAL crates, see [RELEASE.md](docs/RELEASE.md).
