// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2022-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! The frame pointer register

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    read_raw!(u64, "x29", "x");
}

impl Write for Reg {
    write_raw!(u64, "x29", "x");
}

pub const FP: Reg = Reg {};
