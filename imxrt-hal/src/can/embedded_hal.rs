//! `embedded_hal` trait impls.

use super::{Data, Frame, Error, CAN};
use crate::iomuxc::consts::Unsigned;

use embedded_hal::can;
pub use embedded_hal::can::{ExtendedId, Id, StandardId};
pub(crate) use embedded_hal::can::ErrorKind;

impl<M> can::Can for CAN<M>
where
    M: Unsigned,
{
    type Frame = Frame;

    type Error = Error;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        match self.transmit(frame) {
            Ok(_status) => Ok(Some(frame.clone())),
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
        }
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        match self.read_mailboxes() {
            Some(d) => Ok(d.frame),
            None => Err(nb::Error::Other(Error::NoRxData)),
        }
    }
}

impl can::Error for Error {
    fn kind(&self) -> can::ErrorKind {
        match self {
            Self::NoRxData => can::ErrorKind::Other,
            Self::NoTxMailbox => can::ErrorKind::Other,
            Self::EmbeddedHal(e) => e.kind()
        }
    }
}

impl can::Frame for Frame {
    fn new(id: impl Into<can::Id>, data: &[u8]) -> Option<Self> {
        let id = match id.into() {
            can::Id::Standard(id) => unsafe {
                Id::Standard(StandardId::new_unchecked(id.as_raw()))
            },
            can::Id::Extended(id) => unsafe {
                Id::Extended(ExtendedId::new_unchecked(id.as_raw()))
            },
        };

        let data = Data::new(data)?;
        Some(Frame::new_data(id, data))
    }

    fn new_remote(id: impl Into<can::Id>, dlc: usize) -> Option<Self> {
        let id = match id.into() {
            can::Id::Standard(id) => unsafe {
                Id::Standard(StandardId::new_unchecked(id.as_raw()))
            },
            can::Id::Extended(id) => unsafe {
                Id::Extended(ExtendedId::new_unchecked(id.as_raw()))
            },
        };

        if dlc <= 8 {
            Some(Frame::new_remote(id, dlc as u8))
        } else {
            None
        }
    }

    #[inline]
    fn is_extended(&self) -> bool {
        self.is_extended()
    }

    #[inline]
    fn is_remote_frame(&self) -> bool {
        self.is_remote_frame()
    }

    #[inline]
    fn id(&self) -> can::Id {
        match self.id() {
            Id::Standard(id) => unsafe {
                can::Id::Standard(can::StandardId::new_unchecked(id.as_raw()))
            },
            Id::Extended(id) => unsafe {
                can::Id::Extended(can::ExtendedId::new_unchecked(id.as_raw()))
            },
        }
    }

    #[inline]
    fn dlc(&self) -> usize {
        self.dlc().into()
    }

    fn data(&self) -> &[u8] {
        if let Some(data) = self.data() {
            data
        } else {
            &[]
        }
    }
}
