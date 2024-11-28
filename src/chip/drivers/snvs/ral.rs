//! Custom RAL-like interface for SNVS components.
//!
//! This API allows us to separate the SNVS registers by functional
//! capabilities.

#![allow(unused, non_snake_case, non_upper_case_globals)] // Compatibility with RAL

/// Low power domain.
pub mod lp {
    /// Core SNVS registers.
    pub mod core {
        use crate::ral::RWRegister;

        /// Taken from the imxrt-ral definitions.
        #[repr(C)]
        pub struct RegisterBlock {
            /// SNVS_LP Lock Register
            pub LPLR: RWRegister<u32>,

            /// SNVS_LP Control Register
            pub LPCR: RWRegister<u32>,

            /// SNVS_LP Master Key Control Register
            pub LPMKCR: RWRegister<u32>,

            /// SNVS_LP Security Violation Control Register
            pub LPSVCR: RWRegister<u32>,

            _reserved1: [u32; 1],

            /// SNVS_LP Tamper Detectors Configuration Register
            pub LPTDCR: RWRegister<u32>,

            /// SNVS_LP Status Register
            pub LPSR: RWRegister<u32>,
        }

        pub mod LPLR {
            pub use crate::ral::snvs::LPLR::*;
        }

        pub mod LPCR {
            pub use crate::ral::snvs::LPCR::*;
        }

        pub mod LPSR {
            pub use crate::ral::snvs::LPSR::*;
        }
    }

    /// SRTC registers.
    pub mod srtc {
        use crate::ral::RWRegister;

        /// Taken from the imxrt-ral definitions.
        #[repr(C)]
        pub struct RegisterBlock {
            /// SNVS_LP Secure Real Time Counter MSB Register
            pub LPSRTCMR: RWRegister<u32>,

            /// SNVS_LP Secure Real Time Counter LSB Register
            pub LPSRTCLR: RWRegister<u32>,
        }

        pub mod LPSRTCMR {
            pub use crate::ral::snvs::LPSRTCMR::*;
        }
        pub mod LPSRTCLR {
            pub use crate::ral::snvs::LPSRTCLR::*;
        }
    }

    /// The core SNVS registers.
    pub type Core = super::Instance<core::RegisterBlock>;
    /// SRTC-specific registers.
    pub type Srtc = super::Instance<srtc::RegisterBlock>;
}

pub struct Instance<RB>(*const RB);
// Safety: OK to send, since pointer points to static peripheral memory.
unsafe impl<RB> Send for Instance<RB> {}

impl<RB> core::ops::Deref for Instance<RB> {
    type Target = RB;
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: inspection of this module shows that all pointers are derived from
        // valid static addresses.
        unsafe { &*self.0 }
    }
}

/// RAL components.
pub(super) struct Components {
    pub(super) lp_core: lp::Core,
    pub(super) lp_srtc: lp::Srtc,
}

/// Create SNVS components from the RAL instance.
pub(super) fn new(snvs: crate::ral::snvs::SNVS) -> Components {
    Components {
        lp_core: Instance(core::ptr::addr_of!(snvs.LPLR) as *const _),
        lp_srtc: Instance(core::ptr::addr_of!(snvs.LPSRTCMR) as *const _),
    }
}
