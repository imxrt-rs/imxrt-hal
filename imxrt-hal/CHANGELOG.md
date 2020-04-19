# Changelog

## [Unreleased]

### Added

- The iMXRT's general purpose timers (GPT) are now available. The timers do not implement any of the `embedded_hal`
  traits at this time. However, the timers are suitable for generating interrupts on repeated durations. See the
  module-level docs for more information and TODOs.

### Changed

- `imxrt_hal::pit::Unclocked::clock` now takes a `&mut imxrt_hal::ccm::perclk::Configured` mutable reference, rather
  than a value. Users need to add a `&mut` qualifier to the `clock()` argument, and qualify the `Configured`
  object as `mut` to migrate their code. The object may now be shared between PIT and GPT clocking methods.

Prior releases were not tracked with a changelog entry.

[Unreleased]: https://github.com/imxrt-rs/imxrt-rt/compare/v0.2.1...HEAD
