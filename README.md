# i.MX RT Crates

i.MX RT register access layer, hardware abstraction layer, and svd patches.

This is very much modeled on the great stm32ral and stm32-rs crates which do 
much the same for ST's fantastic STM32 family.

## Goals

* Create *the* collaborative group to support using Rust on NXP's i.MX RT series.
* Fast compilation
* Simple but useful register level access, may not be as complete as svd2rust
  in ease of use but won't take very long to build.
* Embedded HAL support.
* RTFM support.
* EVK board support
* Popular boards such as the Teensy 4.

## imxrt-ral Supported Devices

- [x] imxrt1011
- [x] imxrt1015
- [x] imxrt1021
- [x] imxrt1051
- [x] imxrt1052
- [x] imxrt1061
- [x] imxrt1062
- [x] imxrt1064

## imxrt-hal Supported Devices

- [ ] imxrt1011
- [ ] imxrt1015
- [ ] imxrt1021
- [ ] imxrt1051
- [ ] imxrt1052
- [ ] imxrt1061
- [x] imxrt1062
- [ ] imxrt1064
