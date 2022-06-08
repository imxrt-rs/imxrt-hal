Clock control module.

Unlike other drivers in this package, this module only provides a
thin layer over the `imxrt-ral` APIs. It's fairly low level, but
more discoverable than the registers and reference manual.

# Overview

Use `clock_gate` APIs to enable or disable the clock gates for
various peripherals. You'll need to enable clock gates before you
start using peripherals. Other modules expose APIs for the various
CCM clock roots.

# Visibility

If you can see this module, it's because a chip family feature is
enabled in the HAL. These symbols may vary depending on the selected
feature.
