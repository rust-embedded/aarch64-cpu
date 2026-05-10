// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2024 by the author(s)
//
// Author(s):
//   - Sangwan Kwon <sangwan.kwon@samsung.com>
//   - Callum Thomson <callumthom11@gmail.com>

//! AArch64 Instruction Set Attribute Register 1 - EL1
//!
//! Provides information about the features and instructions implemented in AArch64 state.

use tock_registers::{interfaces::Readable, register_bitfields};

register_bitfields! {u64,
    pub ID_AA64ISAR1_EL1 [
        /// Support for LD64B and ST64B instructions
        LS644 OFFSET(60) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
            WithSt64Bv = 0b0010,
            WithSt64Bv0 = 0b0011,
            WithAtomicAccesses = 0b0100,
        ],

        /// Support for the XS attribute
        XS OFFSET(56) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for int8 matrix multiplication instructions
        I8MM OFFSET(52) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for data gathering hint instructions
        DGH OFFSET(48) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for advanced SIMD and floating-point BFloat16 instructions
        BF16 OFFSET(44) NUMBITS(4) [
            NotImplemented = 0b0000,
            InstructionOnly = 0b0001,
            WithEBF = 0b0010,
        ],

        /// Support for prediction invalidation instructions
        SPECRES OFFSET(40) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for SB instructions
        SB OFFSET(36) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for FRINT instructions
        FRINTTS OFFSET(32) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates support for an IMPLEMENTATION DEFINED algorithm is implemented in the PE for
        /// generic code authentication in AArch64 state.
        GPI OFFSET(28) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001
        ],

        /// Indicates whether the QARMA5 algorithm is implemented in the PE for generic code
        /// authentication in AArch64 state.
        GPA OFFSET(24) NUMBITS(4) [
            NotSupported = 0b0000,
            Supported = 0b0001,
        ],

        /// Support for weaker release consistency
        LRCPC OFFSET(20) NUMBITS(4) [
            NotImplemented = 0b0000,
            NoOffset = 0b0001,
            WithUnscaled = 0b0010,
            WithPostIndex = 0b0011,
        ],

        /// Support for complex number addition and multiplication
        FCMA OFFSET(16) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Support for the JavaScript conversion from double-precision floating-point values to integers
        JSCVT OFFSET(12) NUMBITS(4) [
            NotImplemented = 0b0000,
            Implemented = 0b0001,
        ],

        /// Indicates whether an IMPLEMENTATION DEFINED algorithm is implemented in the PE for
        /// address authentication, in AArch64 state. This applies to all Pointer Authentication
        /// instructions other than the PACGA instruction.
        API OFFSET(8) NUMBITS(4) [
            NotImplemented = 0b0000,
            NoEPACAndPauth = 0b0001,
            WithEPACNoPauth = 0b0010,
            WithPauthNoEPAC = 0b0011,
            WithFPACAndPauth2NoEPAC = 0b0100,
            WithFPACPauth2AndFpaccombineNoEPAC = 0b0101,
            WithLRSigningInstructions = 0b0110,
        ],

        /// Indicates whether the QARMA5 algorithm is implemented in the PE for address
        /// authentication, in AArch64 state. This applies to all Pointer Authentication
        /// instructions other than the PACGA instruction.
        APA OFFSET(4) NUMBITS(4) [
            NotImplemented = 0b0000,
            NoEPACAndPauth = 0b0001,
            WithEPACNoPauth = 0b0010,
            WithPauthNoEPAC = 0b0011,
            WithFPACAndPauth2NoEPAC = 0b0100,
            WithFPACPauth2AndFpaccombineNoEPAC = 0b0101,
            WithLRSigningInstructions = 0b0110,
        ],

        /// Data persistence writeback
        DPB OFFSET(0) NUMBITS(4) [
            NotSupported = 0b0000,
            Partial = 0b0001,
            Full = 0b0010,
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ID_AA64ISAR1_EL1::Register;

    sys_coproc_read_raw!(u64, "ID_AA64ISAR1_EL1", "x");
}

pub const ID_AA64ISAR1_EL1: Reg = Reg {};
