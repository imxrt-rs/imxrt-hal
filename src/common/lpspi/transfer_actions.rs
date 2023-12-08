use core::{marker::PhantomData, num::NonZeroUsize};

pub(crate) struct DualDirectionActions {
    read_buf: *mut u8,
    write_buf: *const u8,
    len: [usize; 3],
}

impl DualDirectionActions {
    pub(crate) unsafe fn get_write_actions(&self) -> WriteActionIter {
        WriteActionIter::new(WriteActions {
            write_buf: self.write_buf,
            len: self.len,
        })
    }
}

pub(crate) struct WriteAction {
    pub(crate) buf: *const u8,
    pub(crate) len: NonZeroUsize,
    pub(crate) is_first: bool,
    pub(crate) is_last: bool,
}

pub(crate) struct WriteActionIter {
    actions: WriteActions,
    pos: usize,
}
impl WriteActionIter {
    fn new(actions: WriteActions) -> Self {
        Self { actions, pos: 0 }
    }
}
impl Iterator for WriteActionIter {
    type Item = WriteAction;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.pos == 0;

        let mut lengths = self.actions.len.get(self.pos..)?;
        let len = loop {
            if let Some(len) = NonZeroUsize::new(*lengths.first()?) {
                break len;
            }
            self.pos += 1;
            lengths = self.actions.len.get(self.pos..)?;
        };

        let buf = self.actions.write_buf;

        self.pos += 1;
        self.actions.write_buf = unsafe { self.actions.write_buf.add(len.get()) };

        let is_last = self
            .actions
            .len
            .get(self.pos..)
            .into_iter()
            .flatten()
            .all(|&val| val == 0);

        Some(WriteAction {
            buf,
            len,
            is_first,
            is_last,
        })
    }
}

pub(crate) struct ReadActions {
    read_buf: *mut u8,
    len: [usize; 3],
}

pub(crate) struct WriteActions {
    write_buf: *const u8,
    len: [usize; 3],
}

pub(crate) enum SingleDirectionActions {
    Read(ReadActions),
    Write(WriteActions),
}

impl SingleDirectionActions {
    pub(crate) fn transfer_direction(&self) -> TransferDirection {
        match self {
            SingleDirectionActions::Read(_) => TransferDirection::Read,
            SingleDirectionActions::Write(_) => TransferDirection::Write,
        }
    }
}

/// The order in which the bytes need
/// to be transferred on the bus
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ByteOrder {
    /// Bytes need to be transferred in the order
    /// that they are in
    Normal,
    /// Every group of 4 bytes needs to be reversed
    WordReversed,
    /// Every group of 2 bytes needs to be reversed
    HalfWordReversed,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum TransferDirection {
    Read,
    Write,
}

pub(crate) struct ActionSequence<'a> {
    pub(crate) phase1: Option<DualDirectionActions>,
    pub(crate) phase2: Option<SingleDirectionActions>,
    pub(crate) byteorder: ByteOrder,
    _lifetimes: PhantomData<&'a [u8]>,
}

impl ActionSequence<'_> {
    pub(crate) fn contains_read_actions(&self) -> bool {
        if self.phase1.is_some() {
            true
        } else if let Some(phase2) = &self.phase2 {
            phase2.transfer_direction() == TransferDirection::Read
        } else {
            false
        }
    }
}

pub trait BufferType: Copy + 'static {
    fn byte_order() -> ByteOrder;
}

impl BufferType for u8 {
    fn byte_order() -> ByteOrder {
        ByteOrder::Normal
    }
}
impl BufferType for u16 {
    fn byte_order() -> ByteOrder {
        ByteOrder::HalfWordReversed
    }
}
impl BufferType for u32 {
    fn byte_order() -> ByteOrder {
        ByteOrder::WordReversed
    }
}

fn determine_dma_alignment<T>(dat: &[T]) -> [usize; 3] {
    let (a, b, c) = unsafe { dat.align_to::<u32>() };
    [
        a.len() * core::mem::size_of::<T>(),
        b.len() * core::mem::size_of::<u32>(),
        c.len() * core::mem::size_of::<T>(),
    ]
}

pub(crate) fn create_actions_read_write<'a, T: BufferType>(
    read: &'a mut [T],
    write: &'a [T],
) -> ActionSequence<'a> {
    let phase1;
    let phase2;

    if read.len() > write.len() {
        let (read1, read2) = read.split_at_mut(write.len());

        phase1 = (!read1.is_empty()).then(|| DualDirectionActions {
            read_buf: read1.as_mut_ptr().cast(),
            write_buf: write.as_ptr().cast(),
            len: determine_dma_alignment(read1),
        });

        phase2 = (!read2.is_empty()).then(|| {
            SingleDirectionActions::Read(ReadActions {
                read_buf: read2.as_mut_ptr().cast(),
                len: determine_dma_alignment(read2),
            })
        });
    } else {
        let (write1, write2) = write.split_at(read.len());

        phase1 = (!write1.is_empty()).then(|| DualDirectionActions {
            read_buf: read.as_mut_ptr().cast(),
            write_buf: write1.as_ptr().cast(),
            len: determine_dma_alignment(write1),
        });

        phase2 = (!write2.is_empty()).then(|| {
            SingleDirectionActions::Write(WriteActions {
                write_buf: write2.as_ptr().cast(),
                len: determine_dma_alignment(write2),
            })
        });
    }

    ActionSequence {
        phase1,
        phase2,
        byteorder: T::byte_order(),
        _lifetimes: PhantomData,
    }
}

pub(crate) fn create_actions_read_write_in_place<'a, T: BufferType>(
    buf: &'a mut [T],
) -> ActionSequence<'a> {
    ActionSequence {
        phase1: Some(DualDirectionActions {
            read_buf: buf.as_mut_ptr().cast(),
            write_buf: buf.as_ptr().cast(),
            len: determine_dma_alignment(buf),
        }),
        phase2: None,
        byteorder: T::byte_order(),
        _lifetimes: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::vec::Vec;

    macro_rules! actions_write_iter_test {
        ($len:expr, $expected:expr) => {{
            let actual = WriteActionIter::new(WriteActions {
                write_buf: 1000usize as *const u8,
                len: $len,
            })
            let expected =
            .collect::<Vec<_>>();
        }};
    }

    #[test]
    fn actions_write_iter() {
        actions_write_iter_test([0, 5, 0], [(0, 5, true, true)]);
    }
}
