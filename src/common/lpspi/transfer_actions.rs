use core::{marker::PhantomData, num::NonZeroUsize};

#[derive(Debug)]
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

    pub(crate) unsafe fn get_read_actions(&self) -> ReadActionIter {
        ReadActionIter::new(ReadActions {
            read_buf: self.read_buf,
            len: self.len,
        })
    }
}

pub(crate) struct WriteAction {
    pub(crate) buf: Option<*const u8>,
    pub(crate) read: bool,
    pub(crate) len: NonZeroUsize,
    pub(crate) is_first: bool,
    pub(crate) is_last: bool,
}

pub(crate) struct ReadAction {
    pub(crate) buf: *const u8,
    pub(crate) len: NonZeroUsize,
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
            buf: Some(buf),
            read: true,
            len,
            is_first,
            is_last,
        })
    }
}

pub(crate) struct ReadActionIter {
    actions: ReadActions,
    pos: usize,
}
impl ReadActionIter {
    fn new(actions: ReadActions) -> Self {
        Self { actions, pos: 0 }
    }
}

impl Iterator for ReadActionIter {
    type Item = ReadAction;

    fn next(&mut self) -> Option<Self::Item> {
        let len = loop {
            let mut lengths = self.actions.len.get(self.pos..)?;
            if let Some(len) = NonZeroUsize::new(*lengths.first()?) {
                break len;
            }
            self.pos += 1;
            lengths = self.actions.len.get(self.pos..)?;
        };

        let buf = self.actions.read_buf;

        self.pos += 1;
        self.actions.read_buf = unsafe { self.actions.read_buf.add(len.get()) };

        Some(ReadAction { buf, len })
    }
}

pub(crate) struct SingleDirectionWriteActionIter {
    actions: MaybeWriteActions,
    pos: usize,
}
impl SingleDirectionWriteActionIter {
    fn new(actions: MaybeWriteActions) -> Self {
        Self { actions, pos: 0 }
    }
}
impl Iterator for SingleDirectionWriteActionIter {
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
        self.actions.write_buf = unsafe { self.actions.write_buf.map(|b| b.add(len.get())) };

        let is_last = self
            .actions
            .len
            .get(self.pos..)
            .into_iter()
            .flatten()
            .all(|&val| val == 0);

