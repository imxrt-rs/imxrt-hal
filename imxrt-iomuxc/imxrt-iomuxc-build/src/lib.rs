//! Build script support for the `imxrt-iomuxc` crate family
//!
//! # Audience
//!
//! This crate is intended for i.MX RT IOMUXC crate developers, like developers of the `imxrt106x-iomuxc` crate.
//! The API may change to support the `imxrt-iomuxc` crate family. End users should not use this crate directly.
//!
//! # Generate type aliases
//!
//! Create type aliases for processor pads. Use a combination of [`PadRange`](struct.PadRange.html)
//! and [`write_pads()`](fn.write_pads.html) in a build script to generate your module. Then, `include!` it
//! in your crate.
//!
//! For example, a `build.rs` build script resembling
//!
//! ```no_run
//! // ~~ build.rs ~~
//! use imxrt_iomuxc_build as build;
//! use std::{env, fs, io, path::PathBuf};
//!
//! fn main() -> io::Result<()> {
//!     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//!     let mut pads_rs = fs::File::create(out_dir.join("pads.rs"))?;
//!
//!     let emc = build::PadRange::new("EMC", 0..42);
//!     let ad_b0 = build::PadRange::new("AD_B0", 0..16);
//!     let ad_b1 = build::PadRange::new("AD_B1", 0..16);
//!     let b0 = build::PadRange::new("B0", 0..16);
//!     let b1 = build::PadRange::new("B1", 0..16);
//!     let sd_b0 = build::PadRange::new("SD_B0", 0..6);
//!     let sd_b1 = build::PadRange::new("SD_B1", 0..12);
//!
//!     build::write_pads(
//!         &mut pads_rs,
//!         vec![&emc, &ad_b0, &ad_b1, &b0, &b1, &sd_b0, &sd_b1],
//!     )?;
//!     Ok(())
//! }
//! ```
//!
//! would generate type aliases that could be included in your `lib.rs`
//!
//! ```ignore
//! // ~~ lib.rs ~~
//! include!(concat!(env!("OUT_DIR"), "/pads.rs"));
//! ```
//!
//! See the [`write_pads()`](fn.write_pads.html) function for details on the generated module.
//!
//! # Implement GPIO `Pin` traits
//!
//! For the type aliases created in [crate type aliases](#generate-type-aliases), you can implement
//! GPIO `Pin` traits by using [`ImplGpioPin`](struct.ImplGpioPin.html) and [`write_impl_gpio_pins()`](fn.write_impl_gpio_pins.html).
//!
//! ```no_run
//! # use imxrt_iomuxc_build as build;
//! # let ad_b0 = build::PadRange::new("AD_B0", 0..16);
//! # let ad_b1 = build::PadRange::new("AD_B1", 0..16);
//! # let b0 = build::PadRange::new("B0", 0..16);
//! # let b1 = build::PadRange::new("B1", 0..16);
//! # let mut pads_rs: Vec<u8> = Vec::new();
//! build::write_impl_gpio_pins(
//!     &mut pads_rs,
//!     vec![
//!         // GPIO1
//!         build::ImplGpioPin::from_range(&ad_b0, build::GpioRange::no_offset(1, 5)),
//!         build::ImplGpioPin::from_range(&ad_b1, build::GpioRange {
//!             module: 1,
//!             offset: 16,
//!             alt: 5,
//!         }),
//!         // GPIO2
//!         build::ImplGpioPin::from_range(&b0, build::GpioRange::no_offset(2, 5)),
//!         build::ImplGpioPin::from_range(&b1, build::GpioRange {
//!             module: 2,
//!             offset: 16,
//!             alt: 5,
//!         }),
//!     ],
//! ).unwrap();
//! ```

use std::io::{self, Write};
use std::ops::Range;

/// Defines a range of i.MX RT pads
///
/// ```
/// # use imxrt_iomuxc_build::PadRange;
/// let pad_range = PadRange::new("EMC", 0..42);
/// ```
///
/// In the example above, we assume
///
/// - the crate defines a struct, `EMC`, available in the crate's
///   `bases` module. It may be referenced within the crate using
///   `use crate::bases::EMC`.
/// - the processor has 42 pads with the "EMC" prefix, numbered 0 through
///   41
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PadRange {
    base: String,
    range: Range<usize>,
}

