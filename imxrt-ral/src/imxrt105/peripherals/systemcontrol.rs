#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System Control Block
//!
//! Used by: imxrt1051, imxrt1052

use crate::{RORegister, RWRegister, UnsafeWORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Auxiliary Control Register,
pub mod ACTLR {

    /// Disables folding of IT instructions.
    pub mod DISFOLD {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISFOLD_0: u32 = 0b0;
        }
    }

    /// Disables FPU exception outputs.
    pub mod FPEXCODIS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const FPEXCODIS_0: u32 = 0b0;

            /// 0b1: FPU exception outputs are disabled.
            pub const FPEXCODIS_1: u32 = 0b1;
        }
    }

    /// Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions.
    pub mod DISRAMODE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISRAMODE_0: u32 = 0b0;

            /// 0b1: Dynamic disabled.
            pub const DISRAMODE_1: u32 = 0b1;
        }
    }

    /// Disables ITM and DWT ATB flush.
    pub mod DISITMATBFLUSH {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: ITM and DWT ATB flush disabled, this bit is always 1.
            pub const DISITMATBFLUSH_1: u32 = 0b1;
        }
    }

    /// Disables BTAC read.
    pub mod DISBTACREAD {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISBTACREAD_0: u32 = 0b0;

            /// 0b1: BTAC is not used and only static branch prediction can occur.
            pub const DISBTACREAD_1: u32 = 0b1;
        }
    }

    /// Disables BTAC allocate.
    pub mod DISBTACALLOC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISBTACALLOC_0: u32 = 0b0;

            /// 0b1: No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated.
            pub const DISBTACALLOC_1: u32 = 0b1;
        }
    }

    /// Disables critical AXI Read-Under-Read.
    pub mod DISCRITAXIRUR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISCRITAXIRUR_0: u32 = 0b0;

            /// 0b1: An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set.
            pub const DISCRITAXIRUR_1: u32 = 0b1;
        }
    }

    /// Disables dual-issued.
    pub mod DISDI {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Normal operation.
            pub const DISDI_0: u32 = 0b00000;

            /// 0b00001: Nothing can be dual-issued when this instruction type is in channel 0.
            pub const DISDI_1: u32 = 0b00001;
        }
    }

    /// Disables dual-issued.
    pub mod DISISSCH1 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (5 bits: 0b11111 << 21)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Normal operation.
            pub const DISISSCH1_0: u32 = 0b00000;

            /// 0b00001: Nothing can be dual-issued when this instruction type is in channel 1.
            pub const DISISSCH1_1: u32 = 0b00001;
        }
    }

    /// Disables dynamic allocation of ADD and SUB instructions
    pub mod DISDYNADD {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation. Some ADD and SUB instrctions are resolved in EX1.
            pub const DISDYNADD_0: u32 = 0b0;

            /// 0b1: All ADD and SUB instructions are resolved in EX2.
            pub const DISDYNADD_1: u32 = 0b1;
        }
    }

    /// Disables critical AXI read-under-write
    pub mod DISCRITAXIRUW {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation. This is backwards compatible with r0.
            pub const DISCRITAXIRUW_0: u32 = 0b0;

            /// 0b1: AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete.
            pub const DISCRITAXIRUW_1: u32 = 0b1;
        }
    }

    /// Disables critical AXI read-under-write
    pub mod DISFPUISSOPT {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation.
            pub const DISFPUISSOPT_0: u32 = 0b0;
        }
    }
}

/// CPUID Base Register
pub mod CPUID {

    /// Indicates patch release: 0x0 = Patch 0
    pub mod REVISION {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates part number
    pub mod PARTNO {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ARCHITECTURE
    pub mod ARCHITECTURE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates processor revision: 0x2 = Revision 2
    pub mod VARIANT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Implementer code
    pub mod IMPLEMENTER {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Control and State Register
pub mod ICSR {

    /// Active exception number
    pub mod VECTACTIVE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates whether there are preempted active exceptions
    pub mod RETTOBASE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: there are preempted active exceptions to execute
            pub const RETTOBASE_0: u32 = 0b0;

            /// 0b1: there are no active exceptions, or the currently-executing exception is the only active exception
            pub const RETTOBASE_1: u32 = 0b1;
        }
    }

    /// Exception number of the highest priority pending enabled exception
    pub mod VECTPENDING {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (9 bits: 0x1ff << 12)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt pending flag, excluding NMI and Faults
    pub mod ISRPENDING {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No external interrupt pending.
            pub const ISRPENDING_0: u32 = 0b0;

            /// 0b1: External interrupt pending.
            pub const ISRPENDING_1: u32 = 0b1;
        }
    }

    /// SysTick exception clear-pending bit
    pub mod PENDSTCLR {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no effect
            pub const PENDSTCLR_0: u32 = 0b0;

            /// 0b1: removes the pending state from the SysTick exception
            pub const PENDSTCLR_1: u32 = 0b1;
        }
    }

    /// SysTick exception set-pending bit
    pub mod PENDSTSET {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: write: no effect; read: SysTick exception is not pending
            pub const PENDSTSET_0: u32 = 0b0;

            /// 0b1: write: changes SysTick exception state to pending; read: SysTick exception is pending
            pub const PENDSTSET_1: u32 = 0b1;
        }
    }

    /// PendSV clear-pending bit
    pub mod PENDSVCLR {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no effect
            pub const PENDSVCLR_0: u32 = 0b0;

            /// 0b1: removes the pending state from the PendSV exception
            pub const PENDSVCLR_1: u32 = 0b1;
        }
    }

    /// PendSV set-pending bit
    pub mod PENDSVSET {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: write: no effect; read: PendSV exception is not pending
            pub const PENDSVSET_0: u32 = 0b0;

            /// 0b1: write: changes PendSV exception state to pending; read: PendSV exception is pending
            pub const PENDSVSET_1: u32 = 0b1;
        }
    }

    /// NMI set-pending bit
    pub mod NMIPENDSET {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: write: no effect; read: NMI exception is not pending
            pub const NMIPENDSET_0: u32 = 0b0;

            /// 0b1: write: changes NMI exception state to pending; read: NMI exception is pending
            pub const NMIPENDSET_1: u32 = 0b1;
        }
    }
}

/// Vector Table Offset Register
pub mod VTOR {

    /// Vector table base offset
    pub mod TBLOFF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (25 bits: 0x1ffffff << 7)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Application Interrupt and Reset Control Register
pub mod AIRCR {

    /// Writing 1 to this bit causes a local system reset
    pub mod VECTRESET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No change
            pub const VECTRESET_0: u32 = 0b0;

            /// 0b1: Causes a local system reset
            pub const VECTRESET_1: u32 = 0b1;
        }
    }

    /// Writing 1 to this bit clears all active state information for fixed and configurable exceptions.
    pub mod VECTCLRACTIVE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No change
            pub const VECTCLRACTIVE_0: u32 = 0b0;

            /// 0b1: Clears all active state information for fixed and configurable exceptions
            pub const VECTCLRACTIVE_1: u32 = 0b1;
        }
    }

    /// System reset request
    pub mod SYSRESETREQ {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no system reset request
            pub const SYSRESETREQ_0: u32 = 0b0;

            /// 0b1: asserts a signal to the outer system that requests a reset
            pub const SYSRESETREQ_1: u32 = 0b1;
        }
    }

    /// Interrupt priority grouping field. This field determines the split of group priority from subpriority.
    pub mod PRIGROUP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data endianness
    pub mod ENDIANNESS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Little-endian
            pub const ENDIANNESS_0: u32 = 0b0;

            /// 0b1: Big-endian
            pub const ENDIANNESS_1: u32 = 0b1;
        }
    }

    /// Register key
    pub mod VECTKEY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Control Register
