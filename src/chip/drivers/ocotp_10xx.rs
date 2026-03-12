use super::ocotp;

impl ocotp::Ocotp {
    #[inline(always)]
    pub(crate) fn read_fuse_data(&self) -> u32 {
        crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, READ_FUSE_DATA)
    }

    /// There's no `OUT_STATUS` register on these parts, so
    /// there's DED event to check.
    #[inline(always)]
    pub(crate) fn check_end_fuse_read(&mut self) -> Result<(), ocotp::Error> {
        Ok(())
    }

    /// There's no `OUT_STATUS` register on these parts.
    #[inline(always)]
    pub(crate) fn check_end_fuse_write(&mut self) -> Result<(), ocotp::Error> {
        Ok(())
    }

    pub(crate) const FUSE_ADDRESS_OFFSET: u16 = 0x400;
}
