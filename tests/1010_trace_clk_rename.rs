//! Tests backwards compatibility of a typo correction.

#![cfg(chip = "imxrt1010")]
#![allow(deprecated)]

use hal::ccm::output_source::clko2::Selection;
use imxrt_hal as hal;

const USER_CONSTANT: Selection = Selection::TracClk;

#[test]
fn trace_clk_match() {
    assert!(matches!(USER_CONSTANT, Selection::TracClk));
    assert!(matches!(USER_CONSTANT, Selection::TraceClk));
}
