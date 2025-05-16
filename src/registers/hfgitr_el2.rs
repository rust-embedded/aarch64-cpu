// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Valentin B. <valentin.be@protonmail.com>

//! Hypervisor Fault Group Information Register - EL2
//!
//! Provides information about the fault group.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "HFGITR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "HFGITR_EL2", "x");
}

pub const HFGITR_EL2: Reg = Reg;
