use imxrt_iomuxc_build::{write_impl_gpio_pins, GpioRange, ImplGpioPin, PadRange};

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
