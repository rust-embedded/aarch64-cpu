// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>

//! Interrupt Controller Control Register - EL1
//!
//! Controls aspects of the behavior of the GIC CPU interface and provides information
//! about the features implemented.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ICC_CTLR_EL1 [
        /// Extended INTID range (read-only).
        ExtRange OFFSET(19) NUMBITS(1) [
            WideUnsupported = 0b0, // support int_id 1024 ... 8191
            WideSupported = 0b1,
        ],
        RSS  OFFSET(18) NUMBITS(1) [
            Low = 0b0,
            High = 0b1
        ],
        A3V  OFFSET(15) NUMBITS(1) [
            Disable = 0b0,
            Enable = 0b1,
        ],
        SEIS OFFSET(14) NUMBITS(1) [
            Disable = 0b0,
            Enable = 0b1
        ],
        ID  OFFSET(11) NUMBITS(3) [
            BIT_16 = 0b000,
            BIT_24 = 0b001
        ],
        PRI OFFSET(8) NUMBITS(3) [],
        PMHE OFFSET(6) NUMBITS(1) [
            Enable = 0b1,
            Disable = 0b0,
        ],
        EOI  OFFSET(1) NUMBITS(1) [
            EOIR_DropAndDeactivate = 0b0,
            EOIR_Drop_Dir_Deactivate = 0b1
        ],
        CBPR OFFSET(0) NUMBITS(1) [
            Private = 0b0,
            Shared = 0b1,
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICC_CTLR_EL1::Register;

    sys_coproc_read_raw!(u64, "ICC_CTLR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICC_CTLR_EL1::Register;

    sys_coproc_write_raw!(u64, "ICC_CTLR_EL1", "x");
}

pub const ICC_CTLR_EL1: Reg = Reg {};
