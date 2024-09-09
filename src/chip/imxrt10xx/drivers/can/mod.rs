//! Flexible Controller Area Network (FlexCAN)
//!
//! The FlexCAN module provides a CanBus peripheral that implements
//! the `embedded_can` traits.
//!//!
//! # Example
//!
//! ```no_run
//! use imxrt_hal;
//! use imxrt_ral;
//!
//! let can1_inst = unsafe { imxrt_ral::can::CAN1::instance() };
//! let pads = unsafe { imxrt_hal::iomuxc::pads::Pads::new() }; // let p23 = imxrt_hal::iomuxc.gpio_ad_b1.p09,
//!
//! let clock_frequency = imxrt_hal::ccm::XTAL_OSCILLATOR_HZ;
//! let mut can1 = imxrt_hal::can::CAN::new(
//!     can1_inst,
//!     pads.gpio_ad_b1.p08,
//!     pads.gpio_ad_b1.p09,
//!     clock_frequency,
//! );
//!
//! can1.set_baud_rate(125_000);
//! can1.set_max_mailbox(16);
//! can1.disable_fifo();
//! // create a `Frame` with `StandardID` 0x00
//! // and `Data` [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]
//! let id = imxrt_hal::can::Id::from(imxrt_hal::can::StandardId::new(0x00).unwrap());
//! let data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
//! let frame = imxrt_hal::can::Frame::new_data(id, data);
//! // read all available mailboxes for any available frames
//! can1.read_mailboxes();
//! // transmit the frame
//! can1.transmit(&frame);
//! ```

mod embedded_hal;
pub mod filter;
mod frame;

pub use self::embedded_hal::{ExtendedId, Id, StandardId};
pub use frame::{CodeReg, Data, FlexCanMailboxCSCode, Frame, IdReg};
use imxrt_iomuxc::consts::Const;
use ral::{modify_reg, read_reg, write_reg};

use crate::iomuxc::flexcan;
use crate::ral;

use core::marker::PhantomData;

/// Error from the FlexCan peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Error indicating that no received data is
    /// available in the mailboxes
    NoRxData,
    /// Error indicating that no transmit mailboxes
    /// are available
    NoTxMailbox,
    /// A wrapper around the [`embedded_hal::ErrorKind`]
    /// enum
    EmbeddedHal(embedded_hal::ErrorKind),
}

/// Pins used for the CAN object
pub struct Pins<Tx, Rx> {
    /// CAN TX Pin
    pub tx: Tx,
    /// CAN RX Pin
    pub rx: Rx,
}

/// A CAN master
pub struct CAN<P, const M: u8> {
    reg: ral::can::Instance<M>,
    _pins: PhantomData<P>,
    _module: PhantomData<ral::can::RegisterBlock>,
    clock_frequency: u32,
    _mailbox_reader_index: u8,
}

/// Data contained within a Mailbox
#[derive(Debug)]
pub struct MailboxData {
    /// Frame data
    pub frame: Frame,
    /// Mailbox number associated with this data
    pub mailbox_number: u8,
}

impl<Tx, Rx, const N: u8> CAN<Pins<Tx, Rx>, N>
where
    Tx: flexcan::Pin<Signal = flexcan::Tx, Module = Const<N>>,
    Rx: flexcan::Pin<Signal = flexcan::Rx, Module = Const<N>>,
{
    /// Creates a new CAN object.
    /// 1. Prepares the supplied TX and RX pins for flexcan operation
    /// 2. Initializes the CAN peripheral hardware + starts peripheral operation
    pub fn new(
        instance: ral::can::Instance<N>,
        mut tx: Tx,
        mut rx: Rx,
        clock_frequency: u32,
    ) -> Self {
        imxrt_iomuxc::flexcan::prepare(&mut tx);
        imxrt_iomuxc::flexcan::prepare(&mut rx);

        Self::init(instance, clock_frequency)
    }

    fn init(instance: ral::can::Instance<N>, clock_frequency: u32) -> Self {
        let mut can = CAN {
            reg: instance,
            _pins: PhantomData,
            _module: PhantomData,
            clock_frequency,
            _mailbox_reader_index: 0,
        };
        can.begin();
        can
    }
}

impl<P, const M: u8> CAN<P, M> {
    /// Number of FIFO RX mailboxes
    pub const NUMBER_FIFO_RX_MAILBOXES: u32 = 6;

    fn while_frozen<F: FnMut(&mut Self) -> R, R>(&mut self, mut act: F) -> R {
        let frz_flag_negate = ral::read_reg!(ral::can, self.reg, MCR, FRZACK == FRZACK_0);
        self.enter_freeze_mode();
        let res = act(self);
        if frz_flag_negate {
            self.exit_freeze_mode();
        }
        res
    }

