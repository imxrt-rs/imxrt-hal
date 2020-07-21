# imxrt-iomuxc

i.MX RT pad definitions and pin configuration.

## Goals

- Lay the foundation for split i.MX RT hardware abstraction layers (HALs). The approach
  de-couples one of the most chip-specific implementations -- the chip's pads -- from common
  driver code.
- Provide a better interface for pad configuration, as described in
  [this issue](https://github.com/imxrt-rs/imxrt-rs/issues/26).
- Define a common interface for pad configuration that can be used by HAL drivers,
  and also by advanced drivers maintained outside of the HAL.

## Structure

- `imxrt-iomuxc` defines a set of traits that are implemented
  on pads. The traits specify that a pad may be used for a certain function,
  such as a UART transfer pin or an I2C clock pin. The root crate also provides
  common functions to configure pads. `imxrt-iomuxc` is used in crates that need
  to treat pads as resources. Without any feature flags, the `imxrt-iomuxc` provides
  the general pin configuration interface. There are no processor-specific APIs in
  the default build.
- `imxrt-iomuxc` feature flags, like `imxrt106x`, enable processor-specific pad
  definitions and pin implementations.
- `imxrt-iomuxc-build` provides **build-time** support for defining pads. It's
  used to simply generate all of the pads. It also implements simple, common
  functionality across pads, like GPIO pin traits.

## Users

i.MX RT HAL designers, or advanced driver designers, want to treat pads as resources.
They want to create strongly-typed, infallible interfaces that ensure a pad supports
a capability. These users depend on the traits defined by `imxrt-iomuxc` to create
their driver interfaces. These drivers will accept pads across all i.MX RT chip variants.
These designers *do not* enable any `imxrt-iomuxc` feature flags.

When users are ready to run their code on hardware, they enable a feature in the
`imxrt-iomuxc` crate that describes their i.MX RT variant. Users who want to generalize
their code for different i.MX RT variants enable more feature flags.

## Comparison to Variant-Specific IOMUXC Crates

A previous approach separated the `imxrt-iomuxc` interface and implementations
across crates. Rather than having an `imxrt106x` feature in a single `imxrt-iomuxc`
crate, we had separate crates, named like `imxrt106x-iomuxc`, that implemented the
`imxrt-iomuxc` interfaces. We decided to use feature flags after realizing it was
not only easier to maintain, but also equivalent to the multi-crate approach. This
section compares the maintenance and equivalence of the two approaches.

### Easier to maintain

A single crate with feature flags is easier to maintain than an interface crate with
separate implementation crates:

- The interface crate does not need to support "public but internal" APIs. We signaled
  these APIs behind `#[doc(hidden)]` attributes on public types. When using features,
  these APIs are truly private. The approach reduces the documentation burden, since
  we do not need to identify which APIs are truly public, and which APIs are internal.
- It's easier to release and version a single crate than multiple crates. We don't need
  to plan an approach for releasing separate interface and implementation crates. Users
  do not need to be concerned with our versioning and release strategy.
- It's easier to document and study. We may generate documentation for the interface
  and all implementations simply using `cargo doc --all-features`. It's easier to link
  documentation across the implementations and the interface.

### Equivalence

A single crate with feature flags is equivalent to an interface crate and separate
implementation crates. Consider a user who wants to use the IOMUXC pin configuration
interfaces. That user would depend on `imxrt-iomuxc`, regardless of the approach.

Now, consider a user who wants to use their code on an i.MX RT 106x processor variant.
Under the old approach, that user would include the `imxrt106x-iomuxc` crate, which
includes the `imxrt-iomuxc` crate:

```toml
[dependencies]
imxrt106x-iomuxc = "0.1"
# imxrt-iomuxc = "0.1" - implicit dependency
```

When using feature flags, the user enables the `imxrt106x` feature:

```toml
[dependencies]
imxrt-iomuxc = { version = "0.1", features = ["imxrt106x"] }
```

Both approaches result in the same changes to the dependency graph: the graph now includes code
for i.MX RT 106x processor pads.

Since features are additive,  users who want to support more processors enable more feature flags.
This would have translated to the user explicitly including more crates:

```toml
[dependencies]
imxrt-iomuxc = { version = "0.1", features = ["imxrt102x", "imxrt106x"] }

# Equivalent:

[dependencies]
imxrt102x-iomuxc = "0.1"
imxrt106x-iomuxc = "0.1"
# imxrt-iomuxc = "0.1" - implicit dependency
```

Depending on the release strategy, the user would need to maintain the version for all
implementation crates. The feature-flag approach requires a single version, which may
make it easier for the user.

### Discussion

Since the approaches are equivalent, the change has no effect on a split i.MX RT HAL. An
`imxrt-hal[-common]` crate would depend on the `imxrt-iomuxc` crate without feature flags.
Then, a processor-specific HAL would depend on `imxrt-iomuxc` with the appropriate feature
flag. We achive the goals of a split HAL, as users are unconcerned with feature flags.

We realize that this approach perpetuates the need for feature flags. However,
the `imxrt-iomuxc` crate is a much lower-level interface than the HAL crates; it's equivalent
to a RAL crate or PAC. The `imxrt-iomuxc` crate is intended for HAL developers, not HAL users.
HAL developers cannot escape feature flags, since using RALs and PACs already necessitates feature
flags. By adopting a single `imxrt-iomuxc` crate with feature flags, HAL developers continue to use
feature flags to support the i.MX RT variants.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.