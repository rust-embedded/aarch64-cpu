// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! PRSELR_EL2 - Protection Region Selection Register
//!
//! Selects which region is accessed through PRBAR_EL2 and PRLAR_EL2

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub PRSELR_EL2 [
        /// The number of EL2-controlled MPU regions implemented
        Regions OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PRSELR_EL2::Register;

    sys_coproc_read_raw!(u64, "PRSELR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PRSELR_EL2::Register;

    sys_coproc_write_raw!(u64, "PRSELR_EL2", "x");
}

pub const PRSELR_EL2: Reg = Reg {};
