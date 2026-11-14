// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! The stack pointer - EL0
//!
//! Holds the stack pointer associated with EL0. At higher Exception levels, this is used as the
//! current stack pointer when the value of SPSel.SP is 0.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "SP_EL0", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "SP_EL0", "x");
}

pub const SP_EL0: Reg = Reg {};
