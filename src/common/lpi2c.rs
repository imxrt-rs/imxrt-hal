//! Low-power inter-integrated circuit.
//!
//! The `Lpi2c` driver implements all embedded-hal I2C traits. Use these traits to perform
//! common I2C I/O. The driver also exposes lower-level APIs for the LPI2C controller.
//!
//! # Example
//!
//! Demonstrates how to create an LPI2C peripheral, and perform a write-read with
//! a device. This example skips the LPI2C clock configuration. To understand LPI2C
//! clock configuration, see the [`ccm::lpi2c_clk`](crate::ccm::lpi2c_clk) documentation.
//!
//! ```no_run
//! use imxrt_hal as hal;
//! use imxrt_ral as ral;
//! use hal::lpi2c::{self, Lpi2c};
//! use ral::{ccm::CCM, lpi2c::LPI2C3};
//! use eh02::blocking::i2c::WriteRead;
//!
//! let mut pads = // Handle to all processor pads...
//!     # unsafe { imxrt_iomuxc::imxrt1060::Pads::new() };
//!
//! # || -> Option<()> {
//! let mut ccm = unsafe { CCM::instance() };
//! let mut i2c3 = unsafe { LPI2C3::instance() };
//!
//! # const LPI2C_CLK_HZ: u32 = 8_000_000;
//! const LPI2C_400KHz: lpi2c::Timing = lpi2c::Timing::ideal(LPI2C_CLK_HZ, lpi2c::ClockSpeed::KHz400);
//!
//! let mut i2c3 = Lpi2c::new(
//!     i2c3,
//!     lpi2c::Pins {
//!         scl: pads.gpio_ad_b1.p07,
//!         sda: pads.gpio_ad_b1.p06,
//!     },
//!     &LPI2C_400KHz,
//! );
//!
//! let mut input = [0; 3];
//! let output = [0x74];
//! # const MY_DEVICE_ADDRESS: u8 = 0;
//!
//! i2c3.write_read(MY_DEVICE_ADDRESS, &output, &mut input).ok()?;
//!
//! // Release the driver components...
//! let (i2c3, pins) = i2c3.release();
//!
//! // Re-construct without pins...
//! let mut i2c3 = Lpi2c::without_pins(i2c3, &LPI2C_400KHz);
//! # Some(()) }();
//! ```
//!
//! # Limitations
//!
//! This driver supports standard, fast, and fast+ modes. High speed mode is not
//! yet supported, and supporting the mode was not considered in the initial driver
//! design.
use crate::iomuxc::consts;

use crate::iomuxc::lpi2c;
use crate::ral;
use eh02::blocking::i2c as blocking;

/// Data direction.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Transmit direction (leaving the peripheral).
    Tx,
    /// Receive direction (entering the peripheral).
    Rx,
}

/// LPI2C pins.
pub struct Pins<SCL, SDA>
where
    SCL: lpi2c::Pin<Signal = lpi2c::Scl>,
    SDA: lpi2c::Pin<Signal = lpi2c::Sda, Module = SCL::Module>,
{
    /// Serial clock.
    pub scl: SCL,
    /// Serial data.
    pub sda: SDA,
}

/// An LPI2C driver.
///
/// Use this driver to communicate with I2C devices. This driver
/// implements various `embedded-hal` I2C traits, and you should
/// prefer these implementations for their ease of use.
///
/// See the [module-level documentation](crate::lpi2c) for an example
/// of how to construct this driver.
pub struct Lpi2c<P, const N: u8> {
    lpi2c: ral::lpi2c::Instance<N>,
    pins: P,
}

impl<SCL, SDA, const N: u8> Lpi2c<Pins<SCL, SDA>, N>
where
    SCL: lpi2c::Pin<Signal = lpi2c::Scl, Module = consts::Const<N>>,
    SDA: lpi2c::Pin<Signal = lpi2c::Sda, Module = consts::Const<N>>,
{
    /// Create an LPI2C driver from an LPI2C instance and a pair of pins.
    ///
    /// When this call returns, the LPI2C pins are configured for their
    /// LPI2C functions, the controller is enabled after reset, and the driver
    /// is using the provided timing configuration for the clock.
    pub fn new(
        lpi2c: crate::ral::lpi2c::Instance<N>,
        mut pins: Pins<SCL, SDA>,
        timings: &Timing,
    ) -> Self {
        lpi2c::prepare(&mut pins.scl);
        lpi2c::prepare(&mut pins.sda);
        Self::init(lpi2c, pins, timings)
    }
}

impl<const N: u8> Lpi2c<(), N> {
    /// Create an I2C driver from an LPI2C instance.
    ///
    /// This is similar to [`new()`](Self::new), but it does not configure pins.
    /// You're responsible for configuring pins, and for making sure
    /// the pin configuration doesn't change while this driver is in use.
    pub fn without_pins(lpi2c: ral::lpi2c::Instance<N>, timings: &Timing) -> Self {
        Self::init(lpi2c, (), timings)
    }
}

impl<P, const N: u8> Lpi2c<P, N> {
    /// The peripheral instance.
    pub const N: u8 = N;