pub mod SCR {

    /// Indicates sleep-on-exit when returning from Handler mode to Thread mode
    pub mod SLEEPONEXIT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: o not sleep when returning to Thread mode
            pub const SLEEPONEXIT_0: u32 = 0b0;

            /// 0b1: enter sleep, or deep sleep, on return from an ISR
            pub const SLEEPONEXIT_1: u32 = 0b1;
        }
    }

    /// Controls whether the processor uses sleep or deep sleep as its low power mode
    pub mod SLEEPDEEP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: sleep
            pub const SLEEPDEEP_0: u32 = 0b0;

            /// 0b1: deep sleep
            pub const SLEEPDEEP_1: u32 = 0b1;
        }
    }

    /// Send Event on Pending bit
    pub mod SEVONPEND {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded
            pub const SEVONPEND_0: u32 = 0b0;

            /// 0b1: enabled events and all interrupts, including disabled interrupts, can wakeup the processor
            pub const SEVONPEND_1: u32 = 0b1;
        }
    }
}

/// Configuration and Control Register
pub mod CCR {

    /// Indicates how the processor enters Thread mode
    pub mod NONBASETHRDENA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: processor can enter Thread mode only when no exception is active
            pub const NONBASETHRDENA_0: u32 = 0b0;

            /// 0b1: processor can enter Thread mode from any level under the control of an EXC_RETURN value
            pub const NONBASETHRDENA_1: u32 = 0b1;
        }
    }

    /// Enables unprivileged software access to the STIR
    pub mod USERSETMPEND {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: disable
            pub const USERSETMPEND_0: u32 = 0b0;

            /// 0b1: enable
            pub const USERSETMPEND_1: u32 = 0b1;
        }
    }

    /// Enables unaligned access traps
    pub mod UNALIGN_TRP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: do not trap unaligned halfword and word accesses
            pub const UNALIGN_TRP_0: u32 = 0b0;

            /// 0b1: trap unaligned halfword and word accesses
            pub const UNALIGN_TRP_1: u32 = 0b1;
        }
    }

    /// Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0
    pub mod DIV_0_TRP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: do not trap divide by 0
            pub const DIV_0_TRP_0: u32 = 0b0;

            /// 0b1: trap divide by 0
            pub const DIV_0_TRP_1: u32 = 0b1;
        }
    }

    /// Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.
    pub mod BFHFNMIGN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: data bus faults caused by load and store instructions cause a lock-up
            pub const BFHFNMIGN_0: u32 = 0b0;

            /// 0b1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions
            pub const BFHFNMIGN_1: u32 = 0b1;
        }
    }

    /// Indicates stack alignment on exception entry
    pub mod STKALIGN {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 4-byte aligned
            pub const STKALIGN_0: u32 = 0b0;

            /// 0b1: 8-byte aligned
            pub const STKALIGN_1: u32 = 0b1;
        }
    }

    /// Enables L1 data cache.
    pub mod DC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: L1 data cache disabled
            pub const DC_0: u32 = 0b0;

            /// 0b1: L1 data cache enabled
            pub const DC_1: u32 = 0b1;
        }
    }

    /// Enables L1 instruction cache.
    pub mod IC {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: L1 instruction cache disabled
            pub const IC_0: u32 = 0b0;

            /// 0b1: L1 instruction cache enabled
            pub const IC_1: u32 = 0b1;
        }
    }

    /// Always reads-as-one. It indicates branch prediction is enabled.
    pub mod BP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Handler Priority Register 1
pub mod SHPR1 {

    /// Priority of system handler 4, MemManage
    pub mod PRI_4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priority of system handler 5, BusFault
    pub mod PRI_5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priority of system handler 6, UsageFault
    pub mod PRI_6 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Handler Priority Register 2
pub mod SHPR2 {

    /// Priority of system handler 11, SVCall
    pub mod PRI_11 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Handler Priority Register 3
pub mod SHPR3 {

    /// Priority of system handler 14, PendSV
    pub mod PRI_14 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Priority of system handler 15, SysTick exception
    pub mod PRI_15 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System Handler Control and State Register
pub mod SHCSR {

    /// MemManage exception active bit
    pub mod MEMFAULTACT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const MEMFAULTACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const MEMFAULTACT_1: u32 = 0b1;
        }
    }

    /// BusFault exception active bit
    pub mod BUSFAULTACT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const BUSFAULTACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const BUSFAULTACT_1: u32 = 0b1;
        }
    }

    /// UsageFault exception active bit
    pub mod USGFAULTACT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const USGFAULTACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const USGFAULTACT_1: u32 = 0b1;
        }
    }

    /// SVCall active bit
    pub mod SVCALLACT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const SVCALLACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const SVCALLACT_1: u32 = 0b1;
        }
    }

    /// Debug monitor active bit
    pub mod MONITORACT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const MONITORACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const MONITORACT_1: u32 = 0b1;
        }
    }

    /// PendSV exception active bit
    pub mod PENDSVACT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const PENDSVACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const PENDSVACT_1: u32 = 0b1;
        }
    }

    /// SysTick exception active bit
    pub mod SYSTICKACT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not active
            pub const SYSTICKACT_0: u32 = 0b0;

            /// 0b1: exception is active
            pub const SYSTICKACT_1: u32 = 0b1;
        }
    }

    /// UsageFault exception pending bit
    pub mod USGFAULTPENDED {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not pending
            pub const USGFAULTPENDED_0: u32 = 0b0;

            /// 0b1: exception is pending
            pub const USGFAULTPENDED_1: u32 = 0b1;
        }
    }

    /// MemManage exception pending bit
    pub mod MEMFAULTPENDED {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not pending
            pub const MEMFAULTPENDED_0: u32 = 0b0;

            /// 0b1: exception is pending
            pub const MEMFAULTPENDED_1: u32 = 0b1;
        }
    }

    /// BusFault exception pending bit
    pub mod BUSFAULTPENDED {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not pending
            pub const BUSFAULTPENDED_0: u32 = 0b0;

            /// 0b1: exception is pending
            pub const BUSFAULTPENDED_1: u32 = 0b1;
        }
    }

    /// SVCall pending bit
    pub mod SVCALLPENDED {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: exception is not pending
            pub const SVCALLPENDED_0: u32 = 0b0;

            /// 0b1: exception is pending
            pub const SVCALLPENDED_1: u32 = 0b1;
        }
    }

    /// MemManage enable bit
    pub mod MEMFAULTENA {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: disable the exception
            pub const MEMFAULTENA_0: u32 = 0b0;

            /// 0b1: enable the exception
            pub const MEMFAULTENA_1: u32 = 0b1;
        }
    }

    /// BusFault enable bit
    pub mod BUSFAULTENA {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: disable the exception
            pub const BUSFAULTENA_0: u32 = 0b0;

            /// 0b1: enable the exception
            pub const BUSFAULTENA_1: u32 = 0b1;
        }
    }

    /// UsageFault enable bit
    pub mod USGFAULTENA {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: disable the exception
            pub const USGFAULTENA_0: u32 = 0b0;

            /// 0b1: enable the exception
            pub const USGFAULTENA_1: u32 = 0b1;
        }
    }
}

/// Configurable Fault Status Register
pub mod CFSR {

