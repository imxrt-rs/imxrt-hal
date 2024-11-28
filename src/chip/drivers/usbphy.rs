//! USB physical layer.

use crate::ral;

/// Restart the USB PLL.
///
/// If the PLL was not running before this call, it will be running when this
/// call returns.
pub fn restart_pll<const N: u8>(usbphy: &mut ral::usbphy::Instance<N>) {
    // Reset the important bits to the POR state.
    ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_CLR,
        PLL_POWER: 1,
        PLL_REG_ENABLE: 1,
        PLL_ENABLE: 1,
        PLL_EN_USB_CLKS: 1);
    ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_SET, PLL_BYPASS: 1);

    loop {
        // We're expected to enable the regulator before power.
        //
        // The documentation says there's supposed to be a small delay before enabling power,
        // but we're not explicitly doing that here.
        if ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_REG_ENABLE == 0) {
            ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_SET, PLL_REG_ENABLE: 1);
            continue;
        }
        if ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_POWER == 0) {
            ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_SET, PLL_POWER: 1);
            continue;
        }
        if ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_BYPASS == 1) {
            ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_CLR, PLL_BYPASS: 1);
            continue;
        }
        if ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_ENABLE == 0) {
            ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_SET, PLL_ENABLE: 1);
            continue;
        }
        if ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_EN_USB_CLKS == 0) {
            ral::write_reg!(ral::usbphy, usbphy, PLL_SIC_SET, PLL_EN_USB_CLKS: 1);
            continue;
        }
        break;
    }
    while ral::read_reg!(ral::usbphy, usbphy, PLL_SIC, PLL_LOCK == 0) {}
}