    fn init(mut lpi2c: ral::lpi2c::Instance<N>, pins: P, timings: &Timing) -> Self {
        ral::write_reg!(ral::lpi2c, lpi2c, MCR, RST: RST_1);
        while ral::read_reg!(ral::lpi2c, lpi2c, MCR, RST == RST_1) {
            ral::write_reg!(ral::lpi2c, lpi2c, MCR, RST: RST_0);
        }

        // I2C disabled due to reset.
        set_timings(&mut lpi2c, timings);

        ral::write_reg!(ral::lpi2c, lpi2c, MFCR, RXWATER: 0b01, TXWATER: 0b01);
        ral::write_reg!(ral::lpi2c, lpi2c, MCR, MEN: MEN_1);

        Lpi2c { lpi2c, pins }
    }

    /// Indicates if the controller is (`true`) or is not (`false`) enabled.
    pub fn is_controller_enabled(&self) -> bool {
        ral::read_reg!(ral::lpi2c, self.lpi2c, MCR, MEN == MEN_1)
    }

    /// Enable (`true`) or disable (`false`) the controller.
    pub fn set_controller_enable(&mut self, enable: bool) {
        ral::modify_reg!(ral::lpi2c, self.lpi2c, MCR, MEN: enable as u32)
    }

    /// Reset the controller.
    ///
    /// Note that this may not not reset all peripheral state, like the controller
    /// enabled state.
    pub fn reset_controller(&mut self) {
        ral::modify_reg!(ral::lpi2c, self.lpi2c, MCR, RST: RST_1);
        while ral::read_reg!(ral::lpi2c, self.lpi2c, MCR, RST == RST_1) {
            ral::modify_reg!(ral::lpi2c, self.lpi2c, MCR, RST: RST_0);
        }
    }

    /// Release the LPI2C components.
    ///
    /// This does not change any component state; it releases the components as-is.
    /// If you need to obtain the registers in a known, good state, consider calling
    /// methods like [`reset_controller()`](Self::reset_controller) before releasing
    /// the registers.
    pub fn release(self) -> (ral::lpi2c::Instance<N>, P) {
        (self.lpi2c, self.pins)
    }

    /// Read the controller status bits.
    #[inline]
    pub fn controller_status(&self) -> ControllerStatus {
        ControllerStatus::from_bits_truncate(ral::read_reg!(ral::lpi2c, self.lpi2c, MSR))
    }

    /// Clear the controller status bits that are set high.
    ///
    /// The implementation will clear any read-only bits, so it's OK to pass in
    /// `ControllerStatus::all()`.
    #[inline]
    pub fn clear_controller_status(&self, status: ControllerStatus) {
        let msr = status & ControllerStatus::W1C;
        ral::write_reg!(ral::lpi2c, self.lpi2c, MSR, msr.bits());
    }

    /// Resets the transmit and receive FIFOs.
    #[inline(always)]
    pub fn clear_fifo(&mut self) {
        ral::modify_reg!(ral::lpi2c, self.lpi2c, MCR, RRF: RRF_1, RTF: RTF_1);
    }

    /// Enqueue a command into the controller transmit data register.
    ///
    /// `enqueue_controller_command` does not check that the FIFO can hold the
    /// command. Check for the transmit data flag in the status
    /// response to understand the FIFO's state.
    #[inline]
    pub fn enqueue_controller_command(&self, command: ControllerCommand) {
        ral::write_reg!(ral::lpi2c, self.lpi2c, MTDR, command.raw());
    }

    /// Read the controller receive data register.
    ///
    /// Returns `None` if there is no data in the receive FIFO.
    #[inline]
    pub fn read_data_register(&self) -> Option<u8> {
        let (empty, data) = ral::read_reg!(ral::lpi2c, self.lpi2c, MRDR, RXEMPTY, DATA);
        if empty != 0 {
            None
        } else {
            Some(data as u8)
        }
    }

    /// Temporarily disable the LPI2C peripheral.
    ///
    /// The handle to a [`Disabled`](crate::lpi2c::Disabled) driver lets you modify
    /// LPI2C settings that require a fully disabled peripheral.
    pub fn disabled<R>(&mut self, func: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        let mut disabled = Disabled::new(&mut self.lpi2c);
        func(&mut disabled)
    }
    /// If the bus is busy, return the status flags in the error
    /// position.
    fn check_busy(&self) -> Result<(), ControllerStatus> {
        let status = self.controller_status();
        if status.is_bus_busy() {
            Err(status)
        } else {
            Ok(())
        }
    }

    /// Keep checking for errors until `what` produces a value.
    fn wait_for<T>(
        &self,
        what: impl Fn(ControllerStatus) -> Option<T>,
    ) -> Result<T, ControllerStatus> {
        loop {
            let status = self.controller_status();
            if status.has_error() {
                return Err(status);
            }
            if let Some(val) = what(status) {
                return Ok(val);
            }
        }
    }

    /// Wait for the controller to become idle.
    fn wait_for_controller_idle(&self) -> Result<(), ControllerStatus> {
        self.wait_for(ControllerStatus::break_controller_idle)
    }

