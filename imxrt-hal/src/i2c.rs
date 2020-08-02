//! I2C support
//!
//! # Example
//!
//! ```no_run
//! use imxrt_hal;
//! use imxrt_hal::i2c::ClockSpeed;
//! use embedded_hal::blocking::i2c::WriteRead;
//!
//! let mut peripherals = imxrt_hal::Peripherals::take().unwrap();
//!
//! let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
//!     &mut peripherals.ccm.handle,
//!     imxrt_hal::ccm::i2c::ClockSelect::OSC, // 24MHz clock...
//!     imxrt_hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
//! );
//!
//! let mut i2c3 = i2c3_builder.build(
//!     peripherals.iomuxc.ad_b1.p07,
//!     peripherals.iomuxc.ad_b1.p06,
//! );
//!
//! i2c3.set_bus_idle_timeout(core::time::Duration::from_micros(200)).unwrap();
//! i2c3.set_pin_low_timeout(core::time::Duration::from_millis(1)).unwrap();
//! i2c3.set_clock_speed(ClockSpeed::KHz400).unwrap();
//!
//! let mut input = [0; 3];
//! let output = [0x74];
//! # const MY_SLAVE_ADDRESS: u8 = 0;
//!
//! i2c3.write_read(MY_SLAVE_ADDRESS, &output, &mut input).unwrap();
//! ```

pub use crate::iomuxc::consts::{Unsigned, U1, U2, U3, U4};

use crate::ccm;
use crate::iomuxc::i2c;
use crate::ral;
use core::marker::PhantomData;
use embedded_hal::blocking;

/// Unclocked I2C modules
///
/// The `Unclocked` struct represents all four unconfigured I2C peripherals.
/// Once clocked, you'll have the ability to build I2C peripherals from the
/// compatible processor pins.
pub struct Unclocked {
    pub(crate) i2c1: ral::lpi2c::Instance,
    pub(crate) i2c2: ral::lpi2c::Instance,
    pub(crate) i2c3: ral::lpi2c::Instance,
    pub(crate) i2c4: ral::lpi2c::Instance,
}
impl Unclocked {
    /// Enable clocks to all I2C modules, returning a builder for the four I2C modules.
    pub fn clock(
        self,
        handle: &mut ccm::Handle,
        clock_select: ccm::i2c::ClockSelect,
        divider: ccm::i2c::PrescalarSelect,
    ) -> (Builder<U1>, Builder<U2>, Builder<U3>, Builder<U4>) {
        let (ccm, _) = handle.raw();
        // First, disable clocks
        ral::modify_reg!(ral::ccm, ccm, CCGR2, CG3: 0, CG4: 0, CG5: 0);
        ral::modify_reg!(ral::ccm, ccm, CCGR6, CG12: 0);
        // Select clock, and commit prescalar
        ral::modify_reg!(ral::ccm, ccm, CSCDR2, LPI2C_CLK_PODF: (divider as u32), LPI2C_CLK_SEL: (clock_select as u32));
        // Enable clocks
        ral::modify_reg!(ral::ccm, ccm, CCGR2, CG3: 0b11, CG4: 0b11, CG5: 0b11);
        ral::modify_reg!(ral::ccm, ccm, CCGR6, CG12: 0b11);
        let source_clock = ccm::Frequency::from(clock_select) / ccm::Divider::from(divider);
        (
            Builder::new(source_clock, self.i2c1),
            Builder::new(source_clock, self.i2c2),
            Builder::new(source_clock, self.i2c3),
            Builder::new(source_clock, self.i2c4),
        )
    }
}

/// An I2C builder that can build and I2C peripheral
pub struct Builder<M> {
    _module: PhantomData<M>,
    reg: ral::lpi2c::Instance,
    /// Frequency of the LPI2C source clock. This
    /// accounts for the divider.
    source_clock: ccm::Frequency,
}

impl<M> Builder<M>
where
    M: Unsigned,
{
    fn new(source_clock: ccm::Frequency, reg: ral::lpi2c::Instance) -> Self {
        Builder {
            _module: PhantomData,
            reg,
            source_clock,
        }
    }

    /// Builds an I2C peripheral from the SCL and SDA pins. The return
    /// is a configured I2C master running at 100KHz.
    pub fn build<SCL, SDA>(self, mut scl: SCL, mut sda: SDA) -> I2C<M>
    where
        SCL: i2c::Pin<Module = M, Signal = i2c::SCL>,
        SDA: i2c::Pin<Module = M, Signal = i2c::SDA>,
    {
        crate::iomuxc::i2c::prepare(&mut scl);
        crate::iomuxc::i2c::prepare(&mut sda);
        I2C::new(self.source_clock, self.reg)
    }
}

