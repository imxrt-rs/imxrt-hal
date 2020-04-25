# Changelog

## [Unreleased]

### Added

- The iMXRT's general purpose timers (GPT) are now available.
- DMA channels and DMA-capable peripherals are available. DMA-capable peripherals include SPI and UART.

### Changed

- `imxrt_hal::pit::Unclocked::clock` now takes a `&mut imxrt_hal::ccm::perclk::Configured` mutable reference, rather
  than a value. Users need to add a `&mut` qualifier to the `clock()` argument, and qualify the `Configured`
  object as `mut` to migrate their code. The object may now be shared between PIT and GPT clocking methods.

Prior releases were not tracked with a changelog entry.

[Unreleased]: https://github.com/imxrt-rs/imxrt-rt/compare/v0.2.1...HEAD
