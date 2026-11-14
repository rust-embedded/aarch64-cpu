// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Alban Seurat <alban.seurat@me.com>

//! The link register

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    read_raw!(u64, "lr", "x");
}

impl Write for Reg {
    write_raw!(u64, "lr", "x");
}

pub const LR: Reg = Reg {};