    /// Block until there's space in the transmit FIFO..
    ///
    /// Return errors if detected.
    fn wait_for_transmit(&self) -> Result<(), ControllerStatus> {
        self.wait_for(ControllerStatus::break_transmit_space)
    }

    /// Block until receiving a byte of data.
    ///
    /// Returns errors if detected.
    fn wait_for_data(&self) -> Result<u8, ControllerStatus> {
        self.wait_for(|_| self.read_data_register())
    }

    /// Wait for the end of packet, which happens for STOP or repeated
    /// START conditions.
    ///
    /// Returns errors if detected. This will not unblock for a non-repeated
    /// START.
    fn wait_for_end_of_packet(&self) -> Result<(), ControllerStatus> {
        self.wait_for(ControllerStatus::break_end_of_packet)
    }

    /// Borrow the pins.
    pub fn pins(&self) -> &P {
        &self.pins
    }

    /// Exclusively borrow the pins.
    pub fn pins_mut(&mut self) -> &mut P {
        &mut self.pins
    }

    /// Returns the bitflags that indicate enabled or disabled LPI2C interrupts.
    #[inline]
    pub fn interrupts(&self) -> Interrupts {
        let raw = ral::read_reg!(ral::lpi2c, self.lpi2c, MIER);
        let interrupts = Interrupts::from_bits_truncate(raw);
        interrupts ^ Interrupts::FIFO_ERROR
    }

    /// Enable or disable LPI2C interrupts.
    #[inline]
    pub fn set_interrupts(&self, interrupts: Interrupts) {
        let interrupts = interrupts ^ Interrupts::FIFO_ERROR;
        ral::write_reg!(ral::lpi2c, self.lpi2c, MIER, interrupts.bits());
    }

    /// Returns the watermark level for the given direction.
    #[inline]
    pub fn watermark(&self, direction: Direction) -> u8 {
        (match direction {
            Direction::Rx => ral::read_reg!(ral::lpi2c, self.lpi2c, MFCR, RXWATER),
            Direction::Tx => ral::read_reg!(ral::lpi2c, self.lpi2c, MFCR, TXWATER),
        }) as u8
    }

    /// Returns the FIFO status.
    #[inline]
    pub fn controller_fifo_status(&self) -> ControllerFifoStatus {
        let (rxcount, txcount) = ral::read_reg!(ral::lpi2c, self.lpi2c, MFSR, RXCOUNT, TXCOUNT);
        ControllerFifoStatus {
            rxcount: rxcount as u16,
            txcount: txcount as u16,
        }
    }
}

/// The number of words in each FIFO.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerFifoStatus {
    /// Number of words in the receive FIFO.
    pub rxcount: u16,
    /// Number of words in the transmit FIFO.
    pub txcount: u16,
}

/// Must be called only when the LPI2C peripheral is disabled.
fn set_timings<const N: u8>(lpi2c: &mut ral::lpi2c::Instance<N>, timings: &Timing) {
    let clock_config = timings.clock_configuration();
    ral::write_reg!(ral::lpi2c, lpi2c, MCCR0,
        CLKHI: clock_config.clkhi as u32,
        CLKLO: clock_config.clklo as u32,
        SETHOLD: clock_config.sethold as u32,
        DATAVD: clock_config.datavd as u32
    );
    ral::modify_reg!(ral::lpi2c, lpi2c, MCFGR1, PRESCALE: timings.prescaler() as u32);

    ral::modify_reg!(ral::lpi2c, lpi2c, MCFGR2,
        FILTSDA: clock_config.filtsda as u32,
        FILTSCL: clock_config.filtscl as u32,
        BUSIDLE: timings.busidle
    );
}

/// A temporarily disabled LPI2C peripheral.
///
/// This handle lets you modify LPI2C settings that require
/// a disabled peripheral.
pub struct Disabled<'a, const N: u8> {
    lpi2c: &'a mut ral::lpi2c::Instance<N>,
    men: bool,
}

impl<'a, const N: u8> Disabled<'a, N> {
    fn new(lpi2c: &'a mut ral::lpi2c::Instance<N>) -> Self {
        let men = ral::read_reg!(ral::lpi2c, lpi2c, MCR, MEN == MEN_1);
        ral::modify_reg!(ral::lpi2c, lpi2c, MCR, MEN: MEN_0);
        Self { lpi2c, men }
    }

    /// Modify LPI2C timing parameters.
    ///
    /// This call only affects parameters used in standard, fast, and fast+ modes.
    /// There is no support for switching into high-speed mode.
    pub fn set_timings(&mut self, timings: &Timing) {
        set_timings(self.lpi2c, timings);
    }