    /// Instruction access violation flag
    pub mod IACCVIOL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no instruction access violation fault
            pub const IACCVIOL_0: u32 = 0b0;

            /// 0b1: the processor attempted an instruction fetch from a location that does not permit execution
            pub const IACCVIOL_1: u32 = 0b1;
        }
    }

    /// Data access violation flag
    pub mod DACCVIOL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no data access violation fault
            pub const DACCVIOL_0: u32 = 0b0;

            /// 0b1: the processor attempted a load or store at a location that does not permit the operation
            pub const DACCVIOL_1: u32 = 0b1;
        }
    }

    /// MemManage fault on unstacking for a return from exception
    pub mod MUNSTKERR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no unstacking fault
            pub const MUNSTKERR_0: u32 = 0b0;

            /// 0b1: unstack for an exception return has caused one or more access violations
            pub const MUNSTKERR_1: u32 = 0b1;
        }
    }

    /// MemManage fault on stacking for exception entry
    pub mod MSTKERR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no stacking fault
            pub const MSTKERR_0: u32 = 0b0;

            /// 0b1: stacking for an exception entry has caused one or more access violations
            pub const MSTKERR_1: u32 = 0b1;
        }
    }

    /// MemManage fault occurred during floating-point lazy state preservation
    pub mod MLSPERR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No MemManage fault occurred during floating-point lazy state preservation
            pub const MLSPERR_0: u32 = 0b0;

            /// 0b1: A MemManage fault occurred during floating-point lazy state preservation
            pub const MLSPERR_1: u32 = 0b1;
        }
    }

    /// MemManage Fault Address Register (MMFAR) valid flag
    pub mod MMARVALID {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: value in MMAR is not a valid fault address
            pub const MMARVALID_0: u32 = 0b0;

            /// 0b1: MMAR holds a valid fault address
            pub const MMARVALID_1: u32 = 0b1;
        }
    }

    /// Instruction bus error
    pub mod IBUSERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no instruction bus error
            pub const IBUSERR_0: u32 = 0b0;

            /// 0b1: instruction bus error
            pub const IBUSERR_1: u32 = 0b1;
        }
    }

    /// Precise data bus error
    pub mod PRECISERR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no precise data bus error
            pub const PRECISERR_0: u32 = 0b0;

            /// 0b1: a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault
            pub const PRECISERR_1: u32 = 0b1;
        }
    }

    /// Imprecise data bus error
    pub mod IMPRECISERR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no imprecise data bus error
            pub const IMPRECISERR_0: u32 = 0b0;

            /// 0b1: a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error
            pub const IMPRECISERR_1: u32 = 0b1;
        }
    }

    /// BusFault on unstacking for a return from exception
    pub mod UNSTKERR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no unstacking fault
            pub const UNSTKERR_0: u32 = 0b0;

            /// 0b1: unstack for an exception return has caused one or more BusFaults
            pub const UNSTKERR_1: u32 = 0b1;
        }
    }

    /// BusFault on stacking for exception entry
    pub mod STKERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no stacking fault
            pub const STKERR_0: u32 = 0b0;

            /// 0b1: stacking for an exception entry has caused one or more BusFaults
            pub const STKERR_1: u32 = 0b1;
        }
    }

    /// Bus fault occurred during floating-point lazy state preservation
    pub mod LSPERR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No bus fault occurred during floating-point lazy state preservation
            pub const LSPERR_0: u32 = 0b0;

            /// 0b1: A bus fault occurred during floating-point lazy state preservation
            pub const LSPERR_1: u32 = 0b1;
        }
    }

    /// BusFault Address Register (BFAR) valid flag
    pub mod BFARVALID {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: value in BFAR is not a valid fault address
            pub const BFARVALID_0: u32 = 0b0;

            /// 0b1: BFAR holds a valid fault address
            pub const BFARVALID_1: u32 = 0b1;
        }
    }

    /// Undefined instruction UsageFault
    pub mod UNDEFINSTR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no undefined instruction UsageFault
            pub const UNDEFINSTR_0: u32 = 0b0;

            /// 0b1: the processor has attempted to execute an undefined instruction
            pub const UNDEFINSTR_1: u32 = 0b1;
        }
    }

    /// Invalid state UsageFault
    pub mod INVSTATE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no invalid state UsageFault
            pub const INVSTATE_0: u32 = 0b0;

            /// 0b1: the processor has attempted to execute an instruction that makes illegal use of the EPSR
            pub const INVSTATE_1: u32 = 0b1;
        }
    }

    /// Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN
    pub mod INVPC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no invalid PC load UsageFault
            pub const INVPC_0: u32 = 0b0;

            /// 0b1: the processor has attempted an illegal load of EXC_RETURN to the PC
            pub const INVPC_1: u32 = 0b1;
        }
    }

    /// No coprocessor UsageFault
    pub mod NOCP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no UsageFault caused by attempting to access a coprocessor
            pub const NOCP_0: u32 = 0b0;

            /// 0b1: the processor has attempted to access a coprocessor
            pub const NOCP_1: u32 = 0b1;
        }
    }

    /// Unaligned access UsageFault
    pub mod UNALIGNED {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no unaligned access fault, or unaligned access trapping not enabled
            pub const UNALIGNED_0: u32 = 0b0;

            /// 0b1: the processor has made an unaligned memory access
            pub const UNALIGNED_1: u32 = 0b1;
        }
    }

    /// Divide by zero UsageFault
    pub mod DIVBYZERO {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no divide by zero fault, or divide by zero trapping not enabled
            pub const DIVBYZERO_0: u32 = 0b0;

            /// 0b1: the processor has executed an SDIV or UDIV instruction with a divisor of 0
            pub const DIVBYZERO_1: u32 = 0b1;
        }
    }
}

/// HardFault Status register
pub mod HFSR {

    /// Indicates a BusFault on a vector table read during exception processing.
    pub mod VECTTBL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no BusFault on vector table read
            pub const VECTTBL_0: u32 = 0b0;

            /// 0b1: BusFault on vector table read
            pub const VECTTBL_1: u32 = 0b1;
        }
    }

    /// Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled.
    pub mod FORCED {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no forced HardFault
            pub const FORCED_0: u32 = 0b0;

            /// 0b1: forced HardFault
            pub const FORCED_1: u32 = 0b1;
        }
    }

    /// Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable.
    pub mod DEBUGEVT {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Debug event has occurred.
            pub const DEBUGEVT_0: u32 = 0b0;

            /// 0b1: Debug event has occurred. The Debug Fault Status Register has been updated.
            pub const DEBUGEVT_1: u32 = 0b1;
        }
    }
}

/// Debug Fault Status Register
pub mod DFSR {

    /// Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1.
    pub mod HALTED {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No active halt request debug event
            pub const HALTED_0: u32 = 0b0;

            /// 0b1: Halt request debug event active
            pub const HALTED_1: u32 = 0b1;
        }
    }

    /// Debug event generated by BKPT instruction execution or a breakpoint match in FPB
    pub mod BKPT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No current breakpoint debug event
            pub const BKPT_0: u32 = 0b0;

            /// 0b1: At least one current breakpoint debug event
            pub const BKPT_1: u32 = 0b1;
        }
    }

    /// Debug event generated by the DWT
    pub mod DWTTRAP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No current debug events generated by the DWT
            pub const DWTTRAP_0: u32 = 0b0;

            /// 0b1: At least one current debug event generated by the DWT
            pub const DWTTRAP_1: u32 = 0b1;
        }
    }

    /// Indicates triggering of a Vector catch
    pub mod VCATCH {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No Vector catch triggered
            pub const VCATCH_0: u32 = 0b0;

            /// 0b1: Vector catch triggered
            pub const VCATCH_1: u32 = 0b1;
        }
    }

    /// Debug event generated because of the assertion of an external debug request
    pub mod EXTERNAL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No external debug request debug event
            pub const EXTERNAL_0: u32 = 0b0;

            /// 0b1: External debug request debug event
            pub const EXTERNAL_1: u32 = 0b1;
        }
    }
}

