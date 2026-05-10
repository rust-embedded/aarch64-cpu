// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Processor Feature Register 0 - EL1
//!
//! Provides additional information about implemented PE features in AArch64 state.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64PFR0_EL1 [
        /// Speculative use of faulting data
        CSV3 OFFSET(60) NUMBITS(4) [
            Undisclosed = 0b0000,
            Defined = 0b0001,
        ],

        /// Speculative use of context prediction
        CSV2 OFFSET(56) NUMBITS(4) [
            Undisclosed = 0b0000,
            Partial = 0b0001,
            WithCSV22 = 0b0010,
            WithCSV23 = 0b0011,
        ],

        /// Realm Management Extension
        RME OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            V1Implemented = 0b0001,
            WithGPC2 = 0b0010,
            WithGPC3 = 0b0011
        ],

        /// Data Independent timing
        DIT OFFSET(48) NUMBITS(4) [
            NoGuarantee = 0b0000,
            ViaPstateDIT = 0b0001,
        ],

        /// Indicates support for Activity Monitors Extension.
        AMU OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
            WithCouterVirtualization = 0b0010,
        ],

        /// Major version of MPAM extension
        MPAM OFFSET(40) NUMBITS(4) [
            Version0 = 0b0000,
            Version1 = 0b0001,
        ],

        /// Secure EL2
        SEL2 OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Scalable Vector Extension.
        SVE OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// RAS Extension version
        RAS OFFSET(28) NUMBITS(4) [
            NoRAS = 0b0000,
            Supported = 0b0001,
            WithAdditionalRegisters = 0b0010,
            WithFileGrainedTraps = 0b0011,
        ],

        /// System register GIC CPU interface
        GIC OFFSET(24) NUMBITS(4) [
            Implemented = 0b0000,
            Version3And4 = 0b0001,
            Version41 = 0b0011,
        ],

        /// Advanced SIMD
        AdvSIMD OFFSET(20) NUMBITS(4) [
            Implemented = 0b0000,
            WithHalfPrecision = 0b0001,
            NotImplemented = 0b1111,
        ],

        /// Floating Point
        FP OFFSET(16) NUMBITS(4) [
            Implemented = 0b0000,
            WithHalfPrecision = 0b0001,
            NotImplemented = 0b1111,
        ],

        /// EL3 Exception level handling
        EL3 OFFSET(12) NUMBITS(4) [
            NotImplemented = 0b0000,
            AArch64Only = 0b0001,
            AArch64Or32 = 0b0010,
        ],

        /// EL2 Exception level handling
        EL2 OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            AArch64Only = 0b0001,
            AArch64Or32 = 0b0010,
        ],

        /// EL1 Exception level handling
        EL1 OFFSET(4) NUMBITS(4) [
            AArch64Only = 0b0001,
            AArch64Or32 = 0b0010,
        ],

        /// EL0 Exception level handling
        EL0 OFFSET(0) NUMBITS(4) [
            AArch64Only = 0b0001,
            AArch64Or32 = 0b0010,
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64PFR0_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64PFR0_EL1", "x");
}

pub const ID_AA64PFR0_EL1: Reg = Reg {};
