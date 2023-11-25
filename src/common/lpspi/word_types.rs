use super::LpspiError;

/// A data type that can be used for LPSPI transfers
pub trait LpspiDataBuffer {
    /// Length in bytes
    fn length(&self) -> usize;

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
    fn length(&self) -> usize {
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
    fn length(&self) -> usize {
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
    fn length(&self) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