impl PadRange {
    /// Create a new pad range that defines type aliases for the pads
    /// base with `base`, and numbered over the provided range
    pub fn new(base: &str, range: Range<usize>) -> Self {
        PadRange {
            base: String::from(base),
            range,
        }
    }

    /// Returns a new `PadRange` that will skip the first `skipped` elements
    pub fn skip(&self, skipped: usize) -> Self {
        Self {
            base: self.base.clone(),
            range: (self.range.start + skipped..self.range.end),
        }
    }

    /// Returns a new `PadRange` that will only take the first `taken` elements
    pub fn take(&self, taken: usize) -> Self {
        Self {
            base: self.base.clone(),
            range: (self.range.start..self.range.start + taken),
        }
    }
}

/// Write type for all pad `ranges` to the provided writer, `out`
///
/// On success, `out` will contain a Rust module, `pads`, that has
/// public submodules. The submodules are named after the `PadRange` tags
/// (lower-case). The submodules will contain public type aliases that match
/// the processor pads.
pub fn write_pads<'a, W, I>(out: &mut W, ranges: I) -> io::Result<()>
where
    W: Write,
    I: IntoIterator<Item = &'a PadRange> + Clone,
{
    let modules = ranges.clone().into_iter().map(|range| {
        let types = std::iter::repeat(range.base.clone())
            .zip(range.range.clone())
            .map(|(base, n)| {
                let name = quote::format_ident!("{}_{:02}", base, n);
                let unsigned = quote::format_ident!("U{}", n);
                let base = quote::format_ident!("{}", base);
                quote::quote! {
                    pub type #name = Pad<#base, #unsigned>;
                }
            });
        let pad_members = std::iter::repeat(range.base.clone())
            .zip(range.range.clone())
            .map(|(base, n)| {
                let name = quote::format_ident!("{}_{:02}", base, n);
                let member = quote::format_ident!("p{:02}", n);
                quote::quote! {
                    pub #member: #name
                }
            });
        let pad_init = std::iter::repeat(range.base.clone())
            .zip(range.range.clone())
            .map(|(base, n)| {
                let name = quote::format_ident!("{}_{:02}", base, n);
                let member = quote::format_ident!("p{:02}", n);
                quote::quote! {
                    #member: <#name>::new()
                }
            });

        let base = range.base.to_lowercase();
        let name = quote::format_ident!("{}", base);
        let doc = format!("Pads with the prefix '{}'", range.base);
        quote::quote! {
            #[doc = #doc]
            pub mod #name {
                use crate::{Pad, bases::*};
                use imxrt_iomuxc::consts::*;
                #(#types)*

                #[doc = #doc]
                pub struct Pads {
                    #(#pad_members),*
                }

                impl Pads {
                    /// Take all pads from this group
                    ///
                    /// # Safety
                    ///
                    /// You may safely call this once to acquire all of the pads. Subsequent calls
                    /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                    /// at the start of your program.
                    ///
                    /// Know that the top-level [`Pads::new()`](../struct.Pads.html#method.new) will call this `new()`.
                    pub const unsafe fn new() -> Pads {
                        Pads {
                            #(#pad_init),*
                        }
                    }
                }
            }
        }
    });
    let module_names: Vec<_> = ranges
        .into_iter()
        .map(|range| quote::format_ident!("{}", range.base.to_lowercase()))
        .collect();
    let module_pad_members = module_names.clone().into_iter().map(|name| {
        quote::quote! {
            pub #name: #name::Pads
        }
    });
    let module_pads_init = module_names.into_iter().map(|name| {
        quote::quote! {
            #name: <#name::Pads>::new()
        }
    });
    let module = quote::quote! {
        /// Contains all of the pads
        ///
        /// This module is auto-generated by the `imxrt-iomuxc-build` crate. See
        /// that crate for more information.
        mod pads {
            #![allow(non_camel_case_types)] // Conform with reference manual
            #(#modules)*

            /// All of the pads
            ///
            /// # Convention
            ///
            /// The members of `Pads` are additional structs that provide pads as
            /// objects. The `p` prefix of each pad denotes "pad."
            pub struct Pads {
                #(#module_pad_members),*
            }

            impl Pads {
                /// Take all of the pads
                ///
                /// # Safety
                ///
                /// You may safely call this once to acquire all of the pads. Subsequent calls
                /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                /// at the start of your program.
                pub const unsafe fn new() -> Pads {
                    Pads {
                        #(#module_pads_init),*
                    }
                }
            }
        }
    };

    write!(out, "{}", module)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GpioPinDetail {
    /// Super module of the pad name: `ad_b0`
    pad_module: String,
    /// The pad name: `AD_B0_00`
    name: String,
    /// The pad alt
    alt: u32,
    /// The GPIO module
    gpio_module: u32,
    /// The GPIO offset
    gpio_offset: u32,
}

