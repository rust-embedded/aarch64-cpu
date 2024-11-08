// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller VGIC Type Register - EL2
//!
//! Reports supported GIC virtualization features.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ICH_VTR_EL2 [
        /// Priority bits. Indicates the number of virtual priority bits implemented, minus one.
        PRIbits OFFSET(29) NUMBITS(3) [],

        /// Preemption bits. Indicates the number of virtual preemption bits implemented, minus one.
        PREbits OFFSET(26) NUMBITS(3) [],

        /// The number of virtual interrupt identifier bits supported.
        IDbits OFFSET(23) NUMBITS(3) [
            BITS_16 = 0b000,
            BITS_24 = 0b001
        ],

        SEIS  OFFSET(22) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        A3V OFFSET(21) NUMBITS(1) [
            Zero = 0b0,
            NonZero = 0b1
        ],
        NV4 OFFSET(20) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0
        ],
        TDS OFFSET(19) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        DVIM OFFSET(18) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],

        /// List Registers. Indicates the number of List registers implemented, minus one.
        ListRegs OFFSET(0) NUMBITS(5) [],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICH_VTR_EL2::Register;

    sys_coproc_read_raw!(u64, "ICH_VTR_EL2", "x");
}

pub const ICH_VTR_EL2: Reg = Reg {};
