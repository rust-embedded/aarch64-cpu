// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2023 Amazon.com, Inc. or its affiliates.
//
// Author(s):
//   - Ugur Usug <ugurus@amazon.com>

//! Pointer Authentication Key B for Instruction High - EL1
//!
//! Holds bits[127:64] of key B used for authentication of instruction pointer values.

use tock_registers::{Read, Register, Write};

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = u64;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "APIBKeyHi_EL1", "x");
}

impl Write for Reg {
    sys_coproc_write_raw!(u64, "APIBKeyHi_EL1", "x");
}

pub const APIBKEYHI_EL1: Reg = Reg {};