/// I2C Clock speed
#[derive(Clone, Copy, Debug)]
pub enum ClockSpeed {
    /// 100 KHz
    KHz100,
    /// 400 KHz
    KHz400,
    /// 1 MHz
    MHz1,
}

impl Default for ClockSpeed {
    fn default() -> Self {
        ClockSpeed::KHz100
    }
}

impl ClockSpeed {
    /// Sets the clock speed parameters
    ///
    /// # Safety
    ///
    /// The function touches I2C registers that should only be touched
    /// while the I2C master is disabled.
    unsafe fn set(self, source_clock: ccm::Frequency, reg: &ral::lpi2c::Instance) {
        // Baud rate = (source_clock/2^prescale)/(CLKLO+1+CLKHI+1 + FLOOR((2+FILTSCL)/2^prescale)
        // Assume CLKLO = 2*CLKHI, SETHOLD = CLKHI, DATAVD = CLKHI/2, FILTSCL = FILTSDA = 0,
        // and that risetime is negligible (less than 1 cycle).
        use core::cmp;
        use ral::lpi2c::MCFGR1::PRESCALE::RW::*;

        log::debug!(
            "I2C baud rate = {:?}, source clock = {:?}",
            self,
            source_clock
        );
        let source_clock = source_clock.0;
        const PRESCALARS: [u32; 8] = [
            PRESCALE_0, PRESCALE_1, PRESCALE_2, PRESCALE_3, PRESCALE_4, PRESCALE_5, PRESCALE_6,
            PRESCALE_7,
        ];

        struct ByError {
            prescalar: u32,
            clkhi: u32,
            error: u32,
            computed_rate: u32,
        }

        let baud_rate: u32 = match self {
            Self::KHz100 => 100_000,
            Self::KHz400 => 400_000,
            Self::MHz1 => 1_000_000,
        };

        // prescale = 1, 2, 4, 8, ... 128
        // divider = 2 ^ prescale
        let dividers = PRESCALARS.iter().copied().map(|prescalar| 1 << prescalar);
        let clkhis = 1u32..32u32;
        // possibilities = every divider with every clkhi (8 * 30 == 240 possibilities)
        let possibilities =
            dividers.flat_map(|divider| core::iter::repeat(divider).zip(clkhis.clone()));
        let errors = possibilities.map(|(divider, clkhi)| {
            let computed_rate = if 1 == clkhi {
                // See below for justification on magic numbers.
                // In the 1 == clkhi case, the + 3 is the minimum allowable CLKLO value
                // + 1 is CLKHI itself
                (source_clock / divider) / ((1 + 3 + 2) + 2 / divider)
            } else {
                // CLKLO = 2 * CLKHI, allows us to do 3 * CLKHI
                // + 2 accounts for the CLKLOW + 1 and CLKHI + 1
                // + 2 accounts for the FLOOR((2 + FILTSCL)) factor
                (source_clock / divider) / ((3 * clkhi + 2) + 2 / divider)
            };
            let error = cmp::max(computed_rate, baud_rate) - cmp::min(computed_rate, baud_rate);
            ByError {
                prescalar: divider.saturating_sub(1).count_ones(),
                clkhi, /* (1..32) in u8 range */
                error,
                computed_rate,
            }
        });

        let ByError {
            prescalar,
            clkhi,
            error,
            computed_rate,
        } = errors.min_by(|lhs, rhs| lhs.error.cmp(&rhs.error)).unwrap();

        let (clklo, sethold, datavd) = if clkhi < 2 {
            (3, 2, 1)
        } else {
            (clkhi * 2, clkhi, clkhi / 2)
        };

        log::debug!(
            "COMPUTED_RATE = {}, ERROR = {}, PRESCALAR = {}, CLKHI = {}, CLKLO = {}, SETHOLD = {}, DAVAVD = {}",
            computed_rate,
            error,
            prescalar,
            clkhi,
            clklo,
            sethold,
            datavd
        );
        ral::write_reg!(
            ral::lpi2c,
            reg,
            MCCR0,
            CLKHI: clkhi,
            CLKLO: clklo,
            SETHOLD: sethold,
            DATAVD: datavd
        );
        ral::write_reg!(ral::lpi2c, reg, MCFGR1, PRESCALE: prescalar);
    }
}

