// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2025 by the author(s)
//
// Author(s):
//   - lingfuyi. <lingfuyi@kylinos.cn>

//! Hypervisor Fine-Grained Read Trap Register - EL2
//!
//! Provides controls for traps of MRS and MRC reads of System registers
//! at EL2.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "HFGRTR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "HFGRTR_EL2", "x");
}

pub const HFGRTR_EL2: Reg = Reg;
