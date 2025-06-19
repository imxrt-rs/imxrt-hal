//! Audio playback using sai peripheral and imxrt-hal.
//!
//! Plays back a simple 440Hz (A note) simple square wave tone with the SAI peripheral
//! to an SGTL5000 codec. Tested with Teensy 4.1 and its own audio board (rev D).
//!
//! The audio stream itself is expected to be a 48000Hz 16bit stereo signal.
//!
//! Pinout:
//! Teensy       PCM5102
//! --------------------
//! GND          GND
//! 3.3V         VIN
//! Pin7         DIN
//! Pin18        SDA
//! Pin19        SCL
//! Pin20        LRCK
//! Pin21        BCK
//! Pin23        MCLK
//!

#![no_main]
#![no_std]

/// Half of a sine wave (0 to pi)
/// Can be used to generate a full sine wave by inverting (-1 * SIN_LUT[X]);
const SIN_LUT: [u16; 256] = [
    0, 402, 804, 1206, 1608, 2009, 2411, 2811, 3212, 3612, 4011, 4410, 4808, 5205, 5602, 5998,
    6393, 6787, 7180, 7571, 7962, 8351, 8740, 9127, 9512, 9896, 10279, 10660, 11039, 11417, 11793,
    12167, 12540, 12910, 13279, 13646, 14010, 14373, 14733, 15091, 15447, 15800, 16151, 16500,
    16846, 17190, 17531, 17869, 18205, 18538, 18868, 19195, 19520, 19841, 20160, 20475, 20788,
    21097, 21403, 21706, 22006, 22302, 22595, 22884, 23170, 23453, 23732, 24008, 24279, 24548,
    24812, 25073, 25330, 25583, 25833, 26078, 26320, 26557, 26791, 27020, 27246, 27467, 27684,
    27897, 28106, 28311, 28511, 28707, 28899, 29086, 29269, 29448, 29622, 29792, 29957, 30118,
    30274, 30425, 30572, 30715, 30853, 30986, 31114, 31238, 31357, 31471, 31581, 31686, 31786,
    31881, 31972, 32058, 32138, 32214, 32286, 32352, 32413, 32470, 32522, 32568, 32610, 32647,
    32679, 32706, 32729, 32746, 32758, 32766, 32767, 32766, 32758, 32746, 32729, 32706, 32679,
    32647, 32610, 32568, 32522, 32470, 32413, 32352, 32286, 32214, 32138, 32058, 31972, 31881,
    31786, 31686, 31581, 31471, 31357, 31238, 31114, 30986, 30853, 30715, 30572, 30425, 30274,
    30118, 29957, 29792, 29622, 29448, 29269, 29086, 28899, 28707, 28511, 28311, 28106, 27897,
    27684, 27467, 27246, 27020, 26791, 26557, 26320, 26078, 25833, 25583, 25330, 25073, 24812,
    24548, 24279, 24008, 23732, 23453, 23170, 22884, 22595, 22302, 22006, 21706, 21403, 21097,
    20788, 20475, 20160, 19841, 19520, 19195, 18868, 18538, 18205, 17869, 17531, 17190, 16846,
    16500, 16151, 15800, 15447, 15091, 14733, 14373, 14010, 13646, 13279, 12910, 12540, 12167,
    11793, 11417, 11039, 10660, 10279, 9896, 9512, 9127, 8740, 8351, 7962, 7571, 7180, 6787, 6393,
    5998, 5602, 5205, 4808, 4410, 4011, 3612, 3212, 2811, 2411, 2009, 1608, 1206, 804, 402,
];

/// Generate a sine wave sample
fn sine(t: u32) -> u16 {
    let p = t % 512;
    let s = SIN_LUT[(p % 256) as usize];
    if p < 256 {
        (32768 + s) / 2
    } else {
        (32768 - s) / 2
    }
}

