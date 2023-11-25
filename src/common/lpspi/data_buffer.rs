use core::ops::Range;

/// A data type that can be used for LPSPI transfers
pub trait LpspiDataBuffer {
    /// Length in bytes
    fn bytecount(&self) -> usize;

    /// Retreive a chunk to transmit.
    /// Filled with zeros when overflown.
    ///
    /// # Arguments
    ///
    /// * `pos` - The `[u32]`-offset from where to read.
    ///
    fn read(&self, pos: usize) -> u32;

    /// Store a next received chunk.
    ///
    /// # Arguments
    ///
    /// * `pos` - The `[u32]`-offset where to write.
    /// * `val` - The data to write
    ///
    fn write(&mut self, pos: usize, val: u32);
}

impl LpspiDataBuffer for [u8] {
    fn bytecount(&self) -> usize {
        self.len()
    }

    fn read(&self, pos: usize) -> u32 {
        let pos = 4 * pos;

        u32::from_be_bytes([
            self.get(pos + 0).copied().unwrap_or_default(),
            self.get(pos + 1).copied().unwrap_or_default(),
            self.get(pos + 2).copied().unwrap_or_default(),
            self.get(pos + 3).copied().unwrap_or_default(),
        ])
    }

    fn write(&mut self, pos: usize, val: u32) {
        let pos = 4 * pos;

        for (offset, val) in val.to_be_bytes().into_iter().enumerate() {
            if let Some(x) = self.get_mut(pos + offset) {
                *x = val;
            }
        }
    }
}

impl LpspiDataBuffer for [u16] {
    fn bytecount(&self) -> usize {
        self.len() * 2
    }

    fn read(&self, pos: usize) -> u32 {
        let pos = 2 * pos;

        let [b0, b1] = self.get(pos + 0).copied().unwrap_or_default().to_be_bytes();
        let [b2, b3] = self.get(pos + 1).copied().unwrap_or_default().to_be_bytes();

        u32::from_be_bytes([b0, b1, b2, b3])
    }

    fn write(&mut self, pos: usize, val: u32) {
        let pos = 2 * pos;

        let [b0, b1, b2, b3] = val.to_be_bytes();

        if let Some(x) = self.get_mut(pos + 0) {
            *x = u16::from_be_bytes([b0, b1]);
        }
        if let Some(x) = self.get_mut(pos + 1) {
            *x = u16::from_be_bytes([b2, b3]);
        }
    }
}

impl LpspiDataBuffer for [u32] {
    fn bytecount(&self) -> usize {
        self.len() * 4
    }

    fn read(&self, pos: usize) -> u32 {
        self.get(pos).copied().unwrap_or_default()
    }

    fn write(&mut self, pos: usize, val: u32) {
        if let Some(x) = self.get_mut(pos) {
            *x = val;
        }
    }
}

pub struct LpspiIndexChunks {
    bytecount: usize,
    offset: usize,
    chunk_size: u32,
}

impl LpspiIndexChunks {
    pub fn new(bytecount: usize, chunk_size: u32) -> Self {
        let chunk_size = (chunk_size / 4) * 4; // Round down to next divisible by 4
        Self {
            bytecount,
            offset: 0,
            chunk_size,
        }
    }
}

impl Iterator for LpspiIndexChunks {
    type Item = LpspiIndexChunk;

