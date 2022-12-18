mod embedded_hal;
pub mod filter;
mod frame;
mod id;

pub use frame::{CodeReg, Data, FlexCanMailboxCSCode, Frame, IdReg};
pub use id::{ExtendedId, Id, StandardId};
use ral::{modify_reg, read_reg, write_reg};

use crate::ccm;
use crate::iomuxc::consts::{Unsigned, U1, U2};
use crate::iomuxc::flexcan;
use crate::ral;

use core::convert::Infallible;
use core::marker::PhantomData;

/// Error that indicates that no received frames are available
/// in the FlexCAN mailboxes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoDataError {
    _priv: (),
}

/// Unclocked Can modules
///
/// The `Unclocked` struct represents the unconfigured Can peripherals.
/// Once clocked, you'll have the ability to build Can peripherals from the
/// compatible processor pins.
pub struct Unclocked {
    pub(crate) can1: ral::can::Instance<1>,
    pub(crate) can2: ral::can::Instance<2>,
}

impl Unclocked {
    /// Enable clocks to all Can modules, returning a builder for the two Can modules.
    pub fn clock(
        self,
        // handle: &mut ccm::Handle,
        handle: &mut ral::ccm::RegisterBlock,
        clock_select: ccm::can::ClockSelect,
        divider: ccm::can::PrescalarSelect,
    ) -> (Builder<1>, Builder<2>) {
        // let (ccm, _) = handle.raw();
        let ccm = handle;
        // First, disable clocks
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR1,
            CG0: 0,
            CG1: 0,
            CG2: 0,
            CG3: 0
        );

        let clk_sel = match clock_select {
            ccm::can::ClockSelect::OSC => ral::ccm::CSCMR2::CAN_CLK_SEL::RW::CAN_CLK_SEL_1,
        };

        // Select clock, and commit prescalar
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CSCMR2,
            CAN_CLK_PODF: ral::ccm::CSCMR2::CAN_CLK_PODF::RW::DIVIDE_1,
            CAN_CLK_SEL: clk_sel
        );

        // Enable clocks
        ral::modify_reg!(
            ral::ccm,
            ccm,
            CCGR1,
            CG0: 0b11,
            CG1: 0b11,
            CG2: 0b11,
            CG3: 0b11
        );

        let source_clock = clock_select as u32 / divider as u32;
        (
            Builder::new(source_clock, self.can1),
            Builder::new(source_clock, self.can2),
        )
    }
}

/// A Can builder that can build a Can peripheral
pub struct Builder<const M: u8> {
    _module: PhantomData<ral::can::Instance<M>>,
    reg: ral::can::Instance<M>,
    clock_frequency: u32,
}

impl<const M: u8> Builder<M>
{
    fn new(clock_frequency: u32, reg: ral::can::Instance<M>) -> Self {
        Builder {
            _module: PhantomData,
            reg,
            clock_frequency,
        }
    }

    /// Builds a Can peripheral. The return
    /// is a configured FlexCan peripheral running at 24MHz.
    pub fn build<TX, RX>(self, mut tx: TX, mut rx: RX) -> CAN<M>
    where
        TX: flexcan::Pin<Module = ral::can::Instance<M>, Signal = flexcan::Tx>,
        RX: flexcan::Pin<Module = ral::can::Instance<M>, Signal = flexcan::Rx>,
    {
        imxrt_iomuxc::flexcan::prepare(&mut tx);
        imxrt_iomuxc::flexcan::prepare(&mut rx);

        CAN::new(self.clock_frequency, self.reg)
    }
}

/// A Can master
///
pub struct CAN<const M: u8> {
    reg: ral::can::Instance<M>,
    _module: PhantomData<ral::can::RegisterBlock>,
    clock_frequency: u32,
    _mailbox_reader_index: u8,
}

#[derive(Debug)]
pub struct MailboxData {
    pub frame: Frame,
    pub mailbox_number: u8,
}

