//! Chip family APIs.
//!
//! These submodules may vary by chip family.

cfg_if::cfg_if! {
    if #[cfg(family = "imxrt10xx")] {
        mod imxrt10xx;
        pub use imxrt10xx::*;
    } else if #[cfg(family = "imxrt11xx")] {
        mod imxrt11xx;
        pub use imxrt11xx::*;
    } else {
        mod none; pub use none::*;
    }
}