/// An I2C master
///
/// By default, the I2C master runs at 100KHz, Use `set_clock_speed` to vary
/// the I2C bus speed.
pub struct I2C<M> {
    reg: ral::lpi2c::Instance,
    _module: PhantomData<M>,
    /// LPI2C effective input clock frequency
    source_clock: ccm::Frequency,
}

/// Indicates an error when computing the parameters that control
/// the clock speed.
#[derive(Debug)]
pub struct ClockSpeedError;
/// Indicates an error when computing the parameters that control
/// the pin low timeout
#[derive(Debug)]
pub struct PinLowTimeoutError;
/// Indicates an error when computing the parameters that control
/// the bus idle timeout
#[derive(Debug)]
pub struct BusIdleTimeoutError;

const RETRIES: usize = 100_000;

impl<M> I2C<M>
where
    M: Unsigned,
{
    fn new(source_clock: ccm::Frequency, reg: ral::lpi2c::Instance) -> Self {
        let mut i2c = I2C {
            reg,
            _module: PhantomData,
            source_clock,
        };
        ral::write_reg!(ral::lpi2c, i2c.reg, MCR, RST: RST_1);

        // Enables I2C master
        i2c.set_clock_speed(ClockSpeed::KHz100).unwrap();
        ral::write_reg!(ral::lpi2c, i2c.reg, MFCR, RXWATER: 0b01, TXWATER: 0b01);
        i2c
    }

    fn with_master_disabled<F: FnMut() -> R, R>(&self, mut act: F) -> R {
        // Note that we should really specify the 'instance module'. This approach
        // assumes that the reset values for all instance modules are the same, which
        // is correct in this context.
        //
        // Also, this should be refactored to account for stacked with_master_disabled
        // calls. See the UART module's approach for more details. Not doing this now
        // in order to focus on the porting.
        ral::reset_reg!(ral::lpi2c, self.reg, LPI2C1, MCR);
        let res = act();
        ral::write_reg!(ral::lpi2c, self.reg, MCR, MEN: MEN_1);
        res
    }

    /// Set the I2C master clock speed
    pub fn set_clock_speed(&mut self, clock_speed: ClockSpeed) -> Result<(), ClockSpeedError> {
        self.with_master_disabled(|| unsafe {
            // Safety: master is disabled
            clock_speed.set(self.source_clock, &self.reg);
            Ok(())
        })
    }

    /// Set the pin low timeout
    ///
    /// If SCL or, either SCL or SDA, is low for longer than the specified duration, then the
    /// I2C hardware indicates an error. If the timeout is `0`, then the detection is disabled.
    ///
    /// If the number of cycles required to represent the duration is too large, returns a
    /// `PinLowTimeoutError`. Try using a smaller duration.
    pub fn set_pin_low_timeout(
        &mut self,
        timeout: core::time::Duration,
    ) -> Result<(), PinLowTimeoutError> {
        let divider = 1 << ral::read_reg!(ral::lpi2c, self.reg, MCFGR1, PRESCALE);
        let pin_low_ticks: u16 = ccm::ticks(timeout, self.source_clock.0, divider)
            .map(|ticks: u16| ticks / 256)
            .into_iter()
            .next()
            .filter(|ticks| *ticks <= 0x0FFFu16)
            .ok_or(PinLowTimeoutError)?;
        log::debug!("PINLOW = 0x{:X}", pin_low_ticks);
        self.with_master_disabled(|| {
            ral::modify_reg!(
                ral::lpi2c,
                self.reg,
                MCFGR3,
                PINLOW: u32::from(pin_low_ticks)
            );
            Ok(())
        })
    }

    /// Set the bus idle timeout
    ///
    /// If both SCL and SDA are high for longer than the timeout, then the I2C bus is assumed to be
    /// idle and the master can generate a START condition. If the timeout is `0`, then the idle is
    /// disabled.
    ///
    /// If the number of cycles required to represent the duration is too large, returns a
    /// `BusIdleTimeoutError`. Try using a smaller timeout.
    pub fn set_bus_idle_timeout(
        &mut self,
        timeout: core::time::Duration,
    ) -> Result<(), BusIdleTimeoutError> {
        let divider = 1 << ral::read_reg!(ral::lpi2c, self.reg, MCFGR1, PRESCALE);
        let bus_idle_ticks: u16 = ccm::ticks(timeout, self.source_clock.0, divider)
            .into_iter()
            .next()
            .filter(|ticks| *ticks <= 0xFFFu16)
            .ok_or(BusIdleTimeoutError)?;
        log::debug!("BUSIDLE = 0x{:X}", bus_idle_ticks);
        self.with_master_disabled(|| {
            ral::modify_reg!(
                ral::lpi2c,
                self.reg,
                MCFGR2,
                BUSIDLE: u32::from(bus_idle_ticks)
            );
            Ok(())
        })
    }

    #[inline(always)]
    fn wait<F>(&mut self, on: F) -> Result<(), Error>
    where
        F: Fn(u32) -> bool,
    {
        for _ in 0..RETRIES {
            if on(self.check_errors()?) {
                return Ok(());
            }
        }
        Err(Error::WaitTimeout)
    }

    /// Clears all master status flags that are required to be
    /// low before acting as an I2C master.
    ///
    /// All flags are W1C.
    #[inline(always)]
    fn clear_status(&mut self) {
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MSR,
            EPF: EPF_1,
            SDF: SDF_1,
            NDF: NDF_1,
            ALF: ALF_1,
            FEF: FEF_1,
            PLTF: PLTF_1,
            DMF: DMF_1
        );
    }

    #[inline(always)]
    fn clear_fifo(&mut self) {
        ral::modify_reg!(ral::lpi2c, self.reg, MCR, RRF: RRF_1, RTF: RTF_1);
    }

    /// Check master status flags for erroneous conditions
    #[inline(always)]
    fn check_errors(&mut self) -> Result<u32, Error> {
        use ral::lpi2c::MSR::*;
        let status = ral::read_reg!(ral::lpi2c, self.reg, MSR);
        if (status & PLTF::mask) != 0 {
            Err(Error::PinLowTimeout)
        } else if (status & ALF::mask) != 0 {
            Err(Error::LostBusArbitration)
        } else if (status & NDF::mask) != 0 {
            Err(Error::UnexpectedNACK)
        } else if (status & FEF::mask) != 0 {
            Err(Error::FIFO)
        } else {
            Ok(status)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Master has lost arbitration
    LostBusArbitration,
    /// SCL and / or SDA went low for too long, despite our control
    PinLowTimeout,
    /// Detected a NACK when sending an address or data
    UnexpectedNACK,
    /// Sending or receiving data without a START
    FIFO,
    /// Requesting too much data in a receive; upper limit is `u8::MAX`
    RequestTooMuchData,
    /// Busy-wait on an internal flag was too long
    WaitTimeout,
}

macro_rules! target_fn {
    ($name:expr) => {
        concat!(module_path!(), "::", $name)
    };
}

impl<M> blocking::i2c::Write for I2C<M>
where
    M: Unsigned,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        use ral::lpi2c::MSR::*;
        self.clear_fifo();
        self.clear_status();
        log::trace!(target: target_fn!("write"), "WAIT MBF & TDF");
        self.wait(|msr| (msr & MBF::mask) == 0 && (msr & TDF::mask) != 0)?;

        log::trace!(target: target_fn!("write"), "START");
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MTDR,
            DATA: u32::from(addr) << 1,
            CMD: CMD_4
        );

        log::trace!(target: target_fn!("write"), "'{:?}' -> 0x{:X}", bytes, addr);
        for byte in bytes {
            self.wait(|msr| (msr & TDF::mask) != 0)?;
            ral::write_reg!(ral::lpi2c, self.reg, MTDR, DATA: *byte as u32);
        }

        log::trace!(target: target_fn!("write"), "WAIT TDF");
        self.wait(|msr| (msr & TDF::mask) != 0)?;
        log::trace!(target: target_fn!("write"), "STOP");
        ral::write_reg!(ral::lpi2c, self.reg, MTDR, CMD: CMD_2);

        log::trace!(target: target_fn!("write"), "WAIT EPF");
        self.wait(|msr| (msr & EPF::mask) != 0)?;

        Ok(())
    }
}

