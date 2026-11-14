// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Javier Alvarez <javier.alvarez@allthingsembedded.net>

//! Vector Base Address Register - EL2
//!
//! Holds the vector base address for any exception that is taken to EL2.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "VBAR_EL2", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "VBAR_EL2", "x");
}

pub const VBAR_EL2: Reg = Reg {};