/// A type that describes how to `impl gpio::Pin` for a series of pads
///
/// The following usage
///
/// ```
/// use imxrt_iomuxc_build as build;
/// use build::{ImplGpioPin, GpioRange, PadRange};
///
/// let ad_b0 = PadRange::new("AD_B0", 0..4);
/// // States that the four AD_B0 pads are driven by GPIO3_IO8 through GPIO03_IO12
/// ImplGpioPin::from_range(&ad_b0, GpioRange {
///     module: 3,
///     offset: 8,
///     alt: 5,
/// });
/// ```
///
/// will, when paired with [`write_impl_gpio_pins()`](fn.write_impl_gpio_pins.html),
/// generate Rust code that resembles
///
/// ```ignore
/// impl ::imxrt_iomuxc::gpio::Pin for ad_b0::AD_B0_00 {
///     const ALT: u32 = 5;
///     type Module = U3;
///     type Offset = U8;
/// }
///
/// impl ::imxrt_iomuxc::gpio::Pin for ad_b0::AD_B0_01 {
///     const ALT: u32 = 5;
///     type Module = U3;
///     type Offset = U9;
/// }
///
/// impl ::imxrt_iomuxc::gpio::Pin for ad_b0::AD_B0_02 {
///     const ALT: u32 = 5;
///     type Module = U3;
///     type Offset = U11;
/// }
///
/// impl ::imxrt_iomuxc::gpio::Pin for ad_b0::AD_B0_03 {
///     const ALT: u32 = 5;
///     type Module = U3;
///     type Offset = U11;
/// }
/// ```
#[derive(Debug)]
pub struct ImplGpioPin(Vec<GpioPinDetail>);

/// Defines a GPIO range
///
/// `GpioRange` defines the starting values for a GPIO pin sequence. See
/// [`ImplGpioPin`](struct.ImplGpioPin.html) for usages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioRange {
    /// The GPIO module
    ///
    /// `GPIO2` would be described by `2`.
    pub module: u32,
    /// The GPIO offset
    ///
    /// `GPIO3_IO24` would be described by `24`
    pub offset: u32,
    /// The GPIO alternave
    ///
    /// The `alt` value should apply for *every* GPIO described by the range
    pub alt: u32,
}

impl GpioRange {
    /// Convenience for creating a `GpioRange` with no offset
    pub fn no_offset(module: u32, alt: u32) -> Self {
        GpioRange {
            module,
            alt,
            offset: 0,
        }
    }
}

impl ImplGpioPin {
    /// Define a GPIO implementation from a range that describes the GPIO pattern
    pub fn from_range(range: &PadRange, gpio: GpioRange) -> Self {
        let pad_module = range.base.to_lowercase();
        let names = range
            .range
            .clone()
            .map(|id| format!("{}_{:02}", range.base, id));
        ImplGpioPin(
            names
                .enumerate()
                .map(|(idx, name)| GpioPinDetail {
                    pad_module: pad_module.clone(),
                    name,
                    alt: gpio.alt,
                    gpio_module: gpio.module,
                    gpio_offset: idx as u32 + gpio.offset,
                })
                .collect(),
        )
    }
}