impl<M> blocking::i2c::WriteRead for I2C<M>
where
    M: Unsigned,
{
    type Error = Error;
    fn write_read(
        &mut self,
        address: u8,
        output: &[u8],
        input: &mut [u8],
    ) -> Result<(), Self::Error> {
        use ral::lpi2c::MSR::*;
        if input.len() > 256 {
            return Err(Error::RequestTooMuchData);
        }

        self.clear_fifo();
        self.clear_status();
        log::trace!(target: target_fn!("write_read"), "WAIT MBF & TDF");
        self.wait(|msr| (msr & MBF::mask) == 0 && (msr & TDF::mask) != 0)?;

        log::trace!(target: target_fn!("write_read"), "START");
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MTDR,
            DATA: u32::from(address) << 1,
            CMD: CMD_4
        );

        log::trace!(
            target: target_fn!("write_read"),
            "'{:?}' -> 0x{:X}",
            output,
            address
        );
        for byte in output {
            self.wait(|msr| (msr & TDF::mask) != 0)?;
            ral::write_reg!(ral::lpi2c, self.reg, MTDR, DATA: *byte as u32);
        }

        log::trace!(target: target_fn!("write_read"), "REPEAT START");
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MTDR,
            DATA: (u32::from(address) << 1) | 1,
            CMD: CMD_4
        );

        log::trace!(target: target_fn!("write_read"), "WAIT EPF");
        self.wait(|msr| (msr & EPF::mask) != 0)?;
        ral::write_reg!(ral::lpi2c, self.reg, MSR, EPF: EPF_1);

        if !input.is_empty() {
            log::trace!(target: target_fn!("write_read"), "WAIT TDF");
            self.wait(|msr| (msr & TDF::mask) != 0)?;

            log::trace!(
                target: target_fn!("write_read"),
                "'{}' -> 0x{:X}",
                input.len() - 1,
                address
            );
            ral::write_reg!(
                ral::lpi2c,
                self.reg,
                MTDR,
                DATA: (input.len() - 1) as u32,
                CMD: CMD_1
            );

            log::trace!(target: target_fn!("write_read"), "WAIT DATA");
            for slot in input.iter_mut() {
                let mut j = 0;
                loop {
                    use ral::lpi2c::MRDR::*;
                    self.check_errors()?;
                    let mrdr = ral::read_reg!(ral::lpi2c, self.reg, MRDR);
                    if mrdr & RXEMPTY::mask == 0 {
                        *slot = ((mrdr & DATA::mask) >> DATA::offset) as u8;
                        break;
                    }
                    j += 1;
                    if j > RETRIES {
                        return Err(Error::WaitTimeout);
                    }
                }
            }
        }

        log::trace!(target: target_fn!("write_read"), "WAIT TDF");
        self.wait(|msr| (msr & TDF::mask) != 0)?;
        log::trace!(target: target_fn!("write_read"), "STOP");
        ral::write_reg!(ral::lpi2c, self.reg, MTDR, CMD: CMD_2);

        log::trace!(target: target_fn!("write_read"), "WAIT EPF");
        self.wait(|msr| (msr & EPF::mask) != 0)?;

        Ok(())
    }
}

