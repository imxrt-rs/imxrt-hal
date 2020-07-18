//! An `imxrt-iomuxc` implementation crate for the i.MX RT 106x processor family
//!
//! The crate exports all of the i.MX RT 106x processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits. This crate, combined with the `imxrt-iomuxc`
//! interface crate, lets you design and use embedded Rust APIs that require processor pad
//! configurations.
//!
//! # Example
//!
//! In the example below, we implement a hypothetical `uart_new` function, which is responsible
//! for preparing a UART peripheral. To properly configure the peripheral, we need the two
//! pads that represent a peripheral's TX and RX pins. The implementation will use the
//! `imxrt_iomuxc::uart::prepare()` function to prepare the pins.
//!
//! Note the trait bounds on `uart_new`. The usage requires that
//!
//! - the user provides one TX and one RX pin
//! - the modules for each pin match
//!
//! ```no_run
//! use imxrt_iomuxc as iomuxc;
//! use iomuxc::uart::{Pin, TX, RX};
//!
//! # struct UART;
//! /// Creates a UART peripheral from the TX and RX pads, and a baud rate
//! fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! where
//!     T: Pin<Direction = TX>,
//!     R: Pin<Direction = RX, Module = <T as Pin>::Module>,
//! {
//!     // Check the imxrt-iomuxc documentation to understand why
//!     // this is unsafe.
//!     unsafe {
//!         iomuxc::uart::prepare(&mut tx);
//!         iomuxc::uart::prepare(&mut rx);
//!     }
//!     // Prepare the rest of the UART peripheral, and return it...
//!     # UART
//! }
//!
//! # let ad_b0_13 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B0_13::new() };
//! # let ad_b0_12 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B0_12::new() };
//! // AD_B0_13 and AD_B0_12 are a suitable pair of UART pins
//! uart_new(ad_b0_12, ad_b0_13, 115_200);
//! ```
//!
//! Specifically, the trait bounds guard against non-UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::uart::{Pin, TX, RX};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = TX>,
//! #     R: Pin<Direction = RX, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::uart::prepare(&mut tx);
//! #         iomuxc::uart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let ad_b0_10 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B0_10::new() };
//! # let ad_b0_11 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B0_11::new() };
//! // Neither pad is a valid UART pin
//! uart_new(ad_b0_10, ad_b0_11, 115_200);
//! ```
//!
//! It also guards against mismatched UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::uart::{Pin, TX, RX};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = TX>,
//! #     R: Pin<Direction = RX, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::uart::prepare(&mut tx);
//! #         iomuxc::uart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let ad_b0_13 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B0_13::new() };
//! # let ad_b1_02 = unsafe { imxrt106x_iomuxc::ad_b0::AD_B1_02::new() };
//! // AD_B1_02 is a UART2 TX pin, but AD_B0_13 is a UART1 RX pin
//! uart_new(ad_b1_02, ad_b0_13, 115_200);
//! ```

#![no_std]

mod i2c;
mod pwm;
mod spi;
mod uart;

use imxrt_iomuxc as iomuxc;

iomuxc::define_pad!();
include!(concat!(env!("OUT_DIR"), "/pads.rs"));
pub use pads::*;

mod bases {
    imxrt_iomuxc::define_base!(EMC, 0x401F_8014, 0x401F_8204);
    imxrt_iomuxc::define_base!(AD_B0, 0x401F_80BC, 0x401F_82AC);
    imxrt_iomuxc::define_base!(AD_B1, 0x401F_80FC, 0x401F_82EC);
    imxrt_iomuxc::define_base!(B0, 0x401F_813C, 0x401F_832C);
    imxrt_iomuxc::define_base!(B1, 0x401F_817C, 0x401F_836C);
    imxrt_iomuxc::define_base!(SD_B0, 0x401F_81BC, 0x401F_83AC);
    imxrt_iomuxc::define_base!(SD_B1, 0x401F_81D4, 0x401F_83C4);
}
