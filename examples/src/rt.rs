//! Runtime support.
//!
//! # Safety
//!
//! The code in this module executes before static memory is
//! initialized. It is careful to not touch any static memory
//! in its implementation.

use crate::ral;
use cortex_m_rt::pre_init;

#[inline(always)]
unsafe fn disable_rtwdog() {
    // See 47.4.1 in the reference manual.
    ral::write_reg!(ral::rtwdog, ral::rtwdog::RTWDOG, CNT, 0xD928C520);
    ral::modify_reg!(ral::rtwdog, ral::rtwdog::RTWDOG, CS, EN: EN_0);
}

#[pre_init]
unsafe fn pre_init() {
    cortex_m::interrupt::free(|_| {
        disable_rtwdog();
    });
}
