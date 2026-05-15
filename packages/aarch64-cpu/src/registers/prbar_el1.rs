// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2026 by the author(s)
//
// Author(s):
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! PRBAR_EL1 - Protection Region Base Register
//!
//! Sets the base of the region which region selected via PRSELR_EL1

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub PRBAR_EL1 [
        /// Bits[47:6] of the lower inclusive limit of the selected EL1 MPU memory region.
        Base OFFSET(6) NUMBITS(42) [],

        /// Shareability Attribute
        SH OFFSET(4) NUMBITS(2) [
            NonShareable = 0b00,
            Reserved = 0b01,
            OuterShareable = 0b10,
            InnerShareable = 0b11,
        ],

        /// Access Permissions Attribute
        AP OFFSET(1) NUMBITS(2) [
            ReadWriteEl1 = 0b00,
            ReadWriteEl1El0 = 0b01,
            ReadOnlyEl1 = 0b10,
            ReadOnlyEl1El0 = 0b11,
        ],

        NX OFFSET(1) NUMBITS(1) [
            ExecutionPermitted = 0b0,
            ExecutionNotPermitted = 0b1,
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PRBAR_EL1::Register;

    sys_coproc_read_raw!(u64, "PRBAR_EL1", "x");
}

impl Reg {
    pub fn base_addr(&self) -> *const u8 {
        ((self.read(PRBAR_EL1::Base) as usize) << 6) as *const u8
    }
}

impl Writeable for Reg {
    type T = u64;
    type R = PRBAR_EL1::Register;

    sys_coproc_write_raw!(u64, "PRBAR_EL1", "x");
}

pub const PRBAR_EL1: Reg = Reg {};
