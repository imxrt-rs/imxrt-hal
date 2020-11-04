# imxrt-dma

Direct Memory Access (DMA) driver for i.MX RT processors

`imxrt-dma` is a lower-level DMA driver for all i.MX RT processors.
It provides an `unsafe` interface for allocating DMA channels, and for
scheduling DMA transactions. `imxrt-dma` also provides some traits and
abstractions that help to coordinate DMA transfers.

This DMA driver may be re-exported from a HAL. If it is, you should consider
using the safer APIs provided by your HAL.

## Portability

This DMA driver works across all considered i.MX RT variants (1010 and 1060
family). You must make sure that the DMA channel you're creating is valid for
your i.MX RT processor. This only matters on i.MX RT 1010 processors, which
only support 16 DMA channels. Creating an invalid channel for your 1010 processor
will result in a channel that references reserved memory.

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
