# Changelog

## [Unreleased]

### Added

- Basic support for the secure real time clock (SRTC) peripheral, which
  continues to track time while the SNVS low-power domain remains powered.
- Add ADC channels. The channels implement the `embedded-hal::adc::Oneshot`
  trait.
- Preliminary support for the TRNG, with an implementation of embedded_hal's
  `rng::Read` trait. Includes support for `rand_core::RngCore` behind the
  optional `"rand_core"` feature.

### Fixed

- Ensure that I2C and SPI errors can only be created by the HAL crate, and not by end users.

## [0.4.1] 2020-09-23

### Fix

- Maintain GPIO input/output configuration when enabling or disabling high-speed mode.

## [0.4.0] 2020-08-29

This release contains numerous breaking HAL changes. See the "Changed" section for more information.
The release includes 0.3.1 fixes.

### Added

- `steal()` the top-level `Peripherals` object. The `steal()` method lets users use the `imxrt_hal`
  as an RTIC device.
- DMA `Memcpy` may support interrupt handling
- A new interface for pad configuration, supported by the `imxrt-iomuxc` crate family. See the Changed
  section for migration information.

### Changed

- **BREAKING** The pad configuration interface has simplified. Users will use the
  interface exposed by the `imxrt-iomuxc` crate family. This section describes how you might
  update your 0.3 (and earlier) code for the new interface.

  - *Naming*: the `IOMUXC` instance exposes pads with a new naming convention. Previously, member
    accessed resembled

    ```rust
    let peripherals = imxrt_hal::Peripherals::take().unwrap();
    peripherals.iomuxc.gpio_ad_b1_02;
    ```

    Now, IOMUXC member access resembles

    ```rust
    peripherals.iomuxc.ad_b1.p02
    ```

    Generally, remove the "`gpio_`" prefix, and replace the second underscore with member access and
    a "`p`" symbol.

  - *Pad Types*: The interface simplifies the pad types, removing the alternate type state.
    Usages resembling

    ```rust
    use imxrt_hal as hal;

    type MyPin = hal::iomuxc::gpio::GPIO_AD_B0_12<hal::iomuxc::Alt5>;
    ```

    can be simply expressed as

    ```rust
    type MyPin = hal::iomuxc::ad_b0::AD_B0_12;
    ```

    Note the stuttering convention of `pad_group::pad_group_offset` to reference pad types.

  - *No alternate transition*: there are no `altX()` methods. Usage resembling

    ```rust
    let mut uart = uarts.uart2.init(
        peripherals.iomuxc.gpio_ad_b1_02.alt2(),
        peripherals.iomuxc.gpio_ad_b1_03.alt2(),
        BAUD,
    ).unwrap()
    ```

    should drop the `altX()` calls (after renaming the pads).

    ```rust
    let mut uart = uarts.uart2.init(
        peripherals.iomuxc.ad_b1.p02,
        peripherals.iomuxc.ad_b1.p03,
        BAUD,
    ).unwrap();
    ```

  - *Type Tags*: all custom type-level constants, like `imxrt_hal::uart::module::_1` are now
    `typenum` constants. There are no peripheral-specific constants. Usage resembling

    ```rust
    use imxrt_hal::uart;

    type MyTX = uart::Tx<uart::module::_3>;
    ```

    should update to

    ```rust
    use imxrt_hal::uart;
    use imxrt_hal::iomuxc;

    type MyTX = uart::Tx<iomuxc::consts::U3>;
    ```

  - *GPIO*: the new IOMUXC driver results in a simpler GPIO interface. There is now a single GPIO
    type that wraps an IOMUXC pad. Any GPIO type, like

    ```rust
    use imxrt_hal as hal;

    type HardwareFlag = hal::gpio::GPIO1IO26<hal::gpio::GPIO1, hal::gpio::Output>;
    ```

    should change to

    ```rust
    type HardwareFlag = hal::gpio::GPIO<hal::iomuxc::ad_b1::AD_B1_10, hal::gpio::Output>;
    ```

    The new GPIO types expose a no-return `toggle()` method, which shadows an embedded_hal
    trait method. If you notice compilation issues surrounding `toggle()`, try removing
    `unwrap()` calls:

    ```rust
    led.toggle().unwrap() // Old
    led.toggle()          // New
    ```
    Or, qualify that the `ToggleableOutputPin` trait method should be called.
  
- **BREAKING** The HAL's `"rtfm"` feature is changed to `"rtic"`, reflecting the framework's
  new name. Users who are relying on the `"rtfm"` feature should now use the `"rtic"` feature.
- **BREAKING** The `dma::{Config, ConfigBuilder}` types are gone. This affects the `dma::Peripheral`
  interface. To configure interrupt on completion / half settings, use `dma::Channel::set_interrupt_on_completion()`
  / `dma::Channel::set_interrupt_on_half()` to perform the same configurations before suppling the
  channel to `dma::Peripheral`.

## [0.3.1] 2020-08-29

### Fixed

- The `StatefulOutputPin` implementation was reading from the wrong GPIO register. The interface
  would indicate that the GPIO was always low, even when it was driven high. The implementation
  now reads from the data register, which represents the driven GPIO value.

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

[Unreleased]: https://github.com/imxrt-rs/imxrt-rs/compare/0.4.1...HEAD
[0.4.1]: https://github.com/imxrt-rs/imxrt-rs/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.3.0...0.4.0
[0.3.1]: https://github.com/imxrt-rs/imxrt-rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.2.1...0.3.0
