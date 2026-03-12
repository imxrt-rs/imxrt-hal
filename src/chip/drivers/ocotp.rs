//! On-Chip One-Time Programmable Controller
//!
//! The OCOTP driver lets you read and write fuses, from firmware.
//! The driver is available for most chip-specific builds,
//! and it maintains the same baseline API. However, only
//! 11xx MCUs may signal SEC and DED events.
//!
//! The driver does not manage any timing registers in the
//! OCOTP address space. You may need to do this yourself,
//! depending on your chip and clock settings.
//!
//! # Caution: Writes are Permanent
//!
//! *A fuse write is irreversible. The bits set high cannot be cleared. Furthermore,
//! if the fuse is backed by ECC redundancy or is locked by the write, you cannot
//! change nearby bits in the same fuse bank or word.*

use crate::ral::{self, ocotp};
use core::task::{self, Poll};

/// The magic number to perform a fuse write.
const WRITE_UNLOCK: u32 = 0x3E77;

/// Possible errors when accessing fuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Access to a locked region detected.
    LockedRegionAccess,
    /// Double error detected in fuse value during read.
    DedRead,
    /// Failed to program a fuse during a write.
    Programming,
}

/// The OCOTP driver.
pub struct Ocotp {
    pub(crate) ocotp: ocotp::OCOTP,
}

impl Ocotp {
    /// Construct the driver interface, given the peripheral instance.
    pub fn new(ocotp: ocotp::OCOTP) -> Self {
        Self { ocotp }
    }

    /// Check common status bits.
    fn check_status(&mut self) -> Poll<Result<(), Error>> {
        let (busy, error) = ral::read_reg!(ocotp, self.ocotp, CTRL, BUSY, ERROR);
        if error != 0 {
            ral::write_reg!(ocotp, self.ocotp, CTRL_SET, ERROR: 1);
            return Poll::Ready(Err(Error::LockedRegionAccess));
        }
        if busy != 0 {
            return Poll::Pending;
        }

        Poll::Ready(Ok(()))
    }

    /// Begin to read a fuse.
    ///
    /// Returns `Poll::Pending` if there's an in-flight OCOTP action already
    /// in progress. If this returns `Poll::Ready(Ok(()))`, then a read operation
    /// is in flight.
    ///
    /// If this returns an error, the error is a lagging error from a prior operation.
    /// The implementation will automatically clear that error.
    fn start_fuse_read(&mut self, fuse_address: FuseAddress) -> Poll<Result<(), Error>> {
        task::ready!(self.check_status())?;

        ral::modify_reg!(ocotp, self.ocotp, CTRL, ADDR: u32::from(fuse_address.0));
        ral::write_reg!(ocotp, self.ocotp, READ_CTRL, READ_FUSE: 1);
        Poll::Ready(Ok(()))
    }

    /// Begin to write data to a fuse.
    ///
    /// Returns `Poll::Pending` if there's an in-flight OCOTP action already in
    /// progress. If this returns `Poll::Ready(Ok(()))`, then the write operation
    /// is in flight.
    ///
    /// If this returns an error, the error is a lagging error from a prior operation.
    /// The implementation will automatically clear that error.
    fn start_fuse_write(
        &mut self,
        fuse_address: FuseAddress,
        fuse_value: u32,
    ) -> Poll<Result<(), Error>> {
        task::ready!(self.check_status())?;

        ral::modify_reg!(ocotp, self.ocotp, CTRL, ADDR: u32::from(fuse_address.0), WR_UNLOCK: WRITE_UNLOCK);
        ral::write_reg!(ocotp, self.ocotp, DATA, fuse_value);
        Poll::Ready(Ok(()))
    }

    /// Wait for a fuse read to complete.
    ///
    /// You should sequence this sometime after [`start_fuse_read`](Self::start_fuse_read).
    /// Returns `Poll::Pending` while the read is in progress. If the ECC-redundant fuse
    /// could not be corrected, the error is [`Error::DedRead`]. The implementation will
    /// automatically clear the DED error.
    fn poll_fuse_read(&mut self) -> Poll<Result<u32, Error>> {
        task::ready!(self.check_status())?;

        // We're polling for errors above.
        //
        // Not checking against sentinel 0xBADABADA. What
        // if someone actually wanted to write that fuse
        // value?
        let fuse_data = self.read_fuse_data();
        self.check_end_fuse_read()?;

        Poll::Ready(Ok(fuse_data))
    }

