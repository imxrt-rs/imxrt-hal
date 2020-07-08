# imxrt-iomuxc

An experimental set of crates for i.MX RT pad configuration and resource management.

## Goals

- Lay the foundation for split i.MX RT hardware abstraction layers (HALs). The approach
  de-couples one of the most chip-specific implementations -- the chip's pads -- from common
  driver code.
- Provide a better interface for pad configuration, as described in
  [this issue](https://github.com/imxrt-rs/imxrt-rs/issues/26).
- Define a common interface for pad configuration that can be used by HAL drivers,
  and also by advanced drivers maintained outside of the HAL.

## Structure

- `imxrt-iomuxc`, the root crate, defines a set of traits that are implemented
  on pads. The traits specify that a pad may be used for a certain function,
  such as a UART transfer pin or an I2C clock pin. The root crate also provides
  common functions to configure pads. `imxrt-iomuxc` is used in crates that need
  to treat pads as resources.
- `imxrt106x-iomuxc` provides an implementation of the `imxrt-iomuxc` traits. It
  identifies the 106x pads that may be used for UART, SPI, I2C, PWM, etc. It uses
  `imxrt-iomuxc-build` to generate pads at compile time. When it comes time to
  support other i.MX RT processor variants, we add a new crate, and implement the
  pads there.
- `imxrt-iomuxc-build` provides **build-time** support for defining pads. It's
  used by implementation crates to simply generate all of the pads. It implements
  simple, common functionality across implementation crates family.

## Users

i.MX RT HAL designers, or advanced driver designers, want to treat pads as resources.
They want to create strongly-typed, infallible interfaces that ensure a pad supports
a capability. These users depend on the traits defined by `imxrt-iomuxc` to create
their driver interfaces. These drivers will accept pads across all i.MX RT chip variants.

i.MX RT HAL implementers want an API to specify their pads' capabilities. These users
create a device-specific crate, like `imxrt106x-iomuxc`, to implement pads. To simplify
pad implementation, these users use the build-time support crate. As we build support
for more i.MX RT processors, we can know that these pads will plug-and-play with driver
interfaces that accept pads.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.