        Some(WriteAction {
            buf,
            read: buf.is_none(),
            len,
            is_first,
            is_last,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ReadActions {
    read_buf: *mut u8,
    len: [usize; 3],
}

#[derive(Debug)]
pub(crate) struct WriteActions {
    write_buf: *const u8,
    len: [usize; 3],
}

#[derive(Debug)]
pub(crate) struct MaybeWriteActions {
    write_buf: Option<*const u8>,
    len: [usize; 3],
}

#[derive(Debug)]
pub(crate) enum SingleDirectionActions {
    Read(ReadActions),
    Write(WriteActions),
}

impl SingleDirectionActions {
    pub(crate) fn transfer_direction(&self) -> TransferDirection {
        match self {
            Self::Read(_) => TransferDirection::Read,
            Self::Write(_) => TransferDirection::Write,
        }
    }

    pub(crate) unsafe fn get_write_actions(&self) -> SingleDirectionWriteActionIter {
        SingleDirectionWriteActionIter::new(match self {
            Self::Read(actions) => MaybeWriteActions {
                write_buf: None,
                len: actions.len,
            },
            Self::Write(actions) => MaybeWriteActions {
                write_buf: Some(actions.write_buf),
                len: actions.len,
            },
        })
    }

    pub(crate) unsafe fn get_read_actions(&self) -> Option<ReadActionIter> {
        match self {
            Self::Read(actions) => Some(ReadActionIter::new(ReadActions {
                read_buf: actions.read_buf,
                len: actions.len,
            })),
            Self::Write(_) => None,
        }
    }
}

/// The order in which the bytes need
/// to be transferred on the bus
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    /// Bytes need to be transferred in the order
    /// that they are in
    Normal,
    /// Every group of 4 bytes needs to be reversed
    WordReversed,
    /// Every group of 2 bytes needs to be reversed
    HalfWordReversed,
}

impl ByteOrder {
    pub(crate) fn reorder(self, val: u32) -> u32 {
        match self {
            ByteOrder::Normal => val,
            ByteOrder::WordReversed => val,
            ByteOrder::HalfWordReversed => {
                let [a, b, c, d] = val.to_le_bytes();
                u32::from_le_bytes([b, a, d, c])
            }
        }
    }

    pub(crate) fn requires_reorder(self) -> bool {
        match self {
            ByteOrder::Normal => false,
            ByteOrder::WordReversed => false,
            ByteOrder::HalfWordReversed => true,
        }
    }

    pub(crate) fn requires_flip(self) -> bool {
        match self {
            ByteOrder::Normal => false,
            ByteOrder::WordReversed => true,
            ByteOrder::HalfWordReversed => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TransferDirection {
    Read,
    Write,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct ChunkIterChunk {
    pub(crate) first: bool,
    pub(crate) last: bool,
    pub(crate) position: usize,
    pub(crate) size: NonZeroUsize,
}

pub(crate) struct ChunkIter {
    size: NonZeroUsize,
    position: usize,
    max_chunk_size: usize,
}
impl ChunkIter {
    pub(crate) fn new(size: NonZeroUsize, max_chunk_size: usize) -> Self {
        Self {
            size,
            position: 0,
            max_chunk_size,
        }
    }
}

impl Iterator for ChunkIter {
    type Item = ChunkIterChunk;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.position == 0;

        let position = self.position;
        let leftover = self.size.get().checked_sub(self.position)?;
        let next_chunk_size = leftover.min(self.max_chunk_size);

        self.position += next_chunk_size;
        let last = self.position >= self.size.get();

        Some(ChunkIterChunk {
            first,
            last,
            position,
            size: NonZeroUsize::new(next_chunk_size)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::vec::Vec;

    // TODO: tests for
    // - ChunkIter
    // - Byteorder conversion functions

    macro_rules! actions_read_iter_test {
        ($len:expr, $expected:expr) => {{
            let actual = ReadActionIter::new(ReadActions {
                read_buf: 1000usize as *mut u8,
                len: $len,
            })
            .map(|val| {
                (
                    val.buf.unwrap() as usize - 1000,
                    val.len.get(),
                    val.is_first,
                    val.is_last,
                )
            })
            .collect::<Vec<_>>();
            let expected: &[(usize, usize, bool, bool)] = &$expected;

            assert_eq!(actual, expected);
        }};
    }
    macro_rules! actions_write_iter_test {
        ($len:expr, $expected:expr) => {{
            let actual = WriteActionIter::new(WriteActions {
                write_buf: 1000usize as *const u8,
                len: $len,
            })
            .map(|val| {
                (
                    val.buf.unwrap() as usize - 1000,
                    val.len.get(),
                    val.is_first,
                    val.is_last,
                )
            })
            .collect::<Vec<_>>();
            let expected: &[(usize, usize, bool, bool)] = &$expected;

            assert_eq!(actual, expected);
        }};
    }
    macro_rules! actions_single_direction_write_iter_test {
        ($write:expr, $len:expr, $expected:expr) => {{
            let actual = SingleDirectionWriteActionIter::new(MaybeWriteActions {
                write_buf: if $write {
                    Some(1000usize as *const u8)
                } else {
                    None
                },
                len: $len,
            })
            .map(|val| {
                (
                    val.buf.map(|b| b as usize - 1000),
                    val.len.get(),
                    val.is_first,
                    val.is_last,
                )
            })
            .collect::<Vec<_>>();
            let expected: &[(Option<usize>, usize, bool, bool)] = &$expected;

            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn read_actions_iter() {
        actions_read_iter_test!([0, 5, 0], [(0, 5)]);
        actions_read_iter_test!([2, 3, 4], [(0, 2), (2, 3), (5, 4),]);
        actions_read_iter_test!([2, 0, 4], [(0, 2), (2, 4)]);
        actions_read_iter_test!([2, 0, 0], [(0, 2)]);
        actions_read_iter_test!([0, 0, 4], [(0, 4)]);
    }

    #[test]
    fn write_actions_iter() {
        actions_write_iter_test!([0, 5, 0], [(0, 5, true, true)]);
        actions_write_iter_test!(
            [2, 3, 4],
            [
                (0, 2, true, false),
                (2, 3, false, false),
                (5, 4, false, true),
            ]
        );
        actions_write_iter_test!([2, 0, 4], [(0, 2, true, false), (2, 4, false, true)]);
        actions_write_iter_test!([2, 0, 0], [(0, 2, true, true)]);
        actions_write_iter_test!([0, 0, 4], [(0, 4, true, true)]);
    }

    #[test]
    fn single_direction_write_actions_iter_write() {
        actions_single_direction_write_iter_test!(true, [0, 5, 0], [(Some(0), 5, true, true)]);
        actions_single_direction_write_iter_test!(
            true,
            [2, 3, 4],
            [
                (Some(0), 2, true, false),
                (Some(2), 3, false, false),
                (Some(5), 4, false, true),
            ]
        );
        actions_single_direction_write_iter_test!(
            true,
            [2, 0, 4],
            [(Some(0), 2, true, false), (Some(2), 4, false, true)]
        );
        actions_single_direction_write_iter_test!(true, [2, 0, 0], [(Some(0), 2, true, true)]);
        actions_single_direction_write_iter_test!(true, [0, 0, 4], [(Some(0), 4, true, true)]);
    }

    #[test]
    fn single_direction_write_actions_iter_read() {
        actions_single_direction_write_iter_test!(false, [0, 5, 0], [(None, 5, true, true)]);
        actions_single_direction_write_iter_test!(
            false,
            [2, 3, 4],
            [
                (None, 2, true, false),
                (None, 3, false, false),
                (None, 4, false, true),
            ]
        );
        actions_single_direction_write_iter_test!(
            false,
            [2, 0, 4],
            [(None, 2, true, false), (None, 4, false, true)]
        );
        actions_single_direction_write_iter_test!(false, [2, 0, 0], [(None, 2, true, true)]);
        actions_single_direction_write_iter_test!(false, [0, 0, 4], [(None, 4, true, true)]);
    }
}
