# imxrt-hal

This project provides a Rust HAL (hardware abstraction layer) for all NXP i.MX RT
microcontrollers based on the imxrt-ral crate.

A feature flag needs to be set for any of the supported i.MX RT SoC.

## What is it?

imxrt-hal is an experiment into a lightweight hardware abstraction layer. It
provides access to some of the peripherals of the i.MX RT series processors
from NXP using embedded-hal and other community driven hardware APIs.

The main aims are fast compilation, compactness, and simplicity.

Please consider trying it out and contributing or leaving feedback!

## Goals

* Simple to use and hard to use incorrectly
* All peripherals and busses supported
* Support the entire i.MX RT Series with a single crate to maximize code reuse

## Supported Devices

* imxrt1061
* imxrt1062
* imxrt1064
