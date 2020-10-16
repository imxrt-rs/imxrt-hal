#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod aipstz;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod dcdc_1015_1021;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod aoi_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod aoi_1051_1052_1061_1062_1064;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod xbara;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod xbara1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod iomuxc_snvs_gpr;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ewm;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod wdog;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod rtwdog;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod trng_1011_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod snvs_1011_1015_1021;

#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod ccm_analog_1011_1015;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod tempmon;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod xtalosc24m;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod usbphy_1011_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usbphy_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod csu;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod usbnc_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usbnc_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod dcp;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pgc;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod romc;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod lpuart;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod lpi2c;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod flexio1;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod gpio_1015_1021;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod gpio_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod gpio_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod spdif_1015_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod gpt;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod kpp;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod systemcontrol;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod pit_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod flexram;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod adc;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod trng_1015_1021;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod usb_analog_1015_1021;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod dma0_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod dmamux;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod gpc_1015_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod tmr;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod flexspi_1015_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod flexspi_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod sai;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod xbarb_1015_1021;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod xbarb_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod enc;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pwm;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod bee;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod cmp;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod iomuxc_snvs;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod gpc_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod can_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod ocotp_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usdhc;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod enet_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod enet_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usb;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod spdif_1021_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod dcdc_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pit_1051_1052_1061_1062_1064;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod iomuxc_gpr_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod snvs_1051_1052_1061_1062_1064;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod ccm_analog_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pmu;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod usb_analog_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod tsc;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod dma0_1051_1052_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod src;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod ccm_1051_1052;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod flexio_1051_1052;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod iomuxc_1051_1052;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod semc_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod lpspi;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod adc_etc;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod nvic_1051_1052;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1052",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pxp;

#[cfg(any(feature = "doc", feature = "imxrt1062", feature = "imxrt1064"))]
pub mod csi;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod iomuxc_gpr_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ccm_analog_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usb_analog_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ccm_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod flexio_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod can_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod can3;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ocotp_1061_1062_1064;

#[cfg(any(feature = "doc", feature = "imxrt1061", feature = "imxrt1062"))]
pub mod iomuxc_1061_1062;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod semc_1061_1062_1064;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod nvic_1061_1062_1064;

#[cfg(any(feature = "doc", feature = "imxrt1062", feature = "imxrt1064"))]
pub mod lcdif;