impl<M> blocking::i2c::Read for I2C<M>
where
    M: Unsigned,
{
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        use ral::lpi2c::MSR::*;
        if buffer.len() > 256 {
            return Err(Error::RequestTooMuchData);
        }

        if buffer.is_empty() {
            return Ok(());
        }

        self.clear_fifo();
        self.clear_status();
        log::trace!(target: target_fn!("read"), "WAIT MBF & TDF");
        self.wait(|msr| (msr & MBF::mask) == 0 && (msr & TDF::mask) != 0)?;

        log::trace!(target: target_fn!("read"), "START");
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MTDR,
            DATA: (u32::from(address) << 1) | 1,
            CMD: CMD_4
        );

        log::trace!(target: target_fn!("read"), "WAIT TDF");
        self.wait(|msr| (msr & TDF::mask) != 0)?;

        log::trace!(
            target: target_fn!("read"),
            "'{}' -> 0x{:X}",
            buffer.len() - 1,
            address
        );
        ral::write_reg!(
            ral::lpi2c,
            self.reg,
            MTDR,
            DATA: (buffer.len() - 1) as u32,
            CMD: CMD_1
        );

        log::trace!(target: target_fn!("read"), "WAIT DATA");
        for slot in buffer.iter_mut() {
            let mut j = 0;
            loop {
                use ral::lpi2c::MRDR::*;
                self.check_errors()?;
                let mrdr = ral::read_reg!(ral::lpi2c, self.reg, MRDR);
                if mrdr & RXEMPTY::mask == 0 {
                    *slot = ((mrdr & DATA::mask) >> DATA::offset) as u8;
                    break;
                }
                j += 1;
                if j > RETRIES {
                    return Err(Error::WaitTimeout);
                }
            }
        }

        log::trace!(target: target_fn!("read"), "STOP");
        ral::write_reg!(ral::lpi2c, self.reg, MTDR, CMD: CMD_2);

        log::trace!(target: target_fn!("read"), "WAIT EPF");
        self.wait(|msr| (msr & EPF::mask) != 0)?;

        Ok(())
    }
}
