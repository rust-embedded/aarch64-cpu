// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Processor Feature Register 1 - EL1
//!
//! Provides additional information about implemented PE features in AArch64 state.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64PFR1_EL1 [
        /// Support for physical fault address registers
        PFAR OFFSET(60) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for error routing extensions
        DF2 OFFSET(56) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for additional MTE tag check modes
        MTEX OFFSET(52) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for Translation Hardening Extension
        THE OFFSET(48) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Guarded Control Stack
        GCS OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Asynchronous reporting of a Tag Check Fault
        MTE_frac OFFSET(40) NUMBITS(4) [
            Supported = 0b0000,
            NotSupported = 0b1111,
        ],

        /// Non-maskable Interrupt
        NMI OFFSET(36) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// CSV2 fractional field
        CSV2_frac OFFSET(32) NUMBITS(4) [
            Undisclosed = 0b0000,
            Partial = 0b0001,
            Implemented = 0b0010,
        ],

        /// Random Number trap to EL3
        RNDR_trap OFFSET(28) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Scalable Matrix Extension
        SME OFFSET(24) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Minor version number for the MPAM Extension
        MPAM_frac OFFSET(16) NUMBITS(4) [
            MinorVersion0 = 0b0000,
            MinorVersion1 = 0b0001,
        ],

        /// RAS Extension fractional field
        RAS_frac OFFSET(12) NUMBITS(4) [
            Implmented = 0b0000,
            ImplmentedExtraRegisters = 0b0001,
        ],

        /// Support for the Memory Tagging Extension.
        MTE OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            InstructionOnly = 0b0001,
            InstructionOnlyOptionalAsync = 0b0010,
            InstructionMandatoryAsync = 0b0011,
        ],

        /// Speculative Store Bypassing controls
        SSBS OFFSET(4) NUMBITS(4) [
            NoMechanism = 0b0000,
            SSBSOnly = 0b0001,
            SSBSWithMsrAndMrs = 0b0010,
        ],

        /// Branch Target Identification
        BT OFFSET(0) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64PFR1_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64PFR1_EL1", "x");
}

pub const ID_AA64PFR1_EL1: Reg = Reg {};
