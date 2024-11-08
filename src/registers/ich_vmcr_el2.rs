// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller Virtual Machine Control Register - EL2
//!
//! Enables the hypervisor to save and restore the virtual machine view of the GIC state.

use tock_registers::interfaces::{Readable, Writeable};

register_bitfields! {u64,
    pub ICH_VMCR_EL2 [
        VMPR OFFSET(24) NUMBITS(8) [],
        VBPR0 OFFSET(21) NUMBITS(3) [],
        VBPR1 OFFSET(18) NUMBITS(3) [],
        VEOIM OFFSET(9) NUMBITS(1) [

        ],
        VCBPR OFFSET(4) NUMBITS(1) [
            Shared = 0b1,
            Private = 0b0
        ],
        VFIQEn OFFSET(3) NUMBITS(1) [
            IRQ = 0b0,
            FIQ = 0b1
        ],
        VAckCtl OFFSET(2) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        VENG1  OFFSET(1) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        VENG0  OFFSET(0) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICH_VMCR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_VMCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICH_VMCR_EL2::Register;

    sys_coproc_write_raw!(u64, "ICH_VMCR_EL2", "x");
}

pub const ICH_VMCR_EL2: Reg = Reg {};
