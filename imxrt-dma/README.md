# imxrt-dma

Direct Memory Access (DMA) driver for i.MX RT processors

`imxrt-dma` is a lower-level DMA driver for all i.MX RT processors.
It provides an `unsafe` interface for allocating DMA channels, and for
scheduling DMA transactions. `imxrt-dma` also provides some traits and
abstractions that help to coordinate DMA transfers.

This DMA driver may be re-exported from a HAL. If it is, you should consider
using the safer APIs provided by your HAL.

## Features

The table below describes the feature flags that this driver supports:

|  Feature flag   | Description             | Default? |
| --------------- | ----------------------- | -------- |
| `"channels-32"` | Support 32 DMA channels |     ✓    |

Most i.MX RT processors support 32 DMA channels. If your chip does not support
32 DMA channels, you should disable the default features.

## Example

Use DMA channel 7 to perform a DMA-powered memory copy.

```rust
use imxrt_dma::{Channel, Transfer};

let mut channel = unsafe { Channel::new(7) };
channel.reset();

let source: [u32; 32] = [5; 32];
let destination: [u32; 32] = [0; 32];

let tx = Transfer::buffer_linear(source.as_ptr(), source.len());
let rx = Transfer::buffer_linear(destination.as_ptr(), destination.len());

channel.set_always_on();
channel.set_disable_on_completion(true);

unsafe {
    channel.set_source_transfer(&tx);
    channel.set_destination_transfer(&rx);
}

channel.set_minor_loop_elements::<u32>(1);
channel.set_transfer_iterations(source.len() as u16);

unsafe {
    channel.enable();
    channel.start();
}

if channel.is_error() {
    panic!("Transaction failed!");
}

while !channel.is_complete() {}

assert_eq!(destination, [5;32]);
```
