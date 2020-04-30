macro_rules! alt0 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt0 setting
        pub fn alt0(self) -> $Pad<$crate::iomuxc::Alt0> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT0
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt1 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt1 setting
        pub fn alt1(self) -> $Pad<$crate::iomuxc::Alt1> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT1
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt2 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt2 setting
        pub fn alt2(self) -> $Pad<$crate::iomuxc::Alt2> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT2
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt3 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt3 setting
        pub fn alt3(self) -> $Pad<$crate::iomuxc::Alt3> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT3
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt4 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt4 setting
        pub fn alt4(self) -> $Pad<$crate::iomuxc::Alt4> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT4
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt5 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt5 setting
        pub fn alt5(self) -> $Pad<$crate::iomuxc::Alt5> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT5
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt6 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt6 setting
        pub fn alt6(self) -> $Pad<$crate::iomuxc::Alt6> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT6
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt7 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt7 setting
        pub fn alt7(self) -> $Pad<$crate::iomuxc::Alt7> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT7
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt8 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt8 setting
        pub fn alt8(self) -> $Pad<$crate::iomuxc::Alt8> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT8
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! alt9 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt9 setting
        pub fn alt9(self) -> $Pad<$crate::iomuxc::Alt9> {
            unsafe {
                ::imxrt_ral::modify_reg!(
                    $crate::ral::iomuxc,
                    $crate::ral::iomuxc::IOMUXC,
                    $mux_mod,
                    MUX_MODE: $crate::ral::iomuxc::$mux_mod::MUX_MODE::RW::ALT9
                )
            };
            $Pad {
                _alt: core::marker::PhantomData,
            }
        }
    };
}

macro_rules! pad {
    ($Pad:ident, $mux_mod:ident, $pad_mod:ident, [$($alt_macro:ident),+]) => {
        pub struct $Pad<Alt> {
            _alt: core::marker::PhantomData<Alt>,
        }
        impl<Alt> $Pad<Alt> {
            $(
                $alt_macro!($Pad, $mux_mod);
            )+

            // TODO Remove once all pads are exposed in IOMUXC, and these are properly used
            #[allow(dead_code)]
            pub(crate) fn new() -> Self {
                Self {
                    _alt: core::marker::PhantomData,
                }
            }

            // TODO needs review if this is used
            #[allow(dead_code)]
            pub(crate) fn iomuxc(&self) -> &$crate::ral::iomuxc::RegisterBlock {
                // Safety: register block is always valid
                unsafe { &*$crate::ral::iomuxc::IOMUXC }
            }

            /// Enables software input on (SION) for the pin, which forces
            /// the input path for the pin.
            #[allow(dead_code)] // Method may not be used on a pin
            pub(crate) fn sion_enable(&mut self) {
                unsafe { ::imxrt_ral::modify_reg!($crate::ral::iomuxc, $crate::ral::iomuxc::IOMUXC, $mux_mod, SION: $crate::ral::iomuxc::$mux_mod::SION::RW::ENABLED) };
            }

            /// Disables software input on (SION) for the pin. This means that
            /// the pin's input path is determined by its functionality.
            #[allow(dead_code)] // Method may not be used on a pin
            pub(crate) fn sion_disable(&mut self) {
                unsafe { ::imxrt_ral::modify_reg!($crate::ral::iomuxc, $crate::ral::iomuxc::IOMUXC, $mux_mod, SION: $crate::ral::iomuxc::$mux_mod::SION::RW::DISABLED) };
            }

            /// Configure pin settings
            ///
            /// This takes a given PinConfig and updates the *_PAD_MUX_PAD_*
            /// register related to a given Pin.
            ///
            /// If the PinConfig does not write to all fields, which is checked
            /// by PinConfig::is_modify() the config is applied as a
            /// modification to the current PAD_MUX_PAD register value.
            ///
            /// Otherwise the PAD_MUX_PAD register is overwritten with the new
            /// configuration.
            ///
            /// PinConfig docs supply more information on how to build a
            /// PinConfig and what typical defaults from the reference
            /// manual look like.
            ///
            /// # Example using const builder functions
            ///
            /// ```no_run
            /// use imxrt_hal::iomuxc::pin_config::*;
            /// use imxrt_hal::Peripherals;
            ///
            /// const LED_PIN_CONFIG: PinConfig = PinConfig::with_none()
            ///                      .set_pull_up(PullUp::PullUp_100KOhm)
            ///                      .set_speed(Speed::Speed2_150MHz)
            ///                      .set_drive_strength(DriveStrength::R0_DIV_6);
            /// let mut peripherals = Peripherals::take().unwrap();
            /// peripherals.iomuxc.gpio_ad_b0_00.configure(&LED_PIN_CONFIG);
            /// ```
            pub fn configure(&mut self, cfg: &$crate::iomuxc::pin_config::PinConfig) {
                // Safety: iomux registers are per pin and effectively owned by
                // the pin allowing safe access so long as direct register
                // access isn't used
                unsafe {
                    let reg = if cfg.is_modify() {
                        ::imxrt_ral::read_reg!($crate::ral::iomuxc, $crate::ral::iomuxc::IOMUXC, $pad_mod)
                    } else {
                        0
                    };
                    let reg_new = cfg.modify(reg);
                    ::imxrt_ral::write_reg!($crate::ral::iomuxc, $crate::ral::iomuxc::IOMUXC, $pad_mod, reg_new);
                }
            }
        }
    };
}
