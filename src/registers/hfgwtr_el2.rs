// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2025 by the author(s)
//
// Author(s):
//   - lingfuyi. <lingfuyi@kylinos.cn>

//! Hypervisor Fine-Grained Write Trap Register - EL2
//!
//! Provides controls for traps of MSR and MCR writes of System registers

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "HFGWTR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "HFGWTR_EL2", "x");
}

pub const HFGWTR_EL2: Reg = Reg;
