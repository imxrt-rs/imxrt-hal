# imxrt-log

An extension of `imxrt-hal` to enable device logging. `imxrt-log` supports two
logging front-ends:

- [`defmt`][defmt-docs] for efficient logging.
- [`log`][log-docs] for text-based logging.

It supports two different back-end peripherals:

- A high-speed USB serial device.
- Serial UART with DMA.

For more information, see the API documentation. To try the various front-
and back-ends on hardware, use the `*_logging` examples maintained with
`imxrt-hal`.

[defmt-docs]: https://defmt.ferrous-systems.com
[log-docs]: https://docs.rs/log/0.4/log/
