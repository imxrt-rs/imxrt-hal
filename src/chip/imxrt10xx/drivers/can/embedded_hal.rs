//! `embedded_hal` trait impls.

use super::{Data, Error, Frame, CAN};

use embedded_can;
pub(crate) use embedded_can::ErrorKind;
pub use embedded_can::{ExtendedId, Id, StandardId};

impl<P, const M: u8> embedded_can::nb::Can for CAN<P, M> {
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

impl embedded_can::Error for Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        match self {
            Self::NoRxData => embedded_can::ErrorKind::Other,
            Self::NoTxMailbox => embedded_can::ErrorKind::Other,
            Self::EmbeddedHal(e) => e.kind(),
        }
    }
}

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        let id = match id.into() {
            Id::Standard(id) => unsafe { Id::Standard(StandardId::new_unchecked(id.as_raw())) },
            Id::Extended(id) => unsafe { Id::Extended(ExtendedId::new_unchecked(id.as_raw())) },
        };

        let data = Data::new(data)?;
        Some(Frame::new_data(id, data))
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        let id = match id.into() {
            Id::Standard(id) => unsafe { Id::Standard(StandardId::new_unchecked(id.as_raw())) },
            Id::Extended(id) => unsafe { Id::Extended(ExtendedId::new_unchecked(id.as_raw())) },
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
    fn id(&self) -> Id {
        match self.id() {
            Id::Standard(id) => unsafe { Id::Standard(StandardId::new_unchecked(id.as_raw())) },
            Id::Extended(id) => unsafe { Id::Extended(ExtendedId::new_unchecked(id.as_raw())) },
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
