# imxrt-ral

This project provides a Rust RAL (register access layer) for all NXP i.MX RT
microcontrollers.

This is very much modeled on the great stm32ral crate which does much the same
for ST's fantastic STM32 family.  In fact this readme is largely based on
*that* README.md.

The crate is entirely generated using patched SVD files, patched using svdtools.

## What is it?

imxrt-ral is an experiment into a lightweight register access layer. It provides
access to every register, and provides constants which define the fields and
possible field values in those registers. In that sense it is a lot like C
device header files. However, it also provides a couple of simple macros that
permit very easy register access, with very simple generated code which
is efficient even without optimisations enabled.

The main aims are simplicity, compactness, and completeness. You get a module
structure that contains a struct for each peripheral comprising just its
registers in order, and you get a lot of constants for field widths, positions,
and possible values. There's not much else, so it takes little disk and builds
very quickly. It covers all registers of all i.MX RT devices, including core
Cortex-M peripherals, and aims to include full enumerated values for each
field as soon as possible.

Please consider trying it out and contributing or leaving feedback!

## Quick Example

```rust
use imxrt_ral::{read_reg, write_reg, modify_reg, reset_reg};
use imxrt_ral::{gpio};

// For safe access we have to first `take()` the peripheral instance.
// This only returns Some(Instance) if that instance is not already
// taken; otherwise it returns None. This ensures that no other code can be
// simultaneously accessing the peripheral, which could lead to a race
// condition. There's `release()` to return it. See below for unsafe use.
let gpio1 = gpio::GPIO1::take().unwrap();

// Field-level read/modify/write, with either named values or just literals.
// Most of your code will look like this.
modify_reg!(gpio, gpio1, ICR1, ICR1: HIGH_LEVEL, ICR1: LOW_LEVEL);
while read_reg!(gpio, gpio1, DR, DR && 0x00000001) {
    let pa0 = read_reg!(gpio, gpioa, DR, 0x00000001);
}

// You can also reset whole registers or specific fields
reset_reg!(gpio, gpio1, GPIO1, ICR1, ICR1, ICR2);
reset_reg!(gpio, gpioa, GPIO1, ICR1);

// Whole-register read/modify/write.
// Rarely used but nice to have the option.
let port = read_reg!(gpio, gpioa, IDR);
write_reg!(gpio, gpioa, ODR, 0x12345678);
modify_reg!(gpio, gpioa, MODER, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself.
// The macros above just expand to these forms for you, bringing
// the relevant constants into scope. Nothing else is going on.
let pa1 = (gpioa.IDR.read() & gpio::IDR::IDR1::mask) >> gpio::IDR::IDR1::offset;
gpioa.ODR.write(gpio::ODR::ODR2::RW::Output << gpio::ODR::ODR2::offset);

// Once you're done with a peripheral, you can release it so it is available
// to `take()` again. You can't use `gpioa` after this line.
gpio::GPIOA::release(gpioa);

// For unsafe access, you don't need to first call `take()`, just use `GPIOA`:
unsafe { modify_reg!(gpio, GPIOA, MODER, MODER1: Output) };
// With the `nosync` feature set, this is the only way to access registers.
```

See [the `stm32ral`'s example project](https://github.com/adamgreig/stm32ral-example) for
a more complete example that should build out of the box. Note that this example project
is for a different microcontroller; however, the ideas are the same.

## Why use imxrt-ral?

* Small and lightweight (~11MB total file size)
* Simple (just 4 macros and a lot of constants)
* Quick to compile (~3s build time)
* Covers i.MX RT devices in one crate
* Supports `cortex-m-rt` via the `rt` feature, including interrupts
* Supports `cortex-m-rtfm` via the `rtfm` feature, exposing a `device`
  with all peripherals taken
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use imxrt-ral?

* Still experimental, might have breaking changes to API design
* Won't keep you warm burning CPU time
* A bit like what you're used to from C header files
