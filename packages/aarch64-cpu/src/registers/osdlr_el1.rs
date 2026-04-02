// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Callum Thomson <callumthom11@gmail.com>

//! OS Double Lock Register - EL1

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_bitfields;

pub struct Reg;

register_bitfields! {u64,
    pub OSDLR_EL1 [
        /// OS Double Lock control bit
        DLK OFFSET(0) NUMBITS(1) [
            Unlocked = 0,
            Locked = 1
        ]
    ]
}

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "OSDLR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "OSDLR_EL1", "x");
}

pub const OSDLR_EL1: Reg = Reg;
