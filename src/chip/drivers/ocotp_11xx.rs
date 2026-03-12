use super::ocotp;

impl ocotp::Ocotp {
    #[inline(always)]
    pub(crate) fn read_fuse_data(&self) -> u32 {
        crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, READ_FUSE_DATA0)
    }

    #[inline(always)]
    pub(crate) fn check_end_fuse_read(&mut self) -> Result<(), ocotp::Error> {
        if crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS, DED == 1) {
            // Clear DED by writing to SCT space, not by a general write.
            crate::ral::write_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS_SET, DED: 1);
            return Err(ocotp::Error::DedRead);
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn check_end_fuse_write(&mut self) -> Result<(), ocotp::Error> {
        let (progfail, locked) =
            crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS, PROGFAIL, LOCKED);
        crate::ral::write_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS_SET, PROGFAIL: progfail, LOCKED: locked);

        if 0 != progfail {
            return Err(ocotp::Error::Programming);
        }
        if 0 != locked {
            return Err(ocotp::Error::LockedRegionAccess);
        }
        Ok(())
    }

    /// Returns `true` if the last read had a single bit corrected.
    ///
    /// This automatically clears the SEC bit. You can sequence this
    /// after a read to check if there was a correction. SEC only
    /// applies if the fuse is ECC redundant.
    ///
    /// If you do not call [`clear_read_sec()`](Self::clear_read_sec),
    /// then this bit may latch `true` if there was ever *any* SEC.
    pub fn has_read_sec(&self) -> bool {
        crate::ral::read_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS, SEC == 1)
    }

    /// Clear the status that indicates a SEC during read.
    pub fn clear_read_sec(&self) {
        crate::ral::write_reg!(crate::ral::ocotp, self.ocotp, OUT_STATUS_SET, SEC: 1);
    }

    pub(crate) const FUSE_ADDRESS_OFFSET: u16 = 0x800;
}

#[cfg(test)]
mod tests {
    use super::ocotp::FuseAddress;

    #[test]
    fn fuse_address_from_rm_example() {
        let fuse = FuseAddress::new(0x960).unwrap();
        assert_eq!(fuse.raw(), 0x16);
    }

    #[test]
    fn fuse_address_from_min_fuse() {
        // A low address, non-reserved fuse shown in one
        // reference manual.
        let fuse = FuseAddress::new(0x840).unwrap();
        assert_eq!(fuse.raw(), 4);
    }

    #[test]
    fn fuse_address_from_max_fuse() {
        // Similar to the min test, but using a high address.
        let fuse = FuseAddress::new(0x1800).unwrap();
        assert_eq!(fuse.raw(), 0x100);
    }
}
