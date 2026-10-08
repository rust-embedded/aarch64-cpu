// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>
//   - Berkus Decker <berkus+github@metta.systems>

//! Performance Monitors User Enable Register - EL0
//!
//! Enables or disables EL0 access to the Performance Monitors. Readable at EL0, writable only at
//! EL1 or higher.
//!
//! When EL0 access is not enabled, an EL0 access to a Performance Monitors register is trapped
//! to EL1, or to EL2 when it is implemented and enabled for the current Security state and
//! HCR_EL2.TGE is 1, and reported using EC syndrome value 0x18.

use tock_registers::interfaces::{Readable, Writeable};
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

        /// Event counter Read when FEAT_PMUv3p9 is implemented. Traps EL0 reads of the event counters to EL1.
        ///
        ///     0b0 EL0 reads of PMXEVCNTR_EL0 and PMEVCNTR<n>_EL0, and EL0 read/write accesses
        ///         to PMSELR_EL0, are trapped, unless PMUSERENR_EL0.EN is 1.
        ///
        ///     0b1 EL0 reads of the event counters, and read/write accesses to PMSELR_EL0, are
        ///         not trapped.
        ///
        /// The reset behavior of this field is:
        ///     - On a Warm reset, this field resets to an architecturally UNKNOWN value.
        ER OFFSET(3) NUMBITS(1) [
            TrappedUnlessEnabled = 0,
            NotTrapped = 1
        ],

        /// Cycle counter Read. Traps EL0 reads of the cycle counter to EL1.
        ///
        ///     0b0 EL0 reads of PMCCNTR_EL0 are trapped, unless PMUSERENR_EL0.EN is 1.
        ///
        ///     0b1 EL0 reads of PMCCNTR_EL0 are not trapped.
        ///
        /// The reset behavior of this field is:
        ///     - On a Warm reset, this field resets to an architecturally UNKNOWN value.
        CR OFFSET(2) NUMBITS(1) [
            TrappedUnlessEnabled = 0,
            NotTrapped = 1,
        ],

        /// Software Increment register Write enable. Traps EL0 writes to the software increment register to EL1.
        ///
        ///     0b0 EL0 writes to PMSWINC_EL0 are trapped, unless PMUSERENR_EL0.EN is 1.
        ///
        ///     0b1 EL0 writes to PMSWINC_EL0 are not trapped.
        ///
        /// The reset behavior of this field is:
        ///     - On a Warm reset, this field resets to an architecturally UNKNOWN value.
        SW OFFSET(1) NUMBITS(1) [
            TrappedUnlessEnabled = 0,
            NotTrapped = 1,
        ],

        /// Traps EL0 accesses to the Performance Monitors registers to EL1.
        ///
        ///     0b0 EL0 accesses to the Performance Monitors registers are trapped, except where
        ///         enabled by PMUSERENR_EL0.{SW, CR, ER}.
        ///
        ///     0b1 EL0 accesses to the Performance Monitors registers are not trapped.
        ///
        /// The reset behavior of this field is:
        ///     - On a Warm reset, this field resets to an architecturally UNKNOWN value.
        EN OFFSET(0) NUMBITS(1) [
            TrappedUnlessEnabled = 0,
            NotTrapped = 1
        ],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = PMUSERENR_EL0::Register;

    sys_coproc_read_raw!(u64, "PMUSERENR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMUSERENR_EL0::Register;

    sys_coproc_write_raw!(u64, "PMUSERENR_EL0", "x");
}

pub const PMUSERENR_EL0: Reg = Reg;
