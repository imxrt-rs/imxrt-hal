#[cfg(any(feature="doc", feature="armv6m", feature="armv7em", feature="armv7m"))]
pub mod dcb;

#[cfg(any(feature="doc", feature="armv6m", feature="armv7em", feature="armv7m"))]
pub mod dwt;

#[cfg(any(feature="doc", feature="armv6m", feature="armv7em", feature="armv7m"))]
pub mod syst;

#[cfg(any(feature="doc", feature="armv7em", feature="armv7m"))]
pub mod cpb;

#[cfg(any(feature="doc", feature="armv7em", feature="armv7m"))]
pub mod cpuid;

#[cfg(any(feature="doc", feature="armv7em", feature="armv7m"))]
pub mod fpb;

#[cfg(any(feature="doc", feature="armv7em", feature="armv7m"))]
pub mod itm;

#[cfg(any(feature="doc", feature="armv7em", feature="armv7m"))]
pub mod tpiu;

