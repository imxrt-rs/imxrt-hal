# Changelog

## [Unreleased]

### Changed

- **BREAKING** The HAL's `"rtfm"` feature is changed to `"rtic"`, reflecting the framework's
  new name. Users who are relying on the `"rtfm"` feature should now use the `"rtic"` feature.

## [0.3.0] - 2020-06-18

### Added

- `uart::ReadError` implements `Clone`, `Debug`, `PartialEq`, and `Eq`
- `UART` peripherals may be `split()` into `Tx` and `Rx` halves
- `UART` peripherals implement the blocking `embedded_hal` traits
- General purpose timers (GPT)
- Introducing the DMA module
  - Peripheral-to-memory transfers, supporting both SPI and UART
  - Memory-to-memory copies
  - Statically-allocated linear and circular buffers
- Documentation
  - Getting Started
  - Contributing
  - Rustdoc-checked code examples

### Changed

- `imxrt_hal::pit::Unclocked::clock` now takes a `&mut imxrt_hal::ccm::perclk::Configured` mutable reference, rather
  than a value. Users need to add a `&mut` qualifier to the `clock()` argument, and qualify the `Configured`
  object as `mut` to migrate their code. The object may now be shared between PIT and GPT clocking methods.

Prior releases were not tracked with a changelog entry.

[Unreleased]: https://github.com/imxrt-rs/imxrt-rs/compare/0.3.0...HEAD
[0.3.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.2.1...0.3.0