impl<const M: u8> CAN<M>
{
    pub const NUMBER_FIFO_RX_MAILBOXES: u32 = 6;

    fn new(clock_frequency: u32, reg: ral::can::Instance<M>) -> Self {
        let mut can = CAN {
            reg,
            _module: PhantomData,
            clock_frequency,
            _mailbox_reader_index: 0,
        };
        can.begin();
        can
    }

    pub fn instance(&mut self) -> &mut ral::can::Instance<M> {
        &mut self.reg
    }

    pub fn print_registers(&self) {
        // log::info!("MCR: {:X}", ral::read_reg!(ral::can, self.reg, MCR));
        // log::info!("CTRL1: {:X}", ral::read_reg!(ral::can, self.reg, CTRL1));
        // log::info!("CTRL2: {:X}", ral::read_reg!(ral::can, self.reg, CTRL2));
        // log::info!(
            // "RXMGMASK: {:X}",
            // ral::read_reg!(ral::can, self.reg, RXMGMASK)
        // );
        // log::info!(
            // "RXFGMASK: {:X}",
            // ral::read_reg!(ral::can, self.reg, RXFGMASK)
        // );

        let max_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 8;

        for mailbox_number in 0..max_fifo_filters {
            let mailbox_idflt_tab_addr =
                self.mailbox_number_to_idflt_tab_address(mailbox_number as u8);
            let idflt_tab =
                unsafe { core::ptr::read_volatile((mailbox_idflt_tab_addr) as *mut u32) };
            // log::info!(
                // "IDFLT_TAB[{}, {:X}]: {:X}",
                // mailbox_number,
                // mailbox_idflt_tab_addr,
                // idflt_tab
            // );
        }
    }

    fn while_frozen<F: FnMut(&mut Self) -> R, R>(&mut self, mut act: F) -> R {
        let frz_flag_negate = ral::read_reg!(ral::can, self.reg, MCR, FRZACK == FRZACK_0);
        self.enter_freeze_mode();
        let res = act(self);
        if frz_flag_negate {
            self.exit_freeze_mode();
        }
        res
    }

    pub fn begin(&mut self) {
        self.set_ccm_ccg();

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

    pub fn instance_number(&self) -> u8 {
        M
    }

    pub fn is_can1(&self) -> bool {
        M == 1
    }

    pub fn is_can2(&self) -> bool {
        M  == 2
    }

    pub fn base_address(&self) -> u32 {
        let addr: *const ral::can::RegisterBlock = &*self.reg;
        addr as u32
    }

    pub fn free(self) -> ral::can::Instance<M> {
        self.reg
    }

    fn soft_reset(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, SOFTRST: SOFTRST_1);
        while ral::read_reg!(ral::can, self.reg, MCR, SOFTRST == SOFTRST_1) {}
    }

