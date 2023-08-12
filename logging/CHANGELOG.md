# Changelog

## Unreleased

Increase the USB bulk max packet size (MPS) from 64 to 512. This is required
for USB high-speed enumeration on some hosts. This adds 896 extra bytes to
your firmware image.

## [0.1.1] 2023-03-14

Read the bulk OUT endpoint in the USB device logger. This resolves some poor
behaviors observed with Linux hosts.

## [0.1.0] 2022-01-05

First release. See the README and API documentation for the baseline features.

`imxrt-uart-log` users are encouraged to use the LPUART back-end provided by
this package. Note that it may have different integration requirements; see
the package documentation to understand those requirements.

[0.1.1]: https://github.com/imxrt-rs/imxrt-hal/compare/0.1.0-log...0.1.1-log
[0.1.0]: https://github.com/imxrt-rs/imxrt-hal/releases/tag/0.1.0-log
