// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2020-2023 by the author(s)
//
// Author(s):
//   - Chris Brown <ccbrown112@gmail.com>

//! Debug Data Transfer Register, Receive
//!
//! Transfers data from an external debugger to the PE. For example, it is used by a debugger
//! transferring commands and data to a debug target. See DBGDTR_EL0 for additional architectural
//! mappings. It is a component of the Debug Communications Channel.

use tock_registers::{Read, Register};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "DBGDTRRX_EL0", "x");
}

pub const DBGDTRRX_EL0: Reg = Reg {};
