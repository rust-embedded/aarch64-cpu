// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2025 by the author(s)
//
// Author(s):
//   - Yan Tan <tanyan@kylinos.cn>

//! Monitor Debug System Control Register
//!
//! Main control register for the debug implementation.
//! This register is present only when FEAT_AA64 is implemented.
//! Otherwise, direct accesses to MDSCR_EL1 are UNDEFINED.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub MDSCR_EL1 [
        // Reserved, RES0，[63:51]

        /// Software step control bit for MDSTEPOP_EL1
        ///
        /// When FEAT_STEP2 is implemented:
        /// - 0: Execution from MDSTEPOP_EL1 is disabled
        /// - 1: Execution from MDSTEPOP_EL1 is not disabled by this control
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        /// Otherwise: Reserved, RES0
        EnSTEPOP OFFSET(50) NUMBITS(1) [],

        // Reserved, RES0，[49:36]

        /// Extended Halting Breakpoint and Watchpoint Enable
        ///
        /// When FEAT_Debugv8p9 is implemented:
        /// - Used for save/restore of EDSCR2.EHBWE
        ///
        /// - When OSLSR_EL1.OSLK is 0, software must treat this field as UNK/SBZP.
        /// - When OSLSR_EL1.OSLK is 1, this field holds the value of EDSCR2.EHBWE
        ///
        /// Reads and writes of this field are indirect accesses to EDSCR2.EHBWE
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 0: RO
        /// - Otherwise: RW
        EHBWE OFFSET(35) NUMBITS(1) [],

        /// Enable access to System PMU registers
        ///
        /// When FEAT_SPMU is implemented:
        /// - 0: Accesses of the specified System PMU registers at EL0 are trapped to EL1
        /// - 1: Accesses of the specified System PMU registers are not trapped
        ///
        /// Trapped instructions are reported using EC syndrome value 0x18
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        EnSPM OFFSET(34) NUMBITS(1) [
            Trapped = 0,
            NotTrapped = 1
        ],

        /// Trap Trace Accesses
        ///
        /// When FEAT_TRBE_EXT or FEAT_ETEv1p3 is implemented:
        /// - Used for save/restore of EDSCR2.TTA
        ///
        /// - When OSLSR_EL1.OSLK is 0, software must treat this field as UNK/SBZP
        /// - When OSLSR_EL1.OSLK is 1, this field holds the value of EDSCR2.TTA
        ///
        /// On a Cold reset, this field resets to '0'
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 0: RO
        /// - Otherwise: RW
        TTA OFFSET(33) NUMBITS(1) [],

        /// Extended Monitor Breakpoint and Watchpoint Enable
        ///
        /// When FEAT_Debugv8p9 is implemented:
        /// - 0: Breakpoint and Watchpoint exceptions are disabled for each breakpoint <n> and \
        ///      watchpoint <n>, where n is greater than or equal to 16
        /// - 1: Breakpoint and Watchpoint exceptions are not affected by this mechanism
        ///
        /// This field is ignored when:
        /// - EL3 is implemented and MDCR_EL3.EBWE is 0
        /// - EL2 is implemented and enabled, and MDCR_EL2.EBWE is 0
        EMBWE OFFSET(32) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Trace Filter override
        ///
        /// When FEAT_TRF is implemented:
        /// - Used for save/restore of EDSCR.TFO
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.TFO
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        TFO OFFSET(31) NUMBITS(1) [],

        /// Used for save/restore of EDSCR.RXfull
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.RXfull
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        RXfull OFFSET(30) NUMBITS(1) [],

        /// Used for save/restore of EDSCR.TXfull
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.TXfull
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        TXfull OFFSET(29) NUMBITS(1) [],

        // Reserved, RES0，[28]

        /// Used for save/restore of EDSCR.RXO
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.RXO
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        RXO OFFSET(27) NUMBITS(1) [],

        /// Used for save/restore of EDSCR.TXU
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.TXU
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        TXU OFFSET(26) NUMBITS(1) [],

        // Reserved, RES0，[25:24]

        /// Used for save/restore of EDSCR.INTdis
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this field as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this field holds the value of EDSCR.INTdis
        ///
        /// On a Cold reset, this field resets to '00'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        INTdis OFFSET(22) NUMBITS(2) [],

        /// Used for save/restore of EDSCR.TDA
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.TDA
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        TDA OFFSET(21) NUMBITS(1) [],

        // Reserved, RES0，[20]

        /// Used for save/restore of EDSCR.SC2
        ///
        /// When FEAT_PCSRv8 is implemented, FEAT_VHE is implemented, and FEAT_PCSRv8p2 is not implemented:
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.SC2
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        SC2 OFFSET(19) NUMBITS(1) [],

        // Reserved, RAZ/WI, [18:16]

        // Monitor debug events
        ///
        /// Enable Breakpoint, Watchpoint, and Vector Catch exceptions
        ///
        /// - 0: Breakpoint, Watchpoint, and Vector Catch exceptions disabled
        /// - 1: Breakpoint, Watchpoint, and Vector Catch exceptions enabled
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        MDE OFFSET(15) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Used for save/restore of EDSCR.HDE
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.HDE
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        HDE OFFSET(14) NUMBITS(1) [],

        /// Local (kernel) debug enable
        ///
        /// If ELD is using AArch64, enable debug exceptions within ELD
        ///
        /// - 0: Debug exceptions, other than Breakpoint Instruction exceptions, disabled within ELD
        /// - 1: All debug exceptions enabled within ELD
        ///
        /// RES0 if ELD is using AArch32
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        KDE OFFSET(13) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Traps EL0 accesses to the Debug Communication Channel (DCC) registers
        ///
        /// - 0: This control does not cause any instructions to be trapped
        /// - 1: EL0 accesses to the DCC registers are trapped
        ///
        /// Trapped instructions are reported using EC syndrome value 0x18 for AArch64
        /// or 0x05/0x06/0x0C for AArch32
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        TDCC OFFSET(12) NUMBITS(1) [
            NotTrapped = 0,
            Trapped = 1
        ],

        // Reserved, RES0，[11:7]

        /// Used for save/restore of EDSCR.ERR
        ///
        /// - When OSLSR_EL1.OSLK == 0, software must treat this bit as UNK/SBZP
        /// - When OSLSR_EL1.OSLK == 1, this bit holds the value of EDSCR.ERR
        ///
        /// On a Cold reset, this field resets to '0'
        ///
        /// Access behavior:
        /// - When OSLSR_EL1.OSLK == 1: RW
        /// - When OSLSR_EL1.OSLK == 0: RO
        ERR OFFSET(6) NUMBITS(1) [],

        // Reserved, RES0，[5:1]

        /// Software step control bit
        ///
        /// If ELD is using AArch64, enable Software step
        /// - 0: Software step disabled
        /// - 1: Software step enabled
        ///
        /// RES0 if ELD is using AArch32
        ///
        /// On a Warm reset, this field resets to an architecturally UNKNOWN value
        SS OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ]
}
pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MDSCR_EL1::Register;

    sys_coproc_read_raw!(u64, "S2_0_C0_C2_2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = MDSCR_EL1::Register;

    sys_coproc_write_raw!(u64, "S2_0_C0_C2_2", "x");
}

pub const MDSCR_EL1: Reg = Reg {};
