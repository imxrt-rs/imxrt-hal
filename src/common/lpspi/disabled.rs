use eh1::spi::{Phase, Polarity};

use super::{ral, Disabled, Lpspi, Mode};

impl<'a, 'b, const N: u8> Disabled<'a, 'b, N> {
    pub(crate) fn new(bus: &'a mut Lpspi<'b, N>) -> Self {
        let men = ral::read_reg!(ral::lpspi, bus.data.lpspi.instance(), CR, MEN == MEN_1);
        ral::modify_reg!(ral::lpspi, bus.data.lpspi.instance(), CR, MEN: MEN_0);
        while ral::read_reg!(ral::lpspi, lpspi, CR, MEN == MEN_1) {}
        Self { bus, men }
    }

    /// Set the LPSPI clock speed (Hz).
    pub fn set_clock_hz(&mut self, spi_clock_hz: u32) {
        // Round up, so we always get a resulting SPI clock that is
        // equal or less than the requested frequency.
        let half_div = u32::try_from(
            1 + u64::from(self.bus.source_clock_hz - 1) / (u64::from(spi_clock_hz) * 2),
        )
        .unwrap();

        // Make sure SCKDIV is between 0 and 255
        // For some reason SCK starts to misbehave if half_div is less than 3
        let half_div = half_div.clamp(3, 128);
        // Because half_div is in range [3,128], sckdiv is in range [4, 254].
        let sckdiv = 2 * (half_div - 1);

        ral::write_reg!(ral::lpspi, self.bus.data.lpspi.instance(), CCR,
            // Delay between two clock transitions of two consecutive transfers
            // is exactly sckdiv/2, which causes the transfer to be seamless.
            DBT: half_div - 1,
            // Add one sckdiv/2 setup and hold time before and after the transfer,
            // to make sure the signal is stable at sample time
            PCSSCK: half_div - 1,
            SCKPCS: half_div - 1,
            SCKDIV: sckdiv
        );
    }
}

impl<const N: u8> Drop for Disabled<'_, '_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpspi, self.bus.data.lpspi.instance(), CR, MEN: if self.men {MEN_1} else {MEN_0});
    }
}
