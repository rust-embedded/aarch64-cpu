// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Exception Link Register - EL1
//!
//! When taking an exception to EL1, holds the address to return to.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ELR_EL1", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "ELR_EL1", "x");
}

pub const ELR_EL1: Reg = Reg {};
