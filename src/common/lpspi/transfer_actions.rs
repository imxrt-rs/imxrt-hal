use core::marker::PhantomData;

pub(crate) struct DualDirectionActions {
    read_buf: *mut u8,
    write_buf: *const u8,
    len: [usize; 3],
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

/// The order in which the bytes need
/// to be transferred on the bus
pub(crate) enum ByteOrder {
    /// Bytes need to be transferred in the order
    /// that they are in
    Normal,
    /// Every group of 4 bytes needs to be reversed
    WordReversed,
    /// Every group of 2 bytes needs to be reversed
    HalfWordReversed,
}

pub(crate) enum TransferDirection {
    Read,
    Write,
}

pub(crate) struct ActionSequence<'a> {
    phase1: Option<DualDirectionActions>,
    phase2: Option<SingleDirectionActions>,
    byteorder: ByteOrder,
    _lifetimes: PhantomData<&'a [u8]>,
}

pub(crate) trait BufferType: Copy + 'static {
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
