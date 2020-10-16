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
pub mod aipstz;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod dcdc_v1;

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
pub mod aoi;

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
pub mod xbara;

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
    feature = "imxrt1011",
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
pub mod trng_v1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod snvs_v1;

#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod ccm_analog_v1;

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
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usbphy;

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

#[cfg(any(feature = "doc", feature = "imxrt1011", feature = "imxrt1015"))]
pub mod usb1_v1;

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
pub mod usbnc;

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
    feature = "imxrt1011",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod lpuart_v1;

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
pub mod lpi2c;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1021"
))]
pub mod flexio1;

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
pub mod gpio;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1011",
    feature = "imxrt1015",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod spdif_v1;

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
pub mod pit_v1;

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
    feature = "imxrt1015",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod adc1;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod trng_v2;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod usb_analog_v1;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod dma0_v1;

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
pub mod gpc_v1;

#[cfg(any(feature = "doc", feature = "imxrt1015", feature = "imxrt1021"))]
pub mod lpuart_v2;

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
pub mod tmr1;

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
pub mod flexspi;

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
pub mod lpspi_v1;

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
pub mod xbarb;

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
pub mod enc1;

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
pub mod pwm1;

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
pub mod gpc_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod can_v1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod ocotp_v1;

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
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod enet;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usb1_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1021",
    feature = "imxrt1051",
    feature = "imxrt1052"
))]
pub mod spdif_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod dcdc_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pit_v2;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod iomuxc_gpr_v1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod snvs_v2;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod ccm_analog_v2;

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
pub mod usb_analog_v2;

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
pub mod dma0_v2;

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
pub mod ccm_v1;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod flexio_v1;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod iomuxc_v1;

#[cfg(any(feature = "doc", feature = "imxrt1051", feature = "imxrt1052"))]
pub mod semc_v1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1051",
    feature = "imxrt1052",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod lpspi_v2;

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
pub mod nvic_v1;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1052",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod pxp;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1052",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod csi;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod iomuxc_gpr_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ccm_analog_v3;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod usb_analog_v3;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod ccm_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod flexio_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod can_v2;

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
pub mod ocotp_v2;

#[cfg(any(feature = "doc", feature = "imxrt1061", feature = "imxrt1062"))]
pub mod iomuxc_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod semc_v2;

#[cfg(any(
    feature = "doc",
    feature = "imxrt1061",
    feature = "imxrt1062",
    feature = "imxrt1064"
))]
pub mod nvic_v2;

#[cfg(any(feature = "doc", feature = "imxrt1062", feature = "imxrt1064"))]
pub mod lcdif;
