// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2022-2023 Amazon.com, Inc. or its affiliates.
//
// Author(s):
//   - Ali Saidi <alisaidi@amazon.com>

//! AArch64 Instruction Set Architecture Feature Register 0 - EL1
//!
//! Provides information about the implemented instruction set.

use tock_registers::{register_bitfields, Read, Register};

register_bitfields! {u64,
    pub ID_AA64ISAR0_EL1 [
        /// Support for Random Number instructions in AArch64.
        ///
        /// 0000 No random number instructions are implemented
        /// 0001 RNDR and RNDRSS are implemented
        ///
        /// All other values are reserved.
        RNDR OFFSET(60) NUMBITS(4) [
            Supported = 0b0001,
            NotSupported = 0b0000
        ],
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = ID_AA64ISAR0_EL1::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ID_AA64ISAR0_EL1", "x");
}

pub const ID_AA64ISAR0_EL1: Reg = Reg {};
