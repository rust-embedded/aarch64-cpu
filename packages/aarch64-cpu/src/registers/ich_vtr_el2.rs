// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller VGIC Type Register - EL2
//!
//! Reports supported GIC virtualization features.

use tock_registers::{register_bitfields, Read, Register};

register_bitfields! {u64,
    pub ICH_VTR_EL2 [
        /// Priority bits. Indicates the number of virtual priority bits implemented, minus one.
        PRIbits OFFSET(29) NUMBITS(3) [],

        /// Preemption bits. Indicates the number of virtual preemption bits implemented, minus one.
        PREbits OFFSET(26) NUMBITS(3) [],

        /// The number of virtual interrupt identifier bits supported.
        IDbits OFFSET(23) NUMBITS(3) [],

        /// List Registers. Indicates the number of List registers implemented, minus one.
        ListRegs OFFSET(0) NUMBITS(5) [],
    ]
}

#[derive(Clone, Copy)]
pub struct Reg;

impl Register for Reg {
    type DataType = ICH_VTR_EL2::Register;
}

impl Read for Reg {
    sys_coproc_read_raw!(u64, "ICH_VTR_EL2", "x");
}

pub const ICH_VTR_EL2: Reg = Reg {};
