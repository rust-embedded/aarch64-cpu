// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Valentin B. <valentin.be@protonmail.com>

//! Auxiliary Control Register - EL3
//!
//! Provides implementation-defined configuration and control options for execution
//! at EL3.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ACTLR_EL3", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "ACTLR_EL3", "x");
}

pub const ACTLR_EL3: Reg = Reg;
