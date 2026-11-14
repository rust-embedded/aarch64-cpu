// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Javier Alvarez <javier.alvarez@allthingsembedded.net>

//! OS Lock Access Register - EL1
//!
//! Used to lock or unlock the OS Lock.
//!
//! AArch64 System register `OSLAR_EL1` bits \[31:0\] are architecturally mapped to External
//! register `OSLAR_EL1[31:0]`. The OS Lock can also be locked or unlocked using `DBGOSLAR`.

use tock_registers::{register_bitfields, Read, Register, Write};

register_bitfields! {u64,
    pub OSLAR_EL1 [
        /// On writes to `OSLAR_EL1`, bit[0] is copied to the OS Lock.
        /// Use `OSLSR_EL1.OSLK` to check the current status of the lock.
        OSLK OFFSET(0) NUMBITS(1) [
            Unlocked = 0,
            Locked = 1
        ]
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = OSLAR_EL1::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "OSLAR_EL1", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "OSLAR_EL1", "x");
}

pub const OSLAR_EL1: Reg = Reg {};