/// Write the `impl gpio::Pin for Pad` implementations for all provided `ImplGpioPin` descriptions
///
/// # Requirements
///
/// Assumes that the pads exist and are reachable through their modules, using `use crate::pads::*`.
/// See [`write_pads()`](fn.write_pads.html) for more information.
pub fn write_impl_gpio_pins<W, I>(out: &mut W, impl_gpio_pins: I) -> io::Result<()>
where
    W: Write,
    I: IntoIterator<Item = ImplGpioPin>,
{
    let impls = impl_gpio_pins.into_iter().flat_map(|impl_gpio| {
        impl_gpio.0.into_iter().map(|detail| {
            let pad_module = quote::format_ident!("{}", detail.pad_module);
            let gpio_module = quote::format_ident!("U{}", detail.gpio_module);
            let name = quote::format_ident!("{}", detail.name);
            let gpio_offset = quote::format_ident!("U{}", detail.gpio_offset);
            let alt = detail.alt;

            let doc_alt = format!("ALT{}", alt);
            let doc_module = format!("GPIO{}", detail.gpio_module);
            let doc_offset = format!("IO{:02}", detail.gpio_offset);
            let doc = format!("{}_{} - {}", doc_module, doc_offset, doc_alt);
            quote::quote! {
                #[doc = #doc]
                impl ::imxrt_iomuxc::gpio::Pin for #pad_module::#name {
                    #[doc = #doc_alt]
                    const ALT: u32 = #alt;
                    #[doc = #doc_module]
                    type Module = #gpio_module;
                    #[doc = #doc_offset]
                    type Offset = #gpio_offset;
                }
            }
        })
    });

    let module = quote::quote! {
        mod impl_gpio_pins {
            use ::imxrt_iomuxc::consts::*;
            use crate::pads::*;
            #(#impls)*
        }
    };
    write!(out, "{}", module)
}

#[cfg(test)]
mod tests {
    use super::{write_impl_gpio_pins, write_pads, GpioRange, ImplGpioPin, PadRange};

    #[test]
    fn test_pad_range_skip() {
        assert_eq!(
            PadRange::new("FOO", 0..5).skip(2),
            PadRange::new("FOO", 2..5)
        );
    }

    #[test]
    fn test_pad_range_take() {
        assert_eq!(
            PadRange::new("FOO", 0..5).take(3),
            PadRange::new("FOO", 0..3)
        );
    }

