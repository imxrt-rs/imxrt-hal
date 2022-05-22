Preset clock tree configurations and frequencies.

Use `configure` to simply configure the clock tree for a given
run mode. After `configure`, the system clocks run at the frequencies
described by each `*_frequency` function. The frequencies for a given
run mode are less than or equal to the maximum allowed for the given
run mode. Consult your MCU's reference manual for more information.

Use `*_frequency` functions to understand the target system clock frequencies.
Note that these functions are `const`, and should be usable in constant
contexts.

```
use imxrt_hal as hal;
use hal::{ccm::clock_tree, RunMode};

# #[cfg(feature = "imxrt1060")] {
// Functions suffixed with '_frequency' are const.
const _: () = assert!(clock_tree::ahb_frequency(RunMode::Overdrive) == 600_000_000);
# }
```

`configure` affects the following system clocks:

- AHB (ARM core clock)
- IPG
- PERCLK
- LPSPI
- UART
- LPI2C

# Example

```no_run
use imxrt_hal as hal;
use imxrt_ral as ral;

use hal::ccm::clock_tree;

const RUN_MODE: hal::RunMode = hal::RunMode::Overdrive;

# fn x() -> Option<()> {
let mut ccm = ral::ccm::CCM::take()?;
let mut ccm_analog = ral::ccm_analog::CCM_ANALOG::take()?;

clock_tree::configure(RUN_MODE, &mut ccm, &mut ccm_analog);
# Some(()) }
```
