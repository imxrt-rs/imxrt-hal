# imxrt-hal

A Rust hardware abstraction layer (HAL) for NXP i.MX RT processors.

[![All Checks][all-checks-badge]][all-checks-url] [![Crates.io][imxrt-hal-badge]][imxrt-hal-url]

[all-checks-badge]: https://github.com/imxrt-rs/imxrt-hal/actions/workflows/rust.yml/badge.svg
[all-checks-url]: https://github.com/imxrt-rs/imxrt-hal/actions/workflows/rust.yml
[imxrt-hal-badge]: https://img.shields.io/crates/v/imxrt-hal
[imxrt-hal-url]: https://crates.io/crates/imxrt-hal

[matrix-chat](https://matrix.to/#/#imxrt-rs:matrix.org)

## Getting started

You'll need a compatible Rust toolchain in order to build and use `imxrt-hal`.
For information on installing a toolchain, see
[the imxrt-rs book](https://imxrt-rs.github.io/book/toolchain.html).

If you're new to `imxrt-hal` and want to try its examples on hardware,
see [this guide](./board/README.md). Examples run on various i.MX RT development
boards, like the 1010 EVK and the Teensy 4.

Many HAL drivers work across all of the i.MX RT chips supported by
[`imxrt-ral`](https://github.com/imxrt-rs/imxrt-ral). Select drivers become
available for a specific chip build. If you want to use `imxrt-hal` in
your project, see the package's API docs. For a high-level overview of how
`imxrt-ral` and `imxrt-hal` work together, check out [the ecosystem
walkthrough](https://imxrt-rs.github.io/book/ecosystem_walkthrough/index.html)

## Contributing

If you're interested in extending or changing the HAL, or if you want to add
testing support for a new board, see [CONTRIBUTING](CONTRIBUTING.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
