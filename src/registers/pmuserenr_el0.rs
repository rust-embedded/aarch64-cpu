// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Performance Monitors User Enable Register - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub PMUSERENR_EL0 [
        /// Instruction counter Read-only
        IR OFFSET(5) NUMBITS(1) [
            WritesUnaffected = 0,
            WritesIgnored = 1,
        ],

        /// User Enable
        UEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// Event counters Read enable or Read-only
        ER OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// Cycle counter Read enable or Read-only
        CR OFFSET(2) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],

        /// Software increment register Write enable.
        SW OFFSET(1) NUMBITS(1) [
            El0Enabled = 0,
            El1Enabled = 1,
        ],

        /// Enable
        EN OFFSET(0) NUMBITS(1) [
            TrappedUnlessEnabled = 0,
            Enabled = 1,
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "PMUSERENR_EL0", "x");
}

pub const PMUSERENR_EL0: Reg = Reg;
