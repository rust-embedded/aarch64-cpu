// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2020-2023 by the author(s)
//
// Author(s):
//   - Chris Brown <ccbrown112@gmail.com>

//! Debug Data Transfer Register, Transmit
//!
//! Transfers data from the PE to an external debugger. For example, it is used by a debug target
//! to transfer data to the debugger. See DBGDTR_EL0 for additional architectural mappings. It is a
//! component of the Debug Communication Channel.

use tock_registers::{Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "DBGDTRTX_EL0", "x");
}

pub const DBGDTRTX_EL0: Reg = Reg {};
