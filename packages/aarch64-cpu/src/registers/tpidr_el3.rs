// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Software Thread ID Register - EL3

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "TPIDR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "TPIDR_EL3", "x");
}

pub const TPIDR_EL3: Reg = Reg;
