// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Hypervisor IPA Fault Address Register - EL2
//!
//! Holds the faulting IPA for some aborts on a stage 2 translation taken to EL2.

use tock_registers::{register_bitfields, Read, Register};

register_bitfields! {u64,
    pub HPFAR_EL2 [
        /// Faulting IPA address space.
        NS   OFFSET(63) NUMBITS(1) [],

        /// Faulting Intermediate Physical Address.
        FIPA OFFSET(4) NUMBITS(48) []
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = HPFAR_EL2::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "HPFAR_EL2", "x");
}

pub const HPFAR_EL2: Reg = Reg {};
