# imxrt-hal

A Rust hardware abstraction layer (HAL) for NXP i.MX RT processors.

[![All Checks][all-checks-badge]][all-checks-url] [![Crates.io][imxrt-hal-badge]][imxrt-hal-url]

[all-checks-badge]: https://github.com/imxrt-rs/imxrt-rs/workflows/All%20Checks/badge.svg
[all-checks-url]: https://github.com/imxrt-rs/imxrt-rs/actions?query=workflow%3A%22All+Checks%22
[imxrt-hal-badge]: https://img.shields.io/crates/v/imxrt-hal
[imxrt-hal-url]: https://crates.io/crates/imxrt-hal

[matrix-chat](https://matrix.to/#/#imxrt-rs:matrix.org)

## Getting started

Make sure that you install the appropriate target with `rustup`. `imxrt-hal`
documentation assumes you're using the hardfloat target, so run

```
rustup target add thumbv7em-none-eabihf
```

if you want to follow along. Otherwise, substitute your preferred target.

If you're new to `imxrt-hal` and want to try its examples on hardware,
see [this guide](./board/README.md). Examples run on various i.MX RT development
boards, like the 1010 EVK and the Teensy 4.

Most HAL peripherals work across all of the i.MX RT chips supported by
[`imxrt-ral`](https://github.com/imxrt-rs/imxrt-ral). If you're ready to use
`imxrt-hal` in your project, see the package's API docs. Some features that are
specific to a chip family may not yet have support. See the API docs to
understand what's available, and help us out if you notice gaps.

## Contributing

If you're interested in extending or changing the HAL, or if you want to add
testing support for a new board, see [CONTRIBUTING](CONTRIBUTING.md).

## Future of the HAL

Follow #56 to understand how we're breaking `imxrt-hal` into separate crates. As
of now, all functionality is exposed through a single crate, `imxrt-hal`, that
supports multiple i.MX RT chip families through direct and indirect features.

## Q/A

#### *Are there any board support packages (BSP) that use the `imxrt-hal` crate?*

There are a few BSPs that use the `imxrt-hal` crate:

- [The `imxrt1060evk-bsp` crate](https://github.com/imxrt-rs/imxrt1060evk-bsp),
  useful for the i.MX RT 1060 Evaluation Kit (EVK).
- [The `teensy4-bsp` crate](https://github.com/mciantyre/teensy4-rs), supporting
  Rust development on the Teensy 4.

Consider using those crates if you already own those hardware platforms, as they
may provide a simpler foundation for building Rust applications.

#### *How can I use Rust to boot an i.MX RT-based system? Does the HAL provide the reset handler?*

Neither the HAL nor the RAL can help you boot an i.MX RT-based
system. Typically, Rust developers use [the `cortex-m-rt`
crate](https://github.com/rust-embedded/cortex-m-rt) as a minimal runtime for
Cortex-M processors. However, i.MX RT processors require more setup than what
the `cortex-m-rt` crate offers. As of now, you're required to use your own
runtime crate to support i.MX RT processor start-up.

We have some components that might be helpful when building your own runtime
crate:

- [The `imxrt-boot-gen` crate](https://github.com/imxrt-rs/imxrt-boot-gen) lets
  you define some of the data structures that are required to boot i.MX RT
  processors. It's used in the other projects listed below.
- [The `imxrt-rt` crate](https://github.com/imxrt-rs/imxrt-rt) provides a
  runtime crate that may be useful for other i.MX RT processors. It's the
  runtime crate used in [the `imxrt1060evk-bsp`
  crate](https://github.com/imxrt-rs/imxrt1060evk-bsp).
- [The `teensy4-rs` project](https://github.com/mciantyre/teensy4-rs) provides
  its own runtime crate that utilizes the tightly-coupled memory (TCM)
  regions. The runtime is specific for the Teensy 4, although it may be used
  elsewhere.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
