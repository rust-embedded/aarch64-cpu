// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller Maintenance Interrupt State Register - EL2
//!
//! Indicates which maintenance interrupts are asserted.

use tock_registers::interfaces::{Readable, Writeable};

register_bitfields! {u64,
    pub ICH_MISR_EL2 [
        VGrp1D OFFSET(7) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        VGrp1E OFFSET(6) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        VGrp0D OFFSET(5) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        VGrp0E OFFSET(4) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        NP     OFFSET(3) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        LRENP  OFFSET(2) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        U      OFFSET(1) NUMBITS(1) [Asserted = 0b1, None = 0b0],
        EOI    OFFSET(0) NUMBITS(1) [Asserted = 0b1, None = 0b0],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICH_MISR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_MISR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICH_MISR_EL2::Register;

    sys_coproc_write_raw!(u64, "ICH_MISR_EL2", "x");
}

pub const ICH_MISR_EL2: Reg = Reg {};