/// MemManage Fault Address Register
pub mod MMFAR {

    /// Address of MemManage fault location
    pub mod ADDRESS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// BusFault Address Register
pub mod BFAR {
    pub use super::MMFAR::ADDRESS;
}

/// Processor Feature Register 0
pub mod ID_PFR0 {

    /// ARM instruction set support
    pub mod STATE0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: ARMv7-M unused
            pub const STATE0_0: u32 = 0b0000;

            /// 0b0001: ARMv7-M unused
            pub const STATE0_1: u32 = 0b0001;

            /// 0b0010: ARMv7-M unused
            pub const STATE0_2: u32 = 0b0010;

            /// 0b0011: Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions.
            pub const STATE0_3: u32 = 0b0011;
        }
    }

    /// Thumb instruction set support
    pub mod STATE1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: The processor does not support the ARM instruction set.
            pub const STATE1_0: u32 = 0b0000;

            /// 0b0001: ARMv7-M unused
            pub const STATE1_1: u32 = 0b0001;
        }
    }

    /// ARMv7-M unused
    pub mod STATE2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ARMv7-M unused
    pub mod STATE3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Processor Feature Register 1
pub mod ID_PFR1 {

    /// M profile programmers' model
    pub mod PROGMODEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: ARMv7-M unused
            pub const PROGMODEL_0: u32 = 0b0000;

            /// 0b0010: Two-stack programmers' model supported
            pub const PROGMODEL_2: u32 = 0b0010;
        }
    }
}

/// Debug Feature Register
pub mod ID_DFR0 {

    /// Support for memory-mapped debug model for M profile processors
    pub mod DEBUGMODEL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not supported
            pub const DEBUGMODEL_0: u32 = 0b0000;

            /// 0b0001: Support for M profile Debug architecture, with memory-mapped access.
            pub const DEBUGMODEL_1: u32 = 0b0001;
        }
    }
}

/// Auxiliary Feature Register
pub mod ID_AFR0 {

    /// Gives information about the IMPLEMENTATION DEFINED features of a processor implementation.
    pub mod IMPLEMENTATION_DEFINED0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gives information about the IMPLEMENTATION DEFINED features of a processor implementation.
    pub mod IMPLEMENTATION_DEFINED1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gives information about the IMPLEMENTATION DEFINED features of a processor implementation.
    pub mod IMPLEMENTATION_DEFINED2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Gives information about the IMPLEMENTATION DEFINED features of a processor implementation.
    pub mod IMPLEMENTATION_DEFINED3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Memory Model Feature Register 0
pub mod ID_MMFR0 {

    /// Indicates support for a PMSA
    pub mod PMSASUPPORT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not supported
            pub const PMSASUPPORT_0: u32 = 0b0000;

            /// 0b0001: ARMv7-M unused
            pub const PMSASUPPORT_1: u32 = 0b0001;

            /// 0b0010: ARMv7-M unused
            pub const PMSASUPPORT_2: u32 = 0b0010;

            /// 0b0011: PMSAv7, providing support for a base region and subregions.
            pub const PMSASUPPORT_3: u32 = 0b0011;
        }
    }

    /// Indicates the outermost shareability domain implemented
    pub mod OUTERMOST_SHAREABILITY {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Implemented as Non-cacheable
            pub const OUTERMOST_SHAREABILITY_0: u32 = 0b0000;

            /// 0b0001: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_1: u32 = 0b0001;

            /// 0b0010: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_2: u32 = 0b0010;

            /// 0b0011: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_3: u32 = 0b0011;

            /// 0b0100: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_4: u32 = 0b0100;

            /// 0b0101: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_5: u32 = 0b0101;

            /// 0b0110: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_6: u32 = 0b0110;

            /// 0b0111: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_7: u32 = 0b0111;

            /// 0b1000: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_8: u32 = 0b1000;

            /// 0b1001: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_9: u32 = 0b1001;

            /// 0b1010: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_10: u32 = 0b1010;

            /// 0b1011: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_11: u32 = 0b1011;

            /// 0b1100: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_12: u32 = 0b1100;

            /// 0b1101: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_13: u32 = 0b1101;

            /// 0b1110: ARMv7-M unused
            pub const OUTERMOST_SHAREABILITY_14: u32 = 0b1110;

            /// 0b1111: Shareability ignored.
            pub const OUTERMOST_SHAREABILITY_15: u32 = 0b1111;
        }
    }

    /// Indicates the number of shareability levels implemented
    pub mod SHAREABILITY_LEVELS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: One level of shareability implemented
            pub const SHAREABILITY_LEVELS_0: u32 = 0b0000;

            /// 0b0001: ARMv7-M unused
            pub const SHAREABILITY_LEVELS_1: u32 = 0b0001;
        }
    }

    /// Indicates the support for Tightly Coupled Memory
    pub mod TCM_SUPPORT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No tightly coupled memories implemented.
            pub const TCM_SUPPORT_0: u32 = 0b0000;

            /// 0b0001: Tightly coupled memories implemented with IMPLEMENTATION DEFINED control.
            pub const TCM_SUPPORT_1: u32 = 0b0001;

            /// 0b0010: ARMv7-M unused
            pub const TCM_SUPPORT_2: u32 = 0b0010;
        }
    }

    /// Indicates the support for Auxiliary registers
    pub mod AUXILIARY_REGISTERS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not supported
            pub const AUXILIARY_REGISTERS_0: u32 = 0b0000;

            /// 0b0001: Support for Auxiliary Control Register only.
            pub const AUXILIARY_REGISTERS_1: u32 = 0b0001;

            /// 0b0010: ARMv7-M unused
            pub const AUXILIARY_REGISTERS_2: u32 = 0b0010;
        }
    }
}

/// Memory Model Feature Register 1
pub mod ID_MMFR1 {

    /// Gives information about the implemented memory model and memory management support.
    pub mod ID_MMFR1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Memory Model Feature Register 2
pub mod ID_MMFR2 {

    /// Indicates the support for Wait For Interrupt (WFI) stalling
    pub mod WFI_STALL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Not supported
            pub const WFI_STALL_0: u32 = 0b0000;

            /// 0b0001: Support for WFI stalling
            pub const WFI_STALL_1: u32 = 0b0001;
        }
    }
}

/// Memory Model Feature Register 3
pub mod ID_MMFR3 {

    /// Gives information about the implemented memory model and memory management support.
    pub mod ID_MMFR3 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Instruction Set Attributes Register 0
pub mod ID_ISAR0 {

    /// Indicates the supported Bit Counting instructions
    pub mod BITCOUNT_INSTRS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const BITCOUNT_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the CLZ instruction
            pub const BITCOUNT_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported BitField instructions
    pub mod BITFIELD_INSTRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const BITFIELD_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the BFC, BFI, SBFX, and UBFX instructions
            pub const BITFIELD_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported combined Compare and Branch instructions
    pub mod CMPBRANCH_INSTRS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const CMPBRANCH_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the CBNZ and CBZ instructions
            pub const CMPBRANCH_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported Coprocessor instructions
    pub mod COPROC_INSTRS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, except for separately attributed architectures, for example the Floating-point extension
            pub const COPROC_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for generic CDP, LDC, MCR, MRC, and STC instructions
            pub const COPROC_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for generic CDP2, LDC2, MCR2, MRC2, and STC2 instructions
            pub const COPROC_INSTRS_2: u32 = 0b0010;

