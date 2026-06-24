// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! PRLAR_EL1 - Protection Region Limit Register
//!
//! Sets the limit of the region which region selected via PRSELR_EL1

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub PRLAR_EL1 [
        /// Bits[47:6] of the upper inclusive limit of the selected MPU region
        Limit OFFSET(6) NUMBITS(39) [],

        /// Non-secure bit
        Ns OFFSET(4) NUMBITS(1) [
            Secure = 0,
            NonSecure = 1
        ],

        /// Select attributes from the MAIR
        AttrIndex OFFSET(1) NUMBITS(3) [],

        // Is the region enabled?
        En OFFSET(0) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0
        ]
    ]
}

pub struct Reg;

impl Reg {
    pub fn base_addr(&self) -> *const u8 {
        (((self.read(PRLAR_EL1::Limit) as usize) << 6) | 0x3F) as *const u8
    }
}

impl Readable for Reg {
    type T = u64;
    type R = PRLAR_EL1::Register;

    sys_coproc_read_raw!(u64, "PRLAR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PRLAR_EL1::Register;

    sys_coproc_write_raw!(u64, "PRLAR_EL1", "x");
}

pub const PRLAR_EL1: Reg = Reg {};