    fn next(&mut self) -> Option<Self::Item> {
        let offset_bytes = self.offset * 4;
        if offset_bytes >= self.bytecount {
            return None;
        }

        let leftover_bytes = self.bytecount - offset_bytes;
        let chunk_bytes = leftover_bytes.min(usize::try_from(self.chunk_size).unwrap());
        let chunk_u32s = (chunk_bytes + 3) / 4; // Round up

        let chunk = LpspiIndexChunk {
            bytecount: u32::try_from(chunk_bytes).unwrap(),
            offsets: self.offset..(self.offset + chunk_u32s),
        };

        self.offset += chunk_u32s;

        Some(chunk)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct LpspiIndexChunk {
    bytecount: u32,
    offsets: Range<usize>,
}

impl LpspiIndexChunk {
    pub fn bytecount(&self) -> u32 {
        self.bytecount
    }
    pub fn offsets(&self) -> Range<usize> {
        self.offsets.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::vec::Vec;

    macro_rules! check_read {
        ($t:ty, $data:expr, $pos:expr, $expected:expr) => {{
            let buf: &[$t] = &$data;
            assert_eq!(buf.read($pos), $expected)
        }};
    }

    macro_rules! check_write {
        ($init_val:expr, $pos:expr, $val:expr, $expected:expr) => {{
            let buf = &mut $init_val;
            buf.write($pos, $val);
            assert_eq!(buf, &$expected)
        }};
    }

    #[test]
    fn bytecount_u8() {
        assert_eq!([0x12u8, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde].bytecount(), 7);
    }
    #[test]
    fn bytecount_u16() {
        assert_eq!([0x1234u16, 0x5678, 0x9abc].bytecount(), 6);
    }
    #[test]
    fn bytecount_u32() {
        assert_eq!([0x12345678u32, 0x9abcdeff].bytecount(), 8);
    }

    #[test]
    fn read_u8() {
        check_read!(
            u8,
            [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde],
            0,
            0x12345678
        );
        check_read!(
            u8,
            [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde],
            1,
            0x9abcde00
        );
    }

    #[test]
    fn write_u8() {
        check_write!([0u8; 5], 0, 0x12345678, [0x12, 0x34, 0x56, 0x78, 0]);
        check_write!(
            [0u8; 9],
            1,
            0x12345678,
            [0, 0, 0, 0, 0x12, 0x34, 0x56, 0x78, 0]
        );
        check_write!([0u8; 5], 1, 0x12345678, [0, 0, 0, 0, 0x12]);
    }

    #[test]
    fn read_u16() {
        check_read!(u16, [0x1234, 0x5678, 0x9abc], 0, 0x12345678);
        check_read!(u16, [0x1234, 0x5678, 0x9abc], 1, 0x9abc0000);
    }

    #[test]
    fn write_u16() {
        check_write!([0u16; 3], 0, 0x12345678, [0x1234, 0x5678, 0]);
        check_write!([0u16; 5], 1, 0x12345678, [0, 0, 0x1234, 0x5678, 0]);
        check_write!([0u16; 3], 1, 0x12345678, [0, 0, 0x1234]);
    }

    #[test]
    fn read_u32() {
        check_read!(u32, [0x12345678, 0x9abcdeff], 0, 0x12345678);
        check_read!(u32, [0x12345678, 0x9abcdeff], 1, 0x9abcdeff);
        check_read!(u32, [0x12345678, 0x9abcdeff], 2, 0);
    }

    #[test]
    fn write_u32() {
        check_write!([0u32; 2], 0, 0x12345678, [0x12345678, 0]);
        check_write!([0u32; 2], 1, 0x12345678, [0, 0x12345678]);
        check_write!([0u32; 2], 2, 0x12345678, [0, 0]);
    }

    macro_rules! check_chunks {
        ($t:ty, $count:expr, $chunksize:expr, $expected:expr) => {{
            let chunk_iter = LpspiIndexChunks::new([<$t>::MIN; $count].bytecount(), $chunksize);

            let actual_owned = chunk_iter
                .map(|c| (c.bytecount(), c.offsets().collect::<Vec<usize>>()))
                .collect::<Vec<(u32, Vec<usize>)>>();

            let actual = actual_owned
                .iter()
                .map(|(count, range)| (*count, range.as_slice()))
                .collect::<Vec<_>>();

            let expected: &[(u32, &[usize])] = &$expected;

            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn chunksize_u8() {
        check_chunks!(
            u8,
            41,
            17,
            [(16, &[0, 1, 2, 3]), (16, &[4, 5, 6, 7]), (9, &[8, 9, 10])]
        );
        check_chunks!(u8, 0, 9, []);
        check_chunks!(u8, 1, 9, [(1, &[0])]);
        check_chunks!(u8, 2, 9, [(2, &[0])]);
        check_chunks!(u8, 3, 9, [(3, &[0])]);
        check_chunks!(u8, 4, 9, [(4, &[0])]);
        check_chunks!(u8, 5, 9, [(5, &[0, 1])]);
        check_chunks!(u8, 6, 9, [(6, &[0, 1])]);
        check_chunks!(u8, 7, 9, [(7, &[0, 1])]);
        check_chunks!(u8, 8, 9, [(8, &[0, 1])]);
        check_chunks!(u8, 9, 9, [(8, &[0, 1]), (1, &[2])]);
        check_chunks!(u8, 10, 9, [(8, &[0, 1]), (2, &[2])]);
        check_chunks!(u8, 11, 9, [(8, &[0, 1]), (3, &[2])]);
        check_chunks!(u8, 12, 9, [(8, &[0, 1]), (4, &[2])]);
        check_chunks!(u8, 13, 9, [(8, &[0, 1]), (5, &[2, 3])]);
        check_chunks!(u8, 14, 9, [(8, &[0, 1]), (6, &[2, 3])]);
        check_chunks!(u8, 15, 9, [(8, &[0, 1]), (7, &[2, 3])]);
        check_chunks!(u8, 16, 9, [(8, &[0, 1]), (8, &[2, 3])]);
        check_chunks!(u8, 17, 9, [(8, &[0, 1]), (8, &[2, 3]), (1, &[4])]);
    }

    #[test]
    fn chunksize_u16() {
        check_chunks!(
            u16,
            21,
            17,
            [(16, &[0, 1, 2, 3]), (16, &[4, 5, 6, 7]), (10, &[8, 9, 10])]
        );
        check_chunks!(u16, 0, 9, []);
        check_chunks!(u16, 1, 9, [(2, &[0])]);
        check_chunks!(u16, 2, 9, [(4, &[0])]);
        check_chunks!(u16, 3, 9, [(6, &[0, 1])]);
        check_chunks!(u16, 4, 9, [(8, &[0, 1])]);
        check_chunks!(u16, 5, 9, [(8, &[0, 1]), (2, &[2])]);
        check_chunks!(u16, 6, 9, [(8, &[0, 1]), (4, &[2])]);
        check_chunks!(u16, 7, 9, [(8, &[0, 1]), (6, &[2, 3])]);
        check_chunks!(u16, 8, 9, [(8, &[0, 1]), (8, &[2, 3])]);
        check_chunks!(u16, 9, 9, [(8, &[0, 1]), (8, &[2, 3]), (2, &[4])]);
    }

    #[test]
    fn chunksize_u32() {
        check_chunks!(
            u32,
            11,
            17,
            [(16, &[0, 1, 2, 3]), (16, &[4, 5, 6, 7]), (12, &[8, 9, 10])]
        );
        check_chunks!(u32, 0, 9, []);
        check_chunks!(u32, 1, 9, [(4, &[0])]);
        check_chunks!(u32, 2, 9, [(8, &[0, 1])]);
        check_chunks!(u32, 3, 9, [(8, &[0, 1]), (4, &[2])]);
        check_chunks!(u32, 4, 9, [(8, &[0, 1]), (8, &[2, 3])]);
        check_chunks!(u32, 5, 9, [(8, &[0, 1]), (8, &[2, 3]), (4, &[4])]);
    }

    #[test]
    fn combined_u8() {
        let data = {
            let mut e = [0u8; 99];
            e.iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = (i + 1) as u8);
            e
        };

        let mut data_out = [0u8; 100];

        let chunks = LpspiIndexChunks::new(data.bytecount(), 9);

        let data_collected = chunks
            .flat_map(|chunk| {
                let mut local = std::vec![0u8; chunk.bytecount() as usize];
                let mut local_offset = 0;
                for offset in chunk.offsets() {
                    let val = data.read(offset);
                    data_out.write(offset, val);
                    for val in val.to_be_bytes() {
                        if local_offset < chunk.bytecount() as usize {
                            local[local_offset] = val;
                        }
                        local_offset += 1;
                    }
                }
                local
            })
            .collect::<Vec<_>>();

        assert_eq!(&data_out[..data.len()], data);
        assert_eq!(data_collected, data);

        data_out[data.len()..]
            .iter()
            .for_each(|val| assert_eq!(*val, 0));
    }

    #[test]
    fn combined_u16() {
        let data = {
            let mut e = [0u16; 99];
            e.iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = (((2 * i + 1) << 8) + 2 * i + 2) as u16);
            e
        };

        let mut data_out = [0u16; 100];

        let chunks = LpspiIndexChunks::new(data.bytecount(), 9);

        let data_collected = chunks
            .flat_map(|chunk| {
                let mut local = std::vec![0u8; chunk.bytecount() as usize];
                let mut local_offset = 0;
                for offset in chunk.offsets() {
                    let val = data.read(offset);
                    data_out.write(offset, val);
                    for val in val.to_be_bytes() {
                        if local_offset < chunk.bytecount() as usize {
                            local[local_offset] = val;
                        }
                        local_offset += 1;
                    }
                }
                local
            })
            .collect::<Vec<_>>();

        assert_eq!(&data_out[..data.len()], data);
        data_out[data.len()..]
            .iter()
            .for_each(|val| assert_eq!(*val, 0));

        assert_eq!(
            data_collected,
            data.iter()
                .flat_map(|v| v.to_be_bytes())
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn combined_u32() {
        let data = {
            let mut e = [0u32; 99];
            e.iter_mut().enumerate().for_each(|(i, v)| {
                *v = (((((((4 * i + 1) << 8) + 4 * i + 2) << 8) + 4 * i + 3) << 8) + 4 * i + 4)
                    as u32
            });
            e
        };

        let mut data_out = [0u32; 100];

        let chunks = LpspiIndexChunks::new(data.bytecount(), 9);

        let data_collected = chunks
            .flat_map(|chunk| {
                let mut local = std::vec![0u8; chunk.bytecount() as usize];
                let mut local_offset = 0;
                for offset in chunk.offsets() {
                    let val = data.read(offset);
                    data_out.write(offset, val);
                    for val in val.to_be_bytes() {
                        if local_offset < chunk.bytecount() as usize {
                            local[local_offset] = val;
                        }
                        local_offset += 1;
                    }
                }
                local
            })
            .collect::<Vec<_>>();

        assert_eq!(&data_out[..data.len()], data);
        data_out[data.len()..]
            .iter()
            .for_each(|val| assert_eq!(*val, 0));

        assert_eq!(
            data_collected,
            data.iter()
                .flat_map(|v| v.to_be_bytes())
                .collect::<Vec<_>>()
        )
    }
}
