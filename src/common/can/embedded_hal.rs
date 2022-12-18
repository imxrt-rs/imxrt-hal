//! `embedded_hal` trait impls.

use super::{Data, ExtendedId, Frame, Id, NoDataError, StandardId, CAN};

use crate::iomuxc::consts::Unsigned;
use ecan;

impl<const M: u8> ecan::nb::Can for CAN<M>
{
    type Frame = Frame;

    type Error = NoDataError;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        match self.transmit(frame) {
            Ok(_status) => Ok(Some(frame.clone())),
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => match e {},
        }
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        match self.read_mailboxes() {
            Some(d) => Ok(d.frame),
            None => Err(nb::Error::Other(NoDataError { _priv: () })),
        }
    }
}

impl ecan::Error for NoDataError {
    fn kind(&self) -> ecan::ErrorKind {
        ecan::ErrorKind::Overrun
    }
}

impl ecan::Frame for Frame {
    fn new(id: impl Into<ecan::Id>, data: &[u8]) -> Option<Self> {
        let id = match id.into() {
            ecan::Id::Standard(id) => unsafe {
                Id::Standard(StandardId::new_unchecked(id.as_raw()))
            },
            ecan::Id::Extended(id) => unsafe {
                Id::Extended(ExtendedId::new_unchecked(id.as_raw()))
            },
        };

        let data = Data::new(data)?;
        Some(Frame::new_data(id, data))
    }

    fn new_remote(id: impl Into<ecan::Id>, dlc: usize) -> Option<Self> {
        let id = match id.into() {
            ecan::Id::Standard(id) => unsafe {
                Id::Standard(StandardId::new_unchecked(id.as_raw()))
            },
            ecan::Id::Extended(id) => unsafe {
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
    fn id(&self) -> ecan::Id {
        match self.id() {
            Id::Standard(id) => unsafe {
                ecan::Id::Standard(ecan::StandardId::new_unchecked(id.as_raw()))
            },
            Id::Extended(id) => unsafe {
                ecan::Id::Extended(ecan::ExtendedId::new_unchecked(id.as_raw()))
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
