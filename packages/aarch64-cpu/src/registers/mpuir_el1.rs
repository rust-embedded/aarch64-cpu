// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! MPUIR_EL1 - Memory Protection Unit Type Register
//!
//! Identifies the number of regions supported by the EL1-controlled MPU

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub MPUIR_EL1 [
        /// The number of EL1-controlled MPU regions implemented
        Regions OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MPUIR_EL1::Register;

    sys_coproc_read_raw!(u64, "MPUIR_EL1", "x");
}

pub const MPUIR_EL1: Reg = Reg {};