            /// 0b0011: As for 2, and adds support for generic MCRR and MRRC instructions
            pub const COPROC_INSTRS_3: u32 = 0b0011;

            /// 0b0100: As for 3, and adds support for generic MCRR2 and MRRC2 instructions
            pub const COPROC_INSTRS_4: u32 = 0b0100;
        }
    }

    /// Indicates the supported Debug instructions
    pub mod DEBUG_INSTRS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const DEBUG_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the BKPT instruction
            pub const DEBUG_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported Divide instructions
    pub mod DIVIDE_INSTRS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const DIVIDE_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the SDIV and UDIV instructions
            pub const DIVIDE_INSTRS_1: u32 = 0b0001;
        }
    }
}

/// Instruction Set Attributes Register 1
pub mod ID_ISAR1 {

    /// Indicates the supported Extend instructions
    pub mod EXTEND_INSTRS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const EXTEND_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the SXTB, SXTH, UXTB, and UXTH instructions
            pub const EXTEND_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions
            pub const EXTEND_INSTRS_2: u32 = 0b0010;
        }
    }

    /// Indicates the supported IfThen instructions
    pub mod IFTHEN_INSTRS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const IFTHEN_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the IT instructions, and for the IT bits in the PSRs
            pub const IFTHEN_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the support for data-processing instructions with long immediate
    pub mod IMMEDIATE_INSTRS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const IMMEDIATE_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the ADDW, MOVW, MOVT, and SUBW instructions
            pub const IMMEDIATE_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported Interworking instructions
    pub mod INTERWORK_INSTRS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const INTERWORK_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the BX instruction, and the T bit in the PSR
            pub const INTERWORK_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior
            pub const INTERWORK_INSTRS_2: u32 = 0b0010;

            /// 0b0011: ARMv7-M unused
            pub const INTERWORK_INSTRS_3: u32 = 0b0011;
        }
    }
}

/// Instruction Set Attributes Register 2
pub mod ID_ISAR2 {

    /// Indicates the supported additional load and store instructions
    pub mod LOADSTORE_INSTRS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const LOADSTORE_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the LDRD and STRD instructions
            pub const LOADSTORE_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported Memory Hint instructions
    pub mod MEMHINT_INSTRS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const MEMHINT_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the PLD instruction, ARMv7-M unused.
            pub const MEMHINT_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, ARMv7-M unused.
            pub const MEMHINT_INSTRS_2: u32 = 0b0010;

            /// 0b0011: As for 1 or 2, and adds support for the PLI instruction.
            pub const MEMHINT_INSTRS_3: u32 = 0b0011;
        }
    }

    /// Indicates the support for multi-access interruptible instructions
    pub mod MULTIACCESSINT_INSTRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused.
            pub const MULTIACCESSINT_INSTRS_0: u32 = 0b0000;

            /// 0b0001: LDM and STM instructions are restartable.
            pub const MULTIACCESSINT_INSTRS_1: u32 = 0b0001;

            /// 0b0010: LDM and STM instructions are continuable.
            pub const MULTIACCESSINT_INSTRS_2: u32 = 0b0010;
        }
    }

    /// Indicates the supported additional Multiply instructions
    pub mod MULT_INSTRS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported. This means only MUL is supported. ARMv7-M unused.
            pub const MULT_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the MLA instruction, ARMv7-M unused.
            pub const MULT_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the MLS instruction.
            pub const MULT_INSTRS_2: u32 = 0b0010;
        }
    }

    /// Indicates the supported advanced signed Multiply instructions
    pub mod MULTS_INSTRS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const MULTS_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the SMULL and SMLAL instructions
            pub const MULTS_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions.
            pub const MULTS_INSTRS_2: u32 = 0b0010;

            /// 0b0011: As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions.
            pub const MULTS_INSTRS_3: u32 = 0b0011;
        }
    }

    /// Indicates the supported advanced unsigned Multiply instructions
    pub mod MULTU_INSTRS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const MULTU_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the UMULL and UMLAL instructions.
            pub const MULTU_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the UMAAL instruction.
            pub const MULTU_INSTRS_2: u32 = 0b0010;
        }
    }

    /// Indicates the supported Reversal instructions
    pub mod REVERSAL_INSTRS {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused
            pub const REVERSAL_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused.
            pub const REVERSAL_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the RBIT instruction.
            pub const REVERSAL_INSTRS_2: u32 = 0b0010;
        }
    }
}

/// Instruction Set Attributes Register 3
pub mod ID_ISAR3 {

    /// Indicates the supported Saturate instructions
    pub mod SATURATE_INSTRS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported
            pub const SATURATE_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs.
            pub const SATURATE_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported SIMD instructions
    pub mod SIMD_INSTRS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const SIMD_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs.
            pub const SIMD_INSTRS_1: u32 = 0b0001;

            /// 0b0011: As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE\[3:0\] bits in the PSRs.
            pub const SIMD_INSTRS_3: u32 = 0b0011;
        }
    }

    /// Indicates the supported SVC instructions
    pub mod SVC_INSTRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const SVC_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the SVC instruction.
            pub const SVC_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Together with the ID_ISAR4\[SYNCHPRIM_INSTRS_FRAC\] indicates the supported Synchronization Primitives
    pub mod SYNCHPRIM_INSTRS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the supported Table Branch instructions
    pub mod TABBRANCH_INSTRS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const TABBRANCH_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the TBB and TBH instructions.
            pub const TABBRANCH_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported non flag-setting MOV instructions
    pub mod THUMBCOPY_INSTRS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const THUMBCOPY_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register.
            pub const THUMBCOPY_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported non flag-setting MOV instructions
    pub mod TRUENOP_INSTRS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const TRUENOP_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register.
            pub const TRUENOP_INSTRS_1: u32 = 0b0001;
        }
    }
}

/// Instruction Set Attributes Register 4
pub mod ID_ISAR4 {

    /// Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix.
    pub mod UNPRIV_INSTRS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const UNPRIV_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the LDRBT, LDRT, STRBT, and STRT instructions.
            pub const UNPRIV_INSTRS_1: u32 = 0b0001;

            /// 0b0010: As for 1, and adds support for the LDRHT, LDRSBT, LDRSHT, and STRHT instructions.
            pub const UNPRIV_INSTRS_2: u32 = 0b0010;
        }
    }

    /// Indicates the support for instructions with shifts
    pub mod WITHSHIFTS_INSTRS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Nonzero shifts supported only in MOV and shift instructions.
            pub const WITHSHIFTS_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for shifts of loads and stores over the range LSL 0-3.
            pub const WITHSHIFTS_INSTRS_1: u32 = 0b0001;

            /// 0b0011: As for 1, and adds support for other constant shift options, on loads, stores, and other instructions.
            pub const WITHSHIFTS_INSTRS_3: u32 = 0b0011;

            /// 0b0100: ARMv7-M unused.
            pub const WITHSHIFTS_INSTRS_4: u32 = 0b0100;
        }
    }

    /// Indicates the support for Writeback addressing modes
    pub mod WRITEBACK_INSTRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Basic support. Only the LDM, STM, PUSH, and POP instructions support writeback addressing modes. ARMv7-M unused.
            pub const WRITEBACK_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for all of the writeback addressing modes defined in the ARMv7-M architecture.
            pub const WRITEBACK_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Indicates the supported Barrier instructions
    pub mod BARRIER_INSTRS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const BARRIER_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the DMB, DSB, and ISB barrier instructions.
            pub const BARRIER_INSTRS_1: u32 = 0b0001;
        }
    }

    /// Together with the ID_ISAR3\[SYNCHPRIM_INSTRS\] indicates the supported Synchronization Primitives
    pub mod SYNCHPRIM_INSTRS_FRAC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the supported M profile instructions to modify the PSRs
    pub mod PSR_M_INSTRS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: None supported, ARMv7-M unused.
            pub const PSR_M_INSTRS_0: u32 = 0b0000;

            /// 0b0001: Adds support for the M-profile forms of the CPS, MRS, and MSR instructions, to access the PSRs.
            pub const PSR_M_INSTRS_1: u32 = 0b0001;
        }
    }
}

