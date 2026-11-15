// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Accelerator Data - EL1

use tock_registers::{register_bitfields, Read, Register, Write};

register_bitfields! {u64,
    pub ACCDATA_EL1 [
        ACCDATA OFFSET(0) NUMBITS(32) [],
    ]
}

#[derive(Copy, Clone)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ACCDATA_EL1", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "ACCDATA_EL1", "x");
}

pub const ACCDATA_EL1: Reg = Reg;
