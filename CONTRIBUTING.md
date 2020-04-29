# Contributing

Thanks for helping us build embedded Rust support for NXP's i.MX RT processors! Please open an issue if

- you find a bug in the RAL or HAL crates
- you have an idea for a feature
- something isn't clear in our documentation

## Development

The steps below are useful for developers who want to build and modify the RAL and the HAL. All steps assume that you've cloned the repository.

### Dependencies

You'll need

- a Rust installation, at least Rust 1.40, possibly later. To be safe, use the latest, stable Rust compiler.
- the `thumbv7-none-eabihf` Rust target, which may be installed via `rustup`:

```bash
rustup target add thumbv7em-none-eabihf
```

### RAL

The `imxrt-ral` crate is auto-generated from the checked-in SVD files, available in `imxrt-ral/svd`. Note that the auto-generated RAL source files are not checked into git. They're ignored so that developers are not encouraged to directly modify the Rust source files. If we modified the files directly, the changes might be lost the next time we auto-generate the RAL crate.

To generate the RAL,

- Install Python 3. You'll need at least Python 3.6.
- Install the Python dependencies needed to generate the RAL: `pip3 install --user svdtools`. Alternatively, use the rules in the RAL's `Makefile` to create a virtual environment with the necessary dependencies: `make venv update-venv && source venv/bin/activate`.
- Run `make` in the `imxrt-ral` directory: `cd imxrt-ral; make; cd ..;`. The auto-generation script might generate warnings; that's OK.

If everything went well, you should find that the `imxrt-ral/src` directory is populated with Rust files. The RAL can build by itself: `cd imxrt-ral && cargo check --features imxrt1062 --target thumbv7em-none-eabihf`.

The RAL doesn't change too frequently. But, if you add an SVD patch, you'll need to re-generate the RAL to realize the change. Keep an eye on pull requests that mention a RAL change.

### HAL

Make sure you've generated the RAL (see above). When developing the HAL, specify a feature flag from the command line. To check the HAL for `imxrt1062` processors, `cd imxrt-hal`, then

```
cargo check --features imxrt1062 --target thumbv7em-none-eabihf
```

### SVD Patches

To modify the RAL, you'll need to describe your change as an SVD patch. If you'd like to add patches to the i.MX RT SVD files, learn about [svdtools](https://github.com/stm32-rs/svdtools). Use some of the existing SVD patches as a guide.

### Testing

Our CI system ensures that the RAL and HAL builds for all processor variants. But, we can't automatically test against hardware! To test your changes on hardware, you'll need to test the RAL and the HAL using another project, like a Rust BSP crate. Some BSP crates that use the `imxrt-hal` include

- [the `imxrt1060evk-bsp` crate](https://github.com/imxrt-rs/imxrt1060evk-bsp)
- [the `teensy4-bsp` crate](https://github.com/mciantyre/teensy4-rs)

Follow the instructions in those projects to prepare an environment for testing HAL changes. You may also consider contributing to those projects. The `teensy4-rs` project, in particular, maintains a set of [examples](https://github.com/mciantyre/teensy4-rs/tree/master/teensy4-examples/src) that may help you test changes.

## Resources

- i.MX RT reference manuals are available from NXP. The reference manuals describe the i.MX RT registers and peripheral capabilities. Go [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES), and select your processor. Then, go to "Documentation," and scroll down to "Reference Manual." You'll need a free NXP account to access the reference manuals.
- i.MX RT data sheets are available as free downloads [here](https://www.nxp.com/products/processors-and-microcontrollers/arm-microcontrollers/i-mx-rt-crossover-mcus:IMX-RT-SERIES). The data sheets are useful for understanding high-level capabilities of the i.MX RT processors. Select your processor, then go to "Documentation," then "Data Sheet."
- For other code references, consider studying
  - the [Zephyr Project](https://www.zephyrproject.org/).
  - the ARM CMSIS Packs. Here's the [MIMXRT1062 pack](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA); NXP and ARM also provide CMSIS packs for the other i.MX RT variants.
  - NXP's MCUXpresso SDK, available [here](https://www.nxp.com/design/software/development-software/mcuxpresso-software-and-tools/mcuxpresso-software-development-kit-sdk:MCUXpresso-SDK).

## Release Steps

To create a release of the RAL and HAL crates, see [RELEASE.md](docs/RELEASE.md).