/// Cache Level ID register
pub mod CLIDR {

    /// Indicate the type of cache implemented at level 1.
    pub mod CL1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL1_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL1_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL1_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL1_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL1_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 2.
    pub mod CL2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL2_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL2_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL2_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL2_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL2_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 3.
    pub mod CL3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (3 bits: 0b111 << 6)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL3_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL3_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL3_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL3_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL3_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 4.
    pub mod CL4 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL4_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL4_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL4_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL4_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL4_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 5.
    pub mod CL5 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL5_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL5_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL5_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL5_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL5_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 6.
    pub mod CL6 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL6_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL6_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL6_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL6_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL6_4: u32 = 0b100;
        }
    }

    /// Indicate the type of cache implemented at level 7.
    pub mod CL7 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No cache
            pub const CL7_0: u32 = 0b000;

            /// 0b001: Instruction cache only
            pub const CL7_1: u32 = 0b001;

            /// 0b010: Data cache only
            pub const CL7_2: u32 = 0b010;

            /// 0b011: Separate instruction and data caches
            pub const CL7_3: u32 = 0b011;

            /// 0b100: Unified cache
            pub const CL7_4: u32 = 0b100;
        }
    }

    /// Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ.
    pub mod LOUIS {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0
            pub const LOUIS_0: u32 = 0b000;

            /// 0b001: 1
            pub const LOUIS_1: u32 = 0b001;

            /// 0b010: 2
            pub const LOUIS_2: u32 = 0b010;

            /// 0b011: 3
            pub const LOUIS_3: u32 = 0b011;

            /// 0b100: 4
            pub const LOUIS_4: u32 = 0b100;

            /// 0b101: 5
            pub const LOUIS_5: u32 = 0b101;

            /// 0b110: 6
            pub const LOUIS_6: u32 = 0b110;

            /// 0b111: 7
            pub const LOUIS_7: u32 = 0b111;
        }
    }

    /// Level of Coherency for the cache hierarchy
    pub mod LOC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0
            pub const LOC_0: u32 = 0b000;

            /// 0b001: 1
            pub const LOC_1: u32 = 0b001;

            /// 0b010: 2
            pub const LOC_2: u32 = 0b010;

            /// 0b011: 3
            pub const LOC_3: u32 = 0b011;

            /// 0b100: 4
            pub const LOC_4: u32 = 0b100;

            /// 0b101: 5
            pub const LOC_5: u32 = 0b101;

            /// 0b110: 6
            pub const LOC_6: u32 = 0b110;

            /// 0b111: 7
            pub const LOC_7: u32 = 0b111;
        }
    }

    /// Level of Unification for the cache hierarchy
    pub mod LOU {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (3 bits: 0b111 << 27)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0
            pub const LOU_0: u32 = 0b000;

            /// 0b001: 1
            pub const LOU_1: u32 = 0b001;

            /// 0b010: 2
            pub const LOU_2: u32 = 0b010;

            /// 0b011: 3
            pub const LOU_3: u32 = 0b011;

            /// 0b100: 4
            pub const LOU_4: u32 = 0b100;

            /// 0b101: 5
            pub const LOU_5: u32 = 0b101;

            /// 0b110: 6
            pub const LOU_6: u32 = 0b110;

            /// 0b111: 7
            pub const LOU_7: u32 = 0b111;
        }
    }
}

/// Cache Type register
pub mod CTR {

    /// Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor.
    pub mod IMINLINE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor.
    pub mod DMINLINE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words.
    pub mod ERG {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words.
    pub mod CWG {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the implemented CTR format.
    pub mod FORMAT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: ARMv7 format.
            pub const FORMAT_4: u32 = 0b100;
        }
    }
}

/// Cache Size ID Register
pub mod CCSIDR {

    /// (Log2(Number of words in cache line)) - 2.
    pub mod LINESIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The line length of 4 words.
            pub const LINESIZE_0: u32 = 0b000;

            /// 0b001: The line length of 8 words.
            pub const LINESIZE_1: u32 = 0b001;

            /// 0b010: The line length of 16 words.
            pub const LINESIZE_2: u32 = 0b010;

            /// 0b011: The line length of 32 words.
            pub const LINESIZE_3: u32 = 0b011;

            /// 0b100: The line length of 64 words.
            pub const LINESIZE_4: u32 = 0b100;

            /// 0b101: The line length of 128 words.
            pub const LINESIZE_5: u32 = 0b101;

            /// 0b110: The line length of 256 words.
            pub const LINESIZE_6: u32 = 0b110;

            /// 0b111: The line length of 512 words.
            pub const LINESIZE_7: u32 = 0b111;
        }
    }

    /// (Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2.
    pub mod ASSOCIATIVITY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (10 bits: 0x3ff << 3)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// (Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2.
    pub mod NUMSETS {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (15 bits: 0x7fff << 13)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates whether the cache level supports write-allocation
    pub mod WA {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Feature not supported
            pub const WA_0: u32 = 0b0;

            /// 0b1: Feature supported
            pub const WA_1: u32 = 0b1;
        }
    }

    /// Indicates whether the cache level supports read-allocation
    pub mod RA {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Feature not supported
            pub const RA_0: u32 = 0b0;

            /// 0b1: Feature supported
            pub const RA_1: u32 = 0b1;
        }
    }

    /// Indicates whether the cache level supports write-back
    pub mod WB {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Feature not supported
            pub const WB_0: u32 = 0b0;

            /// 0b1: Feature supported
            pub const WB_1: u32 = 0b1;
        }
    }

    /// Indicates whether the cache level supports write-through
    pub mod WT {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Feature not supported
            pub const WT_0: u32 = 0b0;

            /// 0b1: Feature supported
            pub const WT_1: u32 = 0b1;
        }
    }
}

/// Cache Size Selection Register
pub mod CSSELR {

    /// Instruction not data bit
    pub mod IND {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data or unified cache.
            pub const IND_0: u32 = 0b0;

            /// 0b1: Instruction cache.
            pub const IND_1: u32 = 0b1;
        }
    }

    /// Cache level of required cache
    pub mod LEVEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Level 1 cache.
            pub const LEVEL_0: u32 = 0b000;

            /// 0b001: Level 2 cache.
            pub const LEVEL_1: u32 = 0b001;

            /// 0b010: Level 3 cache.
            pub const LEVEL_2: u32 = 0b010;

            /// 0b011: Level 4 cache.
            pub const LEVEL_3: u32 = 0b011;

            /// 0b100: Level 5 cache.
            pub const LEVEL_4: u32 = 0b100;

            /// 0b101: Level 6 cache.
            pub const LEVEL_5: u32 = 0b101;

            /// 0b110: Level 7 cache.
            pub const LEVEL_6: u32 = 0b110;
        }
    }
}

