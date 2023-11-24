use super::LpspiError;

use eh1::spi::{Error, ErrorKind};

impl Error for LpspiError {
    fn kind(&self) -> ErrorKind {
        match self {
            LpspiError::Busy => ErrorKind::Other,
            LpspiError::ReceiveFifo => ErrorKind::Overrun,
            LpspiError::TransmitFifo => ErrorKind::Other,
            LpspiError::NoData => ErrorKind::Other,
        }
    }
}
