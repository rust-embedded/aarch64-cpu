// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! PRLAR_EL2 - Protection Region Limit Register
//!
//! Sets the limit of the region which region selected via PRSELR_EL1

use tock_registers::{register_bitfields, Read, Register, Write};

register_bitfields! {u64,
    pub PRLAR_EL2 [
        /// Bits[47:6] of the upper inclusive limit of the selected MPU region
        Limit OFFSET(6) NUMBITS(39) [],

        /// Non-secure bit
        Ns OFFSET(4) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        /// Select attributes from the MAIR
        AttrIndex OFFSET(1) NUMBITS(3) [],

        /// Region enable bit
        En OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = PRLAR_EL2::Register;
}

impl Reg {
    pub fn base_addr(&self) -> *const u8 {
        (((self.read(PRLAR_EL2::Limit) as usize) << 6) | 0x3F) as *const u8
    }
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "PRLAR_EL2", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "PRLAR_EL2", "x");
}

pub const PRLAR_EL2: Reg = Reg {};
