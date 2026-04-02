// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Debug Feature Register 1 - EL1
//!
//! Provides top level information about the debug system in AArch64.

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64DFR1_EL1 [
        /// Number of breakpoint that support address linking, minus 1
        ABL_CMPs OFFSET(56) NUMBITS(8) [],

        /// Behaviour of the cycle counter when event counting is frozen by a Statistical Profiling management event
        DPFZS OFFSET(52) NUMBITS(4) [
            NeverAffected = 0b0000,
            FrozenByFZS = 0b0001,
        ],

        /// Exception Based event profiling
        EBEP OFFSET(48) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Instrumentation Trace Extension
        ITE OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Address Breakpoint Linking Extension
        ABLE OFFSET(40) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// PMU fixed-function instruction counter
        PMICNTR OFFSET(36) NUMBITS(4) [
            NotImplmented = 0b0000,
            Implmented = 0b0001,
        ],

        /// System PMU extension
        SPMU OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implmented = 0b0001,
            ImplmentedWithSPMZR = 0b0010,
        ],

        /// Context-aware breakpoints
        CTX_CMPs OFFSET(24) NUMBITS(8) [],

        /// Watchpoints
        WRPs OFFSET(16) NUMBITS(8) [],

        /// Breakpoints
        BRPs OFFSET(8) NUMBITS(8) [],

        /// System PMU ID
        SYSPMUID OFFSET(0) NUMBITS(8) [],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64DFR1_EL1", "x");
}

pub const ID_AA64DFR1_EL1: Reg = Reg;
