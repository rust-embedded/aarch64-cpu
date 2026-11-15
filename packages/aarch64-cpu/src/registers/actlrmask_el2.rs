// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Auxiliary Control Masking Register - EL2

use tock_registers::{Read, Register, Write};

#[derive(Copy, Clone)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ACTLRMASK_EL2", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "ACTLRMASK_EL2", "x");
}

pub const ACTLRMASK_EL2: Reg = Reg;
