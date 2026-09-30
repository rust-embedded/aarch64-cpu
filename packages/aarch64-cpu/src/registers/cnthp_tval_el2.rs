// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2026 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Jonathan Pallant <jonathan.pallant@ferrous-systems.com>

//! Counter-timer Hyp Physical Timer TimerValue register - EL2
//!
//! Holds the timer value for the EL2 physical timer.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "CNTHP_TVAL_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "CNTHP_TVAL_EL2", "x");
}

pub const CNTHP_TVAL_EL2: Reg = Reg {};
