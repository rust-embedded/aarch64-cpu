// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Auxiliary Fault Status Register 0 - EL3
//!
//! Provides IMPLEMENTATION DEFINED fault status information for synchronous aborts that are taken to EL3.

use tock_registers::{Read, Register, Write};

#[derive(Copy, Clone)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "AFSR0_EL3", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "AFSR0_EL3", "x");
}

pub const AFSR0_EL3: Reg = Reg {};
