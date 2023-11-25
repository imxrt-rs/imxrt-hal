use eh1::spi::{Phase, Polarity};

use super::{ral, Disabled, Lpspi, Mode};

impl<'a, 'b, const N: u8> Disabled<'a, 'b, N> {
    pub(crate) fn new(bus: &'a mut Lpspi<'b, N>) -> Self {
        let men = ral::read_reg!(ral::lpspi, bus.data.lpspi.instance(), CR, MEN == MEN_1);
        ral::modify_reg!(ral::lpspi, bus.data.lpspi.instance(), CR, MEN: MEN_0);
        Self { bus, men }
    }

    /// Set the SPI mode for the peripheral
    pub fn set_mode(&mut self, mode: Mode) {
        // This could probably be changed when we're not disabled.
        // However, there's rules about when you can read TCR.
        // Specifically, reading TCR while it's being loaded from
        // the transmit FIFO could result in an incorrect reading.
        // Only permitting this when we're disabled might help
        // us avoid something troublesome.
        ral::modify_reg!(
            ral::lpspi,
            self.bus.data.lpspi.instance(),
            TCR,
            CPOL: ((mode.polarity == Polarity::IdleHigh) as u32),
            CPHA: ((mode.phase == Phase::CaptureOnSecondTransition) as u32)
        );
    }

    /// Set the LPSPI clock speed (Hz).
    pub fn set_clock_hz(&mut self, spi_clock_hz: u32) {
        // Round up, so we always get a resulting SPI clock that is
        // equal or less than the requested frequency.
        let div = 1 + (self.bus.source_clock_hz - 1) / spi_clock_hz;

        // 0 <= div <= 255, and the true coefficient is really div + 2
        let div = div.saturating_sub(2).clamp(0, 255);
        ral::write_reg!(
            ral::lpspi,
            self.bus.data.lpspi.instance(),
            CCR,
            SCKDIV: div,
            // These all don't matter, because we do not use a CS pin.
            // embedded-hal controls the CS pins through `OutputPin`.
            DBT: 0,
            SCKPCS: 0,
            PCSSCK: 0
        );
    }
}

impl<const N: u8> Drop for Disabled<'_, '_, N> {
    fn drop(&mut self) {
        ral::modify_reg!(ral::lpspi, self.bus.data.lpspi.instance(), CR, MEN: self.men as u32);
    }
}
