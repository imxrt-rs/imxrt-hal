//! True Random Number Generator (TRNG)
//!
//! Provides basic support for the True Random Number Generator. The TRNG generates truly random
//! data and is intended for use as a generator of entropy.
//!
//! The TRNG is fairly slow - 15 minutes to an hour to generate 1 megabyte - so it probably
//! should only be used to generate a relatively small amount of entropy for a cryptographic
//! algorithm. Occasionally retrieving entropy from it won't necessarily need to block, as
//! this driver retrieves 512 bits at a time.
//!
//! ## RngCore Support
//!
//! When the crate feature `rand_core` is enabled, the TRNG can be wrapped in a struct that
//! implements [`rand_core`][rand_core]'s `RngCore` trait (via `into_rng()`). The [`rand`][rand]
//! crate's `Rng` trait automatically implements high-level functions on top of `RngCore`.
//!
//! This feature is opt-in because Cargo's current feature resolution may trigger a build failure
//! (or require the use of an [unstable Cargo feature][nfr]) if any dev-dependency has enabled the
//! `std` feature of `rand_core`.
//!
//! Note that only the `try_fill_bytes` function of `RngCore` allows reporting an error. The others
//! will panic if the TRNG reports an error. Errors appear to be extremely rare in the default
//! configuration (none were seen over 3GB of data), but it's possible they will be more common in
//! certain situations, such as extreme temperatures or an inconsistent power supply. The non-public
//! Security Reference Manual may have more information.
//!
//! If you intend to use the `RngCore` wrapper, you should increase the
//! [retry count](Unclocked::set_retry_count) before clocking the TRNG.
//!
//! [rand_core]: https://crates.io/crates/rand_core
//! [rand]: https://crates.io/crates/rand
//! [nfr]: https://github.com/rust-lang/cargo/issues/7916

use core::fmt;

use embedded_hal::blocking::rng::Read;

use crate::ccm;
use crate::ral::trng::{self, Instance};
use crate::ral::{self, modify_reg, read_reg, write_reg};

/// The TRNG, not yet enabled.
pub struct Unclocked {
    reg: Instance,
    sample_mode: SampleMode,
    retry_count: u32,
}

impl Unclocked {
    pub(crate) fn new(trng: Instance) -> Self {
        Unclocked {
            reg: trng,
            sample_mode: Default::default(),
            retry_count: 1, // _DEFAULT_RETRY_COUNT
        }
    }

    /// Set the `SampleMode`.
    pub fn set_sample_mode(&mut self, mode: SampleMode) {
        self.sample_mode = mode;
    }

    /// Set the number of times to retry after a test failure before an error is declared.
    ///
    /// Valid range `1..=15`. Defaults to `1`.
    pub fn set_retry_count(&mut self, retry_count: u32) -> Result<(), InvalidRetryCountError> {
        if (1..=15).contains(&retry_count) {
            self.retry_count = retry_count;
            Ok(())
        } else {
            Err(InvalidRetryCountError(()))
        }
    }

