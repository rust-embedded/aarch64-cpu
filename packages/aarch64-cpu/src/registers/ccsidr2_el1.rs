// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! Current Cache Size ID Register 2 - EL1

use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub CCSIDR2_EL1 [
        NumSets OFFSET(0) NUMBITS(24) [],
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "CCSIDR2_EL1", "x");
}

pub const CCSIDR2_EL1: Reg = Reg;