    /// Set the watermark level for a given direction.
    ///
    /// Returns the watermark level committed to the hardware. This may be different
    /// than the supplied `watermark`, since it's limited by the hardware.
    ///
    /// When `direction == Direction::Rx`, the receive data flag is set whenever the
    /// number of words in the receive FIFO is greater than `watermark`.
    ///
    /// When `direction == Direction::Tx`, the transmit data flag is set whenever the
    /// the number of words in the transmit FIFO is less than, or equal, to `watermark`.
    #[inline]
    pub fn set_watermark(&mut self, direction: Direction, watermark: u8) -> u8 {
        let max_watermark = match direction {
            Direction::Rx => 1 << ral::read_reg!(ral::lpi2c, self.lpi2c, PARAM, MRXFIFO),
            Direction::Tx => 1 << ral::read_reg!(ral::lpi2c, self.lpi2c, PARAM, MTXFIFO),
        };

        let watermark = watermark.min(max_watermark - 1);

        match direction {
            Direction::Rx => {
                ral::modify_reg!(ral::lpi2c, self.lpi2c, MFCR, RXWATER: watermark as u32)
            }
            Direction::Tx => {
                ral::modify_reg!(ral::lpi2c, self.lpi2c, MFCR, TXWATER: watermark as u32)
            }
        }

        watermark
    }
}

impl<const N: u8> Drop for Disabled<'_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpi2c, self.lpi2c, MCR, MEN: self.men as u32);
    }
}

bitflags::bitflags! {
    /// Status flags for the LPI2C controller.
    pub struct ControllerStatus : u32 {
        /// Bus busy flag.
        ///
        /// If high, the bus is busy.
        const BUS_BUSY = 1 << 25;
        /// Controller busy flag.
        ///
        /// If high, the controller is already active.
        const CONTROLLER_BUSY = 1 << 24;

        //
        // Begin W1C bits.
        //

        /// Data match flag.
        const DATA_MATCH = 1 << 14;
        /// Pin low timeout flag.
        const PIN_LOW_TIMEOUT = 1 << 13;
        /// FIFO error flag.
        const FIFO_ERROR = 1 << 12;
        /// Arbitration lost flag.
        const ARBITRATION_LOST = 1 << 11;
        /// NACK detected flag.
        const NACK_DETECTED = 1 << 10;
        /// STOP detected flag.
        const STOP_DETECTED = 1 << 9;
        /// End packet flag.
        const END_PACKET = 1 << 8;

        //
        // End W1C bits
        //

        /// Receive data flag.
        const RECEIVE_DATA = 1 << 1;
        /// Transmit data flag.
        const TRANSMIT_DATA = 1 << 0;
    }
}

impl ControllerStatus {
    /// Mask of write-1-clear bits.
    const W1C: Self = Self::from_bits_truncate(
        Self::DATA_MATCH.bits()
            | Self::PIN_LOW_TIMEOUT.bits()
            | Self::FIFO_ERROR.bits()
            | Self::ARBITRATION_LOST.bits()
            | Self::NACK_DETECTED.bits()
            | Self::STOP_DETECTED.bits()
            | Self::END_PACKET.bits(),
    );

    /// Bits that definitely indicate an error.
    const ERRORS: Self = Self::from_bits_truncate(
        Self::PIN_LOW_TIMEOUT.bits()
            | Self::FIFO_ERROR.bits()
            | Self::ARBITRATION_LOST.bits()
            | Self::NACK_DETECTED.bits(),
    );

    /// The status indicates that the bus is busy, and it's not
    /// because of us.
    const fn is_bus_busy(self) -> bool {
        self.contains(Self::BUS_BUSY) && !self.contains(Self::CONTROLLER_BUSY)
    }

    /// Break when the controller is idle.
    fn break_controller_idle(self) -> Option<()> {
        (!self.contains(Self::CONTROLLER_BUSY)).then_some(())
    }

    /// Break when there's an end of packet.
    fn break_end_of_packet(self) -> Option<()> {
        self.contains(Self::END_PACKET).then_some(())
    }

    /// Break when there's space in the TX FIFO.
    fn break_transmit_space(self) -> Option<()> {
        self.contains(Self::TRANSMIT_DATA).then_some(())
    }

    /// Indicates if any error bit is set.
    const fn has_error(self) -> bool {
        self.intersects(Self::ERRORS)
    }
}

bitflags::bitflags! {
    /// LPI2C interrupt settings.
    ///
    /// A set bit indicates that an interrupt is enabled.
    pub struct Interrupts : u32 {
        /// Data match interrupt enable.
        const DATA_MATCH = 1 << 14;
        /// Pin low timout interrupt enable.
        const PIN_LOW_TIMEOUT = 1 << 13;
        /// FIFO error interrupt enable.
        const FIFO_ERROR = 1 << 12;

        //
        // Note: according to the reference manual and SVD files,
        // FEIE is enabled when low, and disabled when high. This
        // is inconsist with all other bits. To handle the difference,
        // This driver flips the bit in order to provide a consistent
        // behavior in software.
        //

        /// Arbitration lost interrupt enable.
        const ARBITRATION_LOST = 1 << 11;
        /// NACK detect interrupt enable.
        const NACK_DETECT = 1 << 10;
        /// STOP detect interrupt enable.
        const STOP_DETECT = 1 << 9;
        /// End packet interrupt enable.
        const END_PACKET = 1 << 8;
        /// Receive data interrupt enable.
        const RECEIVE_DATA = 1 << 1;
        /// Transmit data interrupt enable.
        const TRANSMIT_DATA = 1 << 0;
    }
}

/// (N)ACK responses.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Response {
    /// Response is acknowledge.
    Ack,
    /// Response is not acknowledge.
    Nack,
}

