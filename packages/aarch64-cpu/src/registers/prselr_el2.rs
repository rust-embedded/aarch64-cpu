// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! PRSELR_EL2 - Protection Region Selection Register
//!
//! Selects which region is accessed through PRBAR_EL2 and PRLAR_EL2

use tock_registers::{register_bitfields, Read, Register, Write};

register_bitfields! {u64,
    pub PRSELR_EL2 [
        /// The number of EL2-controlled MPU regions implemented
        Regions OFFSET(0) NUMBITS(8) []
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = PRSELR_EL2::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "PRSELR_EL2", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "PRSELR_EL2", "x");
}

pub const PRSELR_EL2: Reg = Reg {};
