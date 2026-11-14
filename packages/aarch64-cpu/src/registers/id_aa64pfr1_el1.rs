// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! AArch64 Processor Feature Register 1 - EL1
//!
//! Provides additional information about implemented PE features in AArch64 state.

use tock_registers::{register_bitfields, Read, Register};

register_bitfields! {u64,
    pub ID_AA64PFR1_EL1 [
        /// Support for the Memory Tagging Extension.
        MTE OFFSET(8) NUMBITS(4) [],
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = ID_AA64PFR1_EL1::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ID_AA64PFR1_EL1", "x");
}

pub const ID_AA64PFR1_EL1: Reg = Reg {};