/// LPI2C controller commands.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ControllerCommand {
    /// A transmit command.
    Transmit {
        /// The data byte to enqueue.
        byte: u8,
    },
    /// A receive command.
    Receive {
        /// How many bytes to receive.
        count: u8,
    },
    /// Generate a STOP condition.
    Stop,
    /// Receive and discard.
    ReceiveAndDiscard {
        /// How many bytes to receive and drop.
        drop: u8,
    },
    /// Generate a (repeated) start, transmit the
    /// address `addr`, and expect the `expect` response
    /// from a device.
    Start {
        /// The device you're addressing.
        ///
        /// You're responsible for shifting the address, and setting
        /// the read/write bit. Consider using the `read()` and `write()`
        /// methods for this purpose.
        addr: u8,
        /// The expected response from the device.
        expect: Response,
    },
}

impl ControllerCommand {
    /// Creates a (repeat) start command that describes a read
    /// from a device with address `addr`.
    ///
    /// The expected response is ACK.
    #[inline]
    pub const fn read(addr: u8) -> Self {
        Self::Start {
            addr: (addr << 1) | 1,
            expect: Response::Ack,
        }
    }

    /// Creates a (repeat) start command that describes a write
    /// to a device with address `addr`.
    ///
    /// The expected response is ACK.
    #[inline]
    pub const fn write(addr: u8) -> Self {
        Self::Start {
            addr: addr << 1,
            expect: Response::Ack,
        }
    }
}

impl ControllerCommand {
    /// Turn a command into its raw representation, permitting
    /// 32-bit writes to the transmit data register.
    const fn raw(self) -> u32 {
        use crate::ral::lpi2c::MTDR::CMD::{offset as OFFSET, RW::*};
        match self {
            Self::Transmit { byte } => (CMD_0 << OFFSET) | byte as u32,
            Self::Receive { count } => (CMD_1 << OFFSET) | count.saturating_sub(1) as u32,
            Self::Stop => CMD_2 << OFFSET,
            Self::ReceiveAndDiscard { drop } => (CMD_3 << OFFSET) | drop.saturating_sub(1) as u32,
            Self::Start {
                expect: Response::Ack,
                addr,
            } => (CMD_4 << OFFSET) | addr as u32,
            Self::Start {
                expect: Response::Nack,
                addr,
            } => (CMD_5 << OFFSET) | addr as u32,
        }
    }
}

//
// embedded-hal implementations.
//

impl<P, const N: u8> blocking::TransactionalIter for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn exec_iter<'a, O>(&mut self, address: u8, operations: O) -> Result<(), Self::Error>
    where
        O: IntoIterator<Item = blocking::Operation<'a>>,
    {
        let mut runner = transaction::Runner::new(self)?;
        for mut operation in operations {
            runner.next_operation(address, &mut operation)?;
        }
        runner.stop()?;
        Ok(())
    }
}

impl<P, const N: u8> blocking::WriteIter for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn write<B>(&mut self, address: u8, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        blocking::WriteIterRead::write_iter_read(self, address, bytes, &mut [])
    }
}

impl<P, const N: u8> blocking::WriteIterRead for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>,
    {
        self.check_busy()?;
        self.clear_fifo();
        self.clear_controller_status(ControllerStatus::W1C);
        self.wait_for_transmit()?;
        self.enqueue_controller_command(ControllerCommand::write(address));

        let cmds = bytes
            .into_iter()
            .map(|byte| ControllerCommand::Transmit { byte });
        for cmd in cmds {
            self.wait_for_transmit()?;
            self.enqueue_controller_command(cmd);
        }

        if !buffer.is_empty() {
            self.wait_for_transmit()?;
            self.enqueue_controller_command(ControllerCommand::read(address));

            self.wait_for_transmit()?;
            self.enqueue_controller_command(ControllerCommand::Receive {
                count: buffer.len() as u8,
            });

            for slot in buffer {
                *slot = self.wait_for_data()?;
            }
        }

        self.wait_for_transmit()?;
        self.enqueue_controller_command(ControllerCommand::Stop);
        self.wait_for_end_of_packet()?;
        self.wait_for_controller_idle()?;

        Ok(())
    }
}

impl<P, const N: u8> blocking::Transactional for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn exec(
        &mut self,
        address: u8,
        operations: &mut [blocking::Operation],
    ) -> Result<(), Self::Error> {
        let mut runner = transaction::Runner::new(self)?;
        for operation in operations {
            runner.next_operation(address, operation)?;
        }
        runner.stop()?;
        Ok(())
    }
}

impl<P, const N: u8> blocking::Read for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        blocking::Transactional::exec(self, address, &mut [blocking::Operation::Read(buffer)])
    }
}

impl<P, const N: u8> blocking::Write for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        blocking::Transactional::exec(self, address, &mut [blocking::Operation::Write(bytes)])
    }
}

impl<P, const N: u8> blocking::WriteRead for Lpi2c<P, N> {
    type Error = ControllerStatus;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        blocking::Transactional::exec(
            self,
            address,
            &mut [
                blocking::Operation::Write(bytes),
                blocking::Operation::Read(buffer),
            ],
        )
    }
}