/// Coprocessor Access Control Register
pub mod CPACR {

    /// Access privileges for coprocessor 0.
    pub mod CP0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP0_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP0_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP0_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 1.
    pub mod CP1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP1_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP1_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP1_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 2.
    pub mod CP2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP2_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP2_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP2_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 3.
    pub mod CP3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP3_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP3_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP3_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 4.
    pub mod CP4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP4_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP4_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP4_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 5.
    pub mod CP5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP5_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP5_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP5_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 6.
    pub mod CP6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP6_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP6_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP6_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 7.
    pub mod CP7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP7_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP7_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP7_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 10.
    pub mod CP10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP10_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP10_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP10_3: u32 = 0b11;
        }
    }

    /// Access privileges for coprocessor 11.
    pub mod CP11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access denied. Any attempted access generates a NOCP UsageFault.
            pub const CP11_0: u32 = 0b00;

            /// 0b01: Privileged access only. An unprivileged access generates a NOCP UsageFault.
            pub const CP11_1: u32 = 0b01;

            /// 0b11: Full access.
            pub const CP11_3: u32 = 0b11;
        }
    }
}

/// Instruction cache invalidate all to Point of Unification (PoU)
pub mod STIR {

    /// Indicates the interrupt to be triggered
    pub mod INTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Instruction cache invalidate all to Point of Unification (PoU)
pub mod ICIALLU {

    /// I-cache invalidate all to PoU
    pub mod ICIALLU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Instruction cache invalidate by address to PoU
pub mod ICIMVAU {

    /// I-cache invalidate by MVA to PoU
    pub mod ICIMVAU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache invalidate by address to Point of Coherency (PoC)
pub mod DCIMVAC {

    /// D-cache invalidate by MVA to PoC
    pub mod DCIMVAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache invalidate by set/way
pub mod DCISW {

    /// D-cache invalidate by set-way
    pub mod DCISW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache by address to PoU
pub mod DCCMVAU {

    /// D-cache clean by MVA to PoU
    pub mod DCCMVAU {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache clean by address to PoC
pub mod DCCMVAC {

    /// D-cache clean by MVA to PoC
    pub mod DCCMVAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache clean by set/way
pub mod DCCSW {

    /// D-cache clean by set-way
    pub mod DCCSW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache clean and invalidate by address to PoC
pub mod DCCIMVAC {

    /// D-cache clean and invalidate by MVA to PoC
    pub mod DCCIMVAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Data cache clean and invalidate by set/way
pub mod DCCISW {

    /// D-cache clean and invalidate by set-way
    pub mod DCCISW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Instruction Tightly-Coupled Memory Control Register
pub mod CM7_ITCMCR {

    /// TCM enable. When a TCM is disabled all accesses are made to the AXIM interface.
    pub mod EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TCM disabled.
            pub const EN_0: u32 = 0b0;

            /// 0b1: TCM enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence.
    pub mod RMW {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RMW disabled.
            pub const RMW_0: u32 = 0b0;

            /// 0b1: RMW enabled.
            pub const RMW_1: u32 = 0b1;
        }
    }

    /// Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access.
    pub mod RETEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Retry phase disabled.
            pub const RETEN_0: u32 = 0b0;

            /// 0b1: Retry phase enabled.
            pub const RETEN_1: u32 = 0b1;
        }
    }

    /// TCM size. Indicates the size of the relevant TCM.
    pub mod SZ {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No TCM implemented.
            pub const SZ_0: u32 = 0b0000;

            /// 0b0011: 4KB.
            pub const SZ_3: u32 = 0b0011;

            /// 0b0100: 8KB.
            pub const SZ_4: u32 = 0b0100;

            /// 0b0101: 16KB.
            pub const SZ_5: u32 = 0b0101;

            /// 0b0110: 32KB.
            pub const SZ_6: u32 = 0b0110;

            /// 0b0111: 64KB.
            pub const SZ_7: u32 = 0b0111;

            /// 0b1000: 128KB.
            pub const SZ_8: u32 = 0b1000;

            /// 0b1001: 256KB.
            pub const SZ_9: u32 = 0b1001;

            /// 0b1010: 512KB.
            pub const SZ_10: u32 = 0b1010;

            /// 0b1011: 1MB.
            pub const SZ_11: u32 = 0b1011;

            /// 0b1100: 2MB.
            pub const SZ_12: u32 = 0b1100;

            /// 0b1101: 4MB.
            pub const SZ_13: u32 = 0b1101;

            /// 0b1110: 8MB.
            pub const SZ_14: u32 = 0b1110;

            /// 0b1111: 16MB.
            pub const SZ_15: u32 = 0b1111;
        }
    }
}

/// Data Tightly-Coupled Memory Control Register
pub mod CM7_DTCMCR {
    pub use super::CM7_ITCMCR::EN;
    pub use super::CM7_ITCMCR::RETEN;
    pub use super::CM7_ITCMCR::RMW;
    pub use super::CM7_ITCMCR::SZ;
}

/// AHBP Control Register
pub mod CM7_AHBPCR {

    /// AHBP enable.
    pub mod EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: AHBP disabled. When disabled all accesses are made to the AXIM interface.
            pub const EN_0: u32 = 0b0;

            /// 0b1: AHBP enabled.
            pub const EN_1: u32 = 0b1;
        }
    }

    /// AHBP size.
    pub mod SZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0MB. AHBP disabled.
            pub const SZ_0: u32 = 0b000;

            /// 0b001: 64MB.
            pub const SZ_1: u32 = 0b001;

            /// 0b010: 128MB.
            pub const SZ_2: u32 = 0b010;

            /// 0b011: 256MB.
            pub const SZ_3: u32 = 0b011;

            /// 0b100: 512MB.
            pub const SZ_4: u32 = 0b100;
        }
    }
}

/// L1 Cache Control Register
pub mod CM7_CACR {

    /// Shared cacheable-is-WT for data cache. Enables limited cache coherency usage.
    pub mod SIWT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory.
            pub const SIWT_0: u32 = 0b0;

            /// 0b1: Normal Cacheable shared locations are treated as Write-Through.
            pub const SIWT_1: u32 = 0b1;
        }
    }

    /// Enables ECC in the instruction and data cache.
    pub mod ECCDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Enables ECC in the instruction and data cache.
            pub const ECCDIS_0: u32 = 0b0;

            /// 0b1: Disables ECC in the instruction and data cache.
            pub const ECCDIS_1: u32 = 0b1;
        }
    }

    /// Enables Force Write-Through in the data cache.
    pub mod FORCEWT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disables Force Write-Through.
            pub const FORCEWT_0: u32 = 0b0;

            /// 0b1: Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through.
            pub const FORCEWT_1: u32 = 0b1;
        }
    }
}

/// AHB Slave Control Register
pub mod CM7_AHBSCR {

    /// AHBS prioritization control.
    pub mod CTL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: AHBS access priority demoted. This is the reset value.
            pub const CTL_0: u32 = 0b00;

            /// 0b01: Software access priority demoted.
            pub const CTL_1: u32 = 0b01;

            /// 0b10: AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\[INITCOUNT\] value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\[TPRI\].
            pub const CTL_2: u32 = 0b10;

