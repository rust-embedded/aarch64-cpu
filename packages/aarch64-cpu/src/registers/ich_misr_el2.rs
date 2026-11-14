// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller Maintenance Interrupt State Register - EL2
//!
//! Indicates which maintenance interrupts are asserted.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ICH_MISR_EL2", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "ICH_MISR_EL2", "x");
}

pub const ICH_MISR_EL2: Reg = Reg {};