    #[test]
    fn test_write_pads() {
        let expected_tokens = quote::quote! {
            /// Contains all of the pads
            ///
            /// This module is auto-generated by the `imxrt-iomuxc-build` crate. See
            /// that crate for more information.
            mod pads {
                #![allow(non_camel_case_types)] // Conform with reference manual

                #[doc = "Pads with the prefix 'FOO'"]
                pub mod foo {
                    use crate::{Pad, bases::*};
                    use imxrt_iomuxc::consts::*;

                    pub type FOO_02 = Pad<FOO, U2>;
                    pub type FOO_03 = Pad<FOO, U3>;

                    #[doc = "Pads with the prefix 'FOO'"]
                    pub struct Pads {
                        pub p02: FOO_02,
                        pub p03: FOO_03
                    }

                    impl Pads {
                        /// Take all pads from this group
                        ///
                        /// # Safety
                        ///
                        /// You may safely call this once to acquire all of the pads. Subsequent calls
                        /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                        /// at the start of your program.
                        ///
                        /// Know that the top-level [`Pads::new()`](../struct.Pads.html#method.new) will call this `new()`.
                        pub const unsafe fn new() -> Pads {
                            Pads {
                                p02: <FOO_02>::new(),
                                p03: <FOO_03>::new()
                            }
                        }
                    }
                }

                #[doc = "Pads with the prefix 'BAR'"]
                pub mod bar {
                    use crate::{Pad, bases::*};
                    use imxrt_iomuxc::consts::*;

                    pub type BAR_37 = Pad<BAR, U37>;
                    pub type BAR_38 = Pad<BAR, U38>;

                    #[doc = "Pads with the prefix 'BAR'"]
                    pub struct Pads {
                        pub p37: BAR_37,
                        pub p38: BAR_38
                    }

                    impl Pads {
                        /// Take all pads from this group
                        ///
                        /// # Safety
                        ///
                        /// You may safely call this once to acquire all of the pads. Subsequent calls
                        /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                        /// at the start of your program.
                        ///
                        /// Know that the top-level [`Pads::new()`](../struct.Pads.html#method.new) will call this `new()`.
                        pub const unsafe fn new() -> Pads {
                            Pads {
                                p37: <BAR_37>::new(),
                                p38: <BAR_38>::new()
                            }
                        }
                    }
                }

                /// All of the pads
                ///
                /// # Convention
                ///
                /// The members of `Pads` are additional structs that provide pads as
                /// objects. The `p` prefix of each pad denotes "pad."
                pub struct Pads {
                    pub foo: foo::Pads,
                    pub bar: bar::Pads
                }

                impl Pads {
                    /// Take all of the pads
                    ///
                    /// # Safety
                    ///
                    /// You may safely call this once to acquire all of the pads. Subsequent calls
                    /// may return pads that are mutably aliased elsewhere. Consider calling `new()`
                    /// at the start of your program.
                    pub const unsafe fn new() -> Pads {
                        Pads {
                            foo: <foo::Pads>::new(),
                            bar: <bar::Pads>::new()
                        }
                    }
                }
            }
        };
        let expected = expected_tokens.to_string();
        let mut actual = Vec::new();
        write_pads(
            &mut actual,
            vec![&PadRange::new("FOO", 2..4), &PadRange::new("BAR", 37..39)],
        )
        .unwrap();
        assert_eq!(std::str::from_utf8(&actual).unwrap(), expected);
    }

    #[test]
    fn test_write_impl_gpio_pins() {
        let expected_tokens = quote::quote! {
            mod impl_gpio_pins {
                use ::imxrt_iomuxc::consts::*;
                use crate::pads::*;

                #[doc = "GPIO8_IO23 - ALT4"]
                impl ::imxrt_iomuxc::gpio::Pin for foo::FOO_04 {
                    #[doc = "ALT4"]
                    const ALT: u32 = 4u32;
                    #[doc = "GPIO8"]
                    type Module = U8;
                    #[doc = "IO23"]
                    type Offset = U23;
                }

                #[doc = "GPIO8_IO24 - ALT4"]
                impl ::imxrt_iomuxc::gpio::Pin for foo::FOO_05 {
                    #[doc = "ALT4"]
                    const ALT: u32 = 4u32;
                    #[doc = "GPIO8"]
                    type Module = U8;
                    #[doc = "IO24"]
                    type Offset = U24;
                }

                #[doc = "GPIO3_IO00 - ALT9"]
                impl ::imxrt_iomuxc::gpio::Pin for bar::BAR_00 {
                    #[doc = "ALT9"]
                    const ALT: u32 = 9u32;
                    #[doc = "GPIO3"]
                    type Module = U3;
                    #[doc = "IO00"]
                    type Offset = U0;
                }

                #[doc = "GPIO3_IO01 - ALT9"]
                impl ::imxrt_iomuxc::gpio::Pin for bar::BAR_01 {
                    #[doc = "ALT9"]
                    const ALT: u32 = 9u32;
                    #[doc = "GPIO3"]
                    type Module = U3;
                    #[doc = "IO01"]
                    type Offset = U1;
                }
            }
        };
        let expected = expected_tokens.to_string();
        let mut actual = Vec::new();
        write_impl_gpio_pins(
            &mut actual,
            vec![
                ImplGpioPin::from_range(
                    &PadRange::new("FOO", 4..6),
                    GpioRange {
                        module: 8,
                        offset: 23,
                        alt: 4,
                    },
                ),
                ImplGpioPin::from_range(&PadRange::new("BAR", 0..2), GpioRange::no_offset(3, 9)),
            ],
        )
        .unwrap();
        assert_eq!(std::str::from_utf8(&actual).unwrap(), expected);
    }
}
