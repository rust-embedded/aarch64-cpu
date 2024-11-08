// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Yiyang Wu <toolmanp@outlook.com>

//! Interrupt Mask Bits
//!
//! Allows access to the interrupt mask bits.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! { u64,
    pub ICC_SRE_EL1 [
        EN OFFSET(3) NUMBITS(1) [
            ENABLED = 1,
            DISABLED = 0,
        ],
        DIB OFFSET(2) NUMBITS(1) [
            ENABLED = 1,
            DISABLED = 0,
        ],
        DFB OFFSET(1) NUMBITS(1)[
            ENABLED = 1,
            DISABLED = 0
        ],
        SRE OFFSET(1) NUMBITS(1) [
            MEMORY = 0,
            SR = 1
        ]
    ],
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICC_SRE_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_SRE_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICC_SRE_EL1::Register;

    sys_coproc_write_raw!(u64, "ICC_SRE_EL1", "x");
}

pub const ICC_SRE_EL1: Reg = Reg {};
