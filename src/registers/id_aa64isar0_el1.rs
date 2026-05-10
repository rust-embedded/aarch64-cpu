// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2022-2023 Amazon.com, Inc. or its affiliates.
//
// Author(s):
//   - Ali Saidi <alisaidi@amazon.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Instruction Set Architecture Feature Register 0 - EL1
//!
//! Provides information about the implemented instruction set.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64ISAR0_EL1 [
        /// Support for Random Number instructions in AArch64.
        ///
        /// 0000 No random number instructions are implemented
        /// 0001 RNDR and RNDRSS are implemented
        ///
        /// All other values are reserved.
        RNDR OFFSET(60) NUMBITS(4) [
            Supported = 0b0001,
            NotSupported = 0b0000,
        ],

        /// Support for Outer Sharable and TLB range maintenance instructions
        TLB OFFSET(56) NUMBITS(4) [
            NotImplemented = 0b0000,
            OuterShareable = 0b0001,
            Implemented = 0b0010,
        ],

        /// Support for flag manipulation instructions
        TS OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            Partial = 0b0001,
            Full = 0b0010,
        ],

        /// Support for FMAL and FMLSL instructions
        FHM OFFSET(48) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Dot product instructions
        DP OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SM4 instructions
        SM4 OFFSET(40) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SM3 instructions
        SM3 OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SHA3 instructions
        SHA3 OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SQRDMLAH and SQRDMLSH instructions
        RDM OFFSET(28) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for TME instructions
        TME OFFSET(24) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for Atomic instructions
        Atomic OFFSET(20) NUMBITS(4) [
            NotImplemented = 0b0000,
            Partial = 0b0001,
            Full = 0b0010,
        ],

        /// Support for CRC32 instructions
        CRC32 OFFSET(16) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SHA2 instructions
        SHA2 OFFSET(12) NUMBITS(4) [
            NotImplemented = 0b0000,
            Partial = 0b0001,
            Full = 0b0010,
        ],

        /// Support for SHA1 instructions
        SHA1 OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for AES instructions
        AES OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            Partial = 0b0001,
            Full = 0b0010,
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64ISAR0_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64ISAR0_EL1", "x");
}

pub const ID_AA64ISAR0_EL1: Reg = Reg {};