    /// Clock and configure the True Random Number Generator with default settings,
    /// plus the settings contained in this `Unclocked` struct.
    pub fn clock(self, ccm: &mut ccm::Handle) -> TRNG {
        // Not supported: locking TRNG to prevent programmability.
        // A number of things here are likely configurable if you have access to the SRM.
        // Without it only limited configuration is safe. Garbage configs lead to endless errors.

        let (ccm, _) = ccm.raw();
        modify_reg!(ral::ccm, ccm, CCGR6, CG6: 0b11); // trng_clk_enable
        modify_reg!(trng, self.reg, MCTL, PRGM: 1);
        modify_reg!(trng, self.reg, MCTL, RST_DEF: 1);
        // enter program mode and reset to defaults
        // it isn't clear what defaults it actually sets, so let's set the tests manually

        // All these values are sourced only from the #defines in the MCUXpresso TRNG driver for the
        // IMXRT1062. The doc comments contain both typos and completely wrong values.

        write_reg!(trng, self.reg, SCMISC, RTY_CT: self.retry_count, LRUN_MAX: 34); // _RUN_MAX_LIMIT

        // Note: The SDK uses _MAX and _MIN for values, but the registers use the max value and a
        // range. The SDK _MIN values are expressed as (max - range), making it easy to ensure
        // that these values are correct.
        write_reg!(trng, self.reg, SCM, MONO_MAX: 1384, MONO_RNG: 268); // _MONOBIT_
        write_reg!(trng, self.reg, SCR1, RUN1_MAX: 405, RUN1_RNG: 178); // _RUNBIT1_
        write_reg!(trng, self.reg, SCR2, RUN2_MAX: 220, RUN2_RNG: 122); // _RUNBIT2_
        write_reg!(trng, self.reg, SCR3, RUN3_MAX: 125, RUN3_RNG: 88); // _RUNBIT3_
        write_reg!(trng, self.reg, SCR4, RUN4_MAX: 75, RUN4_RNG: 64); // _RUNBIT4_
        write_reg!(trng, self.reg, SCR5, RUN5_MAX: 47, RUN5_RNG: 46); // _RUNBIT5_
        write_reg!(trng, self.reg, SCR6P, RUN6P_MAX: 47, RUN6P_RNG: 46); // _RUNBIT6PLUS_

        write_reg!(trng, self.reg, PKR, PKR_MAX: 26912); // _POKER_MAXIMUM
        write_reg!(trng, self.reg, PKRRNG, PKR_RNG: 2467);

        write_reg!(trng, self.reg, FRQ, FRQ_MAX: 25600); // _FREQUENCY_MAXIMUM
        write_reg!(trng, self.reg, FRQMIN, FRQ_MIN: 1600); // _FREQUENCY_MINIMUM

        write_reg!(trng, self.reg, SDCTL, SAMP_SIZE: 2500, ENT_DLY: 3200); // _SAMPLE_SIZE, _ENTROPY_DELAY
        write_reg!(trng, self.reg, SBLIM, SB_LIM: 63); // _SPARSE_BIT_LIMIT

        // set sample mode, exit program mode
        modify_reg!(trng, self.reg, MCTL, SAMP_MODE: self.sample_mode as u32);
        modify_reg!(trng, self.reg, MCTL, PRGM: 0);
        // for 1015, 1021, maybe other non i.MX chips: set TRNG_ACC to 1 here
        read_reg!(trng, self.reg, ENT15);
        // reading ENT15 triggers new entropy generation
        TRNG::new(self.reg)
    }
}

/// TRNG sampling mode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SampleMode {
    /// von Neumann data in both entropy shifter and statistical checks. Approximately 4x slower
    /// than the other modes.
    VonNeumann = trng::MCTL::SAMP_MODE::RW::SAMP_MODE_0,
    /// Raw data in both entropy shifter and statistical checks. Likely lower quality than the
    /// other two modes.
    Raw = trng::MCTL::SAMP_MODE::RW::SAMP_MODE_1,
    /// von Neumann data in entropy shifter, raw data in statistical checks
    VonNeumannRaw = trng::MCTL::SAMP_MODE::RW::SAMP_MODE_2,
}

impl SampleMode {
    /// Convert the register value to the enum. Not using std TryFrom to avoid making this public;
    /// users shouldn't need this.
    fn try_from_reg(value: u32) -> Option<SampleMode> {
        match value {
            trng::MCTL::SAMP_MODE::RW::SAMP_MODE_0 => Some(Self::VonNeumann),
            trng::MCTL::SAMP_MODE::RW::SAMP_MODE_1 => Some(Self::Raw),
            trng::MCTL::SAMP_MODE::RW::SAMP_MODE_2 => Some(Self::VonNeumannRaw),
            _ => None,
        }
    }
}

impl Default for SampleMode {
    /// Returns `VonNeumannRaw`.
    fn default() -> Self {
        // "Set sample mode of the TRNG ring oscillator to Von Neumann, for better random data.
        // It is optional." <- SDK, explaining why they set sample mode to 0 (VN)
        // Teensyduino uses VNRaw, the reason appears to be this post:
        // https://forum.pjrc.com/threads/54711-Teensy-4-0-First-Beta-Test?p=195000&viewfull=1#post195000
        // VNRaw appears to produce equivalent quality output and is ~4x faster than VN.
        // As such, VonNeumannRaw is the default SampleMode here.
        Self::VonNeumannRaw
    }
}

/// The True Random Number Generator.
pub struct TRNG {
    reg: Instance,
    block: [u32; 16],
    index: usize,
}

impl fmt::Debug for TRNG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TRNG")
            .field("block", &self.block)
            .field("index", &self.index)
            .finish()
    }
}

impl TRNG {
    fn new(reg: Instance) -> Self {
        Self {
            reg,
            block: [0; 16],
            index: 16, // equal to len, to trigger immediate retrieval
        }
    }

    /// Return the next `u32` available. May need to retrieve another block of random numbers.
    pub fn next_u32(&mut self) -> nb::Result<u32, Error> {
        self.retrieve_if_needed()?;
        let data = nb::Result::Ok(self.block[self.index]);
        self.index += 1;
        data
    }

