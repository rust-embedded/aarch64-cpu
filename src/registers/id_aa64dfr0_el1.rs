// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! AArch64 Debug Feature Register 0 - EL1
//!
//! Provides top level information about the debug system in AArch64 state.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64DFR0_EL1 [
        /// Branch Record Buffer Extension.
        BRBE OFFSET(52) NUMBITS(4) [],

        /// Multi-threaded PMU extension.
        MTPMU OFFSET(48) NUMBITS(4) [],

        /// Trace Buffer Extension.
        TraceBuffer OFFSET(44) NUMBITS(4) [],

        /// Armv8.4 Self-hosted Trace Extension version.
        TraceFilt OFFSET(40) NUMBITS(4) [],

        /// Statistical Profiling Extension version.
        PMSVer OFFSET(32) NUMBITS(4) [],

        /// Number of context-aware breakpoints, minus 1.
        CTX_CMPs OFFSET(28) NUMBITS(4) [],

        /// Number of watchpoints, minus 1.
        WRPs OFFSET(20) NUMBITS(4) [],

        /// Number of breakpoints, minus 1.
        BRPs OFFSET(12) NUMBITS(4) [],

        /// Performance Monitors Extension version.
        PMUVer OFFSET(8) NUMBITS(4) [],

        /// Trace support. Indicates whether System register interface to a trace unit is
        /// implemented.
        TraceVer OFFSET(4) NUMBITS(4) [],

        /// Debug architecture version. Indicates presence of Armv8 debug architecture.
        DebugVer OFFSET(4) NUMBITS(4) [],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64DFR0_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64DFR0_EL1", "x");
}

pub const ID_AA64DFR0_EL1: Reg = Reg {};
