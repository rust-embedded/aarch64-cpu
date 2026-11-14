// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2025 by the author(s)
//
// Author(s):
//   - Yan Tan <tanyan@kylinos.cn>

//! Auxiliary Memory Attribute Indirection Register
//!
//! Provides IMPLEMENTATION DEFINED memory attributes for the stage 1 translation tables.
//! This register is used to define memory attributes that are not defined in the MAIR_EL1 register.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "S3_0_C10_C3_0", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "S3_0_C10_C3_0", "x");
}

pub const AMAIR_EL1: Reg = Reg {};