impl eh1::i2c::Error for ControllerStatus {
    fn kind(&self) -> eh1::i2c::ErrorKind {
        use eh1::i2c::{ErrorKind, NoAcknowledgeSource};

        if self.contains(ControllerStatus::BUS_BUSY) {
            return ErrorKind::Bus;
        }

        if self.contains(ControllerStatus::NACK_DETECTED) {
            return ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown);
        }

        if self.contains(ControllerStatus::ARBITRATION_LOST) {
            return ErrorKind::ArbitrationLoss;
        }

        ErrorKind::Other
    }
}

impl<P, const N: u8> eh1::i2c::ErrorType for Lpi2c<P, N> {
    type Error = ControllerStatus;
}

impl<P, const N: u8> eh1::i2c::I2c for Lpi2c<P, N> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [eh1::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut runner = transaction::Runner::new(self)?;
        for operation in operations {
            // Convert from eh1 Operation to eh02 Operation
            let mut operation = match operation {
                eh1::i2c::Operation::Read(buffer) => blocking::Operation::Read(buffer),
                eh1::i2c::Operation::Write(buffer) => blocking::Operation::Write(buffer),
            };
            runner.next_operation(address, &mut operation)?;
        }
        runner.stop()?;
        Ok(())
    }
}

/// Details for the embedded-hal Transaction
/// implementation.
mod transaction {

    use super::{ControllerCommand, ControllerStatus, Direction, Lpi2c};
    use eh02::blocking::i2c::Operation;

    /// A stateful type that can run I2C operations.
    pub struct Runner<'a, I> {
        lpi2c: &'a mut I,
        direction: Option<Direction>,
    }

    impl<'a, P, const N: u8> Runner<'a, Lpi2c<P, N>> {
        /// Create a new transaction runner.
        ///
        /// Returns an error if the LPI2C is busy.
        pub fn new(lpi2c: &'a mut Lpi2c<P, N>) -> Result<Self, ControllerStatus> {
            lpi2c.check_busy()?;
            lpi2c.clear_fifo();
            lpi2c.clear_controller_status(ControllerStatus::W1C);
            Ok(Self {
                lpi2c,
                direction: None,
            })
        }

        /// Execute the next I2C operation.
        pub fn next_operation(
            &mut self,
            address: u8,
            operation: &mut Operation,
        ) -> Result<(), ControllerStatus> {
            // Quotes throughout indicate the embedded-hal Transactional requirements.
            match (&self.direction, &operation) {
                // "Data from adjacent operations of the same type are sent after each other without an SP or SR."
                (Some(Direction::Tx), Operation::Write(_)) => {}
                (Some(Direction::Rx), Operation::Read(_)) => {}
                // -----------------------------------------------
                // "Before executing the first operation an ST is sent automatically. This is followed by SAD+R/W as appropriate."
                // "Between adjacent operations of a different type an SR and SAD+R/W is sent."
                (Some(Direction::Rx) | None, Operation::Write(_)) => {
                    self.lpi2c.wait_for_transmit()?;
                    self.lpi2c
                        .enqueue_controller_command(ControllerCommand::write(address));
                }
                (Some(Direction::Tx) | None, Operation::Read(_)) => {
                    self.lpi2c.wait_for_transmit()?;
                    self.lpi2c
                        .enqueue_controller_command(ControllerCommand::read(address));
                }
            };

            match operation {
                Operation::Write(buffer) => {
                    for byte in *buffer {
                        self.lpi2c.wait_for_transmit()?;
                        self.lpi2c
                            .enqueue_controller_command(ControllerCommand::Transmit {
                                byte: *byte,
                            });
                    }
                    self.direction = Some(Direction::Tx);
                }
                // "If the last operation is a Read the peripheral does not send an acknowledge for the last byte."
                //
                // Handled by the hardware. See section 47.3.2.1 in the 1060 RM (rev2).
                Operation::Read(buffer) => {
                    if !buffer.is_empty() {
                        self.lpi2c.wait_for_transmit()?;
                        self.lpi2c
                            .enqueue_controller_command(ControllerCommand::Receive {
                                count: buffer.len() as u8,
                            });
                        for slot in buffer.iter_mut() {
                            *slot = self.lpi2c.wait_for_data()?;
                        }
                    }
                    self.direction = Some(Direction::Rx);
                }
            };

            Ok(())
        }

        /// Send a stop command, and finish the transaction.
        pub fn stop(self) -> Result<(), ControllerStatus> {
            self.lpi2c.wait_for_transmit()?;
            self.lpi2c
                .enqueue_controller_command(ControllerCommand::Stop);
            self.lpi2c.wait_for_end_of_packet()?;
            self.lpi2c.wait_for_controller_idle()?;
            Ok(())
        }
    }
}