/// Generate a square wave sample
fn square(t: u32) -> u16 {
    if (t % 128) > 64 {
        32767
    } else {
        0
    }
}

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    //
    // Configure the demo below.
    //

    const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
    const BACKEND: board::logging::Backend = board::logging::BACKEND;

    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 4;

    /// How frequently (milliseconds) should we poll audio
    const AUDIO_POLL_MS: u32 = 1000 * (board::PIT_FREQUENCY / 1_000);

    use crate::{sine, square};
    use eh1::i2c::I2c;
    use imxrt_hal::{self as hal};
    type SaiTx = hal::sai::Tx<1, 16, 2, hal::sai::PackingNone>;
    type SaiRx = hal::sai::Rx<1, 16, 2, hal::sai::PackingNone>;

    //     type I2cScl = iomuxc::gpio_ad_b1::GPIO_AD_B1_00; // P19
    // type I2cSda = iomuxc::gpio_ad_b1::GPIO_AD_B1_01; // P18
    // pub type I2cPins = hal::lpi2c::Pins<I2cScl, I2cSda>;

    // pub type I2c = hal::lpi2c::Lpi2c<I2cPins, 1>;

    //
    // End configurations.
    //

    #[local]
    struct Local {
        led: board::Led,
        poll_log: hal::pit::Pit<1>,

        /// This timer tells us how frequently work on audio.
        audio_pit: hal::pit::Pit<2>,

        /// Sample counter for the wave generation
        counter: u32,
        dac_cp: Sgtl5000<board::I2c>,
    }

    #[shared]
    struct Shared {
        /// Serial audio interface
        sai1_tx: SaiTx,
        sai1_rx: SaiRx,
        poller: board::logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut cortex_m = cx.core;
        let (
            board::Common {
                pit: (_, mut poll_log, mut audio_pit, _),
                usb1,
                usbnc1,
                usbphy1,

                mut dma,
                ..
            },
            board::Specifics {
                led,
                sai1,
                console,
                mut i2c,
                ..
            },
        ) = board::new();

        if BACKEND == board::logging::Backend::Lpuart {
            poll_log.set_load_timer_value(LPUART_POLL_INTERVAL_MS);
            poll_log.set_interrupt_enable(true);
            poll_log.enable();
        } else {
            poll_log.disable();
        }

        let usbd = hal::usbd::Instances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
        };

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

        let mut sai_config = hal::sai::SaiConfig::i2s(hal::sai::bclk_div(8));
        sai_config.sync_mode = hal::sai::SyncMode::TxFollowRx;
        sai_config.bclk_src_swap = true;
        let (Some(sai1_tx), Some(sai1_rx)) = sai1.split(&sai_config) else {
            panic!("Unexpected return from sai split");
        };

        let mut sai1_tx: SaiTx = sai1_tx;
        let mut sai1_rx: SaiRx = sai1_rx;

        let regs = sai1_tx.reg_dump();
        defmt::println!(
            "Regdump of config: TCR1: {:b}, TCR2 {:b}, TCR3 {:b}, TCR4 {:b}, TCR5 {:b}, TCSR: {:b}",
            regs[0],
            regs[1],
            regs[2],
            regs[3],
            regs[4],
            regs[5]
        );

        cortex_m.DCB.enable_trace();
        cortex_m::peripheral::DWT::unlock();
        cortex_m.DWT.enable_cycle_counter();

        audio_pit.set_load_timer_value(AUDIO_POLL_MS);
        audio_pit.set_interrupt_enable(true);
        audio_pit.enable();

        let mut counter: u32 = 0;
        for _i in 0..31 {
            sai1_tx.write_frame(0, [sine(counter), square(counter)]);
            counter += 1;
        }
        sai1_tx.set_enable(true);
        sai1_tx.set_interrupts(
            hal::sai::Interrupts::FIFO_WARNING | hal::sai::Interrupts::FIFO_REQUEST,
        );

        sai1_tx.set_enable(true);
        sai1_rx.set_enable(true);

        i2c.set_controller_enable(true);

        let mut dac_cp = Sgtl5000::new(i2c, 0x0A);
        _ = dac_cp.enable();
        _ = dac_cp.unmute();

        (
            Shared {
                sai1_tx,
                sai1_rx,
                poller,
            },
            Local {
                led,
                poll_log,
                audio_pit,
                dac_cp,
                counter,
            },
        )
    }

    #[task(binds = BOARD_SAI1, shared = [sai1_tx, sai1_rx], local = [counter, led], priority = 2)]
    fn sai1_interrupt(mut cx: sai1_interrupt::Context) {
        let sai1_interrupt::LocalResources { counter, led, .. } = cx.local;

        cx.shared.sai1_tx.lock(|sai1_tx| {
            sai1_tx.clear_status(hal::sai::Status::FIFO_ERROR); //TODO: figure out why FIFO error happens
            while sai1_tx.status().contains(hal::sai::Status::FIFO_REQUEST) {
                sai1_tx.write_frame(0, [sine(*counter), square(*counter)]);
                *counter = (*counter).wrapping_add(1);
            }
            if (*counter % 10000) == 0 {
                led.toggle();
            }
        });
    }

    #[task(binds = BOARD_USB1, priority = 1)]
    fn usb_interrupt(_: usb_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    #[task(binds = BOARD_DMA_A, priority = 1)]
    fn dma_interrupt(_: dma_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// Actually performs the poll call.
    #[task(shared = [poller], priority = 2)]
    async fn poll_logger(mut cx: poll_logger::Context) {
        cx.shared.poller.lock(|poller| poller.poll());
    }

    #[task(binds = BOARD_PIT, shared = [sai1_tx, sai1_rx], local = [audio_pit, poll_log, dac_cp], priority = 1)]
    fn pit_interrupt(mut cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            audio_pit,
            poll_log,
            //dac_cp,
            ..
        } = cx.local;

        while audio_pit.is_elapsed() {
            audio_pit.clear_elapsed();
        }

        let (status, write_pos, read_pos) = cx.shared.sai1_tx.lock(|sai1_tx| {
            let status = sai1_tx.status();
            let (write_pos, read_pos) = sai1_tx.fifo_position(0);
            (status, write_pos, read_pos)
        });

        log::info!(
            "Audio synthesis tx status {:#x}, fifo underrun? {}, word start? {}, write pos {}, read pos {}",
            status.bits(),
            status.contains(hal::sai::Status::FIFO_ERROR),
            status.contains(hal::sai::Status::WORD_START),
            write_pos,
            read_pos,
        );

        let (status, write_pos, read_pos) = cx.shared.sai1_rx.lock(|sai1_rx| {
            let status = sai1_rx.status();
            let (write_pos, read_pos) = sai1_rx.fifo_position(0);
            (status, write_pos, read_pos)
        });

        log::info!(
            "Audio synthesis rx status {:#x}, fifo underrun? {}, word start? {}, write pos {}, read pos {}",
            status.bits(),
            status.contains(hal::sai::Status::FIFO_ERROR),
            status.contains(hal::sai::Status::WORD_START),
            write_pos,
            read_pos,
        );

        // Is it time for us to poll the logger?
        // This only happens for the LPUART backend.
        if poll_log.is_elapsed() {
            while poll_log.is_elapsed() {
                poll_log.clear_elapsed();
            }
            poll_logger::spawn().unwrap();
        }
    }

    use eh1::i2c::SevenBitAddress;
    pub struct Sgtl5000<I2C> {
        i2c: I2C,
        address: u8,
    }
    const CHIP_DIG_POWER: u16 = 0x0002;
    const CHIP_CLK_CTRL: u16 = 0x0004;
    const CHIP_I2S_CTRL: u16 = 0x0006;
    const CHIP_SSS_CTRL: u16 = 0x000A;
    const CHIP_ADCDAC_CTRL: u16 = 0x000E;
    const CHIP_DAC_VOL: u16 = 0x0010;
    const CHIP_ANA_HP_CTRL: u16 = 0x0022;
    const CHIP_ANA_CTRL: u16 = 0x0024;
    const CHIP_LINREG_CTRL: u16 = 0x0026;
    const CHIP_REF_CTRL: u16 = 0x0028;
    const CHIP_LINE_OUT_VOL: u16 = 0x002C;
    const CHIP_ANA_POWER: u16 = 0x0030;
    const CHIP_SHORT_CTRL: u16 = 0x003C;

    const MUTE_HP_MASK: u16 = 1 << 4;

    impl<I2C, E> Sgtl5000<I2C>
    where
        I2C: I2c<SevenBitAddress, Error = E>,
    {
        pub fn new(i2c: I2C, address: u8) -> Self {
            Self { i2c, address }
        }

        /// Low-level: write 16-bit to register
        pub fn write_register(&mut self, reg: u16, val: u16) -> Result<(), E> {
            let buf = [(reg >> 8) as u8, reg as u8, (val >> 8) as u8, val as u8];
            self.i2c.write(self.address, &buf)
        }

        /// Low-level: read 16-bit register
        pub fn read_register(&mut self, reg: u16) -> Result<u16, E> {
            let reg_buf = [(reg >> 8) as u8, reg as u8];
            let mut val_buf = [0u8; 2];
            self.i2c.write_read(self.address, &reg_buf, &mut val_buf)?;
            Ok(((val_buf[0] as u16) << 8) | val_buf[1] as u16)
        }

        pub fn enable(&mut self) -> Result<(), E> {
            // Init sequence based on the C++ driver: https://github.com/PaulStoffregen/Audio/blob/master/control_sgtl5000.cpp
            self.write_register(CHIP_LINREG_CTRL, 0x006C)?;
            self.write_register(CHIP_REF_CTRL, 0x01f2)?;
            self.write_register(CHIP_LINE_OUT_VOL, 0x0F22)?;
            self.write_register(CHIP_SHORT_CTRL, 0x4446)?;
            self.write_register(CHIP_ANA_CTRL, 0x0137)?;
            self.write_register(CHIP_ANA_POWER, 0x40ff)?;
            self.write_register(CHIP_DIG_POWER, 0x0073)?;
            self.write_register(CHIP_LINE_OUT_VOL, 0x1D1D)?;

            self.write_register(CHIP_CLK_CTRL, 0x0008)?; // 48.0 kHz, 256*Fs
            self.write_register(CHIP_I2S_CTRL, 0x0030)?; // SCLK=64*Fs, 16bit, I2S format

            self.write_register(CHIP_SSS_CTRL, 0x0010)?; // ADC->I2S, I2S->DAC
            self.write_register(CHIP_ADCDAC_CTRL, 0x0000)?; // disable dac mute
            self.write_register(CHIP_DAC_VOL, 0x3C3C)?; // digital gain, 0dB
            self.write_register(CHIP_ANA_HP_CTRL, 0x7F7F)?; // set volume (lowest level)
            self.write_register(CHIP_ANA_CTRL, 0x0036)?; // enable zero cross detectors
            Ok(())
        }

        pub fn mute(&mut self) -> Result<(), E> {
            let current = self.read_register(CHIP_ANA_CTRL)?;
            let new = current | MUTE_HP_MASK;
            self.write_register(CHIP_ANA_CTRL, new)
        }

        pub fn unmute(&mut self) -> Result<(), E> {
            let current = self.read_register(CHIP_ANA_CTRL)?;
            let new = current & !MUTE_HP_MASK;
            self.write_register(CHIP_ANA_CTRL, new)
        }

        pub fn set_volume(&mut self, vol: f32) -> Result<(), E> {
            let clamped = vol.clamp(0.0, 1.0);
            let raw = (clamped * 0x3F as f32) as u8;
            let vol16 = ((raw as u16) << 8) | raw as u16;
            self.write_register(CHIP_ANA_HP_CTRL, vol16)
        }
    }
}