    /// Retrieve another block of random numbers if we've used them all up.
    fn retrieve_if_needed(&mut self) -> nb::Result<(), Error> {
        if self.index >= self.block.len() {
            self.retrieve()?;
            self.index = 0;
        }
        Ok(())
    }

    /// Retrieve another block of random numbers.
    fn retrieve(&mut self) -> nb::Result<(), Error> {
        let mctl = read_reg!(trng, self.reg, MCTL);
        if (mctl & trng::MCTL::ERR::mask) != 0 {
            let flags = self.get_error_flags();
            write_reg!(trng, self.reg, MCTL, mctl); // write reg back to clear error
            return Err(nb::Error::Other(Error(flags)));
        }
        if (mctl & trng::MCTL::ENT_VAL::mask) == 0 {
            return Err(nb::Error::WouldBlock); // not ready to read entropy
        }
        self.block[0] = read_reg!(trng, self.reg, ENT0);
        self.block[1] = read_reg!(trng, self.reg, ENT1);
        self.block[2] = read_reg!(trng, self.reg, ENT2);
        self.block[3] = read_reg!(trng, self.reg, ENT3);
        self.block[4] = read_reg!(trng, self.reg, ENT4);
        self.block[5] = read_reg!(trng, self.reg, ENT5);
        self.block[6] = read_reg!(trng, self.reg, ENT6);
        self.block[7] = read_reg!(trng, self.reg, ENT7);
        self.block[8] = read_reg!(trng, self.reg, ENT8);
        self.block[9] = read_reg!(trng, self.reg, ENT9);
        self.block[10] = read_reg!(trng, self.reg, ENT10);
        self.block[11] = read_reg!(trng, self.reg, ENT11);
        self.block[12] = read_reg!(trng, self.reg, ENT12);
        self.block[13] = read_reg!(trng, self.reg, ENT13);
        self.block[14] = read_reg!(trng, self.reg, ENT14);
        self.block[15] = read_reg!(trng, self.reg, ENT15);
        // this can probably be written as a loop with unsafe code
        read_reg!(trng, self.reg, ENT0);
        // SDK (fsl_trng.c):
        //     Dummy read. Defect workaround.
        //     TRNG could not clear ENT_VAL flag automatically, application
        //     had to do a dummy reading operation for anyone TRNG register
        //     to clear it firstly, then to read the RTENT0 to RTENT15 again
        // This appears unnecessary on the 1062? done anyway in case it's necessary for another chip
        Ok(())
    }

    /// Retrieve all known error flags.
    fn get_error_flags(&self) -> ErrorFlags {
        let status = read_reg!(trng, self.reg, STATUS) & 0xFFFF;
        // all the error flags in STATUS are in the low 16 bits
        let mut flags = ErrorFlags::from_bits_truncate(status);
        flags.set(
            ErrorFlags::FCT_FAIL,
            read_reg!(trng, self.reg, MCTL, FCT_FAIL) == 1,
        );
        flags
    }

    /// Disable the TRNG and its clock.
    ///
    /// This should preserve any previously set retry count and sample mode.
    pub fn disable(self, ccm: &mut ccm::Handle) -> Unclocked {
        let (ccm, _) = ccm.raw();
        modify_reg!(trng, self.reg, MCTL, PRGM: 1);
        while read_reg!(trng, self.reg, MCTL, TSTOP_OK) == 0 {
            #[allow(deprecated)]
            core::sync::atomic::spin_loop_hint();
        }
        // need to wait for TSTOP_OK before disabling clock or the ring oscillator will keep running
        let sample_mode = read_reg!(trng, self.reg, MCTL, SAMP_MODE);
        let retry_count = read_reg!(trng, self.reg, SCMISC, RTY_CT);
        let mut unclocked = Unclocked::new(self.reg);
        unclocked.sample_mode = SampleMode::try_from_reg(sample_mode).unwrap_or_default();
        unclocked.retry_count = retry_count.max(1); // RTY_CT is 4 bits, 0 is invalid
        modify_reg!(ral::ccm, ccm, CCGR6, CG6: 0); // disable trng clock
        unclocked
    }

    /// Wrap the TRNG in a struct that implements `rand_core`'s `RngCore` trait.
    #[cfg(feature = "rand_core")]
    pub fn into_rng(self) -> RngCoreWrapper {
        RngCoreWrapper(self)
    }
}

/// Wrapper struct around [`TRNG`] that implements `RngCore`.
#[cfg(feature = "rand_core")]
pub struct RngCoreWrapper(TRNG);