    /// Initializes the CAN peripheral
    /// See section 43.8.1: FLEXCAN Initialization Sequence of the i.MX RT1060 Processor Reference Manual for details
    pub fn begin(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, MDIS: MDIS_0);

        self.enter_freeze_mode();

        ral::modify_reg!(ral::can, self.reg, CTRL1, LOM: LOM_1);
        ral::modify_reg!(ral::can, self.reg, MCR, FRZ: FRZ_1);
        while ral::read_reg!(ral::can, self.reg, MCR, LPMACK == LPMACK_1) {}

        self.soft_reset();

        while ral::read_reg!(ral::can, self.reg, MCR, FRZACK != FRZACK_1) {}

        ral::modify_reg!(
            ral::can,
            self.reg,
            MCR,
            SRXDIS: SRXDIS_1,
            IRMQ: IRMQ_1,
            AEN: AEN_1,
            LPRIOEN: LPRIOEN_1,
            SLFWAK: SLFWAK_1,
            WAKSRC: WAKSRC_1,
            WRNEN: WRNEN_1,
            WAKMSK: WAKMSK_1
        );

        ral::modify_reg!(ral::can, self.reg, MCR, |reg| reg & !0x8800);

        ral::modify_reg!(
            ral::can,
            self.reg,
            CTRL2,
            RRS: RRS_1,
            EACEN: EACEN_1,
            MRP: MRP_1
        );

