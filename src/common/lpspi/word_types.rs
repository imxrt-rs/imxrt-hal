use super::LpspiError;

// TODO: need two traits, one for &[X] and one for &mut[x].
pub trait LpspiInputData {
    /// Length in bytes
    fn length(&self) -> usize;

    /// Retreive the next chunk to transmit.
    /// Filled with zeros when overflown.
    fn take(&mut self) -> u32;

    /// Store the next received chunk.
    ///
    fn store(&mut self, val: u32);
}

pub trait AsLpspiInputData {
    fn process(
        &self,
        f: &dyn FnOnce(&mut dyn LpspiInputData) -> Result<(), LpspiError>,
    ) -> Result<(), LpspiError>;
}

struct U8LpspiInputData<'a> {
    read_pos: usize,
    write_pos: usize,
    data: &'a [u8],
}
struct U16LpspiInputData<'a> {
    read_pos: usize,
    write_pos: usize,
    data: &'a [u16],
}
struct U32LpspiInputData<'a> {
    read_pos: usize,
    write_pos: usize,
    data: &'a [u32],
}

impl AsLpspiInputData for &[u8] {
    fn process(
        &self,
        f: &dyn FnOnce(&mut dyn LpspiInputData) -> Result<(), LpspiError>,
    ) -> Result<(), LpspiError> {
        let mut data = U8LpspiInputData {
            read_pos: 0,
            write_pos: 0,
            data: &self,
        };

        f(&mut data)
    }
}
impl AsLpspiInputData for &[u16] {
    fn process(
        &self,
        f: &dyn FnOnce(&mut dyn LpspiInputData) -> Result<(), LpspiError>,
    ) -> Result<(), LpspiError> {
        let mut data = U16LpspiInputData {
            read_pos: 0,
            write_pos: 0,
            data: &self,
        };

        f(&mut data)
    }
}
impl AsLpspiInputData for &[u32] {
    fn process(
        &self,
        f: &dyn FnOnce(&mut dyn LpspiInputData) -> Result<(), LpspiError>,
    ) -> Result<(), LpspiError> {
        let mut data = U32LpspiInputData {
            read_pos: 0,
            write_pos: 0,
            data: &self,
        };

        f(&mut data)
    }
}

impl LpspiInputData for U8LpspiInputData<'_> {
    fn length(&self) -> usize {
        self.data.len()
    }

    fn take(&mut self) -> Option<u32> {
        let mut output = 0;

        for _ in 0..4 {
            output = output << 8;
            output += u32::from(*self.data.get(self.read_pos)?);
            self.read_pos += 1;
        }

        Some(output)
    }

    fn store(&mut self, val: u32) {
        todo!()
    }
}

impl LpspiInputData for U16LpspiInputData<'_> {
    fn length(&self) -> usize {
        self.data.len() * 2
    }

    fn take(&mut self) -> u32 {
        // Only check the first one, because we need to zero fill otherwise
        let mut output = 0;

        for _ in 0..2 {
            output = output << 16;
            output += u32::from(self.data.get(self.read_pos).copied().unwrap_or_default());
            self.read_pos += 1;
        }

        output
    }

    fn store(&mut self, val: u32) {
        todo!()
    }
}

impl LpspiInputData for U32LpspiInputData<'_> {
    fn length(&self) -> usize {
        self.data.len() * 4
    }

    fn take(&mut self) -> Option<u32> {
        let data = *self.data.get(self.read_pos)?;
        self.read_pos += 1;
        Some(data)
    }

    fn store(&mut self, val: u32) {
        todo!()
    }
}