/// Clock configuration fields.
///
/// These fields are written directly to the clock configuration register.
/// All values are written as-is to the register fields. Values that are
/// less than eight bits are truncated by the implementation. You're
/// responsible for making sure that these parameters meet their timing
/// parameter restrictions.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockConfiguration {
    /// Clock high period.
    ///
    /// Minimum number of cycles that the SCL clock is driven high.
    pub clkhi: u8,
    /// Clock low period.
    ///
    /// Minimum number of cycles that the SCL clock is driven low.
    pub clklo: u8,
    /// Setup hold delay.
    ///
    /// Minimum number of cycles that's used for
    /// - START condition hold
    /// - repeated START setup & hold
    /// - START condition setup
    pub sethold: u8,
    /// Data valid delay.
    ///
    /// Minimum number of cycles for SDA data hold. Must be less than
    /// the minimum SCL low period.
    pub datavd: u8,
    /// Glitch filter SDA.
    ///
    /// Only four bits large. Value of zero represents "no filter," and
    /// non-zero values represent filtered cycles.
    pub filtsda: u8,
    /// Glitch filter for SCL.
    ///
    /// Only four bits large. Value of zero represents "no filter," and
    /// non-zero values represent filtered cycles.
    pub filtscl: u8,
}

/// Source clock prescaler.
///
/// Affects all timing, except for the glitch filters.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Prescaler {
    /// Divide the source clock by 1.
    Prescaler1,
    /// Divide the source clock by 2.
    Prescaler2,
    /// Divide the source clock by 4.
    Prescaler4,
    /// Divide the source clock by 8.
    Prescaler8,
    /// Divide the source clock by 16.
    Prescaler16,
    /// Divide the source clock by 32.
    Prescaler32,
    /// Divide the source clock by 64.
    Prescaler64,
    /// Divide the source clock by 128.
    Prescaler128,
}

impl Prescaler {
    /// Returns the divider value for this prescaler.
    ///
    /// `Prescaler8` produces the value '8'.
    pub const fn divider(self) -> u8 {
        1 << self as u8
    }
}

const _: () = assert!(Prescaler::Prescaler1.divider() == 1);
const _: () = assert!(Prescaler::Prescaler128.divider() == 128);

/// Clock speed.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub enum ClockSpeed {
    /// 100 KHz.
    KHz100,
    /// 400 KHz.
    KHz400,
    /// 1 MHz.
    MHz1,
}

impl ClockSpeed {
    const fn frequency(self) -> u32 {
        match self {
            ClockSpeed::KHz100 => 100_000,
            ClockSpeed::KHz400 => 400_000,
            ClockSpeed::MHz1 => 1_000_000,
        }
    }
}

/// LPI2C timing parameters.
///
/// The implementation computes BUSIDLE based on the clock configuration values,
/// but you can override this after construction.
///
/// The simplest way to construct a `Timing` is to use [`ideal()`](Timing::ideal).
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timing {
    clock_configuration: ClockConfiguration,
    prescaler: Prescaler,
    busidle: u32,
}

/// Computes SCL and SDA latency cycles.
///
/// See table 47.3 in the reference manual. `risetime` is an estimate of the number
/// of clock cycles for the line to rise. Ideally, this is zero.
const fn line_latency_cycles(filter: u8, risetime: u8, prescaler: Prescaler) -> u8 {
    (2 + filter + risetime) / prescaler.divider()
}

/// Assuming that CLKHI = 2 * CLKLO = CLK, compute CLK. Saturate at u8::MAX.
///
/// BAUD = (HZ / (2 ^ PRESCALER)) / (3CLK + 2 + SCL_LATENCY)
///
/// Solve for CLK:
///
/// CLK = (HZ / (2 ^ PRESCALER) / BAUD - SCL_LATENCY - 2) / 3
///
/// Keep in mind that CLKHI and CLKLO are 6 bit fields, so the saturation value is still
/// out of range.
const fn compute_clk(hz: u32, baud: ClockSpeed, prescaler: Prescaler, scl_latency: u8) -> u8 {
    let clk: u32 =
        (hz / prescaler.divider() as u32 / baud.frequency() - scl_latency as u32 - 2) / 3;
    if clk > 0xFF {
        0xFFu8
    } else {
        clk as u8
    }
}