        self.disable_fifo();
        self.exit_freeze_mode();
    }

    /// Returns the instance number
    pub fn instance_number(&self) -> u8 {
        M
    }

    /// Returns `true` if this is `can1`
    pub fn is_can1(&self) -> bool {
        M == 1
    }

    /// Returns `true` if this is `can2`
    pub fn is_can2(&self) -> bool {
        M == 2
    }

    /// Returns the base address of this peripheral
    pub(crate) fn base_address(&self) -> u32 {
        let addr: *const ral::can::RegisterBlock = &*self.reg;
        addr as u32
    }

    /// Relinquishes ownership of the instance
    pub fn free(self) -> ral::can::Instance<M> {
        self.reg
    }

    /// Performs a soft reset of the flexcan peripheral
    fn soft_reset(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, SOFTRST: SOFTRST_1);
        while ral::read_reg!(ral::can, self.reg, MCR, SOFTRST == SOFTRST_1) {}
    }

    /// Enters freeze mode
    /// See section 43.7.10.1: Freeze Mode of the i.MX RT1060 Processor Reference Manual for details
    fn enter_freeze_mode(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, FRZ: FRZ_1);
        ral::modify_reg!(ral::can, self.reg, MCR, HALT: HALT_1);
        while ral::read_reg!(ral::can, self.reg, MCR, FRZACK != FRZACK_1) {}
    }

    /// Exits freeze mode
    /// See section 43.7.10.1: Freeze Mode of the i.MX RT1060 Processor Reference Manual for details
    fn exit_freeze_mode(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, HALT: HALT_0);
        while ral::read_reg!(ral::can, self.reg, MCR, FRZACK != FRZACK_0) {}
    }

    /// Sets the [`MRP`](imxrt_ral::can::CTRL2::MRP) (Mailboxes Reception Priority) bit.
    pub fn set_mrp(&mut self, mrp: bool) {
        ral::modify_reg!(ral::can, self.reg, CTRL2, MRP: mrp as u32)
    }

    /// Sets the [`RRS`](imxrt_ral::can::CTRL2::RRS) (Remote Request Frame) bit.
    pub fn set_rrs(&mut self, rrs: bool) {
        ral::modify_reg!(ral::can, self.reg, CTRL2, RRS: rrs as u32)
    }

    /// Returns the clock frequency
    ///
    /// NOTE: This clock frequency is not derived from reading clocks.
    /// It is calculated based on the input clock frequency when the [`CAN`] object was created
    pub fn get_clock(&self) -> u32 {
        self.clock_frequency
    }

    /// Returns an Optional `\[u32; 3\]` lookup table for setting
    /// [`PROPSEG`](imxrt_ral::can::CTRL1::PROPSEG), [`PSEG1`](imxrt_ral::can::CTRL1::PSEG1), and [`PSEG2`](imxrt_ral::can::CTRL1::PSEG2) registers in [`Self::set_baud_rate()`]
    fn result_to_bit_table(&self, result: u8) -> Option<[u32; 3]> {
        match result {
            0 => Some([0, 0, 1]),
            1 => Some([1, 0, 1]),
            2 => Some([1, 1, 1]),
            3 => Some([2, 1, 1]),
            4 => Some([2, 2, 1]),
            5 => Some([2, 3, 1]),
            6 => Some([2, 3, 2]),
            7 => Some([2, 4, 2]),
            8 => Some([2, 5, 2]),
            9 => Some([2, 5, 3]),
            10 => Some([2, 6, 3]),
            11 => Some([2, 7, 3]),
            12 => Some([2, 7, 4]),
            13 => Some([3, 7, 4]),
            14 => Some([3, 7, 5]),
            15 => Some([4, 7, 5]),
            16 => Some([4, 7, 6]),
            17 => Some([5, 7, 6]),
            18 => Some([6, 7, 6]),
            19 => Some([6, 7, 7]),
            20 => Some([7, 7, 7]),
            _ => None,
        }
    }

    /// Sets the baud rate of the CAN peripheral.
    /// See section 44.4.9.8: Protocol Timing of the i.MX RT1060 Processor Reference Manual for details.
    pub fn set_baud_rate(&mut self, baud: u32) {
        fn calc_result(baud: u32, clock_freq: u32, divisor: u32) -> u32 {
            clock_freq / baud / (divisor + 1)
        }

        fn calc_error(baud: u32, clock_freq: u32, result: u32, divisor: u32) -> u32 {
            baud.abs_diff(clock_freq / (result * (divisor + 1)))
        }

        let clock_freq = self.get_clock();

        let mut divisor = 0;
        let mut best_divisor: u32 = 0;

        let mut result: u32 = calc_result(baud, clock_freq, divisor);

        let mut error = calc_error(baud, clock_freq, result, divisor);
        let mut best_error = error;

        self.while_frozen(|this| {
            while result > 5 {
                divisor += 1;
                result = calc_result(baud, clock_freq, divisor);
                if result <= 25 {
                    error = calc_error(baud, clock_freq, result, divisor);
                    if error < best_error {
                        best_error = error;
                        best_divisor = divisor;
                    }
                    if (error == best_error) && (result > 11) && (result < 19) {
                        best_error = error;
                        best_divisor = divisor;
                    }
                }
            }

            divisor = best_divisor;
            result = clock_freq / baud / (divisor + 1);

            if !(5..=25).contains(&result) || (best_error > 300) {
                return;
            };

            result -= 5;

            if let Some(t) = this.result_to_bit_table(result as u8) {
                modify_reg!(
                    ral::can,
                    this.reg,
                    CTRL1,
                    PROPSEG: t[0],
                    RJW: 1,
                    PSEG1: t[1],
                    PSEG2: t[2],
                    ERRMSK: ERRMSK_1,
                    LOM: LOM_0,
                    PRESDIV: divisor)
            };
        });
    }

    /// Set the max mailbox index
    pub fn set_max_mailbox(&mut self, last: u8) {
        let last = last.clamp(1, 64) - 1;
        self.while_frozen(|this| {
            let fifo_cleared = this.fifo_enabled();
            this.disable_fifo();
            this.write_iflag(this.read_iflag());
            ral::modify_reg!(ral::can, this.reg, MCR, MAXMB: last as u32);
            if fifo_cleared {
                this.set_fifo(true);
            }
        });
    }

    /// Get the max mailbox index
    fn get_max_mailbox(&self) -> u8 {
        ral::read_reg!(ral::can, self.reg, MCR, MAXMB) as u8
    }

    /// Write value to [`IFLAG1`](imxrt_ral::can::RegisterBlock::IFLAG1) / [`IFLAG2`](imxrt_ral::can::RegisterBlock::IFLAG2) registers
    fn write_iflag(&mut self, value: u64) {
        write_reg!(ral::can, self.reg, IFLAG1, value as u32);
        write_reg!(ral::can, self.reg, IFLAG2, (value >> 32) as u32);
    }

    /// Write bit to [`IFLAG1`](imxrt_ral::can::RegisterBlock::IFLAG1) / [`IFLAG2`](imxrt_ral::can::RegisterBlock::IFLAG2) register indicating mailbox interrupt
    fn write_iflag_bit(&mut self, mailbox_number: u8) {
        if mailbox_number < 32 {
            modify_reg!(ral::can, self.reg, IFLAG1, |reg| reg
                | 1_u32 << mailbox_number)
        } else {
            modify_reg!(ral::can, self.reg, IFLAG2, |reg| reg
                | 1_u32 << (mailbox_number - 32))
        }
    }

    /// Write bit to [`IMASK1`](imxrt_ral::can::RegisterBlock::IMASK1) register indicating masked mailbox interrupt
    fn write_imask_bit(&mut self, mailbox_number: u8, value: bool) {
        if mailbox_number < 32 {
            modify_reg!(ral::can, self.reg, IMASK1, |reg| reg
                | (value as u32) << mailbox_number)
        } else {
            modify_reg!(ral::can, self.reg, IMASK2, |reg| reg
                | (value as u32) << (mailbox_number - 32))
        }
    }

    /// Read [`IFLAG1`](imxrt_ral::can::RegisterBlock::IFLAG1) / [`IFLAG2`](imxrt_ral::can::RegisterBlock::IFLAG2) registers
    fn read_iflag(&self) -> u64 {
        (ral::read_reg!(ral::can, self.reg, IFLAG2) as u64) << 32
            | ral::read_reg!(ral::can, self.reg, IFLAG1) as u64
    }

    /// Read [`IMASK1`](imxrt_ral::can::RegisterBlock::IMASK1) / [`IMASK2`](imxrt_ral::can::RegisterBlock::IMASK2) registers
    fn read_imask(&self) -> u64 {
        (ral::read_reg!(ral::can, self.reg, IMASK2) as u64) << 32
            | ral::read_reg!(ral::can, self.reg, IMASK1) as u64
    }

    /// Write value to [`IMASK1`](imxrt_ral::can::RegisterBlock::IMASK1) / [`IMASK2`](imxrt_ral::can::RegisterBlock::IMASK2) registers
    fn write_imask(&mut self, value: u64) {
        write_reg!(ral::can, self.reg, IMASK1, value as u32);
        write_reg!(ral::can, self.reg, IMASK2, (value >> 32) as u32);
    }

    /// Set RX FIFO to
    /// 1. Enabled: `true`
    /// 2. Disabled: `false`
    pub fn set_fifo(&mut self, enabled: bool) {
        self.while_frozen(|this| {
            modify_reg!(ral::can, this.reg, MCR, RFEN: RFEN_0);
            this.write_imask(0);

            for i in 0..this.get_max_mailbox() {
                this.write_mailbox(i, Some(0), Some(0), Some([0x00; 8]));
                this.write_mailbox_rximr(i, Some(0x00));
            }

            write_reg!(ral::can, this.reg, RXMGMASK, 0);
            write_reg!(ral::can, this.reg, RXFGMASK, 0);

            this.write_iflag(this.read_iflag());

            let max_mailbox = this.get_max_mailbox();
            if enabled {
                modify_reg!(ral::can, this.reg, MCR, RFEN: RFEN_1);
                while ral::read_reg!(ral::can, this.reg, MCR, RFEN != RFEN_1) {}
                for i in this.mailbox_offset()..max_mailbox {
                    this.write_mailbox(
                        i,
                        Some(FlexCanMailboxCSCode::TxInactive.to_code_reg()),
                        None,
                        None,
                    );
                    this.enable_mailbox_interrupt(i, true);
                }
            } else {
                for i in 0..max_mailbox {
                    if i < max_mailbox / 2 {
                        let code = FlexCanMailboxCSCode::RxEmpty.to_code_reg() | 0x00400000 | {
                            if i < max_mailbox / 4 {
                                0
                            } else {
                                0x00200000
                            }
                        };
                        this.write_mailbox(i, Some(code), None, None);
                        let eacen = read_reg!(ral::can, this.reg, CTRL2, EACEN == EACEN_1);
                        let rximr = {
                            if eacen {
                                1_u32 << 30
                            } else {
                                0
                            }
                        };
                        this.write_mailbox_rximr(i, Some(rximr));
                    } else {
                        // set inactive
                        this.write_mailbox(
                            i,
                            Some(FlexCanMailboxCSCode::TxInactive.to_code_reg()),
                            Some(0),
                            Some([0x00; 8]),
                        );
                        this.enable_mailbox_interrupt(i, true);
                    }
                }
            }
        })
    }

    /// Wrapper around [`Self::set_fifo(true)`](Self::set_fifo())
    pub fn enable_fifo(&mut self) {
        self.set_fifo(true);
    }

    /// Wrapper around [`Self::set_fifo(false)`](Self::set_fifo())
    pub fn disable_fifo(&mut self) {
        self.set_fifo(false);
    }

    #[allow(rustdoc::private_intra_doc_links)]
    /// Wrapper around [`Self::set_fifo_filter_mask()`] which sets the filter mask to [`filter::FlexCanFlten::RejectAll`]
    pub fn set_fifo_reject_all(&mut self) {
        self.set_fifo_filter_mask(filter::FlexCanFlten::RejectAll)
    }

    #[allow(rustdoc::private_intra_doc_links)]
    /// Wrapper around [`Self::set_fifo_filter_mask()`] which sets the filter mask to [`filter::FlexCanFlten::AcceptAll`]
    pub fn set_fifo_accept_all(&mut self) {
        self.set_fifo_filter_mask(filter::FlexCanFlten::AcceptAll)
    }

    /// Set the FIFO filter mask
    fn set_fifo_filter_mask(&mut self, flten: filter::FlexCanFlten) {
        if !self.fifo_enabled() {
            return;
        }
        let max_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 8;
        self.while_frozen(|this| match flten {
            filter::FlexCanFlten::AcceptAll => {
                let offset = this.mailbox_offset();
                for i in 0..max_fifo_filters {
                    this.write_mailbox_idflt_tab(i as u8, Some(0x00));
                    if i < offset.clamp(0, 32) as u32 {
                        this.write_mailbox_rximr(i as u8, Some(0x00));
                    }
                }
                write_reg!(ral::can, this.reg, RXFGMASK, 0x00);
            }
            filter::FlexCanFlten::RejectAll => match read_reg!(ral::can, this.reg, MCR, IDAM) {
                ral::can::MCR::IDAM::RW::IDAM_0 => {
                    let offset = this.mailbox_offset();
                    for i in 0..max_fifo_filters {
                        this.write_mailbox_idflt_tab(i as u8, Some(0xFFFFFFFF));
                        if i < offset.clamp(0, 32) as u32 {
                            this.write_mailbox_rximr(i as u8, Some(0x3FFFFFFF));
                        }
                    }
                    write_reg!(ral::can, this.reg, RXFGMASK, 0x3FFFFFFF);
                }
                ral::can::MCR::IDAM::RW::IDAM_1 => {
                    let offset = this.mailbox_offset();
                    for i in 0..max_fifo_filters {
                        this.write_mailbox_idflt_tab(i as u8, Some(0xFFFFFFFF));
                        if i < offset.clamp(0, 32) as u32 {
                            this.write_mailbox_rximr(i as u8, Some(0x7FFF7FFF));
                        }
                    }
                    write_reg!(ral::can, this.reg, RXFGMASK, 0x7FFF7FFF);
                }
                ral::can::MCR::IDAM::RW::IDAM_2 => {
                    // not implemented
                }
                ral::can::MCR::IDAM::RW::IDAM_3 => {
                    // not implemented
                }
                _ => {}
            },
        })
    }

    /// Set the FIFO filter to [`filter`]
    pub fn set_fifo_filter(&mut self, filter: filter::FlexCanFilter) {
        if !self.fifo_enabled() {
            return;
        }
        let filter::FlexCanFilter {
            filter_id,
            id,
            ide,
            remote,
        } = filter;
        let max_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 8;
        if filter_id as u32 >= max_fifo_filters {
            return;
        }
        self.while_frozen(|this| match read_reg!(ral::can, this.reg, MCR, IDAM) {
            ral::can::MCR::IDAM::RW::IDAM_0 => {
                let mask: u32 = if ide != filter::FlexCanIde::Ext {
                    ((id ^ 0x7FF) << 19) | 0xC0000001
                } else {
                    ((id ^ 0x1FFFFFFF) << 1) | 0xC0000001
                };
                let mut filter: u32 = u32::from(ide == filter::FlexCanIde::Ext) << 30;
                filter |= u32::from(remote == filter::FlexCanIde::Rtr) << 31;
                filter |= if ide == filter::FlexCanIde::Ext {
                    (id & 0x1FFFFFFF) << 1
                } else {
                    ((id & 0x000007FF) << 18) << 1
                };
                this.write_mailbox_idflt_tab(filter_id, Some(filter));
                let offset = this.mailbox_offset();
                if filter_id < offset.clamp(0, 32) {
                    this.write_mailbox_rximr(filter_id, Some(mask));
                }
                write_reg!(ral::can, this.reg, RXFGMASK, 0x3FFFFFFF);
            }
            ral::can::MCR::IDAM::RW::IDAM_1 => {
                let mut mask: u32 = if ide != filter::FlexCanIde::Ext {
                    (id ^ 0x7FF) << 19
                } else {
                    (id ^ 0x1FFFFFFF) << 16
                } | if remote == filter::FlexCanIde::Rtr {
                    1 << 31
                } else {
                    0
                };
                mask |= if ide != filter::FlexCanIde::Ext {
                    (id ^ 0x7FF) << 3
                } else {
                    id ^ 0x1FFFFFFF
                } | if remote == filter::FlexCanIde::Rtr {
                    1 << 15
                } else {
                    0
                } & 0xFFFF;
                mask |= (1 << 30) | (1 << 14);
                let filter: u32 = u32::from(ide == filter::FlexCanIde::Ext) << 30
                    | u32::from(ide == filter::FlexCanIde::Ext) << 14
                    | u32::from(remote == filter::FlexCanIde::Rtr) << 31
                    | u32::from(remote == filter::FlexCanIde::Rtr) << 15
                    | (if ide == filter::FlexCanIde::Ext {
                        (id >> (29 - 14)) << 16
                    } else {
                        (id & 0x7FF) << 19
                    })
                    | (if ide == filter::FlexCanIde::Ext {
                        id >> (29 - 14)
                    } else {
                        (id & 0x7FF) << 3
                    });
                this.write_mailbox_idflt_tab(filter_id, Some(filter));
                let offset = this.mailbox_offset();
                if filter_id < offset.clamp(0, 32) {
                    this.write_mailbox_rximr(filter_id, Some(mask));
                }
                write_reg!(ral::can, this.reg, RXFGMASK, 0x7FFF7FFF);
            }
            ral::can::MCR::IDAM::RW::IDAM_2 => {
                // not implemented
            }
            ral::can::MCR::IDAM::RW::IDAM_3 => {
                // not implemented
            }
            _ => {}
        })
    }

    /// Set the FIFO interrupt to
    /// 1. Enabled: `true`
    /// 2. Disabled: `false`
    pub fn set_fifo_interrupt(&mut self, enabled: bool) {
        /* FIFO must be enabled first */
        if !self.fifo_enabled() {
            return;
        };
        /* FIFO interrupts already enabled */
        const FLEXCAN_IMASK1_BUF5M: u32 = 0x00000020;
        if (ral::read_reg!(ral::can, self.reg, IMASK1, BUFLM) & FLEXCAN_IMASK1_BUF5M) != 0 {
            return;
        };
        /* disable FIFO interrupt flags */
        modify_reg!(ral::can, self.reg, IMASK1, |reg| reg & !0xFF);
        /* enable FIFO interrupt */
        if enabled {
            const FLEXCAN_IMASK1_BUF5M: u32 = 0x00000020;
            modify_reg!(ral::can, self.reg, IMASK1, BUFLM: FLEXCAN_IMASK1_BUF5M);
        }
    }

    /// Returns whether or not the FIFO is enabled
    fn fifo_enabled(&self) -> bool {
        ral::read_reg!(ral::can, self.reg, MCR, RFEN == RFEN_1)
    }

    /// Returns the mailbox offset, accounting for FIFO mailbox usage
    fn mailbox_offset(&self) -> u8 {
        if self.fifo_enabled() {
            let max_mailbox = self.get_max_mailbox() as u32;
            let num_rx_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 2;
            let remaining_mailboxes = (max_mailbox - 6_u32 - num_rx_fifo_filters) as i32;
            /* return offset MB position after FIFO area */
            if remaining_mailboxes > max_mailbox as i32 {
                max_mailbox as u8
            } else {
                (max_mailbox as i32 - remaining_mailboxes) as u8
            }
        } else {
            /* return offset 0 since FIFO is disabled */
            0
        }
    }

    #[inline(always)]
    fn mailbox_number_to_address(&self, mailbox_number: u8) -> u32 {
        self.base_address() + 0x80_u32 + (mailbox_number as u32 * 0x10_u32)
    }

    #[inline(always)]
    fn mailbox_number_to_rximr_address(&self, mailbox_number: u8) -> u32 {
        self.base_address() + 0x880_u32 + (mailbox_number as u32 * 0x4_u32)
    }

    #[inline(always)]
    fn mailbox_number_to_idflt_tab_address(&self, mailbox_number: u8) -> u32 {
        self.base_address() + 0xE0_u32 + (mailbox_number as u32 * 0x4_u32)
    }

    #[inline(always)]
    fn read_mailbox_code(&mut self, mailbox_number: u8) -> u32 {
        let mailbox_addr = self.mailbox_number_to_address(mailbox_number);
        unsafe { core::ptr::read_volatile(mailbox_addr as *const u32) }
    }

    #[inline(always)]
    fn read_mailbox(&mut self, mailbox_number: u8) -> Option<MailboxData> {
        if (self.read_imask() & (1_u64 << mailbox_number)) != 0 {
            return None;
        }

        let code = self.read_mailbox_code(mailbox_number);
        let cr = CodeReg::new(code);
        let c = FlexCanMailboxCSCode::from_code_reg(code);

        let mailbox_addr = self.mailbox_number_to_address(mailbox_number);

        match c {
            // return None from a transmit mailbox
            c if c.is_tx_mailbox() => None,
            // full or overrun
            c if (c == FlexCanMailboxCSCode::RxFull) | (c == FlexCanMailboxCSCode::RxOverrun) => {
                let dlc = cr.dlc();
                let id =
                    unsafe { core::ptr::read_volatile((mailbox_addr + 0x4_u32) as *const u32) };
                let data0 =
                    unsafe { core::ptr::read_volatile((mailbox_addr + 0x8_u32) as *const u32) };
                // Only valid if the DLC is > 4
                let data1 = {
                    if dlc > 4 {
                        unsafe { core::ptr::read_volatile((mailbox_addr + 0xC_u32) as *const u32) }
                    } else {
                        0_u32
                    }
                };

                let mut data: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

                for i in 0..4 {
                    data[3 - i] = (data0 >> (8 * i)) as u8;
                }
                for i in 0..4 {
                    data[7 - i] = (data1 >> (8 * i)) as u8;
                }

                self.write_mailbox(
                    mailbox_number,
                    Some(FlexCanMailboxCSCode::RxEmpty.to_code_reg()),
                    None,
                    None,
                );
                read_reg!(ral::can, self.reg, TIMER);
                self.write_iflag_bit(mailbox_number);

                let frame = Frame::new_from_raw(code, id, data);

                Some(MailboxData {
                    frame,
                    mailbox_number,
                })
            }
            _ => None,
        }
    }

    /// Write the registers of a mailbox.
    #[inline(always)]
    fn write_mailbox(
        &self,
        mailbox_number: u8,
        code: Option<u32>,
        id: Option<u32>,
        data: Option<[u8; 8]>,
    ) {
        let mailbox_addr = self.mailbox_number_to_address(mailbox_number);
        if let Some(code) = code {
            unsafe { core::ptr::write_volatile((mailbox_addr) as *mut u32, code) };
        }
        if let Some(id) = id {
            unsafe { core::ptr::write_volatile((mailbox_addr + 0x4_u32) as *mut u32, id) };
        }
        if let Some(data) = data {
            let word0 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            let word1 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
            unsafe { core::ptr::write_volatile((mailbox_addr + 0x8_u32) as *mut u32, word0) };
            unsafe { core::ptr::write_volatile((mailbox_addr + 0xC_u32) as *mut u32, word1) };
        }
    }

    /// Write data to an available TX Mailbox.
    ///
    /// This will accept both standard and extended data and remote frames with any ID.
    ///
    /// In order to transmit a Can frame, the CPU must prepare a Message Buffer for
    /// transmission by executing the procedure found here.
    ///
    /// 1.  Check if the respective interruption bit is set and clear it.
    ///
    /// 2.  If the MB is active (transmission pending), write the ABORT code (0b1001) to the
    ///     CODE field of the Control and Status word to request an abortion of the
    ///     transmission. Wait for the corresponding IFLAG to be asserted by polling the IFLAG
    ///     register or by the interrupt request if enabled by the respective IMASK. Then read
    ///     back the CODE field to check if the transmission was aborted or transmitted (see
    ///     Transmission Abort Mechanism). If backwards compatibility is desired (MCR[AEN]
    ///     bit negated), just write the INACTIVE code (0b1000) to the CODE field to inactivate
    ///     the MB but then the pending frame may be transmitted without notification (see
    ///     Message Buffer Inactivation).
    ///
    /// 3.  Write the ID word.
    ///
    /// 4.  Write the data bytes.
    ///
    /// 5.  Write the DLC, Control and Code fields of the Control and Status word to activate
    ///     the MB.
    ///
    /// Once the MB is activated, it will participate into the arbitration process and eventually be
    /// transmitted according to its priority.
    #[inline(always)]
    fn write_tx_mailbox(&mut self, mailbox_number: u8, frame: &Frame) {
        // Check if the respective interruption bit is set and clear it.
        self.write_iflag_bit(mailbox_number);
        self.write_mailbox(
            mailbox_number,
            Some(FlexCanMailboxCSCode::TxInactive.to_code_reg()),
            None,
            None,
        );
        self.write_mailbox(
            mailbox_number,
            None,
            Some(frame.id.to_id_reg()),
            Some(frame.data.bytes),
        );
        self.write_mailbox(mailbox_number, Some(frame.to_tx_once_code()), None, None);
    }

    fn write_mailbox_rximr(&self, mailbox_number: u8, rximr: Option<u32>) {
        let mailbox_rximr_addr = self.mailbox_number_to_rximr_address(mailbox_number);
        if let Some(rximr) = rximr {
            unsafe { core::ptr::write_volatile((mailbox_rximr_addr) as *mut u32, rximr) };
        }
    }

    fn write_mailbox_idflt_tab(&self, mailbox_number: u8, idflt_tab: Option<u32>) {
        let mailbox_idflt_tab_addr = self.mailbox_number_to_idflt_tab_address(mailbox_number);
        if let Some(idflt_tab) = idflt_tab {
            unsafe { core::ptr::write_volatile((mailbox_idflt_tab_addr) as *mut u32, idflt_tab) };
        }
    }

    fn enable_mailbox_interrupt(&mut self, mailbox_number: u8, enabled: bool) {
        /* mailbox not available */
        if mailbox_number < self.mailbox_offset() {
            return;
        }
        if enabled {
            /* enable mailbox interrupt */
            self.write_imask_bit(mailbox_number, true);
            return;
        } else if let Some(d) = self.read_mailbox(mailbox_number) {
            if (d.frame.code.to_code_reg() & 0x0F000000) >> 3 != 0 {
                /* transmit interrupt keeper */
                self.write_imask_bit(mailbox_number, true);
                return;
            }
        }
        /* disable mailbox interrupt */
        self.write_imask_bit(mailbox_number, false);
    }

    /// Read Mailboxes, returning an optional [`MailboxData`] value containing any new Mailbox values.
    pub fn read_mailboxes(&mut self) -> Option<MailboxData> {
        let mut iflag: u64;
        let mut cycle_limit: u8 = 3;
        let offset = self.mailbox_offset();
        let max_mailbox = self.get_max_mailbox();
        while self._mailbox_reader_index <= max_mailbox {
            iflag = self.read_iflag();
            if iflag != 0 && (self._mailbox_reader_index >= (64 - iflag.leading_zeros() as u8)) {
                /* break from MSB's if unset, add 1 to prevent undefined behaviour in clz for 0 check */
                self._mailbox_reader_index = self.mailbox_offset();
                cycle_limit -= 1;
                if cycle_limit == 0 {
                    return None;
                }
            }
            if self.fifo_enabled() {
                /* FIFO is enabled, get only remaining RX (if any) */
                if self._mailbox_reader_index < offset {
                    /* go back to position end of fifo+filter region */
                    self._mailbox_reader_index = offset;
                }
            }
            if self._mailbox_reader_index >= max_mailbox {
                self._mailbox_reader_index = self.mailbox_offset();
                cycle_limit -= 1;
                if cycle_limit == 0 {
                    return None;
                }
            }
            if (self.read_imask() & (1_u64 << self._mailbox_reader_index)) != 0 {
                self._mailbox_reader_index += 1;
                continue; /* don't read interrupt enabled mailboxes */
            }
            if let Some(mailbox_data) = self.read_mailbox(self._mailbox_reader_index) {
                return Some(mailbox_data);
            }
            self._mailbox_reader_index += 1;
        }
        None
    }

    /// Handle Mailbox Interrupts.
    /// Returns an optional [`MailboxData`] containing mailbox data if the DMA is disabled.
    /// If DMA is enabled, the FIFO can't be handled in the ISR
    #[inline(always)]
    pub fn handle_interrupt(&mut self) -> Option<MailboxData> {
        let imask = self.read_imask();
        let iflag = self.read_iflag();

        /* if DMA is disabled, ONLY THEN can you handle FIFO in ISR */
        if self.fifo_enabled() & (imask & 0x00000020 != 0) & (iflag & 0x00000020 != 0) {
            if let Some(mailbox_data) = self.read_mailbox(0) {
                self.write_iflag_bit(5);
                if iflag & 0x00000040 != 0 {
                    self.write_iflag_bit(6);
                }
                if iflag & 0x00000080 != 0 {
                    self.write_iflag_bit(7);
                }
                return Some(mailbox_data);
            }
        }

        None
    }

    /// Transmit a Frame.
    ///
    /// Returns an error if no TX mailboxes are available
    #[inline(always)]
    pub fn transmit(&mut self, frame: &Frame) -> nb::Result<(), Error> {
        for i in self.mailbox_offset()..self.get_max_mailbox() {
            if let FlexCanMailboxCSCode::TxInactive =
                FlexCanMailboxCSCode::from_code_reg(self.read_mailbox_code(i))
            {
                self.write_tx_mailbox(i, frame);
                return Ok(());
            }
        }
        Err(nb::Error::Other(Error::NoTxMailbox))
    }
}
