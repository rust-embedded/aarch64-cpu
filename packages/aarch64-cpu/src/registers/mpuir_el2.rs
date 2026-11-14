// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! MPUIR_EL2 - Memory Protection Unit Type Register
//!
//! Identifies the number of regions supported by the EL2-controlled MPU

use tock_registers::{register_bitfields, Read, Register};

register_bitfields! {u64,
    pub MPUIR_EL2 [
        /// The number of EL2-controlled MPU regions implemented
        Regions OFFSET(0) NUMBITS(8) []
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = MPUIR_EL2::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "MPUIR_EL2", "x");
}

pub const MPUIR_EL2: Reg = Reg {};
