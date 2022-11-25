//! Clock control module for 11xx MCUs.
//!
//! The implementation assumes that the CCM operates in
//! "unassigned mode." See the section on CCM modes in
//! the reference manual (15.5.1.) for more information.
//! The API mimics the high-level clock gate and tree APIs
//!  for the 10xx family.

pub mod clock_gate;
pub mod clock_tree;
pub mod output_source;

pub use crate::common::ccm::XTAL_OSCILLATOR_HZ;