            /// 0b11: AHBSPRI signal has control of access priority.
            pub const CTL_3: u32 = 0b11;
        }
    }

    /// Threshold execution priority for AHBS traffic demotion.
    pub mod TPRI {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (9 bits: 0x1ff << 2)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fairness counter initialization value.
    pub mod INITCOUNT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (5 bits: 0b11111 << 11)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Auxiliary Bus Fault Status Register
pub mod CM7_ABFSR {

    /// Asynchronous fault on ITCM interface.
    pub mod ITCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous fault on DTCM interface.
    pub mod DTCM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous fault on AHBP interface.
    pub mod AHBP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous fault on AXIM interface.
    pub mod AXIM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Asynchronous fault on EPPB interface.
    pub mod EPPB {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1.
    pub mod AXIMTYPE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: OKAY.
            pub const AXIMTYPE_0: u32 = 0b00;

            /// 0b01: EXOKAY.
            pub const AXIMTYPE_1: u32 = 0b01;

            /// 0b10: SLVERR.
            pub const AXIMTYPE_2: u32 = 0b10;

            /// 0b11: DECERR.
            pub const AXIMTYPE_3: u32 = 0b11;
        }
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 2],

    /// Auxiliary Control Register,
    pub ACTLR: RWRegister<u32>,

    _reserved2: [u32; 829],

    /// CPUID Base Register
    pub CPUID: RORegister<u32>,

    /// Interrupt Control and State Register
    pub ICSR: RWRegister<u32>,

    /// Vector Table Offset Register
    pub VTOR: RWRegister<u32>,

    /// Application Interrupt and Reset Control Register
    pub AIRCR: RWRegister<u32>,

    /// System Control Register
    pub SCR: RWRegister<u32>,

    /// Configuration and Control Register
    pub CCR: RWRegister<u32>,

    /// System Handler Priority Register 1
    pub SHPR1: RWRegister<u32>,

    /// System Handler Priority Register 2
    pub SHPR2: RWRegister<u32>,

    /// System Handler Priority Register 3
    pub SHPR3: RWRegister<u32>,

    /// System Handler Control and State Register
    pub SHCSR: RWRegister<u32>,

    /// Configurable Fault Status Register
    pub CFSR: RWRegister<u32>,

    /// HardFault Status register
    pub HFSR: RWRegister<u32>,

    /// Debug Fault Status Register
    pub DFSR: RWRegister<u32>,

    /// MemManage Fault Address Register
    pub MMFAR: RWRegister<u32>,

    /// BusFault Address Register
    pub BFAR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Processor Feature Register 0
    pub ID_PFR0: RORegister<u32>,

    /// Processor Feature Register 1
    pub ID_PFR1: RORegister<u32>,

    /// Debug Feature Register
    pub ID_DFR0: RORegister<u32>,

    /// Auxiliary Feature Register
    pub ID_AFR0: RORegister<u32>,

    /// Memory Model Feature Register 0
    pub ID_MMFR0: RORegister<u32>,

    /// Memory Model Feature Register 1
    pub ID_MMFR1: RORegister<u32>,

    /// Memory Model Feature Register 2
    pub ID_MMFR2: RORegister<u32>,

    /// Memory Model Feature Register 3
    pub ID_MMFR3: RORegister<u32>,

    /// Instruction Set Attributes Register 0
    pub ID_ISAR0: RORegister<u32>,

    /// Instruction Set Attributes Register 1
    pub ID_ISAR1: RORegister<u32>,

    /// Instruction Set Attributes Register 2
    pub ID_ISAR2: RORegister<u32>,

    /// Instruction Set Attributes Register 3
    pub ID_ISAR3: RORegister<u32>,

    /// Instruction Set Attributes Register 4
    pub ID_ISAR4: RORegister<u32>,

    _reserved4: [u32; 1],

    /// Cache Level ID register
    pub CLIDR: RORegister<u32>,

    /// Cache Type register
    pub CTR: RORegister<u32>,

    /// Cache Size ID Register
    pub CCSIDR: RORegister<u32>,

    /// Cache Size Selection Register
    pub CSSELR: RWRegister<u32>,

    /// Coprocessor Access Control Register
    pub CPACR: RWRegister<u32>,

    _reserved5: [u32; 93],

    /// Instruction cache invalidate all to Point of Unification (PoU)
    pub STIR: RWRegister<u32>,

    _reserved6: [u32; 19],

    /// Instruction cache invalidate all to Point of Unification (PoU)
    pub ICIALLU: UnsafeWORegister<u32>,

    _reserved7: [u32; 1],

    /// Instruction cache invalidate by address to PoU
    pub ICIMVAU: UnsafeWORegister<u32>,

    /// Data cache invalidate by address to Point of Coherency (PoC)
    pub DCIMVAC: UnsafeWORegister<u32>,

    /// Data cache invalidate by set/way
    pub DCISW: UnsafeWORegister<u32>,

    /// Data cache by address to PoU
    pub DCCMVAU: UnsafeWORegister<u32>,

    /// Data cache clean by address to PoC
    pub DCCMVAC: UnsafeWORegister<u32>,

    /// Data cache clean by set/way
    pub DCCSW: UnsafeWORegister<u32>,

    /// Data cache clean and invalidate by address to PoC
    pub DCCIMVAC: UnsafeWORegister<u32>,

    /// Data cache clean and invalidate by set/way
    pub DCCISW: UnsafeWORegister<u32>,

    _reserved8: [u32; 6],

    /// Instruction Tightly-Coupled Memory Control Register
    pub CM7_ITCMCR: RWRegister<u32>,

    /// Data Tightly-Coupled Memory Control Register
    pub CM7_DTCMCR: RWRegister<u32>,

    /// AHBP Control Register
    pub CM7_AHBPCR: RWRegister<u32>,

    /// L1 Cache Control Register
    pub CM7_CACR: RWRegister<u32>,

    /// AHB Slave Control Register
    pub CM7_AHBSCR: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// Auxiliary Bus Fault Status Register
    pub CM7_ABFSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ACTLR: u32,
    pub CPUID: u32,
    pub ICSR: u32,
    pub VTOR: u32,
    pub AIRCR: u32,
    pub SCR: u32,
    pub CCR: u32,
    pub SHPR1: u32,
    pub SHPR2: u32,
    pub SHPR3: u32,
    pub SHCSR: u32,
    pub CFSR: u32,
    pub HFSR: u32,
    pub DFSR: u32,
    pub MMFAR: u32,
    pub BFAR: u32,
    pub ID_PFR0: u32,
    pub ID_PFR1: u32,
    pub ID_DFR0: u32,
    pub ID_AFR0: u32,
    pub ID_MMFR0: u32,
    pub ID_MMFR1: u32,
    pub ID_MMFR2: u32,
    pub ID_MMFR3: u32,
    pub ID_ISAR0: u32,
    pub ID_ISAR1: u32,
    pub ID_ISAR2: u32,
    pub ID_ISAR3: u32,
    pub ID_ISAR4: u32,
    pub CLIDR: u32,
    pub CTR: u32,
    pub CCSIDR: u32,
    pub CSSELR: u32,
    pub CPACR: u32,
    pub STIR: u32,
    pub ICIALLU: u32,
    pub ICIMVAU: u32,
    pub DCIMVAC: u32,
    pub DCISW: u32,
    pub DCCMVAU: u32,
    pub DCCMVAC: u32,
    pub DCCSW: u32,
    pub DCCIMVAC: u32,
    pub DCCISW: u32,
    pub CM7_ITCMCR: u32,
    pub CM7_DTCMCR: u32,
    pub CM7_AHBPCR: u32,
    pub CM7_CACR: u32,
    pub CM7_AHBSCR: u32,
    pub CM7_ABFSR: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
