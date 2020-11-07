# imxrt1060-hal

`imxrt1060-hal` is a Rust hardware abstraction layer that's specific to i.MX
RT 1060 processors. This includes some of the following processor parts:

- i.MX RT 1061
- i.MX RT 1062

It is the successor to `imxrt-hal`, version 0.4, with `feature = "imxrt1062"`.

## Features

The table below describes the optional features supported by `imxrt1060-hal`.

| Feature  | Description                        |
| -------- | ---------------------------------- |
| `"rt"`   | Runtime support with `cortex-m-rt` |
| `"rtic"` | Support for RTIC                   |