#[cfg(feature = "rand_core")]
impl RngCoreWrapper {
    /// Deconstruct this wrapper and return the TRNG struct.
    pub fn into_inner(self) -> TRNG {
        self.0
    }
}

#[cfg(feature = "rand_core")]
impl rand_core::RngCore for RngCoreWrapper {
    /// Return the next random `u32`.
    ///
    /// # Panics
    ///
    /// Panics if the TRNG returns an error.
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.fill_bytes(&mut bytes);
        u32::from_be_bytes(bytes)
    }

    /// Return the next random `u64`.
    ///
    /// # Panics
    ///
    /// Panics if the TRNG returns an error.
    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0; 8];
        self.fill_bytes(&mut bytes);
        u64::from_be_bytes(bytes)
    }

    /// Fill `dest` with random data.
    ///
    /// # Panics
    ///
    /// Panics if the TRNG returns an error.
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest).expect("TRNG returned an error")
    }

    /// Fill `dest` with random data.
    ///
    /// If an error occurs, the error's `code` will be identical to the [`ErrorFlags`] that this
    /// driver would have reported. If none of them were set, the `code` will have only the highest
    /// bit set (which does not correspond to any flag).
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        // defer to Read implementation, converting error to rand_core's Error
        self.0.read(dest).map_err(|e| {
            let code = if e.0.bits == 0 { 1 << 31 } else { e.0.bits };
            // Safety: Highest bit is set above if no flags were set.
            unsafe { core::num::NonZeroU32::new_unchecked(code).into() }
        })
    }
}

impl Read for TRNG {
    type Error = Error;
    // e-h RNG Read is a *blocking* trait, so no WouldBlock here
    // Read is part of the unproven API and will likely be changed in the future

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let mut data = [0; 4];
        let mut index = 4;
        for b in buffer.iter_mut() {
            if index == 4 {
                data = nb::block!(self.next_u32())?.to_be_bytes();
                index = 0;
            }
            *b = data[index];
            index += 1;
        }
        Ok(())
    }
}

/// A TRNG error occurred, such as a statistical test failing.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Error(pub ErrorFlags);

bitflags::bitflags! {
    /// Specific errors that may occur during entropy generation
    pub struct ErrorFlags : u32 {
        // STATUS register starts here (automatically set from bits)
        /// 1-bit run sampling 0s test failed
        const TF1BR0 = 1 << 0;
        /// 1-bit run sampling 1s test failed
        const TF1BR1 = 1 << 1;
        /// 2-bit run sampling 0s test failed
        const TF2BR0 = 1 << 2;
        /// 2-bit run sampling 1s test failed
        const TF2BR1 = 1 << 3;
        /// 3-bit run sampling 0s test failed
        const TF3BR0 = 1 << 4;
        /// 3-bit run sampling 1s test failed
        const TF3BR1 = 1 << 5;
        /// 4-bit run sampling 0s test failed
        const TF4BR0 = 1 << 6;
        /// 4-bit run sampling 1s test failed
        const TF4BR1 = 1 << 7;
        /// 5-bit run sampling 0s test failed
        const TF5BR0 = 1 << 8;
        /// 5-bit run sampling 1s test failed
        const TF5BR1 = 1 << 9;
        /// 6-plus-bit run sampling 0s test failed
        const TF6PBR0 = 1 << 10;
        /// 6-plus-bit run sampling 1s test failed
        const TF6PBR1 = 1 << 11;
        /// Sparse bit test failed
        const TFSB = 1 << 12;
        /// Long run test failed
        const TFLR = 1 << 13;
        /// Poker test failed
        const TFP = 1 << 14;
        /// Mono bit test failed
        const TFMB = 1 << 15;
        // MCTL register starts here (set manually)
        /// Count taken during entropy generation was outside the defined range of FRQ_MIN to FRQ_MAX
        const FCT_FAIL = 1 << 16;
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred in the TRNG module")
    }
}

/// The specified retry count was outside of the valid range of `1..=15`.
#[derive(Copy, Clone, Debug)]
pub struct InvalidRetryCountError(());

impl fmt::Display for InvalidRetryCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The specified retry count was outside of the valid range of 1..=15")
    }
}

/// InvalidRetryCountError was accidentally constructible. Test to ensure it isn't.
/// ```compile_fail
/// use imxrt1060_hal as hal;
/// use hal::trng::InvalidRetryCountError;
/// let err = InvalidRetryCountError;
/// ```
#[cfg(doctest)]
struct UnconstructibleError;
