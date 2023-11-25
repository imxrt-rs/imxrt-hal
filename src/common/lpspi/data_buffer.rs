pub enum TransferBuffer<'a, T> {
    Single(&'a mut [T]),
    Dual(&'a mut [T], &'a [T]),
}

impl<T> TransferBuffer<'_, T>
where
    [T]: LpspiDataBuffer,
{
    #[inline]
    pub fn tx_buffer(&self) -> &[T] {
        match self {
            TransferBuffer::Single(x) => x,
            TransferBuffer::Dual(_, x) => x,
        }
    }

    #[inline]
    pub fn rx_buffer(&mut self) -> &mut [T] {
        match self {
            TransferBuffer::Single(x) => x,
            TransferBuffer::Dual(x, _) => x,
        }
    }

    pub fn max_len(&self) -> usize {
        match self {
            TransferBuffer::Single(x) => x.len(),
            TransferBuffer::Dual(x1, x2) => x1.len().max(x2.len()),
        }
    }

    pub fn dma_align(&mut self) -> (TransferBuffer<u8>, TransferBuffer<u32>, TransferBuffer<u8>) {
        match self {
            TransferBuffer::Single(x) => {
                let (a, b, c) = x.dma_align_mut();
                (
                    TransferBuffer::Single(a),
                    TransferBuffer::Single(b),
                    TransferBuffer::Single(c),
                )
            }
            TransferBuffer::Dual(x1, x2) => {
                let (a1, b1, c1) = x1.dma_align_mut();
                let (a2, b2, c2) = x2.dma_align();
                (
                    TransferBuffer::Dual(a1, a2),
                    TransferBuffer::Dual(b1, b2),
                    TransferBuffer::Dual(c1, c2),
                )
            }
        }
    }
}

/// A data type that can be used for LPSPI transfers
pub trait LpspiDataBuffer {
    /// Splits the buffer into DMA and non-DMA parts.
    fn dma_align_mut(&mut self) -> (&mut [u8], &mut [u32], &mut [u8]);

    /// Splits the buffer into DMA and non-DMA parts.
    fn dma_align(&self) -> (&[u8], &[u32], &[u8]);
}

impl LpspiDataBuffer for [u8] {
    fn dma_align_mut(&mut self) -> (&mut [u8], &mut [u32], &mut [u8]) {
        unsafe { self.align_to_mut() }
    }

    fn dma_align(&self) -> (&[u8], &[u32], &[u8]) {
        unsafe { self.align_to() }
    }
}

impl LpspiDataBuffer for [u16] {
    fn dma_align_mut(&mut self) -> (&mut [u8], &mut [u32], &mut [u8]) {
        unsafe {
            let data: &mut [u8] = core::slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut u8,
                self.len() * core::mem::size_of::<u16>(),
            );
            data.align_to_mut()
        }
    }

    fn dma_align(&self) -> (&[u8], &[u32], &[u8]) {
        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * core::mem::size_of::<u16>(),
            );
            data.align_to()
        }
    }
}

impl LpspiDataBuffer for [u32] {
    fn dma_align_mut(&mut self) -> (&mut [u8], &mut [u32], &mut [u8]) {
        (&mut [], self, &mut [])
    }

    fn dma_align(&self) -> (&[u8], &[u32], &[u8]) {
        (&mut [], self, &mut [])
    }
}