    /// Wait for the write to complete.
    ///
    /// You should sequence this sometime after [`start_fuse_write`](Self::start_fuse_write).
    /// Returns `Poll::Pending` while the write is in progress.
    fn poll_fuse_write(&mut self) -> Poll<Result<(), Error>> {
        task::ready!(self.check_status())?;
        self.check_end_fuse_write()?;

        Poll::Ready(Ok(()))
    }

    /// Read a fuse, blocking indefinitely until it returns a value.
    ///
    /// This simply spins on the future returned by [`spin_fuse_read`](Self::spin_fuse_read).
    /// The approach does not give you a way to specify any timeout. Use only
    /// when you're confident the fuse read will complete.
    pub fn blocking_fuse_read(&mut self, fuse_address: FuseAddress) -> Result<u32, Error> {
        crate::spin_on(self.spin_fuse_read(fuse_address))
    }

    /// Write a fuse, blocking indefinitely until it completes.
    ///
    /// This simply spins on the future returned by [`spin_fuse_write`](Self::spin_fuse_write).
    /// The approach does not give you a way to control a timeout. Use only when
    /// you're confident the write will complete.
    pub fn blocking_fuse_write(
        &mut self,
        fuse_address: FuseAddress,
        fuse_value: u32,
    ) -> Result<(), Error> {
        crate::spin_on(self.spin_fuse_write(fuse_address, fuse_value))
    }

    /// Returns a spinning state machine that reads a fuse value.
    ///
    /// The returned task does not consult a waker! You're expected
    /// to spin the CPU on `poll()` until it returns a value.
    ///
    /// The future is not cancel safe. There is no way to cancel an
    /// in-flight fuse read. Therefore, errors from _prior, incomplete_
    /// reads may be returned in a _subsequent_ access.
    ///
    /// You may compose your own timeouts by using another spinning
    /// task, and selecting the winner of the timeout / read.
    pub async fn spin_fuse_read(&mut self, fuse_address: FuseAddress) -> Result<u32, Error> {
        core::future::poll_fn(|_| self.start_fuse_read(fuse_address)).await?;
        let fuse_data = core::future::poll_fn(|_| self.poll_fuse_read()).await?;
        Ok(fuse_data)
    }

    /// Returns a spinning state machine that writes a fuse value.
    ///
    /// The returned task does not consult a waker! You're expected
    /// to spin the CPU on `poll()` until it returns a value.
    ///
    /// The future is not cancel safe. There is no way to cancel an
    /// in-flight fuse write. Therefore, errors from _prior, incomplete_
    /// writes may be returned in a _subsequent_ access.
    ///
    /// You may compose your own timeouts by using another spinning
    /// task, and selecting the winner of the timeout / write.
    pub async fn spin_fuse_write(
        &mut self,
        fuse_address: FuseAddress,
        fuse_value: u32,
    ) -> Result<(), Error> {
        core::future::poll_fn(|_| self.start_fuse_write(fuse_address, fuse_value)).await?;
        core::future::poll_fn(|_| self.poll_fuse_write()).await?;
        Ok(())
    }
}

/// A fuse address.
///
/// Consult your MCU's fuse map for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FuseAddress(u16);

impl FuseAddress {
    /// Construct a fuse address.
    ///
    /// `addr` is the fuse you intend to read or write.
    /// The implementation converts this for access. If
    /// the conversion fails, this returns `None`.
    pub const fn new(addr: u16) -> Option<Self> {
        let Some(addr) = addr.checked_sub(Ocotp::FUSE_ADDRESS_OFFSET) else {
            return None;
        };
        if addr % 16 != 0 {
            return None;
        }
        Some(Self(addr / 16))
    }

    /// Returns the raw fuse address.
    pub const fn raw(&self) -> u16 {
        self.0
    }

    /// Returns the fuse address supplied during construction.
    const fn get(&self) -> u16 {
        self.0 * 16 + Ocotp::FUSE_ADDRESS_OFFSET
    }
}

impl core::fmt::Display for FuseAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.get())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for FuseAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{:#X}", self.get())
    }
}
