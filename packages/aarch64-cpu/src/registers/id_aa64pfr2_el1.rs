// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Processor Feature Register 2 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub ID_AA64PFR2_EL1 [
        /// Support for FPMR
        FPMR OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Support for software injection of Undefined Instruction exceptions
        UINJ OFFSET(16) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemnted = 0b0001,
        ],

        /// Indicates if FAR_ELx[63:60] are UNKNOWN on a syncronous exception due to a Tag Check Fault
        MTEFAR OFFSET(8) NUMBITS(4) [
            BitsAreUnknown = 0b0000,
            BitsAreNotUnknown = 0b0001,
        ],

        /// Support for Store-only Tag checking
        MTESTOREONLY OFFSET(4) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for Allocation tag access permissions
        MTEPERM OFFSET(0) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ]
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ID_AA64PFR2_EL1", "x");
}

pub const ID_AA64PFR2_EL1: Reg = Reg;
