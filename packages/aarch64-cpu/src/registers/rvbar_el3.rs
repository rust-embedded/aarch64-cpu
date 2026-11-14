// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Matt Schulte <schultetwin1@gmail.com>

//! Reset Vector Base Address Register - EL3
//!
//! If EL3 is the highest Exception level implemented, contains the
//! IMPLEMENTATION DEFINED address that execution starts from after reset when
//! executing in AArch64 state.

use tock_registers::{Read, Register};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "RVBAR_EL3", "x");
}

pub const RVBAR_EL3: Reg = Reg;