    fn enter_freeze_mode(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, FRZ: FRZ_1);
        ral::modify_reg!(ral::can, self.reg, MCR, HALT: HALT_1);
        while ral::read_reg!(ral::can, self.reg, MCR, FRZACK != FRZACK_1) {}
    }

    fn exit_freeze_mode(&mut self) {
        ral::modify_reg!(ral::can, self.reg, MCR, HALT: HALT_0);
        while ral::read_reg!(ral::can, self.reg, MCR, FRZACK != FRZACK_0) {}
    }

    pub fn set_mrp(&mut self, mrp: bool) {
        ral::modify_reg!(ral::can, self.reg, CTRL2, MRP: mrp as u32)
    }

    pub fn set_rrs(&mut self, rrs: bool) {
        ral::modify_reg!(ral::can, self.reg, CTRL2, RRS: rrs as u32)
    }

    fn set_ccm_ccg(&mut self) {
        match self.instance_number() {
            1 => {
                unsafe { modify_reg!(ral::ccm, CCM, CCGR0, |reg| reg | 0x3C000) };
            }
            2 => {
                unsafe { modify_reg!(ral::ccm, CCM, CCGR0, |reg| reg | 0x3C0000) };
            }
            u => {
                // log::error!("Invalid Can instance (set_ccm_ccg): {:?}", u);
            }
        }
    }

    pub fn get_clock(&self) -> u32 {
        self.clock_frequency
    }

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

    pub fn set_baud_rate(&mut self, baud: u32) {
        let clock_freq = 24_000_000;

        let mut divisor = 0;
        let mut best_divisor: u32 = 0;
        let mut result: u32 = clock_freq / baud / (divisor + 1);
        let mut error: i16 = (baud - (clock_freq / (result * (divisor + 1)))) as i16;
        let mut best_error = error;

        self.while_frozen(|this| {
            while result > 5 {
                divisor += 1;
                result = clock_freq / baud / (divisor + 1);
                if result <= 25 {
                    error = (baud - (clock_freq / (result * (divisor + 1)))) as i16;
                    if error < 0 {
                        error = -1 * error;
                    }
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

            if !(result < 5) || (result > 25) || (best_error > 300) {
                result -= 5;

                match this.result_to_bit_table(result as u8) {
                    Some(t) => {
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
                    }
                    _ => {}
                }
            }
        });
    }

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

    fn get_max_mailbox(&self) -> u8 {
        ral::read_reg!(ral::can, self.reg, MCR, MAXMB) as u8
    }

    fn write_iflag(&mut self, value: u64) {
        write_reg!(ral::can, self.reg, IFLAG1, value as u32);
        write_reg!(ral::can, self.reg, IFLAG2, (value >> 32) as u32);
    }

    fn write_iflag_bit(&mut self, mailbox_number: u8) {
        if mailbox_number < 32 {
            modify_reg!(ral::can, self.reg, IFLAG1, |reg| reg
                | 1_u32 << mailbox_number)
        } else {
            modify_reg!(ral::can, self.reg, IFLAG2, |reg| reg
                | 1_u32 << (mailbox_number - 32))
        }
    }

    fn write_imask_bit(&mut self, mailbox_number: u8, value: bool) {
        if mailbox_number < 32 {
            modify_reg!(ral::can, self.reg, IMASK1, |reg| reg
                | (value as u32) << mailbox_number)
        } else {
            modify_reg!(ral::can, self.reg, IMASK2, |reg| reg
                | (value as u32) << (mailbox_number - 32))
        }
    }

    fn read_iflag(&self) -> u64 {
        (ral::read_reg!(ral::can, self.reg, IFLAG2) as u64) << 32
            | ral::read_reg!(ral::can, self.reg, IFLAG1) as u64
    }

    fn read_imask(&self) -> u64 {
        (ral::read_reg!(ral::can, self.reg, IMASK2) as u64) << 32
            | ral::read_reg!(ral::can, self.reg, IMASK1) as u64
    }

    fn write_imask(&mut self, value: u64) {
        write_reg!(ral::can, self.reg, IMASK1, value as u32);
        write_reg!(ral::can, self.reg, IMASK2, (value >> 32) as u32);
    }

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
                        let rximr = 0_32 | {
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

    pub fn enable_fifo(&mut self) {
        self.set_fifo(true);
    }

    pub fn disable_fifo(&mut self) {
        self.set_fifo(false);
    }

    pub fn set_fifo_reject_all(&mut self) {
        self.set_fifo_filter_mask(filter::FlexCanFlten::RejectAll)
    }

    pub fn set_fifo_accept_all(&mut self) {
        self.set_fifo_filter_mask(filter::FlexCanFlten::AcceptAll)
    }

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

    pub fn set_fifo_filter(
        &mut self,
        filter_id: u8,
        id: u32,
        ide: filter::FlexCanIde,
        remote: filter::FlexCanIde,
    ) {
        if !self.fifo_enabled() {
            return;
        }
        let max_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 8;
        if filter_id as u32 >= max_fifo_filters {
            return;
        }
        self.while_frozen(|this| match read_reg!(ral::can, this.reg, MCR, IDAM) {
            ral::can::MCR::IDAM::RW::IDAM_0 => {
                let mask: u32 = if ide != filter::FlexCanIde::Ext {
                    ((((id) ^ (id)) ^ 0x7FF) << 19) | 0xC0000001
                } else {
                    ((((id) ^ (id)) ^ 0x1FFFFFFF) << 1) | 0xC0000001
                };
                let mut filter: u32 = (if ide == filter::FlexCanIde::Ext { 1 } else { 0 } << 30);
                filter |= if remote == filter::FlexCanIde::Rtr {
                    1
                } else {
                    0
                } << 31;
                filter |= if ide == filter::FlexCanIde::Ext {
                    (id & 0x1FFFFFFF) << 1
                } else {
                    ((id & 0x000007FF) << 18) << 1
                };
                this.write_mailbox_idflt_tab(filter_id, Some(filter));
                let offset = this.mailbox_offset();
                if filter_id < offset.clamp(0, 32) as u8 {
                    this.write_mailbox_rximr(filter_id, Some(mask));
                }
                write_reg!(ral::can, this.reg, RXFGMASK, 0x3FFFFFFF);
            }
            ral::can::MCR::IDAM::RW::IDAM_1 => {
                let mut mask: u32 = if ide != filter::FlexCanIde::Ext {
                    (((id) ^ (id)) ^ 0x7FF) << 19
                } else {
                    (((id) ^ (id)) ^ 0x1FFFFFFF) << 16
                } | if remote == filter::FlexCanIde::Rtr {
                    1 << 31
                } else {
                    0
                };
                mask |= if ide != filter::FlexCanIde::Ext {
                    (((id) ^ (id)) ^ 0x7FF) << 3
                } else {
                    (((id) ^ (id)) ^ 0x1FFFFFFF) << 0
                } | if remote == filter::FlexCanIde::Rtr {
                    1 << 15
                } else {
                    0
                } & 0xFFFF;
                mask |= (1 << 30) | (1 << 14);
                let filter: u32 = (if ide == filter::FlexCanIde::Ext { 1 } else { 0 } << 30)
                    | (if ide == filter::FlexCanIde::Ext { 1 } else { 0 } << 14)
                    | (if remote == filter::FlexCanIde::Rtr {
                        1
                    } else {
                        0
                    } << 31)
                    | (if remote == filter::FlexCanIde::Rtr {
                        1
                    } else {
                        0
                    } << 15)
                    | (if ide == filter::FlexCanIde::Ext {
                        (id >> (29 - 14)) << 16
                    } else {
                        (id & 0x7FF) << 19
                    })
                    | (if ide == filter::FlexCanIde::Ext {
                        (id >> (29 - 14)) << 0
                    } else {
                        (id & 0x7FF) << 3
                    });
                this.write_mailbox_idflt_tab(filter_id, Some(filter));
                let offset = this.mailbox_offset();
                if filter_id < offset.clamp(0, 32) as u8 {
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

    pub fn set_fifo_filter_2(&mut self, filter: filter::FlexCanFilter) {
        self.set_fifo_filter(filter.filter_id, filter.id, filter.ide, filter.remote)
    }

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

    fn fifo_enabled(&self) -> bool {
        ral::read_reg!(ral::can, self.reg, MCR, RFEN == RFEN_1)
    }

    fn mailbox_offset(&self) -> u8 {
        if self.fifo_enabled() {
            let max_mailbox = self.get_max_mailbox() as u32;
            let num_rx_fifo_filters = (read_reg!(ral::can, self.reg, CTRL2, RFFN) + 1) * 2;
            let remaining_mailboxes = max_mailbox - 6_u32 - num_rx_fifo_filters;
            /* return offset MB position after FIFO area */
            if max_mailbox < max_mailbox - remaining_mailboxes {
                max_mailbox as u8
            } else {
                (max_mailbox - remaining_mailboxes) as u8
            }
        } else {
            /* return offset 0 since FIFO is disabled */
            0
        }
    }

    #[allow(dead_code)]
    fn read_fifo(&self) -> Option<()> {
        // if FIFO enabled and interrupt not enabled
        if !self.fifo_enabled() {
            return None;
        };
        // FIFO interrupt enabled, polling blocked
        if (ral::read_reg!(ral::can, self.reg, IMASK1) & (0x00000020 as u32)) != 0 {
            return None;
        }
        // message available
        if (ral::read_reg!(ral::can, self.reg, IFLAG1) & (0x00000020 as u32)) != 0 {
            return None;
        }
        Some(())
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
        let c = match FlexCanMailboxCSCode::from_code_reg(code) {
            Ok(c) => Some(c),
            Err(_e) => None,
        };

        let mailbox_addr = self.mailbox_number_to_address(mailbox_number);

        match c {
            // return None from a transmit mailbox
            Some(c) if c.is_tx_mailbox() => None,
            // full or overrun
            Some(c)
                if (c == FlexCanMailboxCSCode::RxFull) | (c == FlexCanMailboxCSCode::RxOverrun) =>
            {
                let id =
                    unsafe { core::ptr::read_volatile((mailbox_addr + 0x4_u32) as *const u32) };
                let data0 =
                    unsafe { core::ptr::read_volatile((mailbox_addr + 0x8_u32) as *const u32) };
                let data1 =
                    unsafe { core::ptr::read_volatile((mailbox_addr + 0xC_u32) as *const u32) };

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
            unsafe { core::ptr::write_volatile((mailbox_addr + 0_u32) as *mut u32, code) };
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
    /// 1. Check if the respective interruption bit is set and clear it.
    ///
    /// 2. If the MB is active (transmission pending), write the ABORT code (0b1001) to the
    /// CODE field of the Control and Status word to request an abortion of the
    /// transmission. Wait for the corresponding IFLAG to be asserted by polling the IFLAG
    /// register or by the interrupt request if enabled by the respective IMASK. Then read
    /// back the CODE field to check if the transmission was aborted or transmitted (see
    /// Transmission Abort Mechanism). If backwards compatibility is desired (MCR[AEN]
    /// bit negated), just write the INACTIVE code (0b1000) to the CODE field to inactivate
    /// the MB but then the pending frame may be transmitted without notification (see
    /// Message Buffer Inactivation).
    ///
    /// 3. Write the ID word.
    ///
    /// 4. Write the data bytes.
    ///
    /// 5. Write the DLC, Control and Code fields of the Control and Status word to activate
    /// the MB.
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
        } else {
            match self.read_mailbox(mailbox_number) {
                Some(d) => {
                    if (d.frame.code.to_code_reg() & 0x0F000000) >> 3 != 0 {
                        /* transmit interrupt keeper */
                        self.write_imask_bit(mailbox_number, true);
                        return;
                    }
                }
                _ => {}
            }
        }
        /* disable mailbox interrupt */
        self.write_imask_bit(mailbox_number, false);
        return;
    }

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
            match self.read_mailbox(self._mailbox_reader_index) {
                Some(mailbox_data) => {
                    return Some(mailbox_data);
                }
                _ => {}
            }
            self._mailbox_reader_index += 1;
        }
        None
    }

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

    #[inline(always)]
    pub fn transmit(&mut self, frame: &Frame) -> nb::Result<(), Infallible> {
        for i in self.mailbox_offset()..self.get_max_mailbox() {
            if let Ok(FlexCanMailboxCSCode::TxInactive) =
                FlexCanMailboxCSCode::from_code_reg(self.read_mailbox_code(i))
            {
                self.write_tx_mailbox(i, frame);
                return Ok(());
            }
        }
        Ok(())
    }
}

/// Interface to the Can transmitter part.
pub struct Tx<I> {
    _can: PhantomData<I>,
}