impl Timing {
    /// Compute timing parameters assuming an ideal I2C bus.
    ///
    /// This constructor assumes that
    ///
    /// - the SDA / SCL rise times are negligible (take less than one functional clock cycle).
    /// - there's no need for glitch filters (FLITSCL = FILTSDA = 0).
    ///
    /// These assumptions may not hold true for high clock speeds and I2C bus loadings.
    /// If that's the case, you may find it's better to define timing parameters yourself.
    ///
    /// Note that this function can run at compile time. Consider evaluating in a const
    /// context to avoid the possibility of panics.
    ///
    /// # Panics
    ///
    /// After evaluating all prescalars, this function panics if the computed clock period
    /// cannot be represented in the 6 bits available for the configuration.
    pub const fn ideal(clock_hz: u32, clock_speed: ClockSpeed) -> Self {
        const PRESCALERS: [Prescaler; 8] = [
            Prescaler::Prescaler1,
            Prescaler::Prescaler2,
            Prescaler::Prescaler4,
            Prescaler::Prescaler8,
            Prescaler::Prescaler16,
            Prescaler::Prescaler32,
            Prescaler::Prescaler64,
            Prescaler::Prescaler128,
        ];
        /// 6 bits available for all clock configurations.
        const CLOCK_PERIOD_MAX_VAL: u8 = 0x3Fu8;

        // Can't write a for loop in a const function...
        let mut clk = 0xFFu8;
        let mut idx = 0usize;
        while idx < PRESCALERS.len() {
            // Assuming no filters and rise times less than one clock cycle.
            let scl_latency = line_latency_cycles(0, 0, PRESCALERS[idx]);
            clk = compute_clk(clock_hz, clock_speed, PRESCALERS[idx], scl_latency);
            if clk.saturating_mul(2) <= CLOCK_PERIOD_MAX_VAL {
                break;
            }
            idx += 1;
        }

        assert!(
            clk.saturating_mul(2) <= CLOCK_PERIOD_MAX_VAL,
            "Could not compute CLKHI / CLKLO"
        );
        let prescaler = PRESCALERS[idx];

        let mut clkhi = clk;
        if clkhi < 0x01 {
            clkhi = 0x01;
        }

        let mut clklo = clk * 2;
        if clklo < 0x03 {
            clklo = 0x03;
        }

        // No need to assert CLKLO x (2 ^ PRESCALE) > SCL_LATENCY.
        // By SCL_LATENCY expansion,
        //
        //  CLKLO x (2 ^ PRESCALE) > (2 + FILTSCL + SCL_RISETIME) / (2 ^ PRESCALE)
        //
        // We use 0 for FILTSCL and assume a rise time less than 1 cycle (so, 0).
        // The inequality becomes
        //
        //  CLKLO > 2 / (2 ^ (2 * PRESCALE))
        //  CLKLO > 2 if PRESCALE = 0 (Prescaler1)
        //  CLKLO > 0 if PRESCALE > 0 (Prescaler2, Prescaler4, ...)
        //
        // So we're covered by the CLKLO >= 0x03 restriction.

        // Wait at least CLKHI cycles for (repeat) start / stop.
        let mut sethold = clkhi;
        if sethold < 0x02 {
            sethold = 0x02;
        }

        // Assume data valid after CLHI is high for half its cycles.
        let mut datavd = clkhi / 2;
        if datavd < 0x01 {
            datavd = 0x01;
        }

        Self::new(
            ClockConfiguration {
                clkhi,
                clklo,
                sethold,
                datavd,
                filtsda: 0,
                filtscl: 0,
            },
            prescaler,
        )
    }

    /// Computes timing parameters assuming an ideal circuit.
    ///
    ///
    /// Define LPI2C timings by the clock configuration values, and a prescaler.
    pub const fn new(clock_configuration: ClockConfiguration, prescaler: Prescaler) -> Self {
        const fn max(left: u32, right: u32) -> u32 {
            if left > right {
                left
            } else {
                right
            }
        }
        let busidle = max(
            (clock_configuration.clklo as u32 + clock_configuration.sethold as u32 + 2) * 2,
            clock_configuration.clkhi as u32 + 1,
        );
        Self {
            clock_configuration,
            prescaler,
            busidle,
        }
    }
    /// Returns the clock configuration.
    pub const fn clock_configuration(&self) -> ClockConfiguration {
        self.clock_configuration
    }
    /// Returns the prescaler.
    pub const fn prescaler(&self) -> Prescaler {
        self.prescaler
    }
    /// Override the BUSIDLE parameter.
    ///
    /// The minimum BUSIDLE is computed by CLKLO, SETHOLD, and CLKHI. Use
    /// this method to override the value.
    pub const fn override_busidle(mut self, busidle: u32) -> Self {
        self.busidle = busidle;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{ClockSpeed, Prescaler, Timing};

    #[test]
    fn timing_ideal() {
        let timings = Timing::ideal(8_000_000, ClockSpeed::KHz100);
        assert_eq!(timings.prescaler, Prescaler::Prescaler1);
        assert_eq!(timings.clock_configuration.clkhi, 25);
        assert_eq!(timings.clock_configuration.clklo, 50);
        assert_eq!(timings.clock_configuration.datavd, 12);
        assert_eq!(timings.clock_configuration.sethold, 25);
        assert_eq!(timings.clock_configuration.filtscl, 0);
        assert_eq!(timings.clock_configuration.filtsda, 0);

        let timings = Timing::ideal(8_000_000, ClockSpeed::KHz400);
        assert_eq!(timings.prescaler, Prescaler::Prescaler1);
        assert_eq!(timings.clock_configuration.clkhi, 5);
        assert_eq!(timings.clock_configuration.clklo, 10);
        assert_eq!(timings.clock_configuration.datavd, 2);
        assert_eq!(timings.clock_configuration.sethold, 5);
        assert_eq!(timings.clock_configuration.filtscl, 0);
        assert_eq!(timings.clock_configuration.filtsda, 0);

        let timings = Timing::ideal(8_000_000, ClockSpeed::MHz1);
        assert_eq!(timings.prescaler, Prescaler::Prescaler1);
        assert_eq!(timings.clock_configuration.clkhi, 1);
        assert_eq!(timings.clock_configuration.clklo, 3);
        assert_eq!(timings.clock_configuration.datavd, 1);
        assert_eq!(timings.clock_configuration.sethold, 2);
        assert_eq!(timings.clock_configuration.filtscl, 0);
        assert_eq!(timings.clock_configuration.filtsda, 0);
    }
}
