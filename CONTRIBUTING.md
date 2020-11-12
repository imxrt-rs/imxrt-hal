# Contributing

Thanks for helping us build embedded Rust support for NXP's i.MX RT processors! Please open an issue if

- you find a bug in any chip HAL
- you have an idea for a feature
- something isn't clear in our documentation

## Development

The steps below are useful for developers who want to build and modify this repository's crates. All steps assume that you've cloned the repository.

### Dependencies

You'll need

- a Rust installation, at least Rust 1.40, possibly later. To be safe, use the latest, stable Rust compiler.
- the `thumbv7-none-eabihf` Rust target, which may be installed via `rustup`:

```bash
rustup target add thumbv7em-none-eabihf
```
### Chip-specific HAL(s)

We support one HAL crate per i.MX RT processor family. A "processor family" is described by an NXP datasheet and reference manual. For example, the `imxrt1060-hal` supports the [i.MX RT1060 Crossover Processors](https://www.nxp.com/docs/en/nxp/data-sheets/IMXRT1060CEC.pdf), which includes the following processors:

- i.MX RT 1061
- i.MX RT 1062

When developing the HAL(s) a quick way to check everything compiles, in the project root

```
cargo check --target thumbv7em-none-eabihf
```

### Testing

Our CI system ensures that the RAL and HAL(s) build for all processor variants. But, we can't automatically test against hardware! To test your changes on hardware, you'll need to test the RAL and the HAL(s) using another project, like a Rust BSP crate. Some BSP crates that use the `imxrt1060-hal` include

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
